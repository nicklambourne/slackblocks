from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, Optional, Union
from .errors import InvalidUsageError


class ElementType(Enum):
    """
    Convenience class for referencing the various message elements Slack
    provides.
    """
    TEXT = "text"
    IMAGE = "image"
    BUTTON = "button"
    CONFIRM = "confirm"


class TextType(Enum):
    """
    Allowable types for Slack Text objects.
    N.B: some usages of Text objects only allow the plaintext variety.
    """
    MARKDOWN = "mrkdwn"
    PLAINTEXT = "plain_text"


class Element(ABC):
    """
    Basis element containing attributes and behaviour common to all elements.
    N.B: Element is an abstract class and cannot be used directly.
    """
    def __init__(self, type_: ElementType):
        super().__init__()
        self.type = type_

    def _attributes(self) -> Dict[str, Any]:
        return {
            "type": self.type.value
        }

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        pass


class Text(Element):
    """
    An object containing some text, formatted either as plain_text or using
    Slack's "mrkdwn"
    """
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

    @staticmethod
    def to_text(text: Union[str, "Text"],
                force_plaintext=False,
                max_length: Optional[int] = None) -> "Text":
        type_ = TextType.PLAINTEXT if force_plaintext else TextType.MARKDOWN
        if type(text) is str:
            if max_length and len(text) > max_length:
                raise InvalidUsageError("Text length exceeds Slack-imposed limit")
            return Text(text=text,
                        type_=type_)
        else:
            if max_length and len(text) > max_length:
                raise InvalidUsageError("Text length exceeds Slack-imposed limit")
            return Text(text=text.text,
                        type_=type_)

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


class Confirm(Element):
    """
    An object that defines a dialog that provides a confirmation step
    to any interactive element. This dialog will ask the user to confirm
    their action by offering confirm and deny buttons.
    """
    def __init__(self,
                 title: Union[str, Text],
                 text: Union[str, Text],
                 confirm: Union[str, Text],
                 deny: Union[str, Text]):
        super().__init__(type_=ElementType.CONFIRM)
        self.title = Text.to_text(title, max_length=100, force_plaintext=True)
        self.text = Text.to_text(text, max_length=300)
        self.confirm = Text.to_text(confirm, max_length=30, force_plaintext=True)
        self.deny = Text.to_text(deny, max_length=30, force_plaintext=True)

    def _resolve(self) -> Dict[str, Any]:
        return {
            "title": self.title._resolve(),
            "text": self.text._resolve(),
            "confirm": self.confirm._resolve(),
            "deny": self.deny._resolve()
        }


class Button(Element):
    """
    An interactive element that inserts a button. The button can be a
    trigger for anything from opening a simple link to starting a complex
    workflow.
    """
    def __init__(self,
                 text: Union[str, Text],
                 action_id: str,
                 url: Optional[str] = None,
                 value: Optional[str] = None,
                 style: Optional[str] = None,
                 confirm: Optional[Confirm] = None):
        super().__init__(type_=ElementType.BUTTON)
        self.text = Text.to_text(text, max_length=75, force_plaintext=True)
        self.action_id = action_id
        self.url = url
        self.value = value
        self.style = style
        self.confirm = confirm

    def _resolve(self) -> Dict[str, Any]:
        button = self._attributes()
        button["text"] = self.text._resolve()
        button["action_id"] = self.action_id
        if self.style:
            button["style"] = self.style
        if self.url:
            button["url"] = self.url
        if self.value:
            button["value"] = self.value
        if self.confirm:
            button["confirm"] = self.confirm._resolve()
        return button
