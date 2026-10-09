#!/usr/bin/env python3
"""Check the public API's expected Rust diagnostics, with a passing companion."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser()
parser.add_argument('--toolchain', default='stable')
args = parser.parse_args()
scratch = None
if Path('/Volumes/Repos').exists():
    for mount in ('Repos', 'Cache', 'Scratch'):
        if not os.path.ismount('/Volumes/' + mount):
            raise SystemExit(f'Required /Volumes/{mount} is not mounted')
    scratch = '/Volumes/Scratch'
source = ROOT / 'rust/conformance/ui'
cases = json.loads((source / 'cases.json').read_text())
assert {p.stem for p in source.glob('*.rs')} == set(cases) | {'pass'}
with tempfile.TemporaryDirectory(prefix='slackblocks-rust-ui-', dir=scratch) as temp:
    temp = Path(temp)
    (temp / 'src').mkdir()
    (temp / 'Cargo.toml').write_text(f'''[package]
name = "slackblocks-api-contract"
version = "0.0.0"
edition = "2024"
rust-version = "1.85"
[workspace]
resolver = "3"
[dependencies]
slackblocks = {{ path = {json.dumps(str(ROOT / 'rust'))} }}
''')
    for name in ['pass', *cases]:
        (temp / 'src/main.rs').write_text((source / (name + '.rs')).read_text())
        result = subprocess.run(['cargo', '+' + args.toolchain, 'check', '--message-format=json'], cwd=temp, capture_output=True, text=True)
        diagnostics = [json.loads(line)['message'] for line in result.stdout.splitlines() if line.startswith('{') and json.loads(line).get('reason') == 'compiler-message']
        errors = [d for d in diagnostics if d['level'] == 'error']
        if name == 'pass':
            assert result.returncode == 0, result.stdout + result.stderr
        else:
            expected = cases[name]
            assert result.returncode != 0 and errors, f'{name}: unexpectedly compiled'
            assert all(d.get('code') and d['code']['code'] == expected['code'] and expected['contains'] in d['rendered'] for d in errors), f'{name}: failed for the wrong reason: {errors}'
        print(f'{args.toolchain}: {name}: expected diagnostics passed')
