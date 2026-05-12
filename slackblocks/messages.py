"""
Messages are the core unit of Slack messaging functionality. They can be
built out using blocks, elements, objects, and rich text features.

See: <https://api.slack.com/messaging>
"""

from __future__ import annotations

from enum import Enum
from json import dumps
from typing import Any

from slackblocks._core import resolve
from slackblocks.utils import coerce_to_list

from .attachments import Attachment
from .blocks import Block
from .errors import InvalidUsageError


class ResponseType(Enum):
    """
    Types of messages that can be sent via `WebhookMessage`.
    """

    EPHEMERAL = "ephemeral"
    IN_CHANNEL = "in_channel"

    @staticmethod
    def get_value(value: ResponseType | str) -> str:
        if isinstance(value, ResponseType):
            return value.value
        if value not in [response_type.value for response_type in ResponseType]:
            raise InvalidUsageError("ResponseType must be either `ephemeral` or `in_channel`")
        return value


class _MessagePayloadMixin:
    """Shared serialization helpers used by both BaseMessage and
    WebhookMessage.

    Both expose the same interface (``to_dict()``, ``json()``, ``__repr__``,
    ``__getitem__``, ``keys()``) so that ``**msg`` unpacking works with the
    Slack Web/Webhook clients. Concrete subclasses must implement
    ``_resolve``."""

    def _resolve(self) -> dict[str, Any]:  # pragma: no cover - overridden
        raise NotImplementedError

    def to_dict(self) -> dict[str, Any]:
        return self._resolve()

    def json(self) -> str:
        return dumps(self._resolve(), indent=4)

    def __repr__(self) -> str:
        return self.json()

    def __getitem__(self, item):
        return self._resolve()[item]

    def keys(self) -> list[str]:
        return list(self._resolve().keys())


class BaseMessage(_MessagePayloadMixin):
    """
    Abstract class for shared functionality between Messages and
    MessageResponses.
    """

    def __init__(
        self,
        channel: str | None = None,
        text: str | None = "",
        blocks: Block | list[Block] | None = None,
        attachments: Attachment | list[Attachment] | None = None,
        thread_ts: str | None = None,
        mrkdwn: bool = True,
    ) -> None:
        self.blocks = coerce_to_list(blocks, class_=Block, allow_none=True)
        self.channel = channel
        self.text = text
        self.attachments = coerce_to_list(attachments, class_=Attachment, allow_none=True)
        self.thread_ts = thread_ts
        self.mrkdwn = mrkdwn

    def _resolve(self) -> dict[str, Any]:
        # The 'text' field is intentionally emitted even when it is an empty
        # string (Slack uses it as a notification fallback when blocks are
        # present). resolve() strips None but preserves "" -- exactly the
        # semantics we want here.
        return resolve(
            {
                "channel": self.channel if self.channel else None,
                "mrkdwn": self.mrkdwn,
                "blocks": self.blocks if self.blocks else None,
                "attachments": self.attachments if self.attachments else None,
                "thread_ts": self.thread_ts if self.thread_ts else None,
                "text": self.text if (self.text or self.text == "") else None,
            }
        )


class Message(BaseMessage):
    """
    A Slack message object that can be converted to a JSON string for use with
    the Slack message API.

    Args:
        channel: the Slack channel to send the message to, e.g. "#general".
        text: markdown text to send in the message. If `blocks` are provided
            then this is a fallback to display in notifications.
        blocks: a list of [`Blocks`](/slackblocks/latest/reference/blocks) to form the contents
            of the message instead of the contents of `text`.
        attachments: a list of
            [`Attachments`](/slackblocks/latest/reference/attachments/#attachments.Attachment)
            that form the secondary contents of the message (deprecated).
        thread_ts: the timestamp ID of another unthreaded message that will
            become the parent message of this message (now a reply in a thread).
        mrkdwn: if `True` the contents of `text` will be rendered as markdown
            rather than plain text.
        unfurl_links: if `True`, links in the message will be automatically
            unfurled.
        unfurl_media: if `True`, media from links (e.g. images) will
            automatically unfurl.
    Throws:
        InvalidUsageException: in the event that the items passed to `blocks`
            are not valid [`Blocks`](/slackblocks/latest/reference/blocks).
    """

    def __init__(
        self,
        channel: str,
        text: str | None = "",
        blocks: list[Block] | Block | None = None,
        attachments: list[Attachment] | None = None,
        thread_ts: str | None = None,
        mrkdwn: bool = True,
        unfurl_links: bool | None = None,
        unfurl_media: bool | None = None,
    ) -> None:
        super().__init__(channel, text, blocks, attachments, thread_ts, mrkdwn)
        self.unfurl_links = unfurl_links
        self.unfurl_media = unfurl_media

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **super()._resolve(),
                "unfurl_links": self.unfurl_links,
                "unfurl_media": self.unfurl_media,
            }
        )


