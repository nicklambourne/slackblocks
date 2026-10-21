"""Measurement-only full-site cache comparison; one explicit stage per Actions step."""
from helpers import *
import argparse
p=argparse.ArgumentParser();p.add_argument('stage',choices=['prepare','build','finish','audit']);p.add_argument('mode',nargs='?',choices=['baseline','candidate']);p.add_argument('state',nargs='?',choices=['cold','warm']);a=p.parse_args()
cache=base/'active-cache';work=base/'work'
if a.stage!='audit':
 dest=results/(a.mode+'-'+a.state);dest.mkdir(exist_ok=True)
 env={**os.environ,'UV_CACHE_DIR':str(cache/'uv'),'CARGO_HOME':str(cache/'cargo'),'GOMODCACHE':str(cache/'gomod'),'GOCACHE':str(cache/'gobuild'),'MAVEN_OPTS':'-Dmaven.repo.local='+str(cache/'maven'),'NUGET_PACKAGES':str(cache/'nuget'),'NUGET_HTTP_CACHE_PATH':str(cache/'nuget-http'),'COMPOSER_CACHE_DIR':str(cache/'composer'),'MIX_HOME':str(cache/'mix'),'HEX_HOME':str(cache/'hex'),'DOTNET_CLI_HOME':str(cache/'dotnet'),'npm_config_cache':str(cache/'npm'),'XDG_CACHE_HOME':str(cache/'xdg'),'DOTNET_CLI_TELEMETRY_OPTOUT':'1','MSBUILDDISABLENODEREUSE':'1','UseSharedCompilation':'false','DOCS_BENCH_RESULTS':str(dest),'DOCS_BENCH_WORK':str(work),'CARGO_NET_OFFLINE':'false','NODE_OPTIONS':'--require='+str(harness/'docs-profile.cjs')+' --require='+str(harness/'webpack-profile.cjs'),'PYTHONPATH':str(harness)}
 for k in ['MIX_BUILD_PATH','MIX_DEPS_PATH','CARGO_TARGET_DIR']:env.pop(k,None)
 webpack=work/'docs/node_modules/.cache/webpack'
 def tree(p):return {str(f.relative_to(p)):sha(f) for f in sorted(p.rglob('*')) if f.is_file() and not f.is_symlink()}
 if a.stage=='prepare':
  if a.state=='cold':cache.mkdir()
  before=size(cache);assert bool(before)==(a.state=='warm')
  save(dest/'result.json',{'pair':pair,'mode':a.mode,'state':a.state,'source':SOURCE,'cache_bytes_before':before,'success':False,'runner':os.environ['RUNNER_NAME'],'image':os.getenv('ImageVersion')})
  command(['git','worktree','add','--detach',str(work),SOURCE],repo,env,dest,'checkout')
  tracked=subprocess.check_output(['git','ls-files'],cwd=work,text=True).splitlines();save(dest/'source-inputs.json',{n:sha(work/n) for n in tracked})
  package=work/'docs/package.json';data=json.loads(package.read_text());scripts=data['scripts'];assert scripts['build'].startswith('pnpm generate && ')
  scripts['build:bench']=scripts['build'].removeprefix('pnpm generate && ');package.write_text(json.dumps(data,indent=2)+'\n')
  command(['git','diff','--binary'],work,env,dest,'source-diff')
  command(['composer','install','--working-dir=php','--no-interaction','--prefer-dist'],work,env,dest,'composer')
  command(['mix','local.hex','2.5.1','--force'],work/'elixir',env,dest,'hex')
  command(['mix','deps.get','--check-locked'],work/'elixir',env,dest,'mix-deps')
  command(['pnpm','install','--frozen-lockfile','--store-dir',str(cache/'pnpm')],work,env,dest,'pnpm')
  assert not webpack.exists(),'Fresh checkout unexpectedly has compiler packs'
 elif a.stage=='build':
  restored=tree(webpack);save(dest/'webpack-before.json',restored)
  save(dest/'cache-action.json',{'hit':os.environ.get('CACHE_HIT','')})
  assert os.environ.get('CACHE_HIT','')==('true' if a.mode=='candidate' and a.state=='warm' else ''), 'Unexpected remote cache outcome'
  if a.mode=='candidate' and a.state=='warm':
   assert restored and restored==json.loads((results/'candidate-cold/webpack-after.json').read_text()),'Restored pack differs from saved cold bytes'
  else:assert not restored,'Cold/baseline unexpectedly restored a pack'
  record=json.loads((dest/'result.json').read_text())
  try:
   command(['pnpm','--filter','@slackblocks/docs','generate'],work,env,dest,'references')
   command(['pnpm','--filter','@slackblocks/docs','build:bench'],work,env,dest,'render-and-checks')
   command(['pnpm','--filter','@slackblocks/docs','check:legacy-snapshots'],work,env,dest,'legacy-snapshots')
   command(['pnpm','--filter','@slackblocks/docs','check:release-snapshots'],work,env,dest,'release-snapshots')
   command(['git','diff','--exit-code','--','docs/legacy/manifest.json','docs/versioned_docs','docs/versioned_sidebars','docs/static/img/legacy','pnpm-lock.yaml','python/uv.lock','go/go.mod','go/go.sum','rust/Cargo.lock','docs/examples/multiselect-compat/rust/Cargo.lock','php/composer.lock','elixir/mix.lock'],work,env,dest,'immutable-inputs')
   command(['node',str(harness/'docs-manifest.cjs'),str(work),str(dest/'manifest.json')],work,env,dest,'manifest')
   dependencies(work,cache,dest);assert (dest/'rust-guides-temporary-dependencies.json').exists()
   refs=[json.loads(f.read_text()) for f in dest.glob('reference-*.json')];expected=['generate:python','generate:go','generate:java','generate:csharp','generate:ruby','check:typescript-api','generate:rust','generate:php','generate:elixir']
   assert sorted(x['script'] for x in refs)==sorted(expected) and all(x['code']==0 for x in refs)
   profiles=[json.loads(f.read_text()) for f in dest.glob('webpack-*.json')];assert len(profiles)==1
   assert {x['name'] for x in profiles[0]['records']}=={'client','server'}
   assert all(x['cache']['type']=='filesystem' and x['cache']['cacheDirectory']==str(webpack) for x in profiles[0]['records'])
   after=tree(webpack);assert after;save(dest/'webpack-after.json',after);record['webpack_bytes_after']=size(webpack)
   with tarfile.open(dest/'published-files.tar.gz','w:gz') as tar:
    for item in sorted((work/'docs/build').rglob('*')):
     if item.is_file() and not re.fullmatch(r'search-index(?:-[a-z]+)?\.json',item.name):tar.add(item,arcname=str(item.relative_to(work/'docs/build')),recursive=False)
   record['success']=True
  finally:save(dest/'result.json',record)
 elif a.stage=='finish':
  record=json.loads((dest/'result.json').read_text());assert record['success']
  command(['dotnet','build-server','shutdown'],work,env,dest,'stop-private-build-server')
  # Observe retained dependency bytes after shutting down the private build server.
  record['cache_bytes_after']=size(cache);record['dependency_cache_bytes_after']={f.name:size(f) for f in cache.iterdir() if f.is_dir()};save(dest/'result.json',record)
  command(['git','worktree','remove','--force',str(work)],repo,env,dest,'cleanup')
  if a.state=='warm':cache.rename(base/(a.mode+'-cache'))
else:
 records=[]
 for mode in ['baseline','candidate']:
  for state in ['cold','warm']:
   d=results/(mode+'-'+state);records.append(json.loads((d/'result.json').read_text()))
 save(results/'runs.json',records)
 reference=json.loads((results/'baseline-cold/manifest.json').read_text())
 for row in records:
  assert row['success'];d=results/(row['mode']+'-'+row['state'])
  for name in ['source-inputs.json','dependency-receipts.json','rust-guides-temporary-dependencies.json']:
   assert json.loads((d/name).read_text())==json.loads((results/'baseline-cold'/name).read_text()),name+' differs'
  actual=json.loads((d/'manifest.json').read_text());assert actual['generated']==reference['generated'] and actual['files'].keys()==reference['files'].keys()
  for name,expected in reference['files'].items():
   observed=actual['files'][name]
   if 'canonical' in expected:
    for field in ['canonical','queries','documents']:assert observed[field]==expected[field],(row,name,field)
   else:assert observed==expected,(row,name)
 save(results/'parity.json',{'passed':True,'samples':len(records),'published_files':len(reference['files']),'generated_files':len(reference['generated'])})
