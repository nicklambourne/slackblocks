"""
Rich text objects are containers for rich text elements.

These obejects form the contents of the
    [`RichTextBlock`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock).
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum
from typing import Any

from slackblocks._core import RenderableMixin, resolve
from slackblocks.errors import TypeMismatchError
from slackblocks.rich_text.elements import (
    RichText,
    RichTextChannel,
    RichTextElement,
    RichTextEmoji,
    RichTextLink,
    RichTextUser,
    RichTextUserGroup,
)
from slackblocks.utils import coerce_to_list, validate_int


class RichTextObjectType(Enum):
    SECTION = "rich_text_section"
    LIST = "rich_text_list"
    PREFORMATTED = "rich_text_preformatted"
    QUOTE = "rich_text_quote"
    TABLE_CELL = "rich_text_table_cell"


class ListType(Enum):
    """
    An `Enum` that lists the available types of rich text lists.

    - `ListType.BULLET`: an unorderd (bulleted) list.
    - `ListType.ORDERED`: an ordered (numbered) list.
    """

    BULLET = "bullet"
    ORDERED = "ordered"

    @classmethod
    def all(cls) -> list[str]:
        return [list_type.value for list_type in ListType]


class RichTextObject(RenderableMixin, ABC):
    """
    Abstract class housing shared functionality of RichTextObjects.

    Args:
        type_: the type of rich text object this class is, from
            `RichTextObjectType`.
    """

    def __init__(self, type_: RichTextObjectType) -> None:
        self.type_ = type_

    @abstractmethod
    def _resolve(self) -> dict[str, Any]:
        return {"type": self.type_.value}


class RichTextSection(RichTextObject):
    """
    The most basic rich text container object, which takes rich text elements
        and renders them when `RichTextSection` is passed to a
        [`RichTextBlock`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock).

    See: <https://api.slack.com/reference/block-kit/blocks#rich_text_section>.

    Args:
        elements: one or more rich text elements that will form the content of the section.
            e.g. `RichText`, `RichTextLink`.

    Throws:
        InvalidUsageError: if any of the items passed to `elements` isn't a valid
            `RichTextObject`.
    """

    def __init__(self, elements: RichTextElement | list[RichTextElement]) -> None:
        super().__init__(type_=RichTextObjectType.SECTION)
        self.elements = coerce_to_list(
            elements,
            class_=(
                RichTextChannel,
                RichTextEmoji,
                RichTextLink,
                RichText,
                RichTextUser,
                RichTextUserGroup,
            ),
            min_size=1,
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve({**super()._resolve(), "elements": self.elements})


class RichTextList(RichTextObject):
    """
    Renders to a HTML list containing rich text elements.

    See: <https://api.slack.com/reference/block-kit/blocks#rich_text_list>.

    Args:
        style: one of `ListType.BULLET` or `ListType.ORDERED`.
        elements: a list of (possibly nested) `RichTextSection` elements.
            Each object in this list will be rendered as a list item.
        indent: indent (in pixels) of each list item.
        offset: offset (in pixels) of each list item.
        border: thickness (in pixels) of the (optional) border around the list.

    Throws:
        InvalidUsageError: if style is not a valid `ListType` or any of the
            items in `elements` isn't a valid `RichTextSection`.
    """

    def __init__(
        self,
        style: str | ListType,
        elements: RichTextSection | list[RichTextSection],
        indent: int | None = None,
        offset: int | None = 0,
        border: int | None = 0,
    ) -> None:
        super().__init__(type_=RichTextObjectType.LIST)
        if isinstance(style, str):
            if style in ListType.all():
                self.style = style
            else:
                raise TypeMismatchError(f"`style` must be one of [{ListType.all()}]")
        elif isinstance(style, ListType):
            self.style = style.value
        self.elements = coerce_to_list(elements, RichTextSection, min_size=1)
        self.indent = validate_int(indent, allow_none=True)
        self.offset = validate_int(offset, allow_none=True)
        self.border = validate_int(border, allow_none=True)

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **super()._resolve(),
                "elements": [element for element in self.elements if element is not None]
                if self.elements is not None
                else None,
                "style": self.style,
                "indent": self.indent,
                "offset": self.offset,
                "border": self.border,
            }
        )


class RichTextCodeBlock(RichTextObject):
    """
    A rich text element for representing blocks of code in
        [`RichTextBlocks`](/slackblocks/latest/reference/blocks/#blocks.RichTextBlock).

    This is roughly equivalent to the triple-backtick ```code``` syntax in markdown.

    See: <https://api.slack.com/reference/block-kit/blocks#rich_text_preformatted>.

    Args:
        elements: one or more rich text primitive objexts
            (e.g. [`RichText`](/slackblocks/latest/reference/rich_text/#rich_text.RichText)).
        border: the thickness (in pixels) of the border around the code block.

    Throws:
        InvalidUsageError: if any of the items in `elements` aren't valid rich
            text elements.
    """

    def __init__(
        self,
        elements: RichTextElement | list[RichTextElement],
        border: int | None = None,
    ) -> None:
        super().__init__(type_=RichTextObjectType.PREFORMATTED)
        self.elements = coerce_to_list(
            elements,
            (
                RichText,
                RichTextChannel,
                RichTextEmoji,
                RichTextLink,
                RichTextUser,
                RichTextUserGroup,
            ),
        )
        self.border = border

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **super()._resolve(),
                "elements": [element for element in self.elements if element is not None]
                if self.elements is not None
                else None,
                "border": self.border,
            }
        )


class RichTextQuote(RichTextObject):
    """
    A rich text object for representing a block quote.

    Block quotes are presented with a vertical bar to the left hand side of
        the text.

    See: <https://api.slack.com/reference/block-kit/blocks#rich_text_quote>

    Args:
        elements: one or more rich text primitive objexts
            (e.g. [`RichText`](/slackblocks/latest/reference/rich_text/#rich_text.RichText)).
        border: the thickness (in pixels) of the border around the code block.
    """

    def __init__(
        self,
        elements: RichTextElement | list[RichTextElement],
        border: int | None = None,
    ) -> None:
        super().__init__(RichTextObjectType.QUOTE)
        self.elements = coerce_to_list(
            elements,
            (
                RichText,
                RichTextChannel,
                RichTextEmoji,
                RichTextLink,
                RichTextUser,
                RichTextUserGroup,
            ),
        )
        self.border = border

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **super()._resolve(),
                "elements": [element for element in self.elements if element is not None]
                if self.elements is not None
                else None,
                "border": self.border,
            }
        )
