from __future__ import annotations

import pytest

from slackblocks import (
    Attachment,
    Color,
    InputBlock,
    LengthError,
    MarkdownBlock,
    Message,
    MessageResponse,
    PlainTextInput,
    PlanBlock,
    ResponseType,
    SectionBlock,
    TaskCardBlock,
    Text,
    TypeMismatchError,
    WebhookMessage,
)

from .utils import fetch_sample


def test_basic_message() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = Message(channel="#slackblocks", blocks=block)
    assert repr(message) == fetch_sample("messages/message_basic.json")


def test_message_with_optional_arguments() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = Message(
        channel="#slackblocks",
        blocks=block,
        unfurl_links=False,
        unfurl_media=False,
    )
    assert repr(message) == fetch_sample("messages/message_with_optional_arguments.json")


def test_message_with_attachment() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    attachment = Attachment(blocks=block, color=Color.YELLOW)
    message = Message(
        channel="#slackblocks",
        attachments=[
            attachment,
        ],
    )
    assert repr(message) == fetch_sample("messages/message_with_attachments.json")


def test_message_response() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    message = MessageResponse(blocks=block, ephemeral=True)
    assert repr(message) == fetch_sample("messages/message_response.json")


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
                ),
            ],
            response_type=ResponseType.EPHEMERAL,
            replace_original=True,
            unfurl_links=False,
            unfurl_media=False,
            metadata={
                "sender": "Walt",
            },
        )
    ) == fetch_sample("messages/webhook_message_basic.json")


def test_webhook_message_delete() -> None:
    assert repr(
        WebhookMessage(
            attachments=[
                Attachment(
                    blocks=[
                        SectionBlock(
                            Text("I'M A CODFISH!"),
                            block_id="fake_block_id",
                        )
                    ]
                )
            ],
            blocks=[
                SectionBlock(
                    Text("I'm a codfish."),
                    block_id="fake_block_id",
                ),
                SectionBlock(
                    Text("Louder!"),
                    block_id="fake_block_id",
                ),
            ],
            response_type="in_channel",
            delete_original=True,
            unfurl_links=True,
            unfurl_media=True,
            metadata={
                "sender": "Walt",
            },
        )
    ) == fetch_sample("messages/webhook_message_delete.json")


def test_messages_accept_input_blocks() -> None:
    block = InputBlock(label="Name", element=PlainTextInput(action_id="name"))
    assert Message(channel="C123", blocks=[block]).to_dict()["blocks"]


def test_pending_task_cards_are_only_valid_inside_a_plan() -> None:
    task = TaskCardBlock(task_id="task", title="Task", status="pending")
    Message(channel="C123", blocks=[PlanBlock(title="Plan", tasks=[task])])
    with pytest.raises(TypeMismatchError):
        Attachment(blocks=[task])
    with pytest.raises(TypeMismatchError):
        WebhookMessage(blocks=[task])


def test_markdown_total_counts_attachment_blocks() -> None:
    half = MarkdownBlock("x" * 6001)
    with pytest.raises(LengthError):
        MessageResponse(blocks=[half], attachments=[Attachment(blocks=[half])])
