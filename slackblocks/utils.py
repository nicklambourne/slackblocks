from typing import Any, List, Union

from slackblocks.errors import InvalidUsageError


MAX_ACTION_ID_LENGTH = 255


def coerce_to_list(object_or_objects: Union[Any, List[Any]], class_: type, allow_none: bool = False) -> List[Any]:
    if object_or_objects is None and not allow_none:
        raise InvalidUsageError(f"Type of {object_or_objects} ({type(object_or_objects)})) is None should be type {class_}.")
    if isinstance(object_or_objects, List):
        items = object_or_objects
    else:
        items = [object_or_objects,]

    for item in items:
        if not isinstance(item, class_):
                raise InvalidUsageError(f"Type of {item} ({type(item)})) inconsistent with expected type {class_}.")
    
    return items
