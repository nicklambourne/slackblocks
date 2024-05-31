"""
Composition objects are the lowest-level primitives used inside of Block objects.

See: <https://api.slack.com/reference/block-kit/composition-objects>.
"""

from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

from slackblocks.errors import InvalidUsageError
from slackblocks.utils import coerce_to_list, validate_string


class CompositionObjectType(Enum):
    """
    Convenience class for referencing the types of various message elements Slack
    provides.
    """

    CONFIRM = "confirm"
    DISPATCH = "dispatch"
    FILTER = "conversations_select"
    INPUT_PARAMETER = "input_parameter"
    OPTION = "option"
    OPTION_GROUP = "option_groups"
    SLACK_FILE = "file"
    TEXT = "text"
    TRIGGER = "trigger"
    WORKFLOW = "workflow"


class CompositionObject(ABC):
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

    MARKDOWN: tradional markdown formatting, see
        <https://api.slack.com/reference/surfaces/formatting#basic-formatting>
    PLAINTEXT: simple Unicode text with no formatting (e.g. bold) features.

    N.B: some usages of Text objects only allow the `PLAINTEXT` variety.
    """

    MARKDOWN = "mrkdwn"
    PLAINTEXT = "plain_text"


class Text(CompositionObject):
    """
    An object containing some text, formatted either as `plain_text` or using
    Slack's `mrkdwn`.

    Args:
        text: the text to be rendered in a message (max 3000 characters).
        type_: either `TextType.MARKDOWN` or `TextType.PLAINTEXT`.
        emoji: only usable with `TextType.PLAINTEXT`, if True: emoji will be
            escaped into text format (e.g. `:smile:`).
        verbatim: only usable with `TextType.MARKDOWN`, if True: links, channel
            names, user names will not automatically be rendered as links.

    Throws:
        InvalidUsageException: if the provided `text` fails validation.
    """

    def __init__(
        self,
        text: str,
        type_: TextType = TextType.MARKDOWN,
        emoji: bool = False,
        verbatim: bool = False,
    ) -> "Text":
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
        force_plaintext: bool = False,
        max_length: Optional[int] = None,
        allow_none: bool = False,
    ) -> Optional["Text"]:
        """
        Coerces `str` or `Text` objects into `Text` objects.

        Args:
            text: the `str` or `Text` object to ensure is in `Text` format.
            force_plaintext: if `True`, forces the `str` or `Text` object
                into a `Text` object with the type `TextType.PLAINTEXT`.
            max_length: `text` will be checked against this length in addition
                to the standard `Text` limit of 3000 characters.
            allow_none: whether to accept `None` as a valid value for `text`.
        """
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
        if text and max_length and len(text) > max_length:
            raise InvalidUsageError(
                f"`text` length ({len(text)}) exceeds `max_length` ({max_length})"
            )
        if isinstance(text, str):
            return Text(text=text, type_=type_)
        if isinstance(text, Text):

            return Text(
                text=text.text, type_=type_, emoji=text.emoji, verbatim=text.verbatim
            )
        else:
            raise InvalidUsageError(
                f"Can only coerce Text object from `str` or `Text`, not `{type(text)}`"
            )

    def __str__(self) -> str:
        return dumps(self._resolve())

    def __len__(self) -> int:
        return len(self.text)

    def __eq__(self, other) -> bool:
        return (
            self.text_type == other.text_type
            and self.text == other.text
            and self.emoji == other.emoji
            and self.vertbatim == other.verbatim
        )


# Used for accepting strings and `Text`` where coercion to `Text` is desirable.
TextLike = Union[str, Text]


class ConfirmationDialogue(CompositionObject):
    """
    An object that defines a dialog that provides a confirmation step
    to any interactive element. This dialog will ask the user to confirm
    their action by offering confirm and deny buttons.

    Args:
        title: the text heading presented at the top of the dialogue box (max 100 chars).
        text: the text explaining the decision being made by the user through
            the dialogue box (max 300 chars).
        confirm: the text inside the confirmation button of the dialogue box (max 30 chars).
        deny: the text inside the deny button of the dialogue box (max 30 chars).

    Throws:
        InvalidUsageError: if any of the arguments fail to pass validation checks.
    """

    def __init__(
        self,
        title: TextLike,
        text: TextLike,
        confirm: TextLike,
        deny: TextLike,
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
    Alias for `ConfirmationDialogue` to retain backwards compatibility.

    See:
        [`ConfirmationDialogue`](/slackblocks/latest/reference/objects/#objects.ConfirmationDialogue).  # noqa: E501
    """

    def __init__(self, *args, **kwargs):
        super(*args, **kwargs)


