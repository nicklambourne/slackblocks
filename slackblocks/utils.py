from typing import List

from slackblocks.errors import InvalidUsageError


def coerce_to_list(object_or_objects: Union[Any, List[Any]], class_: class, allow_none: bool = False) -> List[Any]:
    if object_or_objects is None and not allow_none:
        raise InvalidUsageError(f"Type of {item} ({type(item)})) is None should be type {class_}.")
    if isinstance(object_or_objects, List):
        items = object_or_objects
    else:
        items = [object_or_objects,]

    for item in objects:
        if not isinstance(item, class_):
                raise InvalidUsageError(f"Type of {item} ({type(item)})) inconsistent with expected type {class_}.")
    
    return items