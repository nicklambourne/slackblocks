"""
Rich text objects are containers for rich text elements.

These obejects form the contents of the
    [`RichTextBlock`](../blocks/#blocks.RichTextBlock).
"""

from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

from slackblocks.errors import InvalidUsageError
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


class ListType(Enum):
    """
    An `Enum` that lists the available types of rich text lists.

    - `ListType.BULLET`: an unorderd (bulleted) list.
    - `ListType.ORDERED`: an ordered (numbered) list.
    """

    BULLET = "bullet"
    ORDERED = "ordered"

    def all() -> List[str]:
        return [list_type.value for list_type in ListType]


class RichTextObject(ABC):
    """
    Abstract class housing shared functionality of RichTextObjects.

    Args:
        type_: the type of rich text object this class is, from
            `RichTextObjectType`.
    """

    def __init__(self, type_: RichTextObjectType) -> None:
        self.type_ = type_

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        return {"type": self.type_.value}

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


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

    def __init__(self, elements: Union[RichTextElement, List[RichTextElement]]) -> None:
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

    def _resolve(self) -> Dict[str, Any]:
        section = super()._resolve()
        section["elements"] = [element._resolve() for element in self.elements]
        return section


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
        style: Union[str, ListType],
        elements: Union[RichTextSection, List[RichTextSection]],
        indent: Optional[int] = None,
        offset: Optional[int] = 0,
        border: Optional[int] = 0,
    ) -> None:
        super().__init__(type_=RichTextObjectType.LIST)
        if isinstance(style, str):
            if style in ListType.all():
                self.style = style
            else:
                raise InvalidUsageError(f"`style` must be one of [{ListType.all()}]")
        elif isinstance(style, ListType):
            self.style = style.value
        self.elements = coerce_to_list(elements, RichTextSection, min_size=1)
        self.indent = validate_int(indent, allow_none=True)
        self.offset = validate_int(offset, allow_none=True)
        self.border = validate_int(border, allow_none=True)

    def _resolve(self) -> Dict[str, Any]:
        rich_text_list = super()._resolve()
        rich_text_list["elements"] = [element._resolve() for element in self.elements]
        rich_text_list["style"] = self.style
        if self.indent is not None:
            rich_text_list["indent"] = self.indent
        if self.offset is not None:
            rich_text_list["offset"] = self.offset
        if self.border is not None:
            rich_text_list["border"] = self.border
        return rich_text_list


class RichTextCodeBlock(RichTextObject):
    """
    A rich text element for representing blocks of code in
        [`RichTextBlocks`](reference/blocks/#blocks.RichTextBlock).

    This is roughly equivalent to the triple-backtick \`\`\``code`\`\`\` syntax in markdown.

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
        elements: Union[RichTextElement, List[RichTextElement]],
        border: Optional[int] = None,
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

    def _resolve(self) -> Dict[str, Any]:
        preformatted = super()._resolve()
        preformatted["elements"] = [element._resolve() for element in self.elements]
        if self.border is not None:
            preformatted["border"] = self.border
        return preformatted


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
        elements: Union[RichTextElement, List[RichTextElement]],
        border: Optional[int] = None,
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

    def _resolve(self) -> Dict[str, Any]:
        quote = super()._resolve()
        quote["elements"] = [element._resolve() for element in self.elements]
        if self.border is not None:
            quote["border"] = self.border
        return quote
