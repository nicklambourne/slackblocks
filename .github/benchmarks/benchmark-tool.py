"""Controlled, isolated pinned-tool cache measurement; never merge this harness."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import time

root=Path(os.environ['SOURCE']).resolve()
out=Path(os.environ['EVIDENCE']).resolve();out.mkdir(parents=True,exist_ok=True)
tool=Path(os.environ['TOOL_ROOT']);cargo_home=Path(os.environ['CARGO_HOME'])
variant=os.environ['VARIANT'];condition=os.environ['CONDITION']
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def size(path):return sum(p.stat().st_size for p in path.rglob('*') if p.is_file()) if path.exists() else 0
def output(*cmd):return subprocess.check_output(cmd,cwd=root,text=True).strip()
record={'variant':variant,'condition':condition,'pair':os.environ['PAIR'],'source':output('git','rev-parse','HEAD'),'python':sys.version,'compiler':output('rustc','+1.99.0','-vV'),'image':os.environ.get('ImageVersion'),'image_os':os.environ.get('ImageOS'),'runner':os.environ['RUNNER_NAME'],'tool_hit':os.environ.get('TOOL_HIT',''),'dependency_hit':os.environ['DEPENDENCY_HIT'],'before':{'tool_bytes':size(tool),'cargo_bytes':size(cargo_home),'target_bytes':size(root/'rust/target')},'phases':[]}
record['inputs']={str(p.relative_to(root)):sha(p) for directory in ['rust','spec'] for p in (root/directory).rglob('*') if p.is_file() and not any(x in p.parts for x in ['target','.git']) and p.suffix!='.pyc'}
(out/'start.json').write_text(json.dumps(record,indent=2))
def run(name,cmd):
 started=time.perf_counter()
 with (out/(name+'.log')).open('w') as log:result=subprocess.run(cmd,cwd=root,text=True,stdout=log,stderr=subprocess.STDOUT)
 phase={'name':name,'seconds':time.perf_counter()-started,'returncode':result.returncode,'command':cmd};record['phases'].append(phase);result.check_returncode()
started=time.perf_counter()
try:
 assert record['source']=='c80ea3fc00f20cb2e34ac481f2a9926223335f47'
 assert record['dependency_hit']==('true' if condition=='warm' else '')
 assert record['tool_hit']==('true' if condition=='warm' and variant=='candidate' else '')
 if condition=='cold':assert record['before']['cargo_bytes']==record['before']['target_bytes']==0
 if variant=='baseline' or condition=='cold':assert record['before']['tool_bytes']==0
 if variant=='baseline' or record['tool_hit']!='true':
  run('install-tool',['cargo','+1.99.0','install','cargo-llvm-cov','--version','0.9.1','--locked','--root',str(tool)])
 else:record['phases'].append({'name':'install-tool','seconds':0,'returncode':0,'cache_hit':True})
 assert output(str(tool/'bin/cargo-llvm-cov'),'llvm-cov','--version')=='cargo-llvm-cov 0.9.1'
 record['binary_sha256']=sha(tool/'bin/cargo-llvm-cov')
 crates=list((cargo_home/'registry/cache').glob('*/cargo-llvm-cov-0.9.1.crate'));assert len(crates)==1
 record['tool_crate_sha256']=sha(crates[0])
 with tarfile.open(crates[0]) as archive:record['tool_lock_sha256']=hashlib.sha256(archive.extractfile('cargo-llvm-cov-0.9.1/Cargo.lock').read()).hexdigest()
 os.environ['PATH']=str(tool/'bin')+os.pathsep+os.environ['PATH']
 # Execute the unchanged checker against the pinned stable compiler in memory.
 checker=root/'rust/bin/check-coverage.py';code=checker.read_text();assert code.count("'+stable'")==1
 wrapper=out/'coverage-pinned.py';wrapper.write_text('import os\nos.environ["RUSTUP_TOOLCHAIN"]="1.99.0"\n'+'exec(compile('+repr(code.replace("'+stable'","'+1.99.0'"))+','+repr(str(checker))+',"exec"), {"__file__":'+repr(str(checker))+', "__name__":"__main__"})\n')
 run('coverage',['python3',str(wrapper)])
 report=json.loads((root/'rust/target/coverage.json').read_text());files=report['data'][0]['files']
 record['coverage']={str(Path(f['filename']).resolve().relative_to(root)):f['summary'] for f in files if Path(f['filename']).resolve().is_relative_to(root)}
 log=(out/'coverage.log').read_text();record['test_names']=sorted(re.findall(r'^test (.+) \.\.\. ok$',log,re.M));record['test_counts']=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)
 assert record['test_counts'] and all(f=='0' and i=='0' for _,f,i in record['test_counts'])
 record['thresholds']=re.findall(r'^(overall|handwritten) (lines|regions): (.+)$',log,re.M);assert len(record['thresholds'])==4
 record['lock_sha256']=sha(root/'rust/Cargo.lock')
 assert not output('git','status','--porcelain','--untracked-files=no')
 record['returncode']=0
except BaseException as error:
 record['returncode']=1;record['error']=repr(error);raise
finally:
 record['seconds']=time.perf_counter()-started;record['after']={'tool_bytes':size(tool),'cargo_bytes':size(cargo_home),'target_bytes':size(root/'rust/target')};(out/'result.json').write_text(json.dumps(record,indent=2))
