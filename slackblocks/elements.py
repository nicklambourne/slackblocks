import re

from abc import abstractmethod, ABC
from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional, Union

from slackblocks.errors import InvalidUsageError
from slackblocks.objects import ConfirmationDialogue, DispatchActionConfiguration, Text, TextLike, Option
from slackblocks.utils import coerce_to_list, MAX_ACTION_ID_LENGTH


def validate_action_id(action_id: Optional[str], is_required=True, class_: Optional[Element] = None) -> str:
    if action_id is None and is_required:
         raise InvalidUsageError(f"`action_id` is required for this interactive element {type(class_) if class_ else ''}")
    if len(action_id) > MAX_ACTION_ID_LENGTH:
            raise InvalidUsageError("`action_id` must be less than 255 chars")
    return action_id


class ElementType(Enum):
    """
    Convenience class for referencing the various message elements Slack
    provides.
    """

    TEXT = "text"
    IMAGE = "image"
    BUTTON = "button"
    CHECKBOXES = "checkboxes"
    DATE_PICKER = "datepicker"
    DATETIME_PICKER = "datetimepicker"
    EMAIL_INPUT = "email_text_input"
    MULTI_SELECT_STATIC = "multi_static_select"
    MULTI_SELECT_EXTERNAL = "multi_external_select"
    MULTI_SELECT_USER_LIST = "multi_users_select"
    MULTI_SELECT_CONVERSATION_LIST = "multi_conversations_select"
    MULTI_SELECT_PUBLIC_CHANNELS = "multi_channels_select"
    NUMBER_INPUT = "number_input"
    OVERFLOW = "overflow"


class Element(ABC):
    """
    Basis element containing attributes and behaviour common to all elements.
    N.B: Element is an abstract class and cannot be used directly.
    """

    def __init__(self, type_: ElementType):
        super().__init__()
        self.type = type_

    def _attributes(self) -> Dict[str, Any]:
        return {"type": self.type.value}

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        pass


class Button(Element):
    """
    An interactive element that inserts a button. The button can be a
    trigger for anything from opening a simple link to starting a complex
    workflow.
    """

    def __init__(
        self,
        text: TextLike,
        action_id: str,
        url: Optional[str] = None,
        value: Optional[str] = None,
        style: Optional[str] = None,
        confirm: Optional["Confirm"] = None,
    ):
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
    

class CheckboxGroup(Element):
    """
    A checkbox group that allows a user to choose multiple items from a list 
    of possible options.
    """
    def __init__(self, action_id: str, options: Union[Option, List[Option]]):
        super().__init__(type_=ElementType.CHECKBOXES)
        self.action_id = validate_action_id(action_id, class_=self)
        self.options = coerce_to_list(options, Option)
        
    def _resolve(self) -> Dict[str, Any]:
        checkbox_group = self._attributes()
        checkbox_group["action_id"] = self.action_id
        checkbox_group["options"] = [
            option._resolve() for option in self.options
        ]
    

