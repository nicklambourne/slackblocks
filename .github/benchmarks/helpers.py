"""Same-host full nine-language builds: sequential vs two-worker reference generation."""
from pathlib import Path
import datetime,hashlib,json,os,re,shutil,subprocess,tarfile,threading,time
SOURCE='fc44939dffecb7437e41f670e2f9a726de0ed0ac'
repo=Path.cwd();harness=repo/'.github/benchmarks';pair=int(os.environ['BENCH_PAIR'])
base=Path(os.environ['RUNNER_TEMP'])/f'docs-webpack-pair-{pair}';results=repo/'benchmark-results'
base.mkdir(exist_ok=True);results.mkdir(exist_ok=True)
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
