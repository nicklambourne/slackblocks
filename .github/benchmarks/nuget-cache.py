"""Never-merge paired global NuGet cache comparison, all builds/tests remain fresh."""
import argparse,hashlib,json,os,re,shutil,subprocess,tarfile,time,xml.etree.ElementTree as ET
from pathlib import Path
p=argparse.ArgumentParser()
for n in ['source','root','output','locks']:p.add_argument('--'+n,type=Path,required=True)
p.add_argument('--pair',type=int,required=True);p.add_argument('--variant',choices=['baseline','candidate'],required=True);p.add_argument('--condition',choices=['cold','warm'],required=True);a=p.parse_args()
a.source=a.source.resolve();a.root=a.root.resolve();a.output=a.output.resolve();a.locks=a.locks.resolve();a.root.mkdir(exist_ok=True);a.output.mkdir(parents=True)
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def sha(b):return hashlib.sha256(b).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
def text(cmd,cwd=None):return subprocess.check_output(cmd,cwd=cwd,text=True).strip()
source='fc44939dffecb7437e41f670e2f9a726de0ed0ac';assert text(['git','rev-parse','HEAD'],a.source)==source
assert text(['dotnet','--version'])=='10.0.402'
tracked=text(['git','ls-files'],a.source).splitlines();inputs={n:sha((a.source/n).read_bytes()) for n in tracked};save(a.output/'source.json',inputs)
save(a.output/'inputs.json',{'source':source,'sdk':text(['dotnet','--version']),'sdks':text(['dotnet','--list-sdks']),'runtimes':text(['dotnet','--list-runtimes']),'python':text(['python','--version']),'pair':a.pair,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion'),'run':os.environ['GITHUB_RUN_ID']})
archive=a.root/'source.tar';subprocess.run(['git','archive','-o',str(archive),'HEAD'],cwd=a.source,check=True)
work=a.root/'work';assert not work.exists();work.mkdir()
with tarfile.open(archive) as t:t.extractall(work,filter='data')
archive.unlink()
for f in a.locks.rglob('packages.lock.json'):shutil.copy2(f,work/'csharp'/f.relative_to(a.locks))
cache=a.root/'cache';before=size(cache/'packages');warm=a.variant=='candidate' and a.condition=='warm'
assert bool(before)==warm;assert (os.getenv('CACHE_HIT')=='true')==warm
assert size(cache/'http')==0
# These observer-only locks freeze identical inputs; production resolution policy is unchanged.
env={**os.environ,'DOTNET_CLI_TELEMETRY_OPTOUT':'1','DOTNET_NOLOGO':'1','DOTNET_CLI_HOME':str(cache/'home'),'NUGET_PACKAGES':str(cache/'packages'),'NUGET_HTTP_CACHE_PATH':str(cache/'http'),'NUGET_PLUGINS_CACHE_PATH':str(cache/'plugins'),'NUGET_FALLBACK_PACKAGES':'','RestoreLockedMode':'true','RestorePackagesWithLockFile':'true','MSBUILDDISABLENODEREUSE':'1','UseSharedCompilation':'false'}
result={'variant':a.variant,'condition':a.condition,'pair':a.pair,'success':False,'cache_hit':os.getenv('CACHE_HIT'),'cache_bytes_before':before,'http_bytes_before':0,'network_bytes':None,'phases':[]}
out=a.output

def run(name,cmd):
 start=time.perf_counter()
 with (out/(name+'.log')).open('w') as log:rc=subprocess.run(cmd,cwd=work/'csharp',env=env,stdout=log,stderr=subprocess.STDOUT).returncode
 result['phases'].append({'name':name,'seconds':time.perf_counter()-start,'returncode':rc,'command':cmd});save(out/'result.json',result);assert rc==0,(name,rc)
try:
 run('generator',['python','generator/generate_models.py'])
 assert {n:sha((work/n).read_bytes()) for n in tracked}==inputs
 run('format',['dotnet','format','Slackblocks.sln','--verify-no-changes'])
 run('test',['dotnet','test','Slackblocks.sln','--configuration','Release','--collect','XPlat Code Coverage','--logger','trx;LogFilePrefix=native','--results-directory',str(out/'tests'),'-p:RepositoryCommit='+source,'-p:PathMap='+str(work)+'=/_/src'])
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
 for i,f in enumerate(sorted((out/'tests').glob('*/coverage.cobertura.xml'))):
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
finally:save(out/'result.json',result)
subprocess.run(['dotnet','build-server','shutdown'],env=env,stdout=subprocess.DEVNULL,check=True)
shutil.rmtree(work)
# Fresh-job boundary: only candidate packages are sent through Actions cache.
for path in cache.iterdir():
 if path.name!='packages' or a.variant=='baseline':shutil.rmtree(path)
