from json import dumps
from typing import Any, Dict, List, Optional
from .attachments import Attachment
from .blocks import Block


class Message:
    """
    A Slack message object that can be converted to a JSON string for use with
    the Slack message API.
    """
    def __init__(self,
                 text: Optional[str],
                 blocks: Optional[List[Block]],
                 attachments: Optional[List[Attachment]],
                 thread_ts: Optional[str] = None,
                 mrkdwn: bool = True):
        self.blocks = blocks
        self.text = text
        self.attachments = attachments
        self.thread_ts = thread_ts
        self.mrkdwn = mrkdwn

    def _resolve(self) -> Dict[str, Any]:
        message = dict()
        message["blocks"] = [block._resolve() for block in self.blocks]
        message["attachments"] = [attachment._resolve() for attachment in self.attachments]
        message["mrkdwn"] = self.mrkdwn
        if self.thread_ts:
            message["thread_ts"] = self.thread_ts
        if self.text:
            message["text"] = self.text
        return message

    def __repr__(self) -> str:
        return dumps(self._resolve())
