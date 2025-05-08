from slackblocks import Attachment, Color, SectionBlock


def test_single_attachment() -> None:
    block = SectionBlock("I like pretty colours", block_id="fake_block_id")
    attachment = Attachment(
        blocks=block, color=Color.BLACK, fallback="Colours preference"
    )
    with open("test/samples/attachments/attachment_simple.json", "r") as expected:
        assert repr(attachment) == expected.read()


def test_multi_block_attachment() -> None:
    block_0 = SectionBlock("I like pretty colours", block_id="fake_block_id_0")
    block_1 = SectionBlock("I don't like pretty colours", block_id="fake_block_id_1")
    attachment = Attachment(blocks=[block_0, block_1], color=Color.PURPLE)
    with open("test/samples/attachments/attachment_multi_block.json", "r") as expected:
        assert repr(attachment) == expected.read()
