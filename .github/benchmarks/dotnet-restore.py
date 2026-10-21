"""Measurement-only canonical Linux .NET restore orchestration comparison."""
import argparse,hashlib,json,os,re,shutil,subprocess,tarfile,time,xml.etree.ElementTree as ET
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output','locks']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--pair',type=int,required=True);a=p.parse_args()
a.source=a.source.resolve();a.root=a.root.resolve();a.output=a.output.resolve();a.locks=a.locks.resolve()
a.root.mkdir();a.output.mkdir(parents=True)
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def sha(b):return hashlib.sha256(b).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink())
def text(cmd,cwd=None):return subprocess.check_output(cmd,cwd=cwd,text=True).strip()
source='fc44939dffecb7437e41f670e2f9a726de0ed0ac';assert text(['git','rev-parse','HEAD'],a.source)==source
assert text(['dotnet','--version'])=='10.0.402'
tracked=text(['git','ls-files'],a.source).splitlines();inputs={n:sha((a.source/n).read_bytes()) for n in tracked}
save(a.output/'source.json',inputs)
save(a.output/'inputs.json',{'source':source,'sdk':text(['dotnet','--version']),'sdks':text(['dotnet','--list-sdks']),'runtimes':text(['dotnet','--list-runtimes']),'pair':a.pair,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion'),'run':os.environ['GITHUB_RUN_ID']})
archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
for variant in (['candidate','baseline'] if a.pair==2 else ['baseline','candidate']):
 cache=a.root/variant;cache.mkdir()
 env={**os.environ,'DOTNET_CLI_TELEMETRY_OPTOUT':'1','DOTNET_NOLOGO':'1','DOTNET_CLI_HOME':str(cache/'home'),'NUGET_PACKAGES':str(cache/'packages'),'NUGET_HTTP_CACHE_PATH':str(cache/'http'),'NUGET_PLUGINS_CACHE_PATH':str(cache/'plugins'),'NUGET_FALLBACK_PACKAGES':'','RestoreLockedMode':'true','RestorePackagesWithLockFile':'true','MSBUILDDISABLENODEREUSE':'1','UseSharedCompilation':'false'}
 for condition in ['cold','warm']:
  out=a.output/variant/condition;out.mkdir(parents=True);work=a.root/'checkout';work.mkdir()
  with tarfile.open(archive) as t:t.extractall(work,filter='data')
  for f in a.locks.rglob('packages.lock.json'):
   target=work/'csharp'/f.relative_to(a.locks);shutil.copy2(f,target)
  # Both policies use identical build paths and resolved package locks.
  result={'variant':variant,'condition':condition,'pair':a.pair,'success':False,'cache_bytes_before':size(cache/'packages'),'http_bytes_before':size(cache/'http'),'actions_transfer_bytes':0,'network_bytes':None,'phases':[]}
  assert bool(result['cache_bytes_before'])==(condition=='warm')
  def run(name,cmd):
   start=time.perf_counter()
   with (out/(name+'.log')).open('w') as log:rc=subprocess.run(cmd,cwd=work/'csharp',env=env,stdout=log,stderr=subprocess.STDOUT).returncode
   result['phases'].append({'name':name,'seconds':time.perf_counter()-start,'returncode':rc,'command':cmd});save(out/'result.json',result)
   assert rc==0,(name,rc)
  try:
   run('generator',['python','generator/generate_models.py'])
   assert {n:sha((work/n).read_bytes()) for n in tracked}==inputs
   if variant=='candidate':run('restore',['dotnet','restore','Slackblocks.sln','--locked-mode'])
   run('format',['dotnet','format','Slackblocks.sln','--verify-no-changes']+(['--no-restore'] if variant=='candidate' else []))
   run('test',['dotnet','test','Slackblocks.sln','--configuration','Release','--collect','XPlat Code Coverage','--logger','trx;LogFilePrefix=native','--results-directory',str(out/'tests'),'-p:RepositoryCommit='+source,'-p:PathMap='+str(work)+'=/_/src']+(['--no-restore'] if variant=='candidate' else []))
   tests={};coverage={}
   ns={'t':'http://microsoft.com/schemas/VisualStudio/TeamTest/2010'}
   for f in (out/'tests').rglob('*.trx'):
    tree=ET.parse(f).getroot();rows=tree.findall('.//t:UnitTestResult',ns)
    assert len(rows)==375 and all(x.get('outcome')=='Passed' for x in rows)
    methods=tree.findall('.//t:TestDefinitions/t:UnitTest',ns)
    frameworks=set(re.findall(r'/(net8\.0|net10\.0)/',' '.join(x.get('storage','').replace('\\','/') for x in methods)))
    assert len(frameworks)==1,frameworks
    framework=frameworks.pop();assert framework not in tests
    tests[framework]=sorted(x.get('testName') for x in rows)
   assert len(tests)==2,len(tests)
   result['tests']=tests
   for i,f in enumerate(sorted((out/'tests').rglob('coverage.cobertura.xml'))):
    tree=ET.parse(f).getroot();coverage[str(i)]=sorted((c.get('name'),c.get('filename').replace(str(work),'/_/src'),line.get('number'),int(line.get('hits'))>0) for c in tree.findall('.//class') for line in c.findall('./lines/line'))
   assert len(coverage)==2;result['coverage']=sorted(coverage.values())
   result['package_graphs']={str(f.relative_to(work/'csharp')):{k:{a:b for a,b in v.items() if a in ['type','sha512']} for k,v in json.loads(f.read_text())['libraries'].items()} for f in (work/'csharp').rglob('project.assets.json')}
   result['locks']={str(f.relative_to(work/'csharp')):sha(f.read_bytes()) for f in (work/'csharp').rglob('packages.lock.json')}
   assert result['locks']=={str(f.relative_to(a.locks)):sha(f.read_bytes()) for f in a.locks.rglob('packages.lock.json')}
   result['assembly_bytes']={str(f.relative_to(work/'csharp')):sha(f.read_bytes()) for f in (work/'csharp').rglob('bin/Release/*/Slackblocks.dll')}
   result['xml_bytes']={str(f.relative_to(work/'csharp')):sha(f.read_bytes()) for f in (work/'csharp').rglob('bin/Release/*/Slackblocks.xml')}
   assert result['assembly_bytes'] and result['xml_bytes']
   result['cache_bytes_after']=size(cache/'packages');result['http_bytes_after']=size(cache/'http')
   assert {n:sha((work/n).read_bytes()) for n in tracked}==inputs
   result['success']=True
  finally:
   save(out/'result.json',result)
  subprocess.run(['dotnet','build-server','shutdown'],env=env,stdout=subprocess.DEVNULL,check=True)
  shutil.rmtree(work)
