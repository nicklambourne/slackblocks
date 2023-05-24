from string import hexdigits
from typing import Any, List, Optional, Union, TypeVar

from .errors import InvalidUsageError

T = TypeVar("T")


def coerce_to_list(
    object_or_objects: Union[T, List[T]], class_: Any, allow_none: bool = False, max_size: Optional[int] = None
) -> List[T]:
    if object_or_objects is None and not allow_none:
        raise InvalidUsageError(
            f"Type of {object_or_objects} ({type(object_or_objects)})) is None should be type {class_}."
        )
    
    if isinstance(object_or_objects, List):
        items = object_or_objects
    else:
        items = [
            object_or_objects,
        ]

    for item in items:
        if not isinstance(item, class_):
            raise InvalidUsageError(
                f"Type of {item} ({type(item)})) inconsistent with expected type {class_}."
            )
        
    if max_size and len(items) > max_size:
        raise InvalidUsageError(f"Size of list of {type(class_)} exceeds `max_size` ({max_size})")

    return items


def is_hex(string: str) -> bool:
    return all(char in hexdigits for char in string)


def validate_action_id(action_id: str, allow_none: bool = False) -> Optional[str]:
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