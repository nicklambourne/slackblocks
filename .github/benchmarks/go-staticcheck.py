"""Measurement-only extra Staticcheck binary cache on the current bounded Go caches."""
import argparse,hashlib,json,os,re,shutil,subprocess,tarfile,time
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--variant',choices=['baseline','candidate'],required=True);p.add_argument('--condition',choices=['cold','warm'],required=True);p.add_argument('--pair',required=True);a=p.parse_args()
a.source=a.source.resolve();a.root=a.root.resolve();a.output=a.output.resolve();a.root.mkdir(exist_ok=True);a.output.mkdir(parents=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
def cap(cmd,cwd=None,env=None):return subprocess.check_output(cmd,cwd=cwd,env=env,text=True).strip()
assert cap(['git','rev-parse','HEAD'],a.source)=='fc44939dffecb7437e41f670e2f9a726de0ed0ac'
tracked=cap(['git','ls-files'],a.source).splitlines();inputs={n:sha(a.source/n) for n in tracked}
archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
work=a.root/'work';assert not work.exists();work.mkdir()
with tarfile.open(archive) as t:t.extractall(work,filter='data')
cache=a.root/'cache';env={**os.environ,'GOPATH':str(cache/'gopath'),'GOMODCACHE':str(cache/'modules'),'GOCACHE':str(cache/'build'),'GOBIN':str(cache/'bin'),'GOTOOLCHAIN':'local'}
analyzer={**env,'GOTOOLCHAIN':'go1.26.9'}
r={'source':cap(['git','rev-parse','HEAD'],a.source),'variant':a.variant,'condition':a.condition,'pair':a.pair,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion'),'go':cap(['go','version']),'inputs':inputs,'dependency_hit':os.getenv('DEPENDENCY_HIT'),'tool_hit':os.getenv('TOOL_HIT'),'cache_bytes_before':{n:size(cache/n) for n in ['modules','build','bin']},'success':False,'phases':[]}
def save():(a.output/'result.json').write_text(json.dumps(r,indent=2)+'\n')
def phase(name,cmd,environment=env,expected=0):
 start=time.perf_counter()
 with (a.output/(name+'.log')).open('w') as f:rc=subprocess.run(cmd,cwd=work/'go',env=environment,stdout=f,stderr=subprocess.STDOUT).returncode
 r['phases'].append({'name':name,'seconds':time.perf_counter()-start,'returncode':rc,'command':cmd});save();assert rc==expected,(name,rc)
try:
 assert r['go']=='go version go1.27.3 linux/amd64'
 assert r['dependency_hit']==('true' if a.condition=='warm' else '')
 assert r['tool_hit']==('true' if a.condition=='warm' and a.variant=='candidate' else '')
 if a.condition=='cold':assert not any(r['cache_bytes_before'].values())
 phase('format',['bash','-c','test -z "$(gofmt -l .)"'])
 phase('tidy',['go','mod','tidy'])
 phase('generate',['go','generate','./...'])
 assert {n:sha(work/n) for n in tracked}==inputs
 phase('release-notes',['bash','-c',"notes=$(printf '## [9.9.9] — 2099-01-01\\n\\nExpected notes.\\n\\n## [9.9.8] — 2098-01-01\\n\\nOlder notes.\\n' | awk -v version=9.9.9 -f ../.github/scripts/extract-go-release-notes.awk); test \"$notes\" = 'Expected notes.'"])
 phase('vet',['go','vet','./...'])
 if a.variant=='candidate':
  if r['tool_hit']!='true':phase('install-tool',['go','install','honnef.co/go/tools/cmd/staticcheck@v0.8.1'],analyzer)
  binary=str(cache/'bin/staticcheck');r['tool_version']=cap([binary,'-version']);assert r['tool_version'].endswith('(0.8.1)')
  r['tool_build_info']=cap(['go','version','-m',binary],env=analyzer).replace(binary,'STATICCHECK')
  assert 'go1.26.9' in r['tool_build_info'] and 'honnef.co/go/tools\tv0.8.1' in r['tool_build_info']
  r['tool_sha256']=sha(Path(binary));cmd=[binary,'./...']
 else:cmd=['go','run','honnef.co/go/tools/cmd/staticcheck@v0.8.1','./...']
 phase('staticcheck',cmd,analyzer)
 phase('tests',['go','test','-json','-count=1','-race','-cover','./...'])
 events=[json.loads(l) for l in (a.output/'tests.log').read_text().splitlines() if l.startswith('{')]
 assert not any(e['Action'] in ['fail','skip'] for e in events)
 r['tests']=sorted((e['Package'],e['Test']) for e in events if e['Action']=='pass' and 'Test' in e);assert len(r['tests'])>=539
 r['coverage']=sorted(m.group(0) for e in events if (m:=re.search(r'coverage: [0-9.]+% of statements',e.get('Output',''))))
 r['modules']=cap(['go','list','-m','all'],cwd=work/'go',env=env)
 assert {n:sha(work/n) for n in tracked}==inputs
 # Untimed negative control proves the restored analyzer still executes.
 bad=work/'go/negative_staticcheck.go';bad.write_text('package slackblocks\nfunc negativeStaticcheck(x int) bool { return x == x }\n')
 phase('negative-control',cmd,analyzer,1)
 assert 'SA4000' in (a.output/'negative-control.log').read_text();bad.unlink()
 r['cache_bytes_after']={n:size(cache/n) for n in ['modules','build','bin']};r['success']=True
finally:save()
shutil.rmtree(work)
