"""
Composition objects used inside of Block objects.
See: https://api.slack.com/reference/block-kit/composition-objects?ref=bk
"""

from abc import abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

from slackblocks.errors import InvalidUsageError
from slackblocks.utils import coerce_to_list, validate_string


class CompositionObjectType(Enum):
    """
    Convenience class for referencing the various message elements Slack
    provides.
    """

    CONFIRM = "confirm"
    DISPATCH = "dispatch"
    FILTER = "conversations_select"
    INPUT_PARAMETER = "input_parameter"
    OPTION = "option"
    OPTION_GROUP = "option_groups"
    TEXT = "text"
    TRIGGER = "trigger"
    WORKFLOW = "workflow"


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

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


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
        self.text = validate_string(
            text, field_name="text", min_length=1, max_length=3000
        )
        if self.text_type == TextType.MARKDOWN:
            self.verbatim = verbatim
            self.emoji = None
        elif self.text_type == TextType.PLAINTEXT:
            self.verbatim = None
            self.emoji = emoji

    def _resolve(self) -> Dict[str, Any]:
        text = self._attributes()
        text["type"] = self.text_type.value
        text["text"] = self.text
        if self.text_type == TextType.MARKDOWN and self.verbatim:
            text["verbatim"] = self.verbatim
        elif self.text_type == TextType.PLAINTEXT and self.emoji:
            text["emoji"] = self.emoji
        return text

    @staticmethod
    def to_text(
        text: Optional[Union[str, "Text"]],
        force_plaintext=False,
        max_length: Optional[int] = None,
        allow_none: bool = False,
    ) -> Optional["Text"]:
        original_type = text.text_type if isinstance(text, Text) else None
        type_ = (
            TextType.PLAINTEXT
            if force_plaintext
            else original_type or TextType.MARKDOWN
        )
        if text is None:
            if allow_none:
                return None
            raise InvalidUsageError("This field cannot have the value None or ''")
        elif isinstance(text, str):
            if max_length and len(text) > max_length:
                raise InvalidUsageError(
                    f"Text length exceeds Slack-imposed limit ({max_length})"
                )
            return Text(text=text, type_=type_)
        elif isinstance(text, Text):
            if max_length and len(text.text) > max_length:
                raise InvalidUsageError(
                    f"Text length exceeds Slack-imposed limit ({max_length})"
                )
            return Text(
                text=text.text, type_=type_, emoji=text.emoji, verbatim=text.verbatim
            )
        else:
            raise InvalidUsageError(
                f"Can only coerce Text object from `str` or `Text`, not `{type(text)}`"
            )

    def __str__(self) -> str:
        return dumps(self._resolve())

    def __eq__(self, other) -> bool:
        return (
            self.text_type == other.text_type
            and self.text == other.text
            and self.emoji == other.emoji
            and self.vertbatim == other.verbatim
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
        self.value = validate_string(value, field_name="value", max_length=75)
        self.description = Text.to_text(
            description, max_length=75, force_plaintext=True, allow_none=True
        )
        if url and len(url) > 3000:
            raise InvalidUsageError("Option URLs must be less than 3000 characters")
        self.url = url

    def _resolve(self) -> Dict[str, Any]:
        option = {}  # Does not include type in JSON
        option["text"] = self.text._resolve()
        option["value"] = self.value
        if self.description is not None:
            option["description"] = self.description._resolve()
        if self.url is not None:
            option["url"] = self.url
        return option

    def __eq__(self, other) -> bool:
        return (
            self.type == other.type
            and self.text == other.text
            and self.value == other.value
            and self.description == other.description
            and self.url == other.url
        )


class OptionGroup(CompositionObject):
    """
    Provides a way to group options in a select menu or multi-select menu.
    """

    def __init__(self, label: Union[str, Text], options: List[Option]):
        super().__init__(type_=CompositionObjectType.OPTION_GROUP)
        self.label = Text.to_text(label, max_length=75, force_plaintext=True)
        if options:
            self.options = options
        else:
            raise InvalidUsageError("Field `options` cannot be empty")

    def _resolve(self) -> Dict[str, Any]:
        option_group = {}  # Does not include type in JSON
        option_group["label"] = self.label._resolve()
        option_group["options"] = [option._resolve() for option in self.options]
        return option_group


ALLOWABLE_TRIGGERS = ["on_enter_pressed", "on_character_entered"]


class DispatchActionConfiguration(CompositionObject):
    """
    Determines when a plain-text input element will return a block_actions interaction payload.
    """

    def __init__(self, trigger_actions_on: Union[str, List[str]] = None):
        self.trigger_actions_on = list(
            set(coerce_to_list(trigger_actions_on, str, min_size=1, max_size=2))
        )
        for trigger in self.trigger_actions_on:
            if trigger not in ALLOWABLE_TRIGGERS:
                raise InvalidUsageError(
                    f"Trigger {trigger} not in allowable values ({ALLOWABLE_TRIGGERS})"
                )

    def _resolve(self) -> Dict[str, Any]:
        dispatch_action_config = {}  # Does not include type in JSON
        dispatch_action_config["trigger_actions_on"] = self.trigger_actions_on
        return dispatch_action_config


class ConversationFilter(CompositionObject):
    """
    Provides a way to filter the list of options in a conversations select menu or
    conversations multi-select menu.
    """

    def __init__(
        self,
        include: Optional[Union[str, List[str]]] = None,
        exclude_external_shared_channels: Optional[bool] = None,
        exclude_bot_users: Optional[bool] = None,
    ):
        super().__init__(type_=CompositionObjectType.FILTER)
        if not (
            include
            or exclude_external_shared_channels is not None
            or exclude_bot_users is not None
        ):
            raise InvalidUsageError(
                "One of `include`, `exclude_external_shared_channels`, or "
                "`exclude_bot_users` is required."
            )
        self.include = coerce_to_list(include, str, allow_none=True)
        self.exclude_external_shared_channels = exclude_external_shared_channels
        self.exclude_bot_users = exclude_bot_users

    def _resolve(self) -> Dict[str, Any]:
        filter = {}  # Does not include type in JSON
        if self.include:
            filter["include"] = self.include
        if self.exclude_external_shared_channels is not None:
            filter[
                "exclude_external_shared_channels"
            ] = self.exclude_external_shared_channels
        if self.exclude_bot_users is not None:
            filter["exclude_bot_users"] = self.exclude_bot_users
        return filter


class InputParameter(CompositionObject):
    """
    Contains information about an input parameter.
    """

    def __init__(self, name: str, value: str):
        super().__init__(type_=CompositionObjectType.INPUT_PARAMETER)
        self.name = name
        self.value = value

    def _resolve(self) -> Dict[str, Any]:
        input_parameter = {}  # Does not include type in JSON
        input_parameter["name"] = self.name
        input_parameter["value"] = self.value
        return input_parameter


class Trigger(CompositionObject):
    """
    Contains information about a trigger.
    """

    def __init__(
        self,
        url: str,
        customizable_input_parameters: Optional[
            Union[InputParameter, List[InputParameter]]
        ],
    ):
        super().__init__(type_=CompositionObjectType.TRIGGER)
        self.url = url
        self.customizable_input_parameters = coerce_to_list(
            customizable_input_parameters, InputParameter, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        trigger = {}  # Does not include type in JSON
        trigger["url"] = self.url
        if self.customizable_input_parameters:
            trigger["customizable_input_parameters"] = [
                parameter._resolve() for parameter in self.customizable_input_parameters
            ]
        return trigger


class Workflow(CompositionObject):
    """
    Contains information about a workflow.
    """

    def __init__(self, trigger: Trigger):
        super().__init__(type_=CompositionObjectType.WORKFLOW)
        self.trigger = trigger

    def _resolve(self) -> Dict[str, Any]:
        workflow = {}  # Does not include type in JSON
        workflow["trigger"] = self.trigger._resolve()
        return workflow
