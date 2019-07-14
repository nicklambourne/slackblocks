from enum import Enum
from json import dumps
from typing import List, Optional, Union
from uuid import uuid4
from .elements import Element, Text


class BlockType(Enum):
    SECTION = "section"
    DIVIDER = "divider"
    IMAGE = "image"
    ACTIONS = "actions"
    CONTEXT = "context"


class Block:
    def __init__(self, block_id: Optional[str]):
        self.type = None
        self.block_id = block_id if block_id else uuid4()

    def _attributes(self):
        return {
            "type": self.type.value,
            "block_id": self.block_id
        }


class SectionBlock(Block):
    def __init__(self,
                 text: Union[str, Text],
                 block_id: Optional[str],
                 fields: Optional[List[Text]],
                 accessory: Optional[Element]):
        super().__init__(block_id=block_id)
        self.type = BlockType.SECTION
        if type(text) is Text:
            self.text = text
        else:
            self.text = Text()
        self.fields = fields
        self.accessory = accessory

    def __repr__(self) -> str:
        section = self._attributes()
        section["text"] = self.text
        section["fields"] = self.fields
        section["accessory"] = self.accessory
        return dumps(section)

    def __str__(self) -> str:
        return self.__repr__()


class DividerBlock(Block):
    def __init__(self, block_id: Optional[str] = None):
        super().__init__(block_id=block_id)
        self.type = BlockType.DIVIDER


class ImageBlock(Block):
    def __init__(self,
                 image_url: str,
                 alt_text: str = "",
                 title: Optional[str] = None,
                 block_id: Optional[str] = None):
        super().__init__(block_id=block_id)
        self.type = BlockType.IMAGE
        self.image_url = image_url
        self.alt_text = alt_text
        self.title = title


class ActionsBlock(Block):
    def __init__(self,
                 element: Optional[Element],
                 elements: Optional[List[Element]],
                 block_id: Optional[str]):
        super().__init__(block_id=block_id)
        self.type = BlockType.ACTIONS


class ContextBlock():
    def __init__(self,
                 block_id: Optional[str]):
        super().__init__(block_id=block_id)
        self.type = BlockType.CONTEXT