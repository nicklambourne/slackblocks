"""
Blocks are a series of components that can be combined to create rich and
interactive messages.
See: https://api.slack.com/reference/block-kit/blocks?ref=bk
"""
from abc import ABC, abstractmethod
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union
from uuid import uuid4

from slackblocks.elements import (
    ChannelMultiSelectMenu,
    ChannelSelectMenu,
    CheckboxGroup,
    ConversationMultiSelectMenu,
    ConversationSelectMenu,
    DatePicker,
    Element,
    ElementType,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    PlainTextInput,
    RadioButtonGroup,
    StaticMultiSelectMenu,
    StaticSelectMenu,
    UserMultiSelectMenu,
    UserSelectMenu,
)
from slackblocks.errors import InvalidUsageError
from slackblocks.objects import (
    CompositionObject,
    CompositionObjectType,
    Text,
    TextLike,
    TextType,
)
from slackblocks.rich_text import (
    RichTextElement,
    RichTextList,
    RichTextPreformattedCodeBlock,
    RichTextQuote,
    RichTextSection,
)
from slackblocks.utils import coerce_to_list

ALLOWED_INPUT_ELEMENTS = (
    PlainTextInput,
    CheckboxGroup,
    RadioButtonGroup,
    DatePicker,
    ChannelSelectMenu,
    ChannelMultiSelectMenu,
    ConversationSelectMenu,
    ConversationMultiSelectMenu,
    ExternalSelectMenu,
    ExternalMultiSelectMenu,
    StaticSelectMenu,
    StaticMultiSelectMenu,
    UserSelectMenu,
    UserMultiSelectMenu,
)


class BlockType(Enum):
    """
    Convenience class for identifying the different types of blocks available
    in the Slack Blocks API and their programmatic names.
    """

    ACTIONS = "actions"
    CONTEXT = "context"
    DIVIDER = "divider"
    FILE = "file"
    HEADER = "header"
    IMAGE = "image"
    INPUT = "input"
    RICH_TEXT = "rich_text"
    SECTION = "section"


class Block(ABC):
    """
    Basis block containing attributes and behaviour common to all blocks.
    N.B: Block is an abstract class and cannot be sent directly.
    """

    def __init__(self, type_: BlockType, block_id: Optional[str] = None):
        self.type = type_
        self.block_id = block_id if block_id else str(uuid4())

    def __add__(self, other: "Block"):
        return [self, other]

    def _attributes(self):
        return {"type": self.type.value, "block_id": self.block_id}

    @abstractmethod
    def _resolve(self) -> Dict[str, any]:
        pass

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class ActionsBlock(Block):
    """
    A block that is used to hold interactive elements.
    """

    def __init__(
        self,
        elements: Optional[List[Union[Element, CompositionObject]]] = None,
        block_id: Optional[str] = None,
    ):
        super().__init__(type_=BlockType.ACTIONS, block_id=block_id)
        self.elements = coerce_to_list(
            elements, (Element, CompositionObject), allow_none=True, max_size=25
        )

    def _resolve(self):
        actions = self._attributes()
        actions["elements"] = [element._resolve() for element in self.elements]
        return actions


class ContextBlock(Block):
    """
    Displays message context, which can include both images and text.
    """

    def __init__(
        self,
        elements: Optional[List[Union[Element, CompositionObjectType]]] = None,
        block_id: Optional[str] = None,
    ):
        super().__init__(type_=BlockType.CONTEXT, block_id=block_id)
        self.elements = []
        for element in elements:
            if (
                element.type == CompositionObjectType.TEXT
                or element.type == ElementType.IMAGE
            ):
                self.elements.append(element)
            else:
                raise InvalidUsageError(
                    f"Context blocks can only hold image and text elements, not {element.type}"
                )
        if len(self.elements) > 10:
            raise InvalidUsageError("Context blocks can hold a maximum of ten elements")

    def _resolve(self) -> Dict[str, any]:
        context = self._attributes()
        context["elements"] = [element._resolve() for element in self.elements]
        return context


class DividerBlock(Block):
    """
    A content divider, like an <hr>, to split up different blocks inside of
    a message. The divider block is nice and neat, requiring only a type.
    """

    def __init__(self, block_id: Optional[str] = None):
        super().__init__(type_=BlockType.DIVIDER, block_id=block_id)

    def _resolve(self):
        return self._attributes()


class FileBlock(Block):
    """
    Displays a remote file.
    """

    def __init__(self, external_id: str, source: str, block_id: Optional[str]):
        super().__init__(type_=BlockType.FILE, block_id=block_id)
        self.external_id = external_id
        self.source = source

    def _resolve(self) -> Dict[str, any]:
        file = self._attributes()
        file["external_id"] = self.external_id
        file["source"] = self.source
        return file


