from enum import Enum
from json import dumps
from typing import Dict


class ElementType(Enum):
    TEXT = "text"


class TextType(Enum):
    MARKDOWN = "mrkdwn"
    PLAINTEXT = "plain_text"


class Element:
    def __init__(self, type_: ElementType):
        self.type = type_


class Text(Element):
    def __init__(self,
                 text: str,
                 type_: TextType = TextType.MARKDOWN,
                 emoji: bool = False,
                 verbatim: bool = False):
        super().__init__(type_=ElementType.TEXT)
        self.text = text
        if self.type == TextType.MARKDOWN:
            self.verbatim = verbatim
            self.emoji = None
        elif self.type == TextType.PLAINTEXT:
            self.verbatim = None
            self.emoji = emoji

    def _attributes(self) -> Dict[str, str]:
        text = {
            "type": self.type.value,
            "text": self.text,
        }
        if self.type == TextType.MARKDOWN:
            text["verbatim"] = self.verbatim
        else:
            text["emoji"] = self.emoji
        return text

    def __str__(self) -> str:
        return dumps(self.__repr__())