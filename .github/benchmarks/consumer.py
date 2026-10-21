"""Independent fixed-source consumer, excluded from production phase timings."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import time

root = Path(os.environ['SOURCE']).resolve()
out = Path(os.environ['EVIDENCE']).resolve()
work = Path(os.environ['RUNNER_TEMP'])/'independent-go-consumer'
assert not work.exists()
work.mkdir(parents=True)
env = dict(os.environ, GOCACHE=str(work/'build'), GOMODCACHE=str(work/'modules'), GOWORK='off')
record = {'source': os.environ['SOURCE_SHA'], 'phases': []}
def run(name, args, cwd):
    start = time.perf_counter()
    with (out/(name+'.log')).open('w') as log:
        result = subprocess.run(args, cwd=cwd, env=env, stdout=log, stderr=subprocess.STDOUT, text=True)
    record['phases'].append({'name': name, 'seconds': time.perf_counter()-start, 'returncode': result.returncode})
    result.check_returncode()
try:
    archive = work/'source.tar'
    run('consumer-archive', ['git','archive','--format=tar','--output='+str(archive),os.environ['SOURCE_SHA'],'go','docs/examples/go/section_hello.go'], root)
    record['archive_sha256'] = hashlib.sha256(archive.read_bytes()).hexdigest()
    extracted = work/'extracted'
    with tarfile.open(archive) as tf:
        tf.extractall(extracted, filter='data')
    consumer = work/'app'; consumer.mkdir()
    shutil.copy2(extracted/'docs/examples/go/section_hello.go', consumer/'main.go')
    run('consumer-init',['go','mod','init','example.com/slackblocks-cache-probe'],consumer)
    run('consumer-replace',['go','mod','edit','-replace=github.com/nicklambourne/slackblocks/go/v2='+str(extracted/'go')],consumer)
    run('consumer-resolve',['go','mod','tidy'],consumer)
    run('consumer-execute',['go','run','.'],consumer)
    record['wire'] = json.loads((out/'consumer-execute.log').read_text())
    assert record['wire']['channel']=='C0123456'
    assert record['wire']['blocks'][0]['text']['text']=='Hello from slackblocks!'
    record['go_sum_sha256'] = hashlib.sha256((consumer/'go.sum').read_bytes()).hexdigest()
    record['returncode'] = 0
except BaseException as e:
    record['returncode'] = 1; record['error'] = repr(e); raise
finally:
    (out/'consumer.json').write_text(json.dumps(record,indent=2))
