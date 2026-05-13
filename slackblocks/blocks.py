"""
Blocks are a series of container components that can be combined to create rich and
interactive messages.

See: <https://api.slack.com/reference/block-kit/blocks>.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum
from typing import Any
from uuid import uuid4

from slackblocks._core import RenderableMixin, resolve
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
    EmailInput,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    NumberInput,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    StaticMultiSelectMenu,
    StaticSelectMenu,
    URLInput,
    UserMultiSelectMenu,
    UserSelectMenu,
)
from slackblocks.errors import (
    InvalidUsageError,
    LengthError,
    MissingRequiredError,
    TypeMismatchError,
)
from slackblocks.objects import (
    ColumnSettings,
    CompositionObject,
    CompositionObjectType,
    RawText,
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
from slackblocks.utils import (
    coerce_to_list,
    coerce_to_list_nonnull,
    validate_string,
    validate_string_nonnull,
)

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
    EmailInput,
    URLInput,
)


ALLOWED_TABLE_CELL_ELEMENTS = (
    RawText,
    RichTextList,
    RichTextCodeBlock,
    RichTextQuote,
    RichTextSection,
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
    MARKDOWN = "markdown"
    RICH_TEXT = "rich_text"
    SECTION = "section"
    TABLE = "table"


class Block(RenderableMixin, ABC):
    """
    Basis block containing attributes and behaviour common to all blocks.
    N.B: Block is an abstract class and cannot be sent directly.
    """

    def __init__(self, type_: BlockType, block_id: str | None = None) -> None:
        self.type = type_
        self.block_id = block_id if block_id else str(uuid4())

    def __add__(self, other: Block):
        return [self, other]

    def _attributes(self):
        return {"type": self.type.value, "block_id": self.block_id}

    @abstractmethod
    def _resolve(self) -> dict[str, Any]:
        pass


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
        elements: list[Element] | None = None,
        block_id: str | None = None,
    ) -> None:
        super().__init__(type_=BlockType.ACTIONS, block_id=block_id)
        self.elements: list[Element] | None = coerce_to_list(
            elements, (Element), allow_none=True, max_size=25
        )

    def _resolve(self) -> dict[str, Any]:
        return resolve({**self._attributes(), "elements": self.elements})


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
        elements: list[Element | CompositionObject] | None = None,
        block_id: str | None = None,
    ) -> None:
        super().__init__(type_=BlockType.CONTEXT, block_id=block_id)
        self.elements = []
        if elements is not None:
            for element in elements:
                if element.type == CompositionObjectType.TEXT or element.type == ElementType.IMAGE:
                    self.elements.append(element)
                else:
                    raise TypeMismatchError(
                        f"Context blocks can only hold image and text elements, not {element.type}"
                    )
        if len(self.elements) > 10:
            raise LengthError("Context blocks can hold a maximum of ten elements")

    def _resolve(self) -> dict[str, Any]:
        return resolve({**self._attributes(), "elements": self.elements})


class DividerBlock(Block):
    """
    A content divider, like an `<hr>` in HTML, to split up different blocks inside of
    a message.

    Args:
        block_id: you can use this field to provide a deterministic identifier for the block.
    """

    def __init__(self, block_id: str | None = None) -> None:
        super().__init__(type_=BlockType.DIVIDER, block_id=block_id)

    def _resolve(self) -> dict[str, Any]:
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
        self,
        external_id: str,
        block_id: str | None = None,
        source: str = "remote",
    ) -> None:
        super().__init__(type_=BlockType.FILE, block_id=block_id)
        self.external_id = external_id
        self.source = source

    def _resolve(self) -> dict[str, Any]:
        return {
            **self._attributes(),
            "external_id": self.external_id,
            "source": self.source,
        }


class HeaderBlock(Block):
    """
    A Header Block is a plain-text block that displays in a larger, bold font.

    Args:
        text: the text that will be rendered as a heading.
        block_id: you can use this field to provide a deterministic identifier for the block.
    """

    def __init__(self, text: str | Text, block_id: str | None = None) -> None:
        super().__init__(type_=BlockType.HEADER, block_id=block_id)
        if type(text) is Text:
            self.text = text
        else:
            self.text = Text.to_text_nonnull(text=text, force_plaintext=True)

    def _resolve(self) -> dict[str, Any]:
        return resolve({**self._attributes(), "text": self.text})


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
        alt_text: str | None = " ",
        title: Text | str | None = None,
        block_id: str | None = None,
    ) -> None:
        super().__init__(type_=BlockType.IMAGE, block_id=block_id)
        self.image_url = validate_string(
            string=image_url,
            field_name="title",
            max_length=3000,
        )
        self.alt_text = validate_string(alt_text, field_name="alt_text", max_length=2000)
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

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "image_url": self.image_url,
                "alt_text": self.alt_text if self.alt_text else None,
                "title": getattr(self, "title", None),
            }
        )


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
        block_id: str | None = None,
        hint: TextLike | None = None,
        optional: bool = False,
    ) -> None:
        super().__init__(type_=BlockType.INPUT, block_id=block_id)
        self.label = Text.to_text(label, force_plaintext=True, max_length=2000, allow_none=False)
        if not isinstance(element, ALLOWED_INPUT_ELEMENTS):
            raise TypeMismatchError(
                f"InputBlocks can only hold elements of type: {ALLOWED_INPUT_ELEMENTS}"
            )
        self.element = element
        self.dispatch_action = dispatch_action
        self.hint = Text.to_text(hint, force_plaintext=True, max_length=2000, allow_none=True)
        self.optional = optional

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "label": self.label,
                "element": self.element,
                "hint": self.hint if self.hint else None,
                "dispatch_action": self.dispatch_action if self.dispatch_action else None,
                "optional": self.optional if self.optional else None,
            }
        )


class MarkdownBlock(Block):
    """
    Displays formatted Markdown text. Unlike the `mrkdwn` text style used in
    [`SectionBlock`](/slackblocks/latest/reference/blocks/#blocks.SectionBlock),
    `MarkdownBlock` uses **GitHub-flavored Markdown** for richer formatting,
    including features like tables and code blocks. Added to Slack in 2024
    for AI / agentic app outputs.

    See: <https://api.slack.com/reference/block-kit/blocks#markdown>.

    Args:
        text: the Markdown-formatted text to display (1-12000 characters).
        block_id: you can use this field to provide a deterministic identifier
            for the block.

    Throws:
        LengthError: if `text` is empty or longer than 12000 characters.
    """

    def __init__(
        self,
        text: str,
        block_id: str | None = None,
    ) -> None:
        super().__init__(type_=BlockType.MARKDOWN, block_id=block_id)
        self.text = validate_string_nonnull(
            text,
            field_name="text",
            min_length=1,
            max_length=12000,
        )

    def _resolve(self) -> dict[str, Any]:
        return {**self._attributes(), "text": self.text}


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
        elements: RichTextObject | list[RichTextObject],
        block_id: str | None = None,
    ) -> None:
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

    def _resolve(self) -> dict[str, Any]:
        return resolve({**self._attributes(), "elements": self.elements})


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
        accessory: an optional [Element](/slackblocks/latest/reference/elements) object
            that will take a secondary place in the block (after or to the side of  `text`
            or `fields`).

    Throws:
        InvalidUsageError: if any of the provided arguments fail validation checks.
    """

    def __init__(
        self,
        text: TextLike | None = None,
        block_id: str | None = None,
        fields: TextLike | list[TextLike] | None = None,
        accessory: Element | None = None,
    ) -> None:
        super().__init__(type_=BlockType.SECTION, block_id=block_id)
        if not text and not fields:
            raise MissingRequiredError(
                "Must supply either `text` or `fields` or `both` to SectionBlock."
            )
        self.text = Text.to_text(text, max_length=3000, allow_none=True)
        self.fields: list[Text] | None
        if fields is not None:
            field_list: list[str | Text] = coerce_to_list_nonnull(fields, class_=(str, Text))
            self.fields = [
                Text.to_text_nonnull(field, max_length=2000)
                for field in field_list
                if field is not None
            ]
            if len(self.fields) > 10:
                raise LengthError("Section blocks can hold a maximum of ten fields")
        else:
            self.fields = None

        self.accessory = accessory

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "text": self.text if self.text else None,
                "fields": [field for field in self.fields if isinstance(field, Text)]
                if self.fields
                else None,
                "accessory": self.accessory if self.accessory else None,
            }
        )