class DatePicker(Element):
    def __init__(
            self, 
            action_id: str, 
            initial_date: Optional[str] = None, 
            confirm: ConfirmationDialogue = None,
            focus_on_load: bool = False,
            placeholder: Optional[TextLike] = None,
        ):
        super().__init__(type_=ElementType.DATE_PICKER)
        self.action_id = validate_action_id(action_id, class_=self)
        if initial_date:
            self.initial_date = datetime.strptime("%Y-%m-%d").strftime("%Y-%m-%d")
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        date_picker = self._attributes()
        date_picker["action_id"] = self.action_id
        if self.initial_date:
            date_picker["initial_date"] = self.initial_date
        if self.confirm:
            date_picker["confirm"] = self.confirm
        if self.focus_on_load:
            date_picker["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            date_picker["placeholder"] = self.placeholder._resolve()
        return date_picker


class DateTimePicker(Element):
    def __init__(
        self,
        action_id: str, 
        initial_datetime: Optional[int] = None, 
        confirm: ConfirmationDialogue = None,
        focus_on_load: bool = False,
    ):
        super().__init__(type_=ElementType.DATETIME_PICKER)
        self.action_id = validate_action_id(action_id, class_=self)
        if initial_datetime:
            self.initial_datetime = initial_datetime
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> Dict[str, Any]:
        datetime_picker = self._attributes()
        datetime_picker["action_id"] = self.action_id
        if self.initial_datetime:
            datetime_picker["initial_date"] = self.initial_datetime
        if self.confirm:
            datetime_picker["confirm"] = self.confirm
        if self.focus_on_load:
            datetime_picker["focus_on_load"] = self.focus_on_load
        return datetime_picker


class EmailInput(Element):
    """
    Interactive element for the input of emails.
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
        if len(action_id) > 255:
            raise InvalidUsageError("`action_id` must be less than 255 chars")
        self.action_id = validate_action_id(action_id, class_=self)
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self):
        email_input = self._attributes()
        email_input["action_id"] = self.action_id
        if self.initial_value:
            email_input["initial_value"] = self.initial_value
        if self.dispatch_action_config:
            email_input["dispatch_action_config"] = self.dispatch_action_config._resolve()
        if self.focus_on_load:
            email_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            email_input["placeholder"] = self.placeholder._resolve()


class Image(Element):
    """
    An element to insert an image - this element can be used in section
    and context blocks only. If you want a block with only an image in it,
    you're looking for the Image block.
    """

    def __init__(self, image_url: str, alt_text: str):
        super().__init__(type_=ElementType.IMAGE)
        self.image_url = image_url
        self.alt_text = alt_text

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        return image


class StaticMultiSelectMenu(Element):
    def __init__(self):
        super().__init__(type_=ElementType.MULTI_SELECT_STATIC)


class ExternalMultiSelectMenu(Element):
    def __init__(self):
        super().__init__(type_=ElementType.MULTI_SELECT_EXTERNAL)


class UserList(Element):
    def __init__(self):
        super().__init__(type_=ElementType.MULTI_SELECT_USER_LIST)


class NumberInput(Element):
    """
    This input elements accepts both whole and decimal numbers. 
    e.g. 0.25, 5.5, and -10 are all valid input values. 
    Decimal numbers are only allowed when is_decimal_allowed is 
    equal to true
    """
    def __init__(
        self,
        is_decimal_allowed: bool = True,
        action_id: Optional[str] = None,
        initial_value: Optional[str] = None,
        min_value: Optional[Union[int, float]] = None,
        max_value: Optional[Union[int, float]] = None,
        dispatch_action_config: Optional[DispatchActionConfiguration] = None,
        focus_on_load: bool = False,
        placeholder: Optional[TextLike] = None,
    ):
        super().__init__(type_=ElementType.NUMBER_INPUT)
        self.is_decimal_allowed = is_decimal_allowed
        self.action_id = validate_action_id(action_id, class_=self, is_required=False)
        if min_value > max_value:
            raise InvalidUsageError(
                f"Min value ({min_value}) cannot be greated than max value ({max_value})"
            )
        if not (initial_value < max_value and min_value < initial_value):
            raise InvalidUsageError(
                f"Initial value should be between min ({min_value}) and max ({max_value})"
            )
        self.initial_value = initial_value
        self.min_value = min_value
        self.max_value = max_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, force_plaintext=True, max_length=150, allow_none=True
        )

    def _resolve(self) -> Dict[str, Any]:
        number_input = self._attributes()
        number_input["is_decimal_allowed "] = self.is_decimal_allowed
        if self.action_id:
            number_input["action_id"] = self.action_id
        if self.initial_value:
            number_input["initial_value"] = self.initial_value
        if self.min_value:
            number_input["min_value"] = self.min_value
        if self.max_value:
            number_input["max_value"] = self.max_value
        if self.dispatch_action_config:
            number_input["dispatch_action_config"] = self.dispatch_action_config._resolve()
        if self.focus_on_load:
            number_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            number_input["placeholder"] = self.placeholder._resolve()
        return number_input


class OverflowMenu(Element):
    """
    This is like a cross between a button and a select menu - 
    when a user clicks on this overflow button, they will be 
    presented with a list of options to choose from. Unlike 
    the select menu, there is no typeahead field, and the 
    button always appears with an ellipsis ("…") rather than 
    customisable text.
    """
    def __init__(
        self, 
        action_id: str,
        options: Union[Option, List[Option]],
        confirm: Optional[ConfirmationDialogue]
    ):
        super().__init__(type_=ElementType.OVERFLOW)
        self.action_id = validate_action_id(action_id, class_=self)
        self.options = coerce_to_list(options, Option)
        self.confirm = confirm

    def _resolve(self) -> Dict[str, Any]:
        overflow = self._attributes()
        overflow["action_id"] = self.action_id
        overflow["options"] = [
            option._resolve() for option in self.options
        ]
        if self.confirm:
            overflow["confirm"] = self.confirm._resolve()
        return overflow


class PlainTextInput(Element):
    def __init__(self):
        raise NotImplementedError


class RadioButtonGroup(Element):
    def __init__(self):
        raise NotImplementedError


class SelectMenu(Element):
    def __init__(self):
        raise NotImplementedError


class TimePicker(Element):
    def __init__(self):
        raise NotImplementedError


class URLInput(Element):
    def __init__(self):
        raise NotImplementedError


class WorkflowButton(Element):
    def __init__(self):
        raise NotImplementedError


InputElement = Union[
    Button,
    CheckboxGroup, 
    DatePicker, 
    DateTimePicker, 
    EmailInput, 
    ExternalMultiSelectMenu,
    NumberInput,
    OverflowMenu,
    PlainTextInput,
    RadioButtonGroup,
    StaticMultiSelectMenu,
    TimePicker,
    URLInput,
]