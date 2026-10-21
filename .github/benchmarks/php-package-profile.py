"""Observe existing PHP package checker commands; do not skip any verification."""
from pathlib import Path
import hashlib,json,os,runpy,subprocess,sys,tarfile,tempfile,time,zipfile
source=Path(os.environ['SOURCE']).resolve(); root=Path(os.environ.get('PROFILE_ROOT') or str(Path(os.environ['RUNNER_TEMP'])/('php-package-'+os.environ['GITHUB_RUN_ID']+'-'+os.environ['GITHUB_RUN_ATTEMPT']+'-'+os.environ['PAIR']))).resolve();out=Path(os.environ['EVIDENCE']).resolve()/sys.argv[1];out.mkdir(parents=True)
condition=sys.argv[1];sha='92740b4bf585de5b1a0c7619ce4f082a97d90532';root.mkdir(parents=True,exist_ok=True)
original_run=subprocess.run
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip()==sha
archive=root/(condition+'.tar');original_run(['git','archive','--format=tar','-o',str(archive),sha],cwd=source,check=True)
work=root/condition;work.mkdir()
with tarfile.open(archive) as tar:tar.extractall(work,filter='data')
cache=root/'composer-cache';os.environ.update(COMPOSER_CACHE_DIR=str(cache),COMPOSER_HOME=str(root/'composer-home'),COMPOSER_NO_INTERACTION='1',COMPOSER_NO_AUDIT='1')
cache_before=sum(p.stat().st_size for p in cache.rglob('*') if p.is_file())
assert (cache_before==0) if condition=='cold' else cache_before>=0
record=dict(source=sha,condition=condition,pair=os.environ['PAIR'],runner=os.environ.get('RUNNER_NAME','local'),image=os.environ.get('ImageVersion'),success=False,commands=[],cache_bytes_before=cache_before)
record['php']=subprocess.check_output(['php','-r','echo PHP_VERSION;'],text=True)
record['composer']=subprocess.check_output(['composer','--version','--no-ansi'],text=True).strip()
record['source_inputs']={p:hashlib.sha256(subprocess.check_output(['git','show',sha+':'+p],cwd=source)).hexdigest() for p in subprocess.check_output(['git','ls-files','composer.json','php'],cwd=source,text=True).splitlines()}
start=time.monotonic()
def timed(*args,**kwargs):
 t=time.monotonic();result=None
 try:
  result=original_run(*args,**kwargs);return result
 finally:
  row=dict(argv=[str(a) for a in args[0]],cwd=str(kwargs.get('cwd',Path.cwd())),seconds=time.monotonic()-t,code=None if result is None else result.returncode)
  record['commands'].append(row)
  if result is not None and result.stdout is not None:
   data=result.stdout if isinstance(result.stdout,bytes) else result.stdout.encode();(out/('command-'+str(len(record['commands']))+'.stdout')).write_bytes(data)
subprocess.run=timed
OriginalTemporaryDirectory=tempfile.TemporaryDirectory
class ObservedTemporaryDirectory(OriginalTemporaryDirectory):
 def __exit__(self,*args):
  folder=Path(self.name)
  try:
   z=folder/'slackblocks.zip'
   if z.exists():
    (out/'slackblocks.zip').write_bytes(z.read_bytes())
    with zipfile.ZipFile(z) as package:record['package_files']={n:hashlib.sha256(package.read(n)).hexdigest() for n in package.namelist() if not n.endswith('/')}
   record['consumer_locks']={str(p.relative_to(folder)):json.loads(p.read_text()) for p in folder.glob('*/composer.lock')}
   record['consumer_metadata']={str(p.relative_to(folder)):json.loads(p.read_text()) for p in folder.glob('*/vendor/composer/installed.json')}
  finally:super().__exit__(*args)
tempfile.TemporaryDirectory=ObservedTemporaryDirectory
sys.path.insert(0,str(work/'php/bin'))
try:
 runpy.run_path(str(work/'php/bin/check-package.py'),run_name='__main__')
 record['success']=True
finally:
 record['total_seconds']=time.monotonic()-start
 record['cache_bytes_after']=sum(p.stat().st_size for p in cache.rglob('*') if p.is_file())
 (out/'result.json').write_text(json.dumps(record,indent=2))
