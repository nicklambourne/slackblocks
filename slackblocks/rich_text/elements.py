"""
Rich text elements are the primitive elements used to populate the rich
    text object "containers", which are then fed into the
    [`RichTextBlock`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock).
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any

from slackblocks._core import omit_none
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
    USER_GROUP = "usergroup"


def _style_dict(**flags: bool | None) -> dict[str, bool] | None:
    """Build a rich text "style" sub-object from named boolean flags.

    Any flag that is explicitly ``None`` is dropped. If every flag is dropped
    (i.e. nothing was set), returns ``None`` so the caller can decide whether
    to emit the ``style`` key at all.
    """
    style = omit_none(flags)
    return style if style else None


class RichTextElement(ABC):
    """
    Abstract base class for all rich text element classes.

    These are the primitives that form the basis of rich text objects and
    the [`RichTextBlock`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock).
    """

    def __init__(self, type_: RichTextElementType) -> None:
        super().__init__()
        self.type_ = type_

    @abstractmethod
    def _resolve(self) -> dict[str, Any]:
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
        bold: bool | None = None,
        italic: bool | None = None,
        strike: bool | None = None,
        code: bool | None = None,
    ) -> None:
        super().__init__(type_=RichTextElementType.TEXT)
        self.text = text
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.code = code

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                **super()._resolve(),
                "text": self.text,
                "style": _style_dict(
                    bold=self.bold, italic=self.italic, strike=self.strike, code=self.code
                ),
            }
        )


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
        bold: bool | None = None,
        italic: bool | None = None,
        strike: bool | None = None,
        highlight: bool | None = None,
        client_highlight: bool | None = None,
        unlink: bool | None = None,
    ) -> None:
        super().__init__(RichTextElementType.CHANNEL)
        self.channel_id = channel_id
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.highlight = highlight
        self.client_highlight = client_highlight
        self.unlink = unlink

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                **super()._resolve(),
                "channel_id": self.channel_id,
                "style": _style_dict(
                    bold=self.bold,
                    italic=self.italic,
                    strike=self.strike,
                    highlight=self.highlight,
                    client_highlight=self.client_highlight,
                    unlink=self.unlink,
                ),
            }
        )


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

    def _resolve(self) -> dict[str, Any]:
        return {**super()._resolve(), "name": self.name}


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
        text: str | None = None,
        unsafe: bool | None = None,
        bold: bool | None = None,
        italic: bool | None = None,
        strike: bool | None = None,
        code: bool | None = None,
    ) -> None:
        super().__init__(type_=RichTextElementType.LINK)
        self.url = url
        self.text = text
        self.unsafe = unsafe
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.code = code

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                **super()._resolve(),
                "url": self.url,
                "text": self.text,
                "unsafe": self.unsafe,
                "style": _style_dict(
                    bold=self.bold, italic=self.italic, strike=self.strike, code=self.code
                ),
            }
        )


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
        bold: bool | None = None,
        italic: bool | None = None,
        strike: bool | None = None,
        highlight: bool | None = None,
        client_highlight: bool | None = None,
        unlink: bool | None = None,
    ) -> None:
        super().__init__(RichTextElementType.USER)
        self.user_id = user_id
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.highlight = highlight
        self.client_highlight = client_highlight
        self.unlink = unlink

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                **super()._resolve(),
                "user_id": self.user_id,
                "style": _style_dict(
                    bold=self.bold,
                    italic=self.italic,
                    strike=self.strike,
                    highlight=self.highlight,
                    client_highlight=self.client_highlight,
                    unlink=self.unlink,
                ),
            }
        )


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
        bold: bool | None = None,
        italic: bool | None = None,
        strike: bool | None = None,
        highlight: bool | None = None,
        client_highlight: bool | None = None,
        unlink: bool | None = None,
    ) -> None:
        super().__init__(RichTextElementType.USER_GROUP)
        self.user_group_id = user_group_id
        self.bold = bold
        self.italic = italic
        self.strike = strike
        self.highlight = highlight
        self.client_highlight = client_highlight
        self.unlink = unlink

    def _resolve(self) -> dict[str, Any]:
        return omit_none(
            {
                **super()._resolve(),
                "usergroup_id": self.user_group_id,
                "style": _style_dict(
                    bold=self.bold,
                    italic=self.italic,
                    strike=self.strike,
                    highlight=self.highlight,
                    client_highlight=self.client_highlight,
                    unlink=self.unlink,
                ),
            }
        )
