#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, unreachable_pub)]

mod components;
mod error;
mod generated;
mod number;
mod rules;
mod style;
mod text_input;
mod wire;

pub use components::{Accordion, AccordionSection, AccordionSectionBuilder};
pub use error::{ErrorCategory, ValidationError};
pub use number::JsonNumber;
pub use style::RichTextStyle;
pub use text_input::{PlainTextInput, TextInput};
// Explicit facade: independently audited; the model generator does not own this file.
pub use generated::{
    AlertLevel, Attachment, AttachmentBuilder, Block, ButtonElement, ButtonElementBuilder,
    ButtonStyle, ColumnAlign, ColumnSettings, ColumnSettingsBuilder, ConfirmationDialogue,
    ConfirmationDialogueBuilder, ContainerBlock, ContainerBlockBuilder, ContainerWidth,
    ContextElement, DataTableBlock, DataTableBlockBuilder, DataTableCell, DividerBlock,
    DividerBlockBuilder, Element, HeaderBlock, HeaderBlockBuilder, IconButtonIcon, ImageElement,
    ImageElementBuilder, MarkdownText, MarkdownTextBuilder, MessagePayload, MessagePayloadBuilder,
    PlainText, PlainTextBuilder, PlanBlock, PlanBlockBuilder, RawNumber, RawNumberBuilder, RawText,
    RawTextBuilder, ResponseType, RichTextBlock, RichTextBlockBuilder, RichTextBlockElement,
    RichTextList, RichTextListBuilder, RichTextListStyle, RichTextSection, RichTextSectionBuilder,
    RichTextSectionElement, RichTextText, RichTextTextBuilder, SectionBlock, SectionBlockBuilder,
    SelectOption, SelectOptionBuilder, SlackFile, SlackFileBuilder, TableBlock, TableBlockBuilder,
    TableCell, TaskCardBlock, TaskCardBlockBuilder, TaskStatus, Text, UrlSource, UrlSourceBuilder,
};

/// The Cargo package version; independent of the shared specification version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Version of the shared Slack Block Kit contract.
pub const SPEC_VERSION: &str = generated::SPEC_VERSION;
