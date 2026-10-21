"""Measurement only: remove an unused docs rustfmt component, preserve both compilers."""
import hashlib,json,os,shutil,subprocess,time
from pathlib import Path
root=Path(os.environ['RUNNER_TEMP'])/'docs-rustfmt-comparison';root.mkdir()
out=Path('evidence');out.mkdir();pair=int(os.environ['BENCH_PAIR'])
source='fc44939dffecb7437e41f670e2f9a726de0ed0ac'
def cap(cmd,env=None):return subprocess.check_output(cmd,env=env,text=True).strip()
def sha(f):return hashlib.sha256(f.read_bytes()).hexdigest()
def manifest(path):return {str(f.relative_to(path)):{'bytes':f.stat().st_size,'sha256':sha(f)} for f in sorted(path.rglob('*')) if f.is_file() and not f.is_symlink()}
def size(path):return sum(f.stat().st_size for f in path.rglob('*') if f.is_file() and not f.is_symlink())
assert cap(['git','-C','source','rev-parse','HEAD'])==source
inputs={n:sha(Path('source')/n) for n in cap(['git','-C','source','ls-files']).splitlines()};(out/'source.json').write_text(json.dumps(inputs,indent=2))
original=Path(os.environ.get('RUSTUP_HOME',str(Path.home()/'.rustup')))
assert original.is_dir();initial=manifest(original);(out/'initial-installed-files.json').write_text(json.dumps(initial,indent=2))
assert not any('1.85.0' in p for p in initial)
initial_versions=cap(['rustup','toolchain','list']);(out/'initial-toolchains.txt').write_text(initial_versions)
rows=[]
for variant in (['candidate','baseline'] if pair==2 else ['baseline','candidate']):
 private=root/variant;private.mkdir();rustup=private/'rustup';shutil.copytree(original,rustup,symlinks=True)
 assert manifest(rustup)==initial
 env={**os.environ,'RUSTUP_HOME':str(rustup),'CARGO_HOME':str(private/'cargo'),'RUSTUP_AUTO_INSTALL':'0'}
 for condition in ['cold','warm']:
  d=out/(variant+'-'+condition);d.mkdir();before=size(private)
  r={'pair':pair,'variant':variant,'condition':condition,'runner':os.environ['RUNNER_NAME'],'image':os.environ.get('ImageVersion'),'source':source,'initial_toolchains':initial_versions,'initial_bytes':sum(v['bytes'] for v in initial.values()),'bytes_before':before,'phases':[],'success':False}
  def save():(d/'result.json').write_text(json.dumps(r,indent=2)+'\n')
  def run(name,cmd):
   start=time.perf_counter()
   with (d/(name+'.log')).open('w') as f:rc=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT).returncode
   r['phases'].append({'name':name,'seconds':time.perf_counter()-start,'returncode':rc,'command':cmd});save();assert rc==0,(name,rc)
  try:
   cmd=['rustup','toolchain','install','stable','1.85.0','--profile','minimal']
   if variant=='baseline':cmd+=['--component','rustfmt']
   run('install',cmd);run('default',['rustup','default','stable'])
   r['versions']={t:cap(['rustc','+'+t,'-Vv'],env) for t in ['stable','1.85.0']}
   assert 'rustc 1.99.1 ' in r['versions']['stable'] and 'rustc 1.85.0 ' in r['versions']['1.85.0']
   r['components']={t:cap(['rustup','component','list','--installed','--toolchain',t],env).splitlines() for t in ['stable','1.85.0']}
   assert any(x.startswith('rustfmt-') for x in r['components']['1.85.0'])==(variant=='baseline')
   r['compiler_artifacts']={};r['probe_binaries']={}
   probe=root/'probe.rs';probe.write_text('fn main() { println!("retained compiler and standard library"); }\n')
   for t in ['stable','1.85.0']:
    tool=rustup/'toolchains'/(t+'-x86_64-unknown-linux-gnu')
    r['compiler_artifacts'][t]={n:sha(tool/n) for n in ['bin/rustc','bin/cargo']}
    r['compiler_artifacts'][t].update({str(f.relative_to(tool)):sha(f) for f in sorted((tool/'lib/rustlib/x86_64-unknown-linux-gnu/lib').glob('*')) if f.is_file()})
    binary=root/('probe-'+t);subprocess.run(['rustc','+'+t,str(probe),'-o',str(binary)],env=env,check=True)
    assert cap([str(binary)],env)=='retained compiler and standard library';r['probe_binaries'][t]=sha(binary)
   r['bytes_after']=size(private);r['success']=True
  finally:save()
  rows.append(r)
 shutil.rmtree(private)
assert all(r['compiler_artifacts']==rows[0]['compiler_artifacts'] and r['probe_binaries']==rows[0]['probe_binaries'] for r in rows)
assert manifest(original)==initial, 'host toolchain modified'
(out/'summary.json').write_text(json.dumps({'rows':rows,'host_unchanged':True},indent=2))
