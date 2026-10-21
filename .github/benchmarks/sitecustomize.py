"""Observation only: preserve guide dependency receipts before temporary cleanup."""
import hashlib,json,os,tempfile
from pathlib import Path
original=tempfile.TemporaryDirectory.cleanup
def cleanup(self):
 root=Path(self.name);dest=os.getenv('DOCS_BENCH_RESULTS')
 if dest and root.name.startswith(('slackblocks-rust-guides-','slackblocks-multiselect-')) and root.exists():
  prefix='rust-guides' if root.name.startswith('slackblocks-rust-guides-') else 'multiselect'
  receipts={}
  for filename in ['Cargo.lock','Cargo.toml']:
   for f in root.rglob(filename):
    raw=f.read_text();normalized=raw.replace(os.environ['DOCS_BENCH_WORK'],'/SOURCE').replace(str(root),'/TEMP')
    key=str(f.relative_to(root));receipts[key]=normalized
  if receipts:(Path(dest)/(prefix+'-temporary-dependencies.json')).write_text(json.dumps(receipts,indent=2)+'\n')
 return original(self)
tempfile.TemporaryDirectory.cleanup=cleanup
