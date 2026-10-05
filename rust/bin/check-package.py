#!/usr/bin/env python3
"""Verify the actual Cargo archive and consumers that cannot use checkout sources."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile

ROOT=Path(__file__).resolve().parents[2]
RUST=ROOT/'rust'
parser=argparse.ArgumentParser()
parser.add_argument('--toolchain',default='stable')
parser.add_argument('--lower-bounds',action='store_true')
parser.add_argument('--json-features',action='store_true')
args=parser.parse_args()
# Host-local policy is discovered explicitly; CI and other hosts use their normal temp location.
scratch=None
if Path('/Volumes/Repos').exists():
    for mount in ('Repos','Cache','Scratch'):
        if not os.path.ismount('/Volumes/'+mount):raise SystemExit(f'Required /Volumes/{mount} is not mounted')
    scratch='/Volumes/Scratch'
cargo=['cargo','+'+args.toolchain]
def run(arguments,cwd=RUST):subprocess.run(cargo+arguments,cwd=cwd,check=True)
metadata=json.loads(subprocess.check_output(cargo+['metadata','--format-version','1','--no-deps'],cwd=RUST))
package=next(p for p in metadata['packages'] if p['name']=='slackblocks')
version=package['version']
run(['package','--locked','--allow-dirty'])
archive=Path(metadata['target_directory'])/'package'/f'slackblocks-{version}.crate'
with tempfile.TemporaryDirectory(prefix='slackblocks-rust-package-',dir=scratch) as tmp:
    tmp=Path(tmp)
    with tarfile.open(archive) as tar:
        names={str(Path(n).relative_to(f'slackblocks-{version}')) for n in tar.getnames()}
        required={'Cargo.toml','README.md','LICENSE','LICENSE.BSD-3-Clause','CHANGELOG.md','src/lib.rs','examples/api_checkpoint.rs'}
        if not required<=names:raise SystemExit(f'Missing archive files: {required-names}')
        forbidden=[n for n in names if n.startswith(('generator/','conformance/','generated/','bin/','integrations/')) or '/Volumes/' in n]
        if forbidden:raise SystemExit(f'Unexpected archive files: {forbidden}')
        tar.extractall(tmp,filter='data')
    source=tmp/f'slackblocks-{version}'
    # Tests/doctests/examples inside the archive must not resolve ../spec or Python.
    run(['test','--locked','--all-targets'],source)
    run(['test','--locked','--doc'],source)
    run(['run','--locked','--example','api_checkpoint'],source)
    consumer=tmp/'consumer';(consumer/'src').mkdir(parents=True)
    serde_version='=1.0.228' if args.lower_bounds else '1.0.228'
    json_version='=1.0.145' if args.lower_bounds else '1.0.145'
    features=', features = ["arbitrary_precision", "preserve_order"]' if args.json_features else ''
    # The only path dependency is the extracted distributable, never the checkout.
    (consumer/'Cargo.toml').write_text(f'''[package]
name = "slackblocks-artifact-consumer"
version = "0.0.0"
edition = "2024"
rust-version = "1.85"
[workspace]
resolver = "3"
[dependencies]
slackblocks = {{ path = {json.dumps(str(source))} }}
serde = "{serde_version}"
serde_json = {{ version = "{json_version}"{features} }}
''')
    main=(source/'examples/api_checkpoint.rs').read_text()
    main=main.replace('    let option =', ''''    for literal in ["1.2345678901234567890123456789", "18446744073709551617", "1e-400", "1e400"] {
        assert!(serde_json::from_str::<JsonNumber>(literal).is_err(), "{literal}");
        assert!(serde_json::from_slice::<JsonNumber>(literal.as_bytes()).is_err(), "{literal}");
    }
    let exact: JsonNumber = serde_json::from_str("1e3")?;
    assert_eq!(exact.as_number().as_f64(), Some(1000.0));
    let option =''')
    (consumer/'src/main.rs').write_text(main)
    run(['run'],consumer)  # Deliberately fresh resolution: no lockfile or workspace override.
    info=json.loads(subprocess.check_output(cargo+['metadata','--format-version','1','--no-deps'],cwd=source))
    manifest=next(p['manifest_path'] for p in info['packages'] if p['name']=='slackblocks')
    assert str(source) in manifest and str(ROOT) not in manifest
print(f'Archive and fresh consumer passed on {args.toolchain}; lower_bounds={args.lower_bounds}; json_features={args.json_features}')
