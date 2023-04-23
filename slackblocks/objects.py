from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Literal, Optional, Union

from slackblocks.errors import InvalidUsageError


TriggerActionsOnOptions = Literal["on_enter_pressed", "on_character_entered"]
FilterIncludeOptions = Literal["im", "mpim", "private", "public"]


def validate_list(objects: List[Any], class_: type, unique: bool = False) -> List[Any]:
    if unique:
        objects = list(set(objects))
    for object_ in objects:
        if not isinstance(object_, class_):
             raise InvalidUsageError(f"Type of {object_} ({type(object_)})) inconsistent with expected type {class_}.")
    return objects


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
            raise InvalidUsageError("This field cannon have the value None or \'\'")
        elif type(text) is str:
            if max_length and len(text) > max_length:
                raise InvalidUsageError(f"Text length exceeds Slack-imposed limit ({max_length})")
            return Text(text=text, type_=type_)
        else:
            if max_length and len(text) > max_length:
                raise InvalidUsageError(f"Text length exceeds Slack-imposed limit ({max_length})")
            return Text(text=text.text, type_=type_)

    def __str__(self) -> str:
        return dumps(self._resolve())


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
        option_group["options"] = [
            option._resolve() for option in self.options
        ]
        return option_group


class DispatchActionConfiguration(CompositionObject):
    def __init__(self, trigger_actions_on: List[TriggerActionsOnOptions]):
        if len(trigger_actions_on) > 2 or len(trigger_actions_on) < 1:
            raise InvalidUsageError(
                f"Field `trigger_actions_on` can only be one or both of "
                f"'on_enter_pressed' and 'on_character_entered', got "
                f"{trigger_actions_on}."
            )
        super().__init__(type_=CompositionObjectType.DISPATCH)
        self.trigger_actions_on = validate_list(trigger_actions_on, TriggerActionsOnOptions, unique=True)
    
    def _resolve(self) -> Dict[str, Any]:
        return {
            "trigger_actions_on": self.trigger_actions_on
        }


class ConversationFilter(CompositionObject):
    def __init__(
        self,
        include: Optional[List[FilterIncludeOptions]] = None,
        exclude_external_shared_channels: bool = False,
        exclude_bot_users: bool = False
    ):
        super().__init__(type_=CompositionObjectType.FILTER)
        self.include = validate_list(include, FilterIncludeOptions, unique=True)
        self.exclude_external_shared_channels = exclude_external_shared_channels
        self.exclude_bot_users = exclude_bot_users
    
    def _resolve(self) -> Dict[str, Any]:
        conversation_filter = self._attributes()
        if self.include:
            conversation_filter["include"] = self.include
        conversation_filter["exclude_external_shared_channels"] = self.exclude_external_shared_channels
        conversation_filter["exclude_bot_users"] = self.exclude_bot_users