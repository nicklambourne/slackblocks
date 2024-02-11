from slackblocks import (
    Attachment, Color, Message, MessageResponse, ResponseType, SectionBlock, Text, WebhookMessage
) 


def test_basic_message() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = Message(channel="#slackblocks", blocks=block)
    with open("test/samples/messages/message_basic.json", "r") as expected:
        assert repr(message) == expected.read()


def test_message_with_optional_arguments() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = Message(
        channel="#slackblocks",
        blocks=block,
        unfurl_links=False,
        unfurl_media=False,
    )
    with open(
        "test/samples/messages/message_with_optional_arguments.json", "r"
    ) as expected:
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
    with open("test/samples/messages/message_with_attachments.json", "r") as expected:
        assert repr(message) == expected.read()


def test_message_response() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = MessageResponse(blocks=block, ephemeral=True)
    with open("test/samples/messages/message_response.json", "r") as expected:
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
                "text": {"type": "mrkdwn", "text": "Hello, world!"},
            }
        ],
        "text": "",
        "replace_original": False,
        "response_type": "ephemeral",
    }


def test_basic_webhook_message() -> None:
    with open("test/samples/messages/webhook_message_basic.json", "r") as expected:
        assert repr(
            WebhookMessage(
                blocks=[
                    SectionBlock(
                        Text("You wouldn't do ol' Hook in now, would you, lad?"),
                        block_id="fake_block_id",
                    ),
                    SectionBlock(
                        Text("Well, all right... if you... say you're a codfish."),
                        block_id="fake_block_id",
                    )
                ],
                response_type=ResponseType.EPHEMERAL,
                replace_original=True,
                unfurl_links=False,
                unfurl_media=False,
                metadata={
                    "sender": "Walt",
                }
            )
        ) == expected.read()


def test_webhook_message_delete() -> None:
    with open("test/samples/messages/webhook_message_delete.json", "r") as expected:
        assert repr(
            WebhookMessage(
                attachments=[
                    Attachment(blocks=[
                        SectionBlock(
                            Text("I'M A CODFISH!"),
                            block_id="fake_block_id",
                        )
                    ])
                ],
                blocks=[
                    SectionBlock(
                        Text("I'm a codfish."),
                        block_id="fake_block_id",
                    ),
                    SectionBlock(
                        Text("Louder!"),
                        block_id="fake_block_id",
                    )
                ],
                response_type="in_channel",
                delete_original=True,
                unfurl_links=True,
                unfurl_media=True,
                metadata={
                    "sender": "Walt",
                }
            )
        ) == expected.read()