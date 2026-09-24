"""slackblocks/_limits.py must be the current rendering of spec/limits.json."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path

from slackblocks import _limits

ROOT = Path(__file__).resolve().parents[3]
LIMITS = json.loads((ROOT / "spec/limits.json").read_text(encoding="utf-8"))

_spec = importlib.util.spec_from_file_location(
    "generate_limits", ROOT / "python/generator/generate_limits.py"
)
assert _spec is not None and _spec.loader is not None
generator = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(generator)


def test_generated_limits_module_is_current() -> None:
    generated = (ROOT / "python/slackblocks/_limits.py").read_text(encoding="utf-8")
    assert generated == generator.render(LIMITS), (
        "python/slackblocks/_limits.py is stale; run python/generator/generate_limits.py"
    )


def test_limit_constants_match_the_shared_registry() -> None:
    expected = {path.replace(".", "_").upper(): value for path, value in generator.leaves(LIMITS)}
    actual = {name: value for name, value in vars(_limits).items() if name.isupper()}
    assert actual == expected
