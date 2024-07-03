"""
Blocks are a series of container components that can be combined to create rich and
interactive messages.

See: <https://api.slack.com/reference/block-kit/blocks>.
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
    DateTimePicker,
    Element,
    ElementType,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    StaticMultiSelectMenu,
    StaticSelectMenu,
    UserMultiSelectMenu,
    UserSelectMenu,
    NumberInput,
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
    RichTextCodeBlock,
    RichTextList,
    RichTextObject,
    RichTextQuote,
    RichTextSection,
)
from slackblocks.utils import coerce_to_list, validate_string

ALLOWED_INPUT_ELEMENTS = (
    PlainTextInput,
    NumberInput,
    CheckboxGroup,
    RadioButtonGroup,
    DatePicker,
    DateTimePicker,
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
    RichTextInput,
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
    A `Block` that is used to hold interactive elements (normally for users to interface with).

    Args:
        elements: a list of [Elements](/slackblocks/latest/reference/elements)
            (up to a maximum of 25).
        block_id: you can use this field to provide a deterministic identifier for the block.

    Throws:
        InvalidUsageError: if any of the items in `elements` are invalid.
    """

    def __init__(
        self,
        elements: Optional[List[Element]] = None,
        block_id: Optional[str] = None,
    ) -> "ActionsBlock":
        super().__init__(type_=BlockType.ACTIONS, block_id=block_id)
        self.elements = coerce_to_list(
            elements, (Element), allow_none=True, max_size=25
        )

    def _resolve(self):
        actions = self._attributes()
        actions["elements"] = [element._resolve() for element in self.elements]
        return actions


