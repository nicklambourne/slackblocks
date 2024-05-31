"""
Rich text elements are the primitive elements used to populate the rich
    text object "containers", which are then fed into the
    [`RichTextBlock`](../blocks/#blocks.RichTextBlock).
"""

from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, Optional

from slackblocks.utils import validate_string


class RichTextElementType(Enum):
    """
    Used for identification of the lowest level rich text primitives.
    """

    CHANNEL = "channel"
    EMOJI = "emoji"
    LINK = "link"
    TEXT = "text"
    USER = "user"
    USER_GROUP = "user_group"


class RichTextElement(ABC):
    """
    Abstract base class for all rich text element classes.

    These are the primitives that form the basis of rich text objects and
    the [`RichTextBlock`](../blocks/#blocks.RichTextBlock).
    """

    def __init__(self, type_: RichTextElementType) -> None:
        super().__init__()
        self.type_ = type_

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        return {"type": self.type_.value}

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class RichText(RichTextElement):
    """
    The core unit of the rich text API. Allows for the formatting of text
        with visual styles like bolding, italics and strikethroughs.
        Combined with higher-level containers like `RichTextSection`,
        `RichText` can be used to create complicated and deeply nested
        rich text within Slack messages.

    Args:
        text: the text content to render.
        bold: whether to render the given text in bold font.
        italic: whether to render the given text in italics.
        strike: whether to render the given text with a "strikethrough".
        code: whether to render the given text as an inline code snippet
            (monospaced).
    """

    def __init__(
        self,
        text: str,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
        strike: Optional[bool] = None,
        code: Optional[bool] = None,
    ) -> None:
        super().__init__(type_=RichTextElementType.TEXT)
        self.text = text
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.code = code

    def _resolve(self) -> Dict[str, Any]:
        rich_text = super()._resolve()
        rich_text["text"] = self.text
        style = {}
        if self.bold is not None:
            style["bold"] = self.bold
        if self.italic is not None:
            style["italic"] = self.italic
        if self.strike is not None:
            style["strike"] = self.strike
        if self.code is not None:
            style["code"] = self.code
        if style:
            rich_text["style"] = style
        return rich_text


class RichTextChannel(RichTextElement):
    """
    Rich text rendering of a Slack channel (e.g. #general).

    See: <https://api.slack.com/reference/block-kit/blocks#channel-element-type>

    Args:
        channel_id: the ID of the channel to render. You can get this from
            the channel settings or the URL (if using Slack in the browser).
        bold: whether to render the given channel in bold font.
        italic: whether to render the given channel in italics.
        strike: whether to render the given channel with a "strikethrough".
        highlight: whether to give the channel a distinct highlight when rendered.
        client_highlight: ???
        unlink: whether to remove the link to the channel from the channel when
            rendered.
    """

    def __init__(
        self,
        channel_id: str,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
        strike: Optional[bool] = None,
        highlight: Optional[bool] = None,
        client_highlight: Optional[bool] = None,
        unlink: Optional[bool] = None,
    ) -> None:
        super().__init__(RichTextElementType.CHANNEL)
        self.channel_id = channel_id
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.highlight = highlight
        self.client_highlight = client_highlight
        self.unlink = unlink

    def _resolve(self) -> Dict[str, Any]:
        channel = super()._resolve()
        channel["channel_id"] = self.channel_id
        style = {}
        if self.bold is not None:
            style["bold"] = self.bold
        if self.italic is not None:
            style["italic"] = self.italic
        if self.strike is not None:
            style["strike"] = self.strike
        if self.highlight is not None:
            style["highlight"] = self.highlight
        if self.client_highlight is not None:
            style["client_highlight"] = self.client_highlight
        if self.unlink is not None:
            style["unlink"] = self.unlink
        if style:
            channel["style"] = style
        return channel


class RichTextEmoji(RichTextElement):
    """
    A rich text element for displaying an emoji.

    The emoji can either be one built in to Slack or a custom workspace emoji.

    See: <https://api.slack.com/reference/block-kit/blocks#emoji-element-type>

    Args:
        name: the unique name of the emoji to represent e.g. "wave".

    Throws:
        InvalidUsageError: if the emoji `name` provided is empty.
    """

    def __init__(self, name: str) -> None:
        super().__init__(RichTextElementType.EMOJI)
        self.name = validate_string(name, field_name="name", min_length=1)

    def _resolve(self) -> Dict[str, Any]:
        emoji = super()._resolve()
        emoji["name"] = self.name
        return emoji