class TableBlock(Block):
    """
    A `TableBlock` displays data in a table format.

    Args:
        rows: a list of lists of `RawText` or `RichTextObject` objects.
        column_settings: a list of `ColumnSettings` objects.
        block_id: you can use this field to provide a deterministic identifier for the block.

    Throws:
        InvalidUsageError: when items in `rows` are not `RawText` or `RichTextObject` objects.
        InvalidUsageError: when the number of column_settings does not match the number of
            columns in each row.
        InvalidUsageError: when the number of rows is greater than 100.
        InvalidUsageError: when the number of columns in a row is greater than 20.
        InvalidUsageError: when the number of column_settings is greater than 20.
    """

    def __init__(
        self,
        rows: list[list[RawText | RichTextObject]],
        column_settings: list[ColumnSettings] | None = None,
        block_id: str | None = None,
    ) -> None:
        super().__init__(type_=BlockType.TABLE, block_id=block_id)
        # Validate that there is at least one row
        if len(rows) < 1:
            raise LengthError("`rows` must have at least one row.")
        # If column_settings are provided, make sure each row has the same number of elements
        num_columns = len(rows[0])
        for row in rows:
            if len(row) != num_columns:
                raise InvalidUsageError("All rows must have the same number of columns.")
        if column_settings is not None and num_columns != len(column_settings):
            raise InvalidUsageError(
                f"Number of column_settings ({len(column_settings)}) must"
                f"match number of columns in each row ({num_columns})."
            )
        if len(rows) > 100:
            raise LengthError("`rows` can have a maximum of 100 items.")
        for row in rows:
            if len(row) > 20:
                raise LengthError("Each row can have a maximum of 20 cells.")
        # Validate each cell is an allowed type
        self.rows = []
        for row in rows:
            validated_row = []
            for cell in row:
                # Validate cell type
                if not isinstance(cell, ALLOWED_TABLE_CELL_ELEMENTS):
                    raise TypeMismatchError(
                        f"Table cells must be one of {ALLOWED_TABLE_CELL_ELEMENTS}"
                    )
                validated_row.append(cell)
            self.rows.append(validated_row)
        if column_settings and len(column_settings) > 20:
            raise LengthError("`column_settings` can have a maximum of 20 items.")
        self.column_settings = column_settings

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "rows": [[self._resolve_cell(cell) for cell in row] for row in self.rows],
                "column_settings": self.column_settings if self.column_settings else None,
            }
        )

    def _resolve_cell(self, cell: RawText | RichTextObject) -> dict[str, Any]:
        """
        Resolve a table cell to its JSON representation.

        RawText cells are resolved directly.
        RichTextObject cells are wrapped in a rich_text structure.
        """
        if isinstance(cell, RawText):
            return cell._resolve()
        else:
            # Wrap RichTextObject in rich_text structure
            return {"type": "rich_text", "elements": [cell._resolve()]}
