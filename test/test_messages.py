from slackblocks import Attachment, AcknowledgementResponse, Color, Message,\
    SectionBlock


def test_basic_message() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = Message(channel="#slackblocks", blocks=block)
    with open("test/samples/message_simple.json", "r") as expected:
        assert repr(message) == expected.read()


def test_message_with_attachment() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    attachment = Attachment(blocks=block, color=Color.YELLOW)
    message = Message(channel="#slackblocks", attachments=[attachment, ])
    with open("test/samples/message_with_attachments.json", "r") as expected:
        assert repr(message) == expected.read()


def test_ack_response() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = AcknowledgementResponse(blocks=block)
    with open("test/samples/ack_response.json", "r") as expected:
        assert repr(message) == expected.read()
