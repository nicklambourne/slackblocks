"""Three hosted pairs; isolated cold dependency caches, then reuse only that pair's caches."""
from pathlib import Path
import datetime
import json
import os
import subprocess
import time

SOURCE = 'c4d3e55997d1015abd8e13d11e607fc00fb61490'
repo = Path.cwd()
harness = repo / '.github/benchmarks'
pair = int(os.environ['BENCH_PAIR'])
base = Path(os.environ['RUNNER_TEMP']) / f'docs-pair-{pair}'
results = repo / 'benchmark-results'
base.mkdir(exist_ok=False)
results.mkdir(exist_ok=False)
records = []

def command(args, cwd, env, dest, phase):
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    begin = time.monotonic()
    with (dest / f'{phase}.log').open('w') as log:
        proc = subprocess.Popen(args, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
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

def size(folder):
    return sum(p.stat().st_size for p in folder.rglob('*') if p.is_file()) if folder.exists() else 0

modes = ['baseline', 'candidate'] if pair % 2 else ['candidate', 'baseline']
# Cold means empty task-owned dependency/application caches. Hosted image/tool caches and
# OS page caches cannot be cleared and are not claimed cold. Warm uses a fresh checkout.
for mode in modes:
    cache = base / f'{mode}-cache'
    cache.mkdir()
    for state in ['cold', 'warm']:
        dest = results / f'{mode}-{state}'
        dest.mkdir()
        work = base / f'{mode}-{state}'
        env = os.environ.copy()
        env.update(UV_CACHE_DIR=str(cache/'uv'), CARGO_HOME=str(cache/'cargo'),
                   GOMODCACHE=str(cache/'gomod'), GOCACHE=str(cache/'gobuild'),
                   MAVEN_OPTS=f'-Dmaven.repo.local={cache / "maven"}',
                   NUGET_PACKAGES=str(cache/'nuget'), COMPOSER_CACHE_DIR=str(cache/'composer'),
                   MIX_HOME=str(cache/'mix'), HEX_HOME=str(cache/'hex'),
                   DOTNET_CLI_HOME=str(cache/'dotnet'), npm_config_cache=str(cache/'npm'),
                   XDG_CACHE_HOME=str(cache/'xdg'), DOTNET_CLI_TELEMETRY_OPTOUT='1',
                   DOCS_BENCH_RESULTS=str(dest), CARGO_NET_OFFLINE='false')
        # Do not share compiled Mix consumers or repository output across fresh checkouts.
        for name in ['MIX_BUILD_PATH', 'MIX_DEPS_PATH', 'CARGO_TARGET_DIR']:
            env.pop(name, None)
        before = size(cache)
        record = dict(pair=pair, mode=mode, state=state, source=SOURCE, cache_bytes_before=before, success=False)
        try:
            command(['git','worktree','add','--detach',str(work),SOURCE],repo,env,dest,'checkout')
            if mode == 'baseline':
                plugin = work / 'docs/plugins/language-search-indexes.cjs'
                source = plugin.read_text()
                old = '      await reuseLanguageSearchIndexes(buildData, variants);'
                assert source.count(old) == 1
                source = source.replace(old, '''      const { postBuildFactory } = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/postBuildFactory");
      for (const { config, filename } of variants) await postBuildFactory(config, filename)(buildData);''')
                plugin.write_text(source)
            command(['git','diff','--binary'],work,env,dest,'source-diff')
            command(['composer','install','--working-dir=php','--no-interaction','--prefer-dist'],work,env,dest,'composer')
            command(['mix','local.hex','2.5.1','--force'],work/'elixir',env,dest,'hex')
            command(['mix','deps.get','--check-locked'],work/'elixir',env,dest,'mix-deps')
            command(['pnpm','install','--frozen-lockfile','--store-dir',str(cache/'pnpm')],work,env,dest,'pnpm')
            profiled = dict(env, NODE_OPTIONS=f'--require={harness / "docs-profile.cjs"}')
            command(['pnpm','--filter','@slackblocks/docs','build'],work,profiled,dest,'build')
            command(['pnpm','--filter','@slackblocks/docs','check:legacy-snapshots'],work,env,dest,'legacy-snapshots')
            command(['pnpm','--filter','@slackblocks/docs','check:release-snapshots'],work,env,dest,'release-snapshots')
            command(['git','diff','--exit-code','--','docs/legacy/manifest.json','docs/versioned_docs','docs/versioned_sidebars','docs/static/img/legacy'],work,env,dest,'frozen-output')
            command(['git','diff','--exit-code','--','pnpm-lock.yaml','python/uv.lock','rust/Cargo.lock','docs/examples/rust/Cargo.lock','php/composer.lock','elixir/mix.lock'],work,env,dest,'dependency-inputs')
            command(['node',str(harness/'docs-manifest.cjs'),str(work),str(dest/'manifest.json')],work,env,dest,'manifest')
            record['success'] = True
        finally:
            record['cache_bytes_after'] = size(cache)
            (dest/'result.json').write_text(json.dumps(record,indent=2))
            records.append(record)
            (results/'runs.json').write_text(json.dumps(records,indent=2))
        # Only this benchmark's successful disposable checkout is removed, after evidence capture.
        command(['git','worktree','remove','--force',str(work)],repo,env,dest,'cleanup')

reference = json.loads((results/'baseline-cold/manifest.json').read_text())
for row in records:
    actual = json.loads((results/f'{row["mode"]}-{row["state"]}'/'manifest.json').read_text())
    assert actual['generated'] == reference['generated'], 'Generated references/snapshots differ'
    assert actual['files'].keys() == reference['files'].keys(), 'Published filenames differ'
    for name, expected in reference['files'].items():
        observed = actual['files'][name]
        if 'canonical' in expected:
            for field in ['canonical','queries','documents']:
                assert observed[field] == expected[field], (row, name, field)
        else:
            assert observed == expected, (row, name)
(results/'parity.json').write_text(json.dumps({'passed':True,'files':len(reference['files']),'generated':len(reference['generated']),'runs':len(records)}))
