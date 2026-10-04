#!/usr/bin/env python3
"""Enforce line and region coverage separately for all production and handwritten code."""
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
manifest = ROOT / 'rust/Cargo.toml'
cargo = ['cargo', '+stable']
metadata = json.loads(subprocess.check_output(cargo + ['metadata', '--manifest-path', str(manifest), '--format-version', '1', '--no-deps']))
report = Path(metadata['target_directory']) / 'coverage.json'
subprocess.run(cargo + ['llvm-cov', '--manifest-path', str(manifest), '--workspace', '--all-targets', '--all-features', '--locked', '--json', '--output-path', str(report)], check=True)
data = json.loads(report.read_text())['data'][0]
source = (ROOT / 'rust/src').resolve()
files = {str(Path(f['filename']).resolve().relative_to(source)): f for f in data['files'] if Path(f['filename']).resolve().is_relative_to(source)}
# These files contain only module declarations, re-exports and constants. They
# have no executable regions; every other production file must be measured.
non_executable = {'lib.rs', 'generated/mod.rs', 'generated/constants.rs'}
expected = {str(p.relative_to(source)) for p in source.rglob('*.rs')} - non_executable
assert set(files) == expected, f'Coverage file inventory differs: {set(files) ^ expected}'
failed = False
for label, selected in [('overall', files), ('handwritten', {p: f for p, f in files.items() if not p.startswith('generated/')})]:
    for metric in ('lines', 'regions'):
        count = sum(f['summary'][metric]['count'] for f in selected.values())
        covered = sum(f['summary'][metric]['covered'] for f in selected.values())
        percent = covered / count * 100
        print(f'{label} {metric}: {covered}/{count} ({percent:.2f}%; required >= 90%)')
        failed |= percent < 90
if failed:
    raise SystemExit('Coverage gate failed; inspect uncovered production paths')
