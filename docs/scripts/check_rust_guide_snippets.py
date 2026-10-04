#!/usr/bin/env python3
"""Compile and execute the exact Rust README/guide snippets; compare block-guide JSON."""
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
files = [ROOT / 'README.md', ROOT / 'rust/README.md'] + sorted((ROOT / 'docs/docs').rglob('*.mdx'))
cases = []
for file in files:
    if 'reference' in file.parts:
        continue
    source = file.read_text()
    snippets = list(re.finditer(r'```rust\n([\s\S]*?)\n```', source))
    for index, match in enumerate(snippets):
        code = match[1]
        assert 'fn main(' in code, f'{file}: Rust examples must be standalone and executable'
        expected = None
        if file.name == 'using_blocks.mdx':
            heading = list(re.finditer(r'^## (.+) Block$', source[:match.start()], re.M))[-1]
            next_heading = source.find('\n## ', match.end())
            section = source[match.end():next_heading if next_heading >= 0 else None]
            payload = re.search(r'```json\n([\s\S]*?)\n```', section)
            assert payload, f'{heading[1]}: missing independently documented JSON'
            expected = json.loads(payload[1])
        cases.append((f'snippet_{len(cases)}', str(file.relative_to(ROOT)), code, expected))
assert cases and any(expected is not None for _, _, _, expected in cases)
# Every concrete block guide must have an executable Rust/JSON pair.
blocks = (ROOT / 'docs/docs/usage/using_blocks.mdx').read_text()
assert sum(expected is not None for _, _, _, expected in cases) == len(re.findall(r'^## .+ Block$', blocks, re.M))
example = ROOT / "docs/examples/rust/section_hello.rs"
cases.append(("section_hello", str(example.relative_to(ROOT)), example.read_text(),
              json.loads((ROOT / "docs/examples/section_hello.json").read_text())))
scratch = None
if Path('/Volumes/Repos').exists():
    for mount in ('Repos', 'Cache', 'Scratch'):
        assert os.path.ismount('/Volumes/' + mount), f'Missing /Volumes/{mount}'
    scratch = '/Volumes/Scratch'
with tempfile.TemporaryDirectory(prefix='slackblocks-rust-guides-', dir=scratch) as directory:
    directory = Path(directory)
    binaries = directory / 'src/bin'
    binaries.mkdir(parents=True)
    (directory / 'Cargo.toml').write_text(f'''[package]
name = "slackblocks-guide-examples"
version = "0.0.0"
edition = "2024"
rust-version = "1.85"
[workspace]
resolver = "3"
[dependencies]
slackblocks = {{ path = {json.dumps(str(ROOT / 'rust'))} }}
serde_json = "1.0.145"
''')
    for name, _, code, _ in cases:
        (binaries / (name + '.rs')).write_text(code)
    toolchain = os.environ.get('RUST_GUIDE_TOOLCHAIN', 'stable')
    subprocess.run(['cargo', '+' + toolchain, 'build', '--bins'], cwd=directory, check=True)
    metadata = json.loads(subprocess.check_output(['cargo', '+' + toolchain, 'metadata', '--format-version', '1', '--no-deps'], cwd=directory))
    for name, file, _, expected in cases:
        executable = Path(metadata['target_directory']) / 'debug' / (name + ('.exe' if os.name == 'nt' else ''))
        output = subprocess.check_output([str(executable)], text=True)
        if expected is not None:
            assert json.loads(output) == expected, f'{file} {name}: Rust differs from documented JSON'
print(f'Executed {len(cases)} Rust README/guide/reusable examples on {toolchain}; every block example matches its JSON tab.')