class ContextBlock(Block):
    """
    A `ContextBlock` displays contextul message info, including both images and text.

    Args:
        elements: a list of `Text` objects and `Image` elements.
        block_id: you can use this field to provide a deterministic identifier for the block.

    Throws:
        InvalidUsageError: when items in `elements` are not `Text` or `Image` or exceed 10 items.
    """

    def __init__(
        self,
        elements: Optional[List[Union[Element, CompositionObject]]] = None,
        block_id: Optional[str] = None,
    ) -> "ContextBlock":
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
    A content divider, like an `<hr>` in HTML, to split up different blocks inside of
    a message.

    Args:
        block_id: you can use this field to provide a deterministic identifier for the block.
    """

    def __init__(self, block_id: Optional[str] = None) -> "DividerBlock":
        super().__init__(type_=BlockType.DIVIDER, block_id=block_id)

    def _resolve(self):
        return self._attributes()


class FileBlock(Block):
    """
    Displays a remote file (e.g. a PDF).

    For details on how remote files are exposed to Slack, see
    <https://api.slack.com/messaging/files#adding>.

    Args:
        external_id: the ID assigned to the remote file when it was added to Slack.
        block_id: you can use this field to provide a deterministic identifier for the block.
        source: always "remote" as per the Slack API (may change in the future).
    """

    def __init__(
        self, external_id: str, block_id: Optional[str], source: str = "remote"
    ) -> "FileBlock":
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
    A Header Block is a plain-text block that displays in a larger, bold font.

    Args:
        text: the text that will be rendered as a heading.
        block_id: you can use this field to provide a deterministic identifier for the block.
    """

    def __init__(
        self, text: Union[str, Text], block_id: Optional[str] = None
    ) -> "HeaderBlock":
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
    An Image Block contains a single graphic, accessed by URL.

    Args:
        image_url: the URL pointing to the image file you want to display.
        alt_text: alternative text for accessibility purposes and when the image fails to load.
        title: an optional text title to be presented with the image.
        block_id: you can use this field to provide a deterministic identifier for the block.

    Throws:
        InvalidUsageError: when one or more of the provided args fails validation.
    """

    def __init__(
        self,
        image_url: str,
        alt_text: Optional[str] = " ",
        title: Optional[Union[Text, str]] = None,
        block_id: Optional[str] = None,
    ) -> "ImageBlock":
        super().__init__(type_=BlockType.IMAGE, block_id=block_id)
        self.image_url = validate_string(
            string=image_url,
            field_name="title",
            max_length=3000,
        )
        self.alt_text = validate_string(
            alt_text, field_name="alt_text", max_length=2000
        )
        if title and isinstance(title, Text):
            if title.text_type == TextType.MARKDOWN:
                # Coerce title into plaintext
                self.title = Text(
                    text=title.text,
                    type_=TextType.PLAINTEXT,
                    emoji=title.emoji,
                    verbatim=title.verbatim,
                )
            else:
                self.title = title
        elif isinstance(title, str):
            self.title = Text(text=title, type_=TextType.PLAINTEXT)

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        if self.alt_text:
            image["alt_text"] = self.alt_text
        if self.title:
            image["title"] = self.title._resolve()
        return image


class InputBlock(Block):
    """
    A block that collects information from users - it can hold a plain-text
    input element, a checkbox element, a radio button element, a select
    menu element, a multi-select menu element, or a datepicker.

    Args:
        label: the name which identifies the input field.
        element: an interactive [Element](/slackblocks/latest/reference/elements)
            (e.g. a text field).
        dispatch_action: whether the [Element](/slackblocks/latest/reference/elements)
            should trigger the sending of a `block_actions` payload.
        block_id: you can use this field to provide a deterministic identifier for the block.
        hint: an optional additional guide on what input the user should prodive.
        optional: whether this input field may be empty when the user submits e.g. the modal.

    Throws:
        InvalidUsageError: when any of the provided arguments fail validation.
    """

    def __init__(
        self,
        label: TextLike,
        element: Element,
        dispatch_action: bool = False,
        block_id: Optional[str] = None,
        hint: Optional[TextLike] = None,
        optional: bool = False,
    ) -> "InputBlock":
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
    """
    A RichTextBlock is used to provide easier rich text formatting
        than standard markdown text (e.g. in a
        [`SectionBlock`](/slackblocks/latest/reference/blocks/#blocks.SectionBlock))
        and access to text formatting features not available in traditional
        markdown (e.g. strikethrough). See the various rich text elements
        you can include [here](/slackblocks/latest/reference/rich_text).

    Args:
        elements: a single [rich text element](rich_text)
            or a list of those elements.
        block_id: you can use this field to provide a deterministic identifier
            for the block.

    Throws:
        InvalidUsageError: if the elements in `elements` are not valid rich
            text elements.
    """

    def __init__(
        self,
        elements: Union[RichTextObject, List[RichTextObject]],
        block_id: Optional[str] = None,
    ) -> "RichTextBlock":
        super().__init__(type_=BlockType.RICH_TEXT, block_id=block_id)
        self.elements = coerce_to_list(
            elements,
            (
                RichTextList,
                RichTextCodeBlock,
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
    it can be used as a simple text block, or with any of the
    available block elements.

    Section blocks can also optionally be given an "accessory,"
    which is typically one of the interactive
    [Elements](/slackblocks/latest/reference/elements).

    Args:
        text: text to include in the block. Can be a string or `Text` object (of either
            `mrkdwn` or `plaintext` variety). Defaults to markdown if unspecified. One of either
            `text` or `fields` must be provided.
        block_id: you can use this field to provide a deterministic identifier for the block.
        fields: a list of text objects. One of either `text` or `fields` must be provided.
        accessory: an optional [Element](/slackblocks/latest/reference/elements) object that
            will take a secondary place in the block (after or to the side of  `text` or
            `fields`).

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation checks.
    """

    def __init__(
        self,
        text: Optional[TextLike] = None,
        block_id: Optional[str] = None,
        fields: Optional[Union[TextLike, List[TextLike]]] = None,
        accessory: Optional[Element] = None,
    ) -> "SectionBlock":
        super().__init__(type_=BlockType.SECTION, block_id=block_id)
        if not text and not fields:
            raise InvalidUsageError(
                "Must supply either `text` or `fields` or `both` to SectionBlock."
            )
        self.text = Text.to_text(text, max_length=3000, allow_none=True)
        self.fields = coerce_to_list(
            (
                [
                    Text.to_text(field, max_length=2000, allow_none=False)
                    for field in coerce_to_list(
                        fields, class_=(str, Text), allow_none=True
                    )
                ]
                if fields
                else None
            ),
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
