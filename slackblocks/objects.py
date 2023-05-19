from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

from .errors import InvalidUsageError


class CompositionObjectType(Enum):
    """
    Convenience class for referencing the various message elements Slack
    provides.
    """

    CONFIRM = "confirm"
    DISPATCH = "dispatch"
    FILTER = "conversations_select"
    OPTION = "option"
    OPTION_GROUP = "option_groups"
    TEXT = "text"


class CompositionObject:
    """
    Basis element containing attributes and behaviour common to all
    composition objects.
    N.B: CompositionObject is an abstract class and should not be
    instantiated directly.
    """

    def __init__(self, type_: CompositionObjectType):
        super().__init__()
        self.type = type_

    def _attributes(self) -> Dict[str, Any]:
        return {"type": self.type.value}

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        pass


class TextType(Enum):
    """
    Allowable types for Slack Text objects.
    N.B: some usages of Text objects only allow the plaintext variety.
    """

    MARKDOWN = "mrkdwn"
    PLAINTEXT = "plain_text"


class Text(CompositionObject):
    """
    An object containing some text, formatted either as plain_text or using
    Slack's "mrkdwn".
    """

    def __init__(
        self,
        text: str,
        type_: TextType = TextType.MARKDOWN,
        emoji: bool = False,
        verbatim: bool = False,
    ):
        super().__init__(type_=CompositionObjectType.TEXT)
        self.text_type = type_
        self.text = text
        if self.text_type == TextType.MARKDOWN:
            self.verbatim = verbatim
            self.emoji = None
        elif self.text_type == TextType.PLAINTEXT:
            self.verbatim = None
            self.emoji = emoji

    def _resolve(self) -> Dict[str, Any]:
        text = self._attributes()
        text["text"] = self.text
        if self.text_type == TextType.MARKDOWN:
            text["verbatim"] = self.verbatim
        elif self.type == TextType.PLAINTEXT and self.emoji:
            text["emoji"] = self.emoji
        return text

    @staticmethod
    def to_text(
        text: Optional[Union[str, "Text"]],
        force_plaintext=False,
        max_length: Optional[int] = None,
        allow_none: bool = False,
    ) -> Optional["Text"]:
        type_ = TextType.PLAINTEXT if force_plaintext else TextType.MARKDOWN
        if text is None:
            if allow_none:
                return None
            raise InvalidUsageError("This field cannon have the value None or ''")
        elif type(text) is str:
            if max_length and len(text) > max_length:
                raise InvalidUsageError(
                    f"Text length exceeds Slack-imposed limit (max_length)"
                )
            return Text(text=text, type_=type_)
        else:
            if max_length and len(text) > max_length:
                raise InvalidUsageError(
                    f"Text length exceeds Slack-imposed limit (max_length)"
                )
            return Text(text=text.text, type_=type_)

    def __str__(self) -> str:
        return dumps(self._resolve())

    def __eq__(self, other) -> bool:
        return (
            self.text_type == other.text_type and 
            self.text == other.text and 
            self.emoji == other.emoji and
            self.vertbatim == other.verbatim
        )


TextLike = Union[str, Text]


class ConfirmationDialogue(CompositionObject):
    """
    An object that defines a dialog that provides a confirmation step
    to any interactive element. This dialog will ask the user to confirm
    their action by offering confirm and deny buttons.
    """

    def __init__(
        self,
        title: Union[str, Text],
        text: Union[str, Text],
        confirm: Union[str, Text],
        deny: Union[str, Text],
    ):
        super().__init__(type_=CompositionObjectType.CONFIRM)
        self.title = Text.to_text(title, max_length=100, force_plaintext=True)
        self.text = Text.to_text(text, max_length=300)
        self.confirm = Text.to_text(confirm, max_length=30, force_plaintext=True)
        self.deny = Text.to_text(deny, max_length=30, force_plaintext=True)

    def _resolve(self) -> Dict[str, Any]:
        return {
            "title": self.title._resolve(),
            "text": self.text._resolve(),
            "confirm": self.confirm._resolve(),
            "deny": self.deny._resolve(),
        }


class Confirm(ConfirmationDialogue):
    """
    Alias for ConfirmationDialogue to retain backwards compatibility.
    """

    def __init__(self, *args, **kwargs):
        super(*args, **kwargs)


class Option(CompositionObject):
    """
    An object that represents a single selectable item in a select menu, multi-select
    menu, checkbox group, radio button group, or overflow menu.
    """

    def __init__(
        self,
        text: Union[str, Text],
        value: str,
        description: Optional[Union[str, Text]] = None,
        url: Optional[str] = None,
    ):
        super().__init__(type_=CompositionObjectType.OPTION)
        self.text = Text.to_text(text, max_length=75)
        self.value = Text.to_text(value, max_length=75)
        self.description = Text.to_text(
            description, max_length=75, force_plaintext=True, allow_none=True
        )
        if url and len(url) > 3000:
            raise InvalidUsageError("Option URLs must be less than 3000 characters")
        self.url = url

    def _resolve(self) -> Dict[str, Any]:
        option = self._attributes()
        option["text"] = self.text._resolve()
        option["value"] = self.value._resolve()
        if self.description is not None:
            option["description"] = self.description._resolve()
        if self.url is not None:
            option["url"] = self.url
        return option
    
    def __eq__(self, other) -> bool:
        return (
            self.type == other.type and
            self.text == other.text and
            self.value == other.value and
            self.description == other.description and
            self.url == other.url
        )


class OptionGroup(CompositionObject):
    """
    Provides a way to group options in a select menu or multi-select menu.
    """

    def __init__(self, label: Union[str, Text], options: List[Option]):
        super().__init__(type_=CompositionObjectType.OPTION_GROUP)
        self.label = Text.to_text(label, max_length=75)
        if options:
            self.options = options
        else:
            raise InvalidUsageError("Field `options` cannot be empty")

    def _resolve(self) -> Dict[str, Any]:
        option_group = self._attributes()
        option_group["label"] = self.label._resolve()
        option_group["options"] = [option._resolve() for option in self.options]
        return option_group


class DispatchActionConfiguration(CompositionObject):
    def __init__(self):
        raise NotImplementedError


class ConversationFilter(CompositionObject):
    def __init__(self):
        raise NotImplementedError
