"""Exercise unmodified workflow shell bodies with mocked GitHub API JSON via jq."""
import hashlib,json,os,subprocess,tempfile,time
from pathlib import Path
ROOT=Path(os.environ.get('FILTER_REPO','/Volumes/Repos/Codex-worktrees/ci-windows-dotnet-runtime/slackblocks'))
OUT=Path(os.environ.get('FILTER_OUT','/Volumes/Scratch/slackblocks-ci-speedups-20261021/doc-asset-detection'))
BASE='7f1370f78daf0be94364769843f221b663706fe4'

def extract(text):
    tail=text.split('        run: |\n',1)[1]
    lines=[]
    for line in tail.splitlines():
        if line and not line.startswith('          '): break
        lines.append(line[10:])
    body='\n'.join(lines).strip()+'\n'
    return body

def run_filter(body,files,event='pull_request',fail=False):
    with tempfile.TemporaryDirectory(prefix='filter-',dir=OUT) as d:
        p=Path(d);(p/'files.json').write_text(json.dumps(files))
        (p/'gh').write_text('#!/usr/bin/env python3\nimport os,sys,subprocess\nif os.environ["MOCK_FAIL"]=="1": sys.exit(1)\nsys.exit(subprocess.call(["jq","-r",sys.argv[sys.argv.index("-q")+1],os.environ["MOCK_FILES"]]))\n')
        (p/'gh').chmod(0o755)
        rendered=body.replace('${{ github.event_name }}',event).replace('${{ github.repository }}','nicklambourne/slackblocks').replace('${{ github.event.pull_request.number }}','486')
        env=dict(os.environ,EVENT=event,PR='486',PATH=str(p)+os.pathsep+os.environ['PATH'],GITHUB_OUTPUT=str(p/'output'),MOCK_FILES=str(p/'files.json'),MOCK_FAIL=str(int(fail)))
        started=time.perf_counter();r=subprocess.run(['bash','--noprofile','--norc','-e','-o','pipefail','-c',rendered],env=env,capture_output=True,text=True)
        elapsed=time.perf_counter()-started
        assert r.returncode==0,(r.returncode,r.stdout,r.stderr)
        value=(p/'output').read_text().strip();assert value in ['relevant=true','relevant=false'],value
        return value=='relevant=true',elapsed

def main():
    OUT.mkdir(parents=True,exist_ok=True);records=[];bodies={}
    general=[('css',[{'filename':'docs/src/css/custom.css'}]),('image',[{'filename':'docs/static/img/logo.svg'}]),('assets',[{'filename':'docs/src/css/custom.css'},{'filename':'docs/static/img/logo.png'}]),('empty',[]),('mdx',[{'filename':'docs/docs/usage/using_blocks.mdx'}]),('script',[{'filename':'docs/scripts/check_rust_guide_snippets.py'}]),('shared',[{'filename':'spec/block-kit.json'}]),('unknown-doc',[{'filename':'docs/future-input.unknown'}]),('nested-css',[{'filename':'docs/src/pages/nested/style.css'}]),('non-css',[{'filename':'docs/src/components/Future.tsx'}]),('legacy-image',[{'filename':'docs/static/img/legacy/foo.png'}]),('mixed',[{'filename':'docs/src/css/custom.css'},{'filename':'spec/block-kit.json'}]),('rename-guide-to-asset',[{'filename':'docs/static/img/guide.svg','previous_filename':'docs/docs/usage/old.mdx'}]),('rename-native-away',[]),('large-assets',[{'filename':f'docs/static/img/{i}.png'} for i in range(10000)]),('large-mixed',[{'filename':'spec/block-kit.json'}]+[{'filename':f'docs/static/img/{i}.png'} for i in range(10000)])]
    for language in ['rust','elixir','go']:
        path=f'.github/workflows/{language}.yml'
        before=extract(subprocess.check_output(['git','show',f'{BASE}:{path}'],cwd=ROOT,text=True));after=extract((ROOT/path).read_text())
        bodies[language]={variant:hashlib.sha256(body.encode()).hexdigest() for variant,body in [('baseline',before),('candidate',after)]}
        (OUT/f'{language}-baseline-filter.sh').write_text(before);(OUT/f'{language}-candidate-filter.sh').write_text(after)
        for name,files in general:
            if name=='rename-native-away': files=[{'filename':'other/old.txt','previous_filename':f'{language}/old-source.txt'}]
            # The Go detector has a deliberately narrower native input scope.
            if language=='go': continue
            expected=name not in ['css','image','assets','nested-css','legacy-image','large-assets']
            observed,elapsed=run_filter(after,files);assert observed==expected,(language,name,observed,expected)
            old,_=run_filter(before,files)
            records.append(dict(language=language,case=name,baseline=old,candidate=observed,expected=expected,seconds=elapsed))
        for name,files,event,fail in [('api-failure',[{'filename':'docs/static/img/logo.svg'}],'pull_request',True),('push',[],'push',False)]:
            value,elapsed=run_filter(after,files,event,fail);assert value
            records.append(dict(language=language,case=name,candidate=value,expected=True,seconds=elapsed))
        if language=='go':
            for path in ['docs/examples/section_hello.json','docs/examples/response_url_payloads.json','docs/docs/usage/sending_messages.mdx','docs/docs/usage/using_blocks.mdx','go/blocks.go','spec/block-kit.json']:
                for rename in [False,True]:
                    files=[{'filename':'elsewhere/moved.txt','previous_filename':path}] if rename else [{'filename':path}]
                    value,elapsed=run_filter(after,files);assert value,(path,rename)
                    records.append(dict(language=language,case=path+(':rename' if rename else ''),candidate=value,expected=True,seconds=elapsed))
            for path in ['docs/src/css/custom.css','docs/static/img/logo.svg']:
                value,elapsed=run_filter(after,[{'filename':path}]);assert not value
                records.append(dict(language=language,case=path,candidate=value,expected=False,seconds=elapsed))
    (OUT/'filter-regression-results-v2.json').write_text(json.dumps(dict(base=BASE,working_tree=True,shell_sha256=bodies,results=records),indent=2)+'\n')
    print(f'{len(records)} workflow-shell/API-query cases passed')
if __name__=='__main__': main()
