from slackblocks import Attachment, MessageResponse, Color, Message, SectionBlock


def test_basic_message() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = Message(channel="#slackblocks", blocks=block)
    with open("test/samples/message_simple.json", "r") as expected:
        assert repr(message) == expected.read()


def test_message_with_attachment() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    attachment = Attachment(blocks=block, color=Color.YELLOW)
    message = Message(
        channel="#slackblocks",
        attachments=[
            attachment,
        ],
    )
    with open("test/samples/message_with_attachments.json", "r") as expected:
        assert repr(message) == expected.read()


def test_message_response() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = MessageResponse(blocks=block, ephemeral=True)
    with open("test/samples/message_response.json", "r") as expected:
        assert repr(message) == expected.read()


def test_to_dict() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = MessageResponse(blocks=block, ephemeral=True)
    assert message.to_dict() == {
        "mrkdwn": True,
        "blocks": [
            {
                "type": "section",
                "block_id": "fake_block_id",
                "text": {"type": "mrkdwn", "text": "Hello, world!", "verbatim": False},
            }
        ],
        "text": "",
        "replace_original": False,
        "response_type": "ephemeral",
    }
