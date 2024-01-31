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
    Takes and object or list of objects and validates its contents, ensuring that the resulting object is a list.

    Args:
        object_or_objects: the Python object or objects to validate and convert to a list.
        class_: the Python type (or class) of objects expected in the list.
        allow_none: whether or not None is a valid input (and thus output) option.
        min_size: if provided, the length of `object_or_objects` cannot be smaller than this.
        max_size: if provided, the length of `object_or_objects` cannot be larger than this.

    Returns:
        `object_or_objects` if it was a valid list, `[object_or_objects]` if it was a valid object, or `None` if 
        provided and allowed.
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
    """
    return all(char in hexdigits for char in string)


def validate_action_id(action_id: str, allow_none: bool = False) -> Optional[str]:
    """
    

    Args:
        action_id: the action_id string to validate for correctness as per the Slack API.
        allow_none: whether to accept `None` as a valid value for `action_id`.
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
                f"is less than minimum length of {max_length} characters"
            )
        if max_length and length > max_length:
            raise InvalidUsageError(
                f"Argument to field `{field_name}` ({length} characters) "
                f"exceeds length limit of {max_length} characters"
            )
    return string
