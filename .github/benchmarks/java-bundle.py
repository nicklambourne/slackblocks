"""Controlled signed-bundle comparison; never publishes outside loopback."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import secrets
import shutil
import subprocess
import time
import zipfile

p = argparse.ArgumentParser()
p.add_argument('--source', type=Path, required=True)
p.add_argument('--output', type=Path, required=True)
p.add_argument('--cache', type=Path, required=True)
p.add_argument('--variant', choices=['baseline', 'candidate'], required=True)
p.add_argument('--pair', type=int, required=True)
a = p.parse_args()
a.source = a.source.resolve()
a.output.mkdir(parents=True, exist_ok=True)
assert not a.cache.exists(), 'Each job must start with its own empty cache namespace'
a.cache.mkdir(parents=True)

def sha(data):
    return hashlib.sha256(data).hexdigest()

def save(path, obj):
    path.write_text(json.dumps(obj, indent=2) + '\n')

def files(root, suffixes=None):
    return {str(f.relative_to(root)): {'sha256': sha(f.read_bytes()), 'bytes': f.stat().st_size}
            for f in sorted(root.rglob('*')) if f.is_file()
            and (suffixes is None or f.suffix in suffixes)}

def cmd(args, **kw):
    return subprocess.check_output(args, stderr=subprocess.STDOUT, **kw)

tracked = cmd(['git', 'ls-files', '-z'], cwd=a.source).decode().split('\0')
source = {name: sha((a.source / name).read_bytes()) for name in tracked if name}
save(a.output / 'source.json', source)
checker = a.source / '.github/scripts/check-java-bundle.py'
original = checker.read_text()
assert original.count('"-Dmaven.test.skip=true"') == 1
if a.variant == 'baseline':
    checker.write_text(original.replace('"-Dmaven.test.skip=true"', '"-DskipTests"'))
save(a.output / 'inputs.json', {
    'source_commit': cmd(['git', 'rev-parse', 'HEAD'], cwd=a.source).decode().strip(),
    'variant': a.variant, 'pair': a.pair,
    'checker_sha256': sha(checker.read_bytes()),
    'java': cmd(['java', '-version']).decode(),
    'python': cmd(['python', '--version']).decode(),
    'image_os': os.environ.get('ImageOS'), 'image_version': os.environ.get('ImageVersion'),
    'runner_name': os.environ.get('RUNNER_NAME'), 'runner_arch': os.environ.get('RUNNER_ARCH'),
    'run_id': os.environ.get('GITHUB_RUN_ID'), 'run_attempt': os.environ.get('GITHUB_RUN_ATTEMPT'),
})
env = os.environ.copy()
env.update(MAVEN_USER_HOME=str(a.cache / 'maven'),
           MAVEN_OPTS='-Dmaven.repo.local=' + str(a.cache / 'maven/repository'),
           GNUPGHOME=str(a.cache / 'gpg'),
           MAVEN_GPG_PASSPHRASE=secrets.token_hex(24))
Path(env['GNUPGHOME']).mkdir(mode=0o700)
# One throwaway key per pair/policy job. Key generation is outside bundle timings.
cmd(['gpg', '--batch', '--pinentry-mode', 'loopback', '--passphrase', env['MAVEN_GPG_PASSPHRASE'],
     '--quick-gen-key', 'slackblocks benchmark <ci-dry-run@invalid>', 'ed25519', 'sign', '1d'], env=env)
try:
    for temperature in ['cold', 'warm']:
        out = a.output / temperature
        out.mkdir()
        before = files(a.cache / 'maven')
        save(out / 'cache-before.json', before)
        assert bool(before) == (temperature == 'warm')
        # Both policies run clean deploy; the warm sample retains downloads only.
        events = []
        start = time.perf_counter()
        proc = subprocess.Popen(['python', str(checker)], cwd=a.source / 'java', env=env,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1)
        with (out / 'bundle.log').open('w') as log:
            for line in proc.stdout:
                elapsed = time.perf_counter() - start
                log.write(line)
                if re.search(r'\[INFO\] --- .* @ ', line) or '[INFO] BUILD ' in line:
                    events.append({'seconds': elapsed, 'message': line.strip()})
        rc = proc.wait()
        seconds = time.perf_counter() - start
        result = {'variant': a.variant, 'pair': a.pair, 'temperature': temperature,
                  'returncode': rc, 'bundle_seconds': seconds, 'phase_events': events,
                  'cache_reused': bool(before), 'cache_transfer_bytes': 0,
                  'dependency_download_bytes': None,
                  'note': 'Private local downloads retained between cold/warm; no Actions cache transfer. HTTP transfer bytes are unavailable.'}
        save(out / 'result.json', result)
        if rc:
            raise SystemExit(rc)
        target = a.source / 'java/target'
        classes = files(target / 'test-classes', {'.class'})
        assert bool(classes) == (a.variant == 'baseline'), 'Test compilation policy did not execute as intended'
        assert not list(target.glob('surefire-reports/TEST-*.xml')), 'Bundle lane unexpectedly ran tests'
        save(out / 'test-classes.json', classes)
        artifacts = sorted(target.glob('slackblocks-*.jar')) + sorted(target.glob('slackblocks-*.pom'))
        assert len(artifacts) == 4
        assert len(list(target.glob('slackblocks-*.asc'))) == 4
        signatures = []
        payloads = {}
        verify_start = time.perf_counter()
        for artifact in artifacts:
            signatures.append(cmd(['gpg', '--verify', str(artifact) + '.asc', str(artifact)], env=env).decode())
            shutil.copy2(artifact, out / artifact.name)
            shutil.copy2(str(artifact) + '.asc', out / (artifact.name + '.asc'))
            if artifact.suffix == '.jar':
                with zipfile.ZipFile(artifact) as z:
                    payloads[artifact.name] = {n: sha(z.read(n)) for n in sorted(z.namelist()) if not n.endswith('/')}
            else:
                payloads[artifact.name] = sha(artifact.read_bytes())
        result['signature_seconds'] = time.perf_counter() - verify_start
        (out / 'signatures.log').write_text('\n'.join(signatures))
        shutil.copy2(target / 'central-publishing/central-bundle.zip', out / 'central-bundle.zip')
        save(out / 'artifact-payloads.json', payloads)
        after = files(a.cache / 'maven')
        save(out / 'cache-after.json', after)
        # Self-installed project artifacts are outputs, not external dependencies.
        deps = {n: v for n, v in after.items() if n.endswith(('.jar', '.pom', '.zip'))
                and not n.startswith('repository/io/github/nicklambourne/slackblocks/')}
        save(out / 'dependencies.json', deps)
        result.update(cache_bytes_before=sum(v['bytes'] for v in before.values()),
                      cache_bytes_after=sum(v['bytes'] for v in after.values()),
                      test_class_count=len(classes), test_execution_count=0,
                      signature_count=len(signatures))
        save(out / 'result.json', result)
        print(json.dumps({k: result[k] for k in ['variant', 'pair', 'temperature', 'bundle_seconds', 'test_class_count']}), flush=True)
finally:
    checker.write_text(original)
    subprocess.run(['gpgconf', '--kill', 'gpg-agent'], env=env, check=False)
    assert source == {name: sha((a.source / name).read_bytes()) for name in tracked if name}
