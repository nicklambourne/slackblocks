"""Fixed-source Windows tests with isolated installations and observable phases."""
from pathlib import Path
import datetime, hashlib, importlib.util, json, os, shutil, subprocess, sys, tarfile, time, xml.etree.ElementTree as ET, zipfile
source=Path(os.environ['SOURCE']); root=Path(os.environ['BENCH_ROOT']); condition=sys.argv[1]
out=Path(os.environ['EVIDENCE'])/condition;out.mkdir(parents=True,exist_ok=True)
work=root/condition; work.mkdir(exist_ok=False)
sha='dfd1d4bab345abfe150ab56d8268479aaf56bcf2'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip()==sha
archive=root/f'{condition}.tar'
subprocess.run(['git','archive','--format=tar','-o',str(archive),sha],cwd=source,check=True)
with tarfile.open(archive) as tar:tar.extractall(work,filter='data')
locks=Path(os.environ['LOCKS'])
for p in locks.rglob('packages.lock.json'):
 target=work/'csharp'/p.relative_to(locks);target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,target)
exe=Path(os.environ['DOTNET_INSTALL_DIR'])/'dotnet.exe'
env={**os.environ,'DOTNET_ROOT':str(exe.parent),'DOTNET_ROOT_X64':str(exe.parent),'DOTNET_MULTILEVEL_LOOKUP':'0','DOTNET_CLI_HOME':str(root/'cli-home'),'NUGET_PACKAGES':str(root/'nuget'),'NUGET_HTTP_CACHE_PATH':str(root/'nuget-http'),'NUGET_PLUGINS_CACHE_PATH':str(root/'nuget-plugins'),'NUGET_FALLBACK_PACKAGES':'','PATH':str(exe.parent)+os.pathsep+os.environ['PATH']}
phases=[]
def cmd(args,name,cwd=work/'csharp',extra=None):
 start=time.monotonic();r=subprocess.run([str(x) for x in args],cwd=cwd,env={**env,**(extra or {})},text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
 (out/f'{name}.log').write_text(r.stdout,encoding='utf-8');row=dict(name=name,seconds=time.monotonic()-start,code=r.returncode);phases.append(row)
 print(json.dumps(row),flush=True);assert r.returncode==0,r.stdout[-4000:];return r.stdout
record=dict(source=sha,variant=os.environ['VARIANT'],pair=os.environ['PAIR'],condition=condition,success=False,runner=os.environ['RUNNER_NAME'],image=os.environ.get('ImageVersion'),phases=phases)
try:
 sdk=cmd([exe,'--version'],'sdk').strip();assert sdk=='10.0.402',sdk
 info=cmd([exe,'--info'],'info');sdks=cmd([exe,'--list-sdks'],'sdks');runtimes=cmd([exe,'--list-runtimes'],'runtimes')
 assert 'Microsoft.NETCore.App 8.0.32' in runtimes and 'Microsoft.NETCore.App 10.0.13' in runtimes
 assert all(str(exe.parent).lower() in x.lower() for x in (sdks+'\n'+runtimes).splitlines() if x.strip()),(sdks,runtimes)
 assert ('8.0.426' in sdks)==(record['variant']=='baseline')
 record['source_inputs']={p:hashlib.sha256((source/p).read_bytes()).hexdigest() for p in subprocess.check_output(['git','ls-files','csharp','spec','java/generator/generate_models.py','.github/scripts/verify-nuget-consumer.py','docs/examples/csharp','docs/examples/section_hello.json'],cwd=source,text=True).splitlines()}
 record['locks']={str(p.relative_to(locks)):hashlib.sha256(p.read_bytes()).hexdigest() for p in locks.rglob('packages.lock.json')}
 record['installed_bytes']=sum(p.stat().st_size for p in exe.parent.rglob('*') if p.is_file())
 record['nuget_bytes_before']=sum(p.stat().st_size for p in (root/'nuget').rglob('*') if p.is_file())
 assert (record['nuget_bytes_before']>0)==(condition=='warm')
 cmd([exe,'restore','Slackblocks.sln','--locked-mode','--verbosity','normal'],'restore')
 record['installed_packs']=sorted(str(p.relative_to(exe.parent/'packs')) for p in (exe.parent/'packs').glob('*/*') if p.is_dir())
 record['nuget_targeting_packs']={str(p.relative_to(root/'nuget')):sum(f.stat().st_size for f in p.rglob('*') if f.is_file()) for p in (root/'nuget').glob('microsoft.netcore.app.ref/*') if p.is_dir()}
 props=['-p:RepositoryCommit='+sha,'-p:PathMap='+str(work)+'=/_/src']
 cmd([exe,'build','Slackblocks.sln','-c','Release','--no-restore',*props],'build')
 record['tests']={};record['coverage']={}
 for framework in ['net8.0','net10.0']:
  results=out/framework
  cmd([exe,'test','tests/Slackblocks.Tests/Slackblocks.Tests.csproj','-c','Release','-f',framework,'--no-build','--no-restore','--collect','XPlat Code Coverage','--logger','trx','--results-directory',results],f'test-{framework}')
  trx=ET.parse(next(results.rglob('*.trx'))).getroot();ns={'t':'http://microsoft.com/schemas/VisualStudio/TeamTest/2010'}
  tests=trx.findall('.//t:UnitTestResult',ns);assert len(tests)==358 and all(x.get('outcome')=='Passed' for x in tests)
  record['tests'][framework]=sorted(x.get('testName') for x in tests)
  cov=ET.parse(next(results.rglob('coverage.cobertura.xml'))).getroot()
  record['coverage'][framework]=sorted((c.get('name'),c.get('filename').replace(str(work),'/_/src'),line.get('number'),int(line.get('hits'))>0) for c in cov.findall('.//class') for line in c.findall('./lines/line'))
 cmd([exe,'pack','src/Slackblocks/Slackblocks.csproj','-c','Release','--no-build','--no-restore','-o',out/'package',*props],'package')
 nupkg=next((out/'package').glob('*.nupkg'))
 with zipfile.ZipFile(nupkg) as package:
  names=package.namelist();expected=['README.md','LICENSE','LICENSE.BSD-3-Clause','lib/net8.0/Slackblocks.dll','lib/net8.0/Slackblocks.xml']
  assert set(expected)<=set(names)
  record['package_files']={x:hashlib.sha256(package.read(x)).hexdigest() for x in names if not x.endswith('/')}
 feed=out/'feed';feed.mkdir();cmd([exe,'nuget','push',nupkg,'--source',feed],'local-feed')
 spec=importlib.util.spec_from_file_location('consumer',work/'.github/scripts/verify-nuget-consumer.py');consumer_module=importlib.util.module_from_spec(spec);spec.loader.exec_module(consumer_module)
 consumer=root/f'consumer-{condition}';consumer_module.prepare(consumer,'2.8.0')
 config=consumer/'NuGet.Config';config.write_text('<configuration><packageSources><clear/><add key="local" value="'+str(feed)+'"/><add key="nuget.org" value="https://api.nuget.org/v3/index.json"/></packageSources><fallbackPackageFolders><clear/></fallbackPackageFolders></configuration>')
 consumer_env={'NUGET_PACKAGES':str(consumer/'packages'),'NUGET_HTTP_CACHE_PATH':str(consumer/'http'),'DOTNET_CLI_HOME':str(consumer/'home')}
 cmd([exe,'restore','Consumer.csproj','--configfile','NuGet.Config','--force-evaluate','--use-lock-file'],'consumer-restore',consumer,consumer_env)
 result=cmd([exe,'run','--project','Consumer.csproj','--no-restore','-c','Release'],'consumer',consumer,consumer_env);assert 'native-consumer-ok' in result
 record['consumer_lock']=(consumer/'packages.lock.json').read_text();metadata=json.loads((consumer/'packages/slackblocks/2.8.0/.nupkg.metadata').read_text());assert Path(metadata['source']).resolve()==feed.resolve()
 record['nuget_graphs']={str(p.relative_to(work/'csharp')):{k:{a:b for a,b in v.items() if a in ['type','sha512']} for k,v in json.loads(p.read_text())['libraries'].items()} for p in (work/'csharp').rglob('project.assets.json')}
 record['nuget_bytes']=sum(p.stat().st_size for p in (root/'nuget').rglob('*') if p.is_file())
 record['success']=True
finally:
 (out/'result.json').write_text(json.dumps(record,indent=2))
