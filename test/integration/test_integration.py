# TODO(nick): enable in GH actions

from os import environ

from slack import WebClient

from slackblocks import Attachment, Color, ImageBlock, Message, SectionBlock


def test_basic_attachment_message() -> None:
    block = SectionBlock("Hello, world!", block_id="block1")
    attachment = Attachment(blocks=block, color=Color.BLACK)
    message = Message(
        channel="#slackblocks",
        attachments=[
            attachment,
        ],
    )
    client = WebClient(token=environ["SLACK_BOT_TOKEN"])
    response = client.chat_postMessage(**message)
    assert response.status_code == 200
    with open("test/samples/message_basic_attachment.json", "r") as expected:
        assert repr(message) == expected.read()


def test_compound_message() -> None:
    block1 = SectionBlock("Block, One", block_id="fake_block1")
    block2 = SectionBlock("Block, Two", block_id="fake_block2")
    block3 = ImageBlock(
        image_url="http://bit.ly/slack-block-test-image",
        alt_text="crash",
        block_id="fake_block3",
    )
    attachment1 = Attachment(blocks=block1, color=Color.PURPLE)
    attachment2 = Attachment(blocks=[block2, block3], color=Color.YELLOW)
    message = Message(
        channel="#slackblocks",
        blocks=[block1, block3],
        attachments=[attachment1, attachment2],
    )
    client = WebClient(token=environ["SLACK_BOT_TOKEN"])
    response = client.chat_postMessage(**message)
    assert response.status_code == 200
    with open("test/samples/message_compound.json", "r") as expected:
        assert repr(message) == expected.read()
