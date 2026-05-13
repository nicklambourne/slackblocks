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
    VIDEO = "video"


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Block:
        """Parse a Slack block payload back into the appropriate ``Block`` subclass.

        Reads ``data["type"]`` and dispatches to the matching subclass's
        ``from_dict``.

        Throws:
            MissingRequiredError: if ``data["type"]`` is absent.
            TypeMismatchError: if ``data["type"]`` is not a recognised
                block type.
            NotImplementedError: when the block type is recognised but its
                round-trip parser depends on a part of the API that is not
                yet implemented (currently: ``RichTextBlock`` and any block
                containing elements -- see Phase 7.4b and 7.4c).
        """
        if "type" not in data:
            raise MissingRequiredError("Block payload is missing required `type` field.")
        type_str = data["type"]
        registry = _block_from_dict_registry()
        if type_str not in registry:
            raise TypeMismatchError(
                f"Unknown block type {type_str!r}; expected one of {sorted(registry)}."
            )
        return registry[type_str](data)


def _block_from_dict_registry() -> dict[str, Any]:
    """Lazy registry mapping ``BlockType`` values to subclass ``from_dict``
    callables. Built on first call to avoid forward-reference issues."""
    return {
        BlockType.ACTIONS.value: ActionsBlock.from_dict,
        BlockType.CONTEXT.value: ContextBlock.from_dict,
        BlockType.DIVIDER.value: DividerBlock.from_dict,
        BlockType.FILE.value: FileBlock.from_dict,
        BlockType.HEADER.value: HeaderBlock.from_dict,
        BlockType.IMAGE.value: ImageBlock.from_dict,
        BlockType.INPUT.value: InputBlock.from_dict,
        BlockType.MARKDOWN.value: MarkdownBlock.from_dict,
        BlockType.RICH_TEXT.value: RichTextBlock.from_dict,
        BlockType.SECTION.value: SectionBlock.from_dict,
        BlockType.TABLE.value: TableBlock.from_dict,
        BlockType.VIDEO.value: VideoBlock.from_dict,
    }


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ActionsBlock:
        """Parse a Slack ``actions`` block payload.

        Currently raises ``NotImplementedError`` because round-tripping
        depends on parsing nested elements; ``Element.from_dict`` will land
        in Phase 7.4b.
        """
        raise NotImplementedError(
            "ActionsBlock.from_dict requires Element.from_dict, which is not yet "
            "implemented (see issue #208 and the planned Phase 7.4b)."
        )


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ContextBlock:
        """Parse a Slack ``context`` block payload.

        Text elements within the block are parsed back to ``Text``; image
        elements raise ``NotImplementedError`` because ``Element.from_dict``
        has not yet shipped (Phase 7.4b).
        """
        elements: list[Element | CompositionObject] = []
        for raw in data.get("elements", []):
            etype = raw.get("type")
            if etype in {TextType.MARKDOWN.value, TextType.PLAINTEXT.value}:
                elements.append(Text.from_dict(raw))
            else:
                raise NotImplementedError(
                    f"ContextBlock.from_dict cannot parse element of type {etype!r}; "
                    "non-Text elements depend on Element.from_dict (Phase 7.4b)."
                )
        return cls(elements=elements or None, block_id=data.get("block_id"))


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> DividerBlock:
        """Parse a Slack ``divider`` block payload."""
        return cls(block_id=data.get("block_id"))


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> FileBlock:
        """Parse a Slack ``file`` block payload."""
        if "external_id" not in data:
            raise MissingRequiredError("FileBlock payload is missing required `external_id` field.")
        return cls(
            external_id=data["external_id"],
            block_id=data.get("block_id"),
            source=data.get("source", "remote"),
        )


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> HeaderBlock:
        """Parse a Slack ``header`` block payload."""
        if "text" not in data:
            raise MissingRequiredError("HeaderBlock payload is missing required `text` field.")
        return cls(text=Text.from_dict(data["text"]), block_id=data.get("block_id"))


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ImageBlock:
        """Parse a Slack ``image`` block payload."""
        if "image_url" not in data:
            raise MissingRequiredError("ImageBlock payload is missing required `image_url` field.")
        title_raw = data.get("title")
        return cls(
            image_url=data["image_url"],
            alt_text=data.get("alt_text", " "),
            title=Text.from_dict(title_raw) if title_raw is not None else None,
            block_id=data.get("block_id"),
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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> InputBlock:
        """Parse a Slack ``input`` block payload.

        Currently raises ``NotImplementedError`` because the nested
        ``element`` requires ``Element.from_dict``, which lands in Phase 7.4b.
        """
        raise NotImplementedError(
            "InputBlock.from_dict requires Element.from_dict, which is not yet "
            "implemented (see issue #208 and the planned Phase 7.4b)."
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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> MarkdownBlock:
        """Parse a Slack ``markdown`` block payload."""
        if "text" not in data:
            raise MissingRequiredError("MarkdownBlock payload is missing required `text` field.")
        return cls(text=data["text"], block_id=data.get("block_id"))


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RichTextBlock:
        """Parse a Slack ``rich_text`` block payload.

        Currently raises ``NotImplementedError`` because the rich-text
        object hierarchy has a deeply nested element graph; round-tripping
        is deferred to Phase 7.4c.
        """
        raise NotImplementedError(
            "RichTextBlock.from_dict is not yet implemented; rich-text "
            "round-tripping is scheduled for Phase 7.4c (see issue #208)."
        )


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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SectionBlock:
        """Parse a Slack ``section`` block payload.

        Round-trips ``text`` and ``fields``. Raises ``NotImplementedError``
        if an ``accessory`` is present, because the accessory is an
        ``Element`` and ``Element.from_dict`` is not yet implemented
        (Phase 7.4b).
        """
        if data.get("accessory") is not None:
            raise NotImplementedError(
                "SectionBlock.from_dict cannot yet parse an `accessory`; "
                "this requires Element.from_dict (Phase 7.4b)."
            )
        text_raw = data.get("text")
        fields_raw = data.get("fields")
        return cls(
            text=Text.from_dict(text_raw) if text_raw is not None else None,
            fields=[Text.from_dict(f) for f in fields_raw] if fields_raw else None,
            accessory=None,
            block_id=data.get("block_id"),
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

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> TableBlock:
        """Parse a Slack ``table`` block payload.

        Currently raises ``NotImplementedError`` because table cells may
        contain rich-text objects whose round-trip parser is deferred to
        Phase 7.4c.
        """
        raise NotImplementedError(
            "TableBlock.from_dict is not yet implemented; round-tripping requires "
            "the rich-text parsers planned for Phase 7.4c (see issue #208)."
        )


class VideoBlock(Block):
    """
    Embeds a video. Used to display video content inside a Slack message,
    modal, or App Home tab.

    See: <https://api.slack.com/reference/block-kit/blocks#video>.

    Note: Slack restricts which domains may be embedded. The server-side
    whitelist (e.g. YouTube, Vimeo) is enforced by Slack on receipt of the
    payload, not by this library; supplying an unsupported URL will result
    in a Slack API error rather than an `InvalidUsageError` at construction.

    Args:
        alt_text: a plain-text summary of the video, used for accessibility
            and notifications (max 200 chars).
        thumbnail_url: a URL pointing to the preview image shown before
            playback. Must be HTTPS in production usage.
        title: the title shown above the video player (plain text, max 200
            chars). A `str` is coerced to `TextType.PLAINTEXT` `Text`.
        video_url: the URL of the video to embed. Must point to a
            Slack-supported provider (see Slack's documentation).
        author_name: optional author name shown beneath the video
            (max 50 chars).
        block_id: an optional deterministic identifier for the block.
        description: optional plain-text description below the video
            (max 200 chars). A `str` is coerced to `TextType.PLAINTEXT`.
        provider_icon_url: an optional URL to the provider's icon.
        provider_name: an optional provider name shown alongside the icon
            (max 50 chars).
        title_url: an optional URL to link the title to.

    Throws:
        LengthError: if any length-constrained string exceeds its limit.
    """

    def __init__(
        self,
        alt_text: str,
        thumbnail_url: str,
        title: TextLike,
        video_url: str,
        author_name: str | None = None,
        block_id: str | None = None,
        description: TextLike | None = None,
        provider_icon_url: str | None = None,
        provider_name: str | None = None,
        title_url: str | None = None,
    ) -> None:
        super().__init__(type_=BlockType.VIDEO, block_id=block_id)
        self.alt_text = validate_string_nonnull(
            alt_text, field_name="alt_text", min_length=1, max_length=200
        )
        self.thumbnail_url = validate_string_nonnull(
            thumbnail_url, field_name="thumbnail_url", min_length=1
        )
        self.title = Text.to_text(title, force_plaintext=True, max_length=200)
        self.video_url = validate_string_nonnull(video_url, field_name="video_url", min_length=1)
        self.author_name = validate_string(
            author_name, field_name="author_name", max_length=50, allow_none=True
        )
        self.description = Text.to_text(
            description, force_plaintext=True, max_length=200, allow_none=True
        )
        self.provider_icon_url = validate_string(
            provider_icon_url, field_name="provider_icon_url", allow_none=True
        )
        self.provider_name = validate_string(
            provider_name, field_name="provider_name", max_length=50, allow_none=True
        )
        self.title_url = validate_string(title_url, field_name="title_url", allow_none=True)

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **self._attributes(),
                "alt_text": self.alt_text,
                "thumbnail_url": self.thumbnail_url,
                "title": self.title,
                "video_url": self.video_url,
                "author_name": self.author_name,
                "description": self.description,
                "provider_icon_url": self.provider_icon_url,
                "provider_name": self.provider_name,
                "title_url": self.title_url,
            }
        )

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> VideoBlock:
        """Parse a Slack ``video`` block payload."""
        required = ("alt_text", "thumbnail_url", "title", "video_url")
        for field in required:
            if field not in data:
                raise MissingRequiredError(
                    f"VideoBlock payload is missing required `{field}` field."
                )
        description_raw = data.get("description")
        return cls(
            alt_text=data["alt_text"],
            thumbnail_url=data["thumbnail_url"],
            title=Text.from_dict(data["title"]),
            video_url=data["video_url"],
            author_name=data.get("author_name"),
            block_id=data.get("block_id"),
            description=Text.from_dict(description_raw) if description_raw is not None else None,
            provider_icon_url=data.get("provider_icon_url"),
            provider_name=data.get("provider_name"),
            title_url=data.get("title_url"),
        )
