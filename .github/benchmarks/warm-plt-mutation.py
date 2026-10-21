"""Separate untimed negative probe: a warm PLT must not mask invalid contracts."""
from pathlib import Path
import os
import subprocess

package = Path(os.environ['SOURCE']) / 'elixir'
out = Path(os.environ['EVIDENCE'])
probe = package / 'lib/bench_cache_mutation.ex'
assert not probe.exists()
probe.write_text('defmodule BenchCacheMutation do\n  @spec value() :: integer()\n  def value(), do: :not_an_integer\nend\n')
try:
    result = subprocess.run(['mix', 'dialyzer'], cwd=package, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    (out / 'warm-plt-mutation.log').write_text(result.stdout)
    assert result.returncode != 0, 'Dialyzer accepted an invalid contract with warm PLTs'
    assert 'invalid_contract' in result.stdout and 'bench_cache_mutation.ex' in result.stdout, result.stdout
finally:
    probe.unlink()
print('Warm PLT analysis rejected the invalid contract; negative probe passed.')
