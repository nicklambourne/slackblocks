from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Dict


class ElementType(Enum):
    TEXT = "text"


class TextType(Enum):
    MARKDOWN = "mrkdwn"
    PLAINTEXT = "plain_text"


class Element(ABC):
    def __init__(self, type_: ElementType):
        self.type = type_

    @abstractmethod
    def _resolve(self):
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

    def _resolve(self) -> Dict[str, str]:
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
        return dumps(self._resolve())
