from abc import abstractmethod, ABC
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union
from uuid import uuid4
from .elements import Element, ElementType, Text, TextType
from .errors import InvalidUsageError


class BlockType(Enum):
    """
    Convenience class for identifying the different types of blocks available
    in the Slack Blocks API and their programmatic names.
    """
    SECTION = "section"
    DIVIDER = "divider"
    IMAGE = "image"
    ACTIONS = "actions"
    CONTEXT = "context"
    FILE = "file"


class Block(ABC):
    """
    Basis block containing attributes and behaviour common to all blocks.
    N.B: Block is an abstract class and cannot be sent directly.
    """
    def __init__(self,
                 type_: BlockType,
                 block_id: Optional[str] = None):
        self.type = type_
        self.block_id = block_id if block_id else str(uuid4())

    def __add__(self, other: "Block"):
        return [self, other]

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
            section["accessory"] = self.accessory._resolve()
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
        return self._attributes()


class ImageBlock(Block):
    """
    A simple image block, designed to make those cat photos really pop.
    """
    def __init__(self,
                 image_url: str,
                 alt_text: Optional[str] = "",
                 title: Optional[Union[Text, str]] = None,
                 block_id: Optional[str] = None):
        super().__init__(type_=BlockType.IMAGE,
                         block_id=block_id)
        self.image_url = image_url
        self.alt_text = alt_text
        if title and type(title) is Text:
            if title.text_type == TextType.MARKDOWN:
                self.title = Text(text=title.text,
                                  type_=TextType.PLAINTEXT,
                                  emoji=title.emoji,
                                  verbatim=title.verbatim)
            else:
                self.title = title
        elif title:
            self.title = Text(text=title,
                              type_=TextType.PLAINTEXT)
        else:
            self.title = Text(text=" ",
                              type_=TextType.PLAINTEXT)

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        if self.title:
            image["title"] = self.title._resolve()
        return image


class ActionsBlock(Block):
    """
    A block that is used to hold interactive elements.
    """
    def __init__(self,
                 elements: Optional[List[Element]] = None,
                 block_id: Optional[str] = None):
        super().__init__(type_=BlockType.ACTIONS,
                         block_id=block_id)
        if type(elements) is List[Element]:
            self.elements = elements
        elif type(elements) is Element:
            self.elements = [elements, ]

    def _resolve(self):
        actions = self._attributes()
        actions["elements"] = [element._resolve() for element in self.elements]
        return actions


class ContextBlock(Block):
    """
    Displays message context, which can include both images and text.
    """
    def __init__(self,
                 elements: Optional[List[Element]] = None,
                 block_id: Optional[str] = None):
        super().__init__(type_=BlockType.CONTEXT,
                         block_id=block_id)
        self.elements = []
        for element in elements:
            if element.type == ElementType.TEXT or \
                    element.type == ElementType.IMAGE:
                self.elements.append(element)
            else:
                raise InvalidUsageError("Context blocks can only hold image and text"
                                        "elements")
        if len(self.elements) > 10:
            raise InvalidUsageError("Context blocks can hold a maximum of ten elements")

    def _resolve(self) -> Dict[str, any]:
        context = self._attributes()
        context["elements"] = [element._resolve() for element in self.elements]
        return context


class FileBlock(Block):
    """
    Displays a remote file.
    """
    def __init__(self,
                 external_id: str,
                 source: str,
                 block_id: Optional[str]):
        super().__init__(type_=BlockType.FILE,
                         block_id=block_id)
        self.external_id = external_id
        self.source = source

    def _resolve(self) -> Dict[str, any]:
        file = self._attributes()
        file["external_id"] = self.external_id
        file["source"] = self.source
        return file
