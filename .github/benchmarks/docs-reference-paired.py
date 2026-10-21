"""Same-host full nine-language builds: sequential vs two-worker reference generation."""
from pathlib import Path
import datetime,hashlib,json,os,re,shutil,subprocess,tarfile,threading,time
SOURCE='fc44939dffecb7437e41f670e2f9a726de0ed0ac'
repo=Path.cwd();harness=repo/'.github/benchmarks';pair=int(os.environ['BENCH_PAIR'])
base=Path(os.environ['RUNNER_TEMP'])/f'docs-reference-pair-{pair}';results=repo/'benchmark-results'
base.mkdir();results.mkdir();records=[]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def size(p):return sum(f.stat().st_size for f in p.rglob('*') if f.is_file() and not f.is_symlink()) if p.exists() else 0
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def command(args,cwd,env,dest,phase):
 begin=time.monotonic();started=datetime.datetime.now(datetime.timezone.utc).isoformat();stop=threading.Event();memory=[]
 with (dest/(phase+'.log')).open('w') as log:
  proc=subprocess.Popen(args,cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,start_new_session=True)
  def observe():
   while not stop.is_set():
    try:
     rows=[tuple(map(int,line.split())) for line in subprocess.check_output(['ps','-eo','pid=,ppid=,pgid=,rss='],text=True).splitlines()]
     owned={proc.pid}|{pid for pid,ppid,pgid,rss in rows if pgid==proc.pid}
     while True:
      expanded=owned|{pid for pid,ppid,pgid,rss in rows if ppid in owned}
      if expanded==owned:break
      owned=expanded
     memory.append({'elapsed_s':time.monotonic()-begin,'rss_kib':sum(rss for pid,ppid,pgid,rss in rows if pid in owned),'processes':len(owned)})
    except (subprocess.CalledProcessError,ValueError):pass
    stop.wait(.25)
  thread=threading.Thread(target=observe);thread.start()
  for line in proc.stdout:
   log.write(datetime.datetime.now(datetime.timezone.utc).isoformat()+' '+line);log.flush()
  rc=proc.wait();stop.set();thread.join()
 entry={'phase':phase,'args':args,'started_at':started,'elapsed_s':time.monotonic()-begin,'returncode':rc,'peak_process_tree_rss_kib':max((x['rss_kib'] for x in memory),default=0)}
 with (dest/'phases.jsonl').open('a') as f:f.write(json.dumps(entry)+'\n')
 save(dest/(phase+'-memory.json'),memory);print(json.dumps(entry),flush=True)
 if rc:raise subprocess.CalledProcessError(rc,args)
def dependencies(work,cache,dest):
 # Exact resolved artifacts/graphs, not just package counts. Normalization removes only task-owned paths.
 manifests={}
 for area in ['java','csharp','docs/scripts/csharp-reference','docs/examples/multiselect-compat/csharp']:
  for f in (work/area).rglob('project.assets.json'):
   def normalize(v):
    if isinstance(v,str):return v.replace(str(work),'/SOURCE').replace(str(cache),'/CACHE')
    if isinstance(v,list):return [normalize(x) for x in v]
    if isinstance(v,dict):return {normalize(k):normalize(x) for k,x in v.items()}
    return v
   manifests[str(f.relative_to(work))]=normalize(json.loads(f.read_text()))
 for folder,suffixes in [('maven',{'.pom','.jar'}),('nuget',{'.nupkg'}),('cargo',{'.crate'})]:
  for f in (cache/folder).rglob('*'):
   if f.is_file() and f.suffix in suffixes:manifests['cache/'+folder+'/'+str(f.relative_to(cache/folder))]=sha(f)
 for area in ['rust','docs/examples/multiselect-compat/rust']:
  f=work/area/'Cargo.lock';manifests[str(f.relative_to(work))]=f.read_text()
 for f in (work/'node_modules/.pnpm').glob('*/node_modules/*/package.json'):
  manifests[str(f.relative_to(work))]=sha(f)
 for f in (work/'node_modules/.pnpm').glob('*/node_modules/@*/*/package.json'):
  manifests[str(f.relative_to(work))]=sha(f)
 for f in (work/'python/.venv').rglob('*.dist-info/METADATA'):manifests[str(f.relative_to(work))]=sha(f)
 manifests['php-installed']=json.loads((work/'php/vendor/composer/installed.json').read_text())
 for f in (work/'elixir/deps').glob('*/mix.exs'):manifests[str(f.relative_to(work))]=sha(f)
 save(dest/'dependency-receipts.json',manifests)
