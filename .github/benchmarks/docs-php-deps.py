"""Compare only documentation PHP dependency policy, retaining every PHP docs consumer."""
import argparse,hashlib,json,os,shutil,subprocess,tarfile,time
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','output','cache']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--variant',choices=['baseline','candidate'],required=True);p.add_argument('--pair',type=int,required=True);a=p.parse_args()
a.source=a.source.resolve();a.output=a.output.resolve();a.cache=a.cache.resolve();a.output.mkdir(parents=True);assert not a.cache.exists();a.cache.mkdir(parents=True)
def sha(b):return hashlib.sha256(b).hexdigest()
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def output(c,cwd=None):return subprocess.check_output(c,cwd=cwd,stderr=subprocess.STDOUT,text=True).strip()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink())
tracked=output(['git','ls-files'],a.source).splitlines();sources={n:sha((a.source/n).read_bytes()) for n in tracked if n!='.github/workflows/docs.yml'};save(a.output/'source.json',sources)
save(a.output/'inputs.json',{'commit':output(['git','rev-parse','HEAD'],a.source),'variant':a.variant,'pair':a.pair,'php':output(['php','-v']),'composer':output(['composer','--version']),'node':output(['node','--version']),'python':output(['python','--version']),'runner':os.getenv('RUNNER_NAME'),'image':os.getenv('ImageVersion'),'run':os.getenv('GITHUB_RUN_ID')})
archive=a.output/'source.tar'
with archive.open('wb') as f:subprocess.run(['git','archive','HEAD'],cwd=a.source,stdout=f,check=True)
env=os.environ.copy();env.update(COMPOSER_HOME=str(a.cache/'home'),COMPOSER_CACHE_DIR=str(a.cache/'downloads'),COMPOSER_NO_INTERACTION='1',COMPOSER_NO_AUDIT='1')
for temp in ['cold','warm']:
 out=a.output/temp;out.mkdir();work=a.cache/('checkout-'+temp);work.mkdir()
 with tarfile.open(archive) as t:t.extractall(work,filter='data')
 r={'variant':a.variant,'pair':a.pair,'temperature':temp,'phases':{},'cache_bytes_before':size(a.cache/'downloads'),'cache_transfer_bytes':0,'network_bytes':None}
 def run(name,cmd):
  start=time.perf_counter()
  with (out/(name+'.log')).open('w') as f:result=subprocess.run(cmd,cwd=work,env=env,stdout=f,stderr=subprocess.STDOUT)
  r['phases'][name]={'seconds':time.perf_counter()-start,'returncode':result.returncode,'command':cmd};save(out/'result.json',r)
  if result.returncode:raise SystemExit(result.returncode)
 start=time.perf_counter()
 run('install',['composer','install','--working-dir=php','--no-interaction','--prefer-dist']+(['--no-dev'] if a.variant=='candidate' else []))
 run('reflection',['php','docs/scripts/php_reference_adapter.php'])
 reflection=json.loads((out/'reflection.log').read_text());save(out/'reflection.json',reflection)
 run('reference',['node','docs/scripts/generate_php_reference.mjs'])
 assert 'Generated 119 PHP public types' in (out/'reference.log').read_text()
 run('guide-examples',['python','docs/scripts/check_php_guide_snippets.py'])
 assert 'Executed 41 PHP' in (out/'guide-examples.log').read_text() and '3 sending examples' in (out/'guide-examples.log').read_text()
 run('multiselect',['php','docs/examples/multiselect-compat/php.php','php/vendor/autoload.php'])
 payloads=[json.loads(line) for line in (out/'multiselect.log').read_text().splitlines()];assert len(payloads)==15;save(out/'payloads.json',payloads)
 assert len({(r['family'],r['parent']) for r in payloads})==15
 docs=work/'docs/docs/reference/php';save(out/'generated-docs.json',{str(f.relative_to(docs)):sha(f.read_bytes()) for f in sorted(docs.rglob('*')) if f.is_file()})
 installed=json.loads((work/'php/vendor/composer/installed.json').read_text());save(out/'installed.json',installed)
 count=len(installed['packages']);assert (count==0)==(a.variant=='candidate')
 assert (work/'php/composer.lock').read_bytes()==(a.source/'php/composer.lock').read_bytes()
 assert {n:sha((work/n).read_bytes()) for n in sources}==sources
 r.update(elapsed_seconds=time.perf_counter()-start,production_command_seconds=sum(v['seconds'] for v in r['phases'].values()),cache_bytes_after=size(a.cache/'downloads'),vendor_bytes=size(work/'php/vendor'),installed_packages=count,types=119,examples=41,sending_syntax_checks=3,payloads=15)
 save(out/'result.json',r);print(json.dumps(r),flush=True);shutil.rmtree(work)
archive.unlink()
