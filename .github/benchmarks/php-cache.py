"""Never-merge Composer download-cache comparison; fresh vendors in every sample."""
import argparse,hashlib,json,os,shutil,subprocess,tarfile,time,xml.etree.ElementTree as ET
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--pair',type=int,required=True);p.add_argument('--variant',choices=['baseline','candidate'],required=True);p.add_argument('--condition',choices=['cold','warm'],required=True);a=p.parse_args()
a.source=a.source.resolve();a.root=a.root.resolve();a.output=a.output.resolve();a.output.mkdir(parents=True);a.root.mkdir(exist_ok=True)
def cap(cmd,cwd=None):return subprocess.check_output(cmd,cwd=cwd,stderr=subprocess.STDOUT,text=True).strip()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
def save(p,obj):p.write_text(json.dumps(obj,indent=2)+'\n')
source=cap(['git','rev-parse','HEAD'],a.source);assert source=='fc44939dffecb7437e41f670e2f9a726de0ed0ac'
inputs={n:sha(a.source/n) for n in cap(['git','ls-files'],a.source).splitlines()};save(a.output/'source.json',inputs)
work=a.root/'work';assert not work.exists();work.mkdir();archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
with tarfile.open(archive) as t:t.extractall(work,filter='data')
archive.unlink();cache=a.root/'downloads';before=size(cache)
assert before==0 if (a.variant=='baseline' or a.condition=='cold') else before>0
assert (os.getenv('CACHE_HIT')=='true')==(a.variant=='candidate' and a.condition=='warm')
env={**os.environ,'COMPOSER_HOME':str(a.root/'home'),'COMPOSER_CACHE_DIR':str(cache),'COMPOSER_NO_INTERACTION':'1','XDEBUG_MODE':'coverage'}
r={'source':source,'variant':a.variant,'condition':a.condition,'pair':a.pair,'php':cap(['php','-v']),'composer':cap(['composer','--version']),'python':cap(['python','--version']),'runner':os.getenv('RUNNER_NAME'),'image':os.getenv('ImageVersion'),'cache_hit':os.getenv('CACHE_HIT'),'cache_bytes_before':before,'phases':[],'success':False}
def run(name,cmd):
 start=time.perf_counter()
 with (a.output/(name+'.log')).open('w') as f:rc=subprocess.run(cmd,cwd=work/'php',env=env,stdout=f,stderr=subprocess.STDOUT).returncode
 r['phases'].append({'name':name,'command':cmd,'seconds':time.perf_counter()-start,'returncode':rc});save(a.output/'result.json',r);assert rc==0,(name,rc)
try:
 run('install',['composer','install','--no-interaction','--prefer-dist','-v'])
 run('validate',['composer','validate','--strict'])
 run('generator',['python','generator/generate.py','--check'])
 run('format',['composer','format:check'])
 run('analysis',['composer','analyse'])
 run('generator-tests',['python','generator/test_generate.py'])
 run('type-contracts',['python','bin/check-types.py'])
 run('tests-coverage',['php','vendor/bin/phpunit','--log-junit',str(a.output/'junit.xml'),'--coverage-clover',str(a.output/'coverage.xml')])
 run('coverage-gate',['python','bin/check-coverage.py',str(a.output/'coverage.xml')])
 assert '6 negative PHP type contracts verified' in (a.output/'type-contracts.log').read_text()
 junit=ET.parse(a.output/'junit.xml');tests=[]
 for c in junit.iter('testcase'):
  assert not any(c.find(tag) is not None for tag in ['error','failure','skipped'])
  tests.append({k:c.get(k) for k in ['class','name','assertions']})
 assert tests;save(a.output/'tests.json',sorted(tests,key=lambda v:(v['class'] or '',v['name'])))
 coverage=[]
 for f in ET.parse(a.output/'coverage.xml').iter('file'):
  rel=str(Path(f.attrib['name']).relative_to(work));coverage.append({'file':rel,'metrics':dict(f.find('metrics').attrib),'lines':[{k:l.get(k) for k in ['num','type','count']} for l in f.findall('line')]})
 save(a.output/'coverage.json',sorted(coverage,key=lambda v:v['file']))
 save(a.output/'vendor-files.json',{str(f.relative_to(work/'php/vendor')):sha(f) for f in (work/'php/vendor').rglob('*') if f.is_file() and not f.is_symlink()})
 save(a.output/'installed.json',json.loads((work/'php/vendor/composer/installed.json').read_text()))
 assert {n:sha(work/n) for n in inputs}==inputs
 r.update(success=True,tests=len(tests),cache_bytes_after=size(cache),download_zip_bytes=size(cache/'files'),vendor_bytes=size(work/'php/vendor'))
finally:save(a.output/'result.json',r)
shutil.rmtree(work)
# Simulate a fresh CI job. Keep only candidate download files for Actions transfer.
if (a.root/'home').exists():shutil.rmtree(a.root/'home')
if a.variant=='baseline':shutil.rmtree(cache)
