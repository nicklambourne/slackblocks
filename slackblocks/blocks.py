from abc import abstractmethod, ABC
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union
from uuid import uuid4
from .elements import Element, Text, TextType


class BlockType(Enum):
    SECTION = "section"
    DIVIDER = "divider"
    IMAGE = "image"
    ACTIONS = "actions"
    CONTEXT = "context"


class Block(ABC):
    """
    Basis block containing attributes and behaviour common to all blocks.
    """
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
    """
    A section is one of the most flexible blocks available -
    it can be used as a simple text block, in combination with text fields,
    or side-by-side with any of the available block elements.
    """
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
        section["text"] = self.text._resolve()
        if self.fields:
            section["fields"] = self.fields
        if self.accessory:
            section["accessory"] = self.accessory
        return section


class DividerBlock(Block):
    """
    A content divider, like an <hr>, to split up different blocks inside of
    a message. The divider block is nice and neat, requiring only a type.
    """
    def __init__(self, block_id: Optional[str] = None):
        super().__init__(type_=BlockType.DIVIDER,
                         block_id=block_id)

    def _resolve(self):
        divider = self._attributes()
        return dumps(divider)


class ImageBlock(Block):
    """
    A simple image block, designed to make those cat photos really pop.
    """
    def __init__(self,
                 image_url: str,
                 alt_text: str = "",
                 title: Optional[Union[Text, str]] = None,
                 block_id: Optional[str] = None):
        super().__init__(type_=BlockType.IMAGE,
                         block_id=block_id)
        self.image_url = image_url
        self.alt_text = alt_text
        if type(title) == Text:
            if title.text_type == TextType.MARKDOWN:
                self.title = Text(text=title.text,
                                  type_=TextType.PLAINTEXT,
                                  emoji=title.emoji,
                                  verbatim=title.verbatim)
            else:
                self.title = title
        else:
            self.title = Text(text=title,
                              type_=TextType.PLAINTEXT)

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        image["title"] = self.title._resolve()
        return image


class ActionsBlock(Block):
    """
    A block that is used to hold interactive elements.
    """
    def __init__(self,
                 element: Optional[Element] = None,
                 elements: Optional[List[Element]] = None,
                 block_id: Optional[str] = None):
        super().__init__(type_=BlockType.ACTIONS,
                         block_id=block_id)
        self.all_elements = []
        if element:
            self.all_elements.append(element)
        if elements:
            self.all_elements.extend(elements)

    def _resolve(self):
        actions = self._attributes()
        actions["elements"] = [element._resolve() for element in self.all_elements]
        return actions


class ContextBlock():
    def __init__(self,
                 block_id: Optional[str]):
        super().__init__(block_id=block_id)
        self.type = BlockType.CONTEXT
