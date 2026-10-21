"""Same-host frozen-input TypeScript install comparison; never a product checker."""
import argparse, hashlib, json, os, shutil, subprocess, tarfile, time
from pathlib import Path
p=argparse.ArgumentParser()
for name in ['source','output','cache']:p.add_argument('--'+name,type=Path,required=True)
p.add_argument('--variant',choices=['baseline','candidate'],required=True)
p.add_argument('--pair',type=int,required=True)
a=p.parse_args(); a.source=a.source.resolve(); a.output=a.output.resolve(); a.cache=a.cache.resolve()
a.output.mkdir(parents=True); assert not a.cache.exists(); a.cache.mkdir(parents=True)
def sha(b):return hashlib.sha256(b).hexdigest()
def save(path,value):path.write_text(json.dumps(value,indent=2)+'\n')
def output(cmd,cwd=None):return subprocess.check_output(cmd,cwd=cwd,stderr=subprocess.STDOUT,text=True).strip()
def inventory(root):
 return {str(f.relative_to(root)):{'bytes':f.stat().st_size,'sha256':sha(f.read_bytes())} for f in sorted(root.rglob('*')) if f.is_file() and not f.is_symlink()}
def size(root):return sum(f.stat().st_size for f in root.rglob('*') if f.is_file() and not f.is_symlink())
tracked=output(['git','ls-files'],a.source).splitlines()
source={n:sha((a.source/n).read_bytes()) for n in tracked if n not in ['.github/workflows/typescript.yml','docs/examples/typescript/tsconfig.json']}
save(a.output/'source.json',source)
save(a.output/'inputs.json',{'commit':output(['git','rev-parse','HEAD'],a.source),'variant':a.variant,'pair':a.pair,'node':output(['node','--version']),'pnpm':output(['pnpm','--version']),'python':output(['python','--version']),'runner':os.getenv('RUNNER_NAME'),'image':os.getenv('ImageVersion'),'arch':os.getenv('RUNNER_ARCH'),'run':os.getenv('GITHUB_RUN_ID')})
archive=a.output/'source.tar'
with archive.open('wb') as f:subprocess.run(['git','archive','HEAD'],cwd=a.source,stdout=f,check=True)
env=os.environ.copy();env.update(CI='true',npm_config_cache=str(a.cache/'npm'),XDG_CACHE_HOME=str(a.cache/'xdg'),PNPM_HOME=str(a.cache/'pnpm-home'))
store=a.cache/'store'
for temperature in ['cold','warm']:
 out=a.output/temperature;out.mkdir();work=a.cache/('checkout-'+temperature);work.mkdir()
 with tarfile.open(archive) as t:t.extractall(work,filter='data')
 if a.variant=='baseline':(work/'docs/examples/typescript/tsconfig.json').unlink()
 result={'variant':a.variant,'pair':a.pair,'temperature':temperature,'phases':{},'cache_bytes_before':size(store),'cache_transfer_bytes':0,'network_bytes':None}
 assert bool(result['cache_bytes_before'])==(temperature=='warm')
 def run(name,cmd,cwd=work):
  start=time.perf_counter()
  with (out/(name+'.log')).open('w') as log:r=subprocess.run(cmd,cwd=cwd,env=env,stdout=log,stderr=subprocess.STDOUT)
  result['phases'][name]={'seconds':time.perf_counter()-start,'returncode':r.returncode,'command':cmd}
  save(out/'result.json',result)
  if r.returncode:raise SystemExit(r.returncode)
 started=time.perf_counter()
 cmd=['pnpm']+(['--filter','@nicklambourne/slackblocks...'] if a.variant=='candidate' else [])+['install','--frozen-lockfile','--store-dir',str(store)]
 run('install',cmd)
 assert (work/'pnpm-lock.yaml').read_bytes()==(a.source/'pnpm-lock.yaml').read_bytes()
 result['docs_modules_present']=(work/'docs/node_modules').exists()
 assert result['docs_modules_present']==(a.variant=='baseline')
 run('typecheck',['pnpm','--filter','@nicklambourne/slackblocks','typecheck'])
 run('tests',['pnpm','--filter','@nicklambourne/slackblocks','exec','vitest','run','--reporter=json','--outputFile='+str(out/'tests.json')])
 report=json.loads((out/'tests.json').read_text());assert report['success'] and report['numFailedTests']==0 and report['numPendingTests']==0
 tests=sorted(t['fullName'] for s in report['testResults'] for t in s['assertionResults']);assert len(tests)==484,len(tests)
 save(out/'test-inventory.json',tests);result['tests']=len(tests)
 run('package',['pnpm','--filter','@nicklambourne/slackblocks','check:package'])
 run('pack',['pnpm','--dir','typescript','pack','--pack-destination',str(out)])
 tgz=next(out.glob('*.tgz'))
 with tarfile.open(tgz) as t:payload={m.name:sha(t.extractfile(m).read()) for m in t.getmembers() if m.isfile()}
 save(out/'artifact-payloads.json',payload)
 save(out/'dist.json',inventory(work/'typescript/dist'))
 deps={str(f.relative_to(work/'node_modules/.pnpm')):sha(f.read_bytes()) for pattern in ['*/node_modules/*/package.json','*/node_modules/@*/*/package.json'] for f in (work/'node_modules/.pnpm').glob(pattern) if f.is_file()}
 save(out/'dependency-packages.json',deps)
 consumer=a.cache/('consumer-'+temperature);consumer.mkdir();(consumer/'package.json').write_text('{"private":true}')
 run('consumer-install',['npm','install','--ignore-scripts','--no-audit','--no-fund',str(tgz)],consumer)
 run('consumer-esm',['node','--input-type=module','-e','import {sectionBlock} from "@nicklambourne/slackblocks"; if(sectionBlock({text:"smoke"}).type!=="section") throw Error("bad ESM");'],consumer)
 run('consumer-cjs',['node','-e','const {sectionBlock}=require("@nicklambourne/slackblocks"); if(sectionBlock({text:"smoke"}).type!=="section") throw Error("bad CJS");'],consumer)
 save(out/'consumer-lock.json',json.loads((consumer/'package-lock.json').read_text()))
 result.update(elapsed_seconds=time.perf_counter()-started,cache_bytes_after=size(store),node_modules_bytes=size(work/'node_modules'),tarball_sha256=sha(tgz.read_bytes()),tarball_bytes=tgz.stat().st_size)
 assert {n:sha((work/n).read_bytes()) for n in source}==source
 save(out/'result.json',result);print(json.dumps(result),flush=True)
 shutil.rmtree(work);shutil.rmtree(consumer)
archive.unlink()
