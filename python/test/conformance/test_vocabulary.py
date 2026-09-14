"""The Python icon and surface tables must match spec/vocabulary.json."""

from __future__ import annotations

import json
from pathlib import Path
from typing import get_args

from slackblocks import objects
from slackblocks._surfaces import _BLOCK_TYPES_BY_SURFACE

VOCABULARY = json.loads(
    (Path(__file__).resolve().parents[3] / "spec/vocabulary.json").read_text(encoding="utf-8")
)


def test_slack_icon_names_match_the_shared_vocabulary() -> None:
    expected = set(VOCABULARY["slack_icon_names"])
    assert set(objects._SLACK_ICON_NAMES) == expected
    assert set(get_args(objects.SlackIconName)) == expected


def test_surface_block_types_match_the_shared_vocabulary() -> None:
    actual = {surface: set(types) for surface, types in _BLOCK_TYPES_BY_SURFACE.items()}
    expected = {surface: set(types) for surface, types in VOCABULARY["surface_block_types"].items()}
    assert actual == expected
