"""
Composition objects are the lowest-level primitives used inside of Block objects.

See: <https://api.slack.com/reference/block-kit/composition-objects>.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Literal, TypeAlias, cast, overload

from slackblocks._core import RenderableMixin, omit_none, resolve
from slackblocks.errors import (
    LengthError,
    MissingRequiredError,
    MutualExclusivityError,
    TypeMismatchError,
)
from slackblocks.utils import (
    coerce_to_list,
    coerce_to_list_nonnull,
    validate_string_nonnull,
)


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


class CompositionObject(RenderableMixin, ABC):
    """
    Basis element containing attributes and behaviour common to all
    composition objects.
    N.B: CompositionObject is an abstract class and should not be
    instantiated directly.
    """

    def __init__(self, type_: CompositionObjectType) -> None:
        super().__init__()
        self.type = type_

    def _attributes(self) -> dict[str, Any]:
        return {"type": self.type.value}

    @abstractmethod
    def _resolve(self) -> dict[str, Any]:
        pass


class TextType(Enum):
    """
    Allowable types for Slack Text objects.

    MARKDOWN: traditional markdown formatting, see
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
    ) -> None:
        super().__init__(type_=CompositionObjectType.TEXT)
        self.text_type = type_
        self.text = validate_string_nonnull(text, field_name="text", min_length=1, max_length=3000)
        if self.text_type == TextType.MARKDOWN:
            self.verbatim = verbatim
            self.emoji = False
        elif self.text_type == TextType.PLAINTEXT:
            self.verbatim = False
            self.emoji = emoji

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                "type": self.text_type.value,
                "text": self.text,
                "verbatim": self.verbatim
                if self.text_type == TextType.MARKDOWN and self.verbatim
                else None,
                "emoji": self.emoji
                if self.text_type == TextType.PLAINTEXT and self.emoji
                else None,
            }
        )

    @overload
    @staticmethod
    def to_text(
        text: str | Text | None,
        force_plaintext: bool = False,
        max_length: int | None = None,
        *,
        allow_none: Literal[False] = False,
    ) -> Text: ...

    @overload
    @staticmethod
    def to_text(
        text: str | Text | None,
        force_plaintext: bool = False,
        max_length: int | None = None,
        *,
        allow_none: Literal[True],
    ) -> Text | None: ...

    @overload
    @staticmethod
    def to_text(
        text: str | Text | None,
        force_plaintext: bool = False,
        max_length: int | None = None,
        *,
        allow_none: bool,
    ) -> Text | None: ...

    @staticmethod
    def to_text(
        text: str | Text | None,
        force_plaintext: bool = False,
        max_length: int | None = None,
        allow_none: bool = False,
    ) -> Text | None:
        """
        Coerces `str` or `Text` objects into `Text` objects.

        Args:
            text: the `str` or `Text` object to ensure is in `Text` format.
            force_plaintext: if `True`, forces the `str` or `Text` object
                into a `Text` object with the type `TextType.PLAINTEXT`.
            max_length: `text` will be checked against this length in addition
                to the standard `Text` limit of 3000 characters.
            allow_none: whether to accept `None` as a valid value for `text`.
                The return type narrows based on this value:

                - ``allow_none=False`` (the default) -> always returns ``Text``.
                - ``allow_none=True`` -> returns ``Text | None``.
        """
        if text is None:
            if allow_none:
                return None
            raise MissingRequiredError("This field cannot have the value None or ''")
        return Text.to_text_nonnull(
            text,
            force_plaintext,
            max_length,
        )

    @staticmethod
    def to_text_nonnull(
        text: str | Text,
        force_plaintext: bool = False,
        max_length: int | None = None,
    ) -> Text:
        """
        Coerces `str` or `Text` objects into `Text` objects, but does not allow `None` values.

        Args:
            text: the `str` or `Text` object to ensure is in `Text` format.
            force_plaintext: if `True`, forces the `str` or `Text` object
                into a `Text` object with the type `TextType.PLAINTEXT`.
            max_length: `text` will be checked against this length in addition
                to the standard `Text` limit of 3000 characters.

        Returns:
            A `Text` object created from the input.

        Throws:
            InvalidUsageError: if the text length exceeds the specified max_length.
        """
        original_type = text.text_type if isinstance(text, Text) else None
        type_ = TextType.PLAINTEXT if force_plaintext else original_type or TextType.MARKDOWN
        if text and max_length and len(text) > max_length:
            raise LengthError(f"`text` length ({len(text)}) exceeds `max_length` ({max_length})")
        if isinstance(text, str):
            return Text(text=text, type_=type_)
        elif isinstance(text, Text):
            return Text(text=text.text, type_=type_, emoji=text.emoji, verbatim=text.verbatim)
        else:
            raise TypeMismatchError("This field must be a string or Text object")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Text:
        """Parse a Slack-shaped ``text`` composition object back into a ``Text``.

        Unknown fields are ignored so that future Slack additions do not
        break round-tripping.

        Args:
            data: a dict matching the Slack ``text`` composition-object shape,
                e.g. ``{"type": "mrkdwn", "text": "hi", "verbatim": True}``.

        Returns:
            A ``Text`` instance.

        Throws:
            MissingRequiredError: if ``data["text"]`` is absent.
            TypeMismatchError: if ``data["type"]`` is not one of the
                allowable ``TextType`` values.
        """
        if "text" not in data:
            raise MissingRequiredError("Text payload is missing required `text` field.")
        type_str = data.get("type", TextType.MARKDOWN.value)
        try:
            text_type = TextType(type_str)
        except ValueError as exc:
            raise TypeMismatchError(
                f"Text `type` must be one of {[t.value for t in TextType]}, got {type_str!r}."
            ) from exc
        return cls(
            text=data["text"],
            type_=text_type,
            emoji=bool(data.get("emoji", False)),
            verbatim=bool(data.get("verbatim", False)),
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
            and self.verbatim == other.verbatim
        )


