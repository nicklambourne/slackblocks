"""
Block elements can be used inside of section, context, input, and actions layout blocks.

See: <https://api.slack.com/reference/block-kit/block-elements>
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from datetime import datetime
from enum import Enum
from itertools import chain
from json import dumps
from typing import TYPE_CHECKING, Any

from ._core import resolve
from .errors import InvalidUsageError
from .objects import (
    ConfirmationDialogue,
    ConversationFilter,
    DispatchActionConfiguration,
    Option,
    OptionGroup,
    SlackFile,
    Text,
    TextLike,
    TextType,
    Workflow,
)
from .utils import coerce_to_list, validate_action_id, validate_int, validate_string

if TYPE_CHECKING:
    from .rich_text import RichText


class ElementType(Enum):
    """
    Convenience class for referencing the various message elements Slack
    provides.
    """

    BUTTON = "button"
    CHECKBOXES = "checkboxes"
    DATE_PICKER = "datepicker"
    DATETIME_PICKER = "datetimepicker"
    EMAIL_INPUT = "email_text_input"
    FILE_INPUT = "file_input"
    IMAGE = "image"
    MULTI_SELECT_STATIC = "multi_static_select"
    MULTI_SELECT_EXTERNAL = "multi_external_select"
    MULTI_SELECT_USERS = "multi_users_select"
    MULTI_SELECT_CONVERSATIONS = "multi_conversations_select"
    MULTI_SELECT_CHANNELS = "multi_channels_select"
    NUMBER_INPUT = "number_input"
    OVERFLOW_MENU = "overflow"
    PLAIN_TEXT_INPUT = "plain_text_input"
    RADIO_BUTTON_GROUP = "radio_buttons"
    STATIC_SELECT_MENU = "static_select"
    EXTERNAL_SELECT_MENU = "external_select"
    USERS_SELECT_MENU = "users_select"
    CONVERSATIONS_SELECT_MENU = "conversations_select"
    CHANNELS_SELECT_MENU = "channels_select"
    TIME_PICKER = "timepicker"
    URL_INPUT = "url_text_input"
    WORKFLOW_BUTTON = "workflow_button"
    RICH_TEXT_INPUT = "rich_text_input"


class Element(ABC):
    """
    Basis element containing attributes and behaviour common to all elements.
    N.B: Element is an abstract class and cannot be used directly.
    """

    def __init__(self, type_: ElementType) -> None:
        super().__init__()
        self._type = type_

    def _attributes(self) -> dict[str, Any]:
        return {"type": self._type.value}

    @abstractmethod
    def _resolve(self) -> dict[str, Any]:
        pass

    @property
    def type(self) -> ElementType:
        return self._type

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class Button(Element):
    """
    An interactive element that inserts a button. The button can be a
    trigger for anything from opening a simple link to starting a complex
    workflow.

    See: <https://api.slack.com/reference/block-kit/block-elements#button>.

    Args:
        text: text on the button (plaintext only; max 75 chars).
        action_id: an identifier so the source of the action can be known.
        url: a URL to load in the user's browser when the button is clicked.
        value: the value sent with the interaction payload.
        style: the visual style of the button, one of `primary`, `danger`.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the button is clicked.
        accessibility_label: a string label for longer descriptive text about
            a button element. Used by screen readers (max 75 chars).
    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        text: TextLike,
        action_id: str,
        url: str | None = None,
        value: str | None = None,
        style: str | None = None,
        confirm: ConfirmationDialogue | None = None,
        accessibility_label: str | None = None,
    ) -> None:
        super().__init__(type_=ElementType.BUTTON)
        self.text = Text.to_text(text, max_length=75, force_plaintext=True)
        self.action_id = validate_action_id(action_id)
        self.url = validate_string(url, field_name="url", max_length=3000, allow_none=True)
        self.value = validate_string(
            value,
            field_name="value",
            max_length=2000,
            allow_none=True,
        )
        self.style: str | None = None
        if isinstance(style, ButtonStyle):
            self.style = style.value
        elif isinstance(style, str):
            self.style = style
        else:
            self.style = None
        self.confirm = confirm
        self.accessibility_label = validate_string(
            accessibility_label,
            "accessibility_label",
            max_length=75,
            allow_none=True,
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "text": self.text,
                "action_id": self.action_id,
                "style": self.style,
                "url": self.url,
                "value": self.value,
                "confirm": self.confirm,
                "accessibility_label": self.accessibility_label,
            }
        )


