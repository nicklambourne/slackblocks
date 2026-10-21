"""Measure fixed-source Go CI phases; measurement-only, never merge this harness."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

root = Path(os.environ['SOURCE']).resolve()
out = Path(os.environ['EVIDENCE']).resolve()
out.mkdir(parents=True, exist_ok=True)
module = root / 'go'
variant, condition = os.environ['VARIANT'], os.environ['CONDITION']

def output(*cmd, cwd=root, env=None):
    return subprocess.check_output(cmd, cwd=cwd, env=env, text=True).strip()

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def size(path):
    return sum(p.stat().st_size for p in path.rglob('*') if p.is_file()) if path.exists() else 0

cache = {key: Path(os.environ[key]) for key in ['GOMODCACHE', 'GOCACHE']}
initial_sizes = {k: size(p) for k, p in cache.items()}
record = {'source': output('git', 'rev-parse', 'HEAD'), 'variant': variant,
          'condition': condition, 'pair': os.environ['PAIR'], 'lane': os.environ['GO_VERSION'],
          'compiler': output('go', 'version'), 'go_env': json.loads(output('go', 'env', '-json', cwd=module)),
          'runner': os.environ.get('RUNNER_NAME'), 'image': os.environ.get('ImageVersion'),
          'image_os': os.environ.get('ImageOS'), 'python': sys.version,
          'cache_hit': os.environ.get('CACHE_HIT', ''),
          'before': initial_sizes, 'phases': []}
paths = output('git', 'ls-files', 'go', 'spec', 'docs/examples/go',
               'docs/docs/usage/using_blocks.mdx', '.github/scripts/extract-go-release-notes.awk').splitlines()
record['inputs'] = {p: sha(root/p) for p in paths}
(out / 'start.json').write_text(json.dumps(record, indent=2))

def run(name, cmd, cwd=module, env=None):
    start = time.perf_counter()
    with (out / (name + '.log')).open('w') as log:
        result = subprocess.run(cmd, cwd=cwd, env=env, text=True, stdout=log, stderr=subprocess.STDOUT)
    record['phases'].append({'name': name, 'seconds': time.perf_counter()-start,
                             'returncode': result.returncode, 'command': cmd})
    result.check_returncode()

started = time.perf_counter()
try:
    assert record['source'] == os.environ['SOURCE_SHA']
    assert record['compiler'].startswith('go version go' + os.environ['GO_VERSION'] + ' ')
    expected_hit = condition == 'warm' and variant == 'candidate'
    assert (record['cache_hit'] == 'true') == expected_hit
    if not expected_hit:
        assert record['before'] == {'GOMODCACHE': 0, 'GOCACHE': 0}, record['before']
    run('format', ['bash', '-euo', 'pipefail', '-c', 'test -z "$(gofmt -l .)"'])
    run('tidy', ['go', 'mod', 'tidy'])
    run('tidy-guard', ['git', 'diff', '--exit-code', '--', 'go.mod', 'go.sum'])
    run('generate', ['go', 'generate', './...'])
    run('generate-guard', ['git', 'diff', '--exit-code', '--', 'concrete_builders_gen.go', 'limits_gen.go'])
    run('release-notes', ['bash', '-euo', 'pipefail', '-c', '''notes=$(printf '## [9.9.9] — 2099-01-01\\n\\nExpected notes.\\n\\n## [9.9.8] — 2098-01-01\\n\\nOlder notes.\\n' | awk -v version="9.9.9" -f ../.github/scripts/extract-go-release-notes.awk)
        test "$notes" = "Expected notes."'''])
    run('vet', ['go', 'vet', './...'])
    if os.environ['GO_VERSION'] == '1.27.3':
        analyzer_env = dict(os.environ, GOTOOLCHAIN='go1.26.9')
        run('staticcheck', ['go', 'run', 'honnef.co/go/tools/cmd/staticcheck@v0.8.1', './...'], env=analyzer_env)
    run('tests', ['go', 'test', '-count=1', '-race', '-cover', '-coverprofile='+str(out/'coverage.out'), '-json', './...'])
    events = []
    for line in (out/'tests.log').read_text().splitlines():
        if line.startswith('{'):
            events.append(json.loads(line))
    (out/'test-events.json').write_text(json.dumps(events, indent=2))
    assert not [e for e in events if e.get('Action') in ['fail', 'skip']]
    record['test_names'] = sorted((e['Package'], e['Test']) for e in events if e.get('Action') == 'pass' and 'Test' in e)
    assert record['test_names']
    record['package_results'] = [e for e in events if e.get('Action') == 'pass' and 'Test' not in e]
    record['coverage_lines'] = sorted(line for e in events for line in e.get('Output', '').splitlines() if 'coverage:' in line)
    assert record['coverage_lines'] and '(cached)' not in (out/'tests.log').read_text()
    # Deterministic profile body supports exact baseline/candidate coverage parity.
    record['coverage_sha256'] = sha(out/'coverage.out')
    record['generated'] = {p: sha(module/p) for p in ['concrete_builders_gen.go', 'limits_gen.go']}
    record['module_graph'] = output('go', 'list', '-m', 'all', cwd=module).splitlines()
    record['module_zips'] = {str(p.relative_to(cache['GOMODCACHE'])): sha(p) for p in (cache['GOMODCACHE']/'cache/download').rglob('*.zip')}
    assert not output('git', 'status', '--porcelain', '--untracked-files=no')
    record['returncode'] = 0
except BaseException as error:
    record['returncode'] = 1
    record['error'] = repr(error)
    raise
finally:
    record['seconds'] = time.perf_counter()-started
    record['after'] = {k: size(p) for k, p in cache.items()}
    (out/'result.json').write_text(json.dumps(record, indent=2))
