"""
Messages are the core unit of Slack messaging functionality. They can be
built out using blocks, elements, objects, and rich text features.

See: <https://api.slack.com/messaging>
"""

from json import dumps
from typing import Any, Dict, List, Optional, Union

from slackblocks.utils import coerce_to_list

from .attachments import Attachment
from .blocks import Block


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
        channel:
        text: markdown text to send in the message. If `blocks` are provided
            then this is a fallback to display in notifications.
        blocks: a list of [`Blocks`](/reference/blocks) to form the contents
            of the message instead of the contents of `text`.
        attachments: a list of
            [`Attachments`](/reference/attachments/#attachments.Attachment)
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
            are not valid [`Blocks`](/reference/blocks).
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