class CheckboxGroup(Element):
    """
    A checkbox group that allows a user to choose multiple items from a list
    of possible options.

    See: <https://api.slack.com/reference/block-kit/block-elements#checkboxes>.

    Args:
        action_id: an identifier so the source of the action can be known.
        options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will form
            the content of the checkbox group.
        initial_options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will be
            initially selected when first presented to the user.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the checkbox group is used.
        focus_on_load: whether or not the checkbox group will be set to autofocus
            within the view object.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        options: Option | list[Option],
        initial_options: Option | list[Option] | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
    ) -> None:
        super().__init__(type_=ElementType.CHECKBOXES)
        self.action_id = validate_action_id(action_id)
        self.options = coerce_to_list(options, Option)
        self.initial_options = coerce_to_list(initial_options, Option, allow_none=True)
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "options": self.options,
                "initial_options": self.initial_options if self.initial_options else None,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
            }
        )


class DatePicker(Element):
    """
    Interactive element that allows users to select a date from a calendar.

    See: <https://api.slack.com/reference/block-kit/block-elements#datepicker>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_date: the date (in `YYYY-MM-DD` format) that will appear on the
            picker when it first renders.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the date picker is clicked.
        focus_on_load: whether or not the date picker will be set to autofocus
            within the view object.
        placeholder: a `TextType.PLAINTEXT` `Text` object that defines what text
            will initially appear on the picker.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_date: str | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.DATE_PICKER)
        self.action_id = validate_action_id(action_id)
        self.initial_date: str | None = None
        if initial_date:
            self.initial_date = datetime.strptime(initial_date, "%Y-%m-%d").strftime("%Y-%m-%d")
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_date": self.initial_date,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class DateTimePicker(Element):
    """
    Allows users to select both a date and a time of day.

    Provides the date-time formatted as a Unix timestamp.

    See: <https://api.slack.com/reference/block-kit/block-elements#datetimepicker>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_datetime: the initial value the date-time picker will be set to
            when it first renders.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the button is date-time picker is used.
        focus_on_load: whether or not the datetime picker will be set to autofocus
            within the view object.
    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_datetime: int | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
    ) -> None:
        super().__init__(type_=ElementType.DATETIME_PICKER)
        self.action_id = validate_action_id(action_id)
        self.initial_datetime = initial_datetime
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_date_time": self.initial_datetime if self.initial_datetime else None,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
            }
        )


class EmailInput(Element):
    """
    Allows user to enter an email into a single-line text field.

    See: <https://api.slack.com/reference/block-kit/block-elements#email>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_value: The initial value in the email input when it is loaded.
        dispatch_action_config: a `DispatchActionConfiguration` object that
            determines when during text input the element returns a
            `block_actions` payload.
        focus_on_load: whether or not the email input will be set to autofocus
            within the view object.
        placeholder: a `TextType.PLAINTEXT` `Text` object that defines what text
            will initially appear in the input field.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_value: str | None = None,
        dispatch_action_config: DispatchActionConfiguration | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.EMAIL_INPUT)
        self.action_id = validate_action_id(action_id)
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_value": self.initial_value if self.initial_value else None,
                "dispatch_action_config": self.dispatch_action_config,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class FileInput(Element):
    """
    An interactive element that allows users to upload files.

    See: <https://api.slack.com/reference/block-kit/block-elements#file_input>.

    Args:
        action_id: an identifier so the source of the action can be known.
        filetypes: a list of file extensions (as strings) that will be accepted
            for upload.
        max_files: the maximum number of files that can be uploaded (between 1
            and 10).

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str | None = None,
        filetypes: str | list[str] | None = None,
        max_files: int | None = None,
    ) -> None:
        super().__init__(ElementType.FILE_INPUT)
        self.action_id = validate_action_id(action_id)
        self.filetypes = coerce_to_list(
            filetypes,
            (str),
            allow_none=True,
        )
        self.max_files = validate_int(max_files, min_value=1, max_value=10, allow_none=True)

    def _resolve(self) -> dict[str, Any]:
        # FileInput currently does not emit the "type" attribute; this is
        # preserved from prior behaviour. The pre-existing #154 export work
        # surfaced this class but did not change its rendering contract.
        return resolve(
            {
                "action_id": self.action_id,
                "filetypes": self.filetypes,
                "max_files": self.max_files,
            }
        )


class Image(Element):
    """
    An element to insert an image - this element can be used in section
    and context blocks only. If you want a block with only an image in it,
    you're looking for the Image block.

    You must provide either one of `image_url` or `slack_file`

    See: <https://api.slack.com/reference/block-kit/block-elements#image>.

    Args:
        alt_text: a plain-text-only summary of the content of the image.
        image_url: a URL for a publicly hosted image (the user must provide
            either `image_url` or `slack_file`).
        slack_file: a [`SlackFile`](/slackblocks/latest/reference/objects/#objects.SlackFile)
            (the user must provide either `image_url` or `slack_file`).

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation,
            or both/neither of `image_url` and `slack_file` are provided.
    """

    def __init__(
        self,
        alt_text: str = " ",
        image_url: str | None = None,
        slack_file: SlackFile | None = None,
    ) -> None:
        super().__init__(type_=ElementType.IMAGE)
        if image_url is None and slack_file is None:
            raise InvalidUsageError("Must provide one of `image_url` or `slack_file`")
        if image_url and slack_file:
            raise InvalidUsageError("Cannot provide both `image_url` or `slack_file`")
        self.image_url = image_url
        self.alt_text = alt_text
        self.slack_file = slack_file

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "image_url": self.image_url,
                "alt_text": self.alt_text,
                "slack_file": self.slack_file,
            }
        )


class StaticMultiSelectMenu(Element):
    """
    The most basic form of select menu containing a static list of options
    passed in when defining the element.

    See: <https://api.slack.com/reference/block-kit/block-elements#static_multi_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        options: a list of [`Options`](/slackblocks/latest/reference/objects/#objects.Option)
            (max 100). Only one of `options` or `option_groups` must be
            provided.
        option_groups: a list of
            [`OptionGroups`](/slackblocks/latest/reference/objects/#objects.OptionGroup)
            (max 100). Only one of `options` or `option_groups` can be
            provided.
        initial_options: the [`Options`](/slackblocks/latest/reference/objects/#objects.Option)
            to be intially selected when the element is first rendered.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the menu is used.
        max_selected_items: the
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        options: Option | list[Option],
        option_groups: OptionGroup | list[OptionGroup] | None = None,
        initial_options: Option | list[Option] | OptionGroup | list[OptionGroup] | None = None,
        confirm: ConfirmationDialogue | None = None,
        max_selected_items: int | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.MULTI_SELECT_STATIC)
        self.action_id = validate_action_id(action_id)
        if options and option_groups:
            raise InvalidUsageError("Cannot set both `options` and `option_groups` parameters.")
        self.options = coerce_to_list(options, class_=Option, allow_none=True, max_size=100)
        self.option_groups = coerce_to_list(
            option_groups, class_=OptionGroup, allow_none=True, max_size=100
        )
        self.initial_options = coerce_to_list(
            initial_options,  # type: ignore
            class_=(Option, OptionGroup),
            allow_none=True,
            max_size=100,
        )
        if (
            options
            and self.initial_options
            and not all(isinstance(option, Option) for option in self.initial_options)
        ):
            raise InvalidUsageError(
                "If using `options` then `initial_options` must also be of type `List[Option]`, "
                f"not `{type(self.initial_options)}`."
            )
        if (
            option_groups
            and self.initial_options
            and not all(isinstance(option, OptionGroup) for option in self.initial_options)
        ):
            raise InvalidUsageError(
                "If using `option_groups` then `initial_options` must also be of type "
                f"`List[OptionGroup]`, not `{type(self.initial_options)}`."
            )

        # Check that Option Text is all TextType.PLAINTEXT
        options_to_validate: list[Option] = []
        if self.options:
            options_to_validate = self.options
        if self.option_groups:
            options_to_validate = list(
                chain.from_iterable(option_group.options for option_group in self.option_groups)
            )
        for option in options_to_validate:
            if option.text.text_type == TextType.MARKDOWN:
                raise InvalidUsageError(
                    "Text in Options for StaticSelectMenu can only be of TextType.PLAINTEXT"
                )

        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "options": self.options if self.options else None,
                "option_groups": self.option_groups if self.option_groups else None,
                "initial_options": self.initial_options if self.initial_options else None,
                "confirm": self.confirm,
                "max_selected_items": self.max_selected_items,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ExternalMultiSelectMenu(Element):
    """
    An interactive UI element that loads its options from an external data source,
        allowing for a dynamic list of options.

    See: <https://api.slack.com/reference/block-kit/block-elements#external_multi_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        min_query_length: minimum number of characters entered before the query
            is dispactched (defaults to 3 if not provided).
        initial_options: the [`Options`](/slackblocks/latest/reference/objects/#objects.Option)
            to be intially selected when the element is first rendered.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the menu is used.
        max_selected_items: the highest number of items from the list that
            can be selected at one time.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        min_query_length: int | None = None,
        initial_options: Option | list[Option] | OptionGroup | list[OptionGroup] | None = None,
        confirm: ConfirmationDialogue | None = None,
        max_selected_items: int | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.MULTI_SELECT_EXTERNAL)
        self.action_id = validate_action_id(action_id)
        self.min_query_length = min_query_length
        self.initial_options = coerce_to_list(
            initial_options,  # type: ignore
            class_=(Option, OptionGroup),
            allow_none=True,
            max_size=100,
        )
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "min_query_length": self.min_query_length,
                "initial_options": self.initial_options if self.initial_options else None,
                "confirm": self.confirm,
                "max_selected_items": self.max_selected_items,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class UserMultiSelectMenu(Element):
    """
    This interactive UI element allows users to select multiple users visible
        to the current user in the active workspace.

    See: <https://api.slack.com/reference/block-kit/block-elements#users_multi_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_users: a list of string user IDs to be intially selected
            when the element is first rendered.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the menu is used.
        max_selected_items: the highest number of items from the list that
            can be selected at one time.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_users: list[str] | None = None,
        confirm: ConfirmationDialogue | None = None,
        max_selected_items: int | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.MULTI_SELECT_USERS)
        self.action_id = validate_action_id(action_id)
        self.initial_users: list[str] | None = coerce_to_list(initial_users, str, allow_none=True)
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_users": self.initial_users if self.initial_users else None,
                "confirm": self.confirm,
                "max_selected_items": self.max_selected_items,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ConversationMultiSelectMenu(Element):
    """
    This interactive UI element allows users to select multiple conversations visible
        to the current user in the active workspace.

    See: <https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_conversations: a list of conversation IDs as strings that will
            already be selected when the menu renders.
        default_to_current_conversation: Pre-populates the select menu with the
            conversation that the user was viewing when they opened the modal
            (defaults to `False`).
        confirm: a `ConfirmationDialogue` object that will be presented when
            the menu is used.
        max_selected_items: the maximum number of items that can be selected
            in the menu.
        filter: a [`Filter`](/slackblocks/latest/reference/objects/#objects.ConversationFilter)
            object that filters out conversations that don't match the settings
            of the filter.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_conversations: list[str] | None = None,
        default_to_current_conversation: bool | None = False,
        confirm: ConfirmationDialogue | None = None,
        max_selected_items: int | None = None,
        filter: ConversationFilter | None = None,
        focus_on_load: bool | None = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.MULTI_SELECT_CONVERSATIONS)
        self.action_id = validate_action_id(action_id)
        self.initial_conversations: list[str] | None = coerce_to_list(
            initial_conversations, str, allow_none=True
        )
        self.default_to_current_conversation = default_to_current_conversation
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.filter = filter
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_conversations": self.initial_conversations
                if self.initial_conversations
                else None,
                "default_to_current_conversation": self.default_to_current_conversation
                if self.default_to_current_conversation
                else None,
                "confirm": self.confirm,
                "max_selected_items": self.max_selected_items,
                "filter": self.filter,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ChannelMultiSelectMenu(Element):
    """
    This interactive UI element allows users to select multiple channels visible
        to the current user in the active workspace.

    See: <https://api.slack.com/reference/block-kit/block-elements#channel_multi_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_channels: a list of conversation IDs as strings that will
            already be selected when the menu renders.
        confirm: a `ConfirmationDialogue` object that will be presented when
            the menu is used.
        max_selected_items: the maximum number of items that can be selected
            in the menu.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_channels: list[str] | None = None,
        confirm: ConfirmationDialogue | None = None,
        max_selected_items: int | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.MULTI_SELECT_CHANNELS)
        self.action_id = validate_action_id(action_id)
        self.initial_channels: list[str] | None = coerce_to_list(
            initial_channels, class_=str, allow_none=True
        )
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_channels": self.initial_channels if self.initial_channels else None,
                "confirm": self.confirm,
                "max_selected_items": self.max_selected_items,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class NumberInput(Element):
    """
    This input elements accepts both integer and decimal numbers. For example,
    0.25, 5.5, and -10 are all valid input values.

    See <https://api.slack.com/reference/block-kit/block-elements#number>.

    Args:
        is_decimal_allowed: whether to accept decimal values as input.
        action_id: an identifier so the source of the action can be known.
        initial_value: the initial value in the number input when it is loaded.
        min_value: minimum accepted value for the input field.
        max_value: maximum accepted value for the input field.
        dispatch_action_config: a `DispatchActionConfiguration` object that
            determines when during text input the element returns a
            `block_actions` payload.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        is_decimal_allowed: bool,
        action_id: str | None = None,
        initial_value: str | None = None,
        min_value: float | int | None = None,
        max_value: float | int | None = None,
        dispatch_action_config: DispatchActionConfiguration | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.NUMBER_INPUT)
        self.is_decimal_allowed = is_decimal_allowed
        self.action_id = validate_action_id(action_id, allow_none=True)
        self.initial_value = initial_value
        self.min_value = min_value
        self.max_value = max_value
        if min_value and not is_decimal_allowed and isinstance(min_value, float):
            raise InvalidUsageError(
                f"`min_value` ({min_value}) cannot be a float when `is_decimal_allowed` is `False`"
            )
        if max_value and not is_decimal_allowed and isinstance(max_value, float):
            raise InvalidUsageError(
                f"`max_value` ({max_value}) cannot be a float when `is_decimal_allowed` is `False`"
            )
        if min_value is not None and max_value is not None and min_value > max_value:
            raise InvalidUsageError(
                f"`min_value` ({min_value}) cannot be greater than `max_value` ({max_value})"
            )
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "is_decimal_allowed": self.is_decimal_allowed,
                "action_id": self.action_id if self.action_id else None,
                "initial_value": self.initial_value if self.initial_value else None,
                "min_value": self.min_value,
                "max_value": self.max_value,
                "dispatch_action_config": self.dispatch_action_config,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class OverflowMenu(Element):
    """
    Context menu for additional options (think '...').

    See <https://api.slack.com/reference/block-kit/block-elements#overflow>.

    Args:
        action_id: an identifier so the source of the action can be known.
        options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will form
            the content of the overflow menu.
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        options: Option | list[Option],
        confirm: ConfirmationDialogue | None = None,
    ) -> None:
        super().__init__(type_=ElementType.OVERFLOW_MENU)
        self.action_id = validate_action_id(action_id)
        self.options = coerce_to_list(options, Option, min_size=1, max_size=5)
        self.confirm = confirm

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "options": [option for option in self.options if option is not None]
                if self.options is not None
                else None,
                "confirm": self.confirm,
            }
        )


