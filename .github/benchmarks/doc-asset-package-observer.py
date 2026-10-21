#!/usr/bin/env python3
"""Observe the unchanged Hex checker before its independent temp consumers exit."""
import hashlib,io,json,os,runpy,subprocess,sys,tarfile,tempfile,time
from pathlib import Path
if not sys.argv[1].endswith('package_check.py'):os.execv(sys.executable,[sys.executable,*sys.argv[1:]])
out=Path(os.environ['PACKAGE_EVIDENCE']);out.mkdir(parents=True,exist_ok=True)
Original=tempfile.TemporaryDirectory
class Observed(Original):
 def __exit__(self,*args):
  root=Path(self.name)
  try:
   archive=root/'slackblocks.tar'
   if archive.exists():
    (out/'slackblocks.tar').write_bytes(archive.read_bytes())
    with tarfile.open(archive) as outer:
     with tarfile.open(fileobj=io.BytesIO(outer.extractfile('contents.tar.gz').read()),mode='r:gz') as inner:payload={m.name:hashlib.sha256(inner.extractfile(m).read()).hexdigest() for m in inner.getmembers() if m.isfile()}
    (out/'payload.json').write_text(json.dumps(payload,indent=2))
   lock=root/'consumer/mix.lock'
   if lock.exists():(out/'consumer.mix.lock').write_bytes(lock.read_bytes())
  finally:super().__exit__(*args)
tempfile.TemporaryDirectory=Observed
runpy.run_path(sys.argv[1],run_name='__main__')
