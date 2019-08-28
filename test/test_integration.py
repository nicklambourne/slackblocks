from os import environ
from slack import WebClient
from slackblocks import Attachment, Color, ImageBlock, Message, SectionBlock


def test_basic_attachment_message() -> None:
    block = SectionBlock("Hello, world!")
    attachment = Attachment(blocks=block, color=Color.BLACK)
    message = Message(channel="#general", attachments=[attachment, ])
    client = WebClient(token=environ['SLACK_BOT_TOKEN'])
    response = client.chat_postMessage(**message)
    assert response.status_code == 200


def test_compound_message() -> None:
    block1 = SectionBlock("Block, One")
    block2 = SectionBlock("Block, Two")
    block3 = ImageBlock(image_url="http://bit.ly/slack-block-test-image", alt_text="crash")
    attachment1 = Attachment(blocks=block1, color=Color.PURPLE)
    attachment2 = Attachment(blocks=[block2, block3], color=Color.YELLOW)
    message = Message(channel="#general",
                      blocks=[block1, block3],
                      attachments=[attachment1, attachment2])
    client = WebClient(token=environ['SLACK_BOT_TOKEN'])
    response = client.chat_postMessage(**message)
    assert response.status_code == 200