for mode in (['candidate','baseline'] if pair==2 else ['baseline','candidate']):
 cache=base/'active-cache';cache.mkdir()
 for state in ['cold','warm']:
  dest=results/(mode+'-'+state);dest.mkdir();work=base/'work'
  env={**os.environ,'UV_CACHE_DIR':str(cache/'uv'),'CARGO_HOME':str(cache/'cargo'),'GOMODCACHE':str(cache/'gomod'),'GOCACHE':str(cache/'gobuild'),'MAVEN_OPTS':'-Dmaven.repo.local='+str(cache/'maven'),'NUGET_PACKAGES':str(cache/'nuget'),'NUGET_HTTP_CACHE_PATH':str(cache/'nuget-http'),'COMPOSER_CACHE_DIR':str(cache/'composer'),'MIX_HOME':str(cache/'mix'),'HEX_HOME':str(cache/'hex'),'DOTNET_CLI_HOME':str(cache/'dotnet'),'npm_config_cache':str(cache/'npm'),'XDG_CACHE_HOME':str(cache/'xdg'),'DOTNET_CLI_TELEMETRY_OPTOUT':'1','MSBUILDDISABLENODEREUSE':'1','UseSharedCompilation':'false','DOCS_BENCH_RESULTS':str(dest),'DOCS_BENCH_WORK':str(work),'CARGO_NET_OFFLINE':'false','NODE_OPTIONS':'--require='+str(harness/'docs-profile.cjs'),'PYTHONPATH':str(harness)}
  for k in ['MIX_BUILD_PATH','MIX_DEPS_PATH','CARGO_TARGET_DIR']:env.pop(k,None)
  before=size(cache);assert bool(before)==(state=='warm')
  record={'pair':pair,'mode':mode,'state':state,'source':SOURCE,'cache_bytes_before':before,'success':False,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion'),'cache_transfer_bytes':0}
  try:
   command(['git','worktree','add','--detach',str(work),SOURCE],repo,env,dest,'checkout')
   tracked=subprocess.check_output(['git','ls-files'],cwd=work,text=True).splitlines();save(dest/'source-inputs.json',{n:sha(work/n) for n in tracked})
   package=work/'docs/package.json';data=json.loads(package.read_text());scripts=data['scripts'];expected=['generate:python','generate:go','generate:java','generate:csharp','generate:ruby','check:typescript-api','generate:rust','generate:php','generate:elixir']
   assert scripts['generate']==' && '.join('pnpm '+s for s in expected)
   assert scripts['build'].startswith('pnpm generate && ')
   scripts['build:bench']=scripts['build'].removeprefix('pnpm generate && ')
   if mode=='candidate':
    shutil.copy2(harness/'generate_references.mjs',work/'docs/scripts/generate_references.mjs');scripts['generate']='node scripts/generate_references.mjs'
   package.write_text(json.dumps(data,indent=2)+'\n')
   command(['git','diff','--binary'],work,env,dest,'source-diff')
   command(['composer','install','--working-dir=php','--no-interaction','--prefer-dist'],work,env,dest,'composer')
   command(['mix','local.hex','2.5.1','--force'],work/'elixir',env,dest,'hex')
   command(['mix','deps.get','--check-locked'],work/'elixir',env,dest,'mix-deps')
   command(['pnpm','install','--frozen-lockfile','--store-dir',str(cache/'pnpm')],work,env,dest,'pnpm')
   command(['pnpm','--filter','@slackblocks/docs','generate'],work,env,dest,'references')
   command(['pnpm','--filter','@slackblocks/docs','build:bench'],work,env,dest,'render-and-checks')
   command(['pnpm','--filter','@slackblocks/docs','check:legacy-snapshots'],work,env,dest,'legacy-snapshots')
   command(['pnpm','--filter','@slackblocks/docs','check:release-snapshots'],work,env,dest,'release-snapshots')
   command(['git','diff','--exit-code','--','docs/legacy/manifest.json','docs/versioned_docs','docs/versioned_sidebars','docs/static/img/legacy','pnpm-lock.yaml','python/uv.lock','go/go.mod','go/go.sum','rust/Cargo.lock','docs/examples/multiselect-compat/rust/Cargo.lock','php/composer.lock','elixir/mix.lock'],work,env,dest,'immutable-inputs')
   command(['node',str(harness/'docs-manifest.cjs'),str(work),str(dest/'manifest.json')],work,env,dest,'manifest')
   dependencies(work,cache,dest)
   assert (dest/'rust-guides-temporary-dependencies.json').exists(),'Missing temporary Rust dependency receipts'
   refs=[json.loads(f.read_text()) for f in dest.glob('reference-*.json')];assert sorted(x['script'] for x in refs)==sorted(expected) and all(x['code']==0 for x in refs),refs
   with tarfile.open(dest/'published-files.tar.gz','w:gz') as tar:
    for item in sorted((work/'docs/build').rglob('*')):
     if item.is_file() and not re.fullmatch(r'search-index(?:-[a-z]+)?\.json',item.name):tar.add(item,arcname=str(item.relative_to(work/'docs/build')),recursive=False)
   record['success']=True
  finally:
   record['cache_bytes_after']=size(cache);save(dest/'result.json',record);records.append(record);save(results/'runs.json',records)
  command(['dotnet','build-server','shutdown'],work,env,dest,'stop-private-build-server')
  command(['git','worktree','remove','--force',str(work)],repo,env,dest,'cleanup')
 cache.rename(base/(mode+'-cache'))
reference=json.loads((results/'baseline-cold/manifest.json').read_text());deps=json.loads((results/'baseline-cold/dependency-receipts.json').read_text());temporary=json.loads((results/'baseline-cold/rust-guides-temporary-dependencies.json').read_text());source=json.loads((results/'baseline-cold/source-inputs.json').read_text())
for row in records:
 d=results/(row['mode']+'-'+row['state']);actual=json.loads((d/'manifest.json').read_text())
 assert json.loads((d/'source-inputs.json').read_text())==source,'Source input mismatch'
 assert json.loads((d/'dependency-receipts.json').read_text())==deps,'Resolved dependency input mismatch'
 assert json.loads((d/'rust-guides-temporary-dependencies.json').read_text())==temporary,'Temporary consumer dependency mismatch'
 assert actual['generated']==reference['generated'],'Generated outputs differ'
 assert actual['files'].keys()==reference['files'].keys(),'Published filenames differ'
 for name,expected in reference['files'].items():
  observed=actual['files'][name]
  if 'canonical' in expected:
   for field in ['canonical','queries','documents']:assert observed[field]==expected[field],(row,name,field)
  else:assert observed==expected,(row,name)
save(results/'parity.json',{'passed':True,'source_files':len(source),'dependency_receipts':len(deps),'published_files':len(reference['files']),'generated_files':len(reference['generated']),'runs':len(records)})