class Option(CompositionObject):
    """
    An object that represents a single selectable item in a select menu, multi-select
    menu, checkbox group, radio button group, or overflow menu.

    See <https://api.slack.com/reference/block-kit/composition-objects#option>.

    Args:
        text: the text identifying the option (that the user will see).
        value: the underlying value of that option (not seen by the user).
        description: a more detailed explanation of what the option means (user-facing).
        url: a URL to load in the user's browser when the option is clicked.
            Only available in `OverflowMenus`.

    Throws:
        InvalidUsageError: when any of the provided arguments fail validation.
    """

    def __init__(
        self,
        text: TextLike,
        value: str,
        description: Optional[TextLike] = None,
        url: Optional[str] = None,
    ) -> "Option":
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

    See <https://api.slack.com/reference/block-kit/composition-objects#option_group>.

    Args:
        label: a label shown above the group of options.
        options: a list of `Option` objects that will form the contents of the group (max 100).

    Throws:
        InvalidUsageError: if no options are provided or the label is not valid.
    """

    def __init__(self, label: TextLike, options: List[Option]):
        super().__init__(type_=CompositionObjectType.OPTION_GROUP)
        self.label = Text.to_text(label, max_length=75, force_plaintext=True)
        self.options = coerce_to_list(
            options,
            class_=Option,
            min_size=1,
            max_size=100,
            allow_none=False,
        )

    def _resolve(self) -> Dict[str, Any]:
        option_group = {}  # Does not include type in JSON
        option_group["label"] = self.label._resolve()
        option_group["options"] = [option._resolve() for option in self.options]
        return option_group


ALLOWABLE_TRIGGERS = ["on_enter_pressed", "on_character_entered"]


class DispatchActionConfiguration(CompositionObject):
    """
    Determines when a plain-text input element will return a `block_action`s interaction payload.

    Args:
        trigger_actions_on: a list of strings representing interaction types that should return
            a `block_actions` payload. One or both of `on_enter_pressed`, `on_character_entered`.

    Throws:
        InvalidUsageError: if an invalid value is provided amongst the options for
            `trigger_actions_on`.
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

    See: <https://api.slack.com/reference/block-kit/composition-objects#filter_conversations>.

    At least one of the available arguments _must_ be provided.

    Args:
        include: Which types of conversations to include in the list.
            One of more of `im`, `mpim`, `private`, `public`.
        exclude_external_shared_channels: whether to remove shared public channels
            from the list. See <https://api.slack.com/enterprise/shared-channels>.
        exclude_bot_users: whether to remove bot users from the list of conversations.

    Throws:
        InvalidUsageException: in the event that the user provides none of `include`,
            `exclude_external_shared_channels`, or `exclude_bot_users` arguments.
    """

    def __init__(
        self,
        include: Optional[Union[str, List[str]]] = None,
        exclude_external_shared_channels: Optional[bool] = None,
        exclude_bot_users: Optional[bool] = None,
    ) -> "ConversationFilter":
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
            filter["exclude_external_shared_channels"] = (
                self.exclude_external_shared_channels
            )
        if self.exclude_bot_users is not None:
            filter["exclude_bot_users"] = self.exclude_bot_users
        return filter


class InputParameter(CompositionObject):
    """
    Contains information about an input parameter.

    See <https://api.slack.com/automation/workflows#defining-input-parameters>.

    Args:
        name: the name of the input parameter.
        value: the value associated with the input parameter.
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


class SlackFile(CompositionObject):
    """
    Defines an object containing Slack file information to be used in an image
        block or image element.

    This file must be an image and you must provide either the URL or ID (not both).

    See: <https://api.slack.com/reference/block-kit/composition-objects#slack_file>.

    Args:
        url: the URL can be the `url_private` or the `permalink` of the Slack file
            (only one of `url` or `id` can be provided).
        id: the Slack ID of the file
            (only one of `url` or `id` can be provided).

    Throws:
        InvalidUsageError: if both `url` and `id` are provided
    """

    def __init__(
        self,
        url: Optional[str],
        id: Optional[str],
    ) -> "SlackFile":
        super().__init__(CompositionObjectType.SLACK_FILE)
        if url and id:
            raise InvalidUsageError("Cannot provide both `url` and `id`.")
        self.url = url
        self.id = id

    def _resolve(self) -> Dict[str, Any]:
        file = {}  # Does not include type in JSON
        if self.url:
            file["url"] = self.url
        if self.id:
            file["id"] = self.id
        return file


class Trigger(CompositionObject):
    """
    Contains information about a trigger.

    See: <https://api.slack.com/automation/triggers>.

    Args:
        url: a link trigger URL, see
            <https://api.slack.com/automation/triggers/link>
        customizable_input_parameters: a list of `InputParameter` objects
            which map to those parameters defined on the Workflow in
            which they are provided.

    Throws:
        InvalidUsageError: when any of the items in
            `customizable_input_parameters` is not a valid `InputParameter`.
    """

    def __init__(
        self,
        url: str,
        customizable_input_parameters: Optional[
            Union[InputParameter, List[InputParameter]]
        ],
    ) -> "Trigger":
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

    See <https://api.slack.com/automation/workflows>.

    Args:
        trigger: a `Trigger` object that will initiate the workflow.
    """

    def __init__(self, trigger: Trigger) -> "Workflow":
        super().__init__(type_=CompositionObjectType.WORKFLOW)
        self.trigger = trigger

    def _resolve(self) -> Dict[str, Any]:
        workflow = {}  # Does not include type in JSON
        workflow["trigger"] = self.trigger._resolve()
        return workflow
