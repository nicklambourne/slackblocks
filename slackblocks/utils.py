from string import hexdigits
from typing import Any, List, Union, TypeVar

from .errors import InvalidUsageError

T = TypeVar("T")


def coerce_to_list(
    object_or_objects: Union[T, List[T]], class_: Any, allow_none: bool = False
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

    return items


def is_hex(string: str) -> bool:
    return all(char in hexdigits for char in string)