class MessageResponse(BaseMessage):
    """
    A required, immediate response that confirms your app received the payload.
    """

    def __init__(
        self,
        text: str | None = "",
        blocks: list[Block] | Block | None = None,
        attachments: list[Attachment] | None = None,
        thread_ts: str | None = None,
        mrkdwn: bool = True,
        replace_original: bool = False,
        ephemeral: bool = False,
    ) -> None:
        super().__init__(
            text=text,
            blocks=blocks,
            attachments=attachments,
            thread_ts=thread_ts,
            mrkdwn=mrkdwn,
        )
        self.replace_original = replace_original
        self.ephemeral = ephemeral

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **super()._resolve(),
                "replace_original": self.replace_original,
                "response_type": "ephemeral" if self.ephemeral else None,
            }
        )


class WebhookMessage(_MessagePayloadMixin):
    """
    Messages sent via the Slack `WebhookClient` takes different arguments than
        those sent via the regular `WebClient`.

    See: <https://github.com/slackapi/python-slack-sdk/blob/7e71b73/slack_sdk/webhook/client.py#L28>

    Args:
        text: markdown text to send in the message. If `blocks` are provided
            then this is a fallback to display in notifications.
        attachments: a list of
            [`Attachments`](/slackblocks/latest/reference/attachments/#attachments.Attachment)
            that form the secondary contents of the message (deprecated).
        blocks: a list of [`Blocks`](/slackblocks/latest/reference/blocks) to form the contents
            of the message instead of the contents of `text`.
        response_type: one of `ResponseType.EPHEMERAL` or `ResponseType.IN_CHANNEL`.
            Ephemeral messages are shown only to the requesting user whereas
            "in-channel" messages are shown to all channel participants.
        replace_original: when `True`, the message triggering this response will be
            replaced by this message. Mutually exclusive with `delete_original`.
        delete_original: when `True`, the original message triggering this response
            will be deleted, and any content of this message will be posted as a
            new message. Mutually exclusive with `replace_original`.
        unfurl_links: if `True`, links in the message will be automatically
            unfurled.
        unfurl_media: if `True`, media from links (e.g. images) will
            automatically unfurl.
        metadata: additional metadata to attach to the message.
        headers: HTTP request headers to include with the message.

    Throws:
        InvalidUsageError: when any of the passed fields fail validation.
    """

    def __init__(
        self,
        text: str | None = None,
        attachments: Attachment | list[Attachment] | None = None,
        blocks: Block | list[Block] | None = None,
        response_type: ResponseType | str | None = None,
        replace_original: bool | None = None,
        delete_original: bool | None = None,
        unfurl_links: bool | None = None,
        unfurl_media: bool | None = None,
        metadata: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> None:
        self.text = text
        self.attachments: list[Attachment] | None = coerce_to_list(
            attachments, Attachment, allow_none=True
        )
        self.blocks: list[Block] | None = coerce_to_list(blocks, Block, allow_none=True)
        self.response_type = (
            ResponseType.get_value(response_type) if response_type is not None else None
        )
        self.replace_original = replace_original
        self.delete_original = delete_original
        self.unfurl_links = unfurl_links
        self.unfurl_media = unfurl_media
        self.metadata = metadata
        self.headers = headers

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                "text": self.text,
                "attachments": [att for att in self.attachments if att is not None]
                if self.attachments is not None
                else None,
                "blocks": [block for block in self.blocks if block is not None]
                if self.blocks is not None
                else None,
                "response_type": self.response_type,
                "replace_original": self.replace_original,
                "delete_original": self.delete_original,
                "unfurl_links": self.unfurl_links,
                "unfurl_media": self.unfurl_media,
                "metadata": self.metadata,
                "headers": self.headers,
            }
        )
