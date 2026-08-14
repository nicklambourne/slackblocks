from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

from slackblocks import (
    Button,
    ContextBlock,
    DividerBlock,
    FileInput,
    HeaderBlock,
    HomeTabView,
    Image,
    InputBlock,
    LengthError,
    MissingRequiredError,
    MutualExclusivityError,
    NumberInput,
    Option,
    OverflowMenu,
    PlainText,
    RangeError,
    SectionBlock,
    SlackFile,
    StaticSelectMenu,
    Text,
    TypeMismatchError,
)
from slackblocks.errors import InvalidUsageError

if TYPE_CHECKING:
    from collections.abc import Callable

REPO_ROOT = Path(__file__).resolve().parents[3]
SPEC_ROOT = REPO_ROOT / "spec"


def load_json(path: Path) -> dict:
    return json.loads(path.read_text())


def test_valid_manifest_covers_the_entire_fixture_corpus() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    fixture_ids = {fixture["id"] for fixture in manifest["fixtures"]}
    files = {
        path.relative_to(SPEC_ROOT / "fixtures" / "valid").with_suffix("").as_posix()
        for path in (SPEC_ROOT / "fixtures" / "valid").rglob("*.json")
    }
    assert manifest["spec_version"] == "1.0.0"
    assert len(fixture_ids) == 79
    assert fixture_ids == files


def test_every_valid_fixture_is_exercised_by_a_python_construction_test() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    test_source = "\n".join(
        path.read_text() for path in (REPO_ROOT / "python" / "test").rglob("test_*.py")
    )
    missing = [
        fixture["id"]
        for fixture in manifest["fixtures"]
        if f"{fixture['id']}.json" not in test_source
    ]
    assert missing == []


def test_every_valid_fixture_contains_json() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    for fixture in manifest["fixtures"]:
        path = SPEC_ROOT / "fixtures" / "valid" / f"{fixture['id']}.json"
        json.loads(path.read_text())


def option(value: str = "a") -> Option:
    return Option(text=PlainText("A"), value=value)


INVALID_CASES: dict[str, Callable[[], object]] = {
    "text-empty": lambda: Text(""),
    "text-too-long": lambda: Text("x" * 3001),
    "section-missing-content": lambda: SectionBlock(),
    "section-text-too-long": lambda: SectionBlock("x" * 3001),
    "section-too-many-fields": lambda: SectionBlock(fields=["x"] * 11),
    "section-field-too-long": lambda: SectionBlock(fields=["x" * 2001]),
    "header-text-too-long": lambda: HeaderBlock("x" * 151),
    "button-text-too-long": lambda: Button(text="x" * 76, action_id="a"),
    "button-action-id-too-long": lambda: Button(text="A", action_id="x" * 256),
    "option-value-too-long": lambda: option("x" * 76),
    "overflow-too-many-options": lambda: OverflowMenu("a", [option(str(i)) for i in range(6)]),
    "static-select-options-and-groups": lambda: StaticSelectMenu(
        "a",
        options=[option()],
        option_groups=[object()],  # type: ignore[list-item]
    ),
    "image-url-and-slack-file": lambda: Image(
        image_url="https://example.com/image.png", slack_file=SlackFile(url=None, id="F123")
    ),
    "number-input-inverted-range": lambda: NumberInput(
        is_decimal_allowed=True, min_value=2, max_value=1
    ),
    "file-input-max-files-out-of-range": lambda: FileInput(action_id="a", max_files=11),
    "context-invalid-element": lambda: ContextBlock(elements=[DividerBlock()]),
    "input-invalid-element": lambda: InputBlock(label="Label", element=Button("A", "a")),
    "view-missing-blocks": lambda: HomeTabView(blocks=[]),
}

ERROR_CATEGORIES = {
    LengthError: "length-exceeded",
    RangeError: "out-of-range",
    MutualExclusivityError: "mutually-exclusive",
    TypeMismatchError: "type-mismatch",
    MissingRequiredError: "missing-required",
    InvalidUsageError: "invalid-usage",
}


@pytest.mark.parametrize(
    ("case_id", "expected_category"),
    [
        (case["id"], case["category"])
        for case in load_json(SPEC_ROOT / "fixtures" / "invalid" / "manifest.json")["cases"]
    ],
)
def test_invalid_case_category(case_id: str, expected_category: str) -> None:
    construction = INVALID_CASES[case_id]
    with pytest.raises(InvalidUsageError) as caught:
        construction()
    category = next(
        name for error_type, name in ERROR_CATEGORIES.items() if type(caught.value) is error_type
    )
    assert category == expected_category


def test_python_skiplist_is_empty() -> None:
    entries = [
        line
        for line in (REPO_ROOT / "python" / "conformance" / "skiplist.txt").read_text().splitlines()
        if line and not line.startswith("#")
    ]
    assert entries == []
