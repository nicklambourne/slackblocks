"""Compare fixed-source complete quality runs, varying only restored PLTs."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time

repo = Path(os.environ['SOURCE'])
out = Path(os.environ['EVIDENCE'])
out.mkdir(exist_ok=True)
(out / 'observations').mkdir(exist_ok=True)
package = repo / 'elixir'
plt = package / '_build/dev/plts'
expected = os.environ['SOURCE_SHA']
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=repo, text=True).strip() == expected
for key in ['MIX_ENV', 'MIX_BUILD_PATH', 'MIX_BUILD_ROOT', 'MIX_DEPS_PATH', 'SLACKBLOCKS_PLT_PATH']:
    assert not os.environ.get(key), key
env = {**os.environ, 'SLACKBLOCKS_PYTHON': sys.executable,
       'BENCH_OBSERVATIONS': str(out / 'observations')}

def files(path):
    return {str(p.relative_to(path)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(path.rglob('*')) if p.is_file()}

def size(path):
    return sum(p.stat().st_size for p in path.rglob('*') if p.is_file())

def command(args, phase):
    begin = time.monotonic()
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (out / f'{phase}.log').open('w') as log:
        proc = subprocess.Popen(args, cwd=package, env={**env, 'BENCH_PHASE': phase},
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        for line in proc.stdout:
            log.write(f'{datetime.datetime.now(datetime.timezone.utc).isoformat()} {line}')
            log.flush()
        code = proc.wait()
    row = dict(phase=phase, command=args, elapsed_s=time.monotonic()-begin, started_at=start, returncode=code)
    with (out / 'phases.jsonl').open('a') as log:
        log.write(json.dumps(row) + '\n')
    print(json.dumps(row), flush=True)
    if code:
        raise subprocess.CalledProcessError(code, args)

record = {key: os.environ.get(key) for key in ['SOURCE_SHA', 'CONDITION', 'VARIANT', 'PAIR', 'CACHE_HIT', 'RUNNER_OS', 'RUNNER_ARCH', 'RUNNER_NAME', 'ImageVersion', 'ImageOS']}
record.update(success=False, plt_bytes_before=size(plt), plt_files_before=files(plt))
try:
    warm = record['CONDITION'] == 'warm' and record['VARIANT'] == 'candidate'
    assert (record['CACHE_HIT'] == 'true') == warm
    assert (record['plt_bytes_before'] > 0) == warm
    source_paths = subprocess.check_output(['git', 'ls-files', 'elixir', 'spec', '.github/scripts', 'docs/docs/changelog.mdx'], cwd=repo, text=True).splitlines()
    (out / 'source-inputs.json').write_text(json.dumps({p: hashlib.sha256((repo / p).read_bytes()).hexdigest() for p in source_paths}, indent=2))
    command(['mix', 'local.hex', '2.5.1', '--force'], 'hex')
    command(['mix', 'deps.get', '--check-locked'], 'dependencies')
    command(['mix', 'run', 'tools/check.exs', '--profile', 'ci-quality'], 'quality')
    for version in ['2.4.0', '3.0.0']:
        env['SLACKBLOCKS_DECIMAL'] = f'== {version}'
        command([sys.executable, 'tools/package_check.py'], f'decimal-{version}')
    env.pop('SLACKBLOCKS_DECIMAL')
    command(['git', 'diff', '--exit-code'], 'generated-guard')
    shutil.copy2(package / 'mix.lock', out / 'mix.lock')
    shutil.copy2(package / 'cover/summary.json', out / 'coverage-summary.json')
    log = (out / 'quality.log').read_text()
    (out / 'test-inventory.json').write_text(json.dumps(re.findall(r'[^\n]*(?:\d+ tests|\d+ doctests|\d+ properties|mutation guards passed)[^\n]*', log), indent=2))
    (out / 'generated.json').write_text(json.dumps(files(package / 'lib'), indent=2))
    command(['mix', 'hex.build', '--output', str(out / 'slackblocks.tar')], 'artifact-evidence')
    record['success'] = True
finally:
    record.update(plt_bytes_after=size(plt), plt_files_after=files(plt))
    (out / 'result.json').write_text(json.dumps(record, indent=2))
