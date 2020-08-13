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
        self.text = text
        self.attachments = attachments or []
        self.thread_ts = thread_ts
        self.mrkdwn = mrkdwn

    def _resolve(self) -> Dict[str, Any]:
        message = dict()
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
        super().__init__(text, blocks, attachments, thread_ts, mrkdwn)
        self.channel = channel

    def _resolve(self) -> Dict[str, Any]:
        return {"channel": self.channel, **super()._resolve()}
