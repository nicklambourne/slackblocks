"""
Messages are the core unit of Slack messaging functionality. They can be
built out using blocks, elements, objects, and rich text features.

See: <https://api.slack.com/messaging>
"""

from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

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
    def get_value(value: Union["ResponseType", str]) -> str:
        if isinstance(value, ResponseType):
            return value.value
        if value not in [response_type.value for response_type in ResponseType]:
            raise InvalidUsageError(
                "ResponseType must be either `ephemeral` or `in_channel`"
            )
        return value


class BaseMessage:
    """
    Abstract class for shared functionality between Messages and
    MessageResponses.
    """

    def __init__(
        self,
        channel: Optional[str] = None,
        text: Optional[str] = "",
        blocks: Optional[Union[List[Block], Block]] = None,
        attachments: Optional[List[Attachment]] = None,
        thread_ts: Optional[str] = None,
        mrkdwn: bool = True,
    ):
        self.blocks = coerce_to_list(blocks, class_=Block, allow_none=True)
        self.channel = channel
        self.text = text
        self.attachments = attachments or []
        self.thread_ts = thread_ts
        self.mrkdwn = mrkdwn

    def _resolve(self) -> Dict[str, Any]:
        message = dict()
        if self.channel:
            message["channel"] = self.channel
        message["mrkdwn"] = self.mrkdwn
        if self.blocks:
            message["blocks"] = [block._resolve() for block in self.blocks]
        if self.attachments:
            message["attachments"] = [
                attachment._resolve() for attachment in self.attachments
            ]
        if self.thread_ts:
            message["thread_ts"] = self.thread_ts
        if self.text or self.text == "":
            message["text"] = self.text
        return message

    def to_dict(self) -> Dict[str, Any]:
        return self._resolve()

    def json(self) -> str:
        return dumps(self._resolve(), indent=4)

    def __repr__(self) -> str:
        return self.json()

    def __getitem__(self, item):
        return self._resolve()[item]

    def keys(self) -> Dict[str, Any]:
        return self._resolve().keys()


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
        text: Optional[str] = "",
        blocks: Optional[Union[List[Block], Block]] = None,
        attachments: Optional[List[Attachment]] = None,
        thread_ts: Optional[str] = None,
        mrkdwn: bool = True,
        unfurl_links: Optional[bool] = None,
        unfurl_media: Optional[bool] = None,
    ) -> "Message":
        super().__init__(channel, text, blocks, attachments, thread_ts, mrkdwn)
        self.unfurl_links = unfurl_links
        self.unfurl_media = unfurl_media

    def _resolve(self) -> Dict[str, Any]:
        result = {**super()._resolve()}
        if self.unfurl_links is not None:
            result["unfurl_links"] = self.unfurl_links
        if self.unfurl_media is not None:
            result["unfurl_media"] = self.unfurl_media
        return result


class MessageResponse(BaseMessage):
    """
    A required, immediate response that confirms your app received the payload.
    """

    def __init__(
        self,
        text: Optional[str] = "",
        blocks: Optional[Union[List[Block], Block]] = None,
        attachments: Optional[List[Attachment]] = None,
        thread_ts: Optional[str] = None,
        mrkdwn: bool = True,
        replace_original: bool = False,
        ephemeral: bool = False,
    ):
        super().__init__(
            text=text,
            blocks=blocks,
            attachments=attachments,
            thread_ts=thread_ts,
            mrkdwn=mrkdwn,
        )
        self.replace_original = replace_original
        self.ephemeral = ephemeral

    def _resolve(self) -> Dict[str, Any]:
        result = {**super()._resolve(), "replace_original": self.replace_original}
        if self.ephemeral:
            result["response_type"] = "ephemeral"
        return result


class WebhookMessage:
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
        replace_orginal: when `True`, the message triggering this response will be
            replaced by this messaage. Mutually exclusive with `delete_original`.
        delete_original: when `True`, the original message triggering this response
            will be deleted, and any content of this message will be posted as a
            new message. Mutually exclusive with `replace_orginal`.
        unfurl_links: if `True`, links in the message will be automatically
            unfurled.
        unfurl_media: if `True`, media from links (e.g. images) will
            automatically unfurl.
        metadata: additional metadata to attach to the message.
        headres: HTTP request headers to include with the message.

    Throws:
        InvalidUsageError: when any of the passed fields fail validation.
    """

    def __init__(
        self,
        text: Optional[str] = None,
        attachments: Optional[List[Attachment]] = None,
        blocks: Optional[Union[List[Block], Block]] = None,
        response_type: Union[ResponseType, str] = None,
        replace_original: Optional[bool] = None,
        delete_original: Optional[bool] = None,
        unfurl_links: Optional[bool] = None,
        unfurl_media: Optional[bool] = None,
        metadata: Optional[Dict[str, Any]] = None,
        headers: Optional[Dict[str, str]] = None,
    ) -> "WebhookMessage":
        self.text = text
        self.attachments = coerce_to_list(attachments, Attachment, allow_none=True)
        self.blocks = coerce_to_list(blocks, Block, allow_none=True)
        self.response_type = ResponseType.get_value(response_type)
        self.replace_original = replace_original
        self.delete_original = delete_original
        self.unfurl_links = unfurl_links
        self.unfurl_media = unfurl_media
        self.metadata = metadata
        self.headers = headers

    def _resolve(self) -> None:
        webhook_message = {}
        if self.text is not None:
            webhook_message["text"] = self.text
        if self.attachments is not None:
            webhook_message["attachments"] = [
                attachment._resolve() for attachment in self.attachments
            ]
        if self.blocks is not None:
            webhook_message["blocks"] = [block._resolve() for block in self.blocks]
        if self.response_type is not None:
            webhook_message["response_type"] = self.response_type
        if self.replace_original is not None:
            webhook_message["replace_original"] = self.replace_original
        if self.delete_original is not None:
            webhook_message["delete_original"] = self.delete_original
        if self.unfurl_links is not None:
            webhook_message["unfurl_links"] = self.unfurl_links
        if self.unfurl_media is not None:
            webhook_message["unfurl_media"] = self.unfurl_media
        if self.metadata is not None:
            webhook_message["metadata"] = self.metadata
        if self.headers is not None:
            webhook_message["headers"] = self.headers
        return webhook_message

    def to_dict(self) -> Dict[str, Any]:
        return self._resolve()

    def json(self) -> str:
        return dumps(self._resolve(), indent=4)

    def __repr__(self) -> str:
        return self.json()

    def __getitem__(self, item):
        return self._resolve()[item]

    def keys(self) -> Dict[str, Any]:
        return self._resolve().keys()