# Used for accepting strings and `Text`` where coercion to `Text` is desirable.
TextLike: TypeAlias = str | Text


class PlainText(Text):
    """Convenience wrapper for `Text` with `type_=TextType.PLAINTEXT`.

    `PlainText("Hi", emoji=True)` is equivalent to
    `Text("Hi", type_=TextType.PLAINTEXT, emoji=True)`. Anywhere a `Text` or
    `TextLike` is accepted, a `PlainText` works because `PlainText` is a
    subclass of `Text`. The rendered JSON is identical to the equivalent
    `Text` call.

    Args:
        text: the text to render (1-3000 characters).
        emoji: if `True`, emoji (e.g. `:smile:`) are escaped into Unicode.

    Throws:
        LengthError: if the provided `text` is empty or too long.
    """

    def __init__(self, text: str, emoji: bool = False) -> None:
        super().__init__(text=text, type_=TextType.PLAINTEXT, emoji=emoji)


class Markdown(Text):
    """Convenience wrapper for `Text` with `type_=TextType.MARKDOWN`.

    `Markdown("_italic_", verbatim=True)` is equivalent to
    `Text("_italic_", type_=TextType.MARKDOWN, verbatim=True)`. Anywhere a
    `Text` or `TextLike` is accepted, a `Markdown` works because `Markdown`
    is a subclass of `Text`. The rendered JSON is identical to the
    equivalent `Text` call.

    Args:
        text: the markdown-formatted text to render (1-3000 characters).
        verbatim: if `True`, links, channel names, and user names are
            rendered verbatim rather than as Slack-style references.

    Throws:
        LengthError: if the provided `text` is empty or too long.
    """

    def __init__(self, text: str, verbatim: bool = False) -> None:
        super().__init__(text=text, type_=TextType.MARKDOWN, verbatim=verbatim)


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
    ) -> None:
        super().__init__(type_=CompositionObjectType.CONFIRM)
        self.title = Text.to_text_nonnull(title, max_length=100, force_plaintext=True)
        self.text = Text.to_text_nonnull(text, max_length=300)
        self.confirm = Text.to_text_nonnull(confirm, max_length=30, force_plaintext=True)
        self.deny = Text.to_text_nonnull(deny, max_length=30, force_plaintext=True)

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                "title": self.title,
                "text": self.text,
                "confirm": self.confirm,
                "deny": self.deny,
            }
        )

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConfirmationDialogue:
        """Parse a Slack ``confirm`` composition object back into an instance.

        Throws:
            MissingRequiredError: if any of ``title``, ``text``, ``confirm``,
                ``deny`` is absent.
        """
        for field in ("title", "text", "confirm", "deny"):
            if field not in data:
                raise MissingRequiredError(
                    f"Confirmation dialogue payload is missing required `{field}` field."
                )
        return cls(
            title=Text.from_dict(data["title"]),
            text=Text.from_dict(data["text"]),
            confirm=Text.from_dict(data["confirm"]),
            deny=Text.from_dict(data["deny"]),
        )