class PlainTextInput(Element):
    """
    A plain-text input, similar to the HTML <input> tag, creates a field where a user
    can enter freeform data. It can appear as a single-line field or a larger text
    area using the multiline flag.

    See: <https://api.slack.com/reference/block-kit/block-elements#input>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_value: the initial value in the plain-text input when it is loaded.
        multiline: whether to accept multiple lines of input(defaults to false).
        min_length: minimum number of characters to accept as input.
        max_length: maximum number of characters to accept as input.
        dispatch_action_config: a `DispatchActionConfiguration` object that
            determines when during text input the element returns a
            `block_actions` payload.
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_value: str | None = None,
        multiline: bool = False,
        min_length: int | None = None,
        max_length: int | None = None,
        dispatch_action_config: DispatchActionConfiguration | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.PLAIN_TEXT_INPUT)
        self.action_id = validate_action_id(action_id)
        self.multiline = multiline
        self.initial_value = initial_value
        self.min_length = min_length
        if max_length and max_length > 3000:
            raise InvalidUsageError("`max_length` value cannot exceed 3000 characters")
        self.max_length = max_length
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "multiline": self.multiline if self.multiline else None,
                "action_id": self.action_id if self.action_id else None,
                "initial_value": self.initial_value if self.initial_value else None,
                "min_length": self.min_length if self.min_length else None,
                "max_length": self.max_length if self.max_length else None,
                "dispatch_action_config": self.dispatch_action_config,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class RadioButtonGroup(Element):
    """
    A radio button group that allows a user to choose one item from a list of possible options.

    See: <https://api.slack.com/reference/block-kit/block-elements#radio>.

    Args:
        action_id: an identifier so the source of the action can be known.
        options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will form
            the content of the radio button group.
        initial_option: an
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) object that will be
            initially selected when first presented to the user.
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        options: list[Option],
        initial_option: Option | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
    ) -> None:
        super().__init__(type_=ElementType.RADIO_BUTTON_GROUP)
        self.action_id = validate_action_id(action_id)
        if len(options) < 1 or len(options) > 10:
            raise InvalidUsageError(
                "Number of options to RadioButtonGroup must be between 1 and 10 (inclusive)."
            )
        self.options: list[Option] | None = coerce_to_list(options, class_=Option, allow_none=False)
        if initial_option is not None and initial_option not in options:
            raise InvalidUsageError("`initial_option` must be a member of `options`")
        self.initial_option = initial_option
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "options": self.options,
                "initial_option": self.initial_option,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
            }
        )