class RichTextLink(RichTextElement):
    """
    A rich text primitive to display links in text.

    See: <https://api.slack.com/reference/block-kit/blocks#link-element-type>

    Args:
        url: the url which the link will point to.
        text: the text to render with the link. If not provided, the raw URL
            will be used.
        unsafe: whether the link is "safe".
        bold: whether to render the given text in bold font.
        italic: whether to render the given text in italics.
        strike: whether to render the given text with a "strikethrough".
        code: whether to render the given text as an inline code snippet
            (monospaced).
    """

    def __init__(
        self,
        url: str,
        text: Optional[str] = None,
        unsafe: Optional[bool] = None,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
        strike: Optional[bool] = None,
        code: Optional[bool] = None,
    ) -> None:
        super().__init__(type_=RichTextElementType.LINK)
        self.url = url
        self.text = text
        self.unsafe = unsafe
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.code = code

    def _resolve(self) -> Dict[str, Any]:
        link = super()._resolve()
        link["url"] = self.url
        if self.text is not None:
            link["text"] = self.text
        if self.unsafe is not None:
            link["unsafe"] = self.unsafe
        style = {}
        if self.bold is not None:
            style["bold"] = self.bold
        if self.italic is not None:
            style["italic"] = self.italic
        if self.strike is not None:
            style["strike"] = self.strike
        if self.code is not None:
            style["code"] = self.code
        if style:
            link["style"] = style
        return link


class RichTextUser(RichTextElement):
    """
    Rich text element for representing users in
        [`RichTextBlocks`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock).

    See: <https://api.slack.com/reference/block-kit/blocks#user-element-type>.

    Args:
        user_id: the Slack ID of the user in question, you can get these
            from users' profiles or Slack client requests.
        bold: whether to render the given user in bold font.
        italic: whether to render the given user in italics.
        strike: whether to render the given user with a "strikethrough".
        highlight: whether to give the user a distinct highlight when rendered.
        client_highlight: ???
        unlink: whether to remove the link to the user from the channel when
            rendered.
    """

    def __init__(
        self,
        user_id: str,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
        strike: Optional[bool] = None,
        highlight: Optional[bool] = None,
        client_highlight: Optional[bool] = None,
        unlink: Optional[bool] = None,
    ) -> None:
        super().__init__(RichTextElementType.USER)
        self.user_id = user_id
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.highlight = highlight
        self.client_highlight = client_highlight
        self.unlink = unlink

    def _resolve(self) -> Dict[str, Any]:
        user = super()._resolve()
        user["user_id"] = self.user_id
        style = {}
        if self.bold is not None:
            style["bold"] = self.bold
        if self.italic is not None:
            style["italic"] = self.italic
        if self.strike is not None:
            style["strike"] = self.strike
        if self.highlight is not None:
            style["highlight"] = self.highlight
        if self.client_highlight is not None:
            style["client_highlight"] = self.client_highlight
        if self.unlink is not None:
            style["unlink"] = self.unlink
        if style:
            user["style"] = style
        return user


class RichTextUserGroup(RichTextElement):
    """
    Rich text element for representing groups of users in
        [`RichTextBlocks`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock)`.

    See: <https://api.slack.com/reference/block-kit/blocks#user-element-type>.

    Args:
        user_group_id: the Slack ID of the user group being represented.
        bold: whether to render the given user in bold font.
        italic: whether to render the given user in italics.
        strike: whether to render the given user with a "strikethrough".
        highlight: whether to give the user a distinct highlight when rendered.
        client_highlight: ???
        unlink: whether to remove the link to the user from the channel when
            rendered.
    """

    def __init__(
        self,
        user_group_id: str,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
        strike: Optional[bool] = None,
        highlight: Optional[bool] = None,
        client_highlight: Optional[bool] = None,
        unlink: Optional[bool] = None,
    ) -> None:
        super().__init__(RichTextElementType.USER_GROUP)
        self.user_group_id = user_group_id
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.highlight = highlight
        self.client_highlight = client_highlight
        self.unlink = unlink

    def _resolve(self) -> Dict[str, Any]:
        user_group = super()._resolve()
        user_group["user_group_id"] = self.user_group_id
        style = {}
        if self.bold is not None:
            style["bold"] = self.bold
        if self.italic is not None:
            style["italic"] = self.italic
        if self.strike is not None:
            style["strike"] = self.strike
        if self.highlight is not None:
            style["highlight"] = self.highlight
        if self.client_highlight is not None:
            style["client_highlight"] = self.client_highlight
        if self.unlink is not None:
            style["unlink"] = self.unlink
        if style:
            user_group["style"] = style
        return user_group
