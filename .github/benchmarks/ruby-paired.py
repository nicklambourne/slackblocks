"""Same-host confirmation; remote cache transfer remains measured in the retained matrix."""
from pathlib import Path
import json,os,subprocess,sys,time
root=Path.cwd();source=root/'source';pair=os.environ['BENCH_PAIR'];harness=root/'harness/.github/benchmarks/ruby-native.py'
sha=subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip()
assert sha=='3f224a6e89751d00c0391dc52d6578f0e943d0f5'
variants=['baseline','candidate'] if pair!='2' else ['candidate','baseline']
for variant in variants:
 work=Path(os.environ['RUNNER_TEMP'])/('ruby-paired-source-'+variant)
 assert not work.exists();subprocess.run(['git','worktree','add','--detach',str(work),sha],cwd=source,check=True)
 cache=Path(os.environ['RUNNER_TEMP'])/('ruby-paired-cache-'+variant);assert not cache.exists();cache.mkdir()
 env={**os.environ,'BENCH_SOURCE':str(work),'BENCH_MODE':variant,'BUNDLE_WITHOUT':'quality' if variant=='candidate' else '',
      'BUNDLE_PATH':str(cache/'gems'),'BUNDLE_USER_HOME':str(cache/'user'),'BUNDLE_APP_CONFIG':str(cache/'config'),
      'GEM_SPEC_CACHE':str(cache/'spec-cache'),'BUNDLE_DISABLE_SHARED_GEMS':'true','BUNDLE_FROZEN':'true'}
 for temperature in ['cold','warm']:
  output=root/'evidence'/variant/temperature;output.mkdir(parents=True)
  env.update(BENCH_TEMP=temperature,BENCH_EVIDENCE=str(output))
  if temperature=='warm':subprocess.run(['git','restore','--','ruby/Gemfile'],cwd=work,check=True)
  subprocess.run([sys.executable,str(harness),'prepare'],env=env,check=True)
  before=sum(p.stat().st_size for p in cache.rglob('*') if p.is_file())
  if temperature=='cold':assert before==0
  started=time.perf_counter()
  with (output/'bundle-install.log').open('w') as f:
   proc=subprocess.run(['ruby','-S','bundle','_4.0.21_','install','--jobs','4','--retry','2'],cwd=work/'ruby',env=env,stdout=f,stderr=subprocess.STDOUT)
  receipt={'seconds':time.perf_counter()-started,'returncode':proc.returncode,'before_bytes':before,'after_bytes':sum(p.stat().st_size for p in cache.rglob('*') if p.is_file()),'actions_transfer_bytes':0,'network_bytes':None}
  (output/'bundle-install.json').write_text(json.dumps(receipt,indent=2))
  proc.check_returncode();subprocess.run([sys.executable,str(harness),'measure'],env=env,check=True)
