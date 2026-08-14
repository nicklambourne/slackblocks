from __future__ import annotations

import json
import runpy
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]


def test_section_hello_example_matches_the_documented_json() -> None:
    namespace = runpy.run_path(REPO_ROOT / "docs/examples/python/section_hello.py")
    expected = json.loads((REPO_ROOT / "docs/examples/section_hello.json").read_text())
    assert namespace["payload"] == expected