class Confirm(ConfirmationDialogue):
    """
    Alias for `ConfirmationDialogue` to retain backwards compatibility.

    See:
        [`ConfirmationDialogue`](/slackblocks/latest/reference/objects/#objects.ConfirmationDialogue).
    """

    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)


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
        description: TextLike | None = None,
        url: str | None = None,
    ) -> None:
        super().__init__(type_=CompositionObjectType.OPTION)
        self.text = Text.to_text_nonnull(text, max_length=75)
        self.value = validate_string_nonnull(value, field_name="value", max_length=75)
        self.description = Text.to_text(
            description, max_length=75, force_plaintext=True, allow_none=True
        )
        if url and len(url) > 3000:
            raise LengthError("Option URLs must be less than 3000 characters")
        self.url = url

    def _resolve(self) -> dict[str, Any]:
        # Option does not include "type" in its rendered JSON.
        return resolve(
            {
                "text": self.text,
                "value": self.value,
                "description": self.description,
                "url": self.url,
            }
        )

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Option:
        """Parse a Slack ``option`` composition object back into an ``Option``.

        Throws:
            MissingRequiredError: if ``text`` or ``value`` is absent.
        """
        for field in ("text", "value"):
            if field not in data:
                raise MissingRequiredError(f"Option payload is missing required `{field}` field.")
        description = data.get("description")
        return cls(
            text=Text.from_dict(data["text"]),
            value=data["value"],
            description=Text.from_dict(description) if description is not None else None,
            url=data.get("url"),
        )

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

    def __init__(self, label: TextLike, options: list[Option]) -> None:
        super().__init__(type_=CompositionObjectType.OPTION_GROUP)
        self.label = Text.to_text(label, max_length=75, force_plaintext=True)
        self.options: list[Option] = coerce_to_list_nonnull(
            options,
            class_=Option,
            min_size=1,
            max_size=100,
        )

    def _resolve(self) -> dict[str, Any]:
        # OptionGroup does not include "type" in its rendered JSON.
        return resolve({"label": self.label, "options": self.options})

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> OptionGroup:
        """Parse a Slack ``option_group`` composition object.

        Throws:
            MissingRequiredError: if ``label`` or ``options`` is absent.
        """
        for field in ("label", "options"):
            if field not in data:
                raise MissingRequiredError(
                    f"OptionGroup payload is missing required `{field}` field."
                )
        return cls(
            label=Text.from_dict(data["label"]),
            options=[Option.from_dict(item) for item in data["options"]],
        )


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

    def __init__(self, trigger_actions_on: str | list[str] | None = None) -> None:
        super().__init__(type_=CompositionObjectType.DISPATCH)
        trigger_actions_on = trigger_actions_on or []
        self.trigger_actions_on = list(
            set(coerce_to_list_nonnull(trigger_actions_on, str, min_size=1, max_size=2))
        )
        for trigger in self.trigger_actions_on:
            if trigger not in ALLOWABLE_TRIGGERS:
                raise TypeMismatchError(
                    f"Trigger {trigger} not in allowable values ({ALLOWABLE_TRIGGERS})"
                )

    def _resolve(self) -> dict[str, Any]:
        # DispatchActionConfiguration does not include "type" in its rendered JSON.
        return {"trigger_actions_on": self.trigger_actions_on}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> DispatchActionConfiguration:
        """Parse a Slack ``dispatch_action_config`` composition object."""
        return cls(trigger_actions_on=data.get("trigger_actions_on"))


