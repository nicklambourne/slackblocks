from typing import List, Optional
from .blocks import Block


class Message:
    def __init__(self,
                 text: Optional[str],
                 blocks: Optional[List[Block]],
                 attachments,
                 thread_ts: Optional[str] = None,
                 mrkdwm: bool = True):
        self.blocks = blocks
        self.text = text