class StaticSelectMenu(Element):
    """
    A simple select menu interactive UI element, with a static list of options passed in when
        defining the element.

    See: <https://api.slack.com/reference/block-kit/block-elements#static_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will form
            the content of the menu (max 100).
        option_groups: a list of
            [`OptionGroups`](/slackblocks/latest/reference/objects/#objects.OptionGroup)
            (max 100). Only one of `options` or `option_groups` can be
            provided.
        initial_option: an
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) object that will be
            initially selected when first presented to the user.
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        options: list[Option] | None = None,
        option_groups: list[OptionGroup] | None = None,
        initial_option: Option | OptionGroup | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.STATIC_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        if options and option_groups:
            raise InvalidUsageError("Cannot set both `options` and `option_groups` parameters.")
        self.options: list[Option] | None = coerce_to_list(
            options, class_=Option, allow_none=True, max_size=100
        )
        self.option_groups: list[OptionGroup] | None = coerce_to_list(
            option_groups, class_=OptionGroup, allow_none=True, max_size=100
        )
        if options and initial_option and not isinstance(initial_option, Option):
            raise InvalidUsageError(
                "If using `options` then `initial_option` must also be of type `Option`, "
                f"not `{type(initial_option)}`."
            )
        if option_groups and initial_option and not isinstance(initial_option, OptionGroup):
            raise InvalidUsageError(
                "If using `option_groups` then `initial_option` must also be of type "
                f"`OptionGroup`, not `{type(initial_option)}`."
            )

        # Check that Option Text is all TextType.PLAINTEXT
        options_to_validate: list[Option] = []
        if self.options:
            options_to_validate = self.options
        if self.option_groups:
            options_to_validate = list(
                chain.from_iterable(option_group.options for option_group in self.option_groups)
            )
        for option in options_to_validate:
            if option.text.text_type == TextType.MARKDOWN:
                raise InvalidUsageError(
                    "Text in Options for StaticSelectMenu can only be of TextType.PLAINTEXT"
                )

        self.initial_option = initial_option
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "options": self.options if self.options else None,
                "option_groups": self.option_groups if self.option_groups else None,
                "initial_option": self.initial_option,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ExternalSelectMenu(Element):
    """
    A select menu interactive UI element, sourced with externally provided options.

    See:
        <https://api.slack.com/slackblocks/latest/reference/block-kit/block-elements#external_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_option: an
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) object that will be
            initially selected when first presented to the user.
        min_query_length: minimum number of characters entered before the query
            is dispactched (defaults to 3 if not provided).
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_option: Option | OptionGroup | None = None,
        min_query_length: int | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.EXTERNAL_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        self.initial_option = initial_option
        self.min_query_length = min_query_length
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder,
            max_length=150,
            force_plaintext=True,
            allow_none=True,
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_option": self.initial_option,
                "min_query_length": self.min_query_length,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class UserSelectMenu(Element):
    """
    A select menu interactive UI element, sourced automatically with Slack users from the
        current workspace visible to the current user.

    See: <https://api.slack.com/reference/block-kit/block-elements#users_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_user: the single (string) user ID that will be initially selected
            when first presented to the user.
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_user: str | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.USERS_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        self.initial_user = initial_user
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_user": self.initial_user if self.initial_user else None,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ConversationSelectMenu(Element):
    """
    A select menu interactive UI element, sourced with a list of public and private channels,
        DMs, and MPIMs visible to the current user.

    See: <https://api.slack.com/reference/block-kit/block-elements#conversations_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_conversation: the single (string) coversation ID that will be initially
            selected when first presented to the user.
        default_to_current_conversation: Pre-populates the select menu with the
            conversation that the user was viewing when they opened the modal
            (defaults to `False`).
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.
        response_url_enabled: When set to true, the view_submission payload from the
            menu's parent view will contain a response_url. (This response_url can be
            used for message responses).
        filter: a [`Filter`](/slackblocks/latest/reference/objects/#objects.ConversationFilter)
            object that filters out conversations that don't match the settings
            of the filter.
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_conversation: str | None = None,
        default_to_current_conversation: bool | None = False,
        confirm: ConfirmationDialogue | None = None,
        response_url_enabled: bool | None = False,
        filter: ConversationFilter | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.CONVERSATIONS_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        self.initial_conversation = initial_conversation
        self.default_to_current_conversation = default_to_current_conversation
        self.confirm = confirm
        self.response_url_enabled = response_url_enabled
        self.filter = filter
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder,
            max_length=150,
            force_plaintext=True,
            allow_none=True,
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_conversation": self.initial_conversation
                if self.initial_conversation
                else None,
                "default_to_current_conversation": self.default_to_current_conversation
                if self.default_to_current_conversation
                else None,
                "confirm": self.confirm,
                "response_url_enabled": self.response_url_enabled
                if self.response_url_enabled
                else None,
                "filter": self.filter,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ChannelSelectMenu(Element):
    """
    A select menu interactive UI element, sourced with a list of public channels visible
        to the current user.

    See: <https://api.slack.com/reference/block-kit/block-elements#channels_select>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_channel: the single (string) user ID that will be initially selected
            when first presented to the user.
        confirm: a `ConfirmationDialogue` object that will be presented when an
            option in the overflow menu is selected.
        response_url_enabled: When set to true, the view_submission payload from the
            menu's parent view will contain a response_url. (This response_url can be
            used for message responses).
        focus_on_load: whether or not the input will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_channel: str | None = None,
        confirm: ConfirmationDialogue | None = None,
        response_url_enabled: bool | None = False,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.CHANNELS_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        self.initial_channel = initial_channel
        self.confirm = confirm
        self.response_url_enabled = response_url_enabled
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder,
            max_length=150,
            force_plaintext=True,
            allow_none=True,
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_channel": self.initial_channel if self.initial_channel else None,
                "confirm": self.confirm,
                "response_url_enabled": self.response_url_enabled
                if self.response_url_enabled
                else None,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class TimePicker(Element):
    """
    An interactive UI element that allows users to select a time of day.

    See: <https://api.slack.com/reference/block-kit/block-elements#timepicker>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_time:
        confirm: a `ConfirmationDialogue` object that will be presented when
            the input field is used.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.
        timezone: a string in the IANA format, e.g. "America/Chicago".

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_time: str | None = None,
        confirm: ConfirmationDialogue | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
        timezone: str | None = None,
    ) -> None:
        super().__init__(type_=ElementType.TIME_PICKER)
        self.action_id = validate_action_id(action_id)
        self.initial_time = initial_time
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder,
            max_length=150,
            force_plaintext=True,
            allow_none=True,
        )
        self.timezone = timezone

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_time": self.initial_time if self.initial_time else None,
                "confirm": self.confirm,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
                "timezone": self.timezone,
            }
        )


