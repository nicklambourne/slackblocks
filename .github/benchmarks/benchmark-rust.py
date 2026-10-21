#!/usr/bin/env python3
"""Measurement-only paired archive checks; never publish or merge this harness."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

parser=argparse.ArgumentParser()
parser.add_argument('--source',type=Path,required=True)
parser.add_argument('--baseline-checker',type=Path,required=True)
parser.add_argument('--pair',type=int,required=True)
parser.add_argument('--output',type=Path,required=True)
a=parser.parse_args();a.source=a.source.resolve();a.output=a.output.resolve();a.output.mkdir(parents=True,exist_ok=True)

def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def size(path):return sum(p.stat().st_size for p in path.rglob('*') if p.is_file()) if path.exists() else 0

def command(args,**kw):return subprocess.check_output(args,text=True,**kw).strip()
sha=command(['git','rev-parse','HEAD'],cwd=a.source)
assert not command(['git','status','--porcelain'],cwd=a.source)
inputs={str(p.relative_to(a.source)):digest(p) for folder in ['rust','spec'] for p in (a.source/folder).rglob('*') if p.is_file() and '.git' not in p.parts and 'target' not in p.parts}
inputs.pop('rust/bin/check-package.py')
metadata={'source':sha,'baseline_checker_sha256':digest(a.baseline_checker),'candidate_checker_sha256':digest(a.source/'rust/bin/check-package.py'),'inputs':inputs,'python':sys.version,'rust85':command(['rustc','+1.85.0','-Vv']),'rust_stable':command(['rustc','+1.99.0','-Vv']),'image_os':os.environ.get('ImageOS'),'image_version':os.environ.get('ImageVersion'),'machine':command(['uname','-a']),'pair':a.pair}
(a.output/'metadata.json').write_text(json.dumps(metadata,indent=2))
# This common instrumentation replaces only the subprocess helper in both variants.
# The checker still owns archive validation, generated consumer code and execution.
instrument=r'''def run(arguments,cwd=RUST):
    before_lock=(cwd/'Cargo.lock').exists()
    consumer=arguments==['run']
    if consumer:
        assert not before_lock, 'Consumer must independently resolve without a lock'
        assert ROOT not in cwd.parents and cwd!=ROOT
    start=time.perf_counter()
    completed=subprocess.run(cargo+arguments,cwd=cwd,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    elapsed=time.perf_counter()-start
    print(completed.stdout,end='',flush=True)
    phase={'toolchain':args.toolchain,'command':arguments,'directory':cwd.name,'seconds':elapsed,'returncode':completed.returncode,'log':completed.stdout}
    _phases.append(phase)
    if completed.returncode==0 and arguments[0]=='package':
        archive=Path(metadata['target_directory'])/'package'/f'slackblocks-{version}.crate'
        with tarfile.open(archive) as tar:
            files={m.name:hashlib.sha256(tar.extractfile(m).read()).hexdigest() for m in tar.getmembers() if m.isfile()}
        _archives.append({'toolchain':args.toolchain,'sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'bytes':archive.stat().st_size,'files':files})
    if completed.returncode==0 and consumer:
        info=json.loads(subprocess.check_output(cargo+['metadata','--format-version','1','--locked','--offline'],cwd=cwd))
        package=next(p for p in info['packages'] if p['name']=='slackblocks')
        manifest=Path(package['manifest_path']).resolve()
        assert ROOT not in manifest.parents
        assert manifest.parent.parent==cwd.parent
        normalized=(cwd/'Cargo.toml').read_text().replace(str(manifest.parent),'<ARCHIVE>')
        _consumers.append({'toolchain':args.toolchain,'lower_bounds':args.lower_bounds if not getattr(args,'all_consumers',False) else 'lower-bounds' in cwd.name,'json_features':args.json_features if not getattr(args,'all_consumers',False) else 'json-features' in cwd.name,'fresh_lock':not before_lock,'lock':(cwd/'Cargo.lock').read_text(),'manifest':normalized,'main_sha256':hashlib.sha256((cwd/'src/main.rs').read_bytes()).hexdigest(),'dependencies':sorted((p['name'],p['version'],p.get('source')) for p in info['packages']),'features':sorted((next(p['name'] for p in info['packages'] if p['id']==n['id']),n['features']) for n in info['resolve']['nodes']),'output':[line for line in completed.stdout.splitlines() if line.startswith('{"channel"')],'isolated':True})
    completed.check_returncode()
'''
roots={};caches={}
for variant in ['baseline','candidate']:
    root=a.output.parent/f'work-{a.pair}-{variant}'
    subprocess.run(['git','worktree','add','--quiet','--detach',str(root),sha],cwd=a.source,check=True)
    roots[variant]=root;caches[variant]=a.output.parent/f'cache-{a.pair}-{variant}';caches[variant].mkdir()
order=['baseline','candidate'] if a.pair%2 else ['candidate','baseline']
records=[]
for condition in ['cold','warm']:
    for variant in order:
        root=roots[variant];cache=caches[variant];sample=f'{variant}-{condition}'
        env={**os.environ,'CARGO_HOME':str(cache),'CARGO_TERM_COLOR':'never'};env.pop('CARGO_TARGET_DIR',None)
        before={'cargo_home_bytes':size(cache),'target_bytes':size(root/'rust/target')}
        if condition=='cold':assert before=={'cargo_home_bytes':0,'target_bytes':0}
        else:
            prior=next(r for r in records if r['variant']==variant and r['condition']=='cold')
            assert before==prior['after']
        profiles=[['1.85.0','--all-consumers'],['1.99.0']] if variant=='candidate' else [['1.85.0'],['1.99.0'],['1.85.0','--lower-bounds'],['1.85.0','--json-features']]
        result={'variant':variant,'condition':condition,'before':before,'profiles':[]};start=time.perf_counter();returncode=0
        for i,profile in enumerate(profiles):
            source=(root/'rust/bin/check-package.py').read_text() if variant=='candidate' else a.baseline_checker.read_text()
            source=source.replace('ROOT=Path(__file__).resolve().parents[2]','ROOT=Path('+repr(str(root))+')')
            first=source.index('def run(');last=source.index('metadata=json.loads',first);source=source[:first]+instrument+source[last:]
            receipt=a.output/f'{sample}-{i}.json';program=a.output/f'{sample}-{i}.py';log=a.output/f'{sample}-{i}.log'
            # Indenting whole source would change multiline literal bytes; compile/exec preserves them.
            program.write_text("import hashlib,json,time\nfrom pathlib import Path\n_phases=[];_archives=[];_consumers=[]\ntry:\n    exec(compile("+repr(source)+", __file__, 'exec'))\nfinally:\n    Path("+repr(str(receipt))+").write_text(json.dumps({'phases':_phases,'archives':_archives,'consumers':_consumers},indent=2))\n")
            with log.open('w') as stream:
                completed=subprocess.run([sys.executable,str(program),'--toolchain',*profile],cwd=root,env=env,stdout=stream,stderr=subprocess.STDOUT)
            result['profiles'].append({'arguments':profile,'receipt':receipt.name,'log':log.name,'returncode':completed.returncode})
            if completed.returncode:
                returncode=completed.returncode;print(log.read_text());break
        result.update(seconds=time.perf_counter()-start,returncode=returncode,after={'cargo_home_bytes':size(cache),'target_bytes':size(root/'rust/target')})
        records.append(result);(a.output/'samples.json').write_text(json.dumps(records,indent=2));print(json.dumps(result),flush=True)
        if returncode:raise SystemExit(returncode)
