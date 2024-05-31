"""
Block elements can be used inside of section, context, input, and actions layout blocks.

See: <https://api.slack.com/reference/block-kit/block-elements>
"""

from abc import ABC, abstractmethod
from datetime import datetime
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

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
from .rich_text import RichText
from .utils import coerce_to_list, validate_action_id, validate_int, validate_string


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

    def __init__(self, type_: ElementType):
        super().__init__()
        self._type = type_

    def _attributes(self) -> Dict[str, Any]:
        return {"type": self._type.value}

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
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
        url: Optional[str] = None,
        value: Optional[str] = None,
        style: Optional[str] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        accessibility_label: Optional[str] = None,
    ) -> "Button":
        super().__init__(type_=ElementType.BUTTON)
        self.text = Text.to_text(text, max_length=75, force_plaintext=True)
        self.action_id = validate_action_id(action_id)
        self.url = validate_string(
            url, field_name="url", max_length=3000, allow_none=True
        )
        self.value = validate_string(
            value,
            field_name="value",
            max_length=2000,
            allow_none=True,
        )
        self.style = style.value if isinstance(style, ButtonStyle) else style
        self.confirm = confirm
        self.accessibility_label = validate_string(
            accessibility_label,
            "accessibility_label",
            max_length=75,
            allow_none=True,
        )

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
        if self.accessibility_label:
            button["accessibility_label"] = self.accessibility_label
        return button


