from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union
from .blocks import Block
from .errors import InvalidUsageError


class Color(Enum):
    """
    Color utility class for use with the Slack secondary attachments API.
    """
    GOOD = "good"
    WARNING = "warning"
    DANGER = "danger"
    RED = "#ff0000"
    BLUE = "#0000ff"
    YELLOW = "#ffff00"
    GREEN = "#00ff00"
    ORANGE = "#ff8800"
    PURPLE = "#8800ff"
    BLACK = "#000000"


class Field:
    """
    Field text objects for use with Slack's secondary attachment API.
    """
    def __init__(self,
                 title: Optional[str] = None,
                 value: Optional[str] = None,
                 short: Optional[bool] = False):
        self.title = title
        self.value = value
        self.short = short

    def _resolve(self):
        field = dict()
        field["short"] = self.short
        if self.title:
            field["title"] = self.title
        if self.value:
            field["value"] = self.value
        return dumps(field)


class Attachment:
    """
    Secondary content can be attached to messages to include lower priority content
     - content that doesn't necessarily need to be seen to appreciate the intent of
    the message, but perhaps adds further context or additional information.
    """
    def __init__(self,
                 blocks: Optional[List[Block]],
                 color: Optional[Union[str, Color]]):
        self.blocks = blocks
        if type(color) is Color:
            self.color = color.value
        elif type(color) is str:
            if len(color) == 7 and color[0] == "#":
                self.color = color
            else:
                raise InvalidUsageError("Color must be a valid hex code (e.g. #ffffff)")

    def _resolve(self) -> Dict[str, Any]:
        attachment = dict()
        attachment["blocks"] = [block._resolve() for block in self.blocks]
        return attachment

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


