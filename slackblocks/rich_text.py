from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, TypeVar, Union

from .errors import InvalidUsageError
from .utils import coerce_to_list, validate_int


class RichTextObjectType(Enum):
    SECTION = "rich_text_section"
    LIST = "rich_text_list"
    PREFORMATTED = "rich_text_preformatted"
    QUOTE = "rich_text_quote"


class ListType(Enum):
    BULLET = "bullet"
    ORDERED = "ordered"

    def all() -> List[str]:
        return [list_type.value for list_type in ListType]


class RichTextObject(ABC):
    def __init__(self, type_: RichTextObjectType) -> None:
        self.type_ = type_

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        return {"type": self.type_.value}

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class RichTextSection(RichTextObject):
    def __init__(self, elements: Union[RichTextObject, List[RichTextObject]]) -> None:
        super().__init__(type_=RichTextObjectType.SECTION)
        self.elements = coerce_to_list(elements, class_=RichTextObject)

    def _resolve(self) -> Dict[str, Any]:
        section = super()._resolve()
        section["elements"] = [element._resolve() for element in self.elements]
        return section


class RichTextList(RichTextObject):
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


class RichTextSubElement(Enum):
    TEXT = "text"
    LINK = "link"


class RichText(RichTextObject):
    def __init__(
        self,
        text: str,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
        strike: Optional[bool] = None,
    ) -> None:
        super().__init__(type_=RichTextSubElement.TEXT)
        self.text = text
        self.bold = bold
        self.italic = italic
        self.strike = strike

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
        if style:
            rich_text["style"] = style
        return rich_text


class RichTextLink(RichTextObject):
    def __init__(
        self,
        url: str,
        text: Optional[str] = None,
        unsafe: Optional[bool] = None,
        bold: Optional[bool] = None,
        italic: Optional[bool] = None,
    ) -> None:
        super().__init__(type_=RichTextSubElement.LINK)
        self.url = url
        self.text = text
        self.unsafe = unsafe
        self.bold = bold
        self.italic = italic

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
        if style:
            link["style"] = style
        return link


class RichTextPreformattedCodeBlock(RichTextObject):
    def __init__(
        self,
        elements: Union[RichText, RichTextLink, List[Union[RichText, RichTextLink]]],
        border: Optional[int] = None,
    ) -> None:
        super().__init__(type_=RichTextObjectType.PREFORMATTED)
        self.elements = coerce_to_list(elements, (RichText, RichTextLink))
        self.border = border

    def _resolve(self) -> Dict[str, Any]:
        preformatted = super()._resolve()
        preformatted["elements"] = [element._resolve() for element in self.elements]
        if self.border is not None:
            preformatted["border"] = self.border
        return preformatted


class RichTextQuote(RichTextObject):
    def __init__(
        self,
        elements: Union[RichTextObject, List[RichTextObject]],
        border: Optional[int] = None,
    ) -> None:
        super().__init__(RichTextObjectType.QUOTE)
        self.elements = coerce_to_list(elements, RichTextObject)
        self.border = border

    def _resolve(self) -> Dict[str, Any]:
        quote = super()._resolve()
        quote["elements"] = [element._resolve() for element in self.elements]
        if self.border is not None:
            quote["border"] = self.border
        return quote


RichTextElement = TypeVar(
    "RichTextElement",
    RichTextList,
    RichTextPreformattedCodeBlock,
    RichTextQuote,
    RichTextSection,
)
