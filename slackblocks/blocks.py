from abc import abstractmethod, ABC
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union
from uuid import uuid4
from .elements import Element, Text


class BlockType(Enum):
    SECTION = "section"
    DIVIDER = "divider"
    IMAGE = "image"
    ACTIONS = "actions"
    CONTEXT = "context"


class Block(ABC):
    def __init__(self,
                 type_: BlockType,
                 block_id: Optional[str] = None):
        self.type = type_
        self.block_id = block_id if block_id else str(uuid4())

    def _attributes(self):
        return {
            "type": self.type.value,
            "block_id": self.block_id
        }

    @abstractmethod
    def _resolve(self) -> Dict[str, any]:
        pass

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class SectionBlock(Block):
    def __init__(self,
                 text: Union[str, Text],
                 block_id: Optional[str] = None,
                 fields: Optional[List[Text]] = None,
                 accessory: Optional[Element] = None):
        super().__init__(type_=BlockType.SECTION,
                         block_id=block_id)
        if type(text) is Text:
            self.text = text
        else:
            self.text = Text(text)
        self.fields = fields
        self.accessory = accessory

    def _resolve(self) -> Dict[str, Any]:
        section = self._attributes()
        section["text"] = self.text._attributes()
        if self.fields:
            section["fields"] = self.fields
        if self.accessory:
            section["accessory"] = self.accessory
        return section


class DividerBlock(Block):
    def __init__(self, block_id: Optional[str] = None):
        super().__init__(type_=BlockType.DIVIDER,
                         block_id=block_id)

    def _resolve(self):
        divider = self._attributes()
        return dumps(divider)


class ImageBlock(Block):
    def __init__(self,
                 image_url: str,
                 alt_text: str = "",
                 title: Optional[str] = None,
                 block_id: Optional[str] = None):
        super().__init__(type_=BlockType.IMAGE,
                         block_id=block_id)
        self.image_url = image_url
        self.alt_text = alt_text
        self.title = title

    def _resolve(self) -> Dict[str, any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        image["title"] = self.title
        return image


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
