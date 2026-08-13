"""Tests for ``slackblocks.builder.block_kit_builder_url``."""

from __future__ import annotations

import json
from urllib.parse import unquote

import pytest

from slackblocks import (
    DividerBlock,
    HomeTabView,
    Message,
    ModalView,
    SectionBlock,
    WebhookMessage,
    block_kit_builder_url,
)


def _decode(url: str) -> dict[str, object]:
    """Pull the JSON payload back out of a Builder URL fragment."""
    fragment = url.split("#", 1)[1]
    return json.loads(unquote(fragment))


def test_returns_block_kit_builder_url() -> None:
    url = block_kit_builder_url(SectionBlock("Hi"))
    assert url.startswith("https://app.slack.com/block-kit-builder/#")


def test_single_block_is_wrapped_in_blocks_envelope() -> None:
    block = SectionBlock("Hi", block_id="b1")
    decoded = _decode(block_kit_builder_url(block))
    assert "blocks" in decoded
    assert len(decoded["blocks"]) == 1
    assert decoded["blocks"][0]["type"] == "section"
    assert decoded["blocks"][0]["block_id"] == "b1"


def test_list_of_blocks_is_wrapped_in_blocks_envelope() -> None:
    blocks = [
        SectionBlock("Heading", block_id="b1"),
        DividerBlock(block_id="b2"),
    ]
    decoded = _decode(block_kit_builder_url(blocks))
    assert "blocks" in decoded
    assert len(decoded["blocks"]) == 2
    assert [b["type"] for b in decoded["blocks"]] == ["section", "divider"]


def test_message_passes_through_unchanged() -> None:
    msg = Message(channel="#g", blocks=SectionBlock("Hi", block_id="b1"))
    decoded = _decode(block_kit_builder_url(msg))
    assert decoded["channel"] == "#g"
    assert decoded["mrkdwn"] is True
    assert len(decoded["blocks"]) == 1


def test_webhook_message_passes_through_unchanged() -> None:
    msg = WebhookMessage(text="hi", blocks=[SectionBlock("Hi", block_id="b1")])
    decoded = _decode(block_kit_builder_url(msg))
    assert decoded["text"] == "hi"
    assert len(decoded["blocks"]) == 1


def test_modal_view_passes_through_unchanged() -> None:
    view = ModalView(title="My modal", blocks=[SectionBlock("Hi", block_id="b1")])
    decoded = _decode(block_kit_builder_url(view))
    assert decoded["type"] == "modal"
    assert decoded["title"]["text"] == "My modal"


def test_home_tab_view_passes_through_unchanged() -> None:
    view = HomeTabView(blocks=[SectionBlock("Hi", block_id="b1")])
    decoded = _decode(block_kit_builder_url(view))
    assert decoded["type"] == "home"


def test_raw_dict_is_used_verbatim() -> None:
    """The escape hatch: pass any dict and it appears in the URL as-is."""
    raw = {"blocks": [{"type": "section", "text": {"type": "mrkdwn", "text": "Hi"}}]}
    decoded = _decode(block_kit_builder_url(raw))
    assert decoded == raw


def test_team_id_produces_workspace_specific_url() -> None:
    url = block_kit_builder_url(SectionBlock("Hi"), team_id="T0123ABCD")
    assert url.startswith("https://app.slack.com/block-kit-builder/T0123ABCD#")


def test_team_id_omitted_produces_generic_url() -> None:
    url = block_kit_builder_url(SectionBlock("Hi"))
    # No team ID between the slash and the '#' fragment delimiter.
    assert url.startswith("https://app.slack.com/block-kit-builder/#")


def test_payload_round_trips_through_url_decode() -> None:
    """The payload going in must equal the payload coming out, modulo
    block_id auto-generation (we pass an explicit one to make the test
    deterministic)."""
    msg = Message(channel="#g", blocks=SectionBlock("Special: <>&\"'", block_id="b1"))
    decoded = _decode(block_kit_builder_url(msg))
    # The block_id is preserved, the special characters in text survive
    # URL encoding intact.
    assert decoded["blocks"][0]["text"]["text"] == "Special: <>&\"'"
    assert decoded["blocks"][0]["block_id"] == "b1"


def test_unsupported_payload_type_raises() -> None:
    """Passing something that doesn't resolve to a dict (e.g. a string)
    should raise TypeError rather than produce garbage."""
    with pytest.raises(TypeError):
        block_kit_builder_url("just a string")  # type: ignore[arg-type]


def test_json_payload_uses_compact_separators() -> None:
    """The encoded JSON must use compact separators (no spaces) to keep
    URLs short. Verified by absence of '%20' (encoded space) in the
    fragment."""
    url = block_kit_builder_url(SectionBlock("Hi"))
    fragment = url.split("#", 1)[1]
    # The Hi text contains no space, so any %20 in the fragment would
    # come from JSON separator whitespace.
    assert "%20" not in fragment
