"""Measurement only: reuse an immediately preceding Release test build for pack."""
import argparse,hashlib,importlib.util,json,os,re,shutil,subprocess,tarfile,time,zipfile,xml.etree.ElementTree as ET
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output','locks']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--pair',type=int,required=True);a=p.parse_args()
for n in ['source','root','output','locks']:setattr(a,n,getattr(a,n).resolve())
a.root.mkdir();a.output.mkdir(parents=True)
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def sha(b):return hashlib.sha256(b).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
def text(cmd,cwd=None):return subprocess.check_output(cmd,cwd=cwd,text=True).strip()
source='fc44939dffecb7437e41f670e2f9a726de0ed0ac';assert text(['git','rev-parse','HEAD'],a.source)==source
assert text(['dotnet','--version'])=='10.0.402'
tracked=text(['git','ls-files'],a.source).splitlines();inputs={n:sha((a.source/n).read_bytes()) for n in tracked};save(a.output/'source.json',inputs)
save(a.output/'inputs.json',{'source':source,'sdk':text(['dotnet','--version']),'sdks':text(['dotnet','--list-sdks']),'runtimes':text(['dotnet','--list-runtimes']),'python':text(['python','--version']),'pair':a.pair,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion'),'run':os.environ['GITHUB_RUN_ID']})
archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
def env_for(cache,locked=False):
 return {**os.environ,'DOTNET_CLI_TELEMETRY_OPTOUT':'1','DOTNET_NOLOGO':'1','DOTNET_CLI_HOME':str(cache/'home'),'NUGET_PACKAGES':str(cache/'packages'),'NUGET_HTTP_CACHE_PATH':str(cache/'http'),'NUGET_PLUGINS_CACHE_PATH':str(cache/'plugins'),'NUGET_FALLBACK_PACKAGES':'','RestoreLockedMode':str(locked).lower(),'RestorePackagesWithLockFile':str(locked).lower(),'MSBUILDDISABLENODEREUSE':'1','UseSharedCompilation':'false'}
def logged(path,cmd,cwd,env):
 with path.open('w') as log:return subprocess.run(cmd,cwd=cwd,env=env,stdout=log,stderr=subprocess.STDOUT).returncode
# Independent, untimed package-validation regression control. Known-good is immutable.
control=a.root/'control';control.mkdir();out=a.output/'negative-control';out.mkdir();env=env_for(a.root/'control-cache')
(control/'Control.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net8.0</TargetFramework><PackageId>PackValidationControl</PackageId><Version>1.0.0</Version><EnablePackageValidation>true</EnablePackageValidation></PropertyGroup></Project>\n')
api=control/'Api.cs';api.write_text('public static class Api { public static int Keep() => 1; }\n')
assert logged(out/'known-good.log',['dotnet','pack','-c','Release','-o',str(out/'known-good')],control,env)==0
baseline=out/'known-good/PackValidationControl.1.0.0.nupkg';good_hash=sha(baseline.read_bytes())
api.write_text('public static class Api { public static int Changed() => 1; }\n')
assert logged(out/'broken-build.log',['dotnet','build','-c','Release'],control,env)==0
negative={}
for variant,flags in [('baseline',[]),('candidate',['--no-build','--no-restore'])]:
 for f in control.rglob('*.semaphore'):f.unlink()
 log=out/(variant+'.log');cmd=['dotnet','pack','-c','Release','-o',str(out/('broken-'+variant)),'--verbosity','normal','-p:PackageValidationBaselinePath='+str(baseline),*flags]
 rc=logged(log,cmd,control,env);assert rc!=0 and 'CP0002' in log.read_text(),(variant,rc)
 assert sha(baseline.read_bytes())==good_hash
 negative[variant]={'exit_code':rc,'diagnostic':'CP0002','known_good_sha256':good_hash,'command':cmd}
save(out/'negative.json',negative)
subprocess.run(['dotnet','build-server','shutdown'],env=env,stdout=subprocess.DEVNULL,check=True)
for variant in (['candidate','baseline'] if a.pair==2 else ['baseline','candidate']):
 cache=a.root/(variant+'-cache');env=env_for(cache,True)
 for condition in ['cold','warm']:
  out=a.output/(variant+'-'+condition);out.mkdir();work=a.root/(variant+'-'+condition);work.mkdir()
  with tarfile.open(archive) as t:t.extractall(work,filter='data')
  for f in a.locks.rglob('packages.lock.json'):shutil.copy2(f,work/'csharp'/f.relative_to(a.locks))
  before=size(cache/'packages');assert bool(before)==(condition=='warm')
  result={'pair':a.pair,'variant':variant,'condition':condition,'success':False,'phases':[],'cache_bytes_before':before,'cache_hit':'local-private-reuse' if before else 'empty','cache_transfer_bytes':0}
  def run(name,cmd):
   start=time.perf_counter();rc=logged(out/(name+'.log'),cmd,work/'csharp',env)
   result['phases'].append({'name':name,'seconds':time.perf_counter()-start,'returncode':rc,'command':cmd});save(out/'result.json',result);assert rc==0,(name,rc)
  try:
   props=['-p:RepositoryCommit='+source,'-p:PathMap='+str(work)+'=/_/src']
   run('test',['dotnet','test','Slackblocks.sln','--configuration','Release','--logger','trx;LogFilePrefix=native','--results-directory',str(out/'tests'),*props])
   ns={'t':'http://microsoft.com/schemas/VisualStudio/TeamTest/2010'};tests={}
   for f in (out/'tests').rglob('*.trx'):
    tree=ET.parse(f).getroot();rows=tree.findall('.//t:UnitTestResult',ns);assert len(rows)==375 and all(x.get('outcome')=='Passed' for x in rows)
    methods=tree.findall('.//t:TestDefinitions/t:UnitTest',ns);frameworks=set(re.findall(r'/(net8\.0|net10\.0)/',' '.join(x.get('storage','').replace('\\','/') for x in methods)));assert len(frameworks)==1
    framework=frameworks.pop();assert framework not in tests;tests[framework]=sorted(x.get('testName') for x in rows)
   assert len(tests)==2;result['tests']=tests
   build=work/'csharp/src/Slackblocks/bin/Release/net8.0'
   before_build={f.name:sha(f.read_bytes()) for f in build.iterdir() if f.suffix in ['.dll','.pdb','.xml']};assert len(before_build)==3
   package=out/'package';flags=['--no-build','--no-restore'] if variant=='candidate' else []
   run('pack',['dotnet','pack','src/Slackblocks/Slackblocks.csproj','--configuration','Release','--output',str(package),'--verbosity','normal',*props,*flags])
   log=(out/'pack.log').read_text();assert 'RunPackageValidation:' in log and 'APICompat ran successfully' in log
   assert before_build=={f.name:sha(f.read_bytes()) for f in build.iterdir() if f.suffix in ['.dll','.pdb','.xml']}
   result['build_bytes']=before_build;result['archives']={};result['archive_sha256']={}
   packages=sorted(package.glob('*nupkg'));assert len(packages)==2
   for f in packages:
    result['archive_sha256'][f.name]=sha(f.read_bytes())
    with zipfile.ZipFile(f) as z:
     result['archives'][f.name]={n:sha(z.read(n)) for n in z.namelist()};assert not z.testzip()
     if f.suffix=='.nupkg':
      assert {'README.md','LICENSE','LICENSE.BSD-3-Clause','lib/net8.0/Slackblocks.dll','lib/net8.0/Slackblocks.xml'}.issubset(z.namelist())
   # Fresh local-archive consumer, not a registry/publication claim. Reuse existing native assertions.
   spec=importlib.util.spec_from_file_location('consumer',work/'.github/scripts/verify-nuget-consumer.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);mod.SOURCE=str(package)
   consumer=a.root/(variant+'-'+condition+'-consumer');mod.prepare(consumer,'2.8.0');cenv=env_for(consumer/'private-cache');cenv['NUGET_PACKAGES']=str(consumer/'packages')
   cp=['-p:ImportDirectoryBuildProps=false','-p:ImportDirectoryBuildTargets=false']
   assert logged(out/'consumer-restore.log',['dotnet','restore','Consumer.csproj','--configfile','NuGet.Config','--no-cache','--force-evaluate',*cp],consumer,cenv)==0
   installed_sha=mod.verify_assets(consumer,'2.8.0');assert installed_sha==result['archive_sha256']['Slackblocks.2.8.0.nupkg']
   assert logged(out/'consumer-run.log',['dotnet','run','--project','Consumer.csproj','--no-restore','--configuration','Release',*cp],consumer,cenv)==0
   assert 'native-consumer-ok' in (out/'consumer-run.log').read_text()
   result['consumer']={'native_assertions':'constructor, serialization, validation','libraries':sorted(json.loads((consumer/'obj/project.assets.json').read_text())['libraries']),'archive_sha256':installed_sha}
   assert result['consumer']['libraries']==['Slackblocks/2.8.0']
   shutil.copy2(consumer/'obj/project.assets.json',out/'consumer-project.assets.json')
   result['package_graphs']={str(f.relative_to(work/'csharp')):{k:{a:b for a,b in v.items() if a in ['type','sha512']} for k,v in json.loads(f.read_text())['libraries'].items()} for f in (work/'csharp').rglob('project.assets.json')}
   result['locks']={str(f.relative_to(work/'csharp')):sha(f.read_bytes()) for f in (work/'csharp').rglob('packages.lock.json')}
   assert result['locks']=={str(f.relative_to(a.locks)):sha(f.read_bytes()) for f in a.locks.rglob('packages.lock.json')}
   assert {n:sha((work/n).read_bytes()) for n in tracked}==inputs
   result['cache_bytes_after']=size(cache/'packages');result['http_bytes_after']=size(cache/'http');result['success']=True
  finally:save(out/'result.json',result)
  subprocess.run(['dotnet','build-server','shutdown'],env=env,stdout=subprocess.DEVNULL,check=True);shutil.rmtree(work);shutil.rmtree(consumer)
