"""Measurement only: bounded Go cache for publishing verification, without release dispatch."""
import argparse,hashlib,json,os,re,shutil,subprocess,tarfile,time
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--pair',type=int,required=True);p.add_argument('--variant',choices=['baseline','candidate'],required=True);p.add_argument('--condition',choices=['cold','warm'],required=True);a=p.parse_args()
for n in ['source','root','output']:setattr(a,n,getattr(a,n).resolve())
a.root.mkdir(exist_ok=True);a.output.mkdir(parents=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
def text(cmd,cwd=None,env=None):return subprocess.check_output(cmd,cwd=cwd,env=env,text=True).strip()
source='fc44939dffecb7437e41f670e2f9a726de0ed0ac';assert text(['git','rev-parse','HEAD'],a.source)==source
assert text(['go','version'])=='go version go1.27.3 linux/amd64'
tracked=text(['git','ls-files'],a.source).splitlines();inputs={n:sha(a.source/n) for n in tracked}
(a.output/'source.json').write_text(json.dumps(inputs,indent=2)+'\n')
archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
work=a.root/'work';work.mkdir()
with tarfile.open(archive) as t:t.extractall(work,filter='data')
archive.unlink();cache=a.root/'cache';before=size(cache);warm=a.variant=='candidate' and a.condition=='warm';assert bool(before)==warm;assert (os.getenv('CACHE_HIT')=='true')==warm
sample={**os.environ,'GOPATH':str(cache/'gopath'),'GOMODCACHE':str(cache/'modules'),'GOCACHE':str(cache/'build'),'GOTOOLCHAIN':'local'}
r={'source':source,'pair':a.pair,'variant':a.variant,'condition':a.condition,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion'),'go':text(['go','version']),'identity':json.loads(text(['go','env','-json','GOVERSION','GOOS','GOARCH','GOAMD64','GOARM','GOEXPERIMENT','CGO_ENABLED'],env=sample)),'cache_bytes_before':before,'cache_hit':os.getenv('CACHE_HIT'),'success':False,'phases':[]}
def save():(a.output/'result.json').write_text(json.dumps(r,indent=2)+'\n')
def run(name,cmd,cwd,output=None):
 start=time.perf_counter()
 with (a.output/(name+'.log')).open('w') as log:
  if output:
   with output.open('w') as out:rc=subprocess.run(cmd,cwd=cwd,env=sample,stdout=out,stderr=log).returncode
  else:rc=subprocess.run(cmd,cwd=cwd,env=sample,stdout=log,stderr=subprocess.STDOUT).returncode
 r['phases'].append({'name':name,'seconds':time.perf_counter()-start,'returncode':rc,'command':cmd});save();assert rc==0,(name,rc)
try:
 run('format',['bash','-c','test -z "$(gofmt -l .)"'],work/'go')
 run('vet',['go','vet','./...'],work/'go')
 cmd=['go','test','-json','-race','-coverprofile='+str(a.output/'coverage.out')]
 if a.variant=='candidate':cmd.append('-count=1')
 cmd.append('./...');run('tests',cmd,work/'go')
 events=[json.loads(l) for l in (a.output/'tests.log').read_text().splitlines() if l.startswith('{')]
 assert not any(e['Action'] in ['fail','skip'] for e in events)
 r['tests']=sorted((e['Package'],e['Test']) for e in events if e['Action']=='pass' and 'Test' in e);assert len(r['tests'])==577,len(r['tests'])
 r['packages']=sorted(e['Package'] for e in events if e['Action']=='pass' and 'Test' not in e)
 assert all(e.get('Elapsed',0)>0 for e in events if e['Action']=='pass' and 'Test' not in e)
 assert not any('(cached)' in e.get('Output','') for e in events)
 r['coverage']=sorted((a.output/'coverage.out').read_text().splitlines());assert r['coverage']
 r['modules']=text(['go','list','-m','all'],work/'go',sample).replace(str(work),'/SOURCE')
 r['cache_bytes_after']={n:size(cache/n) for n in ['modules','build','gopath']}
 assert {n:sha(work/n) for n in tracked}==inputs
 r['success']=True
finally:save()
shutil.rmtree(work)
if a.variant=='baseline':
 subprocess.run(['chmod','-R','u+w',str(cache)],check=True)
 shutil.rmtree(cache)