class URLInput(Element):
    """
    An interactive UI element for collecting URL input from users.

    See: <https://api.slack.com/reference/block-kit/block-elements#url>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_value: the text to populate the input field with when it
            is first rendered.
        dispatch_action_config: a `DispatchActionConfiguration` object that
            determines when during text input the element returns a
            `block_actions` payload.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the input when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_value: str | None = None,
        dispatch_action_config: DispatchActionConfiguration | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(type_=ElementType.URL_INPUT)
        self.action_id = validate_action_id(action_id)
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder,
            force_plaintext=True,
            max_length=150,
            allow_none=True,
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_value": self.initial_value,
                "dispatch_action_config": self.dispatch_action_config,
                "focus_on_load": self.focus_on_load if self.focus_on_load else None,
                "placeholder": self.placeholder if self.placeholder else None,
            }
        )


class ButtonStyle(Enum):
    """
    Utility class for determining the style of `Buttons` and `WorkflowButtons`.
    """

    DEFAULT = None
    PRIMARY = "primary"
    DANGER = "danger"

    @staticmethod
    def to_button_style(style: ButtonStyle | str | None) -> ButtonStyle:
        if isinstance(style, ButtonStyle):
            return style
        if isinstance(style, str):
            return ButtonStyle[style]
        raise InvalidUsageError(
            f"Can only coerce to ButtonStyle from ButtonStyle or string, not {type(style)}."
        )


ButtonStyleLike = ButtonStyle | str


class WorkflowButton(Element):
    """
    An interactive component that allows users to run a link trigger with
        customizable inputs.

    See: <https://api.slack.com/reference/block-kit/block-elements#workflow_button>.

    Args:
        text: the text content that will appear in the button.
        workflow: a [`Workflow`](/slackblocks/latest/reference/objects/#objects.Workflow) object
            that contains details about the workflow that will run when the
            button is clicked.
        style: one of `Default`, `Primary`, or `Danger`, determines the
            visual style of the button. Consider using the `ButtonStyle`
            object for this.
        accessibility_label: a string label for longer descriptive text about
            a button element. Used by screen readers (max 75 chars).

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        text: TextLike,
        workflow: Workflow | None = None,
        style: ButtonStyleLike | None = ButtonStyle.DEFAULT,
        accessibility_label: str | None = None,
    ) -> None:
        super().__init__(type_=ElementType.WORKFLOW_BUTTON)
        self.text = Text.to_text(text, force_plaintext=True, max_length=75)
        self.workflow = workflow
        self.style = ButtonStyle.to_button_style(style).value
        self.accessibility_label = accessibility_label

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "text": self.text,
                "workflow": self.workflow,
                "style": self.style,
                "accessibility_label": self.accessibility_label
                if self.accessibility_label
                else None,
            }
        )


