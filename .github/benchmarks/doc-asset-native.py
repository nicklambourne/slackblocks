"""Measurement-only: bound the avoided minimum native-lane work for site assets."""
import hashlib,importlib.util,json,os,subprocess,time
from pathlib import Path
source=Path(os.environ['SOURCE']).resolve();out=Path(os.environ['EVIDENCE']).resolve();out.mkdir(parents=True,exist_ok=True)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip()==os.environ['SOURCE_SHA']
spec=importlib.util.spec_from_file_location('filters',Path(__file__).with_name('doc-asset-filters.py'));filters=importlib.util.module_from_spec(spec);spec.loader.exec_module(filters);filters.ROOT=source;filters.OUT=out
lane=os.environ['LANE'];pair=int(os.environ['PAIR'])
files=[{'filename':'docs/src/css/custom.css'},{'filename':'docs/static/img/logo.svg'}]
path=f'.github/workflows/{lane}.yml'
bodies={'baseline':filters.extract(subprocess.check_output(['git','show',filters.BASE+':'+path],cwd=source,text=True)),'candidate':filters.extract((source/path).read_text())}
inputs={p:hashlib.sha256(subprocess.check_output(['git','show','HEAD:'+p],cwd=source)).hexdigest() for p in subprocess.check_output(['git','ls-files'],cwd=source,text=True).splitlines() if not p.startswith('.github/workflows/')}
# Product changes are workflow-only. Native sources/dependencies are identical.
assert not subprocess.check_output(['git','diff','--name-only',filters.BASE,'HEAD','--',':(exclude).github/workflows/**'],cwd=source,text=True).strip()
(out/'source-inputs.json').write_text(json.dumps(inputs,indent=2))
(out/'filter-bodies.json').write_text(json.dumps(bodies,indent=2))
cache_paths={k:Path(os.environ[k]) for k in (['CARGO_HOME'] if lane=='rust' else ['HEX_HOME','MIX_HOME'])}
def size(p): return sum(f.stat().st_size for f in p.rglob('*') if f.is_file()) if p.exists() else 0
def run(name,cmd,record,folder,cwd):
    started=time.perf_counter()
    with (folder/(name+'.log')).open('w') as log:r=subprocess.run(cmd,cwd=cwd,text=True,stdout=log,stderr=subprocess.STDOUT)
    record['phases'].append(dict(name=name,command=cmd,seconds=time.perf_counter()-started,code=r.returncode));r.check_returncode()
for condition in ['cold','warm']:
    order=['baseline','candidate'] if (pair+(condition=='warm'))%2 else ['candidate','baseline']
    for variant in order:
        folder=out/(condition+'-'+variant);folder.mkdir()
        record=dict(source=os.environ['SOURCE_SHA'],baseline=filters.BASE,lane=lane,pair=pair,condition=condition,variant=variant,phases=[],runner=os.environ.get('RUNNER_NAME'),image=os.environ.get('ImageVersion'),cache_bytes_before={k:size(p) for k,p in cache_paths.items()},success=False)
        relevant,elapsed=filters.run_filter(bodies[variant],files);record['classification_seconds']=elapsed;record['relevant']=relevant
        assert relevant==(variant=='baseline')
        try:
            if relevant and lane=='rust':
                run('unit-api',['cargo','+1.85.0','test','--manifest-path','rust/Cargo.toml','--workspace','--all-targets','--locked'],record,folder,source)
                run('doctests',['cargo','+1.85.0','test','--manifest-path','rust/Cargo.toml','--workspace','--doc','--locked'],record,folder,source)
                run('api-checkpoint',['cargo','+1.85.0','run','--manifest-path','rust/Cargo.toml','--example','api_checkpoint','--locked'],record,folder,source)
                run('integration',['cargo','+1.85.0','test','--manifest-path','rust/integrations/Cargo.toml','--all-targets','--locked'],record,folder,source)
            elif relevant:
                os.environ['PACKAGE_EVIDENCE']=str(folder/'package')
                run('deps',['mix','deps.get','--check-locked'],record,folder,source/'elixir')
                run('native-and-package',['mix','run','tools/check.exs','--profile','native'],record,folder,source/'elixir')
            record['success']=True
            assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=no'],cwd=source,text=True)
        finally:
            record['scoped_seconds']=elapsed+sum(p['seconds'] for p in record['phases'])
            record['cache_bytes_after']={k:size(p) for k,p in cache_paths.items()}
            record['cache_transfer_bytes']=0
            record['cache_note']='Private per-run/attempt/pair local downloads and build state; no remote cache restore/save. Warm repeats retain that state. Candidate executes no native work.'
            (folder/'result.json').write_text(json.dumps(record,indent=2))
