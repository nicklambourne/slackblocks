"""Never-merge paired Rust quality orchestration comparison with independent owner proof."""
import argparse,hashlib,json,os,re,shutil,subprocess,tarfile,time,shlex
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--pair',type=int,required=True);a=p.parse_args();a.source=a.source.resolve();a.root=a.root.resolve();a.output=a.output.resolve();a.root.mkdir();a.output.mkdir(parents=True)
def cap(cmd,cwd=None,env=None):return subprocess.check_output(cmd,cwd=cwd,env=env,text=True).strip()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
source=cap(['git','rev-parse','HEAD'],a.source);assert source=='fc44939dffecb7437e41f670e2f9a726de0ed0ac'
inputs={n:sha(a.source/n) for n in cap(['git','ls-files'],a.source).splitlines()}
(a.output/'inputs.json').write_text(json.dumps({'source':source,'hashes':inputs,'pair':a.pair,'rustc':cap(['rustc','+1.99.0','-vV']),'minimum':cap(['rustc','+1.85.0','-vV']),'python':cap(['python3','--version']),'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion')},indent=2))
archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
owner_commands=[
 ['cargo','+1.99.0','test','--manifest-path','rust/Cargo.toml','--workspace','--all-targets','--locked'],
 ['cargo','+1.99.0','test','--manifest-path','rust/integrations/Cargo.toml','--all-targets','--locked'],
 ['cargo','+1.99.0','test','--manifest-path','rust/Cargo.toml','--workspace','--doc','--locked'],
 ['cargo','+1.99.0','run','--manifest-path','rust/Cargo.toml','--example','api_checkpoint','--locked']]
# Keep the exact checker order and retain generator tests, format, clippy, rustdoc,
# both compile-contract compilers and executable minimum-version guide checks.
commands=[
 ('generator',['python3','rust/generator/generate.py','--check']),
 ('generator-tests',['python3','-m','unittest','discover','-s','rust/generator','-p','test_*.py']),
 ('format-main',['cargo','+1.85.0','fmt','--manifest-path','rust/Cargo.toml','--all','--','--check']),
 ('format-integrations',['cargo','+1.85.0','fmt','--manifest-path','rust/integrations/Cargo.toml','--all','--','--check']),
 ('clippy-integrations',['cargo','+1.99.0','clippy','--manifest-path','rust/integrations/Cargo.toml','--all-targets','--locked','--','-D','warnings']),
 ('clippy-main',['cargo','+1.99.0','clippy','--manifest-path','rust/Cargo.toml','--workspace','--all-targets','--locked','--','-D','warnings']),
 *[(f'native-{i}',cmd) for i,cmd in enumerate(owner_commands)],
 ('rustdoc',['cargo','+1.99.0','doc','--manifest-path','rust/Cargo.toml','--no-deps','--locked']),
 ('ui-current',['python3','rust/bin/check-ui.py','--toolchain','1.99.0']),
 ('ui-minimum',['python3','rust/bin/check-ui.py','--toolchain','1.85.0']),
 ('guides',['python3','docs/scripts/check_rust_guide_snippets.py'])]
# Require the measured commands to match the actual full checker, apart from pinning stable.
expected=[cmd for _,cmd in commands]
actual=(a.source/'rust/bin/check').read_text().splitlines()[3:]
actual=[l.replace('+stable','+1.99.0').replace('--toolchain stable','--toolchain 1.99.0').removeprefix("RUSTDOCFLAGS='-D warnings' ").removeprefix('RUST_GUIDE_TOOLCHAIN=1.85.0 ') for l in actual]
actual=[shlex.split(line) for line in actual]
assert actual==expected,(actual,expected)
rustfmt=cap(['rustup','which','--toolchain','1.85.0','rustfmt'])
base_env={**os.environ,'RUSTFMT':rustfmt,'RUSTUP_TOOLCHAIN':'1.99.0','RUST_GUIDE_TOOLCHAIN':'1.85.0'}
def unpack():
 work=a.root/'work';work.mkdir()
 with tarfile.open(archive) as t:t.extractall(work,filter='data')
 return work
# Independent same-source native owner, with its own empty cache. Not included in
# either policy timer or used to warm a quality cache.
work=unpack();owner_home=a.root/'owner-cargo';env={**base_env,'CARGO_HOME':str(owner_home)};owner={}
for i,cmd in enumerate(owner_commands):
 start=time.perf_counter();log=a.output/f'owner-{i}.log'
 with log.open('w') as f:r=subprocess.run(cmd,cwd=work,env=env,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==0
 text=log.read_text();owner[str(i)]={'seconds':time.perf_counter()-start,'tests':sorted(re.findall(r'^test (.+) \.\.\. ok$',text,re.M)),'counts':re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)}
 assert all(f=='0' and ignored=='0' for _,f,ignored in owner[str(i)]['counts'])
(a.output/'owner.json').write_text(json.dumps(owner,indent=2));shutil.rmtree(work);shutil.rmtree(owner_home)
for variant in (['candidate','baseline'] if a.pair==2 else ['baseline','candidate']):
 work=unpack();home=a.root/(variant+'-cargo');env={**base_env,'CARGO_HOME':str(home)}
 for condition in ['cold','warm']:
  out=a.output/variant/condition;out.mkdir(parents=True)
  r={'variant':variant,'condition':condition,'pair':a.pair,'success':False,'before':{'cargo':size(home),'target':size(work/'rust/target'),'integrations':size(work/'rust/integrations/target')},'phases':[]}
  if condition=='cold':assert not any(r['before'].values())
  try:
   for name,cmd in commands:
    if variant=='candidate' and name.startswith('native-'):continue
    start=time.perf_counter();log=out/(name+'.log')
    with log.open('w') as f:rc=subprocess.run(cmd,cwd=work,env={**env,**({'RUSTDOCFLAGS':'-D warnings'} if name=='rustdoc' else {})},stdout=f,stderr=subprocess.STDOUT).returncode
    r['phases'].append({'name':name,'command':cmd,'seconds':time.perf_counter()-start,'returncode':rc});(out/'result.json').write_text(json.dumps(r,indent=2));assert rc==0,(name,rc)
    if name.startswith('native-'):
     text=log.read_text();own=owner[name.split('-')[-1]]
     assert sorted(re.findall(r'^test (.+) \.\.\. ok$',text,re.M))==own['tests']
     assert re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)==own['counts']
   assert {n:sha(work/n) for n in inputs}==inputs
   r['doc_outputs']={str(f.relative_to(work/'rust/target/doc')):sha(f) for f in (work/'rust/target/doc').rglob('*') if f.is_file()}
   assert r['doc_outputs']
   r['after']={'cargo':size(home),'target':size(work/'rust/target'),'integrations':size(work/'rust/integrations/target')}
   r['success']=True
  finally:(out/'result.json').write_text(json.dumps(r,indent=2)+'\n')
 shutil.rmtree(work);shutil.rmtree(home)
