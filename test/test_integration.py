from os import environ
from slack import WebClient
from slackblocks import Attachment, Color, Message, SectionBlock


def test_basic_attachment_message() -> None:
    block = SectionBlock("Hello, world!")
    attachment = Attachment(blocks=block, color=Color.BLACK)
    message = Message(channel="#general", attachments=[attachment, ])
    client = WebClient(token=environ['SLACK_BOT_TOKEN'])
    response = client.chat_postMessage(**message)
    assert response.status_code == 200