class CheckboxGroup(Element):
    """
    A checkbox group that allows a user to choose multiple items from a list
    of possible options.

    See: <https://api.slack.com/reference/block-kit/block-elements#checkboxes>.

    Args:
        action_id: an identifier so the source of the action can be known.
        options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will
            form the content of the checkbox group.
        initial_options: a list of
            [`Option`](/slackblocks/latest/reference/objects/#objects.Option) objects that will
            be initially selected when first presented to the user.
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
        options: Union[Option, List[Option]],
        initial_options: Optional[Union[Option, List[Option]]] = None,
        confirm: ConfirmationDialogue = None,
        focus_on_load: bool = False,
    ) -> "CheckboxGroup":
        super().__init__(type_=ElementType.CHECKBOXES)
        self.action_id = validate_action_id(action_id)
        self.options = coerce_to_list(options, Option)
        self.initial_options = coerce_to_list(initial_options, Option, allow_none=True)
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> Dict[str, Any]:
        checkbox_group = self._attributes()
        checkbox_group["action_id"] = self.action_id
        checkbox_group["options"] = [option._resolve() for option in self.options]
        if self.initial_options:
            checkbox_group["initial_options"] = [
                option._resolve() for option in self.initial_options
            ]
        if self.confirm:
            checkbox_group["confirm"] = self.confirm._resolve()
        if self.focus_on_load:
            checkbox_group["focus_on_load"] = self.focus_on_load
        return checkbox_group


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
        initial_date: Optional[str] = None,
        confirm: ConfirmationDialogue = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ) -> "DatePicker":
        super().__init__(type_=ElementType.DATE_PICKER)
        self.action_id = validate_action_id(action_id)
        if initial_date:
            self.initial_date = datetime.strptime(initial_date, "%Y-%m-%d").strftime(
                "%Y-%m-%d"
            )
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        date_picker = self._attributes()
        date_picker["action_id"] = self.action_id
        if self.initial_date is not None:
            date_picker["initial_date"] = self.initial_date
        if self.confirm:
            date_picker["confirm"] = self.confirm
        if self.focus_on_load:
            date_picker["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            date_picker["placeholder"] = self.placeholder._resolve()
        return date_picker


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
        initial_datetime: Optional[int] = None,
        confirm: ConfirmationDialogue = None,
        focus_on_load: bool = False,
    ) -> "DateTimePicker":
        super().__init__(type_=ElementType.DATETIME_PICKER)
        self.action_id = validate_action_id(action_id)
        if initial_datetime:
            self.initial_datetime = initial_datetime
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> Dict[str, Any]:
        datetime_picker = self._attributes()
        datetime_picker["action_id"] = self.action_id
        if self.initial_datetime:
            datetime_picker["initial_date_time"] = self.initial_datetime
        if self.confirm:
            datetime_picker["confirm"] = self.confirm
        if self.focus_on_load:
            datetime_picker["focus_on_load"] = self.focus_on_load
        return datetime_picker


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
        initial_value: Optional[str] = None,
        dispatch_action_config: Optional[DispatchActionConfiguration] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.EMAIL_INPUT)
        self.action_id = validate_action_id(action_id)
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self):
        email_input = self._attributes()
        email_input["action_id"] = self.action_id
        if self.initial_value:
            email_input["initial_value"] = self.initial_value
        if self.dispatch_action_config:
            email_input["dispatch_action_config"] = (
                self.dispatch_action_config._resolve()
            )
        if self.focus_on_load:
            email_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            email_input["placeholder"] = self.placeholder._resolve()
        return email_input


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
        action_id: Optional[str] = None,
        filetypes: Optional[Union[str, List[str]]] = None,
        max_files: Optional[int] = None,
    ) -> "FileInput":
        super().__init__(ElementType.FILE_INPUT)
        self.action_id = validate_action_id(action_id)
        self.filetypes = coerce_to_list(
            filetypes,
            (str),
            allow_none=True,
        )
        self.max_files = validate_int(
            max_files, min_value=1, max_value=10, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        file_input = super()._resolve()
        if self.action_id is not None:
            file_input["action_id"] = self.action_id
        if self.filetypes is not None:
            file_input["filetypes"] = self.filetypes
        if self.max_files is not None:
            file_input["max_files"] = self.max_files
        return file_input


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
        slack_file: a
            [`SlackFile`](/slackblocks/latest/reference/objects/#objects.SlackFile)
            (the user must provide either `image_url` or `slack_file`).

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation,
            or both/neither of `image_url` and `slack_file` are provided.
    """

    def __init__(
        self,
        alt_text: str = " ",
        image_url: Optional[str] = None,
        slack_file: Optional[SlackFile] = None,
    ):
        super().__init__(type_=ElementType.IMAGE)
        if image_url is None and slack_file is None:
            raise InvalidUsageError("Must provide one of `image_url` or `slack_file`")
        if image_url and slack_file:
            raise InvalidUsageError("Cannot provide both `image_url` or `slack_file`")
        self.image_url = image_url
        self.alt_text = alt_text
        self.slack_file = slack_file

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        if self.image_url is not None:
            image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        if self.slack_file is not None:
            image["slack_file"] = self.slack_file
        return image


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
        options: Union[Option, List[Option]],
        option_groups: List[OptionGroup] = None,
        initial_options: Optional[
            Union[Option, List[Option], OptionGroup, List[OptionGroup]]
        ] = None,
        confirm: ConfirmationDialogue = None,
        max_selected_items: Optional[int] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.MULTI_SELECT_STATIC)
        self.action_id = validate_action_id(action_id)
        if options and option_groups:
            raise InvalidUsageError(
                "Cannot set both `options` and `option_groups` parameters."
            )
        self.options = coerce_to_list(
            options, class_=Option, allow_none=True, max_size=100
        )
        self.option_groups = coerce_to_list(
            option_groups, class_=OptionGroup, allow_none=True, max_size=100
        )
        self.initial_options = coerce_to_list(
            initial_options, class_=(Option, OptionGroup), allow_none=True, max_size=100
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
            and not all(
                isinstance(option, OptionGroup) for option in self.initial_options
            )
        ):
            raise InvalidUsageError(
                "If using `option_groups` then `initial_options` must also be of type "
                f"`List[OptionGroup]`, not `{type(self.initial_options)}`."
            )

        # Check that Option Text is all TextType.PLAINTEXT
        if self.options:
            options_to_validate = self.options
        if self.option_groups:
            options_to_validate = sum(
                [option_group.options for option_group in option_groups], []
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

    def _resolve(self) -> Dict[str, Any]:
        static_multi_select = self._attributes()
        static_multi_select["action_id"] = self.action_id
        if self.options:
            static_multi_select["options"] = [
                option._resolve() for option in self.options
            ]
        if self.option_groups:
            static_multi_select["option_groups"] = [
                option_group._resolve() for option_group in self.option_groups
            ]
        if self.initial_options:
            static_multi_select["initial_options"] = [
                initial_option._resolve() for initial_option in self.initial_options
            ]
        if self.confirm:
            static_multi_select["confirm"] = self.confirm._resolve()
        if self.max_selected_items:
            static_multi_select["max_selected_items"] = self.max_selected_items
        if self.focus_on_load:
            static_multi_select["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            static_multi_select["placeholder"] = self.placeholder._resolve()
        return static_multi_select


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
        min_query_length: Optional[int] = None,
        initial_options: Optional[
            Union[Option, List[Option], OptionGroup, List[OptionGroup]]
        ] = None,
        confirm: ConfirmationDialogue = None,
        max_selected_items: Optional[int] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.MULTI_SELECT_EXTERNAL)
        self.action_id = validate_action_id(action_id)
        self.min_query_length = min_query_length
        self.initial_options = coerce_to_list(
            initial_options, class_=(Option, OptionGroup), allow_none=True, max_size=100
        )
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        external_select_menu = self._attributes()
        external_select_menu["action_id"] = self.action_id
        if self.min_query_length:
            external_select_menu["min_query_length"] = self.min_query_length
        if self.initial_options:
            external_select_menu["initial_options"] = [
                initial_option._resolve() for initial_option in self.initial_options
            ]
        if self.confirm:
            external_select_menu["confirm"] = self.confirm._resolve()
        if self.max_selected_items:
            external_select_menu["max_selected_items"] = self.max_selected_items
        if self.focus_on_load:
            external_select_menu["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            external_select_menu["placeholder"] = self.placeholder._resolve()
        return external_select_menu


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
        initial_users: Optional[List[str]] = None,
        confirm: ConfirmationDialogue = None,
        max_selected_items: Optional[int] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.MULTI_SELECT_USERS)
        self.action_id = validate_action_id(action_id)
        self.initial_users = coerce_to_list(initial_users, str, allow_none=True)
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        user_multi_select = self._attributes()
        user_multi_select["action_id"] = self.action_id
        if self.initial_users:
            user_multi_select["initial_users"] = self.initial_users
        if self.confirm:
            user_multi_select["confirm"] = self.confirm._resolve()
        if self.max_selected_items:
            user_multi_select["max_selected_items"] = self.max_selected_items
        if self.focus_on_load:
            user_multi_select["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            user_multi_select["placeholder"] = self.placeholder._resolve()
        return user_multi_select


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
        initial_conversations: Optional[List[str]] = None,
        default_to_current_conversation: Optional[bool] = False,
        confirm: ConfirmationDialogue = None,
        max_selected_items: Optional[int] = None,
        filter: Optional[ConversationFilter] = None,
        focus_on_load: Optional[bool] = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.MULTI_SELECT_CONVERSATIONS)
        self.action_id = validate_action_id(action_id)
        self.initial_conversations = coerce_to_list(
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

    def _resolve(self) -> Dict[str, Any]:
        conversation_multi_select = self._attributes()
        conversation_multi_select["action_id"] = self.action_id
        if self.initial_conversations:
            conversation_multi_select["intial_conversations"] = (
                self.initial_conversations
            )
        if self.default_to_current_conversation:
            conversation_multi_select["default_to_current_conversation"] = (
                self.default_to_current_conversation
            )
        if self.confirm:
            conversation_multi_select["confirm"] = self.confirm._resolve()
        if self.max_selected_items:
            conversation_multi_select["max_selected_items"] = self.max_selected_items
        if self.filter:
            conversation_multi_select["filter"] = self.filter._resolve()
        if self.focus_on_load:
            conversation_multi_select["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            conversation_multi_select["placeholder"] = self.placeholder._resolve()
        return conversation_multi_select


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
        initial_channels: Optional[List[str]] = None,
        confirm: ConfirmationDialogue = None,
        max_selected_items: Optional[int] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.MULTI_SELECT_CHANNELS)
        self.action_id = validate_action_id(action_id)
        self.initial_channels = coerce_to_list(
            initial_channels, class_=str, allow_none=True
        )
        self.confirm = confirm
        self.max_selected_items = max_selected_items
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        channel_multi_select = self._attributes()
        channel_multi_select["action_id"] = self.action_id
        if self.initial_channels:
            channel_multi_select["initial_channels"] = [
                initial_option._resolve() for initial_option in self.initial_channels
            ]
        if self.confirm:
            channel_multi_select["confirm"] = self.confirm._resolve()
        if self.max_selected_items:
            channel_multi_select["max_selected_items"] = self.max_selected_items
        if self.focus_on_load:
            channel_multi_select["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            channel_multi_select["placeholder"] = self.placeholder._resolve()
        return channel_multi_select


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
        action_id: Optional[str] = None,
        initial_value: Optional[str] = None,
        min_value: Optional[Union[float, int]] = None,
        max_value: Optional[Union[float, int]] = None,
        dispatch_action_config: Optional[DispatchActionConfiguration] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.NUMBER_INPUT)
        self.is_decimal_allowed = is_decimal_allowed
        self.action_id = validate_action_id(action_id, allow_none=True)
        self.initial_value = initial_value
        self.min_value = min_value
        self.max_value = max_value
        if min_value:
            if not is_decimal_allowed:
                if isinstance(min_value, float):
                    raise InvalidUsageError(
                        f"`min_value` ({min_value}) cannot be a float when "
                        "`is_decimal_allowed` is `False`"
                    )
        if max_value:
            if not is_decimal_allowed:
                if isinstance(max_value, float):
                    raise InvalidUsageError(
                        f"`max_value` ({max_value}) cannot be a float when "
                        "`is_decimal_allowed` is `False`"
                    )
        if (min_value or min_value == 0) and (max_value or max_value == 0):
            if min_value > max_value:
                raise InvalidUsageError(
                    f"`min_value` ({min_value}) cannot be greater than "
                    "`max_value` ({min_value})"
                )
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        number_input = self._attributes()
        number_input["is_decimal_allowed"] = self.is_decimal_allowed
        if self.action_id:
            number_input["action_id"] = self.action_id
        if self.initial_value:
            number_input["initial_value"] = self.initial_value
        if self.min_value:
            number_input["min_value"] = self.min_value
        if self.max_value:
            number_input["max_value"] = self.max_value
        if self.dispatch_action_config:
            number_input["dispatch_action_config"] = (
                self.dispatch_action_config._resolve()
            )
        if self.focus_on_load:
            number_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            number_input["placeholder"] = self.placeholder._resolve()
        return number_input


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
        options: Union[Option, List[Option]],
        confirm: ConfirmationDialogue = None,
    ):
        super().__init__(type_=ElementType.OVERFLOW_MENU)
        self.action_id = validate_action_id(action_id)
        self.options = coerce_to_list(options, Option, min_size=1, max_size=5)
        self.confirm = confirm

    def _resolve(self) -> Dict[str, Any]:
        overflow_menu = self._attributes()
        overflow_menu["action_id"] = self.action_id
        overflow_menu["options"] = [option._resolve() for option in self.options]
        if self.confirm:
            overflow_menu["confirm"] = self.confirm._resolve()
        return overflow_menu


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
        initial_value: Optional[str] = None,
        multiline: bool = False,
        min_length: Optional[int] = None,
        max_length: Optional[int] = None,
        dispatch_action_config: Optional[DispatchActionConfiguration] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        plain_text_input = self._attributes()
        if self.multiline:
            plain_text_input["multiline"] = self.multiline
        if self.action_id:
            plain_text_input["action_id"] = self.action_id
        if self.initial_value:
            plain_text_input["initial_value"] = self.initial_value
        if self.min_length:
            plain_text_input["min_length"] = self.min_length
        if self.max_length:
            plain_text_input["max_length"] = self.max_length
        if self.dispatch_action_config:
            plain_text_input["dispatch_action_config"] = (
                self.dispatch_action_config._resolve()
            )
        if self.focus_on_load:
            plain_text_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            plain_text_input["placeholder"] = self.placeholder._resolve()
        return plain_text_input


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
        options: List[Option],
        initial_option: Optional[Option] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        focus_on_load: bool = False,
    ):
        super().__init__(type_=ElementType.RADIO_BUTTON_GROUP)
        self.action_id = validate_action_id(action_id)
        if len(options) < 1 or len(options) > 10:
            raise InvalidUsageError(
                "Number of options to RadioButtonGroup must be between 1 and 10 (inclusive)."
            )
        self.options = coerce_to_list(options, class_=Option, allow_none=False)
        if initial_option is not None and initial_option not in options:
            raise InvalidUsageError("`initial_option` must be a member of `options`")
        self.initial_option = initial_option
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> Dict[str, Any]:
        radio_button_group = self._attributes()
        radio_button_group["action_id"] = self.action_id
        radio_button_group["options"] = [option._resolve() for option in self.options]
        if self.initial_option:
            radio_button_group["initial_option"] = self.initial_option._resolve()
        if self.confirm:
            radio_button_group["confirm"] = self.confirm._resolve()
        if self.focus_on_load:
            radio_button_group["focus_on_load"] = self.focus_on_load
        return radio_button_group


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
        options: List[Option] = None,
        option_groups: List[OptionGroup] = None,
        initial_option: Optional[Union[Option, OptionGroup]] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.STATIC_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        if options and option_groups:
            raise InvalidUsageError(
                "Cannot set both `options` and `option_groups` parameters."
            )
        self.options = coerce_to_list(
            options, class_=Option, allow_none=True, max_size=100
        )
        self.option_groups = coerce_to_list(
            option_groups, class_=OptionGroup, allow_none=True, max_size=100
        )
        if options and initial_option and not isinstance(initial_option, Option):
            raise InvalidUsageError(
                "If using `options` then `initial_option` must also be of type `Option`, "
                f"not `{type(initial_option)}`."
            )
        if (
            option_groups
            and initial_option
            and not isinstance(initial_option, OptionGroup)
        ):
            raise InvalidUsageError(
                "If using `option_groups` then `initial_option` must also be of type "
                f"`OptionGroup`, not `{type(initial_option)}`."
            )

        # Check that Option Text is all TextType.PLAINTEXT
        if self.options:
            options_to_validate = self.options
        if self.option_groups:
            options_to_validate = sum(
                [option_group.options for option_group in option_groups], []
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

    def _resolve(self) -> Dict[str, Any]:
        static_select_menu = self._attributes()
        static_select_menu["action_id"] = self.action_id
        if self.options:
            static_select_menu["options"] = [
                option._resolve() for option in self.options
            ]
        if self.option_groups:
            static_select_menu["option_groups"] = [
                option_group._resolve() for option_group in self.option_groups
            ]
        if self.initial_option:
            static_select_menu["initial_option"] = self.initial_option._resolve()
        if self.confirm:
            static_select_menu["confirm"] = self.confirm._resolve()
        if self.focus_on_load:
            static_select_menu["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            static_select_menu["placeholder"] = self.placeholder._resolve()
        return static_select_menu


class ExternalSelectMenu(Element):
    """
    A select menu interactive UI element, sourced with externally provided options.

    See:
        <https://api.slack.com/slackblocks/latest/reference/block-kit/block-elements#external_select>. # noqa: E501

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
        initial_option: Union[Option, OptionGroup] = None,
        min_query_length: Optional[int] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        external_select_menu = self._attributes()
        external_select_menu["action_id"] = self.action_id
        if self.initial_option:
            external_select_menu["initial_option"] = self.initial_option._resolve()
        if self.min_query_length is not None:
            external_select_menu["min_query_length"] = self.min_query_length
        if self.confirm:
            external_select_menu["confirm"] = self.confirm._resolve()
        if self.focus_on_load:
            external_select_menu["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            external_select_menu["placeholder"] = self.placeholder._resolve()
        return external_select_menu


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
        initial_user: Optional[str] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.USERS_SELECT_MENU)
        self.action_id = validate_action_id(action_id)
        self.initial_user = initial_user
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        user_select_menu = self._attributes()
        user_select_menu["action_id"] = self.action_id
        if self.initial_user:
            user_select_menu["initial_user"] = self.initial_user
        if self.confirm:
            user_select_menu["confirm"] = self.confirm._resolve()
        if self.focus_on_load:
            user_select_menu["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            user_select_menu["placeholder"] = self.placeholder._resolve()
        return user_select_menu


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
        initial_conversation: Optional[str] = None,
        default_to_current_conversation: Optional[bool] = False,
        confirm: Optional[ConfirmationDialogue] = None,
        response_url_enabled: Optional[bool] = False,
        filter: Optional[ConversationFilter] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        conversation_select_menu = self._attributes()
        conversation_select_menu["action_id"] = self.action_id
        if self.initial_conversation:
            conversation_select_menu["initial_conversation"] = self.initial_conversation
        if self.default_to_current_conversation:
            conversation_select_menu["default_to_current_conversation"] = (
                self.default_to_current_conversation
            )
        if self.confirm:
            conversation_select_menu["confirm"] = self.confirm._resolve()
        if self.response_url_enabled:
            conversation_select_menu["response_url_enabled"] = self.response_url_enabled
        if self.filter:
            conversation_select_menu["filter"] = self.filter
        if self.focus_on_load:
            conversation_select_menu["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            conversation_select_menu["placeholder"] = self.placeholder._resolve()
        return conversation_select_menu


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
        initial_channel: Optional[str] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        response_url_enabled: Optional[bool] = False,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        channel_select_menu = self._attributes()
        channel_select_menu["action_id"] = self.action_id
        if self.initial_channel:
            channel_select_menu["initial_channel"] = self.initial_channel
        if self.confirm:
            channel_select_menu["confirm"] = self.confirm._resolve()
        if self.response_url_enabled:
            channel_select_menu["response_url_enabled"] = self.response_url_enabled
        if self.focus_on_load:
            channel_select_menu["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            channel_select_menu["placeholder"] = self.placeholder._resolve()
        return channel_select_menu


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
        initial_time: Optional[str] = None,
        confirm: Optional[ConfirmationDialogue] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
        timezone: Optional[str] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        time_picker = self._attributes()
        time_picker["action_id"] = self.action_id
        if self.initial_time:
            time_picker["initial_time"] = self.initial_time
        if self.confirm:
            time_picker["confirm"] = self.confirm
        if self.focus_on_load:
            time_picker["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            time_picker["placeholder"] = self.placeholder._resolve()
        if self.timezone is not None:
            time_picker["timezone"] = self.timezone
        return time_picker


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
        initial_value: Optional[str] = None,
        dispatch_action_config: Optional[DispatchActionConfiguration] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        url_input = self._attributes()
        url_input["action_id"] = self.action_id
        if self.initial_value is not None:
            url_input["initial_value"] = self.initial_value
        if self.dispatch_action_config:
            url_input["dispatch_action_config"] = self.dispatch_action_config._resolve()
        if self.focus_on_load:
            url_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            url_input["placeholder"] = self.placeholder
        return url_input


class ButtonStyle(Enum):
    """
    Utility class for determining the style of `Buttons` and `WorkflowButtons`.
    """

    DEFAULT = None
    PRIMARY = "primary"
    DANGER = "danger"

    @staticmethod
    def to_button_style(style: Optional[Union["ButtonStyle", str]]) -> "ButtonStyle":
        if isinstance(style, ButtonStyle):
            return style
        if isinstance(style, (str, None)):
            return ButtonStyle[style]
        raise InvalidUsageError(
            f"Can only coerce to ButtonStyle from ButtonStyle or string, not {type(style)}."
        )


ButtonStyleLike = Union[ButtonStyle, str]


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
        workflow: Optional[Workflow] = None,
        style: Optional[ButtonStyleLike] = ButtonStyle.DEFAULT,
        accessibility_label: Optional[str] = None,
    ):
        super().__init__(type_=ElementType.WORKFLOW_BUTTON)
        self.text = Text.to_text(text, force_plaintext=True, max_length=75)
        self.workflow = workflow
        self.style = ButtonStyle.to_button_style(style).value
        self.accessibility_label = accessibility_label

    def _resolve(self) -> Dict[str, Any]:
        workflow_button = self._attributes()
        workflow_button["text"] = self.text._resolve()
        if self.workflow:
            workflow_button["workflow"] = self.workflow._resolve()
        if self.style is not None:
            workflow_button["style"] = self.style
        if self.accessibility_label:
            workflow_button["accessibility_label"] = self.accessibility_label
        return workflow_button


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
        initial_value: Optional[RichText] = None,
        dispatch_action_config: Optional[DispatchActionConfiguration] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ) -> "RichTextInput":
        super().__init__(ElementType.RICH_TEXT_INPUT)
        self.action_id = validate_action_id(action_id)
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = placeholder

    def _resolve(self) -> Dict[str, Any]:
        rich_text_input = super()._attributes()
        rich_text_input["action_id"] = self.action_id
        if self.initial_value is not None:
            rich_text_input["initial_value"] = self.initial_value._resolve()
        if self.dispatch_action_config is not None:
            rich_text_input["dispatch_action_config"] = self.dispatch_action_config
        if self.focus_on_load is not None:
            rich_text_input["focus_on_load"] = self.focus_on_load
        if self.placeholder is not None:
            rich_text_input["placeholder"] = self.placeholder
        return rich_text_input