class HeaderBlock(Block):
    """
    A header is a plain-text block that displays in a larger, bold font.
    """

    def __init__(self, text: Union[str, Text], block_id: Optional[str] = None):
        super().__init__(type_=BlockType.HEADER, block_id=block_id)
        if type(text) is Text:
            self.text = text
        else:
            self.text = Text(text, type_=TextType.PLAINTEXT, verbatim=False)

    def _resolve(self) -> Dict[str, any]:
        header = self._attributes()
        header["text"] = self.text._resolve()
        return header


class ImageBlock(Block):
    """
    A simple image block, designed to make those cat photos really pop.
    """

    def __init__(
        self,
        image_url: str,
        alt_text: Optional[str] = "",
        title: Optional[Union[Text, str]] = None,
        block_id: Optional[str] = None,
    ):
        super().__init__(type_=BlockType.IMAGE, block_id=block_id)
        self.image_url = image_url
        self.alt_text = alt_text
        if title and type(title) is Text:
            if title.text_type == TextType.MARKDOWN:
                self.title = Text(
                    text=title.text,
                    type_=TextType.PLAINTEXT,
                    emoji=title.emoji,
                    verbatim=title.verbatim,
                )
            else:
                self.title = title
        elif title:
            self.title = Text(text=title, type_=TextType.PLAINTEXT)
        else:
            self.title = Text(text=" ", type_=TextType.PLAINTEXT)

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        if self.title:
            image["title"] = self.title._resolve()
        return image


class InputBlock(Block):
    """
    A block that collects information from users - it can hold a plain-text
    input element, a checkbox element, a radio button element, a select
    menu element, a multi-select menu element, or a datepicker.
    """

    def __init__(
        self,
        label: TextLike,
        element: Element,
        dispatch_action: bool = False,
        block_id: Optional[str] = None,
        hint: Optional[TextLike] = None,
        optional: bool = False,
    ):
        super().__init__(type_=BlockType.INPUT, block_id=block_id)
        self.label = Text.to_text(
            label, force_plaintext=True, max_length=2000, allow_none=False
        )
        if not isinstance(element, ALLOWED_INPUT_ELEMENTS):
            raise InvalidUsageError("")
        self.element = element
        self.dispatch_action = dispatch_action
        self.hint = Text.to_text(
            hint, force_plaintext=True, max_length=2000, allow_none=True
        )
        self.optional = optional

    def _resolve(self) -> Dict[str, Any]:
        input_block = self._attributes()
        input_block["label"] = self.label._resolve()
        input_block["element"] = self.element._resolve()
        if self.hint:
            input_block["hint"] = self.hint._resolve()
        if self.dispatch_action:
            input_block["dispatch_action"] = self.dispatch_action
        if self.optional:
            input_block["optional"] = self.optional
        return input_block


class RichTextBlock(Block):
    def __init__(
        self,
        elements: Union[RichTextElement, List[RichTextElement]],
        block_id: Optional[str] = None,
    ) -> None:
        super().__init__(type_=BlockType.RICH_TEXT, block_id=block_id)
        self.elements = coerce_to_list(
            elements,
            (
                RichTextList,
                RichTextPreformattedCodeBlock,
                RichTextQuote,
                RichTextSection,
            ),
            min_size=1,
        )

    def _resolve(self) -> Dict[str, Any]:
        rich_text_block = self._attributes()
        rich_text_block["elements"] = [element._resolve() for element in self.elements]
        return rich_text_block


class SectionBlock(Block):
    """
    A section is one of the most flexible blocks available -
    it can be used as a simple text block, in combination with text fields,
    or side-by-side with any of the available block elements.
    """

    def __init__(
        self,
        text: Optional[TextLike] = None,
        block_id: Optional[str] = None,
        fields: Optional[Union[TextLike, List[TextLike]]] = None,
        accessory: Optional[Element] = None,
    ):
        super().__init__(type_=BlockType.SECTION, block_id=block_id)
        if not text and not fields:
            raise InvalidUsageError(
                "Must supply either `text` or `fields` or `both` to SectionBlock."
            )
        self.text = Text.to_text(text, max_length=3000, allow_none=True)
        self.fields = coerce_to_list(
            [
                Text.to_text(field, max_length=2000, allow_none=False)
                for field in coerce_to_list(fields, class_=(str, Text), allow_none=True)
            ]
            if fields
            else None,
            class_=Text,
            allow_none=True,
            max_size=10,
        )

        self.accessory = accessory

    def _resolve(self) -> Dict[str, Any]:
        section = self._attributes()
        if self.text:
            section["text"] = self.text._resolve()
        if self.fields:
            section["fields"] = [field._resolve() for field in self.fields]
        if self.accessory:
            section["accessory"] = self.accessory._resolve()
        return section