ConversationType: TypeAlias = Literal["im", "mpim", "private", "public"]
"""The four kinds of Slack conversation that ``ConversationFilter.include`` may
contain. See <https://api.slack.com/reference/block-kit/composition-objects#filter_conversations>."""


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
        include: ConversationType | list[ConversationType] | None = None,
        exclude_external_shared_channels: bool | None = None,
        exclude_bot_users: bool | None = None,
    ) -> None:
        super().__init__(type_=CompositionObjectType.FILTER)
        if not (
            include or exclude_external_shared_channels is not None or exclude_bot_users is not None
        ):
            raise MissingRequiredError(
                "One of `include`, `exclude_external_shared_channels`, or "
                "`exclude_bot_users` is required."
            )
        self.include = coerce_to_list(cast("str | list[str] | None", include), str, allow_none=True)
        self.exclude_external_shared_channels = exclude_external_shared_channels
        self.exclude_bot_users = exclude_bot_users

    def _resolve(self) -> dict[str, Any]:
        # ConversationFilter does not include "type" in its rendered JSON.
        # Note that ``include`` is omitted when empty (falsy list) to match the
        # historical behaviour; ``exclude_*`` fields are kept when explicitly
        # set to False.
        return omit_none(
            {
                "include": self.include if self.include else None,
                "exclude_external_shared_channels": self.exclude_external_shared_channels,
                "exclude_bot_users": self.exclude_bot_users,
            }
        )

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConversationFilter:
        """Parse a Slack ``filter`` composition object.

        At least one of ``include``, ``exclude_external_shared_channels``,
        or ``exclude_bot_users`` must be present; otherwise the underlying
        constructor raises ``MissingRequiredError``.
        """
        return cls(
            include=data.get("include"),
            exclude_external_shared_channels=data.get("exclude_external_shared_channels"),
            exclude_bot_users=data.get("exclude_bot_users"),
        )


