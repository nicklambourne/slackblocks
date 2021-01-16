from json import dumps
from typing import Any, Dict, List, Optional, Union
from .attachments import Attachment
from .blocks import Block


class BaseMessage:
    """
    Abstract class for shared functionality between Messages and
    Acknowledgement responses.
    """
    def __init__(self,
                 channel: Optional[str] = None,
                 text: Optional[str] = "",
                 blocks: Optional[Union[List[Block], Block]] = None,
                 attachments: Optional[List[Attachment]] = None,
                 thread_ts: Optional[str] = None,
                 mrkdwn: bool = True):
        if isinstance(blocks, List):
            self.blocks = blocks
        elif isinstance(blocks, Block):
            self.blocks = [blocks, ]
        else:
            self.blocks = None
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
            message["attachments"] = [attachment._resolve() for attachment in self.attachments]
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
        return self._resolve()


class Message(BaseMessage):
    """
    A Slack message object that can be converted to a JSON string for use with
    the Slack message API.
    """
    def __init__(self,
                 channel: str,
                 text: Optional[str] = "",
                 blocks: Optional[Union[List[Block], Block]] = None,
                 attachments: Optional[List[Attachment]] = None,
                 thread_ts: Optional[str] = None,
                 mrkdwn: bool = True):
        super().__init__(channel, text, blocks, attachments, thread_ts, mrkdwn)


class MessageResponse(BaseMessage):
    """
    A required, immediate response that confirms your app received the payload.
    """
    def __init__(self,
                 text: Optional[str] = "",
                 blocks: Optional[Union[List[Block], Block]] = None,
                 attachments: Optional[List[Attachment]] = None,
                 thread_ts: Optional[str] = None,
                 mrkdwn: bool = True,
                 replace_original: bool = False,
                 ephemeral: bool = False):
        super().__init__(text=text,
                         blocks=blocks,
                         attachments=attachments,
                         thread_ts=thread_ts,
                         mrkdwn=mrkdwn)
        self.replace_original = replace_original
        self.ephemeral = ephemeral

    def _resolve(self) -> Dict[str, Any]:
        result = {**super()._resolve(),
                  "replace_original": self.replace_original}
        if self.ephemeral:
            result["response_type"] = "ephemeral"
        return result
