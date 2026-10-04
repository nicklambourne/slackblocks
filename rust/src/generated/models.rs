// Generated from spec/model.json; edit the generator, not this file.
use crate::wire::{self, FromWire};
use crate::{ErrorCategory, JsonNumber, PlainTextInput, RichTextStyle, TextInput, ValidationError};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
/// Severity levels supported by alert blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AlertLevel {
    /// Neutral styling.
    Default,
    /// Informational styling.
    Info,
    /// Warning styling.
    Warning,
    /// Error styling.
    Error,
    /// Success styling.
    Success,
}
impl AlertLevel {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Success => "success",
        }
    }
}
impl FromWire for AlertLevel {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("default") => Ok(Self::Default),
            Some("info") => Ok(Self::Info),
            Some("warning") => Ok(Self::Warning),
            Some("error") => Ok(Self::Error),
            Some("success") => Ok(Self::Success),
            _ => Err(wire::mismatch(path, "AlertLevel")),
        }
    }
}
impl Serialize for AlertLevel {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for AlertLevel {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "AlertLevel")
    }
}
impl<'de> Deserialize<'de> for AlertLevel {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for AlertLevel {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// Widths supported by Slack container blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ContainerWidth {
    /// A compact container.
    Narrow,
    /// Slack's standard container width.
    Standard,
    /// A wide container.
    Wide,
    /// The full available width.
    Full,
}
impl ContainerWidth {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Narrow => "narrow",
            Self::Standard => "standard",
            Self::Wide => "wide",
            Self::Full => "full",
        }
    }
}
impl FromWire for ContainerWidth {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("narrow") => Ok(Self::Narrow),
            Some("standard") => Ok(Self::Standard),
            Some("wide") => Ok(Self::Wide),
            Some("full") => Ok(Self::Full),
            _ => Err(wire::mismatch(path, "ContainerWidth")),
        }
    }
}
impl Serialize for ContainerWidth {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for ContainerWidth {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ContainerWidth")
    }
}
impl<'de> Deserialize<'de> for ContainerWidth {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for ContainerWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for ContainerWidth {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// States supported by task card blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TaskStatus {
    /// The task has not started.
    Pending,
    /// The task is running.
    InProgress,
    /// The task finished successfully.
    Complete,
    /// The task failed.
    Error,
}
impl TaskStatus {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Complete => "complete",
            Self::Error => "error",
        }
    }
}
impl FromWire for TaskStatus {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("pending") => Ok(Self::Pending),
            Some("in_progress") => Ok(Self::InProgress),
            Some("complete") => Ok(Self::Complete),
            Some("error") => Ok(Self::Error),
            _ => Err(wire::mismatch(path, "TaskStatus")),
        }
    }
}
impl Serialize for TaskStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for TaskStatus {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "TaskStatus")
    }
}
impl<'de> Deserialize<'de> for TaskStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for TaskStatus {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// Visual emphasis for buttons and confirmation dialogs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ButtonStyle {
    /// Green emphasis for the affirmative action.
    Primary,
    /// Red emphasis for destructive actions.
    Danger,
}
impl ButtonStyle {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Danger => "danger",
        }
    }
}
impl FromWire for ButtonStyle {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("primary") => Ok(Self::Primary),
            Some("danger") => Ok(Self::Danger),
            _ => Err(wire::mismatch(path, "ButtonStyle")),
        }
    }
}
impl Serialize for ButtonStyle {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for ButtonStyle {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ButtonStyle")
    }
}
impl<'de> Deserialize<'de> for ButtonStyle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for ButtonStyle {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// Icons supported by icon buttons.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum IconButtonIcon {
    /// A trash can, for delete actions.
    Trash,
}
impl IconButtonIcon {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trash => "trash",
        }
    }
}
impl FromWire for IconButtonIcon {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("trash") => Ok(Self::Trash),
            _ => Err(wire::mismatch(path, "IconButtonIcon")),
        }
    }
}
impl Serialize for IconButtonIcon {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for IconButtonIcon {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "IconButtonIcon")
    }
}
impl<'de> Deserialize<'de> for IconButtonIcon {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for IconButtonIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for IconButtonIcon {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// Horizontal alignment for table columns.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ColumnAlign {
    /// Align cell content to the left.
    Left,
    /// Center cell content.
    Center,
    /// Align cell content to the right.
    Right,
}
impl ColumnAlign {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}
impl FromWire for ColumnAlign {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("left") => Ok(Self::Left),
            Some("center") => Ok(Self::Center),
            Some("right") => Ok(Self::Right),
            _ => Err(wire::mismatch(path, "ColumnAlign")),
        }
    }
}
impl Serialize for ColumnAlign {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for ColumnAlign {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ColumnAlign")
    }
}
impl<'de> Deserialize<'de> for ColumnAlign {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for ColumnAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for ColumnAlign {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// Marker styles for rich text lists.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RichTextListStyle {
    /// An unordered, bulleted list.
    Bullet,
    /// A numbered list.
    Ordered,
}
impl RichTextListStyle {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bullet => "bullet",
            Self::Ordered => "ordered",
        }
    }
}
impl FromWire for RichTextListStyle {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("bullet") => Ok(Self::Bullet),
            Some("ordered") => Ok(Self::Ordered),
            _ => Err(wire::mismatch(path, "RichTextListStyle")),
        }
    }
}
impl Serialize for RichTextListStyle {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for RichTextListStyle {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextListStyle")
    }
}
impl<'de> Deserialize<'de> for RichTextListStyle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for RichTextListStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for RichTextListStyle {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// Visibility of a slash-command or interaction response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResponseType {
    /// Visible to everyone in the channel.
    InChannel,
    /// Visible only to the user who triggered the response.
    Ephemeral,
}
impl ResponseType {
    /// Returns the Slack wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InChannel => "in_channel",
            Self::Ephemeral => "ephemeral",
        }
    }
}
impl FromWire for ResponseType {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.as_str() {
            Some("in_channel") => Ok(Self::InChannel),
            Some("ephemeral") => Ok(Self::Ephemeral),
            _ => Err(wire::mismatch(path, "ResponseType")),
        }
    }
}
impl Serialize for ResponseType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl TryFrom<Value> for ResponseType {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ResponseType")
    }
}
impl<'de> Deserialize<'de> for ResponseType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl std::str::FromStr for ResponseType {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Value::String(s.into()))
    }
}
/// A validated Slack layout block accepted directly by a JSON transport.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Block {
    /// A titled group of blocks that can optionally collapse.
    Container(Box<ContainerBlock>),
    /// A sortable, paginated table of raw text, numbers, and rich text.
    DataTable(Box<DataTableBlock>),
    /// A horizontal rule that separates blocks.
    Divider(Box<DividerBlock>),
    /// Large, bold plain text that introduces a group of blocks.
    Header(Box<HeaderBlock>),
    /// A titled sequence of task cards.
    Plan(Box<PlanBlock>),
    /// Formatted text built from sections, lists, preformatted blocks, and quotes.
    RichText(Box<RichTextBlock>),
    /// Text, a grid of fields, or both, with an optional accessory element.
    Section(Box<SectionBlock>),
    /// A simple table of raw text and rich text cells.
    Table(Box<TableBlock>),
    /// One task with its status, details, output, and sources.
    TaskCard(Box<TaskCardBlock>),
}
impl Serialize for Block {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Container(v) => v.serialize(s),
            Self::DataTable(v) => v.serialize(s),
            Self::Divider(v) => v.serialize(s),
            Self::Header(v) => v.serialize(s),
            Self::Plan(v) => v.serialize(s),
            Self::RichText(v) => v.serialize(s),
            Self::Section(v) => v.serialize(s),
            Self::Table(v) => v.serialize(s),
            Self::TaskCard(v) => v.serialize(s),
        }
    }
}
impl FromWire for Block {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("container") => ContainerBlock::from_wire(value, path).map(Into::into),
            Some("data_table") => DataTableBlock::from_wire(value, path).map(Into::into),
            Some("divider") => DividerBlock::from_wire(value, path).map(Into::into),
            Some("header") => HeaderBlock::from_wire(value, path).map(Into::into),
            Some("plan") => PlanBlock::from_wire(value, path).map(Into::into),
            Some("rich_text") => RichTextBlock::from_wire(value, path).map(Into::into),
            Some("section") => SectionBlock::from_wire(value, path).map(Into::into),
            Some("table") => TableBlock::from_wire(value, path).map(Into::into),
            Some("task_card") => TaskCardBlock::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a Block discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for Block {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Block")
    }
}
impl<'de> Deserialize<'de> for Block {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<ContainerBlock> for Block {
    fn from(value: ContainerBlock) -> Self {
        Self::Container(Box::new(value))
    }
}
impl From<DataTableBlock> for Block {
    fn from(value: DataTableBlock) -> Self {
        Self::DataTable(Box::new(value))
    }
}
impl From<DividerBlock> for Block {
    fn from(value: DividerBlock) -> Self {
        Self::Divider(Box::new(value))
    }
}
impl From<HeaderBlock> for Block {
    fn from(value: HeaderBlock) -> Self {
        Self::Header(Box::new(value))
    }
}
impl From<PlanBlock> for Block {
    fn from(value: PlanBlock) -> Self {
        Self::Plan(Box::new(value))
    }
}
impl From<RichTextBlock> for Block {
    fn from(value: RichTextBlock) -> Self {
        Self::RichText(Box::new(value))
    }
}
impl From<SectionBlock> for Block {
    fn from(value: SectionBlock) -> Self {
        Self::Section(Box::new(value))
    }
}
impl From<TableBlock> for Block {
    fn from(value: TableBlock) -> Self {
        Self::Table(Box::new(value))
    }
}
impl From<TaskCardBlock> for Block {
    fn from(value: TaskCardBlock) -> Self {
        Self::TaskCard(Box::new(value))
    }
}
/// A value that can appear in a context block: an image element or a text object.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ContextElement {
    /// An image displayed inside a section, context, or card.
    ImageElement(ImageElement),
    /// A mrkdwn text composition object.
    MarkdownText(MarkdownText),
    /// A plain-text composition object.
    PlainText(PlainText),
}
impl Serialize for ContextElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ImageElement(v) => v.serialize(s),
            Self::MarkdownText(v) => v.serialize(s),
            Self::PlainText(v) => v.serialize(s),
        }
    }
}
impl FromWire for ContextElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("image") => ImageElement::from_wire(value, path).map(Into::into),
            Some("mrkdwn") => MarkdownText::from_wire(value, path).map(Into::into),
            Some("plain_text") => PlainText::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a ContextElement discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for ContextElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ContextElement")
    }
}
impl<'de> Deserialize<'de> for ContextElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<ImageElement> for ContextElement {
    fn from(value: ImageElement) -> Self {
        Self::ImageElement(value)
    }
}
impl From<MarkdownText> for ContextElement {
    fn from(value: MarkdownText) -> Self {
        Self::MarkdownText(value)
    }
}
impl From<PlainText> for ContextElement {
    fn from(value: PlainText) -> Self {
        Self::PlainText(value)
    }
}
/// A value that can appear as a table block cell: raw text or a rich text block.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TableCell {
    /// Formatted text built from sections, lists, preformatted blocks, and quotes.
    RichText(RichTextBlock),
    /// An unformatted table cell.
    RawText(RawText),
}
impl Serialize for TableCell {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RichText(v) => v.serialize(s),
            Self::RawText(v) => v.serialize(s),
        }
    }
}
impl FromWire for TableCell {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("rich_text") => RichTextBlock::from_wire(value, path).map(Into::into),
            Some("raw_text") => RawText::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a TableCell discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for TableCell {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "TableCell")
    }
}
impl<'de> Deserialize<'de> for TableCell {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<RichTextBlock> for TableCell {
    fn from(value: RichTextBlock) -> Self {
        Self::RichText(value)
    }
}
impl From<RawText> for TableCell {
    fn from(value: RawText) -> Self {
        Self::RawText(value)
    }
}
/// A value that can appear as a data table cell: raw text, a raw number, or a rich text block.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum DataTableCell {
    /// Formatted text built from sections, lists, preformatted blocks, and quotes.
    RichText(RichTextBlock),
    /// A numeric data table cell with display text.
    RawNumber(RawNumber),
    /// An unformatted table cell.
    RawText(RawText),
}
impl Serialize for DataTableCell {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RichText(v) => v.serialize(s),
            Self::RawNumber(v) => v.serialize(s),
            Self::RawText(v) => v.serialize(s),
        }
    }
}
impl FromWire for DataTableCell {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("rich_text") => RichTextBlock::from_wire(value, path).map(Into::into),
            Some("raw_number") => RawNumber::from_wire(value, path).map(Into::into),
            Some("raw_text") => RawText::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a DataTableCell discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for DataTableCell {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DataTableCell")
    }
}
impl<'de> Deserialize<'de> for DataTableCell {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<RichTextBlock> for DataTableCell {
    fn from(value: RichTextBlock) -> Self {
        Self::RichText(value)
    }
}
impl From<RawNumber> for DataTableCell {
    fn from(value: RawNumber) -> Self {
        Self::RawNumber(value)
    }
}
impl From<RawText> for DataTableCell {
    fn from(value: RawText) -> Self {
        Self::RawText(value)
    }
}
/// A validated interactive, visual, or input element nested inside a Slack block.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Element {
    /// An interactive button that sends a payload or opens a URL.
    Button(Box<ButtonElement>),
    /// An image displayed inside a section, context, or card.
    Image(Box<ImageElement>),
}
impl Serialize for Element {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Button(v) => v.serialize(s),
            Self::Image(v) => v.serialize(s),
        }
    }
}
impl FromWire for Element {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("button") => ButtonElement::from_wire(value, path).map(Into::into),
            Some("image") => ImageElement::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a Element discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for Element {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Element")
    }
}
impl<'de> Deserialize<'de> for Element {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<ButtonElement> for Element {
    fn from(value: ButtonElement) -> Self {
        Self::Button(Box::new(value))
    }
}
impl From<ImageElement> for Element {
    fn from(value: ImageElement) -> Self {
        Self::Image(Box::new(value))
    }
}
/// A Slack text composition object: plain text or mrkdwn.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Text {
    /// A mrkdwn text composition object.
    Markdown(MarkdownText),
    /// A plain-text composition object.
    Plain(PlainText),
}
impl Serialize for Text {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Markdown(v) => v.serialize(s),
            Self::Plain(v) => v.serialize(s),
        }
    }
}
impl FromWire for Text {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("mrkdwn") => MarkdownText::from_wire(value, path).map(Into::into),
            Some("plain_text") => PlainText::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a Text discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for Text {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Text")
    }
}
impl<'de> Deserialize<'de> for Text {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<MarkdownText> for Text {
    fn from(value: MarkdownText) -> Self {
        Self::Markdown(value)
    }
}
impl From<PlainText> for Text {
    fn from(value: PlainText) -> Self {
        Self::Plain(value)
    }
}
/// A top-level element of a rich text block: a section, list, preformatted block, or quote.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum RichTextBlockElement {
    /// A bulleted or numbered list inside rich text.
    List(RichTextList),
    /// A paragraph of inline rich text elements.
    Section(RichTextSection),
}
impl Serialize for RichTextBlockElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::List(v) => v.serialize(s),
            Self::Section(v) => v.serialize(s),
        }
    }
}
impl FromWire for RichTextBlockElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("rich_text_list") => RichTextList::from_wire(value, path).map(Into::into),
            Some("rich_text_section") => RichTextSection::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a RichTextBlockElement discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for RichTextBlockElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextBlockElement")
    }
}
impl<'de> Deserialize<'de> for RichTextBlockElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<RichTextList> for RichTextBlockElement {
    fn from(value: RichTextList) -> Self {
        Self::List(value)
    }
}
impl From<RichTextSection> for RichTextBlockElement {
    fn from(value: RichTextSection) -> Self {
        Self::Section(value)
    }
}
/// An inline rich text element: text, a link, an emoji, or a channel, user, or user group mention.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum RichTextSectionElement {
    /// A run of optionally styled text inside rich text.
    Text(RichTextText),
}
impl Serialize for RichTextSectionElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text(v) => v.serialize(s),
        }
    }
}
impl FromWire for RichTextSectionElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("text") => RichTextText::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a RichTextSectionElement discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for RichTextSectionElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextSectionElement")
    }
}
impl<'de> Deserialize<'de> for RichTextSectionElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<RichTextText> for RichTextSectionElement {
    fn from(value: RichTextText) -> Self {
        Self::Text(value)
    }
}
/// A titled group of blocks that can optionally collapse.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/container-block).
/// Provide a title or a rich text title.
#[derive(Clone, Debug, PartialEq)]
pub struct ContainerBlock {
    child_blocks: Vec<Block>,
    title: Option<PlainText>,
    rich_text_title: Option<RichTextBlock>,
    subtitle: Option<Text>,
    width: Option<ContainerWidth>,
    icon: Option<ImageElement>,
    is_collapsible: Option<bool>,
    default_collapsed: Option<bool>,
    has_header_divider: Option<bool>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ContainerBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ContainerBlockBuilder {
    child_blocks: Option<Vec<Block>>,
    title: Option<PlainTextInput>,
    rich_text_title: Option<RichTextBlock>,
    subtitle: Option<TextInput>,
    width: Option<ContainerWidth>,
    icon: Option<ImageElement>,
    is_collapsible: Option<bool>,
    default_collapsed: Option<bool>,
    has_header_divider: Option<bool>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for ContainerBlockBuilder {
    fn default() -> Self {
        Self {
            child_blocks: None,
            title: None,
            rich_text_title: None,
            subtitle: None,
            width: None,
            icon: None,
            is_collapsible: None,
            default_collapsed: None,
            has_header_divider: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl ContainerBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ContainerBlockBuilder {
        ContainerBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ContainerBlockBuilder {
        ContainerBlockBuilder {
            child_blocks: Some(self.child_blocks),
            title: self.title.map(Into::into),
            rich_text_title: self.rich_text_title,
            subtitle: self.subtitle.map(Into::into),
            width: self.width,
            icon: self.icon,
            is_collapsible: self.is_collapsible,
            default_collapsed: self.default_collapsed,
            has_header_divider: self.has_header_divider,
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `child_blocks`. Adds the blocks grouped inside the container, in display order.
    pub fn child_blocks(&self) -> &[Block] {
        &self.child_blocks
    }
    /// Borrows or copies `title`. Sets the container title. Provide either a title or a rich text title.
    pub fn title(&self) -> Option<&PlainText> {
        self.title.as_ref()
    }
    /// Borrows or copies `rich_text_title`. Sets a rich text block used as the title in place of a plain-text title.
    pub fn rich_text_title(&self) -> Option<&RichTextBlock> {
        self.rich_text_title.as_ref()
    }
    /// Borrows or copies `subtitle`. Sets the supporting text shown below the title.
    pub fn subtitle(&self) -> Option<&Text> {
        self.subtitle.as_ref()
    }
    /// Borrows or copies `width`. Sets the horizontal width of the container.
    pub fn width(&self) -> Option<ContainerWidth> {
        self.width
    }
    /// Borrows or copies `icon`. Sets the image displayed beside the title.
    pub fn icon(&self) -> Option<&ImageElement> {
        self.icon.as_ref()
    }
    /// Borrows or copies `is_collapsible`. Sets whether users can collapse and expand the container.
    pub fn is_collapsible(&self) -> Option<bool> {
        self.is_collapsible
    }
    /// Borrows or copies `default_collapsed`. Sets whether a collapsible container starts collapsed. Requires the container to be collapsible.
    pub fn default_collapsed(&self) -> Option<bool> {
        self.default_collapsed
    }
    /// Borrows or copies `has_header_divider`. Sets whether Slack draws a divider below the container header. Only valid for non-collapsible containers.
    pub fn has_header_divider(&self) -> Option<bool> {
        self.has_header_divider
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "container")?;
        }
        map.serialize_entry("child_blocks", &self.child_blocks)?;
        if let Some(value) = &self.title {
            map.serialize_entry("title", value)?;
        }
        if let Some(value) = &self.rich_text_title {
            map.serialize_entry("rich_text_title", value)?;
        }
        if let Some(value) = &self.subtitle {
            map.serialize_entry("subtitle", value)?;
        }
        if let Some(value) = &self.width {
            map.serialize_entry("width", value)?;
        }
        if let Some(value) = &self.icon {
            map.serialize_entry("icon", value)?;
        }
        if let Some(value) = &self.is_collapsible {
            map.serialize_entry("is_collapsible", value)?;
        }
        if let Some(value) = &self.default_collapsed {
            map.serialize_entry("default_collapsed", value)?;
        }
        if let Some(value) = &self.has_header_divider {
            map.serialize_entry("has_header_divider", value)?;
        }
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ContainerBlockBuilder {
    /// Adds the blocks grouped inside the container, in display order.
    /// Replaces the collection; order is preserved.
    pub fn child_blocks<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Block>,
    {
        self.child_blocks = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn child_block(mut self, value: impl Into<Block>) -> Self {
        self.child_blocks
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Sets the container title. Provide either a title or a rich text title.
    pub fn title(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Omits `title`, including any model default.
    pub fn clear_title(mut self) -> Self {
        self.title = None;
        self
    }
    /// Sets a rich text block used as the title in place of a plain-text title.
    pub fn rich_text_title(mut self, value: impl Into<RichTextBlock>) -> Self {
        self.rich_text_title = Some(value.into());
        self
    }
    /// Omits `rich_text_title`, including any model default.
    pub fn clear_rich_text_title(mut self) -> Self {
        self.rich_text_title = None;
        self
    }
    /// Sets the supporting text shown below the title.
    pub fn subtitle(mut self, value: impl Into<TextInput>) -> Self {
        self.subtitle = Some(value.into());
        self
    }
    /// Omits `subtitle`, including any model default.
    pub fn clear_subtitle(mut self) -> Self {
        self.subtitle = None;
        self
    }
    /// Sets the horizontal width of the container.
    pub fn width(mut self, value: ContainerWidth) -> Self {
        self.width = Some(value);
        self
    }
    /// Omits `width`, including any model default.
    pub fn clear_width(mut self) -> Self {
        self.width = None;
        self
    }
    /// Sets the image displayed beside the title.
    pub fn icon(mut self, value: impl Into<ImageElement>) -> Self {
        self.icon = Some(value.into());
        self
    }
    /// Omits `icon`, including any model default.
    pub fn clear_icon(mut self) -> Self {
        self.icon = None;
        self
    }
    /// Sets whether users can collapse and expand the container.
    pub fn is_collapsible(mut self, value: bool) -> Self {
        self.is_collapsible = Some(value);
        self
    }
    /// Omits `is_collapsible`, including any model default.
    pub fn clear_is_collapsible(mut self) -> Self {
        self.is_collapsible = None;
        self
    }
    /// Sets whether a collapsible container starts collapsed. Requires the container to be collapsible.
    pub fn default_collapsed(mut self, value: bool) -> Self {
        self.default_collapsed = Some(value);
        self
    }
    /// Omits `default_collapsed`, including any model default.
    pub fn clear_default_collapsed(mut self) -> Self {
        self.default_collapsed = None;
        self
    }
    /// Sets whether Slack draws a divider below the container header. Only valid for non-collapsible containers.
    pub fn has_header_divider(mut self, value: bool) -> Self {
        self.has_header_divider = Some(value);
        self
    }
    /// Omits `has_header_divider`, including any model default.
    pub fn clear_has_header_divider(mut self) -> Self {
        self.has_header_divider = None;
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<ContainerBlock, ValidationError> {
        let child_blocks = wire::required(self.child_blocks, "ContainerBlock.child_blocks")?;
        let title = self
            .title
            .map(|v| v.resolve().map_err(|e| e.at("ContainerBlock.title")))
            .transpose()?;
        let rich_text_title = self.rich_text_title;
        let subtitle = self
            .subtitle
            .map(|v| {
                v.resolve(false)
                    .map_err(|e| e.at("ContainerBlock.subtitle"))
            })
            .transpose()?;
        let width = self.width;
        let icon = self.icon;
        let is_collapsible = self.is_collapsible;
        let default_collapsed = self.default_collapsed;
        let has_header_divider = self.has_header_divider;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &[
                "child_blocks",
                "title",
                "rich_text_title",
                "subtitle",
                "width",
                "icon",
                "is_collapsible",
                "default_collapsed",
                "has_header_divider",
                "block_id",
            ],
            "ContainerBlock",
        )?;
        let value = ContainerBlock {
            child_blocks,
            title,
            rich_text_title,
            subtitle,
            width,
            icon,
            is_collapsible,
            default_collapsed,
            has_header_divider,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ContainerBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("child_blocks") {
            crate::rules::limits(v, "container.child_blocks", "ContainerBlock.child_blocks")?;
        }
        if let Some(v) = wire.get("title") {
            crate::rules::limits(v, "container.title", "ContainerBlock.title")?;
        }
        if let Some(v) = wire.get("subtitle") {
            crate::rules::limits(v, "container.subtitle", "ContainerBlock.subtitle")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "ContainerBlock.block_id")?;
        }
        crate::rules::validate("ContainerBlock", &wire, "ContainerBlock")?;
        Ok(value)
    }
}
impl Serialize for ContainerBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ContainerBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "container", path)?;
        let child_blocks = wire::field::<Vec<Block>>(&mut map, "child_blocks", path, false)?;
        crate::rules::coerce_text(map.get_mut("title"), "plain_text");
        let title = wire::field::<PlainText>(&mut map, "title", path, false)?;
        let rich_text_title =
            wire::field::<RichTextBlock>(&mut map, "rich_text_title", path, false)?;
        crate::rules::coerce_text(map.get_mut("subtitle"), "mrkdwn");
        let subtitle = wire::field::<Text>(&mut map, "subtitle", path, false)?;
        let width = wire::field::<ContainerWidth>(&mut map, "width", path, false)?;
        let icon = wire::field::<ImageElement>(&mut map, "icon", path, false)?;
        let is_collapsible = wire::field::<bool>(&mut map, "is_collapsible", path, false)?;
        let default_collapsed = wire::field::<bool>(&mut map, "default_collapsed", path, false)?;
        let has_header_divider = wire::field::<bool>(&mut map, "has_header_divider", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        ContainerBlockBuilder {
            child_blocks,
            title: title.map(Into::into),
            rich_text_title,
            subtitle: subtitle.map(Into::into),
            width,
            icon,
            is_collapsible,
            default_collapsed,
            has_header_divider,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ContainerBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ContainerBlock")
    }
}
impl<'de> Deserialize<'de> for ContainerBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A sortable, paginated table of raw text, numbers, and rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-table-block).
/// The first row is the header row and cannot contain rich text. Every row must have the same number of cells, and all cell text together is limited to 20,000 characters.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableBlock {
    rows: Vec<Vec<DataTableCell>>,
    caption: String,
    page_size: Option<i64>,
    row_header_column_index: Option<i64>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DataTableBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DataTableBlockBuilder {
    rows: Option<Vec<Vec<DataTableCell>>>,
    caption: Option<String>,
    page_size: Option<i64>,
    row_header_column_index: Option<i64>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for DataTableBlockBuilder {
    fn default() -> Self {
        Self {
            rows: None,
            caption: None,
            page_size: Some(5),
            row_header_column_index: Some(0),
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl DataTableBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DataTableBlockBuilder {
        DataTableBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DataTableBlockBuilder {
        DataTableBlockBuilder {
            rows: Some(self.rows),
            caption: Some(self.caption),
            page_size: self.page_size,
            row_header_column_index: self.row_header_column_index,
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `rows`. Adds complete table rows in display order. Every row must have the same number of cells.
    pub fn rows(&self) -> &[Vec<DataTableCell>] {
        &self.rows
    }
    /// Borrows or copies `caption`. Sets the caption that describes the table's contents.
    pub fn caption(&self) -> &str {
        &self.caption
    }
    /// Borrows or copies `page_size`. Sets how many rows Slack shows per page.
    pub fn page_size(&self) -> Option<i64> {
        self.page_size
    }
    /// Borrows or copies `row_header_column_index`. Sets the zero-based index of the column whose cells act as row headers.
    pub fn row_header_column_index(&self) -> Option<i64> {
        self.row_header_column_index
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "data_table")?;
        }
        map.serialize_entry("rows", &self.rows)?;
        map.serialize_entry("caption", &self.caption)?;
        if let Some(value) = &self.page_size {
            map.serialize_entry("page_size", value)?;
        }
        if let Some(value) = &self.row_header_column_index {
            map.serialize_entry("row_header_column_index", value)?;
        }
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DataTableBlockBuilder {
    /// Adds complete table rows in display order. Every row must have the same number of cells.
    /// Replaces all rows, preserving row and cell order.
    pub fn rows<I, R, T>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<DataTableCell>,
    {
        self.rows = Some(
            rows.into_iter()
                .map(|r| r.into_iter().map(Into::into).collect())
                .collect(),
        );
        self
    }
    /// Appends a complete row.
    pub fn row<I, T>(mut self, row: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<DataTableCell>,
    {
        self.rows
            .get_or_insert_with(Vec::new)
            .push(row.into_iter().map(Into::into).collect());
        self
    }
    /// Sets the caption that describes the table's contents.
    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }
    /// Sets how many rows Slack shows per page.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Omits `page_size`, including any model default.
    pub fn clear_page_size(mut self) -> Self {
        self.page_size = None;
        self
    }
    /// Sets the zero-based index of the column whose cells act as row headers.
    pub fn row_header_column_index(mut self, value: i64) -> Self {
        self.row_header_column_index = Some(value);
        self
    }
    /// Omits `row_header_column_index`, including any model default.
    pub fn clear_row_header_column_index(mut self) -> Self {
        self.row_header_column_index = None;
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<DataTableBlock, ValidationError> {
        let rows = wire::required(self.rows, "DataTableBlock.rows")?;
        let caption = wire::required(self.caption, "DataTableBlock.caption")?;
        let page_size = self.page_size;
        let row_header_column_index = self.row_header_column_index;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &[
                "rows",
                "caption",
                "page_size",
                "row_header_column_index",
                "block_id",
            ],
            "DataTableBlock",
        )?;
        let value = DataTableBlock {
            rows,
            caption,
            page_size,
            row_header_column_index,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "DataTableBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("rows") {
            crate::rules::limits(v, "data_table.rows", "DataTableBlock.rows")?;
        }
        if let Some(v) = wire.get("page_size") {
            crate::rules::limits(v, "data_table.page_size", "DataTableBlock.page_size")?;
        }
        if let Some(v) = wire.get("row_header_column_index") {
            crate::rules::limits(
                v,
                "data_table.row_header_column_index",
                "DataTableBlock.row_header_column_index",
            )?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "DataTableBlock.block_id")?;
        }
        crate::rules::validate("DataTableBlock", &wire, "DataTableBlock")?;
        Ok(value)
    }
}
impl Serialize for DataTableBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DataTableBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "data_table", path)?;
        let rows = wire::field::<Vec<Vec<DataTableCell>>>(&mut map, "rows", path, false)?;
        let caption = wire::field::<String>(&mut map, "caption", path, false)?;
        let page_size = wire::field::<i64>(&mut map, "page_size", path, false)?;
        let row_header_column_index =
            wire::field::<i64>(&mut map, "row_header_column_index", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        DataTableBlockBuilder {
            rows,
            caption,
            page_size,
            row_header_column_index,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DataTableBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DataTableBlock")
    }
}
impl<'de> Deserialize<'de> for DataTableBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A horizontal rule that separates blocks.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/divider-block).

#[derive(Clone, Debug, PartialEq)]
pub struct DividerBlock {
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DividerBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DividerBlockBuilder {
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for DividerBlockBuilder {
    fn default() -> Self {
        Self {
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl DividerBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DividerBlockBuilder {
        DividerBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DividerBlockBuilder {
        DividerBlockBuilder {
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "divider")?;
        }
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DividerBlockBuilder {
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<DividerBlock, ValidationError> {
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["block_id"], "DividerBlock")?;
        let value = DividerBlock {
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "DividerBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "DividerBlock.block_id")?;
        }
        crate::rules::validate("DividerBlock", &wire, "DividerBlock")?;
        Ok(value)
    }
}
impl Serialize for DividerBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DividerBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "divider", path)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        DividerBlockBuilder {
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DividerBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DividerBlock")
    }
}
impl<'de> Deserialize<'de> for DividerBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Large, bold plain text that introduces a group of blocks.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/header-block).

#[derive(Clone, Debug, PartialEq)]
pub struct HeaderBlock {
    text: PlainText,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`HeaderBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct HeaderBlockBuilder {
    text: Option<PlainTextInput>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for HeaderBlockBuilder {
    fn default() -> Self {
        Self {
            text: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl HeaderBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> HeaderBlockBuilder {
        HeaderBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> HeaderBlockBuilder {
        HeaderBlockBuilder {
            text: Some(self.text.into()),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the header text, shown in a larger bold font.
    pub fn text(&self) -> &PlainText {
        &self.text
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "header")?;
        }
        map.serialize_entry("text", &self.text)?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl HeaderBlockBuilder {
    /// Sets the header text, shown in a larger bold font.
    pub fn text(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<HeaderBlock, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve().map_err(|e| e.at("HeaderBlock.text")))
                .transpose()?,
            "HeaderBlock.text",
        )?;
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["text", "block_id"], "HeaderBlock")?;
        let value = HeaderBlock {
            text,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "HeaderBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "header.text", "HeaderBlock.text")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "HeaderBlock.block_id")?;
        }
        crate::rules::validate("HeaderBlock", &wire, "HeaderBlock")?;
        Ok(value)
    }
}
impl Serialize for HeaderBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for HeaderBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "header", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "plain_text");
        let text = wire::field::<PlainText>(&mut map, "text", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        HeaderBlockBuilder {
            text: text.map(Into::into),
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for HeaderBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "HeaderBlock")
    }
}
impl<'de> Deserialize<'de> for HeaderBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A titled sequence of task cards.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/plan-block).
/// Task IDs must be unique within a plan.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanBlock {
    title: String,
    tasks: Vec<TaskCardBlock>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`PlanBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct PlanBlockBuilder {
    title: Option<String>,
    tasks: Option<Vec<TaskCardBlock>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for PlanBlockBuilder {
    fn default() -> Self {
        Self {
            title: None,
            tasks: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl PlanBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> PlanBlockBuilder {
        PlanBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> PlanBlockBuilder {
        PlanBlockBuilder {
            title: Some(self.title),
            tasks: Some(self.tasks),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `title`. Sets the plan title.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Borrows or copies `tasks`. Adds task cards in display order.
    pub fn tasks(&self) -> &[TaskCardBlock] {
        &self.tasks
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "plan")?;
        }
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("tasks", &crate::rules::PlanTasks(&self.tasks))?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PlanBlockBuilder {
    /// Sets the plan title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Adds task cards in display order.
    /// Replaces the collection; order is preserved.
    pub fn tasks<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TaskCardBlock>,
    {
        self.tasks = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn task(mut self, value: impl Into<TaskCardBlock>) -> Self {
        self.tasks.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<PlanBlock, ValidationError> {
        let title = wire::required(self.title, "PlanBlock.title")?;
        let tasks = wire::required(self.tasks, "PlanBlock.tasks")?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["title", "tasks", "block_id"],
            "PlanBlock",
        )?;
        let value = PlanBlock {
            title,
            tasks,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "PlanBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("tasks") {
            crate::rules::limits(v, "plan.tasks", "PlanBlock.tasks")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "PlanBlock.block_id")?;
        }
        crate::rules::validate("PlanBlock", &wire, "PlanBlock")?;
        Ok(value)
    }
}
impl Serialize for PlanBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for PlanBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "plan", path)?;
        let title = wire::field::<String>(&mut map, "title", path, false)?;
        crate::rules::restore_tasks(map.get_mut("tasks"));
        let tasks = wire::field::<Vec<TaskCardBlock>>(&mut map, "tasks", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        PlanBlockBuilder {
            title,
            tasks,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for PlanBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "PlanBlock")
    }
}
impl<'de> Deserialize<'de> for PlanBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Formatted text built from sections, lists, preformatted blocks, and quotes.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextBlock {
    elements: Vec<RichTextBlockElement>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextBlockBuilder {
    elements: Option<Vec<RichTextBlockElement>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for RichTextBlockBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextBlockBuilder {
        RichTextBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextBlockBuilder {
        RichTextBlockBuilder {
            elements: Some(self.elements),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds sections, lists, preformatted blocks, and quotes in display order.
    pub fn elements(&self) -> &[RichTextBlockElement] {
        &self.elements
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "rich_text")?;
        }
        map.serialize_entry("elements", &self.elements)?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextBlockBuilder {
    /// Adds sections, lists, preformatted blocks, and quotes in display order.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<RichTextBlockElement>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<RichTextBlockElement>) -> Self {
        self.elements
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<RichTextBlock, ValidationError> {
        let elements = wire::required(self.elements, "RichTextBlock.elements")?;
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["elements", "block_id"], "RichTextBlock")?;
        let value = RichTextBlock {
            elements,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "RichTextBlock.block_id")?;
        }
        crate::rules::validate("RichTextBlock", &wire, "RichTextBlock")?;
        Ok(value)
    }
}
impl Serialize for RichTextBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "rich_text", path)?;
        let elements = wire::field::<Vec<RichTextBlockElement>>(&mut map, "elements", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        RichTextBlockBuilder {
            elements,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextBlock")
    }
}
impl<'de> Deserialize<'de> for RichTextBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Text, a grid of fields, or both, with an optional accessory element.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/section-block).
/// Provide text, fields, or both.
#[derive(Clone, Debug, PartialEq)]
pub struct SectionBlock {
    text: Option<Text>,
    fields: Option<Vec<Text>>,
    accessory: Option<Element>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`SectionBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct SectionBlockBuilder {
    text: Option<TextInput>,
    fields: Option<Vec<TextInput>>,
    accessory: Option<Element>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for SectionBlockBuilder {
    fn default() -> Self {
        Self {
            text: None,
            fields: None,
            accessory: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl SectionBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> SectionBlockBuilder {
        SectionBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> SectionBlockBuilder {
        SectionBlockBuilder {
            text: self.text.map(Into::into),
            fields: self.fields.map(|v| v.into_iter().map(Into::into).collect()),
            accessory: self.accessory,
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the main text of the section.
    pub fn text(&self) -> Option<&Text> {
        self.text.as_ref()
    }
    /// Borrows or copies `fields`. Adds section fields, shown in a two-column grid, in display order.
    pub fn fields(&self) -> Option<&[Text]> {
        self.fields.as_deref()
    }
    /// Borrows or copies `accessory`. Sets the element displayed beside the section text.
    pub fn accessory(&self) -> Option<&Element> {
        self.accessory.as_ref()
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "section")?;
        }
        if let Some(value) = &self.text {
            map.serialize_entry("text", value)?;
        }
        if let Some(value) = &self.fields {
            map.serialize_entry("fields", value)?;
        }
        if let Some(value) = &self.accessory {
            map.serialize_entry("accessory", value)?;
        }
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SectionBlockBuilder {
    /// Sets the main text of the section.
    pub fn text(mut self, value: impl Into<TextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Omits `text`, including any model default.
    pub fn clear_text(mut self) -> Self {
        self.text = None;
        self
    }
    /// Adds section fields, shown in a two-column grid, in display order.
    /// Replaces the collection; order is preserved.
    pub fn fields<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TextInput>,
    {
        self.fields = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn field(mut self, value: impl Into<TextInput>) -> Self {
        self.fields.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `fields`, including any model default.
    pub fn clear_fields(mut self) -> Self {
        self.fields = None;
        self
    }
    /// Sets the element displayed beside the section text.
    pub fn accessory(mut self, value: impl Into<Element>) -> Self {
        self.accessory = Some(value.into());
        self
    }
    /// Omits `accessory`, including any model default.
    pub fn clear_accessory(mut self) -> Self {
        self.accessory = None;
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<SectionBlock, ValidationError> {
        let text = self
            .text
            .map(|v| v.resolve(false).map_err(|e| e.at("SectionBlock.text")))
            .transpose()?;
        let fields = self
            .fields
            .map(|vs| {
                vs.into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        v.resolve(false)
                            .map_err(|e| e.at(&format!("SectionBlock.fields[{i}]")))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let accessory = self.accessory;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["text", "fields", "accessory", "block_id"],
            "SectionBlock",
        )?;
        let value = SectionBlock {
            text,
            fields,
            accessory,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "SectionBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "section.text", "SectionBlock.text")?;
        }
        if let Some(v) = wire.get("fields") {
            crate::rules::limits(v, "section.fields", "SectionBlock.fields")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "SectionBlock.block_id")?;
        }
        crate::rules::validate("SectionBlock", &wire, "SectionBlock")?;
        Ok(value)
    }
}
impl Serialize for SectionBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for SectionBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "section", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "mrkdwn");
        let text = wire::field::<Text>(&mut map, "text", path, false)?;
        if let Some(Value::Array(vs)) = map.get_mut("fields") {
            for v in vs {
                crate::rules::coerce_text(Some(v), "mrkdwn");
            }
        }
        let fields = wire::field::<Vec<Text>>(&mut map, "fields", path, false)?;
        let accessory = wire::field::<Element>(&mut map, "accessory", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        SectionBlockBuilder {
            text: text.map(Into::into),
            fields: fields.map(|vs| vs.into_iter().map(Into::into).collect()),
            accessory,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for SectionBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "SectionBlock")
    }
}
impl<'de> Deserialize<'de> for SectionBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A simple table of raw text and rich text cells.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/table-block).

#[derive(Clone, Debug, PartialEq)]
pub struct TableBlock {
    rows: Vec<Vec<TableCell>>,
    column_settings: Option<Vec<ColumnSettings>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`TableBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct TableBlockBuilder {
    rows: Option<Vec<Vec<TableCell>>>,
    column_settings: Option<Vec<ColumnSettings>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for TableBlockBuilder {
    fn default() -> Self {
        Self {
            rows: None,
            column_settings: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl TableBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> TableBlockBuilder {
        TableBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> TableBlockBuilder {
        TableBlockBuilder {
            rows: Some(self.rows),
            column_settings: self.column_settings,
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `rows`. Adds complete table rows in display order.
    pub fn rows(&self) -> &[Vec<TableCell>] {
        &self.rows
    }
    /// Borrows or copies `column_settings`. Adds per-column settings in column order. Provide one entry for every column.
    pub fn column_settings(&self) -> Option<&[ColumnSettings]> {
        self.column_settings.as_deref()
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "table")?;
        }
        map.serialize_entry("rows", &self.rows)?;
        if let Some(value) = &self.column_settings {
            map.serialize_entry("column_settings", value)?;
        }
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TableBlockBuilder {
    /// Adds complete table rows in display order.
    /// Replaces all rows, preserving row and cell order.
    pub fn rows<I, R, T>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        self.rows = Some(
            rows.into_iter()
                .map(|r| r.into_iter().map(Into::into).collect())
                .collect(),
        );
        self
    }
    /// Appends a complete row.
    pub fn row<I, T>(mut self, row: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TableCell>,
    {
        self.rows
            .get_or_insert_with(Vec::new)
            .push(row.into_iter().map(Into::into).collect());
        self
    }
    /// Adds per-column settings in column order. Provide one entry for every column.
    /// Replaces the collection; order is preserved.
    pub fn column_settings<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ColumnSettings>,
    {
        self.column_settings = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn column_setting(mut self, value: impl Into<ColumnSettings>) -> Self {
        self.column_settings
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `column_settings`, including any model default.
    pub fn clear_column_settings(mut self) -> Self {
        self.column_settings = None;
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<TableBlock, ValidationError> {
        let rows = wire::required(self.rows, "TableBlock.rows")?;
        let column_settings = self.column_settings;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["rows", "column_settings", "block_id"],
            "TableBlock",
        )?;
        let value = TableBlock {
            rows,
            column_settings,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "TableBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("rows") {
            crate::rules::limits(v, "table.rows", "TableBlock.rows")?;
        }
        if let Some(v) = wire.get("column_settings") {
            crate::rules::limits(v, "table.column_settings", "TableBlock.column_settings")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "TableBlock.block_id")?;
        }
        crate::rules::validate("TableBlock", &wire, "TableBlock")?;
        Ok(value)
    }
}
impl Serialize for TableBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for TableBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "table", path)?;
        let rows = wire::field::<Vec<Vec<TableCell>>>(&mut map, "rows", path, false)?;
        let column_settings =
            wire::field::<Vec<ColumnSettings>>(&mut map, "column_settings", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        TableBlockBuilder {
            rows,
            column_settings,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for TableBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "TableBlock")
    }
}
impl<'de> Deserialize<'de> for TableBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// One task with its status, details, output, and sources.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/task-card-block).
/// A standalone task card's status cannot be pending; pending is only valid for plan tasks.
#[derive(Clone, Debug, PartialEq)]
pub struct TaskCardBlock {
    task_id: String,
    title: String,
    details: Option<RichTextBlock>,
    output: Option<RichTextBlock>,
    sources: Option<Vec<UrlSource>>,
    status: TaskStatus,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`TaskCardBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct TaskCardBlockBuilder {
    task_id: Option<String>,
    title: Option<String>,
    details: Option<RichTextBlock>,
    output: Option<RichTextBlock>,
    sources: Option<Vec<UrlSource>>,
    status: Option<TaskStatus>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for TaskCardBlockBuilder {
    fn default() -> Self {
        Self {
            task_id: None,
            title: None,
            details: None,
            output: None,
            sources: None,
            status: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl TaskCardBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> TaskCardBlockBuilder {
        TaskCardBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> TaskCardBlockBuilder {
        TaskCardBlockBuilder {
            task_id: Some(self.task_id),
            title: Some(self.title),
            details: self.details,
            output: self.output,
            sources: self.sources,
            status: Some(self.status),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `task_id`. Sets the identifier of the task.
    pub fn task_id(&self) -> &str {
        &self.task_id
    }
    /// Borrows or copies `title`. Sets the task title.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Borrows or copies `details`. Sets a rich text description of the task.
    pub fn details(&self) -> Option<&RichTextBlock> {
        self.details.as_ref()
    }
    /// Borrows or copies `output`. Sets rich text describing the task's output.
    pub fn output(&self) -> Option<&RichTextBlock> {
        self.output.as_ref()
    }
    /// Borrows or copies `sources`. Adds links to the sources the task used, in display order.
    pub fn sources(&self) -> Option<&[UrlSource]> {
        self.sources.as_deref()
    }
    /// Borrows or copies `status`. Sets the task's current state.
    pub fn status(&self) -> TaskStatus {
        self.status
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "task_card")?;
        }
        map.serialize_entry("task_id", &self.task_id)?;
        map.serialize_entry("title", &self.title)?;
        if let Some(value) = &self.details {
            map.serialize_entry("details", value)?;
        }
        if let Some(value) = &self.output {
            map.serialize_entry("output", value)?;
        }
        if let Some(value) = &self.sources {
            map.serialize_entry("sources", value)?;
        }
        map.serialize_entry("status", &self.status)?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TaskCardBlockBuilder {
    /// Sets the identifier of the task.
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }
    /// Sets the task title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Sets a rich text description of the task.
    pub fn details(mut self, value: impl Into<RichTextBlock>) -> Self {
        self.details = Some(value.into());
        self
    }
    /// Omits `details`, including any model default.
    pub fn clear_details(mut self) -> Self {
        self.details = None;
        self
    }
    /// Sets rich text describing the task's output.
    pub fn output(mut self, value: impl Into<RichTextBlock>) -> Self {
        self.output = Some(value.into());
        self
    }
    /// Omits `output`, including any model default.
    pub fn clear_output(mut self) -> Self {
        self.output = None;
        self
    }
    /// Adds links to the sources the task used, in display order.
    /// Replaces the collection; order is preserved.
    pub fn sources<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<UrlSource>,
    {
        self.sources = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn source(mut self, value: impl Into<UrlSource>) -> Self {
        self.sources.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `sources`, including any model default.
    pub fn clear_sources(mut self) -> Self {
        self.sources = None;
        self
    }
    /// Sets the task's current state.
    pub fn status(mut self, value: TaskStatus) -> Self {
        self.status = Some(value);
        self
    }
    /// Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }
    /// Omits `block_id`, including any model default.
    pub fn clear_block_id(mut self) -> Self {
        self.block_id = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<TaskCardBlock, ValidationError> {
        let task_id = wire::required(self.task_id, "TaskCardBlock.task_id")?;
        let title = wire::required(self.title, "TaskCardBlock.title")?;
        let details = self.details;
        let output = self.output;
        let sources = self.sources;
        let status = wire::required(self.status, "TaskCardBlock.status")?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &[
                "task_id", "title", "details", "output", "sources", "status", "block_id",
            ],
            "TaskCardBlock",
        )?;
        let value = TaskCardBlock {
            task_id,
            title,
            details,
            output,
            sources,
            status,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "TaskCardBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "TaskCardBlock.block_id")?;
        }
        crate::rules::validate("TaskCardBlock", &wire, "TaskCardBlock")?;
        Ok(value)
    }
}
impl Serialize for TaskCardBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for TaskCardBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "task_card", path)?;
        let task_id = wire::field::<String>(&mut map, "task_id", path, false)?;
        let title = wire::field::<String>(&mut map, "title", path, false)?;
        let details = wire::field::<RichTextBlock>(&mut map, "details", path, false)?;
        let output = wire::field::<RichTextBlock>(&mut map, "output", path, false)?;
        let sources = wire::field::<Vec<UrlSource>>(&mut map, "sources", path, false)?;
        let status = wire::field::<TaskStatus>(&mut map, "status", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        TaskCardBlockBuilder {
            task_id,
            title,
            details,
            output,
            sources,
            status,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for TaskCardBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "TaskCardBlock")
    }
}
impl<'de> Deserialize<'de> for TaskCardBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An interactive button that sends a payload or opens a URL.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/button-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ButtonElement {
    text: PlainText,
    action_id: Option<String>,
    url: Option<String>,
    value: Option<String>,
    style: Option<ButtonStyle>,
    confirm: Option<ConfirmationDialogue>,
    accessibility_label: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ButtonElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ButtonElementBuilder {
    text: Option<PlainTextInput>,
    action_id: Option<String>,
    url: Option<String>,
    value: Option<String>,
    style: Option<ButtonStyle>,
    confirm: Option<ConfirmationDialogue>,
    accessibility_label: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for ButtonElementBuilder {
    fn default() -> Self {
        Self {
            text: None,
            action_id: None,
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
            extensions: Map::new(),
        }
    }
}
impl ButtonElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ButtonElementBuilder {
        ButtonElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ButtonElementBuilder {
        ButtonElementBuilder {
            text: Some(self.text.into()),
            action_id: self.action_id,
            url: self.url,
            value: self.value,
            style: self.style,
            confirm: self.confirm,
            accessibility_label: self.accessibility_label,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the button label.
    pub fn text(&self) -> &PlainText {
        &self.text
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `url`. Sets a URL opened in the user's browser when the button is clicked. Slack still sends an interaction payload.
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    /// Borrows or copies `value`. Sets the application-defined value sent in interaction payloads.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    /// Borrows or copies `style`. Sets the button's emphasis. Omit it for the default neutral style.
    pub fn style(&self) -> Option<ButtonStyle> {
        self.style
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `accessibility_label`. Sets the label read by screen readers in place of the visible text.
    pub fn accessibility_label(&self) -> Option<&str> {
        self.accessibility_label.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "button")?;
        }
        map.serialize_entry("text", &self.text)?;
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.url {
            map.serialize_entry("url", value)?;
        }
        if let Some(value) = &self.value {
            map.serialize_entry("value", value)?;
        }
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.accessibility_label {
            map.serialize_entry("accessibility_label", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ButtonElementBuilder {
    /// Sets the button label.
    pub fn text(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(mut self, value: impl Into<String>) -> Self {
        self.action_id = Some(value.into());
        self
    }
    /// Omits `action_id`, including any model default.
    pub fn clear_action_id(mut self) -> Self {
        self.action_id = None;
        self
    }
    /// Sets a URL opened in the user's browser when the button is clicked. Slack still sends an interaction payload.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Omits `url`, including any model default.
    pub fn clear_url(mut self) -> Self {
        self.url = None;
        self
    }
    /// Sets the application-defined value sent in interaction payloads.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Omits `value`, including any model default.
    pub fn clear_value(mut self) -> Self {
        self.value = None;
        self
    }
    /// Sets the button's emphasis. Omit it for the default neutral style.
    pub fn style(mut self, value: ButtonStyle) -> Self {
        self.style = Some(value);
        self
    }
    /// Omits `style`, including any model default.
    pub fn clear_style(mut self) -> Self {
        self.style = None;
        self
    }
    /// Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(mut self, value: impl Into<ConfirmationDialogue>) -> Self {
        self.confirm = Some(value.into());
        self
    }
    /// Omits `confirm`, including any model default.
    pub fn clear_confirm(mut self) -> Self {
        self.confirm = None;
        self
    }
    /// Sets the label read by screen readers in place of the visible text.
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }
    /// Omits `accessibility_label`, including any model default.
    pub fn clear_accessibility_label(mut self) -> Self {
        self.accessibility_label = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<ButtonElement, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve().map_err(|e| e.at("ButtonElement.text")))
                .transpose()?,
            "ButtonElement.text",
        )?;
        let action_id = self.action_id;
        let url = self.url;
        let value = self.value;
        let style = self.style;
        let confirm = self.confirm;
        let accessibility_label = self.accessibility_label;
        wire::extensions(
            &self.extensions,
            &[
                "text",
                "action_id",
                "url",
                "value",
                "style",
                "confirm",
                "accessibility_label",
            ],
            "ButtonElement",
        )?;
        let value = ButtonElement {
            text,
            action_id,
            url,
            value,
            style,
            confirm,
            accessibility_label,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ButtonElement", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "button.text", "ButtonElement.text")?;
        }
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ButtonElement.action_id")?;
        }
        if let Some(v) = wire.get("url") {
            crate::rules::limits(v, "button.url", "ButtonElement.url")?;
        }
        if let Some(v) = wire.get("value") {
            crate::rules::limits(v, "button.value", "ButtonElement.value")?;
        }
        if let Some(v) = wire.get("accessibility_label") {
            crate::rules::limits(
                v,
                "button.accessibility_label",
                "ButtonElement.accessibility_label",
            )?;
        }
        crate::rules::validate("ButtonElement", &wire, "ButtonElement")?;
        Ok(value)
    }
}
impl Serialize for ButtonElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ButtonElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "button", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "plain_text");
        let text = wire::field::<PlainText>(&mut map, "text", path, false)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let url = wire::field::<String>(&mut map, "url", path, false)?;
        let value = wire::field::<String>(&mut map, "value", path, false)?;
        let style = wire::field::<ButtonStyle>(&mut map, "style", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let accessibility_label =
            wire::field::<String>(&mut map, "accessibility_label", path, false)?;
        ButtonElementBuilder {
            text: text.map(Into::into),
            action_id,
            url,
            value,
            style,
            confirm,
            accessibility_label,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ButtonElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ButtonElement")
    }
}
impl<'de> Deserialize<'de> for ButtonElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An image displayed inside a section, context, or card.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/image-element).
/// Provide exactly one of an image URL or a Slack file.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageElement {
    alt_text: String,
    image_url: Option<String>,
    slack_file: Option<SlackFile>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ImageElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ImageElementBuilder {
    alt_text: Option<String>,
    image_url: Option<String>,
    slack_file: Option<SlackFile>,
    extensions: Map<String, Value>,
}
impl Default for ImageElementBuilder {
    fn default() -> Self {
        Self {
            alt_text: None,
            image_url: None,
            slack_file: None,
            extensions: Map::new(),
        }
    }
}
impl ImageElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ImageElementBuilder {
        ImageElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ImageElementBuilder {
        ImageElementBuilder {
            alt_text: Some(self.alt_text),
            image_url: self.image_url,
            slack_file: self.slack_file,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `alt_text`. Sets a plain-text summary of the image or video for assistive technology.
    pub fn alt_text(&self) -> &str {
        &self.alt_text
    }
    /// Borrows or copies `image_url`. Sets the publicly accessible URL of the image. Cannot be combined with a Slack file.
    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }
    /// Borrows or copies `slack_file`. Sets a file hosted in Slack as the image source. Cannot be combined with an image URL.
    pub fn slack_file(&self) -> Option<&SlackFile> {
        self.slack_file.as_ref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "image")?;
        }
        map.serialize_entry("alt_text", &self.alt_text)?;
        if let Some(value) = &self.image_url {
            map.serialize_entry("image_url", value)?;
        }
        if let Some(value) = &self.slack_file {
            map.serialize_entry("slack_file", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ImageElementBuilder {
    /// Sets a plain-text summary of the image or video for assistive technology.
    pub fn alt_text(mut self, value: impl Into<String>) -> Self {
        self.alt_text = Some(value.into());
        self
    }
    /// Sets the publicly accessible URL of the image. Cannot be combined with a Slack file.
    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }
    /// Omits `image_url`, including any model default.
    pub fn clear_image_url(mut self) -> Self {
        self.image_url = None;
        self
    }
    /// Sets a file hosted in Slack as the image source. Cannot be combined with an image URL.
    pub fn slack_file(mut self, value: impl Into<SlackFile>) -> Self {
        self.slack_file = Some(value.into());
        self
    }
    /// Omits `slack_file`, including any model default.
    pub fn clear_slack_file(mut self) -> Self {
        self.slack_file = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<ImageElement, ValidationError> {
        let alt_text = wire::required(self.alt_text, "ImageElement.alt_text")?;
        let image_url = self.image_url;
        let slack_file = self.slack_file;
        wire::extensions(
            &self.extensions,
            &["alt_text", "image_url", "slack_file"],
            "ImageElement",
        )?;
        let value = ImageElement {
            alt_text,
            image_url,
            slack_file,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ImageElement", e.to_string())
        })?;
        if let Some(v) = wire.get("alt_text") {
            crate::rules::limits(v, "image_element.alt_text", "ImageElement.alt_text")?;
        }
        if let Some(v) = wire.get("image_url") {
            crate::rules::limits(v, "image_element.image_url", "ImageElement.image_url")?;
        }
        crate::rules::validate("ImageElement", &wire, "ImageElement")?;
        Ok(value)
    }
}
impl Serialize for ImageElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ImageElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "image", path)?;
        let alt_text = wire::field::<String>(&mut map, "alt_text", path, false)?;
        let image_url = wire::field::<String>(&mut map, "image_url", path, false)?;
        let slack_file = wire::field::<SlackFile>(&mut map, "slack_file", path, false)?;
        ImageElementBuilder {
            alt_text,
            image_url,
            slack_file,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ImageElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ImageElement")
    }
}
impl<'de> Deserialize<'de> for ImageElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Alignment and wrapping settings for one table column.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/table-block).

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnSettings {
    align: Option<ColumnAlign>,
    is_wrapped: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ColumnSettings`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ColumnSettingsBuilder {
    align: Option<ColumnAlign>,
    is_wrapped: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for ColumnSettingsBuilder {
    fn default() -> Self {
        Self {
            align: None,
            is_wrapped: None,
            extensions: Map::new(),
        }
    }
}
impl ColumnSettings {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ColumnSettingsBuilder {
        ColumnSettingsBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ColumnSettingsBuilder {
        ColumnSettingsBuilder {
            align: self.align,
            is_wrapped: self.is_wrapped,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `align`. Sets how cell content in the column is aligned.
    pub fn align(&self) -> Option<ColumnAlign> {
        self.align
    }
    /// Borrows or copies `is_wrapped`. Sets whether long cell content wraps instead of being truncated.
    pub fn is_wrapped(&self) -> Option<bool> {
        self.is_wrapped
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        let _ = tagged;
        if let Some(value) = &self.align {
            map.serialize_entry("align", value)?;
        }
        if let Some(value) = &self.is_wrapped {
            map.serialize_entry("is_wrapped", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ColumnSettingsBuilder {
    /// Sets how cell content in the column is aligned.
    pub fn align(mut self, value: ColumnAlign) -> Self {
        self.align = Some(value);
        self
    }
    /// Omits `align`, including any model default.
    pub fn clear_align(mut self) -> Self {
        self.align = None;
        self
    }
    /// Sets whether long cell content wraps instead of being truncated.
    pub fn is_wrapped(mut self, value: bool) -> Self {
        self.is_wrapped = Some(value);
        self
    }
    /// Omits `is_wrapped`, including any model default.
    pub fn clear_is_wrapped(mut self) -> Self {
        self.is_wrapped = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<ColumnSettings, ValidationError> {
        let align = self.align;
        let is_wrapped = self.is_wrapped;
        wire::extensions(&self.extensions, &["align", "is_wrapped"], "ColumnSettings")?;
        let value = ColumnSettings {
            align,
            is_wrapped,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ColumnSettings", e.to_string())
        })?;
        crate::rules::validate("ColumnSettings", &wire, "ColumnSettings")?;
        Ok(value)
    }
}
impl Serialize for ColumnSettings {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ColumnSettings {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let align = wire::field::<ColumnAlign>(&mut map, "align", path, false)?;
        let is_wrapped = wire::field::<bool>(&mut map, "is_wrapped", path, false)?;
        ColumnSettingsBuilder {
            align,
            is_wrapped,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ColumnSettings {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ColumnSettings")
    }
}
impl<'de> Deserialize<'de> for ColumnSettings {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A dialog that asks users to confirm an action before it is sent.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object).

#[derive(Clone, Debug, PartialEq)]
pub struct ConfirmationDialogue {
    title: PlainText,
    text: Text,
    confirm: PlainText,
    deny: PlainText,
    style: Option<ButtonStyle>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ConfirmationDialogue`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ConfirmationDialogueBuilder {
    title: Option<PlainTextInput>,
    text: Option<TextInput>,
    confirm: Option<PlainTextInput>,
    deny: Option<PlainTextInput>,
    style: Option<ButtonStyle>,
    extensions: Map<String, Value>,
}
impl Default for ConfirmationDialogueBuilder {
    fn default() -> Self {
        Self {
            title: None,
            text: None,
            confirm: None,
            deny: None,
            style: None,
            extensions: Map::new(),
        }
    }
}
impl ConfirmationDialogue {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ConfirmationDialogueBuilder {
        ConfirmationDialogueBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ConfirmationDialogueBuilder {
        ConfirmationDialogueBuilder {
            title: Some(self.title.into()),
            text: Some(self.text.into()),
            confirm: Some(self.confirm.into()),
            deny: Some(self.deny.into()),
            style: self.style,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `title`. Sets the dialog title.
    pub fn title(&self) -> &PlainText {
        &self.title
    }
    /// Borrows or copies `text`. Sets the explanatory text in the dialog body.
    pub fn text(&self) -> &Text {
        &self.text
    }
    /// Borrows or copies `confirm`. Sets the label of the button that confirms the action.
    pub fn confirm(&self) -> &PlainText {
        &self.confirm
    }
    /// Borrows or copies `deny`. Sets the label of the button that cancels the action.
    pub fn deny(&self) -> &PlainText {
        &self.deny
    }
    /// Borrows or copies `style`. Sets the emphasis of the confirm button.
    pub fn style(&self) -> Option<ButtonStyle> {
        self.style
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        let _ = tagged;
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("confirm", &self.confirm)?;
        map.serialize_entry("deny", &self.deny)?;
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ConfirmationDialogueBuilder {
    /// Sets the dialog title.
    pub fn title(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Sets the explanatory text in the dialog body.
    pub fn text(mut self, value: impl Into<TextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the label of the button that confirms the action.
    pub fn confirm(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.confirm = Some(value.into());
        self
    }
    /// Sets the label of the button that cancels the action.
    pub fn deny(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.deny = Some(value.into());
        self
    }
    /// Sets the emphasis of the confirm button.
    pub fn style(mut self, value: ButtonStyle) -> Self {
        self.style = Some(value);
        self
    }
    /// Omits `style`, including any model default.
    pub fn clear_style(mut self) -> Self {
        self.style = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<ConfirmationDialogue, ValidationError> {
        let title = wire::required(
            self.title
                .map(|v| v.resolve().map_err(|e| e.at("ConfirmationDialogue.title")))
                .transpose()?,
            "ConfirmationDialogue.title",
        )?;
        let text = wire::required(
            self.text
                .map(|v| {
                    v.resolve(false)
                        .map_err(|e| e.at("ConfirmationDialogue.text"))
                })
                .transpose()?,
            "ConfirmationDialogue.text",
        )?;
        let confirm = wire::required(
            self.confirm
                .map(|v| {
                    v.resolve()
                        .map_err(|e| e.at("ConfirmationDialogue.confirm"))
                })
                .transpose()?,
            "ConfirmationDialogue.confirm",
        )?;
        let deny = wire::required(
            self.deny
                .map(|v| v.resolve().map_err(|e| e.at("ConfirmationDialogue.deny")))
                .transpose()?,
            "ConfirmationDialogue.deny",
        )?;
        let style = self.style;
        wire::extensions(
            &self.extensions,
            &["title", "text", "confirm", "deny", "style"],
            "ConfirmationDialogue",
        )?;
        let value = ConfirmationDialogue {
            title,
            text,
            confirm,
            deny,
            style,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ConfirmationDialogue",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("title") {
            crate::rules::limits(v, "confirmation.title", "ConfirmationDialogue.title")?;
        }
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "confirmation.text", "ConfirmationDialogue.text")?;
        }
        if let Some(v) = wire.get("confirm") {
            crate::rules::limits(v, "confirmation.confirm", "ConfirmationDialogue.confirm")?;
        }
        if let Some(v) = wire.get("deny") {
            crate::rules::limits(v, "confirmation.deny", "ConfirmationDialogue.deny")?;
        }
        crate::rules::validate("Confirmation", &wire, "ConfirmationDialogue")?;
        Ok(value)
    }
}
impl Serialize for ConfirmationDialogue {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ConfirmationDialogue {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        crate::rules::coerce_text(map.get_mut("title"), "plain_text");
        let title = wire::field::<PlainText>(&mut map, "title", path, false)?;
        crate::rules::coerce_text(map.get_mut("text"), "mrkdwn");
        let text = wire::field::<Text>(&mut map, "text", path, false)?;
        crate::rules::coerce_text(map.get_mut("confirm"), "plain_text");
        let confirm = wire::field::<PlainText>(&mut map, "confirm", path, false)?;
        crate::rules::coerce_text(map.get_mut("deny"), "plain_text");
        let deny = wire::field::<PlainText>(&mut map, "deny", path, false)?;
        let style = wire::field::<ButtonStyle>(&mut map, "style", path, false)?;
        ConfirmationDialogueBuilder {
            title: title.map(Into::into),
            text: text.map(Into::into),
            confirm: confirm.map(Into::into),
            deny: deny.map(Into::into),
            style,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ConfirmationDialogue {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ConfirmationDialogue")
    }
}
impl<'de> Deserialize<'de> for ConfirmationDialogue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A mrkdwn text composition object.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/text-object).

#[derive(Clone, Debug, PartialEq)]
pub struct MarkdownText {
    text: String,
    verbatim: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`MarkdownText`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct MarkdownTextBuilder {
    text: Option<String>,
    verbatim: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for MarkdownTextBuilder {
    fn default() -> Self {
        Self {
            text: None,
            verbatim: None,
            extensions: Map::new(),
        }
    }
}
impl MarkdownText {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> MarkdownTextBuilder {
        MarkdownTextBuilder::default()
    }
    /// Constructs validated text.
    ///
    /// # Errors
    /// Returns a validation error when text violates its field limits.
    pub fn new(text: impl Into<String>) -> Result<Self, ValidationError> {
        Self::builder().text(text).build()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> MarkdownTextBuilder {
        MarkdownTextBuilder {
            text: Some(self.text),
            verbatim: self.verbatim,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the mrkdwn-formatted text content.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Borrows or copies `verbatim`. Sets whether Slack leaves URLs, mentions, and channel names unlinked.
    pub fn verbatim(&self) -> Option<bool> {
        self.verbatim
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "mrkdwn")?;
        }
        map.serialize_entry("text", &self.text)?;
        if let Some(value) = &self.verbatim {
            map.serialize_entry("verbatim", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl MarkdownTextBuilder {
    /// Sets the mrkdwn-formatted text content.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets whether Slack leaves URLs, mentions, and channel names unlinked.
    pub fn verbatim(mut self, value: bool) -> Self {
        self.verbatim = Some(value);
        self
    }
    /// Omits `verbatim`, including any model default.
    pub fn clear_verbatim(mut self) -> Self {
        self.verbatim = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<MarkdownText, ValidationError> {
        let text = wire::required(self.text, "MarkdownText.text")?;
        let verbatim = self.verbatim;
        wire::extensions(&self.extensions, &["text", "verbatim"], "MarkdownText")?;
        let value = MarkdownText {
            text,
            verbatim,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "MarkdownText", e.to_string())
        })?;
        crate::rules::limits(&wire["text"], "text", "MarkdownText.text")?;
        crate::rules::validate("MarkdownText", &wire, "MarkdownText")?;
        Ok(value)
    }
}
impl Serialize for MarkdownText {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for MarkdownText {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "mrkdwn", path)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let verbatim = wire::field::<bool>(&mut map, "verbatim", path, false)?;
        MarkdownTextBuilder {
            text,
            verbatim,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for MarkdownText {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "MarkdownText")
    }
}
impl<'de> Deserialize<'de> for MarkdownText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A selectable option for menus, checkboxes, radio buttons, and overflow menus.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/option-object).

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption {
    text: Text,
    value: String,
    description: Option<Text>,
    url: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`SelectOption`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct SelectOptionBuilder {
    text: Option<TextInput>,
    value: Option<String>,
    description: Option<TextInput>,
    url: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for SelectOptionBuilder {
    fn default() -> Self {
        Self {
            text: None,
            value: None,
            description: None,
            url: None,
            extensions: Map::new(),
        }
    }
}
impl SelectOption {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> SelectOptionBuilder {
        SelectOptionBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> SelectOptionBuilder {
        SelectOptionBuilder {
            text: Some(self.text.into()),
            value: Some(self.value),
            description: self.description.map(Into::into),
            url: self.url,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the option label shown to users.
    pub fn text(&self) -> &Text {
        &self.text
    }
    /// Borrows or copies `value`. Sets the value sent in interaction payloads when this option is chosen.
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Borrows or copies `description`. Sets supporting text shown below the label. Only rendered by checkboxes and radio buttons.
    pub fn description(&self) -> Option<&Text> {
        self.description.as_ref()
    }
    /// Borrows or copies `url`. Sets a URL loaded in the user's browser when an overflow menu option is chosen.
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        let _ = tagged;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("value", &self.value)?;
        if let Some(value) = &self.description {
            map.serialize_entry("description", value)?;
        }
        if let Some(value) = &self.url {
            map.serialize_entry("url", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SelectOptionBuilder {
    /// Sets the option label shown to users.
    pub fn text(mut self, value: impl Into<TextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the value sent in interaction payloads when this option is chosen.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Sets supporting text shown below the label. Only rendered by checkboxes and radio buttons.
    pub fn description(mut self, value: impl Into<TextInput>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Omits `description`, including any model default.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }
    /// Sets a URL loaded in the user's browser when an overflow menu option is chosen.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Omits `url`, including any model default.
    pub fn clear_url(mut self) -> Self {
        self.url = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<SelectOption, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve(true).map_err(|e| e.at("SelectOption.text")))
                .transpose()?,
            "SelectOption.text",
        )?;
        let value = wire::required(self.value, "SelectOption.value")?;
        let description = self
            .description
            .map(|v| {
                v.resolve(true)
                    .map_err(|e| e.at("SelectOption.description"))
            })
            .transpose()?;
        let url = self.url;
        wire::extensions(
            &self.extensions,
            &["text", "value", "description", "url"],
            "SelectOption",
        )?;
        let value = SelectOption {
            text,
            value,
            description,
            url,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "SelectOption", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "option.text", "SelectOption.text")?;
        }
        if let Some(v) = wire.get("value") {
            crate::rules::limits(v, "option.value", "SelectOption.value")?;
        }
        if let Some(v) = wire.get("description") {
            crate::rules::limits(v, "option.description", "SelectOption.description")?;
        }
        if let Some(v) = wire.get("url") {
            crate::rules::limits(v, "option.url", "SelectOption.url")?;
        }
        crate::rules::validate("Option", &wire, "SelectOption")?;
        Ok(value)
    }
}
impl Serialize for SelectOption {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for SelectOption {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "plain_text");
        let text = wire::field::<Text>(&mut map, "text", path, false)?;
        let value = wire::field::<String>(&mut map, "value", path, false)?;
        crate::rules::coerce_text(map.get_mut("description"), "plain_text");
        let description = wire::field::<Text>(&mut map, "description", path, false)?;
        let url = wire::field::<String>(&mut map, "url", path, false)?;
        SelectOptionBuilder {
            text: text.map(Into::into),
            value,
            description: description.map(Into::into),
            url,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for SelectOption {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "SelectOption")
    }
}
impl<'de> Deserialize<'de> for SelectOption {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A plain-text composition object.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/text-object).

#[derive(Clone, Debug, PartialEq)]
pub struct PlainText {
    text: String,
    emoji: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`PlainText`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct PlainTextBuilder {
    text: Option<String>,
    emoji: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for PlainTextBuilder {
    fn default() -> Self {
        Self {
            text: None,
            emoji: None,
            extensions: Map::new(),
        }
    }
}
impl PlainText {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> PlainTextBuilder {
        PlainTextBuilder::default()
    }
    /// Constructs validated text.
    ///
    /// # Errors
    /// Returns a validation error when text violates its field limits.
    pub fn new(text: impl Into<String>) -> Result<Self, ValidationError> {
        Self::builder().text(text).build()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> PlainTextBuilder {
        PlainTextBuilder {
            text: Some(self.text),
            emoji: self.emoji,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the text content.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Borrows or copies `emoji`. Sets whether emoji shortcodes such as :tada: are rendered as emoji.
    pub fn emoji(&self) -> Option<bool> {
        self.emoji
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "plain_text")?;
        }
        map.serialize_entry("text", &self.text)?;
        if let Some(value) = &self.emoji {
            map.serialize_entry("emoji", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PlainTextBuilder {
    /// Sets the text content.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets whether emoji shortcodes such as :tada: are rendered as emoji.
    pub fn emoji(mut self, value: bool) -> Self {
        self.emoji = Some(value);
        self
    }
    /// Omits `emoji`, including any model default.
    pub fn clear_emoji(mut self) -> Self {
        self.emoji = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<PlainText, ValidationError> {
        let text = wire::required(self.text, "PlainText.text")?;
        let emoji = self.emoji;
        wire::extensions(&self.extensions, &["text", "emoji"], "PlainText")?;
        let value = PlainText {
            text,
            emoji,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "PlainText", e.to_string())
        })?;
        crate::rules::limits(&wire["text"], "text", "PlainText.text")?;
        crate::rules::validate("PlainText", &wire, "PlainText")?;
        Ok(value)
    }
}
impl Serialize for PlainText {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for PlainText {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "plain_text", path)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let emoji = wire::field::<bool>(&mut map, "emoji", path, false)?;
        PlainTextBuilder {
            text,
            emoji,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for PlainText {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "PlainText")
    }
}
impl<'de> Deserialize<'de> for PlainText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A numeric data table cell with display text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-table-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RawNumber {
    value: JsonNumber,
    text: String,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RawNumber`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RawNumberBuilder {
    value: Option<JsonNumber>,
    text: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for RawNumberBuilder {
    fn default() -> Self {
        Self {
            value: None,
            text: None,
            extensions: Map::new(),
        }
    }
}
impl RawNumber {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RawNumberBuilder {
        RawNumberBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RawNumberBuilder {
        RawNumberBuilder {
            value: Some(self.value),
            text: Some(self.text),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `value`. Sets the numeric value used for sorting.
    pub fn value(&self) -> &JsonNumber {
        &self.value
    }
    /// Borrows or copies `text`. Sets the formatted text displayed for the number, such as 1,234.5.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "raw_number")?;
        }
        map.serialize_entry("value", &self.value)?;
        map.serialize_entry("text", &self.text)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RawNumberBuilder {
    /// Sets the numeric value used for sorting.
    pub fn value(mut self, value: impl Into<JsonNumber>) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Sets the formatted text displayed for the number, such as 1,234.5.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<RawNumber, ValidationError> {
        let value = wire::required(self.value, "RawNumber.value")?;
        let text = wire::required(self.text, "RawNumber.text")?;
        wire::extensions(&self.extensions, &["value", "text"], "RawNumber")?;
        let value = RawNumber {
            value,
            text,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RawNumber", e.to_string())
        })?;
        crate::rules::validate("RawNumber", &wire, "RawNumber")?;
        Ok(value)
    }
}
impl Serialize for RawNumber {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RawNumber {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "raw_number", path)?;
        let value = wire::field::<JsonNumber>(&mut map, "value", path, false)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        RawNumberBuilder {
            value,
            text,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RawNumber {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RawNumber")
    }
}
impl<'de> Deserialize<'de> for RawNumber {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An unformatted table cell.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/table-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RawText {
    text: String,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RawText`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RawTextBuilder {
    text: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for RawTextBuilder {
    fn default() -> Self {
        Self {
            text: None,
            extensions: Map::new(),
        }
    }
}
impl RawText {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RawTextBuilder {
        RawTextBuilder::default()
    }
    /// Constructs validated text.
    ///
    /// # Errors
    /// Returns a validation error when text violates its field limits.
    pub fn new(text: impl Into<String>) -> Result<Self, ValidationError> {
        Self::builder().text(text).build()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RawTextBuilder {
        RawTextBuilder {
            text: Some(self.text),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the cell text.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "raw_text")?;
        }
        map.serialize_entry("text", &self.text)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RawTextBuilder {
    /// Sets the cell text.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<RawText, ValidationError> {
        let text = wire::required(self.text, "RawText.text")?;
        wire::extensions(&self.extensions, &["text"], "RawText")?;
        let value = RawText {
            text,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RawText", e.to_string())
        })?;
        crate::rules::validate("RawText", &wire, "RawText")?;
        Ok(value)
    }
}
impl Serialize for RawText {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RawText {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "raw_text", path)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        RawTextBuilder {
            text,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RawText {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RawText")
    }
}
impl<'de> Deserialize<'de> for RawText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A bulleted or numbered list inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextList {
    style: RichTextListStyle,
    elements: Vec<RichTextSection>,
    indent: Option<i64>,
    offset: Option<i64>,
    border: Option<i64>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextList`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextListBuilder {
    style: Option<RichTextListStyle>,
    elements: Option<Vec<RichTextSection>>,
    indent: Option<i64>,
    offset: Option<i64>,
    border: Option<i64>,
    extensions: Map<String, Value>,
}
impl Default for RichTextListBuilder {
    fn default() -> Self {
        Self {
            style: None,
            elements: None,
            indent: None,
            offset: None,
            border: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextList {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextListBuilder {
        RichTextListBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextListBuilder {
        RichTextListBuilder {
            style: Some(self.style),
            elements: Some(self.elements),
            indent: self.indent,
            offset: self.offset,
            border: self.border,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `style`. Sets whether the list is bulleted or numbered.
    pub fn style(&self) -> RichTextListStyle {
        self.style
    }
    /// Borrows or copies `elements`. Adds list items, one rich text section per item.
    pub fn elements(&self) -> &[RichTextSection] {
        &self.elements
    }
    /// Borrows or copies `indent`. Sets the list's indentation level.
    pub fn indent(&self) -> Option<i64> {
        self.indent
    }
    /// Borrows or copies `offset`. Sets the number of items to skip when numbering an ordered list.
    pub fn offset(&self) -> Option<i64> {
        self.offset
    }
    /// Borrows or copies `border`. Sets the width of the left border, in pixels.
    pub fn border(&self) -> Option<i64> {
        self.border
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "rich_text_list")?;
        }
        map.serialize_entry("style", &self.style)?;
        map.serialize_entry("elements", &self.elements)?;
        if let Some(value) = &self.indent {
            map.serialize_entry("indent", value)?;
        }
        if let Some(value) = &self.offset {
            map.serialize_entry("offset", value)?;
        }
        if let Some(value) = &self.border {
            map.serialize_entry("border", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextListBuilder {
    /// Sets whether the list is bulleted or numbered.
    pub fn style(mut self, value: RichTextListStyle) -> Self {
        self.style = Some(value);
        self
    }
    /// Adds list items, one rich text section per item.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<RichTextSection>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<RichTextSection>) -> Self {
        self.elements
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Sets the list's indentation level.
    pub fn indent(mut self, value: i64) -> Self {
        self.indent = Some(value);
        self
    }
    /// Omits `indent`, including any model default.
    pub fn clear_indent(mut self) -> Self {
        self.indent = None;
        self
    }
    /// Sets the number of items to skip when numbering an ordered list.
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Omits `offset`, including any model default.
    pub fn clear_offset(mut self) -> Self {
        self.offset = None;
        self
    }
    /// Sets the width of the left border, in pixels.
    pub fn border(mut self, value: i64) -> Self {
        self.border = Some(value);
        self
    }
    /// Omits `border`, including any model default.
    pub fn clear_border(mut self) -> Self {
        self.border = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<RichTextList, ValidationError> {
        let style = wire::required(self.style, "RichTextList.style")?;
        let elements = wire::required(self.elements, "RichTextList.elements")?;
        let indent = self.indent;
        let offset = self.offset;
        let border = self.border;
        wire::extensions(
            &self.extensions,
            &["style", "elements", "indent", "offset", "border"],
            "RichTextList",
        )?;
        let value = RichTextList {
            style,
            elements,
            indent,
            offset,
            border,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextList", e.to_string())
        })?;
        if let Some(v) = wire.get("indent") {
            crate::rules::limits(v, "rich_text_list.indent", "RichTextList.indent")?;
        }
        if let Some(v) = wire.get("offset") {
            crate::rules::limits(v, "rich_text_list.offset", "RichTextList.offset")?;
        }
        if let Some(v) = wire.get("border") {
            crate::rules::limits(v, "rich_text_list.border", "RichTextList.border")?;
        }
        crate::rules::validate("RichTextList", &wire, "RichTextList")?;
        Ok(value)
    }
}
impl Serialize for RichTextList {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextList {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "rich_text_list", path)?;
        let style = wire::field::<RichTextListStyle>(&mut map, "style", path, false)?;
        let elements = wire::field::<Vec<RichTextSection>>(&mut map, "elements", path, false)?;
        let indent = wire::field::<i64>(&mut map, "indent", path, false)?;
        let offset = wire::field::<i64>(&mut map, "offset", path, false)?;
        let border = wire::field::<i64>(&mut map, "border", path, false)?;
        RichTextListBuilder {
            style,
            elements,
            indent,
            offset,
            border,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextList {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextList")
    }
}
impl<'de> Deserialize<'de> for RichTextList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A paragraph of inline rich text elements.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextSection {
    elements: Vec<RichTextSectionElement>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextSection`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextSectionBuilder {
    elements: Option<Vec<RichTextSectionElement>>,
    extensions: Map<String, Value>,
}
impl Default for RichTextSectionBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextSection {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextSectionBuilder {
        RichTextSectionBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextSectionBuilder {
        RichTextSectionBuilder {
            elements: Some(self.elements),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds inline rich text elements in display order.
    pub fn elements(&self) -> &[RichTextSectionElement] {
        &self.elements
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "rich_text_section")?;
        }
        map.serialize_entry("elements", &self.elements)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextSectionBuilder {
    /// Adds inline rich text elements in display order.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<RichTextSectionElement>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<RichTextSectionElement>) -> Self {
        self.elements
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<RichTextSection, ValidationError> {
        let elements = wire::required(self.elements, "RichTextSection.elements")?;
        wire::extensions(&self.extensions, &["elements"], "RichTextSection")?;
        let value = RichTextSection {
            elements,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "RichTextSection",
                e.to_string(),
            )
        })?;
        crate::rules::validate("RichTextSection", &wire, "RichTextSection")?;
        Ok(value)
    }
}
impl Serialize for RichTextSection {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextSection {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "rich_text_section", path)?;
        let elements =
            wire::field::<Vec<RichTextSectionElement>>(&mut map, "elements", path, false)?;
        RichTextSectionBuilder {
            elements,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextSection {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextSection")
    }
}
impl<'de> Deserialize<'de> for RichTextSection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A run of optionally styled text inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextText {
    text: String,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextText`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextTextBuilder {
    text: Option<String>,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
impl Default for RichTextTextBuilder {
    fn default() -> Self {
        Self {
            text: None,
            style: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextText {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextTextBuilder {
        RichTextTextBuilder::default()
    }
    /// Constructs validated text.
    ///
    /// # Errors
    /// Returns a validation error when text violates its field limits.
    pub fn new(text: impl Into<String>) -> Result<Self, ValidationError> {
        Self::builder().text(text).build()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextTextBuilder {
        RichTextTextBuilder {
            text: Some(self.text),
            style: self.style,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the text content.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Borrows or copies `style`. Sets the visual style.
    pub fn style(&self) -> Option<&RichTextStyle> {
        self.style.as_ref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "text")?;
        }
        map.serialize_entry("text", &self.text)?;
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextTextBuilder {
    /// Sets the text content.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the visual style.
    pub fn style(mut self, value: RichTextStyle) -> Self {
        self.style = Some(value);
        self
    }
    /// Omits `style`, including any model default.
    pub fn clear_style(mut self) -> Self {
        self.style = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<RichTextText, ValidationError> {
        let text = wire::required(self.text, "RichTextText.text")?;
        let style = self.style;
        wire::extensions(&self.extensions, &["text", "style"], "RichTextText")?;
        let value = RichTextText {
            text,
            style,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextText", e.to_string())
        })?;
        if let Some(v) = wire.get("style") {
            crate::rules::style(
                v,
                &["bold", "italic", "strike", "code"],
                "RichTextText.style",
            )?;
        }
        crate::rules::validate("RichTextText", &wire, "RichTextText")?;
        Ok(value)
    }
}
impl Serialize for RichTextText {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextText {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "text", path)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let style = wire::field::<RichTextStyle>(&mut map, "style", path, false)?;
        RichTextTextBuilder {
            text,
            style,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextText {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextText")
    }
}
impl<'de> Deserialize<'de> for RichTextText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A reference to an image file hosted in Slack.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/slack-file-object).
/// Provide exactly one of an ID or a URL. An ID must match ^F[A-Z0-9]{8,}$.
#[derive(Clone, Debug, PartialEq)]
pub struct SlackFile {
    id: Option<String>,
    url: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`SlackFile`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct SlackFileBuilder {
    id: Option<String>,
    url: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for SlackFileBuilder {
    fn default() -> Self {
        Self {
            id: None,
            url: None,
            extensions: Map::new(),
        }
    }
}
impl SlackFile {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> SlackFileBuilder {
        SlackFileBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> SlackFileBuilder {
        SlackFileBuilder {
            id: self.id,
            url: self.url,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `id`. Sets the ID of a file previously uploaded to Slack. Cannot be combined with a URL.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    /// Borrows or copies `url`. Sets the URL of a file hosted in Slack. Cannot be combined with an ID.
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        let _ = tagged;
        if let Some(value) = &self.id {
            map.serialize_entry("id", value)?;
        }
        if let Some(value) = &self.url {
            map.serialize_entry("url", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SlackFileBuilder {
    /// Sets the ID of a file previously uploaded to Slack. Cannot be combined with a URL.
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Omits `id`, including any model default.
    pub fn clear_id(mut self) -> Self {
        self.id = None;
        self
    }
    /// Sets the URL of a file hosted in Slack. Cannot be combined with an ID.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Omits `url`, including any model default.
    pub fn clear_url(mut self) -> Self {
        self.url = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<SlackFile, ValidationError> {
        let id = self.id;
        let url = self.url;
        wire::extensions(&self.extensions, &["id", "url"], "SlackFile")?;
        let value = SlackFile {
            id,
            url,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "SlackFile", e.to_string())
        })?;
        crate::rules::validate("SlackFile", &wire, "SlackFile")?;
        Ok(value)
    }
}
impl Serialize for SlackFile {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for SlackFile {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let id = wire::field::<String>(&mut map, "id", path, false)?;
        let url = wire::field::<String>(&mut map, "url", path, false)?;
        SlackFileBuilder {
            id,
            url,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for SlackFile {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "SlackFile")
    }
}
impl<'de> Deserialize<'de> for SlackFile {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A linked source cited by a task card.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/task-card-block).

#[derive(Clone, Debug, PartialEq)]
pub struct UrlSource {
    url: String,
    text: String,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`UrlSource`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct UrlSourceBuilder {
    url: Option<String>,
    text: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for UrlSourceBuilder {
    fn default() -> Self {
        Self {
            url: None,
            text: None,
            extensions: Map::new(),
        }
    }
}
impl UrlSource {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> UrlSourceBuilder {
        UrlSourceBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> UrlSourceBuilder {
        UrlSourceBuilder {
            url: Some(self.url),
            text: Some(self.text),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `url`. Sets the source URL.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Borrows or copies `text`. Sets the source link text.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if tagged {
            map.serialize_entry("type", "url")?;
        }
        map.serialize_entry("url", &self.url)?;
        map.serialize_entry("text", &self.text)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl UrlSourceBuilder {
    /// Sets the source URL.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Sets the source link text.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<UrlSource, ValidationError> {
        let url = wire::required(self.url, "UrlSource.url")?;
        let text = wire::required(self.text, "UrlSource.text")?;
        wire::extensions(&self.extensions, &["url", "text"], "UrlSource")?;
        let value = UrlSource {
            url,
            text,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "UrlSource", e.to_string())
        })?;
        crate::rules::validate("UrlSource", &wire, "UrlSource")?;
        Ok(value)
    }
}
impl Serialize for UrlSource {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for UrlSource {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "url", path)?;
        let url = wire::field::<String>(&mut map, "url", path, false)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        UrlSourceBuilder {
            url,
            text,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for UrlSource {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "UrlSource")
    }
}
impl<'de> Deserialize<'de> for UrlSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A legacy message attachment containing blocks and an optional color bar.
///
/// [Slack reference](https://docs.slack.dev/messaging/formatting-message-text#when-to-use-attachments).
/// Only blocks supported in messages are accepted.
#[derive(Clone, Debug, PartialEq)]
pub struct Attachment {
    blocks: Vec<Block>,
    color: Option<String>,
    fallback: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`Attachment`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct AttachmentBuilder {
    blocks: Option<Vec<Block>>,
    color: Option<String>,
    fallback: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for AttachmentBuilder {
    fn default() -> Self {
        Self {
            blocks: None,
            color: None,
            fallback: None,
            extensions: Map::new(),
        }
    }
}
impl Attachment {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> AttachmentBuilder {
        AttachmentBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> AttachmentBuilder {
        AttachmentBuilder {
            blocks: Some(self.blocks),
            color: self.color,
            fallback: self.fallback,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `blocks`. Adds the attachment's blocks in display order.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    /// Borrows or copies `color`. Sets the attachment's left border color: a six-digit hex color or good, warning, or danger.
    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }
    /// Borrows or copies `fallback`. Sets plain-text summary shown in clients that cannot display attachments.
    pub fn fallback(&self) -> Option<&str> {
        self.fallback.as_deref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        let _ = tagged;
        map.serialize_entry("blocks", &self.blocks)?;
        if let Some(value) = &self.color {
            map.serialize_entry("color", value)?;
        }
        if let Some(value) = &self.fallback {
            map.serialize_entry("fallback", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttachmentBuilder {
    /// Adds the attachment's blocks in display order.
    /// Replaces the collection; order is preserved.
    pub fn blocks<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Block>,
    {
        self.blocks = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn block(mut self, value: impl Into<Block>) -> Self {
        self.blocks.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Sets the attachment's left border color: a six-digit hex color or good, warning, or danger.
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }
    /// Omits `color`, including any model default.
    pub fn clear_color(mut self) -> Self {
        self.color = None;
        self
    }
    /// Sets plain-text summary shown in clients that cannot display attachments.
    pub fn fallback(mut self, value: impl Into<String>) -> Self {
        self.fallback = Some(value.into());
        self
    }
    /// Omits `fallback`, including any model default.
    pub fn clear_fallback(mut self) -> Self {
        self.fallback = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<Attachment, ValidationError> {
        let blocks = wire::required(self.blocks, "Attachment.blocks")?;
        let color = self.color;
        let fallback = self.fallback;
        wire::extensions(
            &self.extensions,
            &["blocks", "color", "fallback"],
            "Attachment",
        )?;
        let value = Attachment {
            blocks,
            color,
            fallback,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "Attachment", e.to_string())
        })?;
        crate::rules::validate("Attachment", &wire, "Attachment")?;
        Ok(value)
    }
}
impl Serialize for Attachment {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for Attachment {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let blocks = wire::field::<Vec<Block>>(&mut map, "blocks", path, false)?;
        let color = wire::field::<String>(&mut map, "color", path, false)?;
        let fallback = wire::field::<String>(&mut map, "fallback", path, false)?;
        AttachmentBuilder {
            blocks,
            color,
            fallback,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for Attachment {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Attachment")
    }
}
impl<'de> Deserialize<'de> for Attachment {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A complete payload for chat.postMessage and related Web API methods.
///
/// [Slack reference](https://docs.slack.dev/reference/methods/chat.postMessage).
/// Only blocks supported in messages are accepted. Markdown block text is limited to 12,000 characters, and data table cell text to 20,000 characters, across the whole message.
#[derive(Clone, Debug, PartialEq)]
pub struct MessagePayload {
    channel: String,
    blocks: Option<Vec<Block>>,
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
    mrkdwn: Option<bool>,
    unfurl_links: Option<bool>,
    unfurl_media: Option<bool>,
    metadata: Option<Map<String, Value>>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`MessagePayload`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct MessagePayloadBuilder {
    channel: Option<String>,
    blocks: Option<Vec<Block>>,
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
    mrkdwn: Option<bool>,
    unfurl_links: Option<bool>,
    unfurl_media: Option<bool>,
    metadata: Option<Map<String, Value>>,
    extensions: Map<String, Value>,
}
impl Default for MessagePayloadBuilder {
    fn default() -> Self {
        Self {
            channel: None,
            blocks: None,
            attachments: None,
            text: Some("".into()),
            mrkdwn: Some(true),
            unfurl_links: None,
            unfurl_media: None,
            metadata: None,
            extensions: Map::new(),
        }
    }
}
impl MessagePayload {
    /// Starts a builder with the documented model defaults.
    pub fn builder(channel: impl Into<String>) -> MessagePayloadBuilder {
        MessagePayloadBuilder::default().channel(channel)
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> MessagePayloadBuilder {
        MessagePayloadBuilder {
            channel: Some(self.channel),
            blocks: self.blocks,
            attachments: self.attachments,
            text: self.text,
            mrkdwn: self.mrkdwn,
            unfurl_links: self.unfurl_links,
            unfurl_media: self.unfurl_media,
            metadata: self.metadata,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `channel`. Sets the ID of the channel, private group, or conversation that receives the message.
    pub fn channel(&self) -> &str {
        &self.channel
    }
    /// Borrows or copies `blocks`. Adds the message blocks in display order.
    pub fn blocks(&self) -> Option<&[Block]> {
        self.blocks.as_deref()
    }
    /// Borrows or copies `attachments`. Adds legacy attachments in display order.
    pub fn attachments(&self) -> Option<&[Attachment]> {
        self.attachments.as_deref()
    }
    /// Borrows or copies `text`. Sets the fallback text used in notifications and by clients that cannot display blocks.
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
    /// Borrows or copies `mrkdwn`. Sets whether Slack formats the top-level text as mrkdwn.
    pub fn mrkdwn(&self) -> Option<bool> {
        self.mrkdwn
    }
    /// Borrows or copies `unfurl_links`. Sets whether Slack unfurls text-based links.
    pub fn unfurl_links(&self) -> Option<bool> {
        self.unfurl_links
    }
    /// Borrows or copies `unfurl_media`. Sets whether Slack unfurls media links.
    pub fn unfurl_media(&self) -> Option<bool> {
        self.unfurl_media
    }
    /// Borrows or copies `metadata`. Sets message metadata as a JSON-compatible map, such as event_type and event_payload.
    pub fn metadata(&self) -> Option<&Map<String, Value>> {
        self.metadata.as_ref()
    }
    /// Borrows checked, unmodeled JSON extension fields.
    pub fn extensions(&self) -> &Map<String, Value> {
        &self.extensions
    }
    fn serialize_value<S: Serializer>(
        &self,
        serializer: S,
        tagged: bool,
    ) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        let _ = tagged;
        map.serialize_entry("channel", &self.channel)?;
        if let Some(value) = &self.blocks {
            map.serialize_entry("blocks", value)?;
        }
        if let Some(value) = &self.attachments {
            map.serialize_entry("attachments", value)?;
        }
        if let Some(value) = &self.text {
            map.serialize_entry("text", value)?;
        }
        if let Some(value) = &self.mrkdwn {
            map.serialize_entry("mrkdwn", value)?;
        }
        if let Some(value) = &self.unfurl_links {
            map.serialize_entry("unfurl_links", value)?;
        }
        if let Some(value) = &self.unfurl_media {
            map.serialize_entry("unfurl_media", value)?;
        }
        if let Some(value) = &self.metadata {
            map.serialize_entry("metadata", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl MessagePayloadBuilder {
    /// Sets the ID of the channel, private group, or conversation that receives the message.
    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }
    /// Adds the message blocks in display order.
    /// Replaces the collection; order is preserved.
    pub fn blocks<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Block>,
    {
        self.blocks = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn block(mut self, value: impl Into<Block>) -> Self {
        self.blocks.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `blocks`, including any model default.
    pub fn clear_blocks(mut self) -> Self {
        self.blocks = None;
        self
    }
    /// Adds legacy attachments in display order.
    /// Replaces the collection; order is preserved.
    pub fn attachments<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Attachment>,
    {
        self.attachments = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn attachment(mut self, value: impl Into<Attachment>) -> Self {
        self.attachments
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `attachments`, including any model default.
    pub fn clear_attachments(mut self) -> Self {
        self.attachments = None;
        self
    }
    /// Sets the fallback text used in notifications and by clients that cannot display blocks.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Omits `text`, including any model default.
    pub fn clear_text(mut self) -> Self {
        self.text = None;
        self
    }
    /// Sets whether Slack formats the top-level text as mrkdwn.
    pub fn mrkdwn(mut self, value: bool) -> Self {
        self.mrkdwn = Some(value);
        self
    }
    /// Omits `mrkdwn`, including any model default.
    pub fn clear_mrkdwn(mut self) -> Self {
        self.mrkdwn = None;
        self
    }
    /// Sets whether Slack unfurls text-based links.
    pub fn unfurl_links(mut self, value: bool) -> Self {
        self.unfurl_links = Some(value);
        self
    }
    /// Omits `unfurl_links`, including any model default.
    pub fn clear_unfurl_links(mut self) -> Self {
        self.unfurl_links = None;
        self
    }
    /// Sets whether Slack unfurls media links.
    pub fn unfurl_media(mut self, value: bool) -> Self {
        self.unfurl_media = Some(value);
        self
    }
    /// Omits `unfurl_media`, including any model default.
    pub fn clear_unfurl_media(mut self) -> Self {
        self.unfurl_media = None;
        self
    }
    /// Sets message metadata as a JSON-compatible map, such as event_type and event_payload.
    pub fn metadata(mut self, value: Map<String, Value>) -> Self {
        self.metadata = Some(value);
        self
    }
    /// Omits `metadata`, including any model default.
    pub fn clear_metadata(mut self) -> Self {
        self.metadata = None;
        self
    }
    /// Adds or replaces an unmodeled extension; reserved names fail at build time.
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
    /// Validates all fields and their receiving context.
    ///
    /// # Errors
    /// Returns a [`ValidationError`] for missing fields, invalid values, reserved
    /// extension names, or unsupported field combinations and child contexts.
    pub fn build(self) -> Result<MessagePayload, ValidationError> {
        let channel = wire::required(self.channel, "MessagePayload.channel")?;
        let blocks = self.blocks;
        let attachments = self.attachments;
        let text = self.text;
        let mrkdwn = self.mrkdwn;
        let unfurl_links = self.unfurl_links;
        let unfurl_media = self.unfurl_media;
        let metadata = self.metadata;
        wire::extensions(
            &self.extensions,
            &[
                "channel",
                "blocks",
                "attachments",
                "text",
                "mrkdwn",
                "unfurl_links",
                "unfurl_media",
                "metadata",
            ],
            "MessagePayload",
        )?;
        let value = MessagePayload {
            channel,
            blocks,
            attachments,
            text,
            mrkdwn,
            unfurl_links,
            unfurl_media,
            metadata,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "MessagePayload", e.to_string())
        })?;
        if let Some(v) = wire.get("channel") {
            crate::rules::limits(v, "message.channel", "MessagePayload.channel")?;
        }
        if let Some(v) = wire.get("blocks") {
            crate::rules::limits(v, "message.blocks", "MessagePayload.blocks")?;
        }
        if let Some(v) = wire.get("attachments") {
            crate::rules::limits(v, "message.attachments", "MessagePayload.attachments")?;
        }
        crate::rules::validate("MessagePayload", &wire, "MessagePayload")?;
        Ok(value)
    }
}
impl Serialize for MessagePayload {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for MessagePayload {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let channel = wire::field::<String>(&mut map, "channel", path, false)?;
        let blocks = wire::field::<Vec<Block>>(&mut map, "blocks", path, false)?;
        let attachments = wire::field::<Vec<Attachment>>(&mut map, "attachments", path, false)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let mrkdwn = wire::field::<bool>(&mut map, "mrkdwn", path, false)?;
        let unfurl_links = wire::field::<bool>(&mut map, "unfurl_links", path, false)?;
        let unfurl_media = wire::field::<bool>(&mut map, "unfurl_media", path, false)?;
        let metadata = wire::field::<Map<String, Value>>(&mut map, "metadata", path, false)?;
        MessagePayloadBuilder {
            channel,
            blocks,
            attachments,
            text,
            mrkdwn,
            unfurl_links,
            unfurl_media,
            metadata,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for MessagePayload {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "MessagePayload")
    }
}
impl<'de> Deserialize<'de> for MessagePayload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
