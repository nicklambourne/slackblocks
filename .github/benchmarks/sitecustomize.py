"""Retain independent package evidence without changing production subprocesses."""
import hashlib, io, json, os, shutil, subprocess, tarfile, uuid
from pathlib import Path
original=subprocess.run
def observe(*args,**kwargs):
    result=original(*args,**kwargs);cmd=kwargs.get('args',args[0] if args else None);target=os.environ.get('BENCH_OBSERVATIONS')
    if not target or not isinstance(cmd,(tuple,list)) or not cmd or result.returncode:return result
    name=Path(str(cmd[0])).stem.lower()
    if name not in ['mix','erl']:return result
    cwd=Path(kwargs.get('cwd') or Path.cwd());root=Path(target);root.mkdir(exist_ok=True)
    row=dict(command=[str(x) for x in cmd],cwd=str(cwd),phase=os.environ.get('BENCH_PHASE'),locks={})
    if (cwd/'mix.lock').is_file():row['locks']['mix.lock']=(cwd/'mix.lock').read_text()
    if name=='mix' and 'hex.build' in cmd and '--output' in cmd:
        archive=Path(cmd[cmd.index('--output')+1]);shutil.copy2(archive,root/'slackblocks.tar')
        with tarfile.open(archive) as outer:
            row['metadata']=outer.extractfile('metadata.config').read().decode()
            with tarfile.open(fileobj=io.BytesIO(outer.extractfile('contents.tar.gz').read()),mode='r:gz') as inner:
                row['archive_files']={m.name:hashlib.sha256(inner.extractfile(m).read()).hexdigest() for m in inner.getmembers() if m.isfile()}
    (root/(uuid.uuid4().hex+'.json')).write_text(json.dumps(row,indent=2));return result
subprocess.run=observe
