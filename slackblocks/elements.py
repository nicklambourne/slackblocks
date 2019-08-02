from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict


class ElementType(Enum):
    TEXT = "text"
    IMAGE = "image"


class TextType(Enum):
    MARKDOWN = "mrkdwn"
    PLAINTEXT = "plain_text"


class Element(ABC):
    def __init__(self, type_: ElementType):
        self.type = type_

    def _attributes(self) -> Dict[str, Any]:
        return {
            "type": self.type.value
        }

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        pass


class Text(Element):
    def __init__(self,
                 text: str,
                 type_: TextType = TextType.MARKDOWN,
                 emoji: bool = False,
                 verbatim: bool = False):
        super().__init__(type_=ElementType.TEXT)
        self.text_type = type_
        self.text = text
        if self.text_type == TextType.MARKDOWN:
            self.verbatim = verbatim
            self.emoji = None
        elif self.text_type == TextType.PLAINTEXT:
            self.verbatim = None
            self.emoji = emoji

    def _resolve(self) -> Dict[str, Any]:
        text = {
            "type": self.text_type.value,
            "text": self.text,
        }
        if self.text_type == TextType.MARKDOWN:
            text["verbatim"] = self.verbatim
        elif self.type == TextType.PLAINTEXT and self.emoji:
            text["emoji"] = self.emoji
        return text

    def __str__(self) -> str:
        return dumps(self._resolve())


class Image(Element):
    """
    An element to insert an image - this element can be used in section
    and context blocks only. If you want a block with only an image in it,
    you're looking for the image block.
    """
    def __init__(self,
                 image_url: str,
                 alt_text: str):
        super().__init__(type_=ElementType.IMAGE)
        self.image_url = image_url
        self.alt_text = alt_text

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        return image

