import re
from abc import ABC, abstractmethod
from datetime import datetime
from enum import Enum
from json import dumps
from typing import Any, Dict, Optional, Union

from slackblocks.utils import coerce_to_list

from .errors import InvalidUsageError
from .objects import (ConfirmationDialogue, DispatchActionConfiguration,
                      Option, Text, TextLike)


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
    NUMBER_INPUT = "number_input"


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
        self.action_id = action_id
        self.options = coerce_to_list(options, Option)

    def _resolve(self) -> Dict[str, Any]:
        checkbox_group = self._attributes()
        checkbox_group["action_id"] = self.action_id
        checkbox_group["options"] = [option._resolve() for option in self.options]


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
        if len(action_id) > 255:
            raise InvalidUsageError("`action_id` must be less than 255 chars")
        self.action_id = action_id
        if initial_date:
            self.initial_date = datetime.strptime("%Y-%m-%d").strftime("%Y-%m-%d")
        self.confirm = confirm
        self.focus_on_load = focus_on_load
        self.placeholder = placeholder

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
            date_picker["placeholder"] = self.placeholder
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
        if len(action_id) > 255:
            raise InvalidUsageError("`action_id` must be less than 255 chars")
        self.action_id = action_id
        if initial_datetime:
            self.initial_datetime = initial_datetime
        self.confirm = confirm
        self.focus_on_load = focus_on_load

    def _resolve(self) -> Dict[str, Any]:
        datetime_picker = self._attributes()
        datetime_picker["action_id"] = self.action_id
        if self.initial_date:
            datetime_picker["initial_date"] = self.initial_date
        if self.confirm:
            datetime_picker["confirm"] = self.confirm
        if self.focus_on_load:
            datetime_picker["focus_on_load"] = self.focus_on_load
        return datetime_picker


class EmailInput(Element):
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
        self.action_id = action_id
        self.initial_value = initial_value
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = Text.to_text(
            placeholder, max_length=150, force_plaintext=True
        )

    def _resolve(self):
        email_input = self._attributes()
        email_input["action_id"] = self.action_id
        if self.initial_value:
            email_input["initial_value"] = self.initial_value
        if self.dispatch_action_config:
            email_input[
                "dispatch_action_config"
            ] = self.dispatch_action_config._resolve()
        if self.focus_on_load:
            email_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            email_input["placeholder"] = self.placeholder._resolve()
        return email_input


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


class MultiSelectMenu(Element):
    def __init__(self):
        raise NotImplementedError


class NumberInput(Element):
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
        self.action_id = action_id
        self.initial_value = initial_value
        self.min_value = min_value
        self.max_value = max_value
        if min_value:
            if not is_decimal_allowed:
                if isinstance(min_value, float):
                    raise InvalidUsageError(
                        f"`min_value` ({min_value}) cannot be a float when `is_decimal_allowed` is `False`"
                    )
        if max_value:
            if not is_decimal_allowed:
                if isinstance(max_value, float):
                    raise InvalidUsageError(
                        f"`max_value` ({max_value}) cannot be a float when `is_decimal_allowed` is `False`"
                    )
        if (min_value or min_value == 0) and (max_value or max_value == 0):
            if min_value > max_value:
                raise InvalidUsageError(
                    f"`min_value` ({min_value}) cannot be greater than `max_value` ({min_value})"
                )
        self.dispatch_action_config = dispatch_action_config
        self.focus_on_load = focus_on_load
        self.placeholder = placeholder

    def _resove() -> Dict[str, Any]:
        number_input = self._attributes()
        number_input["is_decimal_allowed"] = is_decimal_allowed
        if self.action_id:
            number_input["action_id"] = self.action_id
        if self.initial_value:
            number_input["initial_value"] = self.initial_value
        if self.min_value:
            number_input["min_value"] = self.min_value
        if self.max_value:
            number_input["max_value"] = self.max_value
        if self.dispatch_action_config:
            number_input[
                "dispatch_action_config"
            ] = self.dispatch_action_config._resolve()
        if self.focus_on_load:
            number_input["focus_on_load"] = self.focus_on_load
        if self.placeholder:
            number_input["placeholder"] = self.placeholder._resolve()
        return number_input


class OverflowMenu(Element):
    def __init__(self):
        raise NotImplementedError


class PlainTextInputs(Element):
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