class InputParameter(CompositionObject):
    """
    Contains information about an input parameter.

    See <https://api.slack.com/automation/workflows#defining-input-parameters>.

    Args:
        name: the name of the input parameter.
        value: the value associated with the input parameter.
    """

    def __init__(self, name: str, value: str) -> None:
        super().__init__(type_=CompositionObjectType.INPUT_PARAMETER)
        self.name = name
        self.value = value

    def _resolve(self) -> dict[str, Any]:
        # InputParameter does not include "type" in its rendered JSON.
        return {"name": self.name, "value": self.value}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> InputParameter:
        """Parse a Slack ``input_parameter`` composition object."""
        for field in ("name", "value"):
            if field not in data:
                raise MissingRequiredError(
                    f"InputParameter payload is missing required `{field}` field."
                )
        return cls(name=data["name"], value=data["value"])


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
        url: str | None,
        id: str | None,
    ) -> None:
        super().__init__(CompositionObjectType.SLACK_FILE)
        if url and id:
            raise MutualExclusivityError("Cannot provide both `url` and `id`.")
        self.url = url
        self.id = id

    def _resolve(self) -> dict[str, Any]:
        # SlackFile does not include "type" in its rendered JSON.
        return omit_none({"url": self.url, "id": self.id})

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SlackFile:
        """Parse a Slack ``slack_file`` composition object."""
        return cls(url=data.get("url"), id=data.get("id"))


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
        customizable_input_parameters: InputParameter | list[InputParameter] | None,
    ) -> None:
        super().__init__(type_=CompositionObjectType.TRIGGER)
        self.url = url
        self.customizable_input_parameters = coerce_to_list(
            customizable_input_parameters, InputParameter, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        # Trigger does not include "type" in its rendered JSON.
        return resolve(
            {
                "url": self.url,
                "customizable_input_parameters": self.customizable_input_parameters
                if self.customizable_input_parameters
                else None,
            }
        )

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Trigger:
        """Parse a Slack ``trigger`` composition object."""
        if "url" not in data:
            raise MissingRequiredError("Trigger payload is missing required `url` field.")
        raw_params = data.get("customizable_input_parameters")
        params: list[InputParameter] | None
        if raw_params is None:
            params = None
        else:
            params = [InputParameter.from_dict(p) for p in raw_params]
        return cls(url=data["url"], customizable_input_parameters=params)


class Workflow(CompositionObject):
    """
    Contains information about a workflow.

    See <https://api.slack.com/automation/workflows>.

    Args:
        trigger: a `Trigger` object that will initiate the workflow.
    """

    def __init__(self, trigger: Trigger) -> None:
        super().__init__(type_=CompositionObjectType.WORKFLOW)
        self.trigger = trigger

    def _resolve(self) -> dict[str, Any]:
        # Workflow does not include "type" in its rendered JSON.
        return resolve({"trigger": self.trigger})

    @classmethod
    def from_url(cls, url: str, **input_parameters: str) -> Workflow:
        """Build a `Workflow` from a trigger URL and input parameters in one call.

        ``Workflow.from_url(url, a='1', b='2')`` is equivalent to::

            Workflow(
                trigger=Trigger(
                    url=url,
                    customizable_input_parameters=[
                        InputParameter(name='a', value='1'),
                        InputParameter(name='b', value='2'),
                    ],
                ),
            )

        When no input parameters are supplied, the resulting trigger has
        ``customizable_input_parameters=None`` (the key is omitted from JSON).

        Args:
            url: the link trigger URL.
            **input_parameters: zero or more ``name=value`` pairs that become
                ``InputParameter`` entries on the trigger.

        Returns:
            A new ``Workflow`` instance.
        """
        params: list[InputParameter] | None
        if input_parameters:
            params = [
                InputParameter(name=name, value=value) for name, value in input_parameters.items()
            ]
        else:
            params = None
        return cls(trigger=Trigger(url=url, customizable_input_parameters=params))

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Workflow:
        """Parse a Slack ``workflow`` composition object."""
        if "trigger" not in data:
            raise MissingRequiredError("Workflow payload is missing required `trigger` field.")
        return cls(trigger=Trigger.from_dict(data["trigger"]))


class RawText:
    """
    An object containing some text, formatted as `raw_text` for use in
    `Table` blocks.

    Args:
        text: the text to be rendered in a message.
        emoji: only usable with `TextType.PLAINTEXT`, if True: emoji will be
            escaped into text format (e.g. `:smile:`).

    Throws:
        InvalidUsageException: if the provided `text` fails validation.
    """

    def __init__(
        self,
        text: str,
        emoji: bool = False,
    ) -> None:
        self.type = "raw_text"
        self.text = text
        self.emoji = emoji

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                "type": self.type,
                "text": self.text,
                "emoji": self.emoji if self.emoji else None,
            }
        )


ColumnAlignment: TypeAlias = Literal["left", "center", "right"]
"""Allowable values for ``ColumnSettings.align``."""


class ColumnSettings:
    """
    An object that defines the settings for a column in a `Table` block.

    Args:
        align: the alignment of the column, one of `left`, `center`, or `right`.
        is_wrapped: whether the text in the column should be wrapped.
    """

    def __init__(
        self,
        align: ColumnAlignment | None = None,
        is_wrapped: bool | None = None,
    ) -> None:
        if align and align not in ["left", "center", "right"]:
            raise TypeMismatchError("`align` must be one of `left`, `center`, or `right`")
        self.align = align
        self.is_wrapped = is_wrapped

    def _resolve(self) -> dict[str, Any]:
        return omit_none({"align": self.align, "is_wrapped": self.is_wrapped})
