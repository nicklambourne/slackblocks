#!/usr/bin/env python3
"""Check positive PHP consumers and exact diagnostic identifiers for negative cases."""
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
base = ['php', str(ROOT/'vendor/bin/phpstan'), 'analyse', '-c', str(ROOT/'phpstan.neon'), '--no-progress', '--error-format=json']
subprocess.run(base + [str(ROOT/'typecheck/positive.php')], cwd=ROOT, check=True)
expected = {'keyword.php':'argument.unknown','text.php':'argument.type','enum.php':'argument.type','nested.php':'argument.type','list.php':'argument.type','readonly.php':'property.readOnlyAssignOutOfClass'}
result = subprocess.run(base + [str(ROOT/'typecheck/negative')], cwd=ROOT, text=True, capture_output=True)
if result.returncode != 1: raise SystemExit('Negative PHPStan cases did not fail as expected: '+result.stdout+result.stderr)
data = json.loads(result.stdout)
if data['errors']:raise SystemExit(data['errors'])
actual = {Path(path).name: {m['identifier'] for m in info['messages']} for path,info in data['files'].items()}
if set(actual) != set(expected) or any(actual[name] != {identifier} for name,identifier in expected.items()):
    raise SystemExit(f'Unexpected diagnostics: {actual}')
print(f'{len(expected)} negative PHP type contracts verified')
