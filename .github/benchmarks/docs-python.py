"""Measurement-only same-host Python docs dependency comparison; never merge."""
import hashlib,importlib.metadata,json,os,subprocess,sys,time
from pathlib import Path
root=Path.cwd();source=root/'source';evidence=root/'evidence';evidence.mkdir();pair=os.environ['BENCH_PAIR'];sha='f4f0dc7da0170efc06cd16fd08b71325e862d26b'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip()==sha
assert sys.version_info[:3]==(3,14,8)
assert subprocess.check_output(['uv','--version'],text=True).startswith('uv 0.13.4 ')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def size(p):return sum(x.stat().st_size for x in p.rglob('*') if x.is_file())
files=subprocess.check_output(['git','ls-files'],cwd=source,text=True).splitlines();inputs={f:digest(source/f) for f in files if (source/f).is_file() and f!='docs/package.json'}
(evidence/'inputs.json').write_text(json.dumps({'source':sha,'files':inputs,'python':sys.version,'uv':subprocess.check_output(['uv','--version'],text=True).strip(),'runner':{k:os.environ.get(k) for k in ('RUNNER_NAME','RUNNER_OS','RUNNER_ARCH','ImageOS','ImageVersion')}},indent=2))
for variant in (['baseline','candidate'] if pair!='2' else ['candidate','baseline']):
 cache=Path(os.environ['RUNNER_TEMP'])/('docs-python-cache-'+variant);assert not cache.exists();cache.mkdir()
 for condition in ('cold','warm'):
  work=Path(os.environ['RUNNER_TEMP'])/f'docs-python-{variant}-{condition}';assert not work.exists();subprocess.run(['git','worktree','add','--detach',str(work),sha],cwd=source,check=True)
  out=evidence/variant/condition;out.mkdir(parents=True);venv=Path(os.environ['RUNNER_TEMP'])/f'docs-python-venv-{variant}-{condition}';assert not venv.exists()
  env={**os.environ,'UV_CACHE_DIR':str(cache),'UV_PROJECT_ENVIRONMENT':str(venv),'UV_PYTHON':sys.executable,'UV_PYTHON_DOWNLOADS':'never'}
  before=size(cache);assert (before==0 if condition=='cold' else before>0)
  options=['--project',str(work/'python'),'--group','docs','--locked','--python',sys.executable]+(['--no-dev'] if variant=='candidate' else [])
  rows=[]
  def phase(name,cmd):
   start=time.perf_counter()
   with (out/(name+'.log')).open('w') as f:proc=subprocess.run(cmd,cwd=work/'docs',env=env,stdout=f,stderr=subprocess.STDOUT)
   rows.append({'name':name,'seconds':time.perf_counter()-start,'returncode':proc.returncode,'command':cmd});(out/'phases.json').write_text(json.dumps(rows,indent=2));proc.check_returncode()
  phase('install',['uv','sync',*options])
  phase('reference',['uv','run',*options,'python','scripts/generate_python_reference.py'])
  phase('legacy_generate',['uv','run',*options,'python','scripts/port_legacy_docs.py','generate'])
  phase('legacy_check',['uv','run',*options,'python','scripts/port_legacy_docs.py','check'])
  phase('legacy_fixture',['uv','run',*options,'python','scripts/port_legacy_docs.py','fixture'])
  assert '114 public symbols and types' in (out/'reference.log').read_text()
  assert 'Verified 22 immutable historical snapshots' in (out/'legacy_check.log').read_text()
  assert '4388d8b9041c9b55e43305906f1d6aafae4fd6c6a83bc5cf7b9d1526656f22bd' in (out/'legacy_fixture.log').read_text()
  manifest={}
  for folder in ('docs/docs/reference/python','docs/versioned_docs','docs/versioned_sidebars','docs/static/img/legacy'):
   manifest.update({str(f.relative_to(work)):digest(f) for f in (work/folder).rglob('*') if f.is_file()})
  for file in ('docs/legacy/manifest.json','docs/versions.json'):manifest[file]=digest(work/file)
  (out/'outputs.json').write_text(json.dumps(manifest,indent=2))
  inventory=subprocess.check_output([str(venv/'bin/python'),'-c','import importlib.metadata as m,hashlib,json; print(json.dumps({d.metadata["Name"]:{"version":d.version,"metadata_sha256":hashlib.sha256(d.read_text("METADATA").encode()).hexdigest()} for d in m.distributions()}))'],cwd=work/'docs',env=env,text=True)
  (out/'dependencies.json').write_text(inventory)
  assert not subprocess.check_output(['git','status','--porcelain'],cwd=work,text=True), 'tracked output changed'
  r={'source':sha,'variant':variant,'condition':condition,'pair':pair,'cache_before_bytes':before,'cache_after_bytes':size(cache),'venv_bytes':size(venv),'actions_transfer_bytes':0,'network_bytes':None,'phases':rows,'complete':True};(out/'result.json').write_text(json.dumps(r,indent=2))
