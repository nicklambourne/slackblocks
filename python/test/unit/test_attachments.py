from __future__ import annotations

import json

import pytest

from slackblocks import Attachment, Color, Field, Message, MissingRequiredError, SectionBlock

from .utils import fetch_sample


def test_single_attachment() -> None:
    block = SectionBlock("I like pretty colours", block_id="fake_block_id")
    attachment = Attachment(blocks=block, color=Color.BLACK, fallback="Colours preference")
    assert repr(attachment) == fetch_sample("attachments/attachment_simple.json")


def test_multi_block_attachment() -> None:
    block_0 = SectionBlock("I like pretty colours", block_id="fake_block_id_0")
    block_1 = SectionBlock("I don't like pretty colours", block_id="fake_block_id_1")
    attachment = Attachment(blocks=[block_0, block_1], color=Color.PURPLE)
    assert repr(attachment) == fetch_sample("attachments/attachment_multi_block.json")


def test_legacy_fields_survive_nested_message_serialization() -> None:
    attachment = Attachment(
        blocks=SectionBlock("Details", block_id="details"),
        fields=[Field("Status", "Ready", short=False), Field("Owner", "Test", short=True)],
        fallback="Status: Ready; Owner: Test",
    )
    payload = json.loads(Message(channel="C123", attachments=[attachment]).json())
    assert payload["attachments"][0]["fields"] == [
        {"title": "Status", "value": "Ready", "short": False},
        {"title": "Owner", "value": "Test", "short": True},
    ]
    assert payload["attachments"][0]["fallback"] == "Status: Ready; Owner: Test"


def test_blocks_only_omits_legacy_fields() -> None:
    assert "fields" not in json.loads(repr(Attachment(blocks=SectionBlock("Details"))))


def test_legacy_fields_only_uses_an_explicit_empty_block_list() -> None:
    attachment = Attachment(blocks=[], fields=Field("Status", "Ready"), fallback="Ready")
    assert json.loads(repr(attachment)) == {
        "fields": [{"title": "Status", "value": "Ready", "short": False}],
        "fallback": "Ready",
    }


def test_omitted_blocks_remains_a_required_constructor_argument() -> None:
    with pytest.raises(MissingRequiredError, match="blocks"):
        Attachment(fields=[Field("Status", "Ready")])
