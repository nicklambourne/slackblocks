from typing import List, Optional
from .blocks import Block


class Message:
    def __init__(self, blocks: Optional[List[Block]]):
        self.blocks = blocks
