"""Compare the unchanged Java verifier with the CI-only formatter skip."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
import xml.etree.ElementTree as ET
import zipfile

p = argparse.ArgumentParser()
p.add_argument('--source', type=Path, required=True)
p.add_argument('--output', type=Path, required=True)
p.add_argument('--cache', type=Path, required=True)
p.add_argument('--variant', choices=['baseline', 'candidate'], required=True)
p.add_argument('--pair', type=int, required=True)
p.add_argument('--jdk', required=True)
a = p.parse_args()
a.source = a.source.resolve()
a.output.mkdir(parents=True, exist_ok=True)
assert not a.cache.exists(), 'Use a fresh bounded cache namespace for each policy/pair/JDK'
a.cache.mkdir(parents=True)

def sha(data):
    return hashlib.sha256(data).hexdigest()

def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

def files(root, suffixes=None):
    return {str(f.relative_to(root)): {'sha256': sha(f.read_bytes()), 'bytes': f.stat().st_size}
            for f in sorted(root.rglob('*')) if f.is_file()
            and (suffixes is None or f.suffix in suffixes)}

def output(command, **kwargs):
    return subprocess.check_output(command, stderr=subprocess.STDOUT, **kwargs).decode()

tracked = output(['git', 'ls-files', '-z'], cwd=a.source).split('\0')
save(a.output / 'source.json', {n: sha((a.source / n).read_bytes()) for n in tracked if n})
save(a.output / 'inputs.json', {
    'commit': output(['git', 'rev-parse', 'HEAD'], cwd=a.source).strip(),
    'jdk': a.jdk, 'variant': a.variant, 'pair': a.pair,
    'java': output(['java', '-version']), 'python': output(['python', '--version']),
    'image_os': os.environ.get('ImageOS'), 'image_version': os.environ.get('ImageVersion'),
    'runner_name': os.environ.get('RUNNER_NAME'), 'runner_arch': os.environ.get('RUNNER_ARCH'),
    'run_id': os.environ.get('GITHUB_RUN_ID'), 'run_attempt': os.environ.get('GITHUB_RUN_ATTEMPT')})
env = os.environ.copy()
env.update(MAVEN_USER_HOME=str(a.cache / 'maven'),
           MAVEN_OPTS='-Dmaven.repo.local=' + str(a.cache / 'maven/repository'))
for temperature in ['cold', 'warm']:
    out = a.output / temperature
    out.mkdir()
    before = files(a.cache / 'maven')
    save(out / 'cache-before.json', before)
    assert bool(before) == (temperature == 'warm')
    command = ['./mvnw', '-B', '-ntp', 'clean', 'verify']
    if a.variant == 'candidate':
        command.append('-Dspotless.skip=true')
    events = []
    start = time.perf_counter()
    proc = subprocess.Popen(command, cwd=a.source / 'java', env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1)
    with (out / 'verify.log').open('w') as log:
        for line in proc.stdout:
            elapsed = time.perf_counter() - start
            log.write(line)
            if re.search(r'\[INFO\] --- .* @ ', line) or '[INFO] BUILD ' in line:
                events.append({'seconds': elapsed, 'message': line.strip()})
    rc = proc.wait()
    result = {'jdk': a.jdk, 'variant': a.variant, 'pair': a.pair, 'temperature': temperature,
              'returncode': rc, 'verify_seconds': time.perf_counter() - start,
              'phase_events': events, 'command': command,
              'cache_reused': bool(before), 'cache_bytes_before': sum(v['bytes'] for v in before.values()),
              'cache_transfer_bytes': 0, 'dependency_download_bytes': None}
    save(out / 'result.json', result)
    if rc:
        raise SystemExit(rc)
    target = a.source / 'java/target'
    reports = sorted((target / 'surefire-reports').glob('TEST-*.xml'))
    assert reports
    inventory = []
    for report in reports:
        suite = ET.parse(report).getroot()
        assert int(suite.attrib['failures']) == int(suite.attrib['errors']) == int(suite.attrib['skipped']) == 0
        inventory.extend((t.attrib.get('classname', ''), t.attrib['name']) for t in suite.iter('testcase'))
    assert len(inventory) == 559, len(inventory)
    save(out / 'test-inventory.json', sorted(inventory))
    save(out / 'classes.json', files(target / 'classes', {'.class'}))
    save(out / 'test-classes.json', files(target / 'test-classes', {'.class'}))
    artifacts = sorted(target.glob('slackblocks-*.jar'))
    assert len(artifacts) == 3
    payloads = {}
    for artifact in artifacts:
        with zipfile.ZipFile(artifact) as z:
            payloads[artifact.name] = {n: sha(z.read(n)) for n in sorted(z.namelist()) if not n.endswith('/')}
    save(out / 'artifact-payloads.json', payloads)
    save(out / 'artifacts.json', {f.name: {'sha256': sha(f.read_bytes()), 'bytes': f.stat().st_size} for f in artifacts})
    after = files(a.cache / 'maven')
    save(out / 'cache-after.json', after)
    save(out / 'dependencies.json', {n:v for n,v in after.items() if n.endswith(('.jar','.pom','.zip'))})
    result.update(test_count=len(inventory), cache_bytes_after=sum(v['bytes'] for v in after.values()))
    save(out / 'result.json', result)
    subprocess.run(['git','diff','--exit-code'],cwd=a.source,check=True)
    print(json.dumps({k:result[k] for k in ['jdk','variant','pair','temperature','verify_seconds','test_count']}),flush=True)
