from __future__ import annotations

import ast
import json
import re
import runpy
from pathlib import Path
from typing import Any

import pytest

REPO_ROOT = Path(__file__).resolve().parents[3]
USING_BLOCKS = REPO_ROOT / "docs/docs/usage/using_blocks.mdx"

# Block sections in using_blocks.mdx whose Python snippet cannot be executed
# and compared against the JSON tab by this harness. Empty today: every
# section parses and round-trips. Add a section title here (with a comment
# explaining why) only if a future section's Tabs structure resists parsing.
EXCLUDED_SECTIONS: set[str] = set()

PYTHON_FENCE = re.compile(r"```python\n(.*?)```", re.DOTALL)
JSON_FENCE = re.compile(r"```json\n(.*?)```", re.DOTALL)
HEADING = re.compile(r"^## (.+)$", re.MULTILINE)


def block_sections() -> list[pytest.param]:
    """Extract (python snippet, JSON tab) pairs from every block section."""
    text = USING_BLOCKS.read_text(encoding="utf-8")
    headings = list(HEADING.finditer(text))
    sections = []
    for heading, next_heading in zip(headings, [*headings[1:], None], strict=True):
        title = heading.group(1)
        if title in EXCLUDED_SECTIONS:
            continue
        body = text[heading.end() : next_heading.start() if next_heading else len(text)]
        code = PYTHON_FENCE.search(body)
        expected = JSON_FENCE.search(body)
        if code and expected:
            sections.append(pytest.param(code.group(1), json.loads(expected.group(1)), id=title))
    return sections


def evaluate_snippet(code: str) -> Any:
    """Run a documentation snippet and return the value of its final expression."""
    module = ast.parse(code)
    *statements, last = module.body
    assert isinstance(last, ast.Expr), "snippet must end with a block expression"
    namespace: dict[str, Any] = {}
    exec(compile(ast.Module(body=statements, type_ignores=[]), "<docs>", "exec"), namespace)
    return eval(compile(ast.Expression(body=last.value), "<docs>", "eval"), namespace)


def test_section_hello_example_matches_the_documented_json() -> None:
    namespace = runpy.run_path(REPO_ROOT / "docs/examples/python/section_hello.py")
    expected = json.loads(
        (REPO_ROOT / "docs/examples/section_hello.json").read_text(encoding="utf-8")
    )
    assert namespace["payload"] == expected


def test_every_block_section_is_covered() -> None:
    """Every block section in the guide must be picked up by the harness."""
    titles = HEADING.findall(USING_BLOCKS.read_text(encoding="utf-8"))
    covered = {parameters.id for parameters in block_sections()}
    assert covered == set(titles) - EXCLUDED_SECTIONS


@pytest.mark.parametrize(("code", "expected"), block_sections())
def test_block_section_matches_the_documented_json(code: str, expected: Any) -> None:
    block = evaluate_snippet(code)
    assert json.loads(repr(block)) == expected
