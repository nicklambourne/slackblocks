"""Measure the same fixed source with only the full/ci-quality dispatch varied."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import re
import subprocess
import sys
import time

SOURCE = 'f47a72f366d4e11017e04259247ada9e60cc5217'
repo = Path.cwd()
pair = int(os.environ['BENCH_PAIR'])
base = Path(os.environ['RUNNER_TEMP']) / f'elixir-pair-{pair}'
results = repo / 'benchmark-results'
base.mkdir(exist_ok=False)
results.mkdir(exist_ok=False)
records = []


def size(folder):
    return sum(p.stat().st_size for p in folder.rglob('*') if p.is_file())


def command(args, cwd, env, dest, phase):
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    begin = time.monotonic()
    with (dest / f'{phase}.log').open('w') as log:
        proc = subprocess.Popen(args, cwd=cwd, env={**env, 'BENCH_PHASE': phase}, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        for line in proc.stdout:
            log.write(f'{datetime.datetime.now(datetime.timezone.utc).isoformat()} {line}')
            log.flush()
        code = proc.wait()
    entry = dict(phase=phase, args=args, started_at=started, elapsed_s=time.monotonic()-begin, returncode=code)
    with (dest / 'phases.jsonl').open('a') as log:
        log.write(json.dumps(entry)+'\n')
    print(json.dumps(entry), flush=True)
    if code:
        raise subprocess.CalledProcessError(code, args)


for mode in (['baseline', 'candidate'] if pair % 2 else ['candidate', 'baseline']):
    cache = base / f'{mode}-cache'
    cache.mkdir()
    for state in ['cold', 'warm']:
        dest = results / f'{mode}-{state}'
        dest.mkdir()
        (dest / 'observations').mkdir()
        work = base / f'{mode}-{state}'
        temporary = base / f'{mode}-{state}-temporary'
        temporary.mkdir()
        env = {**os.environ, 'MIX_HOME': str(cache/'mix'), 'HEX_HOME': str(cache/'hex'),
               'REBAR_CACHE_DIR': str(cache/'rebar'), 'XDG_CACHE_HOME': str(cache/'xdg'),
               'TMPDIR': str(temporary), 'SLACKBLOCKS_PYTHON': sys.executable,
               'PYTHONPATH': str(repo/'.github/benchmarks'),
               'BENCH_OBSERVATIONS': str(dest/'observations')}
        for key in ['MIX_BUILD_PATH','MIX_BUILD_ROOT','MIX_DEPS_PATH','MIX_ENV','SLACKBLOCKS_PLT_PATH','REBAR_BASE_DIR']:
            env.pop(key, None)
        record = dict(pair=pair, mode=mode, state=state, source=SOURCE,
                      cache_bytes_before=size(cache), success=False)
        try:
            command(['git','worktree','add','--detach',str(work),SOURCE],repo,env,dest,'checkout')
            package = work/'elixir'
            command(['mix','local.hex','2.5.1','--force'],package,env,dest,'hex')
            command(['mix','deps.get','--check-locked'],package,env,dest,'dependencies')
            command(['mix','run','tools/check.exs','--profile', 'full' if mode == 'baseline' else 'ci-quality'],package,env,dest,'quality')
            for version in ['2.4.0','3.0.0']:
                command([sys.executable,'tools/package_check.py'],package,{**env,'SLACKBLOCKS_DECIMAL':f'== {version}'},dest,f'decimal-{version}')
            command(['git','diff','--exit-code'],work,env,dest,'source-and-generated-output')
            (dest/'mix.lock').write_bytes((package/'mix.lock').read_bytes())
            # Native test totals and all phase timings are retained verbatim for inventory comparison.
            log = (dest/'quality.log').read_text()
            (dest/'test-inventory.json').write_text(json.dumps(re.findall(r'[^\n]*(?:\d+ tests|\d+ doctests|\d+ properties|coverage|mutation guards passed)[^\n]*',log),indent=2))
            command(['mix','hex.build','--output',str(dest/'slackblocks.tar')],package,env,dest,'artifact-evidence')
            record['lock_sha256'] = hashlib.sha256((package/'mix.lock').read_bytes()).hexdigest()
            record['success'] = True
        finally:
            record['cache_bytes_after'] = size(cache)
            (dest/'result.json').write_text(json.dumps(record,indent=2))
            records.append(record)
            (results/'runs.json').write_text(json.dumps(records,indent=2))
        command(['git','worktree','remove','--force',str(work)],repo,env,dest,'cleanup')

assert len(records) == 4 and all(row['success'] for row in records)
assert len({row['lock_sha256'] for row in records}) == 1
(results/'completed.json').write_text(json.dumps({'runs':4, 'same_source_and_root_lock':True,
    'cache_scope':'Empty task-owned dependency caches for cold; fresh checkout for every run; same-mode dependency cache for warm. PLTs and compiled project/consumer output are not reused. Hosted image/OS caches are uncontrolled.',
    'remaining_review':'Compare test inventories, observed consumer locks/artifacts and dedicated same-commit required CI jobs before accepting results.'},indent=2))