class RichTextInput(Element):
    """
    Allows users to enter formatted text in a WYSIWYG editor, similar to the Slack
        messaging experience.

    See: <https://api.slack.com/reference/block-kit/block-elements#rich_text_input>.

    Args:
        action_id: an identifier so the source of the action can be known.
        initial_value: The initial value in the rich text input when it is loaded.
        dispatch_action_config: a `DispatchActionConfiguration` object that
            determines when during text input the element returns a
            `block_actions` payload.
        focus_on_load: whether or not the menu will be set to autofocus
            within the view object.
        placeholder: a plain-text `Text` object (max 150 chars) that shows
            in the menu when it's initially rendered.

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation.
    """

    def __init__(
        self,
        action_id: str,
        initial_value: RichText | None = None,
        dispatch_action_config: DispatchActionConfiguration | None = None,
        focus_on_load: bool = False,
        placeholder: TextLike | None = None,
    ) -> None:
        super().__init__(ElementType.RICH_TEXT_INPUT)
        self.action_id = validate_action_id(action_id)
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder,
            force_plaintext=True,
            max_length=150,
            allow_none=True,
        )

    def _resolve(self) -> dict[str, Any]:
        # Note: focus_on_load is emitted unconditionally (the check used to be
        # ``if self.focus_on_load is not None``, which is always true given the
        # default of False). Preserving this behaviour to avoid breaking the
        # existing test golden file.
        return resolve(
            {
                **self._attributes(),
                "action_id": self.action_id,
                "initial_value": self.initial_value,
                "dispatch_action_config": self.dispatch_action_config,
                "focus_on_load": self.focus_on_load,
                "placeholder": self.placeholder,
            }
        )
