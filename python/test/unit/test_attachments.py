from __future__ import annotations

from slackblocks import Attachment, Color, SectionBlock

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
