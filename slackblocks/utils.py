"""
This module collects various utility functions used for validating
the input to `Messages`, `Blocks`, `Elements` and `Objects`.
"""

from string import hexdigits
from typing import Any, List, Optional, Tuple, TypeVar, Union

from .errors import InvalidUsageError

T = TypeVar("T")


def coerce_to_list(
    object_or_objects: Union[T, List[T]],
    class_: Union[Any, List[Any]],
    allow_none: bool = False,
    min_size: Optional[int] = None,
    max_size: Optional[int] = None,
) -> List[T]:
    """
    Takes and object or list of objects and validates its contents, ensuring that the
    resulting object is a list.

    Args:
        object_or_objects: the Python object or objects to validate and convert to a list.
        class_: the Python type (or class) of objects expected in the list.
        allow_none: whether or not None is a valid input (and thus output) option.
        min_size: if provided, the length of `object_or_objects` cannot be smaller than this.
        max_size: if provided, the length of `object_or_objects` cannot be larger than this.

    Returns:
        `object_or_objects` if it was a valid list, `[object_or_objects]` if it was a valid
            object, or `None` if provided and allowed.

    Throws:
        InvalidUsageError: if any of the validation checks fail.
    """
    if object_or_objects is None and allow_none:
        return None
    if object_or_objects is None and not allow_none:
        raise InvalidUsageError(
            f"Type of {object_or_objects} ({type(object_or_objects)})) is "
            f"None should be type `{class_}`."
        )

    if isinstance(object_or_objects, List):
        items = object_or_objects
    else:
        items = [
            object_or_objects,
        ]

    for item in items:
        if not isinstance(class_, Tuple):
            class_ = (class_,)
        if not isinstance(item, class_):
            raise InvalidUsageError(
                f"Type of {item} ({type(item)})) inconsistent with expected type {class_}."
            )

    if items is not None:
        length = len(items)
        if min_size is not None and length < min_size:
            raise InvalidUsageError(
                f"Size ({length}) of list of {type(class_)} is less than `min_size` ({min_size})"
            )

        if max_size is not None and length > max_size:
            raise InvalidUsageError(
                f"Size ({length}) of list of {type(class_)} exceeds `max_size` ({max_size})"
            )

    return items


def is_hex(string: str) -> bool:
    """
    Determines whether a given string is a valid hexadecimal number.

    Args:
        string: the string to examine for hex characters.

    Returns:
        `True` if the string is a valid hexadecimal number, otherwise `False`.
    """
    return all(char in hexdigits for char in string)


def validate_action_id(action_id: str, allow_none: bool = False) -> Optional[str]:
    """
    Action IDs are used in the handing of user interactivity within Slack blocks.
    This function checks that a given `action_id` is valid as per the requirements
    imposed by the Slack API.

    See: <https://api.slack.com/interactivity/handling>

    Args:
        action_id: the action_id string to validate for correctness as per the Slack API.
        allow_none: whether to accept `None` as a valid value for `action_id`.

    Returns:
        The original value `action_id` if all validation checks pass.

    Throws:
        InvalidUsageError if any of the validation checks fail.
    """
    if action_id is None:
        if not allow_none:
            raise InvalidUsageError("`action_id` cannot be None.")
    else:
        length = len(action_id)
        if length < 1:
            raise InvalidUsageError("`action_id` cannot be empty.")
        if length > 255:
            raise InvalidUsageError(
                f"`action_id` length ({length}) exceeds limit of 255 characters (id: {action_id})."
            )
    return action_id


def validate_string(
    string: Optional[str],
    field_name: str,
    max_length: Optional[int] = None,
    min_length: Optional[int] = None,
    allow_none: bool = False,
) -> Optional[str]:
    """
    Performs basic validation actions (e.g. length checking) on a given string
    based on the provided criteria.

    Args:
        string: the string to validate
        field_name: the name of the field the string belongs to (for error reporting purposes).
        min_length: if the string is less than this length, an error will be raised.
        max_length: if the string is greated than this length, an error will be raised.
        allow_none: whether `None` is a valid value for the string being validated.

    Returns:
        The original string if it deemed to be valid (i.e. no errors are thrown).

    Throws:
        InvalidUsageError: if any of the validation checks (length, `None`) fail.
    """
    if string is None:
        if not allow_none:
            raise InvalidUsageError(
                f"Expecting string for field `{field_name}`, cannot be None."
            )
    else:
        length = len(string)
        if min_length and length < min_length:
            raise InvalidUsageError(
                f"Argument to field `{field_name}` ({length} characters) "
                f"is less than minimum length of {min_length} characters"
            )
        if max_length and length > max_length:
            raise InvalidUsageError(
                f"Argument to field `{field_name}` ({length} characters) "
                f"exceeds length limit of {max_length} characters"
            )
    return string


def validate_int(
    num: Union[int, None],
    min_value: Optional[int] = None,
    max_value: Optional[int] = None,
    allow_none: bool = False,
) -> int:
    """
    Performs basic validation checks against a given integer.

    Args:
        num: the number to validate.
        min_value: if `num` is less than this value, an error will be thrown.
        max_value: if `num` is greater than this value, an error will be thrown.
        allow_none: whether `None` is a valid value for `num`. If `num` is `None`
            `allow_none` is `False`, an error will be thrown.

    Returns:
        The original value of `num` if it passes all validation checks.

    Throws:
        InvalidUsageError: if any of the validation checks fail.
    """
    if num is None and not allow_none:
        raise InvalidUsageError("`num` is None, which is disallowed.")
    if min_value is not None:
        if num < min_value:
            raise InvalidUsageError(f"{num} is less than the minimum {min_value}")
    if max_value is not None:
        if num > max_value:
            raise InvalidUsageError(f"{num} is less than the minimum {max_value}")
    return num
