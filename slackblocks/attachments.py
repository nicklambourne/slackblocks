"""
Secondary (less important) content can be attached using the deprecated
attachments API. 
See: <https://api.slack.com/reference/messaging/attachments>
"""
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

from slackblocks.blocks import Block
from slackblocks.errors import InvalidUsageError
from slackblocks.utils import coerce_to_list, is_hex


class Color(Enum):
    """
    Color is a utility class for use with the Slack secondary attachments API.
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

    def __repr__(self) -> str:
        return f"<slackblocks Color {self.name}: {self.value}>"


class Field:
    """
    Field text objects for use with Slack's secondary attachment API.

    See <https://api.slack.com/reference/messaging/attachments#fields>.

    Args:
        title: text shown as a bold heading on the field.
        value: text (`mrkdwn` or `plaintext`) representing the value of the field.
        short: whether the contents of the field is short enough to be presented in
            multipe columns.
    """

    def __init__(
        self,
        title: Optional[str] = None,
        value: Optional[str] = None,
        short: Optional[bool] = False,
    ):
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
    Lower priority content can be attached to messages using Attachments.
    This is content that doesn't necessarily need to be seen to appreciate
    the intent of the message, but perhaps adds further context or additional information.

    See <https://api.slack.com/reference/messaging/attachments>.

    N.B: `fields` is a deprecated field, included only for legacy purposes. Other legacy
    fields, e.g. `author_name` are deliberately omitted as they were never implemented in
    `slackblocks`.

    Args:
        blocks: an array of Blocks that define the content of the attachment.
        color: the color (in hex format, e.g. #ffffff) of the vertical bar to the left of the
            attachment content. Consider using the `Color` enum from this module.
        fields: a list of `Field` objects to be included in what's rendered in the attachment.

    Throws:
        InvalidUsageError: if the `color` code provided is invalid.
    """

    def __init__(
        self,
        blocks: Optional[Union[Block, List[Block]]] = None,
        color: Optional[Union[str, Color]] = None,
        fields: Optional[Union[Field, List[Field]]] = None,
    ):
        self.blocks = coerce_to_list(blocks, Block, allow_none=True)
        self.fields = coerce_to_list(fields, Field, allow_none=True)
        if type(color) is Color:
            self.color = color.value
        elif type(color) is str:
            if len(color) == 7 and color.startswith("#") and is_hex(color[1:]):
                self.color = color
            elif len(color) == 6 and is_hex(color):
                self.color = f"#{color}"
            else:
                raise InvalidUsageError(
                    "Color must be a valid hex code (e.g. `#ffffff`)"
                )
        else:
            self.color = None

    def _resolve(self) -> Dict[str, Any]:
        attachment = dict()
        if self.blocks:
            attachment["blocks"] = [block._resolve() for block in self.blocks]
        if self.color:
            attachment["color"] = self.color
        return attachment

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)
