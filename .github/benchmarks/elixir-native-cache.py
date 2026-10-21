"""Measure unchanged native checks with isolated dependency caches; never publish."""
import datetime, hashlib, json, os, shutil, subprocess, sys, time
from pathlib import Path
repo=Path(os.environ['SOURCE']); package=repo/'elixir'; out=Path(os.environ['EVIDENCE']);out.mkdir(exist_ok=True)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()==os.environ['SOURCE_SHA']
for key in ['MIX_ENV','MIX_BUILD_PATH','MIX_BUILD_ROOT','MIX_DEPS_PATH']:assert not os.environ.get(key),key
assert not subprocess.check_output(['git','diff','--name-only',os.environ['BASE_SHA'],'HEAD','--',':(exclude).github/workflows/**'],cwd=repo,text=True).strip()
paths=subprocess.check_output(['git','ls-files'],cwd=repo,text=True).splitlines()
(out/'source-inputs.json').write_text(json.dumps({p:hashlib.sha256(subprocess.check_output(['git','show','HEAD:'+p],cwd=repo)).hexdigest() for p in paths if not p.startswith('.github/workflows/')},indent=2))
cache_paths=[package/'deps',package/'_build',Path(os.environ['HEX_HOME'])/'packages']
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file()) if p.exists() else 0
def beams(own):return {str(p.relative_to(package)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (package/'_build').glob('*/lib/*/ebin/*.beam') if (p.relative_to(package/'_build').parts[2]=='slackblocks')==own}
def command(args,phase):
    executable=shutil.which(args[0]) or args[0];started=time.monotonic()
    with (out/(phase+'.log')).open('w') as log:r=subprocess.run([executable,*args[1:]],cwd=package,env={**os.environ,'SLACKBLOCKS_PYTHON':sys.executable,'BENCH_OBSERVATIONS':str(out/'observations'),'BENCH_PHASE':phase},stdout=log,stderr=subprocess.STDOUT,text=True)
    row=dict(name=phase,command=args,seconds=time.monotonic()-started,code=r.returncode)
    record['phases'].append(row);r.check_returncode()
record={k:os.environ.get(k) for k in ['SOURCE_SHA','BASE_SHA','VARIANT','CONDITION','PAIR','CACHE_HIT','RUNNER_NAME','RUNNER_OS','RUNNER_ARCH','ImageVersion','ImageOS']};record.update(success=False,phases=[],cache_bytes_before=[size(p) for p in cache_paths])
(out/'observations').mkdir(exist_ok=True)
try:
    warm=record['VARIANT']=='candidate' and record['CONDITION']=='warm'
    assert (record['CACHE_HIT']=='true')==warm
    own_before=beams(True);deps_before=beams(False)
    assert bool(own_before)==warm and bool(deps_before)==warm
    command(['mix','local.hex','2.5.1','--force'],'hex')
    command(['mix','deps.get','--check-locked'],'dependencies')
    if record['VARIANT']=='candidate':
        command(['mix','clean'],'clean-project')
        assert not beams(True)
        assert beams(False)==deps_before
    record['project_beams_removed']=len(own_before);record['dependency_beams_preserved']=len(deps_before)
    command(['mix','run','tools/check.exs','--profile','native'],'native-package')
    assert beams(True)
    assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=no'],cwd=repo,text=True)
    shutil.copy2(package/'mix.lock',out/'mix.lock')
    record['success']=True
finally:
    record['cache_bytes_after']=[size(p) for p in cache_paths]
    record['production_phase_seconds']=sum(x['seconds'] for x in record['phases'])
    (out/'result.json').write_text(json.dumps(record,indent=2))
    print(json.dumps(record),flush=True)
