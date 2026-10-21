"""Observe successful real Mix/Rebar subprocesses without changing their behavior."""
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import uuid

_original_run = subprocess.run


def _run(*args, **kwargs):
    result = _original_run(*args, **kwargs)
    command = kwargs.get('args', args[0] if args else None)
    output = os.environ.get('BENCH_OBSERVATIONS')
    if not output or not isinstance(command, (list, tuple)) or not command:
        return result
    name = Path(str(command[0])).name
    if name not in ['mix', 'rebar3'] or result.returncode != 0:
        return result
    cwd = Path(kwargs.get('cwd') or Path.cwd())
    row = dict(command=[str(x) for x in command], cwd=str(cwd), phase=os.environ.get('BENCH_PHASE'), locks={})
    for lock in ['mix.lock', 'rebar.lock']:
        if (cwd/lock).is_file():
            row['locks'][lock] = (cwd/lock).read_text()
    if name == 'mix' and 'hex.build' in command and '--output' in command:
        archive = Path(command[command.index('--output')+1])
        if archive.is_file():
            try:
                with tarfile.open(archive) as outer:
                    contents = outer.extractfile('contents.tar.gz').read()
                    row['metadata'] = outer.extractfile('metadata.config').read().decode()
                with tarfile.open(fileobj=io.BytesIO(contents), mode='r:gz') as inner:
                    row['archive_files'] = {item.name:hashlib.sha256(inner.extractfile(item).read()).hexdigest()
                                            for item in inner.getmembers() if item.isfile()}
            except (tarfile.TarError, KeyError, AttributeError) as error:
                row['observation_error'] = str(error)
    Path(output, f'{uuid.uuid4().hex}.json').write_text(json.dumps(row,indent=2))
    return result


subprocess.run = _run
