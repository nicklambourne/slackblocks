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
    /// A block that holds interactive elements such as buttons and menus.
    Actions(Box<ActionsBlock>),
    /// A severity-labelled alert, available in modals.
    Alert(Box<AlertBlock>),
    /// A compact card with an image, text, and up to three buttons.
    Card(Box<CardBlock>),
    /// A horizontally scrolling set of cards, available in messages and App Home.
    Carousel(Box<CarouselBlock>),
    /// A titled group of blocks that can optionally collapse.
    Container(Box<ContainerBlock>),
    /// A row of feedback or icon buttons, usually shown below AI-generated content.
    ContextActions(Box<ContextActionsBlock>),
    /// A block of small, secondary images and text.
    Context(Box<ContextBlock>),
    /// A sortable, paginated table of raw text, numbers, and rich text.
    DataTable(Box<DataTableBlock>),
    /// A titled pie, bar, area, or line chart.
    DataVisualization(Box<DataVisualizationBlock>),
    /// A horizontal rule that separates blocks.
    Divider(Box<DividerBlock>),
    /// A remote file previously added with the files.remote API.
    File(Box<FileBlock>),
    /// Large, bold plain text that introduces a group of blocks.
    Header(Box<HeaderBlock>),
    /// A standalone image with alternative text and an optional title.
    Image(Box<ImageBlock>),
    /// A labelled input that collects a value in a modal, App Home, or message.
    Input(Box<InputBlock>),
    /// Standard markdown rendered by Slack, intended for AI-generated content.
    Markdown(Box<MarkdownBlock>),
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
    /// An embedded video player.
    Video(Box<VideoBlock>),
}
impl Serialize for Block {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Actions(v) => v.serialize(s),
            Self::Alert(v) => v.serialize(s),
            Self::Card(v) => v.serialize(s),
            Self::Carousel(v) => v.serialize(s),
            Self::Container(v) => v.serialize(s),
            Self::ContextActions(v) => v.serialize(s),
            Self::Context(v) => v.serialize(s),
            Self::DataTable(v) => v.serialize(s),
            Self::DataVisualization(v) => v.serialize(s),
            Self::Divider(v) => v.serialize(s),
            Self::File(v) => v.serialize(s),
            Self::Header(v) => v.serialize(s),
            Self::Image(v) => v.serialize(s),
            Self::Input(v) => v.serialize(s),
            Self::Markdown(v) => v.serialize(s),
            Self::Plan(v) => v.serialize(s),
            Self::RichText(v) => v.serialize(s),
            Self::Section(v) => v.serialize(s),
            Self::Table(v) => v.serialize(s),
            Self::TaskCard(v) => v.serialize(s),
            Self::Video(v) => v.serialize(s),
        }
    }
}
impl FromWire for Block {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("actions") => ActionsBlock::from_wire(value, path).map(Into::into),
            Some("alert") => AlertBlock::from_wire(value, path).map(Into::into),
            Some("card") => CardBlock::from_wire(value, path).map(Into::into),
            Some("carousel") => CarouselBlock::from_wire(value, path).map(Into::into),
            Some("container") => ContainerBlock::from_wire(value, path).map(Into::into),
            Some("context_actions") => ContextActionsBlock::from_wire(value, path).map(Into::into),
            Some("context") => ContextBlock::from_wire(value, path).map(Into::into),
            Some("data_table") => DataTableBlock::from_wire(value, path).map(Into::into),
            Some("data_visualization") => {
                DataVisualizationBlock::from_wire(value, path).map(Into::into)
            }
            Some("divider") => DividerBlock::from_wire(value, path).map(Into::into),
            Some("file") => FileBlock::from_wire(value, path).map(Into::into),
            Some("header") => HeaderBlock::from_wire(value, path).map(Into::into),
            Some("image") => ImageBlock::from_wire(value, path).map(Into::into),
            Some("input") => InputBlock::from_wire(value, path).map(Into::into),
            Some("markdown") => MarkdownBlock::from_wire(value, path).map(Into::into),
            Some("plan") => PlanBlock::from_wire(value, path).map(Into::into),
            Some("rich_text") => RichTextBlock::from_wire(value, path).map(Into::into),
            Some("section") => SectionBlock::from_wire(value, path).map(Into::into),
            Some("table") => TableBlock::from_wire(value, path).map(Into::into),
            Some("task_card") => TaskCardBlock::from_wire(value, path).map(Into::into),
            Some("video") => VideoBlock::from_wire(value, path).map(Into::into),
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

impl From<ActionsBlock> for Block {
    fn from(value: ActionsBlock) -> Self {
        Self::Actions(Box::new(value))
    }
}
impl From<AlertBlock> for Block {
    fn from(value: AlertBlock) -> Self {
        Self::Alert(Box::new(value))
    }
}
impl From<CardBlock> for Block {
    fn from(value: CardBlock) -> Self {
        Self::Card(Box::new(value))
    }
}
impl From<CarouselBlock> for Block {
    fn from(value: CarouselBlock) -> Self {
        Self::Carousel(Box::new(value))
    }
}
impl From<ContainerBlock> for Block {
    fn from(value: ContainerBlock) -> Self {
        Self::Container(Box::new(value))
    }
}
impl From<ContextActionsBlock> for Block {
    fn from(value: ContextActionsBlock) -> Self {
        Self::ContextActions(Box::new(value))
    }
}
impl From<ContextBlock> for Block {
    fn from(value: ContextBlock) -> Self {
        Self::Context(Box::new(value))
    }
}
impl From<DataTableBlock> for Block {
    fn from(value: DataTableBlock) -> Self {
        Self::DataTable(Box::new(value))
    }
}
impl From<DataVisualizationBlock> for Block {
    fn from(value: DataVisualizationBlock) -> Self {
        Self::DataVisualization(Box::new(value))
    }
}
impl From<DividerBlock> for Block {
    fn from(value: DividerBlock) -> Self {
        Self::Divider(Box::new(value))
    }
}
impl From<FileBlock> for Block {
    fn from(value: FileBlock) -> Self {
        Self::File(Box::new(value))
    }
}
impl From<HeaderBlock> for Block {
    fn from(value: HeaderBlock) -> Self {
        Self::Header(Box::new(value))
    }
}
impl From<ImageBlock> for Block {
    fn from(value: ImageBlock) -> Self {
        Self::Image(Box::new(value))
    }
}
impl From<InputBlock> for Block {
    fn from(value: InputBlock) -> Self {
        Self::Input(Box::new(value))
    }
}
impl From<MarkdownBlock> for Block {
    fn from(value: MarkdownBlock) -> Self {
        Self::Markdown(Box::new(value))
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
impl From<VideoBlock> for Block {
    fn from(value: VideoBlock) -> Self {
        Self::Video(Box::new(value))
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
/// A value that can appear in a context actions block: feedback buttons or an icon button.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ContextActionsElement {
    /// A pair of thumbs-up and thumbs-down feedback buttons.
    FeedbackButtons(Box<FeedbackButtonsElement>),
    /// A button that shows an icon instead of text.
    IconButton(Box<IconButtonElement>),
}
impl Serialize for ContextActionsElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FeedbackButtons(v) => v.serialize(s),
            Self::IconButton(v) => v.serialize(s),
        }
    }
}
impl FromWire for ContextActionsElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("feedback_buttons") => {
                FeedbackButtonsElement::from_wire(value, path).map(Into::into)
            }
            Some("icon_button") => IconButtonElement::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a ContextActionsElement discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for ContextActionsElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ContextActionsElement")
    }
}
impl<'de> Deserialize<'de> for ContextActionsElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<FeedbackButtonsElement> for ContextActionsElement {
    fn from(value: FeedbackButtonsElement) -> Self {
        Self::FeedbackButtons(Box::new(value))
    }
}
impl From<IconButtonElement> for ContextActionsElement {
    fn from(value: IconButtonElement) -> Self {
        Self::IconButton(Box::new(value))
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
    /// A menu for selecting multiple public channels.
    ChannelMultiSelect(Box<ChannelMultiSelectElement>),
    /// A menu for selecting one public channel.
    ChannelSelect(Box<ChannelSelectElement>),
    /// A group of checkboxes.
    Checkboxes(Box<CheckboxesElement>),
    /// A menu for selecting multiple conversations.
    ConversationMultiSelect(Box<ConversationMultiSelectElement>),
    /// A menu for selecting one conversation.
    ConversationSelect(Box<ConversationSelectElement>),
    /// A calendar date picker.
    DatePicker(Box<DatePickerElement>),
    /// A combined date and time picker.
    DateTimePicker(Box<DateTimePickerElement>),
    /// A single-line input that accepts an email address.
    EmailInput(Box<EmailInputElement>),
    /// A menu for selecting multiple options loaded from your app.
    ExternalMultiSelect(Box<ExternalMultiSelectElement>),
    /// A menu for selecting one option loaded from your app.
    ExternalSelect(Box<ExternalSelectElement>),
    /// A pair of thumbs-up and thumbs-down feedback buttons.
    FeedbackButtons(Box<FeedbackButtonsElement>),
    /// An input that lets users upload files.
    FileInput(Box<FileInputElement>),
    /// A button that shows an icon instead of text.
    IconButton(Box<IconButtonElement>),
    /// An image displayed inside a section, context, or card.
    Image(Box<ImageElement>),
    /// An input that accepts whole or decimal numbers.
    NumberInput(Box<NumberInputElement>),
    /// A compact menu of up to five options.
    Overflow(Box<OverflowElement>),
    /// A single-line or multi-line free-text input.
    PlainTextInput(Box<PlainTextInputElement>),
    /// A group of radio buttons.
    RadioButtons(Box<RadioButtonsElement>),
    /// An input that accepts formatted rich text.
    RichTextInput(Box<RichTextInputElement>),
    /// A menu for selecting multiple options defined in the payload.
    StaticMultiSelect(Box<StaticMultiSelectElement>),
    /// A menu for selecting one option defined in the payload.
    StaticSelect(Box<StaticSelectElement>),
    /// A time-of-day picker.
    TimePicker(Box<TimePickerElement>),
    /// A single-line input that accepts a URL.
    UrlInput(Box<UrlInputElement>),
    /// A menu for selecting multiple workspace users.
    UserMultiSelect(Box<UserMultiSelectElement>),
    /// A menu for selecting one workspace user.
    UserSelect(Box<UserSelectElement>),
    /// A button that starts a workflow through a link trigger.
    WorkflowButton(Box<WorkflowButtonElement>),
}
impl Serialize for Element {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Button(v) => v.serialize(s),
            Self::ChannelMultiSelect(v) => v.serialize(s),
            Self::ChannelSelect(v) => v.serialize(s),
            Self::Checkboxes(v) => v.serialize(s),
            Self::ConversationMultiSelect(v) => v.serialize(s),
            Self::ConversationSelect(v) => v.serialize(s),
            Self::DatePicker(v) => v.serialize(s),
            Self::DateTimePicker(v) => v.serialize(s),
            Self::EmailInput(v) => v.serialize(s),
            Self::ExternalMultiSelect(v) => v.serialize(s),
            Self::ExternalSelect(v) => v.serialize(s),
            Self::FeedbackButtons(v) => v.serialize(s),
            Self::FileInput(v) => v.serialize(s),
            Self::IconButton(v) => v.serialize(s),
            Self::Image(v) => v.serialize(s),
            Self::NumberInput(v) => v.serialize(s),
            Self::Overflow(v) => v.serialize(s),
            Self::PlainTextInput(v) => v.serialize(s),
            Self::RadioButtons(v) => v.serialize(s),
            Self::RichTextInput(v) => v.serialize(s),
            Self::StaticMultiSelect(v) => v.serialize(s),
            Self::StaticSelect(v) => v.serialize(s),
            Self::TimePicker(v) => v.serialize(s),
            Self::UrlInput(v) => v.serialize(s),
            Self::UserMultiSelect(v) => v.serialize(s),
            Self::UserSelect(v) => v.serialize(s),
            Self::WorkflowButton(v) => v.serialize(s),
        }
    }
}
impl FromWire for Element {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("button") => ButtonElement::from_wire(value, path).map(Into::into),
            Some("multi_channels_select") => {
                ChannelMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("channels_select") => ChannelSelectElement::from_wire(value, path).map(Into::into),
            Some("checkboxes") => CheckboxesElement::from_wire(value, path).map(Into::into),
            Some("multi_conversations_select") => {
                ConversationMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("conversations_select") => {
                ConversationSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("datepicker") => DatePickerElement::from_wire(value, path).map(Into::into),
            Some("datetimepicker") => DateTimePickerElement::from_wire(value, path).map(Into::into),
            Some("email_text_input") => EmailInputElement::from_wire(value, path).map(Into::into),
            Some("multi_external_select") => {
                ExternalMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("external_select") => {
                ExternalSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("feedback_buttons") => {
                FeedbackButtonsElement::from_wire(value, path).map(Into::into)
            }
            Some("file_input") => FileInputElement::from_wire(value, path).map(Into::into),
            Some("icon_button") => IconButtonElement::from_wire(value, path).map(Into::into),
            Some("image") => ImageElement::from_wire(value, path).map(Into::into),
            Some("number_input") => NumberInputElement::from_wire(value, path).map(Into::into),
            Some("overflow") => OverflowElement::from_wire(value, path).map(Into::into),
            Some("plain_text_input") => {
                PlainTextInputElement::from_wire(value, path).map(Into::into)
            }
            Some("radio_buttons") => RadioButtonsElement::from_wire(value, path).map(Into::into),
            Some("rich_text_input") => RichTextInputElement::from_wire(value, path).map(Into::into),
            Some("multi_static_select") => {
                StaticMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("static_select") => StaticSelectElement::from_wire(value, path).map(Into::into),
            Some("timepicker") => TimePickerElement::from_wire(value, path).map(Into::into),
            Some("url_text_input") => UrlInputElement::from_wire(value, path).map(Into::into),
            Some("multi_users_select") => {
                UserMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("users_select") => UserSelectElement::from_wire(value, path).map(Into::into),
            Some("workflow_button") => {
                WorkflowButtonElement::from_wire(value, path).map(Into::into)
            }
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
impl From<ChannelMultiSelectElement> for Element {
    fn from(value: ChannelMultiSelectElement) -> Self {
        Self::ChannelMultiSelect(Box::new(value))
    }
}
impl From<ChannelSelectElement> for Element {
    fn from(value: ChannelSelectElement) -> Self {
        Self::ChannelSelect(Box::new(value))
    }
}
impl From<CheckboxesElement> for Element {
    fn from(value: CheckboxesElement) -> Self {
        Self::Checkboxes(Box::new(value))
    }
}
impl From<ConversationMultiSelectElement> for Element {
    fn from(value: ConversationMultiSelectElement) -> Self {
        Self::ConversationMultiSelect(Box::new(value))
    }
}
impl From<ConversationSelectElement> for Element {
    fn from(value: ConversationSelectElement) -> Self {
        Self::ConversationSelect(Box::new(value))
    }
}
impl From<DatePickerElement> for Element {
    fn from(value: DatePickerElement) -> Self {
        Self::DatePicker(Box::new(value))
    }
}
impl From<DateTimePickerElement> for Element {
    fn from(value: DateTimePickerElement) -> Self {
        Self::DateTimePicker(Box::new(value))
    }
}
impl From<EmailInputElement> for Element {
    fn from(value: EmailInputElement) -> Self {
        Self::EmailInput(Box::new(value))
    }
}
impl From<ExternalMultiSelectElement> for Element {
    fn from(value: ExternalMultiSelectElement) -> Self {
        Self::ExternalMultiSelect(Box::new(value))
    }
}
impl From<ExternalSelectElement> for Element {
    fn from(value: ExternalSelectElement) -> Self {
        Self::ExternalSelect(Box::new(value))
    }
}
impl From<FeedbackButtonsElement> for Element {
    fn from(value: FeedbackButtonsElement) -> Self {
        Self::FeedbackButtons(Box::new(value))
    }
}
impl From<FileInputElement> for Element {
    fn from(value: FileInputElement) -> Self {
        Self::FileInput(Box::new(value))
    }
}
impl From<IconButtonElement> for Element {
    fn from(value: IconButtonElement) -> Self {
        Self::IconButton(Box::new(value))
    }
}
impl From<ImageElement> for Element {
    fn from(value: ImageElement) -> Self {
        Self::Image(Box::new(value))
    }
}
impl From<NumberInputElement> for Element {
    fn from(value: NumberInputElement) -> Self {
        Self::NumberInput(Box::new(value))
    }
}
impl From<OverflowElement> for Element {
    fn from(value: OverflowElement) -> Self {
        Self::Overflow(Box::new(value))
    }
}
impl From<PlainTextInputElement> for Element {
    fn from(value: PlainTextInputElement) -> Self {
        Self::PlainTextInput(Box::new(value))
    }
}
impl From<RadioButtonsElement> for Element {
    fn from(value: RadioButtonsElement) -> Self {
        Self::RadioButtons(Box::new(value))
    }
}
impl From<RichTextInputElement> for Element {
    fn from(value: RichTextInputElement) -> Self {
        Self::RichTextInput(Box::new(value))
    }
}
impl From<StaticMultiSelectElement> for Element {
    fn from(value: StaticMultiSelectElement) -> Self {
        Self::StaticMultiSelect(Box::new(value))
    }
}
impl From<StaticSelectElement> for Element {
    fn from(value: StaticSelectElement) -> Self {
        Self::StaticSelect(Box::new(value))
    }
}
impl From<TimePickerElement> for Element {
    fn from(value: TimePickerElement) -> Self {
        Self::TimePicker(Box::new(value))
    }
}
impl From<UrlInputElement> for Element {
    fn from(value: UrlInputElement) -> Self {
        Self::UrlInput(Box::new(value))
    }
}
impl From<UserMultiSelectElement> for Element {
    fn from(value: UserMultiSelectElement) -> Self {
        Self::UserMultiSelect(Box::new(value))
    }
}
impl From<UserSelectElement> for Element {
    fn from(value: UserSelectElement) -> Self {
        Self::UserSelect(Box::new(value))
    }
}
impl From<WorkflowButtonElement> for Element {
    fn from(value: WorkflowButtonElement) -> Self {
        Self::WorkflowButton(Box::new(value))
    }
}
/// An element that can be placed in an input block.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum InputElement {
    /// A menu for selecting multiple public channels.
    ChannelMultiSelect(Box<ChannelMultiSelectElement>),
    /// A menu for selecting one public channel.
    ChannelSelect(Box<ChannelSelectElement>),
    /// A group of checkboxes.
    Checkboxes(Box<CheckboxesElement>),
    /// A menu for selecting multiple conversations.
    ConversationMultiSelect(Box<ConversationMultiSelectElement>),
    /// A menu for selecting one conversation.
    ConversationSelect(Box<ConversationSelectElement>),
    /// A calendar date picker.
    DatePicker(Box<DatePickerElement>),
    /// A combined date and time picker.
    DateTimePicker(Box<DateTimePickerElement>),
    /// A single-line input that accepts an email address.
    EmailInput(Box<EmailInputElement>),
    /// A menu for selecting multiple options loaded from your app.
    ExternalMultiSelect(Box<ExternalMultiSelectElement>),
    /// A menu for selecting one option loaded from your app.
    ExternalSelect(Box<ExternalSelectElement>),
    /// An input that lets users upload files.
    FileInput(Box<FileInputElement>),
    /// An input that accepts whole or decimal numbers.
    NumberInput(Box<NumberInputElement>),
    /// A single-line or multi-line free-text input.
    PlainTextInput(Box<PlainTextInputElement>),
    /// A group of radio buttons.
    RadioButtons(Box<RadioButtonsElement>),
    /// An input that accepts formatted rich text.
    RichTextInput(Box<RichTextInputElement>),
    /// A menu for selecting multiple options defined in the payload.
    StaticMultiSelect(Box<StaticMultiSelectElement>),
    /// A menu for selecting one option defined in the payload.
    StaticSelect(Box<StaticSelectElement>),
    /// A time-of-day picker.
    TimePicker(Box<TimePickerElement>),
    /// A single-line input that accepts a URL.
    UrlInput(Box<UrlInputElement>),
    /// A menu for selecting multiple workspace users.
    UserMultiSelect(Box<UserMultiSelectElement>),
    /// A menu for selecting one workspace user.
    UserSelect(Box<UserSelectElement>),
}
impl Serialize for InputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ChannelMultiSelect(v) => v.serialize(s),
            Self::ChannelSelect(v) => v.serialize(s),
            Self::Checkboxes(v) => v.serialize(s),
            Self::ConversationMultiSelect(v) => v.serialize(s),
            Self::ConversationSelect(v) => v.serialize(s),
            Self::DatePicker(v) => v.serialize(s),
            Self::DateTimePicker(v) => v.serialize(s),
            Self::EmailInput(v) => v.serialize(s),
            Self::ExternalMultiSelect(v) => v.serialize(s),
            Self::ExternalSelect(v) => v.serialize(s),
            Self::FileInput(v) => v.serialize(s),
            Self::NumberInput(v) => v.serialize(s),
            Self::PlainTextInput(v) => v.serialize(s),
            Self::RadioButtons(v) => v.serialize(s),
            Self::RichTextInput(v) => v.serialize(s),
            Self::StaticMultiSelect(v) => v.serialize(s),
            Self::StaticSelect(v) => v.serialize(s),
            Self::TimePicker(v) => v.serialize(s),
            Self::UrlInput(v) => v.serialize(s),
            Self::UserMultiSelect(v) => v.serialize(s),
            Self::UserSelect(v) => v.serialize(s),
        }
    }
}
impl FromWire for InputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("multi_channels_select") => {
                ChannelMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("channels_select") => ChannelSelectElement::from_wire(value, path).map(Into::into),
            Some("checkboxes") => CheckboxesElement::from_wire(value, path).map(Into::into),
            Some("multi_conversations_select") => {
                ConversationMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("conversations_select") => {
                ConversationSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("datepicker") => DatePickerElement::from_wire(value, path).map(Into::into),
            Some("datetimepicker") => DateTimePickerElement::from_wire(value, path).map(Into::into),
            Some("email_text_input") => EmailInputElement::from_wire(value, path).map(Into::into),
            Some("multi_external_select") => {
                ExternalMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("external_select") => {
                ExternalSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("file_input") => FileInputElement::from_wire(value, path).map(Into::into),
            Some("number_input") => NumberInputElement::from_wire(value, path).map(Into::into),
            Some("plain_text_input") => {
                PlainTextInputElement::from_wire(value, path).map(Into::into)
            }
            Some("radio_buttons") => RadioButtonsElement::from_wire(value, path).map(Into::into),
            Some("rich_text_input") => RichTextInputElement::from_wire(value, path).map(Into::into),
            Some("multi_static_select") => {
                StaticMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("static_select") => StaticSelectElement::from_wire(value, path).map(Into::into),
            Some("timepicker") => TimePickerElement::from_wire(value, path).map(Into::into),
            Some("url_text_input") => UrlInputElement::from_wire(value, path).map(Into::into),
            Some("multi_users_select") => {
                UserMultiSelectElement::from_wire(value, path).map(Into::into)
            }
            Some("users_select") => UserSelectElement::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a InputElement discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for InputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "InputElement")
    }
}
impl<'de> Deserialize<'de> for InputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<ChannelMultiSelectElement> for InputElement {
    fn from(value: ChannelMultiSelectElement) -> Self {
        Self::ChannelMultiSelect(Box::new(value))
    }
}
impl From<ChannelSelectElement> for InputElement {
    fn from(value: ChannelSelectElement) -> Self {
        Self::ChannelSelect(Box::new(value))
    }
}
impl From<CheckboxesElement> for InputElement {
    fn from(value: CheckboxesElement) -> Self {
        Self::Checkboxes(Box::new(value))
    }
}
impl From<ConversationMultiSelectElement> for InputElement {
    fn from(value: ConversationMultiSelectElement) -> Self {
        Self::ConversationMultiSelect(Box::new(value))
    }
}
impl From<ConversationSelectElement> for InputElement {
    fn from(value: ConversationSelectElement) -> Self {
        Self::ConversationSelect(Box::new(value))
    }
}
impl From<DatePickerElement> for InputElement {
    fn from(value: DatePickerElement) -> Self {
        Self::DatePicker(Box::new(value))
    }
}
impl From<DateTimePickerElement> for InputElement {
    fn from(value: DateTimePickerElement) -> Self {
        Self::DateTimePicker(Box::new(value))
    }
}
impl From<EmailInputElement> for InputElement {
    fn from(value: EmailInputElement) -> Self {
        Self::EmailInput(Box::new(value))
    }
}
impl From<ExternalMultiSelectElement> for InputElement {
    fn from(value: ExternalMultiSelectElement) -> Self {
        Self::ExternalMultiSelect(Box::new(value))
    }
}
impl From<ExternalSelectElement> for InputElement {
    fn from(value: ExternalSelectElement) -> Self {
        Self::ExternalSelect(Box::new(value))
    }
}
impl From<FileInputElement> for InputElement {
    fn from(value: FileInputElement) -> Self {
        Self::FileInput(Box::new(value))
    }
}
impl From<NumberInputElement> for InputElement {
    fn from(value: NumberInputElement) -> Self {
        Self::NumberInput(Box::new(value))
    }
}
impl From<PlainTextInputElement> for InputElement {
    fn from(value: PlainTextInputElement) -> Self {
        Self::PlainTextInput(Box::new(value))
    }
}
impl From<RadioButtonsElement> for InputElement {
    fn from(value: RadioButtonsElement) -> Self {
        Self::RadioButtons(Box::new(value))
    }
}
impl From<RichTextInputElement> for InputElement {
    fn from(value: RichTextInputElement) -> Self {
        Self::RichTextInput(Box::new(value))
    }
}
impl From<StaticMultiSelectElement> for InputElement {
    fn from(value: StaticMultiSelectElement) -> Self {
        Self::StaticMultiSelect(Box::new(value))
    }
}
impl From<StaticSelectElement> for InputElement {
    fn from(value: StaticSelectElement) -> Self {
        Self::StaticSelect(Box::new(value))
    }
}
impl From<TimePickerElement> for InputElement {
    fn from(value: TimePickerElement) -> Self {
        Self::TimePicker(Box::new(value))
    }
}
impl From<UrlInputElement> for InputElement {
    fn from(value: UrlInputElement) -> Self {
        Self::UrlInput(Box::new(value))
    }
}
impl From<UserMultiSelectElement> for InputElement {
    fn from(value: UserMultiSelectElement) -> Self {
        Self::UserMultiSelect(Box::new(value))
    }
}
impl From<UserSelectElement> for InputElement {
    fn from(value: UserSelectElement) -> Self {
        Self::UserSelect(Box::new(value))
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
/// A chart that can be displayed by a data visualization block.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Chart {
    /// An area chart for a data visualization block.
    Area(AreaChart),
    /// A bar chart for a data visualization block.
    Bar(BarChart),
    /// A line chart for a data visualization block.
    Line(LineChart),
    /// A pie chart for a data visualization block.
    Pie(PieChart),
}
impl Serialize for Chart {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Area(v) => v.serialize(s),
            Self::Bar(v) => v.serialize(s),
            Self::Line(v) => v.serialize(s),
            Self::Pie(v) => v.serialize(s),
        }
    }
}
impl FromWire for Chart {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("area") => AreaChart::from_wire(value, path).map(Into::into),
            Some("bar") => BarChart::from_wire(value, path).map(Into::into),
            Some("line") => LineChart::from_wire(value, path).map(Into::into),
            Some("pie") => PieChart::from_wire(value, path).map(Into::into),
            _ => Err(wire::mismatch(
                &format!("{path}.type"),
                "a Chart discriminator",
            )),
        }
    }
}
impl TryFrom<Value> for Chart {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Chart")
    }
}
impl<'de> Deserialize<'de> for Chart {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl From<AreaChart> for Chart {
    fn from(value: AreaChart) -> Self {
        Self::Area(value)
    }
}
impl From<BarChart> for Chart {
    fn from(value: BarChart) -> Self {
        Self::Bar(value)
    }
}
impl From<LineChart> for Chart {
    fn from(value: LineChart) -> Self {
        Self::Line(value)
    }
}
impl From<PieChart> for Chart {
    fn from(value: PieChart) -> Self {
        Self::Pie(value)
    }
}
/// A top-level element of a rich text block: a section, list, preformatted block, or quote.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum RichTextBlockElement {
    /// A preformatted code block inside rich text.
    CodeBlock(RichTextCodeBlock),
    /// A bulleted or numbered list inside rich text.
    List(RichTextList),
    /// A quotation inside rich text.
    Quote(RichTextQuote),
    /// A paragraph of inline rich text elements.
    Section(RichTextSection),
}
impl Serialize for RichTextBlockElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CodeBlock(v) => v.serialize(s),
            Self::List(v) => v.serialize(s),
            Self::Quote(v) => v.serialize(s),
            Self::Section(v) => v.serialize(s),
        }
    }
}
impl FromWire for RichTextBlockElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("rich_text_preformatted") => {
                RichTextCodeBlock::from_wire(value, path).map(Into::into)
            }
            Some("rich_text_list") => RichTextList::from_wire(value, path).map(Into::into),
            Some("rich_text_quote") => RichTextQuote::from_wire(value, path).map(Into::into),
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

impl From<RichTextCodeBlock> for RichTextBlockElement {
    fn from(value: RichTextCodeBlock) -> Self {
        Self::CodeBlock(value)
    }
}
impl From<RichTextList> for RichTextBlockElement {
    fn from(value: RichTextList) -> Self {
        Self::List(value)
    }
}
impl From<RichTextQuote> for RichTextBlockElement {
    fn from(value: RichTextQuote) -> Self {
        Self::Quote(value)
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
    /// A channel mention inside rich text.
    Channel(RichTextChannel),
    /// An emoji inside rich text.
    Emoji(RichTextEmoji),
    /// A hyperlink inside rich text.
    Link(RichTextLink),
    /// A run of optionally styled text inside rich text.
    Text(RichTextText),
    /// A user mention inside rich text.
    User(RichTextUser),
    /// A user group mention inside rich text.
    UserGroup(RichTextUserGroup),
}
impl Serialize for RichTextSectionElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Channel(v) => v.serialize(s),
            Self::Emoji(v) => v.serialize(s),
            Self::Link(v) => v.serialize(s),
            Self::Text(v) => v.serialize(s),
            Self::User(v) => v.serialize(s),
            Self::UserGroup(v) => v.serialize(s),
        }
    }
}
impl FromWire for RichTextSectionElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value.get("type").and_then(Value::as_str) {
            Some("channel") => RichTextChannel::from_wire(value, path).map(Into::into),
            Some("emoji") => RichTextEmoji::from_wire(value, path).map(Into::into),
            Some("link") => RichTextLink::from_wire(value, path).map(Into::into),
            Some("text") => RichTextText::from_wire(value, path).map(Into::into),
            Some("user") => RichTextUser::from_wire(value, path).map(Into::into),
            Some("usergroup") => RichTextUserGroup::from_wire(value, path).map(Into::into),
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

impl From<RichTextChannel> for RichTextSectionElement {
    fn from(value: RichTextChannel) -> Self {
        Self::Channel(value)
    }
}
impl From<RichTextEmoji> for RichTextSectionElement {
    fn from(value: RichTextEmoji) -> Self {
        Self::Emoji(value)
    }
}
impl From<RichTextLink> for RichTextSectionElement {
    fn from(value: RichTextLink) -> Self {
        Self::Link(value)
    }
}
impl From<RichTextText> for RichTextSectionElement {
    fn from(value: RichTextText) -> Self {
        Self::Text(value)
    }
}
impl From<RichTextUser> for RichTextSectionElement {
    fn from(value: RichTextUser) -> Self {
        Self::User(value)
    }
}
impl From<RichTextUserGroup> for RichTextSectionElement {
    fn from(value: RichTextUserGroup) -> Self {
        Self::UserGroup(value)
    }
}
/// A block that holds interactive elements such as buttons and menus.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/actions-block).

#[derive(Clone, Debug, PartialEq)]
pub struct ActionsBlock {
    elements: Vec<Element>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ActionsBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ActionsBlockBuilder {
    elements: Option<Vec<Element>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for ActionsBlockBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl ActionsBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ActionsBlockBuilder {
        ActionsBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ActionsBlockBuilder {
        ActionsBlockBuilder {
            elements: Some(self.elements),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds interactive elements in display order.
    pub fn elements(&self) -> &[Element] {
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
            map.serialize_entry("type", "actions")?;
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
impl ActionsBlockBuilder {
    /// Adds interactive elements in display order.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Element>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<Element>) -> Self {
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
    pub fn build(self) -> Result<ActionsBlock, ValidationError> {
        let elements = wire::required(self.elements, "ActionsBlock.elements")?;
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["elements", "block_id"], "ActionsBlock")?;
        let value = ActionsBlock {
            elements,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ActionsBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("elements") {
            crate::rules::limits(v, "actions.elements", "ActionsBlock.elements")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "ActionsBlock.block_id")?;
        }
        crate::rules::validate("ActionsBlock", &wire, "ActionsBlock")?;
        Ok(value)
    }
}
impl Serialize for ActionsBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ActionsBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "actions", path)?;
        let elements = wire::field::<Vec<Element>>(&mut map, "elements", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        ActionsBlockBuilder {
            elements,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ActionsBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ActionsBlock")
    }
}
impl<'de> Deserialize<'de> for ActionsBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A severity-labelled alert, available in modals.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/alert-block).

#[derive(Clone, Debug, PartialEq)]
pub struct AlertBlock {
    text: Text,
    level: Option<AlertLevel>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`AlertBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct AlertBlockBuilder {
    text: Option<TextInput>,
    level: Option<AlertLevel>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for AlertBlockBuilder {
    fn default() -> Self {
        Self {
            text: None,
            level: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl AlertBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> AlertBlockBuilder {
        AlertBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> AlertBlockBuilder {
        AlertBlockBuilder {
            text: Some(self.text.into()),
            level: self.level,
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the alert message.
    pub fn text(&self) -> &Text {
        &self.text
    }
    /// Borrows or copies `level`. Sets the alert severity, which controls its color and icon.
    pub fn level(&self) -> Option<AlertLevel> {
        self.level
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
            map.serialize_entry("type", "alert")?;
        }
        map.serialize_entry("text", &self.text)?;
        if let Some(value) = &self.level {
            map.serialize_entry("level", value)?;
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
impl AlertBlockBuilder {
    /// Sets the alert message.
    pub fn text(mut self, value: impl Into<TextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the alert severity, which controls its color and icon.
    pub fn level(mut self, value: AlertLevel) -> Self {
        self.level = Some(value);
        self
    }
    /// Omits `level`, including any model default.
    pub fn clear_level(mut self) -> Self {
        self.level = None;
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
    pub fn build(self) -> Result<AlertBlock, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve(false).map_err(|e| e.at("AlertBlock.text")))
                .transpose()?,
            "AlertBlock.text",
        )?;
        let level = self.level;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["text", "level", "block_id"],
            "AlertBlock",
        )?;
        let value = AlertBlock {
            text,
            level,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "AlertBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "alert.text", "AlertBlock.text")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "AlertBlock.block_id")?;
        }
        crate::rules::validate("AlertBlock", &wire, "AlertBlock")?;
        Ok(value)
    }
}
impl Serialize for AlertBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for AlertBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "alert", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "mrkdwn");
        let text = wire::field::<Text>(&mut map, "text", path, false)?;
        let level = wire::field::<AlertLevel>(&mut map, "level", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        AlertBlockBuilder {
            text: text.map(Into::into),
            level,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for AlertBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "AlertBlock")
    }
}
impl<'de> Deserialize<'de> for AlertBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A compact card with an image, text, and up to three buttons.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/card-block).
/// Provide at least one of a hero image, title, actions, or body. An image icon and a Slack icon cannot be combined.
#[derive(Clone, Debug, PartialEq)]
pub struct CardBlock {
    hero_image: Option<ImageElement>,
    icon: Option<ImageElement>,
    title: Option<Text>,
    subtitle: Option<Text>,
    body: Option<Text>,
    actions: Option<Vec<ButtonElement>>,
    slack_icon: Option<SlackIcon>,
    subtext: Option<Text>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`CardBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct CardBlockBuilder {
    hero_image: Option<ImageElement>,
    icon: Option<ImageElement>,
    title: Option<TextInput>,
    subtitle: Option<TextInput>,
    body: Option<TextInput>,
    actions: Option<Vec<ButtonElement>>,
    slack_icon: Option<SlackIcon>,
    subtext: Option<TextInput>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for CardBlockBuilder {
    fn default() -> Self {
        Self {
            hero_image: None,
            icon: None,
            title: None,
            subtitle: None,
            body: None,
            actions: None,
            slack_icon: None,
            subtext: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl CardBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> CardBlockBuilder {
        CardBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> CardBlockBuilder {
        CardBlockBuilder {
            hero_image: self.hero_image,
            icon: self.icon,
            title: self.title.map(Into::into),
            subtitle: self.subtitle.map(Into::into),
            body: self.body.map(Into::into),
            actions: self.actions,
            slack_icon: self.slack_icon,
            subtext: self.subtext.map(Into::into),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `hero_image`. Sets the image displayed prominently at the top of the card.
    pub fn hero_image(&self) -> Option<&ImageElement> {
        self.hero_image.as_ref()
    }
    /// Borrows or copies `icon`. Sets the image displayed beside the title.
    pub fn icon(&self) -> Option<&ImageElement> {
        self.icon.as_ref()
    }
    /// Borrows or copies `title`. Sets the card title.
    pub fn title(&self) -> Option<&Text> {
        self.title.as_ref()
    }
    /// Borrows or copies `subtitle`. Sets the supporting text shown below the title.
    pub fn subtitle(&self) -> Option<&Text> {
        self.subtitle.as_ref()
    }
    /// Borrows or copies `body`. Sets the main body text of the card.
    pub fn body(&self) -> Option<&Text> {
        self.body.as_ref()
    }
    /// Borrows or copies `actions`. Adds buttons displayed at the bottom of the card, in display order.
    pub fn actions(&self) -> Option<&[ButtonElement]> {
        self.actions.as_deref()
    }
    /// Borrows or copies `slack_icon`. Sets a Slack-provided icon displayed beside the title. Cannot be combined with an image icon.
    pub fn slack_icon(&self) -> Option<&SlackIcon> {
        self.slack_icon.as_ref()
    }
    /// Borrows or copies `subtext`. Sets the small print shown at the bottom of the card.
    pub fn subtext(&self) -> Option<&Text> {
        self.subtext.as_ref()
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
            map.serialize_entry("type", "card")?;
        }
        if let Some(value) = &self.hero_image {
            map.serialize_entry("hero_image", value)?;
        }
        if let Some(value) = &self.icon {
            map.serialize_entry("icon", value)?;
        }
        if let Some(value) = &self.title {
            map.serialize_entry("title", value)?;
        }
        if let Some(value) = &self.subtitle {
            map.serialize_entry("subtitle", value)?;
        }
        if let Some(value) = &self.body {
            map.serialize_entry("body", value)?;
        }
        if let Some(value) = &self.actions {
            map.serialize_entry("actions", value)?;
        }
        if let Some(value) = &self.slack_icon {
            map.serialize_entry("slack_icon", value)?;
        }
        if let Some(value) = &self.subtext {
            map.serialize_entry("subtext", value)?;
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
impl CardBlockBuilder {
    /// Sets the image displayed prominently at the top of the card.
    pub fn hero_image(mut self, value: impl Into<ImageElement>) -> Self {
        self.hero_image = Some(value.into());
        self
    }
    /// Omits `hero_image`, including any model default.
    pub fn clear_hero_image(mut self) -> Self {
        self.hero_image = None;
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
    /// Sets the card title.
    pub fn title(mut self, value: impl Into<TextInput>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Omits `title`, including any model default.
    pub fn clear_title(mut self) -> Self {
        self.title = None;
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
    /// Sets the main body text of the card.
    pub fn body(mut self, value: impl Into<TextInput>) -> Self {
        self.body = Some(value.into());
        self
    }
    /// Omits `body`, including any model default.
    pub fn clear_body(mut self) -> Self {
        self.body = None;
        self
    }
    /// Adds buttons displayed at the bottom of the card, in display order.
    /// Replaces the collection; order is preserved.
    pub fn actions<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ButtonElement>,
    {
        self.actions = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn action(mut self, value: impl Into<ButtonElement>) -> Self {
        self.actions.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `actions`, including any model default.
    pub fn clear_actions(mut self) -> Self {
        self.actions = None;
        self
    }
    /// Sets a Slack-provided icon displayed beside the title. Cannot be combined with an image icon.
    pub fn slack_icon(mut self, value: impl Into<SlackIcon>) -> Self {
        self.slack_icon = Some(value.into());
        self
    }
    /// Omits `slack_icon`, including any model default.
    pub fn clear_slack_icon(mut self) -> Self {
        self.slack_icon = None;
        self
    }
    /// Sets the small print shown at the bottom of the card.
    pub fn subtext(mut self, value: impl Into<TextInput>) -> Self {
        self.subtext = Some(value.into());
        self
    }
    /// Omits `subtext`, including any model default.
    pub fn clear_subtext(mut self) -> Self {
        self.subtext = None;
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
    pub fn build(self) -> Result<CardBlock, ValidationError> {
        let hero_image = self.hero_image;
        let icon = self.icon;
        let title = self
            .title
            .map(|v| v.resolve(false).map_err(|e| e.at("CardBlock.title")))
            .transpose()?;
        let subtitle = self
            .subtitle
            .map(|v| v.resolve(false).map_err(|e| e.at("CardBlock.subtitle")))
            .transpose()?;
        let body = self
            .body
            .map(|v| v.resolve(false).map_err(|e| e.at("CardBlock.body")))
            .transpose()?;
        let actions = self.actions;
        let slack_icon = self.slack_icon;
        let subtext = self
            .subtext
            .map(|v| v.resolve(false).map_err(|e| e.at("CardBlock.subtext")))
            .transpose()?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &[
                "hero_image",
                "icon",
                "title",
                "subtitle",
                "body",
                "actions",
                "slack_icon",
                "subtext",
                "block_id",
            ],
            "CardBlock",
        )?;
        let value = CardBlock {
            hero_image,
            icon,
            title,
            subtitle,
            body,
            actions,
            slack_icon,
            subtext,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "CardBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("title") {
            crate::rules::limits(v, "card.title", "CardBlock.title")?;
        }
        if let Some(v) = wire.get("subtitle") {
            crate::rules::limits(v, "card.subtitle", "CardBlock.subtitle")?;
        }
        if let Some(v) = wire.get("body") {
            crate::rules::limits(v, "card.body", "CardBlock.body")?;
        }
        if let Some(v) = wire.get("actions") {
            crate::rules::limits(v, "card.actions", "CardBlock.actions")?;
        }
        if let Some(v) = wire.get("subtext") {
            crate::rules::limits(v, "card.subtext", "CardBlock.subtext")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "CardBlock.block_id")?;
        }
        crate::rules::validate("CardBlock", &wire, "CardBlock")?;
        Ok(value)
    }
}
impl Serialize for CardBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for CardBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "card", path)?;
        let hero_image = wire::field::<ImageElement>(&mut map, "hero_image", path, false)?;
        let icon = wire::field::<ImageElement>(&mut map, "icon", path, false)?;
        crate::rules::coerce_text(map.get_mut("title"), "mrkdwn");
        let title = wire::field::<Text>(&mut map, "title", path, false)?;
        crate::rules::coerce_text(map.get_mut("subtitle"), "mrkdwn");
        let subtitle = wire::field::<Text>(&mut map, "subtitle", path, false)?;
        crate::rules::coerce_text(map.get_mut("body"), "mrkdwn");
        let body = wire::field::<Text>(&mut map, "body", path, false)?;
        let actions = wire::field::<Vec<ButtonElement>>(&mut map, "actions", path, false)?;
        let slack_icon = wire::field::<SlackIcon>(&mut map, "slack_icon", path, false)?;
        crate::rules::coerce_text(map.get_mut("subtext"), "mrkdwn");
        let subtext = wire::field::<Text>(&mut map, "subtext", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        CardBlockBuilder {
            hero_image,
            icon,
            title: title.map(Into::into),
            subtitle: subtitle.map(Into::into),
            body: body.map(Into::into),
            actions,
            slack_icon,
            subtext: subtext.map(Into::into),
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for CardBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "CardBlock")
    }
}
impl<'de> Deserialize<'de> for CardBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A horizontally scrolling set of cards, available in messages and App Home.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/carousel-block).

#[derive(Clone, Debug, PartialEq)]
pub struct CarouselBlock {
    elements: Vec<CardBlock>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`CarouselBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct CarouselBlockBuilder {
    elements: Option<Vec<CardBlock>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for CarouselBlockBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl CarouselBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> CarouselBlockBuilder {
        CarouselBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> CarouselBlockBuilder {
        CarouselBlockBuilder {
            elements: Some(self.elements),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds the cards in display order.
    pub fn elements(&self) -> &[CardBlock] {
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
            map.serialize_entry("type", "carousel")?;
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
impl CarouselBlockBuilder {
    /// Adds the cards in display order.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<CardBlock>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<CardBlock>) -> Self {
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
    pub fn build(self) -> Result<CarouselBlock, ValidationError> {
        let elements = wire::required(self.elements, "CarouselBlock.elements")?;
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["elements", "block_id"], "CarouselBlock")?;
        let value = CarouselBlock {
            elements,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "CarouselBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("elements") {
            crate::rules::limits(v, "carousel.elements", "CarouselBlock.elements")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "CarouselBlock.block_id")?;
        }
        crate::rules::validate("CarouselBlock", &wire, "CarouselBlock")?;
        Ok(value)
    }
}
impl Serialize for CarouselBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for CarouselBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "carousel", path)?;
        let elements = wire::field::<Vec<CardBlock>>(&mut map, "elements", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        CarouselBlockBuilder {
            elements,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for CarouselBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "CarouselBlock")
    }
}
impl<'de> Deserialize<'de> for CarouselBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
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

/// A row of feedback or icon buttons, usually shown below AI-generated content.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/context-actions-block).

#[derive(Clone, Debug, PartialEq)]
pub struct ContextActionsBlock {
    elements: Vec<ContextActionsElement>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ContextActionsBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ContextActionsBlockBuilder {
    elements: Option<Vec<ContextActionsElement>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for ContextActionsBlockBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl ContextActionsBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ContextActionsBlockBuilder {
        ContextActionsBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ContextActionsBlockBuilder {
        ContextActionsBlockBuilder {
            elements: Some(self.elements),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds feedback buttons or icon buttons in display order.
    pub fn elements(&self) -> &[ContextActionsElement] {
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
            map.serialize_entry("type", "context_actions")?;
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
impl ContextActionsBlockBuilder {
    /// Adds feedback buttons or icon buttons in display order.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ContextActionsElement>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<ContextActionsElement>) -> Self {
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
    pub fn build(self) -> Result<ContextActionsBlock, ValidationError> {
        let elements = wire::required(self.elements, "ContextActionsBlock.elements")?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["elements", "block_id"],
            "ContextActionsBlock",
        )?;
        let value = ContextActionsBlock {
            elements,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ContextActionsBlock",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("elements") {
            crate::rules::limits(
                v,
                "context_actions.elements",
                "ContextActionsBlock.elements",
            )?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "ContextActionsBlock.block_id")?;
        }
        crate::rules::validate("ContextActionsBlock", &wire, "ContextActionsBlock")?;
        Ok(value)
    }
}
impl Serialize for ContextActionsBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ContextActionsBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "context_actions", path)?;
        let elements =
            wire::field::<Vec<ContextActionsElement>>(&mut map, "elements", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        ContextActionsBlockBuilder {
            elements,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ContextActionsBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ContextActionsBlock")
    }
}
impl<'de> Deserialize<'de> for ContextActionsBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A block of small, secondary images and text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/context-block).

#[derive(Clone, Debug, PartialEq)]
pub struct ContextBlock {
    elements: Vec<ContextElement>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ContextBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ContextBlockBuilder {
    elements: Option<Vec<ContextElement>>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for ContextBlockBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl ContextBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ContextBlockBuilder {
        ContextBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ContextBlockBuilder {
        ContextBlockBuilder {
            elements: Some(self.elements),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds images and text in display order.
    pub fn elements(&self) -> &[ContextElement] {
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
            map.serialize_entry("type", "context")?;
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
impl ContextBlockBuilder {
    /// Adds images and text in display order.
    /// Replaces the collection; order is preserved.
    pub fn elements<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ContextElement>,
    {
        self.elements = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn element(mut self, value: impl Into<ContextElement>) -> Self {
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
    pub fn build(self) -> Result<ContextBlock, ValidationError> {
        let elements = wire::required(self.elements, "ContextBlock.elements")?;
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["elements", "block_id"], "ContextBlock")?;
        let value = ContextBlock {
            elements,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ContextBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("elements") {
            crate::rules::limits(v, "context.elements", "ContextBlock.elements")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "ContextBlock.block_id")?;
        }
        crate::rules::validate("ContextBlock", &wire, "ContextBlock")?;
        Ok(value)
    }
}
impl Serialize for ContextBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ContextBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "context", path)?;
        let elements = wire::field::<Vec<ContextElement>>(&mut map, "elements", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        ContextBlockBuilder {
            elements,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ContextBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ContextBlock")
    }
}
impl<'de> Deserialize<'de> for ContextBlock {
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

/// A titled pie, bar, area, or line chart.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).

#[derive(Clone, Debug, PartialEq)]
pub struct DataVisualizationBlock {
    title: String,
    chart: Chart,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DataVisualizationBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DataVisualizationBlockBuilder {
    title: Option<String>,
    chart: Option<Chart>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for DataVisualizationBlockBuilder {
    fn default() -> Self {
        Self {
            title: None,
            chart: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl DataVisualizationBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DataVisualizationBlockBuilder {
        DataVisualizationBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DataVisualizationBlockBuilder {
        DataVisualizationBlockBuilder {
            title: Some(self.title),
            chart: Some(self.chart),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `title`. Sets the chart title.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Borrows or copies `chart`. Sets the chart to display.
    pub fn chart(&self) -> &Chart {
        &self.chart
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
            map.serialize_entry("type", "data_visualization")?;
        }
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("chart", &self.chart)?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DataVisualizationBlockBuilder {
    /// Sets the chart title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Sets the chart to display.
    pub fn chart(mut self, value: impl Into<Chart>) -> Self {
        self.chart = Some(value.into());
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
    pub fn build(self) -> Result<DataVisualizationBlock, ValidationError> {
        let title = wire::required(self.title, "DataVisualizationBlock.title")?;
        let chart = wire::required(self.chart, "DataVisualizationBlock.chart")?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["title", "chart", "block_id"],
            "DataVisualizationBlock",
        )?;
        let value = DataVisualizationBlock {
            title,
            chart,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "DataVisualizationBlock",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("title") {
            crate::rules::limits(
                v,
                "data_visualization.title",
                "DataVisualizationBlock.title",
            )?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "DataVisualizationBlock.block_id")?;
        }
        crate::rules::validate("DataVisualizationBlock", &wire, "DataVisualizationBlock")?;
        Ok(value)
    }
}
impl Serialize for DataVisualizationBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DataVisualizationBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "data_visualization", path)?;
        let title = wire::field::<String>(&mut map, "title", path, false)?;
        let chart = wire::field::<Chart>(&mut map, "chart", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        DataVisualizationBlockBuilder {
            title,
            chart,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DataVisualizationBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DataVisualizationBlock")
    }
}
impl<'de> Deserialize<'de> for DataVisualizationBlock {
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

/// A remote file previously added with the files.remote API.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/file-block).

#[derive(Clone, Debug, PartialEq)]
pub struct FileBlock {
    external_id: String,
    source: String,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`FileBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct FileBlockBuilder {
    external_id: Option<String>,
    source: Option<String>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for FileBlockBuilder {
    fn default() -> Self {
        Self {
            external_id: None,
            source: Some("remote".into()),
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl FileBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> FileBlockBuilder {
        FileBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> FileBlockBuilder {
        FileBlockBuilder {
            external_id: Some(self.external_id),
            source: Some(self.source),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `external_id`. Sets the external identifier of a remote file previously added with the files.remote API.
    pub fn external_id(&self) -> &str {
        &self.external_id
    }
    /// Borrows or copies `source`. Sets the file source. Slack currently supports only remote files.
    pub fn source(&self) -> &str {
        &self.source
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
            map.serialize_entry("type", "file")?;
        }
        map.serialize_entry("external_id", &self.external_id)?;
        map.serialize_entry("source", &self.source)?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl FileBlockBuilder {
    /// Sets the external identifier of a remote file previously added with the files.remote API.
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }
    /// Sets the file source. Slack currently supports only remote files.
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
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
    pub fn build(self) -> Result<FileBlock, ValidationError> {
        let external_id = wire::required(self.external_id, "FileBlock.external_id")?;
        let source = wire::required(self.source, "FileBlock.source")?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["external_id", "source", "block_id"],
            "FileBlock",
        )?;
        let value = FileBlock {
            external_id,
            source,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "FileBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "FileBlock.block_id")?;
        }
        crate::rules::validate("FileBlock", &wire, "FileBlock")?;
        Ok(value)
    }
}
impl Serialize for FileBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for FileBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "file", path)?;
        let external_id = wire::field::<String>(&mut map, "external_id", path, false)?;
        if !map.contains_key("source") {
            map.insert("source".into(), serde_json::json!("remote"));
        }
        let source = wire::field::<String>(&mut map, "source", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        FileBlockBuilder {
            external_id,
            source,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for FileBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "FileBlock")
    }
}
impl<'de> Deserialize<'de> for FileBlock {
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

/// A standalone image with alternative text and an optional title.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/image-block).
/// Provide exactly one of an image URL or a Slack file.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageBlock {
    image_url: Option<String>,
    slack_file: Option<SlackFile>,
    alt_text: String,
    title: Option<PlainText>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ImageBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ImageBlockBuilder {
    image_url: Option<String>,
    slack_file: Option<SlackFile>,
    alt_text: Option<String>,
    title: Option<PlainTextInput>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for ImageBlockBuilder {
    fn default() -> Self {
        Self {
            image_url: None,
            slack_file: None,
            alt_text: None,
            title: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl ImageBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ImageBlockBuilder {
        ImageBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ImageBlockBuilder {
        ImageBlockBuilder {
            image_url: self.image_url,
            slack_file: self.slack_file,
            alt_text: Some(self.alt_text),
            title: self.title.map(Into::into),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `image_url`. Sets the publicly accessible URL of the image. Cannot be combined with a Slack file.
    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }
    /// Borrows or copies `slack_file`. Sets a file hosted in Slack as the image source. Cannot be combined with an image URL.
    pub fn slack_file(&self) -> Option<&SlackFile> {
        self.slack_file.as_ref()
    }
    /// Borrows or copies `alt_text`. Sets a plain-text summary of the image or video for assistive technology.
    pub fn alt_text(&self) -> &str {
        &self.alt_text
    }
    /// Borrows or copies `title`. Sets the title shown above the image.
    pub fn title(&self) -> Option<&PlainText> {
        self.title.as_ref()
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
            map.serialize_entry("type", "image")?;
        }
        if let Some(value) = &self.image_url {
            map.serialize_entry("image_url", value)?;
        }
        if let Some(value) = &self.slack_file {
            map.serialize_entry("slack_file", value)?;
        }
        map.serialize_entry("alt_text", &self.alt_text)?;
        if let Some(value) = &self.title {
            map.serialize_entry("title", value)?;
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
impl ImageBlockBuilder {
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
    /// Sets a plain-text summary of the image or video for assistive technology.
    pub fn alt_text(mut self, value: impl Into<String>) -> Self {
        self.alt_text = Some(value.into());
        self
    }
    /// Sets the title shown above the image.
    pub fn title(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Omits `title`, including any model default.
    pub fn clear_title(mut self) -> Self {
        self.title = None;
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
    pub fn build(self) -> Result<ImageBlock, ValidationError> {
        let image_url = self.image_url;
        let slack_file = self.slack_file;
        let alt_text = wire::required(self.alt_text, "ImageBlock.alt_text")?;
        let title = self
            .title
            .map(|v| v.resolve().map_err(|e| e.at("ImageBlock.title")))
            .transpose()?;
        let block_id = self.block_id;
        wire::extensions(
            &self.extensions,
            &["image_url", "slack_file", "alt_text", "title", "block_id"],
            "ImageBlock",
        )?;
        let value = ImageBlock {
            image_url,
            slack_file,
            alt_text,
            title,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ImageBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("image_url") {
            crate::rules::limits(v, "image.image_url", "ImageBlock.image_url")?;
        }
        if let Some(v) = wire.get("alt_text") {
            crate::rules::limits(v, "image.alt_text", "ImageBlock.alt_text")?;
        }
        if let Some(v) = wire.get("title") {
            crate::rules::limits(v, "image.title", "ImageBlock.title")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "ImageBlock.block_id")?;
        }
        crate::rules::validate("ImageBlock", &wire, "ImageBlock")?;
        Ok(value)
    }
}
impl Serialize for ImageBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ImageBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "image", path)?;
        let image_url = wire::field::<String>(&mut map, "image_url", path, false)?;
        let slack_file = wire::field::<SlackFile>(&mut map, "slack_file", path, false)?;
        let alt_text = wire::field::<String>(&mut map, "alt_text", path, false)?;
        crate::rules::coerce_text(map.get_mut("title"), "plain_text");
        let title = wire::field::<PlainText>(&mut map, "title", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        ImageBlockBuilder {
            image_url,
            slack_file,
            alt_text,
            title: title.map(Into::into),
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ImageBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ImageBlock")
    }
}
impl<'de> Deserialize<'de> for ImageBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A labelled input that collects a value in a modal, App Home, or message.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/input-block).

#[derive(Clone, Debug, PartialEq)]
pub struct InputBlock {
    label: PlainText,
    element: InputElement,
    dispatch_action: Option<bool>,
    block_id: Option<String>,
    hint: Option<PlainText>,
    optional: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`InputBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct InputBlockBuilder {
    label: Option<PlainTextInput>,
    element: Option<InputElement>,
    dispatch_action: Option<bool>,
    block_id: Option<String>,
    hint: Option<PlainTextInput>,
    optional: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for InputBlockBuilder {
    fn default() -> Self {
        Self {
            label: None,
            element: None,
            dispatch_action: None,
            block_id: None,
            hint: None,
            optional: None,
            extensions: Map::new(),
        }
    }
}
impl InputBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> InputBlockBuilder {
        InputBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> InputBlockBuilder {
        InputBlockBuilder {
            label: Some(self.label.into()),
            element: Some(self.element),
            dispatch_action: self.dispatch_action,
            block_id: self.block_id,
            hint: self.hint.map(Into::into),
            optional: self.optional,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `label`. Sets the label shown above the input.
    pub fn label(&self) -> &PlainText {
        &self.label
    }
    /// Borrows or copies `element`. Sets the input element that collects the user's value.
    pub fn element(&self) -> &InputElement {
        &self.element
    }
    /// Borrows or copies `dispatch_action`. Sets whether changing the element sends a block_actions payload immediately.
    pub fn dispatch_action(&self) -> Option<bool> {
        self.dispatch_action
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows or copies `hint`. Sets helper text shown below the input.
    pub fn hint(&self) -> Option<&PlainText> {
        self.hint.as_ref()
    }
    /// Borrows or copies `optional`. Sets whether the modal can be submitted when this input is empty.
    pub fn optional(&self) -> Option<bool> {
        self.optional
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
            map.serialize_entry("type", "input")?;
        }
        map.serialize_entry("label", &self.label)?;
        map.serialize_entry("element", &self.element)?;
        if let Some(value) = &self.dispatch_action {
            map.serialize_entry("dispatch_action", value)?;
        }
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        if let Some(value) = &self.hint {
            map.serialize_entry("hint", value)?;
        }
        if let Some(value) = &self.optional {
            map.serialize_entry("optional", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl InputBlockBuilder {
    /// Sets the label shown above the input.
    pub fn label(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.label = Some(value.into());
        self
    }
    /// Sets the input element that collects the user's value.
    pub fn element(mut self, value: impl Into<InputElement>) -> Self {
        self.element = Some(value.into());
        self
    }
    /// Sets whether changing the element sends a block_actions payload immediately.
    pub fn dispatch_action(mut self, value: bool) -> Self {
        self.dispatch_action = Some(value);
        self
    }
    /// Omits `dispatch_action`, including any model default.
    pub fn clear_dispatch_action(mut self) -> Self {
        self.dispatch_action = None;
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
    /// Sets helper text shown below the input.
    pub fn hint(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.hint = Some(value.into());
        self
    }
    /// Omits `hint`, including any model default.
    pub fn clear_hint(mut self) -> Self {
        self.hint = None;
        self
    }
    /// Sets whether the modal can be submitted when this input is empty.
    pub fn optional(mut self, value: bool) -> Self {
        self.optional = Some(value);
        self
    }
    /// Omits `optional`, including any model default.
    pub fn clear_optional(mut self) -> Self {
        self.optional = None;
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
    pub fn build(self) -> Result<InputBlock, ValidationError> {
        let label = wire::required(
            self.label
                .map(|v| v.resolve().map_err(|e| e.at("InputBlock.label")))
                .transpose()?,
            "InputBlock.label",
        )?;
        let element = wire::required(self.element, "InputBlock.element")?;
        let dispatch_action = self.dispatch_action;
        let block_id = self.block_id;
        let hint = self
            .hint
            .map(|v| v.resolve().map_err(|e| e.at("InputBlock.hint")))
            .transpose()?;
        let optional = self.optional;
        wire::extensions(
            &self.extensions,
            &[
                "label",
                "element",
                "dispatch_action",
                "block_id",
                "hint",
                "optional",
            ],
            "InputBlock",
        )?;
        let value = InputBlock {
            label,
            element,
            dispatch_action,
            block_id,
            hint,
            optional,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "InputBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("label") {
            crate::rules::limits(v, "input.label", "InputBlock.label")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "InputBlock.block_id")?;
        }
        if let Some(v) = wire.get("hint") {
            crate::rules::limits(v, "input.hint", "InputBlock.hint")?;
        }
        crate::rules::validate("InputBlock", &wire, "InputBlock")?;
        Ok(value)
    }
}
impl Serialize for InputBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for InputBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "input", path)?;
        crate::rules::coerce_text(map.get_mut("label"), "plain_text");
        let label = wire::field::<PlainText>(&mut map, "label", path, false)?;
        let element = wire::field::<InputElement>(&mut map, "element", path, false)?;
        let dispatch_action = wire::field::<bool>(&mut map, "dispatch_action", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        crate::rules::coerce_text(map.get_mut("hint"), "plain_text");
        let hint = wire::field::<PlainText>(&mut map, "hint", path, false)?;
        let optional = wire::field::<bool>(&mut map, "optional", path, false)?;
        InputBlockBuilder {
            label: label.map(Into::into),
            element,
            dispatch_action,
            block_id,
            hint: hint.map(Into::into),
            optional,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for InputBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "InputBlock")
    }
}
impl<'de> Deserialize<'de> for InputBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Standard markdown rendered by Slack, intended for AI-generated content.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/markdown-block).

#[derive(Clone, Debug, PartialEq)]
pub struct MarkdownBlock {
    text: String,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`MarkdownBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct MarkdownBlockBuilder {
    text: Option<String>,
    block_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for MarkdownBlockBuilder {
    fn default() -> Self {
        Self {
            text: None,
            block_id: None,
            extensions: Map::new(),
        }
    }
}
impl MarkdownBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> MarkdownBlockBuilder {
        MarkdownBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> MarkdownBlockBuilder {
        MarkdownBlockBuilder {
            text: Some(self.text),
            block_id: self.block_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets standard markdown content, which Slack renders directly.
    pub fn text(&self) -> &str {
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
            map.serialize_entry("type", "markdown")?;
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
impl MarkdownBlockBuilder {
    /// Sets standard markdown content, which Slack renders directly.
    pub fn text(mut self, value: impl Into<String>) -> Self {
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
    pub fn build(self) -> Result<MarkdownBlock, ValidationError> {
        let text = wire::required(self.text, "MarkdownBlock.text")?;
        let block_id = self.block_id;
        wire::extensions(&self.extensions, &["text", "block_id"], "MarkdownBlock")?;
        let value = MarkdownBlock {
            text,
            block_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "MarkdownBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "markdown.text", "MarkdownBlock.text")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "MarkdownBlock.block_id")?;
        }
        crate::rules::validate("MarkdownBlock", &wire, "MarkdownBlock")?;
        Ok(value)
    }
}
impl Serialize for MarkdownBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for MarkdownBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "markdown", path)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        MarkdownBlockBuilder {
            text,
            block_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for MarkdownBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "MarkdownBlock")
    }
}
impl<'de> Deserialize<'de> for MarkdownBlock {
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

/// An embedded video player.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/video-block).

#[derive(Clone, Debug, PartialEq)]
pub struct VideoBlock {
    alt_text: String,
    thumbnail_url: String,
    title: PlainText,
    video_url: String,
    block_id: Option<String>,
    author_name: Option<String>,
    description: Option<PlainText>,
    provider_icon_url: Option<String>,
    provider_name: Option<String>,
    title_url: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`VideoBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct VideoBlockBuilder {
    alt_text: Option<String>,
    thumbnail_url: Option<String>,
    title: Option<PlainTextInput>,
    video_url: Option<String>,
    block_id: Option<String>,
    author_name: Option<String>,
    description: Option<PlainTextInput>,
    provider_icon_url: Option<String>,
    provider_name: Option<String>,
    title_url: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for VideoBlockBuilder {
    fn default() -> Self {
        Self {
            alt_text: None,
            thumbnail_url: None,
            title: None,
            video_url: None,
            block_id: None,
            author_name: None,
            description: None,
            provider_icon_url: None,
            provider_name: None,
            title_url: None,
            extensions: Map::new(),
        }
    }
}
impl VideoBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> VideoBlockBuilder {
        VideoBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> VideoBlockBuilder {
        VideoBlockBuilder {
            alt_text: Some(self.alt_text),
            thumbnail_url: Some(self.thumbnail_url),
            title: Some(self.title.into()),
            video_url: Some(self.video_url),
            block_id: self.block_id,
            author_name: self.author_name,
            description: self.description.map(Into::into),
            provider_icon_url: self.provider_icon_url,
            provider_name: self.provider_name,
            title_url: self.title_url,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `alt_text`. Sets a plain-text summary of the image or video for assistive technology.
    pub fn alt_text(&self) -> &str {
        &self.alt_text
    }
    /// Borrows or copies `thumbnail_url`. Sets the URL of the image shown before the video plays.
    pub fn thumbnail_url(&self) -> &str {
        &self.thumbnail_url
    }
    /// Borrows or copies `title`. Sets the video title.
    pub fn title(&self) -> &PlainText {
        &self.title
    }
    /// Borrows or copies `video_url`. Sets the embeddable URL of the video. The domain must be listed in the app's unfurl domains.
    pub fn video_url(&self) -> &str {
        &self.video_url
    }
    /// Borrows or copies `block_id`. Sets a unique identifier for this block. Slack returns it in interaction payloads, so use a stable value when you need to find the block again.
    pub fn block_id(&self) -> Option<&str> {
        self.block_id.as_deref()
    }
    /// Borrows or copies `author_name`. Sets the name of the video's author.
    pub fn author_name(&self) -> Option<&str> {
        self.author_name.as_deref()
    }
    /// Borrows or copies `description`. Sets the video description.
    pub fn description(&self) -> Option<&PlainText> {
        self.description.as_ref()
    }
    /// Borrows or copies `provider_icon_url`. Sets the URL of the video provider's icon.
    pub fn provider_icon_url(&self) -> Option<&str> {
        self.provider_icon_url.as_deref()
    }
    /// Borrows or copies `provider_name`. Sets the name of the video provider, such as YouTube.
    pub fn provider_name(&self) -> Option<&str> {
        self.provider_name.as_deref()
    }
    /// Borrows or copies `title_url`. Sets the HTTPS URL opened when the title is clicked.
    pub fn title_url(&self) -> Option<&str> {
        self.title_url.as_deref()
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
            map.serialize_entry("type", "video")?;
        }
        map.serialize_entry("alt_text", &self.alt_text)?;
        map.serialize_entry("thumbnail_url", &self.thumbnail_url)?;
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("video_url", &self.video_url)?;
        if let Some(value) = &self.block_id {
            map.serialize_entry("block_id", value)?;
        }
        if let Some(value) = &self.author_name {
            map.serialize_entry("author_name", value)?;
        }
        if let Some(value) = &self.description {
            map.serialize_entry("description", value)?;
        }
        if let Some(value) = &self.provider_icon_url {
            map.serialize_entry("provider_icon_url", value)?;
        }
        if let Some(value) = &self.provider_name {
            map.serialize_entry("provider_name", value)?;
        }
        if let Some(value) = &self.title_url {
            map.serialize_entry("title_url", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl VideoBlockBuilder {
    /// Sets a plain-text summary of the image or video for assistive technology.
    pub fn alt_text(mut self, value: impl Into<String>) -> Self {
        self.alt_text = Some(value.into());
        self
    }
    /// Sets the URL of the image shown before the video plays.
    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }
    /// Sets the video title.
    pub fn title(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Sets the embeddable URL of the video. The domain must be listed in the app's unfurl domains.
    pub fn video_url(mut self, value: impl Into<String>) -> Self {
        self.video_url = Some(value.into());
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
    /// Sets the name of the video's author.
    pub fn author_name(mut self, value: impl Into<String>) -> Self {
        self.author_name = Some(value.into());
        self
    }
    /// Omits `author_name`, including any model default.
    pub fn clear_author_name(mut self) -> Self {
        self.author_name = None;
        self
    }
    /// Sets the video description.
    pub fn description(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Omits `description`, including any model default.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }
    /// Sets the URL of the video provider's icon.
    pub fn provider_icon_url(mut self, value: impl Into<String>) -> Self {
        self.provider_icon_url = Some(value.into());
        self
    }
    /// Omits `provider_icon_url`, including any model default.
    pub fn clear_provider_icon_url(mut self) -> Self {
        self.provider_icon_url = None;
        self
    }
    /// Sets the name of the video provider, such as YouTube.
    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }
    /// Omits `provider_name`, including any model default.
    pub fn clear_provider_name(mut self) -> Self {
        self.provider_name = None;
        self
    }
    /// Sets the HTTPS URL opened when the title is clicked.
    pub fn title_url(mut self, value: impl Into<String>) -> Self {
        self.title_url = Some(value.into());
        self
    }
    /// Omits `title_url`, including any model default.
    pub fn clear_title_url(mut self) -> Self {
        self.title_url = None;
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
    pub fn build(self) -> Result<VideoBlock, ValidationError> {
        let alt_text = wire::required(self.alt_text, "VideoBlock.alt_text")?;
        let thumbnail_url = wire::required(self.thumbnail_url, "VideoBlock.thumbnail_url")?;
        let title = wire::required(
            self.title
                .map(|v| v.resolve().map_err(|e| e.at("VideoBlock.title")))
                .transpose()?,
            "VideoBlock.title",
        )?;
        let video_url = wire::required(self.video_url, "VideoBlock.video_url")?;
        let block_id = self.block_id;
        let author_name = self.author_name;
        let description = self
            .description
            .map(|v| v.resolve().map_err(|e| e.at("VideoBlock.description")))
            .transpose()?;
        let provider_icon_url = self.provider_icon_url;
        let provider_name = self.provider_name;
        let title_url = self.title_url;
        wire::extensions(
            &self.extensions,
            &[
                "alt_text",
                "thumbnail_url",
                "title",
                "video_url",
                "block_id",
                "author_name",
                "description",
                "provider_icon_url",
                "provider_name",
                "title_url",
            ],
            "VideoBlock",
        )?;
        let value = VideoBlock {
            alt_text,
            thumbnail_url,
            title,
            video_url,
            block_id,
            author_name,
            description,
            provider_icon_url,
            provider_name,
            title_url,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "VideoBlock", e.to_string())
        })?;
        if let Some(v) = wire.get("alt_text") {
            crate::rules::limits(v, "video.alt_text", "VideoBlock.alt_text")?;
        }
        if let Some(v) = wire.get("thumbnail_url") {
            crate::rules::limits(v, "video.thumbnail_url", "VideoBlock.thumbnail_url")?;
        }
        if let Some(v) = wire.get("title") {
            crate::rules::limits(v, "video.title", "VideoBlock.title")?;
        }
        if let Some(v) = wire.get("video_url") {
            crate::rules::limits(v, "video.video_url", "VideoBlock.video_url")?;
        }
        if let Some(v) = wire.get("block_id") {
            crate::rules::limits(v, "block_id", "VideoBlock.block_id")?;
        }
        if let Some(v) = wire.get("author_name") {
            crate::rules::limits(v, "video.author_name", "VideoBlock.author_name")?;
        }
        if let Some(v) = wire.get("description") {
            crate::rules::limits(v, "video.description", "VideoBlock.description")?;
        }
        if let Some(v) = wire.get("provider_icon_url") {
            crate::rules::limits(v, "video.provider_icon_url", "VideoBlock.provider_icon_url")?;
        }
        if let Some(v) = wire.get("provider_name") {
            crate::rules::limits(v, "video.provider_name", "VideoBlock.provider_name")?;
        }
        if let Some(v) = wire.get("title_url") {
            crate::rules::limits(v, "video.title_url", "VideoBlock.title_url")?;
        }
        crate::rules::validate("VideoBlock", &wire, "VideoBlock")?;
        Ok(value)
    }
}
impl Serialize for VideoBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for VideoBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "video", path)?;
        let alt_text = wire::field::<String>(&mut map, "alt_text", path, false)?;
        let thumbnail_url = wire::field::<String>(&mut map, "thumbnail_url", path, false)?;
        crate::rules::coerce_text(map.get_mut("title"), "plain_text");
        let title = wire::field::<PlainText>(&mut map, "title", path, false)?;
        let video_url = wire::field::<String>(&mut map, "video_url", path, false)?;
        let block_id = wire::field::<String>(&mut map, "block_id", path, false)?;
        let author_name = wire::field::<String>(&mut map, "author_name", path, false)?;
        crate::rules::coerce_text(map.get_mut("description"), "plain_text");
        let description = wire::field::<PlainText>(&mut map, "description", path, false)?;
        let provider_icon_url = wire::field::<String>(&mut map, "provider_icon_url", path, false)?;
        let provider_name = wire::field::<String>(&mut map, "provider_name", path, false)?;
        let title_url = wire::field::<String>(&mut map, "title_url", path, false)?;
        VideoBlockBuilder {
            alt_text,
            thumbnail_url,
            title: title.map(Into::into),
            video_url,
            block_id,
            author_name,
            description: description.map(Into::into),
            provider_icon_url,
            provider_name,
            title_url,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for VideoBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "VideoBlock")
    }
}
impl<'de> Deserialize<'de> for VideoBlock {
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

/// A menu for selecting multiple public channels.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelMultiSelectElement {
    action_id: Option<String>,
    initial_channels: Option<Vec<String>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ChannelMultiSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ChannelMultiSelectElementBuilder {
    action_id: Option<String>,
    initial_channels: Option<Vec<String>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for ChannelMultiSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_channels: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl ChannelMultiSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ChannelMultiSelectElementBuilder {
        ChannelMultiSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ChannelMultiSelectElementBuilder {
        ChannelMultiSelectElementBuilder {
            action_id: self.action_id,
            initial_channels: self.initial_channels,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_channels`. Adds channel IDs that are selected when the menu loads.
    pub fn initial_channels(&self) -> Option<&[String]> {
        self.initial_channels.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `max_selected_items`. Sets the maximum number of items a user can select.
    pub fn max_selected_items(&self) -> Option<i64> {
        self.max_selected_items
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "multi_channels_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_channels {
            map.serialize_entry("initial_channels", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.max_selected_items {
            map.serialize_entry("max_selected_items", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ChannelMultiSelectElementBuilder {
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
    /// Adds channel IDs that are selected when the menu loads.
    /// Replaces the collection; order is preserved.
    pub fn initial_channels<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.initial_channels = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn initial_channel(mut self, value: impl Into<String>) -> Self {
        self.initial_channels
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `initial_channels`, including any model default.
    pub fn clear_initial_channels(mut self) -> Self {
        self.initial_channels = None;
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
    /// Sets the maximum number of items a user can select.
    pub fn max_selected_items(mut self, value: i64) -> Self {
        self.max_selected_items = Some(value);
        self
    }
    /// Omits `max_selected_items`, including any model default.
    pub fn clear_max_selected_items(mut self) -> Self {
        self.max_selected_items = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<ChannelMultiSelectElement, ValidationError> {
        let action_id = self.action_id;
        let initial_channels = self.initial_channels;
        let confirm = self.confirm;
        let max_selected_items = self.max_selected_items;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("ChannelMultiSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_channels",
                "confirm",
                "max_selected_items",
                "focus_on_load",
                "placeholder",
            ],
            "ChannelMultiSelectElement",
        )?;
        let value = ChannelMultiSelectElement {
            action_id,
            initial_channels,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ChannelMultiSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ChannelMultiSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("max_selected_items") {
            crate::rules::limits(
                v,
                "multi_select.max_selected_items",
                "ChannelMultiSelectElement.max_selected_items",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "select.placeholder",
                "ChannelMultiSelectElement.placeholder",
            )?;
        }
        crate::rules::validate(
            "ChannelMultiSelectElement",
            &wire,
            "ChannelMultiSelectElement",
        )?;
        Ok(value)
    }
}
impl Serialize for ChannelMultiSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ChannelMultiSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "multi_channels_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_channels =
            wire::field::<Vec<String>>(&mut map, "initial_channels", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let max_selected_items = wire::field::<i64>(&mut map, "max_selected_items", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        ChannelMultiSelectElementBuilder {
            action_id,
            initial_channels,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ChannelMultiSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ChannelMultiSelectElement")
    }
}
impl<'de> Deserialize<'de> for ChannelMultiSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting one public channel.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelSelectElement {
    action_id: Option<String>,
    initial_channel: Option<String>,
    response_url_enabled: Option<bool>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ChannelSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ChannelSelectElementBuilder {
    action_id: Option<String>,
    initial_channel: Option<String>,
    response_url_enabled: Option<bool>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for ChannelSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_channel: None,
            response_url_enabled: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl ChannelSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ChannelSelectElementBuilder {
        ChannelSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ChannelSelectElementBuilder {
        ChannelSelectElementBuilder {
            action_id: self.action_id,
            initial_channel: self.initial_channel,
            response_url_enabled: self.response_url_enabled,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_channel`. Sets the channel ID selected when the menu loads.
    pub fn initial_channel(&self) -> Option<&str> {
        self.initial_channel.as_deref()
    }
    /// Borrows or copies `response_url_enabled`. Sets whether the selected conversation receives a response URL in the view submission payload. Only valid in modals.
    pub fn response_url_enabled(&self) -> Option<bool> {
        self.response_url_enabled
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "channels_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_channel {
            map.serialize_entry("initial_channel", value)?;
        }
        if let Some(value) = &self.response_url_enabled {
            map.serialize_entry("response_url_enabled", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ChannelSelectElementBuilder {
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
    /// Sets the channel ID selected when the menu loads.
    pub fn initial_channel(mut self, value: impl Into<String>) -> Self {
        self.initial_channel = Some(value.into());
        self
    }
    /// Omits `initial_channel`, including any model default.
    pub fn clear_initial_channel(mut self) -> Self {
        self.initial_channel = None;
        self
    }
    /// Sets whether the selected conversation receives a response URL in the view submission payload. Only valid in modals.
    pub fn response_url_enabled(mut self, value: bool) -> Self {
        self.response_url_enabled = Some(value);
        self
    }
    /// Omits `response_url_enabled`, including any model default.
    pub fn clear_response_url_enabled(mut self) -> Self {
        self.response_url_enabled = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<ChannelSelectElement, ValidationError> {
        let action_id = self.action_id;
        let initial_channel = self.initial_channel;
        let response_url_enabled = self.response_url_enabled;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("ChannelSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_channel",
                "response_url_enabled",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "ChannelSelectElement",
        )?;
        let value = ChannelSelectElement {
            action_id,
            initial_channel,
            response_url_enabled,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ChannelSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ChannelSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(v, "select.placeholder", "ChannelSelectElement.placeholder")?;
        }
        crate::rules::validate("ChannelSelectElement", &wire, "ChannelSelectElement")?;
        Ok(value)
    }
}
impl Serialize for ChannelSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ChannelSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "channels_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_channel = wire::field::<String>(&mut map, "initial_channel", path, false)?;
        let response_url_enabled =
            wire::field::<bool>(&mut map, "response_url_enabled", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        ChannelSelectElementBuilder {
            action_id,
            initial_channel,
            response_url_enabled,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ChannelSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ChannelSelectElement")
    }
}
impl<'de> Deserialize<'de> for ChannelSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A group of checkboxes.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element).

#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxesElement {
    action_id: Option<String>,
    options: Vec<SelectOption>,
    initial_options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`CheckboxesElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct CheckboxesElementBuilder {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    initial_options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for CheckboxesElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            options: None,
            initial_options: None,
            confirm: None,
            focus_on_load: None,
            extensions: Map::new(),
        }
    }
}
impl CheckboxesElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> CheckboxesElementBuilder {
        CheckboxesElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> CheckboxesElementBuilder {
        CheckboxesElementBuilder {
            action_id: self.action_id,
            options: Some(self.options),
            initial_options: self.initial_options,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `options`. Adds selectable options in display order.
    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }
    /// Borrows or copies `initial_options`. Adds options that are selected when the element loads. Each must match an option in the element.
    pub fn initial_options(&self) -> Option<&[SelectOption]> {
        self.initial_options.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
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
            map.serialize_entry("type", "checkboxes")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        map.serialize_entry("options", &self.options)?;
        if let Some(value) = &self.initial_options {
            map.serialize_entry("initial_options", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CheckboxesElementBuilder {
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
    /// Adds selectable options in display order.
    /// Replaces the collection; order is preserved.
    pub fn options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option(mut self, value: impl Into<SelectOption>) -> Self {
        self.options.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Adds options that are selected when the element loads. Each must match an option in the element.
    /// Replaces the collection; order is preserved.
    pub fn initial_options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.initial_options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn initial_option(mut self, value: impl Into<SelectOption>) -> Self {
        self.initial_options
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `initial_options`, including any model default.
    pub fn clear_initial_options(mut self) -> Self {
        self.initial_options = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
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
    pub fn build(self) -> Result<CheckboxesElement, ValidationError> {
        let action_id = self.action_id;
        let options = wire::required(self.options, "CheckboxesElement.options")?;
        let initial_options = self.initial_options;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "options",
                "initial_options",
                "confirm",
                "focus_on_load",
            ],
            "CheckboxesElement",
        )?;
        let value = CheckboxesElement {
            action_id,
            options,
            initial_options,
            confirm,
            focus_on_load,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "CheckboxesElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "CheckboxesElement.action_id")?;
        }
        if let Some(v) = wire.get("options") {
            crate::rules::limits(v, "checkboxes.options", "CheckboxesElement.options")?;
        }
        crate::rules::validate("CheckboxesElement", &wire, "CheckboxesElement")?;
        Ok(value)
    }
}
impl Serialize for CheckboxesElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for CheckboxesElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "checkboxes", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let options = wire::field::<Vec<SelectOption>>(&mut map, "options", path, false)?;
        let initial_options =
            wire::field::<Vec<SelectOption>>(&mut map, "initial_options", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        CheckboxesElementBuilder {
            action_id,
            options,
            initial_options,
            confirm,
            focus_on_load,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for CheckboxesElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "CheckboxesElement")
    }
}
impl<'de> Deserialize<'de> for CheckboxesElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting multiple conversations.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ConversationMultiSelectElement {
    action_id: Option<String>,
    initial_conversations: Option<Vec<String>>,
    default_to_current_conversation: Option<bool>,
    filter: Option<ConversationFilter>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ConversationMultiSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ConversationMultiSelectElementBuilder {
    action_id: Option<String>,
    initial_conversations: Option<Vec<String>>,
    default_to_current_conversation: Option<bool>,
    filter: Option<ConversationFilter>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for ConversationMultiSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_conversations: None,
            default_to_current_conversation: None,
            filter: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl ConversationMultiSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ConversationMultiSelectElementBuilder {
        ConversationMultiSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ConversationMultiSelectElementBuilder {
        ConversationMultiSelectElementBuilder {
            action_id: self.action_id,
            initial_conversations: self.initial_conversations,
            default_to_current_conversation: self.default_to_current_conversation,
            filter: self.filter,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_conversations`. Adds conversation IDs that are selected when the menu loads.
    pub fn initial_conversations(&self) -> Option<&[String]> {
        self.initial_conversations.as_deref()
    }
    /// Borrows or copies `default_to_current_conversation`. Sets whether the menu pre-selects the conversation the user is viewing.
    pub fn default_to_current_conversation(&self) -> Option<bool> {
        self.default_to_current_conversation
    }
    /// Borrows or copies `filter`. Sets which conversation types the menu offers.
    pub fn filter(&self) -> Option<&ConversationFilter> {
        self.filter.as_ref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `max_selected_items`. Sets the maximum number of items a user can select.
    pub fn max_selected_items(&self) -> Option<i64> {
        self.max_selected_items
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "multi_conversations_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_conversations {
            map.serialize_entry("initial_conversations", value)?;
        }
        if let Some(value) = &self.default_to_current_conversation {
            map.serialize_entry("default_to_current_conversation", value)?;
        }
        if let Some(value) = &self.filter {
            map.serialize_entry("filter", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.max_selected_items {
            map.serialize_entry("max_selected_items", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ConversationMultiSelectElementBuilder {
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
    /// Adds conversation IDs that are selected when the menu loads.
    /// Replaces the collection; order is preserved.
    pub fn initial_conversations<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.initial_conversations = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn initial_conversation(mut self, value: impl Into<String>) -> Self {
        self.initial_conversations
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `initial_conversations`, including any model default.
    pub fn clear_initial_conversations(mut self) -> Self {
        self.initial_conversations = None;
        self
    }
    /// Sets whether the menu pre-selects the conversation the user is viewing.
    pub fn default_to_current_conversation(mut self, value: bool) -> Self {
        self.default_to_current_conversation = Some(value);
        self
    }
    /// Omits `default_to_current_conversation`, including any model default.
    pub fn clear_default_to_current_conversation(mut self) -> Self {
        self.default_to_current_conversation = None;
        self
    }
    /// Sets which conversation types the menu offers.
    pub fn filter(mut self, value: impl Into<ConversationFilter>) -> Self {
        self.filter = Some(value.into());
        self
    }
    /// Omits `filter`, including any model default.
    pub fn clear_filter(mut self) -> Self {
        self.filter = None;
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
    /// Sets the maximum number of items a user can select.
    pub fn max_selected_items(mut self, value: i64) -> Self {
        self.max_selected_items = Some(value);
        self
    }
    /// Omits `max_selected_items`, including any model default.
    pub fn clear_max_selected_items(mut self) -> Self {
        self.max_selected_items = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<ConversationMultiSelectElement, ValidationError> {
        let action_id = self.action_id;
        let initial_conversations = self.initial_conversations;
        let default_to_current_conversation = self.default_to_current_conversation;
        let filter = self.filter;
        let confirm = self.confirm;
        let max_selected_items = self.max_selected_items;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("ConversationMultiSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_conversations",
                "default_to_current_conversation",
                "filter",
                "confirm",
                "max_selected_items",
                "focus_on_load",
                "placeholder",
            ],
            "ConversationMultiSelectElement",
        )?;
        let value = ConversationMultiSelectElement {
            action_id,
            initial_conversations,
            default_to_current_conversation,
            filter,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ConversationMultiSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ConversationMultiSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("max_selected_items") {
            crate::rules::limits(
                v,
                "multi_select.max_selected_items",
                "ConversationMultiSelectElement.max_selected_items",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "select.placeholder",
                "ConversationMultiSelectElement.placeholder",
            )?;
        }
        crate::rules::validate(
            "ConversationMultiSelectElement",
            &wire,
            "ConversationMultiSelectElement",
        )?;
        Ok(value)
    }
}
impl Serialize for ConversationMultiSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ConversationMultiSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "multi_conversations_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_conversations =
            wire::field::<Vec<String>>(&mut map, "initial_conversations", path, false)?;
        let default_to_current_conversation =
            wire::field::<bool>(&mut map, "default_to_current_conversation", path, false)?;
        let filter = wire::field::<ConversationFilter>(&mut map, "filter", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let max_selected_items = wire::field::<i64>(&mut map, "max_selected_items", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        ConversationMultiSelectElementBuilder {
            action_id,
            initial_conversations,
            default_to_current_conversation,
            filter,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ConversationMultiSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ConversationMultiSelectElement")
    }
}
impl<'de> Deserialize<'de> for ConversationMultiSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting one conversation.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ConversationSelectElement {
    action_id: Option<String>,
    initial_conversation: Option<String>,
    default_to_current_conversation: Option<bool>,
    filter: Option<ConversationFilter>,
    response_url_enabled: Option<bool>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ConversationSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ConversationSelectElementBuilder {
    action_id: Option<String>,
    initial_conversation: Option<String>,
    default_to_current_conversation: Option<bool>,
    filter: Option<ConversationFilter>,
    response_url_enabled: Option<bool>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for ConversationSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_conversation: None,
            default_to_current_conversation: None,
            filter: None,
            response_url_enabled: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl ConversationSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ConversationSelectElementBuilder {
        ConversationSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ConversationSelectElementBuilder {
        ConversationSelectElementBuilder {
            action_id: self.action_id,
            initial_conversation: self.initial_conversation,
            default_to_current_conversation: self.default_to_current_conversation,
            filter: self.filter,
            response_url_enabled: self.response_url_enabled,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_conversation`. Sets the conversation ID selected when the menu loads.
    pub fn initial_conversation(&self) -> Option<&str> {
        self.initial_conversation.as_deref()
    }
    /// Borrows or copies `default_to_current_conversation`. Sets whether the menu pre-selects the conversation the user is viewing.
    pub fn default_to_current_conversation(&self) -> Option<bool> {
        self.default_to_current_conversation
    }
    /// Borrows or copies `filter`. Sets which conversation types the menu offers.
    pub fn filter(&self) -> Option<&ConversationFilter> {
        self.filter.as_ref()
    }
    /// Borrows or copies `response_url_enabled`. Sets whether the selected conversation receives a response URL in the view submission payload. Only valid in modals.
    pub fn response_url_enabled(&self) -> Option<bool> {
        self.response_url_enabled
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "conversations_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_conversation {
            map.serialize_entry("initial_conversation", value)?;
        }
        if let Some(value) = &self.default_to_current_conversation {
            map.serialize_entry("default_to_current_conversation", value)?;
        }
        if let Some(value) = &self.filter {
            map.serialize_entry("filter", value)?;
        }
        if let Some(value) = &self.response_url_enabled {
            map.serialize_entry("response_url_enabled", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ConversationSelectElementBuilder {
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
    /// Sets the conversation ID selected when the menu loads.
    pub fn initial_conversation(mut self, value: impl Into<String>) -> Self {
        self.initial_conversation = Some(value.into());
        self
    }
    /// Omits `initial_conversation`, including any model default.
    pub fn clear_initial_conversation(mut self) -> Self {
        self.initial_conversation = None;
        self
    }
    /// Sets whether the menu pre-selects the conversation the user is viewing.
    pub fn default_to_current_conversation(mut self, value: bool) -> Self {
        self.default_to_current_conversation = Some(value);
        self
    }
    /// Omits `default_to_current_conversation`, including any model default.
    pub fn clear_default_to_current_conversation(mut self) -> Self {
        self.default_to_current_conversation = None;
        self
    }
    /// Sets which conversation types the menu offers.
    pub fn filter(mut self, value: impl Into<ConversationFilter>) -> Self {
        self.filter = Some(value.into());
        self
    }
    /// Omits `filter`, including any model default.
    pub fn clear_filter(mut self) -> Self {
        self.filter = None;
        self
    }
    /// Sets whether the selected conversation receives a response URL in the view submission payload. Only valid in modals.
    pub fn response_url_enabled(mut self, value: bool) -> Self {
        self.response_url_enabled = Some(value);
        self
    }
    /// Omits `response_url_enabled`, including any model default.
    pub fn clear_response_url_enabled(mut self) -> Self {
        self.response_url_enabled = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<ConversationSelectElement, ValidationError> {
        let action_id = self.action_id;
        let initial_conversation = self.initial_conversation;
        let default_to_current_conversation = self.default_to_current_conversation;
        let filter = self.filter;
        let response_url_enabled = self.response_url_enabled;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("ConversationSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_conversation",
                "default_to_current_conversation",
                "filter",
                "response_url_enabled",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "ConversationSelectElement",
        )?;
        let value = ConversationSelectElement {
            action_id,
            initial_conversation,
            default_to_current_conversation,
            filter,
            response_url_enabled,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ConversationSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ConversationSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "select.placeholder",
                "ConversationSelectElement.placeholder",
            )?;
        }
        crate::rules::validate(
            "ConversationSelectElement",
            &wire,
            "ConversationSelectElement",
        )?;
        Ok(value)
    }
}
impl Serialize for ConversationSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ConversationSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "conversations_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_conversation =
            wire::field::<String>(&mut map, "initial_conversation", path, false)?;
        let default_to_current_conversation =
            wire::field::<bool>(&mut map, "default_to_current_conversation", path, false)?;
        let filter = wire::field::<ConversationFilter>(&mut map, "filter", path, false)?;
        let response_url_enabled =
            wire::field::<bool>(&mut map, "response_url_enabled", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        ConversationSelectElementBuilder {
            action_id,
            initial_conversation,
            default_to_current_conversation,
            filter,
            response_url_enabled,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ConversationSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ConversationSelectElement")
    }
}
impl<'de> Deserialize<'de> for ConversationSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A calendar date picker.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element).

#[derive(Clone, Debug, PartialEq)]
pub struct DatePickerElement {
    action_id: Option<String>,
    initial_date: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DatePickerElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DatePickerElementBuilder {
    action_id: Option<String>,
    initial_date: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for DatePickerElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_date: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl DatePickerElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DatePickerElementBuilder {
        DatePickerElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DatePickerElementBuilder {
        DatePickerElementBuilder {
            action_id: self.action_id,
            initial_date: self.initial_date,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_date`. Sets the initially selected date, formatted as YYYY-MM-DD.
    pub fn initial_date(&self) -> Option<&str> {
        self.initial_date.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "datepicker")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_date {
            map.serialize_entry("initial_date", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DatePickerElementBuilder {
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
    /// Sets the initially selected date, formatted as YYYY-MM-DD.
    pub fn initial_date(mut self, value: impl Into<String>) -> Self {
        self.initial_date = Some(value.into());
        self
    }
    /// Omits `initial_date`, including any model default.
    pub fn clear_initial_date(mut self) -> Self {
        self.initial_date = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<DatePickerElement, ValidationError> {
        let action_id = self.action_id;
        let initial_date = self.initial_date;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("DatePickerElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_date",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "DatePickerElement",
        )?;
        let value = DatePickerElement {
            action_id,
            initial_date,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "DatePickerElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "DatePickerElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "date_picker.placeholder",
                "DatePickerElement.placeholder",
            )?;
        }
        crate::rules::validate("DatePickerElement", &wire, "DatePickerElement")?;
        Ok(value)
    }
}
impl Serialize for DatePickerElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DatePickerElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "datepicker", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_date = wire::field::<String>(&mut map, "initial_date", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        DatePickerElementBuilder {
            action_id,
            initial_date,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DatePickerElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DatePickerElement")
    }
}
impl<'de> Deserialize<'de> for DatePickerElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A combined date and time picker.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element).

#[derive(Clone, Debug, PartialEq)]
pub struct DateTimePickerElement {
    action_id: Option<String>,
    initial_date_time: Option<i64>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DateTimePickerElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DateTimePickerElementBuilder {
    action_id: Option<String>,
    initial_date_time: Option<i64>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for DateTimePickerElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_date_time: None,
            confirm: None,
            focus_on_load: None,
            extensions: Map::new(),
        }
    }
}
impl DateTimePickerElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DateTimePickerElementBuilder {
        DateTimePickerElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DateTimePickerElementBuilder {
        DateTimePickerElementBuilder {
            action_id: self.action_id,
            initial_date_time: self.initial_date_time,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_date_time`. Sets the initially selected moment as a UNIX timestamp in seconds.
    pub fn initial_date_time(&self) -> Option<i64> {
        self.initial_date_time
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
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
            map.serialize_entry("type", "datetimepicker")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_date_time {
            map.serialize_entry("initial_date_time", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DateTimePickerElementBuilder {
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
    /// Sets the initially selected moment as a UNIX timestamp in seconds.
    pub fn initial_date_time(mut self, value: i64) -> Self {
        self.initial_date_time = Some(value);
        self
    }
    /// Omits `initial_date_time`, including any model default.
    pub fn clear_initial_date_time(mut self) -> Self {
        self.initial_date_time = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
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
    pub fn build(self) -> Result<DateTimePickerElement, ValidationError> {
        let action_id = self.action_id;
        let initial_date_time = self.initial_date_time;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        wire::extensions(
            &self.extensions,
            &["action_id", "initial_date_time", "confirm", "focus_on_load"],
            "DateTimePickerElement",
        )?;
        let value = DateTimePickerElement {
            action_id,
            initial_date_time,
            confirm,
            focus_on_load,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "DateTimePickerElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "DateTimePickerElement.action_id")?;
        }
        crate::rules::validate("DateTimePickerElement", &wire, "DateTimePickerElement")?;
        Ok(value)
    }
}
impl Serialize for DateTimePickerElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DateTimePickerElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "datetimepicker", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_date_time = wire::field::<i64>(&mut map, "initial_date_time", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        DateTimePickerElementBuilder {
            action_id,
            initial_date_time,
            confirm,
            focus_on_load,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DateTimePickerElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DateTimePickerElement")
    }
}
impl<'de> Deserialize<'de> for DateTimePickerElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A single-line input that accepts an email address.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/email-input-element).

#[derive(Clone, Debug, PartialEq)]
pub struct EmailInputElement {
    action_id: Option<String>,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`EmailInputElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct EmailInputElementBuilder {
    action_id: Option<String>,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for EmailInputElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl EmailInputElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> EmailInputElementBuilder {
        EmailInputElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> EmailInputElementBuilder {
        EmailInputElementBuilder {
            action_id: self.action_id,
            initial_value: self.initial_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_value`. Sets the value present when the input loads.
    pub fn initial_value(&self) -> Option<&str> {
        self.initial_value.as_deref()
    }
    /// Borrows or copies `dispatch_action_config`. Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(&self) -> Option<&DispatchActionConfiguration> {
        self.dispatch_action_config.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "email_text_input")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_value {
            map.serialize_entry("initial_value", value)?;
        }
        if let Some(value) = &self.dispatch_action_config {
            map.serialize_entry("dispatch_action_config", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl EmailInputElementBuilder {
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
    /// Sets the value present when the input loads.
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
        self
    }
    /// Omits `initial_value`, including any model default.
    pub fn clear_initial_value(mut self) -> Self {
        self.initial_value = None;
        self
    }
    /// Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(mut self, value: impl Into<DispatchActionConfiguration>) -> Self {
        self.dispatch_action_config = Some(value.into());
        self
    }
    /// Omits `dispatch_action_config`, including any model default.
    pub fn clear_dispatch_action_config(mut self) -> Self {
        self.dispatch_action_config = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<EmailInputElement, ValidationError> {
        let action_id = self.action_id;
        let initial_value = self.initial_value;
        let dispatch_action_config = self.dispatch_action_config;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("EmailInputElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_value",
                "dispatch_action_config",
                "focus_on_load",
                "placeholder",
            ],
            "EmailInputElement",
        )?;
        let value = EmailInputElement {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "EmailInputElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "EmailInputElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "email_input.placeholder",
                "EmailInputElement.placeholder",
            )?;
        }
        crate::rules::validate("EmailInputElement", &wire, "EmailInputElement")?;
        Ok(value)
    }
}
impl Serialize for EmailInputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for EmailInputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "email_text_input", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_value = wire::field::<String>(&mut map, "initial_value", path, false)?;
        let dispatch_action_config = wire::field::<DispatchActionConfiguration>(
            &mut map,
            "dispatch_action_config",
            path,
            false,
        )?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        EmailInputElementBuilder {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for EmailInputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "EmailInputElement")
    }
}
impl<'de> Deserialize<'de> for EmailInputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting multiple options loaded from your app.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ExternalMultiSelectElement {
    action_id: Option<String>,
    min_query_length: Option<i64>,
    initial_options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ExternalMultiSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ExternalMultiSelectElementBuilder {
    action_id: Option<String>,
    min_query_length: Option<i64>,
    initial_options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for ExternalMultiSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            min_query_length: None,
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl ExternalMultiSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ExternalMultiSelectElementBuilder {
        ExternalMultiSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ExternalMultiSelectElementBuilder {
        ExternalMultiSelectElementBuilder {
            action_id: self.action_id,
            min_query_length: self.min_query_length,
            initial_options: self.initial_options,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `min_query_length`. Sets how many characters the user must type before Slack queries your options endpoint.
    pub fn min_query_length(&self) -> Option<i64> {
        self.min_query_length
    }
    /// Borrows or copies `initial_options`. Adds options that are selected when the element loads. Each must match an option in the element.
    pub fn initial_options(&self) -> Option<&[SelectOption]> {
        self.initial_options.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `max_selected_items`. Sets the maximum number of items a user can select.
    pub fn max_selected_items(&self) -> Option<i64> {
        self.max_selected_items
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "multi_external_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.min_query_length {
            map.serialize_entry("min_query_length", value)?;
        }
        if let Some(value) = &self.initial_options {
            map.serialize_entry("initial_options", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.max_selected_items {
            map.serialize_entry("max_selected_items", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ExternalMultiSelectElementBuilder {
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
    /// Sets how many characters the user must type before Slack queries your options endpoint.
    pub fn min_query_length(mut self, value: i64) -> Self {
        self.min_query_length = Some(value);
        self
    }
    /// Omits `min_query_length`, including any model default.
    pub fn clear_min_query_length(mut self) -> Self {
        self.min_query_length = None;
        self
    }
    /// Adds options that are selected when the element loads. Each must match an option in the element.
    /// Replaces the collection; order is preserved.
    pub fn initial_options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.initial_options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn initial_option(mut self, value: impl Into<SelectOption>) -> Self {
        self.initial_options
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `initial_options`, including any model default.
    pub fn clear_initial_options(mut self) -> Self {
        self.initial_options = None;
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
    /// Sets the maximum number of items a user can select.
    pub fn max_selected_items(mut self, value: i64) -> Self {
        self.max_selected_items = Some(value);
        self
    }
    /// Omits `max_selected_items`, including any model default.
    pub fn clear_max_selected_items(mut self) -> Self {
        self.max_selected_items = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<ExternalMultiSelectElement, ValidationError> {
        let action_id = self.action_id;
        let min_query_length = self.min_query_length;
        let initial_options = self.initial_options;
        let confirm = self.confirm;
        let max_selected_items = self.max_selected_items;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("ExternalMultiSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "min_query_length",
                "initial_options",
                "confirm",
                "max_selected_items",
                "focus_on_load",
                "placeholder",
            ],
            "ExternalMultiSelectElement",
        )?;
        let value = ExternalMultiSelectElement {
            action_id,
            min_query_length,
            initial_options,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ExternalMultiSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ExternalMultiSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("max_selected_items") {
            crate::rules::limits(
                v,
                "multi_select.max_selected_items",
                "ExternalMultiSelectElement.max_selected_items",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "select.placeholder",
                "ExternalMultiSelectElement.placeholder",
            )?;
        }
        crate::rules::validate(
            "ExternalMultiSelectElement",
            &wire,
            "ExternalMultiSelectElement",
        )?;
        Ok(value)
    }
}
impl Serialize for ExternalMultiSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ExternalMultiSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "multi_external_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let min_query_length = wire::field::<i64>(&mut map, "min_query_length", path, false)?;
        let initial_options =
            wire::field::<Vec<SelectOption>>(&mut map, "initial_options", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let max_selected_items = wire::field::<i64>(&mut map, "max_selected_items", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        ExternalMultiSelectElementBuilder {
            action_id,
            min_query_length,
            initial_options,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ExternalMultiSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ExternalMultiSelectElement")
    }
}
impl<'de> Deserialize<'de> for ExternalMultiSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting one option loaded from your app.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct ExternalSelectElement {
    action_id: Option<String>,
    min_query_length: Option<i64>,
    initial_option: Option<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ExternalSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ExternalSelectElementBuilder {
    action_id: Option<String>,
    min_query_length: Option<i64>,
    initial_option: Option<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for ExternalSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            min_query_length: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl ExternalSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ExternalSelectElementBuilder {
        ExternalSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ExternalSelectElementBuilder {
        ExternalSelectElementBuilder {
            action_id: self.action_id,
            min_query_length: self.min_query_length,
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `min_query_length`. Sets how many characters the user must type before Slack queries your options endpoint.
    pub fn min_query_length(&self) -> Option<i64> {
        self.min_query_length
    }
    /// Borrows or copies `initial_option`. Sets the option selected when the element loads. It must match an option in the element.
    pub fn initial_option(&self) -> Option<&SelectOption> {
        self.initial_option.as_ref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "external_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.min_query_length {
            map.serialize_entry("min_query_length", value)?;
        }
        if let Some(value) = &self.initial_option {
            map.serialize_entry("initial_option", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ExternalSelectElementBuilder {
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
    /// Sets how many characters the user must type before Slack queries your options endpoint.
    pub fn min_query_length(mut self, value: i64) -> Self {
        self.min_query_length = Some(value);
        self
    }
    /// Omits `min_query_length`, including any model default.
    pub fn clear_min_query_length(mut self) -> Self {
        self.min_query_length = None;
        self
    }
    /// Sets the option selected when the element loads. It must match an option in the element.
    pub fn initial_option(mut self, value: impl Into<SelectOption>) -> Self {
        self.initial_option = Some(value.into());
        self
    }
    /// Omits `initial_option`, including any model default.
    pub fn clear_initial_option(mut self) -> Self {
        self.initial_option = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<ExternalSelectElement, ValidationError> {
        let action_id = self.action_id;
        let min_query_length = self.min_query_length;
        let initial_option = self.initial_option;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("ExternalSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "min_query_length",
                "initial_option",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "ExternalSelectElement",
        )?;
        let value = ExternalSelectElement {
            action_id,
            min_query_length,
            initial_option,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ExternalSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "ExternalSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(v, "select.placeholder", "ExternalSelectElement.placeholder")?;
        }
        crate::rules::validate("ExternalSelectElement", &wire, "ExternalSelectElement")?;
        Ok(value)
    }
}
impl Serialize for ExternalSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ExternalSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "external_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let min_query_length = wire::field::<i64>(&mut map, "min_query_length", path, false)?;
        let initial_option = wire::field::<SelectOption>(&mut map, "initial_option", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        ExternalSelectElementBuilder {
            action_id,
            min_query_length,
            initial_option,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ExternalSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ExternalSelectElement")
    }
}
impl<'de> Deserialize<'de> for ExternalSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A pair of thumbs-up and thumbs-down feedback buttons.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element).

#[derive(Clone, Debug, PartialEq)]
pub struct FeedbackButtonsElement {
    positive_button: FeedbackButton,
    negative_button: FeedbackButton,
    action_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`FeedbackButtonsElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct FeedbackButtonsElementBuilder {
    positive_button: Option<FeedbackButton>,
    negative_button: Option<FeedbackButton>,
    action_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for FeedbackButtonsElementBuilder {
    fn default() -> Self {
        Self {
            positive_button: None,
            negative_button: None,
            action_id: None,
            extensions: Map::new(),
        }
    }
}
impl FeedbackButtonsElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> FeedbackButtonsElementBuilder {
        FeedbackButtonsElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> FeedbackButtonsElementBuilder {
        FeedbackButtonsElementBuilder {
            positive_button: Some(self.positive_button),
            negative_button: Some(self.negative_button),
            action_id: self.action_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `positive_button`. Sets the button for positive feedback.
    pub fn positive_button(&self) -> &FeedbackButton {
        &self.positive_button
    }
    /// Borrows or copies `negative_button`. Sets the button for negative feedback.
    pub fn negative_button(&self) -> &FeedbackButton {
        &self.negative_button
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
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
            map.serialize_entry("type", "feedback_buttons")?;
        }
        map.serialize_entry("positive_button", &self.positive_button)?;
        map.serialize_entry("negative_button", &self.negative_button)?;
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl FeedbackButtonsElementBuilder {
    /// Sets the button for positive feedback.
    pub fn positive_button(mut self, value: impl Into<FeedbackButton>) -> Self {
        self.positive_button = Some(value.into());
        self
    }
    /// Sets the button for negative feedback.
    pub fn negative_button(mut self, value: impl Into<FeedbackButton>) -> Self {
        self.negative_button = Some(value.into());
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
    pub fn build(self) -> Result<FeedbackButtonsElement, ValidationError> {
        let positive_button = wire::required(
            self.positive_button,
            "FeedbackButtonsElement.positive_button",
        )?;
        let negative_button = wire::required(
            self.negative_button,
            "FeedbackButtonsElement.negative_button",
        )?;
        let action_id = self.action_id;
        wire::extensions(
            &self.extensions,
            &["positive_button", "negative_button", "action_id"],
            "FeedbackButtonsElement",
        )?;
        let value = FeedbackButtonsElement {
            positive_button,
            negative_button,
            action_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "FeedbackButtonsElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "FeedbackButtonsElement.action_id")?;
        }
        crate::rules::validate("FeedbackButtonsElement", &wire, "FeedbackButtonsElement")?;
        Ok(value)
    }
}
impl Serialize for FeedbackButtonsElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for FeedbackButtonsElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "feedback_buttons", path)?;
        let positive_button =
            wire::field::<FeedbackButton>(&mut map, "positive_button", path, false)?;
        let negative_button =
            wire::field::<FeedbackButton>(&mut map, "negative_button", path, false)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        FeedbackButtonsElementBuilder {
            positive_button,
            negative_button,
            action_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for FeedbackButtonsElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "FeedbackButtonsElement")
    }
}
impl<'de> Deserialize<'de> for FeedbackButtonsElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An input that lets users upload files.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/file-input-element).

#[derive(Clone, Debug, PartialEq)]
pub struct FileInputElement {
    action_id: Option<String>,
    filetypes: Option<Vec<String>>,
    max_files: Option<i64>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`FileInputElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct FileInputElementBuilder {
    action_id: Option<String>,
    filetypes: Option<Vec<String>>,
    max_files: Option<i64>,
    extensions: Map<String, Value>,
}
impl Default for FileInputElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            filetypes: None,
            max_files: None,
            extensions: Map::new(),
        }
    }
}
impl FileInputElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> FileInputElementBuilder {
        FileInputElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> FileInputElementBuilder {
        FileInputElementBuilder {
            action_id: self.action_id,
            filetypes: self.filetypes,
            max_files: self.max_files,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `filetypes`. Adds accepted file extensions, such as pdf or png.
    pub fn filetypes(&self) -> Option<&[String]> {
        self.filetypes.as_deref()
    }
    /// Borrows or copies `max_files`. Sets the maximum number of files a user can upload.
    pub fn max_files(&self) -> Option<i64> {
        self.max_files
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
            map.serialize_entry("type", "file_input")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.filetypes {
            map.serialize_entry("filetypes", value)?;
        }
        if let Some(value) = &self.max_files {
            map.serialize_entry("max_files", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl FileInputElementBuilder {
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
    /// Adds accepted file extensions, such as pdf or png.
    /// Replaces the collection; order is preserved.
    pub fn filetypes<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.filetypes = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn filetype(mut self, value: impl Into<String>) -> Self {
        self.filetypes
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `filetypes`, including any model default.
    pub fn clear_filetypes(mut self) -> Self {
        self.filetypes = None;
        self
    }
    /// Sets the maximum number of files a user can upload.
    pub fn max_files(mut self, value: i64) -> Self {
        self.max_files = Some(value);
        self
    }
    /// Omits `max_files`, including any model default.
    pub fn clear_max_files(mut self) -> Self {
        self.max_files = None;
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
    pub fn build(self) -> Result<FileInputElement, ValidationError> {
        let action_id = self.action_id;
        let filetypes = self.filetypes;
        let max_files = self.max_files;
        wire::extensions(
            &self.extensions,
            &["action_id", "filetypes", "max_files"],
            "FileInputElement",
        )?;
        let value = FileInputElement {
            action_id,
            filetypes,
            max_files,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "FileInputElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "FileInputElement.action_id")?;
        }
        if let Some(v) = wire.get("max_files") {
            crate::rules::limits(v, "file_input.max_files", "FileInputElement.max_files")?;
        }
        crate::rules::validate("FileInputElement", &wire, "FileInputElement")?;
        Ok(value)
    }
}
impl Serialize for FileInputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for FileInputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "file_input", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let filetypes = wire::field::<Vec<String>>(&mut map, "filetypes", path, false)?;
        let max_files = wire::field::<i64>(&mut map, "max_files", path, false)?;
        FileInputElementBuilder {
            action_id,
            filetypes,
            max_files,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for FileInputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "FileInputElement")
    }
}
impl<'de> Deserialize<'de> for FileInputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A button that shows an icon instead of text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element).

#[derive(Clone, Debug, PartialEq)]
pub struct IconButtonElement {
    text: PlainText,
    icon: IconButtonIcon,
    action_id: Option<String>,
    value: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    accessibility_label: Option<String>,
    visible_to_user_ids: Option<Vec<String>>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`IconButtonElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct IconButtonElementBuilder {
    text: Option<PlainTextInput>,
    icon: Option<IconButtonIcon>,
    action_id: Option<String>,
    value: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    accessibility_label: Option<String>,
    visible_to_user_ids: Option<Vec<String>>,
    extensions: Map<String, Value>,
}
impl Default for IconButtonElementBuilder {
    fn default() -> Self {
        Self {
            text: None,
            icon: Some(IconButtonIcon::Trash),
            action_id: None,
            value: None,
            confirm: None,
            accessibility_label: None,
            visible_to_user_ids: None,
            extensions: Map::new(),
        }
    }
}
impl IconButtonElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> IconButtonElementBuilder {
        IconButtonElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> IconButtonElementBuilder {
        IconButtonElementBuilder {
            text: Some(self.text.into()),
            icon: Some(self.icon),
            action_id: self.action_id,
            value: self.value,
            confirm: self.confirm,
            accessibility_label: self.accessibility_label,
            visible_to_user_ids: self.visible_to_user_ids,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the text used as the button's accessible name.
    pub fn text(&self) -> &PlainText {
        &self.text
    }
    /// Borrows or copies `icon`. Sets the icon shown on the button.
    pub fn icon(&self) -> IconButtonIcon {
        self.icon
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `value`. Sets the application-defined value sent in interaction payloads.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `accessibility_label`. Sets the label read by screen readers in place of the visible text.
    pub fn accessibility_label(&self) -> Option<&str> {
        self.accessibility_label.as_deref()
    }
    /// Borrows or copies `visible_to_user_ids`. Adds the IDs of users who can see this button.
    pub fn visible_to_user_ids(&self) -> Option<&[String]> {
        self.visible_to_user_ids.as_deref()
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
            map.serialize_entry("type", "icon_button")?;
        }
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("icon", &self.icon)?;
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.value {
            map.serialize_entry("value", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.accessibility_label {
            map.serialize_entry("accessibility_label", value)?;
        }
        if let Some(value) = &self.visible_to_user_ids {
            map.serialize_entry("visible_to_user_ids", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl IconButtonElementBuilder {
    /// Sets the text used as the button's accessible name.
    pub fn text(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the icon shown on the button.
    pub fn icon(mut self, value: IconButtonIcon) -> Self {
        self.icon = Some(value);
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
    /// Adds the IDs of users who can see this button.
    /// Replaces the collection; order is preserved.
    pub fn visible_to_user_ids<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.visible_to_user_ids = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn visible_to_user_id(mut self, value: impl Into<String>) -> Self {
        self.visible_to_user_ids
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `visible_to_user_ids`, including any model default.
    pub fn clear_visible_to_user_ids(mut self) -> Self {
        self.visible_to_user_ids = None;
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
    pub fn build(self) -> Result<IconButtonElement, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve().map_err(|e| e.at("IconButtonElement.text")))
                .transpose()?,
            "IconButtonElement.text",
        )?;
        let icon = wire::required(self.icon, "IconButtonElement.icon")?;
        let action_id = self.action_id;
        let value = self.value;
        let confirm = self.confirm;
        let accessibility_label = self.accessibility_label;
        let visible_to_user_ids = self.visible_to_user_ids;
        wire::extensions(
            &self.extensions,
            &[
                "text",
                "icon",
                "action_id",
                "value",
                "confirm",
                "accessibility_label",
                "visible_to_user_ids",
            ],
            "IconButtonElement",
        )?;
        let value = IconButtonElement {
            text,
            icon,
            action_id,
            value,
            confirm,
            accessibility_label,
            visible_to_user_ids,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "IconButtonElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "IconButtonElement.action_id")?;
        }
        if let Some(v) = wire.get("value") {
            crate::rules::limits(v, "icon_button.value", "IconButtonElement.value")?;
        }
        if let Some(v) = wire.get("accessibility_label") {
            crate::rules::limits(
                v,
                "icon_button.accessibility_label",
                "IconButtonElement.accessibility_label",
            )?;
        }
        if let Some(v) = wire.get("visible_to_user_ids") {
            crate::rules::limits(
                v,
                "icon_button.visible_to_user_ids",
                "IconButtonElement.visible_to_user_ids",
            )?;
        }
        crate::rules::validate("IconButtonElement", &wire, "IconButtonElement")?;
        Ok(value)
    }
}
impl Serialize for IconButtonElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for IconButtonElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "icon_button", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "plain_text");
        let text = wire::field::<PlainText>(&mut map, "text", path, false)?;
        if !map.contains_key("icon") {
            map.insert("icon".into(), serde_json::json!("trash"));
        }
        let icon = wire::field::<IconButtonIcon>(&mut map, "icon", path, false)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let value = wire::field::<String>(&mut map, "value", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let accessibility_label =
            wire::field::<String>(&mut map, "accessibility_label", path, false)?;
        let visible_to_user_ids =
            wire::field::<Vec<String>>(&mut map, "visible_to_user_ids", path, false)?;
        IconButtonElementBuilder {
            text: text.map(Into::into),
            icon,
            action_id,
            value,
            confirm,
            accessibility_label,
            visible_to_user_ids,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for IconButtonElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "IconButtonElement")
    }
}
impl<'de> Deserialize<'de> for IconButtonElement {
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

/// An input that accepts whole or decimal numbers.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/number-input-element).
/// The minimum value cannot exceed the maximum value.
#[derive(Clone, Debug, PartialEq)]
pub struct NumberInputElement {
    action_id: Option<String>,
    is_decimal_allowed: bool,
    initial_value: Option<String>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`NumberInputElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct NumberInputElementBuilder {
    action_id: Option<String>,
    is_decimal_allowed: Option<bool>,
    initial_value: Option<String>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for NumberInputElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            is_decimal_allowed: None,
            initial_value: None,
            min_value: None,
            max_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl NumberInputElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> NumberInputElementBuilder {
        NumberInputElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> NumberInputElementBuilder {
        NumberInputElementBuilder {
            action_id: self.action_id,
            is_decimal_allowed: Some(self.is_decimal_allowed),
            initial_value: self.initial_value,
            min_value: self.min_value,
            max_value: self.max_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `is_decimal_allowed`. Sets whether the input accepts decimal numbers.
    pub fn is_decimal_allowed(&self) -> bool {
        self.is_decimal_allowed
    }
    /// Borrows or copies `initial_value`. Sets the number present when the input loads, as a string such as 42 or 3.5.
    pub fn initial_value(&self) -> Option<&str> {
        self.initial_value.as_deref()
    }
    /// Borrows or copies `min_value`. Sets the smallest accepted number.
    pub fn min_value(&self) -> Option<f64> {
        self.min_value
    }
    /// Borrows or copies `max_value`. Sets the largest accepted number.
    pub fn max_value(&self) -> Option<f64> {
        self.max_value
    }
    /// Borrows or copies `dispatch_action_config`. Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(&self) -> Option<&DispatchActionConfiguration> {
        self.dispatch_action_config.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "number_input")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        map.serialize_entry("is_decimal_allowed", &self.is_decimal_allowed)?;
        if let Some(value) = &self.initial_value {
            map.serialize_entry("initial_value", value)?;
        }
        if let Some(value) = &self.min_value {
            map.serialize_entry("min_value", value)?;
        }
        if let Some(value) = &self.max_value {
            map.serialize_entry("max_value", value)?;
        }
        if let Some(value) = &self.dispatch_action_config {
            map.serialize_entry("dispatch_action_config", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl NumberInputElementBuilder {
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
    /// Sets whether the input accepts decimal numbers.
    pub fn is_decimal_allowed(mut self, value: bool) -> Self {
        self.is_decimal_allowed = Some(value);
        self
    }
    /// Sets the number present when the input loads, as a string such as 42 or 3.5.
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
        self
    }
    /// Omits `initial_value`, including any model default.
    pub fn clear_initial_value(mut self) -> Self {
        self.initial_value = None;
        self
    }
    /// Sets the smallest accepted number.
    pub fn min_value(mut self, value: f64) -> Self {
        self.min_value = Some(value);
        self
    }
    /// Omits `min_value`, including any model default.
    pub fn clear_min_value(mut self) -> Self {
        self.min_value = None;
        self
    }
    /// Sets the largest accepted number.
    pub fn max_value(mut self, value: f64) -> Self {
        self.max_value = Some(value);
        self
    }
    /// Omits `max_value`, including any model default.
    pub fn clear_max_value(mut self) -> Self {
        self.max_value = None;
        self
    }
    /// Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(mut self, value: impl Into<DispatchActionConfiguration>) -> Self {
        self.dispatch_action_config = Some(value.into());
        self
    }
    /// Omits `dispatch_action_config`, including any model default.
    pub fn clear_dispatch_action_config(mut self) -> Self {
        self.dispatch_action_config = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<NumberInputElement, ValidationError> {
        let action_id = self.action_id;
        let is_decimal_allowed = wire::required(
            self.is_decimal_allowed,
            "NumberInputElement.is_decimal_allowed",
        )?;
        let initial_value = self.initial_value;
        let min_value = self.min_value;
        if min_value.is_some_and(|v| !v.is_finite()) {
            return Err(ValidationError::new(
                ErrorCategory::OutOfRange,
                "NumberInputElement.min_value",
                "expected a finite number",
            ));
        }
        let max_value = self.max_value;
        if max_value.is_some_and(|v| !v.is_finite()) {
            return Err(ValidationError::new(
                ErrorCategory::OutOfRange,
                "NumberInputElement.max_value",
                "expected a finite number",
            ));
        }
        let dispatch_action_config = self.dispatch_action_config;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("NumberInputElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "is_decimal_allowed",
                "initial_value",
                "min_value",
                "max_value",
                "dispatch_action_config",
                "focus_on_load",
                "placeholder",
            ],
            "NumberInputElement",
        )?;
        let value = NumberInputElement {
            action_id,
            is_decimal_allowed,
            initial_value,
            min_value,
            max_value,
            dispatch_action_config,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "NumberInputElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "NumberInputElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "number_input.placeholder",
                "NumberInputElement.placeholder",
            )?;
        }
        crate::rules::validate("NumberInputElement", &wire, "NumberInputElement")?;
        Ok(value)
    }
}
impl Serialize for NumberInputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for NumberInputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "number_input", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let is_decimal_allowed = wire::field::<bool>(&mut map, "is_decimal_allowed", path, false)?;
        let initial_value = wire::field::<String>(&mut map, "initial_value", path, false)?;
        let min_value = wire::field::<f64>(&mut map, "min_value", path, false)?;
        let max_value = wire::field::<f64>(&mut map, "max_value", path, false)?;
        let dispatch_action_config = wire::field::<DispatchActionConfiguration>(
            &mut map,
            "dispatch_action_config",
            path,
            false,
        )?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        NumberInputElementBuilder {
            action_id,
            is_decimal_allowed,
            initial_value,
            min_value,
            max_value,
            dispatch_action_config,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for NumberInputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "NumberInputElement")
    }
}
impl<'de> Deserialize<'de> for NumberInputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A compact menu of up to five options.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct OverflowElement {
    action_id: Option<String>,
    options: Vec<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`OverflowElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct OverflowElementBuilder {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    extensions: Map<String, Value>,
}
impl Default for OverflowElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            options: None,
            confirm: None,
            extensions: Map::new(),
        }
    }
}
impl OverflowElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> OverflowElementBuilder {
        OverflowElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> OverflowElementBuilder {
        OverflowElementBuilder {
            action_id: self.action_id,
            options: Some(self.options),
            confirm: self.confirm,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `options`. Adds selectable options in display order.
    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
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
            map.serialize_entry("type", "overflow")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        map.serialize_entry("options", &self.options)?;
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl OverflowElementBuilder {
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
    /// Adds selectable options in display order.
    /// Replaces the collection; order is preserved.
    pub fn options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option(mut self, value: impl Into<SelectOption>) -> Self {
        self.options.get_or_insert_with(Vec::new).push(value.into());
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
    pub fn build(self) -> Result<OverflowElement, ValidationError> {
        let action_id = self.action_id;
        let options = wire::required(self.options, "OverflowElement.options")?;
        let confirm = self.confirm;
        wire::extensions(
            &self.extensions,
            &["action_id", "options", "confirm"],
            "OverflowElement",
        )?;
        let value = OverflowElement {
            action_id,
            options,
            confirm,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "OverflowElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "OverflowElement.action_id")?;
        }
        if let Some(v) = wire.get("options") {
            crate::rules::limits(v, "overflow.options", "OverflowElement.options")?;
        }
        crate::rules::validate("OverflowElement", &wire, "OverflowElement")?;
        Ok(value)
    }
}
impl Serialize for OverflowElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for OverflowElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "overflow", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let options = wire::field::<Vec<SelectOption>>(&mut map, "options", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        OverflowElementBuilder {
            action_id,
            options,
            confirm,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for OverflowElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "OverflowElement")
    }
}
impl<'de> Deserialize<'de> for OverflowElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A single-line or multi-line free-text input.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/plain-text-input-element).

#[derive(Clone, Debug, PartialEq)]
pub struct PlainTextInputElement {
    action_id: Option<String>,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<i64>,
    max_length: Option<i64>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`PlainTextInputElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct PlainTextInputElementBuilder {
    action_id: Option<String>,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<i64>,
    max_length: Option<i64>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for PlainTextInputElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl PlainTextInputElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> PlainTextInputElementBuilder {
        PlainTextInputElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> PlainTextInputElementBuilder {
        PlainTextInputElementBuilder {
            action_id: self.action_id,
            initial_value: self.initial_value,
            multiline: self.multiline,
            min_length: self.min_length,
            max_length: self.max_length,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_value`. Sets the value present when the input loads.
    pub fn initial_value(&self) -> Option<&str> {
        self.initial_value.as_deref()
    }
    /// Borrows or copies `multiline`. Sets whether the input is a multi-line text area.
    pub fn multiline(&self) -> Option<bool> {
        self.multiline
    }
    /// Borrows or copies `min_length`. Sets the minimum number of characters the user must enter.
    pub fn min_length(&self) -> Option<i64> {
        self.min_length
    }
    /// Borrows or copies `max_length`. Sets the maximum number of characters the user can enter.
    pub fn max_length(&self) -> Option<i64> {
        self.max_length
    }
    /// Borrows or copies `dispatch_action_config`. Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(&self) -> Option<&DispatchActionConfiguration> {
        self.dispatch_action_config.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "plain_text_input")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_value {
            map.serialize_entry("initial_value", value)?;
        }
        if let Some(value) = &self.multiline {
            map.serialize_entry("multiline", value)?;
        }
        if let Some(value) = &self.min_length {
            map.serialize_entry("min_length", value)?;
        }
        if let Some(value) = &self.max_length {
            map.serialize_entry("max_length", value)?;
        }
        if let Some(value) = &self.dispatch_action_config {
            map.serialize_entry("dispatch_action_config", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PlainTextInputElementBuilder {
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
    /// Sets the value present when the input loads.
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
        self
    }
    /// Omits `initial_value`, including any model default.
    pub fn clear_initial_value(mut self) -> Self {
        self.initial_value = None;
        self
    }
    /// Sets whether the input is a multi-line text area.
    pub fn multiline(mut self, value: bool) -> Self {
        self.multiline = Some(value);
        self
    }
    /// Omits `multiline`, including any model default.
    pub fn clear_multiline(mut self) -> Self {
        self.multiline = None;
        self
    }
    /// Sets the minimum number of characters the user must enter.
    pub fn min_length(mut self, value: i64) -> Self {
        self.min_length = Some(value);
        self
    }
    /// Omits `min_length`, including any model default.
    pub fn clear_min_length(mut self) -> Self {
        self.min_length = None;
        self
    }
    /// Sets the maximum number of characters the user can enter.
    pub fn max_length(mut self, value: i64) -> Self {
        self.max_length = Some(value);
        self
    }
    /// Omits `max_length`, including any model default.
    pub fn clear_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }
    /// Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(mut self, value: impl Into<DispatchActionConfiguration>) -> Self {
        self.dispatch_action_config = Some(value.into());
        self
    }
    /// Omits `dispatch_action_config`, including any model default.
    pub fn clear_dispatch_action_config(mut self) -> Self {
        self.dispatch_action_config = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<PlainTextInputElement, ValidationError> {
        let action_id = self.action_id;
        let initial_value = self.initial_value;
        let multiline = self.multiline;
        let min_length = self.min_length;
        let max_length = self.max_length;
        let dispatch_action_config = self.dispatch_action_config;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("PlainTextInputElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_value",
                "multiline",
                "min_length",
                "max_length",
                "dispatch_action_config",
                "focus_on_load",
                "placeholder",
            ],
            "PlainTextInputElement",
        )?;
        let value = PlainTextInputElement {
            action_id,
            initial_value,
            multiline,
            min_length,
            max_length,
            dispatch_action_config,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "PlainTextInputElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "PlainTextInputElement.action_id")?;
        }
        if let Some(v) = wire.get("min_length") {
            crate::rules::limits(
                v,
                "plain_text_input.min_length",
                "PlainTextInputElement.min_length",
            )?;
        }
        if let Some(v) = wire.get("max_length") {
            crate::rules::limits(
                v,
                "plain_text_input.max_length",
                "PlainTextInputElement.max_length",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "plain_text_input.placeholder",
                "PlainTextInputElement.placeholder",
            )?;
        }
        crate::rules::validate("PlainTextInputElement", &wire, "PlainTextInputElement")?;
        Ok(value)
    }
}
impl Serialize for PlainTextInputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for PlainTextInputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "plain_text_input", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_value = wire::field::<String>(&mut map, "initial_value", path, false)?;
        let multiline = wire::field::<bool>(&mut map, "multiline", path, false)?;
        let min_length = wire::field::<i64>(&mut map, "min_length", path, false)?;
        let max_length = wire::field::<i64>(&mut map, "max_length", path, false)?;
        let dispatch_action_config = wire::field::<DispatchActionConfiguration>(
            &mut map,
            "dispatch_action_config",
            path,
            false,
        )?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        PlainTextInputElementBuilder {
            action_id,
            initial_value,
            multiline,
            min_length,
            max_length,
            dispatch_action_config,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for PlainTextInputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "PlainTextInputElement")
    }
}
impl<'de> Deserialize<'de> for PlainTextInputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A group of radio buttons.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element).

#[derive(Clone, Debug, PartialEq)]
pub struct RadioButtonsElement {
    action_id: Option<String>,
    options: Vec<SelectOption>,
    initial_option: Option<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RadioButtonsElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RadioButtonsElementBuilder {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    initial_option: Option<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for RadioButtonsElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            options: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
            extensions: Map::new(),
        }
    }
}
impl RadioButtonsElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RadioButtonsElementBuilder {
        RadioButtonsElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RadioButtonsElementBuilder {
        RadioButtonsElementBuilder {
            action_id: self.action_id,
            options: Some(self.options),
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `options`. Adds selectable options in display order.
    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }
    /// Borrows or copies `initial_option`. Sets the option selected when the element loads. It must match an option in the element.
    pub fn initial_option(&self) -> Option<&SelectOption> {
        self.initial_option.as_ref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
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
            map.serialize_entry("type", "radio_buttons")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        map.serialize_entry("options", &self.options)?;
        if let Some(value) = &self.initial_option {
            map.serialize_entry("initial_option", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RadioButtonsElementBuilder {
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
    /// Adds selectable options in display order.
    /// Replaces the collection; order is preserved.
    pub fn options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option(mut self, value: impl Into<SelectOption>) -> Self {
        self.options.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Sets the option selected when the element loads. It must match an option in the element.
    pub fn initial_option(mut self, value: impl Into<SelectOption>) -> Self {
        self.initial_option = Some(value.into());
        self
    }
    /// Omits `initial_option`, including any model default.
    pub fn clear_initial_option(mut self) -> Self {
        self.initial_option = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
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
    pub fn build(self) -> Result<RadioButtonsElement, ValidationError> {
        let action_id = self.action_id;
        let options = wire::required(self.options, "RadioButtonsElement.options")?;
        let initial_option = self.initial_option;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "options",
                "initial_option",
                "confirm",
                "focus_on_load",
            ],
            "RadioButtonsElement",
        )?;
        let value = RadioButtonsElement {
            action_id,
            options,
            initial_option,
            confirm,
            focus_on_load,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "RadioButtonsElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "RadioButtonsElement.action_id")?;
        }
        if let Some(v) = wire.get("options") {
            crate::rules::limits(v, "radio_buttons.options", "RadioButtonsElement.options")?;
        }
        crate::rules::validate("RadioButtonsElement", &wire, "RadioButtonsElement")?;
        Ok(value)
    }
}
impl Serialize for RadioButtonsElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RadioButtonsElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "radio_buttons", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let options = wire::field::<Vec<SelectOption>>(&mut map, "options", path, false)?;
        let initial_option = wire::field::<SelectOption>(&mut map, "initial_option", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        RadioButtonsElementBuilder {
            action_id,
            options,
            initial_option,
            confirm,
            focus_on_load,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RadioButtonsElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RadioButtonsElement")
    }
}
impl<'de> Deserialize<'de> for RadioButtonsElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An input that accepts formatted rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/rich-text-input-element).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextInputElement {
    action_id: String,
    initial_value: Option<RichTextBlock>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    min_lines: Option<i64>,
    max_lines: Option<i64>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextInputElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextInputElementBuilder {
    action_id: Option<String>,
    initial_value: Option<RichTextBlock>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    min_lines: Option<i64>,
    max_lines: Option<i64>,
    extensions: Map<String, Value>,
}
impl Default for RichTextInputElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
            min_lines: None,
            max_lines: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextInputElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextInputElementBuilder {
        RichTextInputElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextInputElementBuilder {
        RichTextInputElementBuilder {
            action_id: Some(self.action_id),
            initial_value: self.initial_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            min_lines: self.min_lines,
            max_lines: self.max_lines,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> &str {
        &self.action_id
    }
    /// Borrows or copies `initial_value`. Sets the rich text present when the input loads.
    pub fn initial_value(&self) -> Option<&RichTextBlock> {
        self.initial_value.as_ref()
    }
    /// Borrows or copies `dispatch_action_config`. Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(&self) -> Option<&DispatchActionConfiguration> {
        self.dispatch_action_config.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
    }
    /// Borrows or copies `min_lines`. Sets the minimum visible height of the input, in lines.
    pub fn min_lines(&self) -> Option<i64> {
        self.min_lines
    }
    /// Borrows or copies `max_lines`. Sets the maximum visible height of the input, in lines.
    pub fn max_lines(&self) -> Option<i64> {
        self.max_lines
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
            map.serialize_entry("type", "rich_text_input")?;
        }
        map.serialize_entry("action_id", &self.action_id)?;
        if let Some(value) = &self.initial_value {
            map.serialize_entry("initial_value", value)?;
        }
        if let Some(value) = &self.dispatch_action_config {
            map.serialize_entry("dispatch_action_config", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        if let Some(value) = &self.min_lines {
            map.serialize_entry("min_lines", value)?;
        }
        if let Some(value) = &self.max_lines {
            map.serialize_entry("max_lines", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextInputElementBuilder {
    /// Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(mut self, value: impl Into<String>) -> Self {
        self.action_id = Some(value.into());
        self
    }
    /// Sets the rich text present when the input loads.
    pub fn initial_value(mut self, value: impl Into<RichTextBlock>) -> Self {
        self.initial_value = Some(value.into());
        self
    }
    /// Omits `initial_value`, including any model default.
    pub fn clear_initial_value(mut self) -> Self {
        self.initial_value = None;
        self
    }
    /// Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(mut self, value: impl Into<DispatchActionConfiguration>) -> Self {
        self.dispatch_action_config = Some(value.into());
        self
    }
    /// Omits `dispatch_action_config`, including any model default.
    pub fn clear_dispatch_action_config(mut self) -> Self {
        self.dispatch_action_config = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
        self
    }
    /// Sets the minimum visible height of the input, in lines.
    pub fn min_lines(mut self, value: i64) -> Self {
        self.min_lines = Some(value);
        self
    }
    /// Omits `min_lines`, including any model default.
    pub fn clear_min_lines(mut self) -> Self {
        self.min_lines = None;
        self
    }
    /// Sets the maximum visible height of the input, in lines.
    pub fn max_lines(mut self, value: i64) -> Self {
        self.max_lines = Some(value);
        self
    }
    /// Omits `max_lines`, including any model default.
    pub fn clear_max_lines(mut self) -> Self {
        self.max_lines = None;
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
    pub fn build(self) -> Result<RichTextInputElement, ValidationError> {
        let action_id = wire::required(self.action_id, "RichTextInputElement.action_id")?;
        let initial_value = self.initial_value;
        let dispatch_action_config = self.dispatch_action_config;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("RichTextInputElement.placeholder"))
            })
            .transpose()?;
        let min_lines = self.min_lines;
        let max_lines = self.max_lines;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_value",
                "dispatch_action_config",
                "focus_on_load",
                "placeholder",
                "min_lines",
                "max_lines",
            ],
            "RichTextInputElement",
        )?;
        let value = RichTextInputElement {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder,
            min_lines,
            max_lines,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "RichTextInputElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "RichTextInputElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "rich_text_input.placeholder",
                "RichTextInputElement.placeholder",
            )?;
        }
        if let Some(v) = wire.get("min_lines") {
            crate::rules::limits(
                v,
                "rich_text_input.min_lines",
                "RichTextInputElement.min_lines",
            )?;
        }
        if let Some(v) = wire.get("max_lines") {
            crate::rules::limits(
                v,
                "rich_text_input.max_lines",
                "RichTextInputElement.max_lines",
            )?;
        }
        crate::rules::validate("RichTextInputElement", &wire, "RichTextInputElement")?;
        Ok(value)
    }
}
impl Serialize for RichTextInputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextInputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "rich_text_input", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_value = wire::field::<RichTextBlock>(&mut map, "initial_value", path, false)?;
        let dispatch_action_config = wire::field::<DispatchActionConfiguration>(
            &mut map,
            "dispatch_action_config",
            path,
            false,
        )?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        let min_lines = wire::field::<i64>(&mut map, "min_lines", path, false)?;
        let max_lines = wire::field::<i64>(&mut map, "max_lines", path, false)?;
        RichTextInputElementBuilder {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            min_lines,
            max_lines,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextInputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextInputElement")
    }
}
impl<'de> Deserialize<'de> for RichTextInputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting multiple options defined in the payload.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element).
/// Use options or option groups, not both.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticMultiSelectElement {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    option_groups: Option<Vec<SelectOptionGroup>>,
    initial_options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`StaticMultiSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct StaticMultiSelectElementBuilder {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    option_groups: Option<Vec<SelectOptionGroup>>,
    initial_options: Option<Vec<SelectOption>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for StaticMultiSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            options: None,
            option_groups: None,
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl StaticMultiSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> StaticMultiSelectElementBuilder {
        StaticMultiSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> StaticMultiSelectElementBuilder {
        StaticMultiSelectElementBuilder {
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
            initial_options: self.initial_options,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `options`. Adds selectable options in display order.
    pub fn options(&self) -> Option<&[SelectOption]> {
        self.options.as_deref()
    }
    /// Borrows or copies `option_groups`. Adds labelled groups of options in display order. Cannot be combined with ungrouped options.
    pub fn option_groups(&self) -> Option<&[SelectOptionGroup]> {
        self.option_groups.as_deref()
    }
    /// Borrows or copies `initial_options`. Adds options that are selected when the element loads. Each must match an option in the element.
    pub fn initial_options(&self) -> Option<&[SelectOption]> {
        self.initial_options.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `max_selected_items`. Sets the maximum number of items a user can select.
    pub fn max_selected_items(&self) -> Option<i64> {
        self.max_selected_items
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "multi_static_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.options {
            map.serialize_entry("options", value)?;
        }
        if let Some(value) = &self.option_groups {
            map.serialize_entry("option_groups", value)?;
        }
        if let Some(value) = &self.initial_options {
            map.serialize_entry("initial_options", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.max_selected_items {
            map.serialize_entry("max_selected_items", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticMultiSelectElementBuilder {
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
    /// Adds selectable options in display order.
    /// Replaces the collection; order is preserved.
    pub fn options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option(mut self, value: impl Into<SelectOption>) -> Self {
        self.options.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `options`, including any model default.
    pub fn clear_options(mut self) -> Self {
        self.options = None;
        self
    }
    /// Adds labelled groups of options in display order. Cannot be combined with ungrouped options.
    /// Replaces the collection; order is preserved.
    pub fn option_groups<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOptionGroup>,
    {
        self.option_groups = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option_group(mut self, value: impl Into<SelectOptionGroup>) -> Self {
        self.option_groups
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `option_groups`, including any model default.
    pub fn clear_option_groups(mut self) -> Self {
        self.option_groups = None;
        self
    }
    /// Adds options that are selected when the element loads. Each must match an option in the element.
    /// Replaces the collection; order is preserved.
    pub fn initial_options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.initial_options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn initial_option(mut self, value: impl Into<SelectOption>) -> Self {
        self.initial_options
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `initial_options`, including any model default.
    pub fn clear_initial_options(mut self) -> Self {
        self.initial_options = None;
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
    /// Sets the maximum number of items a user can select.
    pub fn max_selected_items(mut self, value: i64) -> Self {
        self.max_selected_items = Some(value);
        self
    }
    /// Omits `max_selected_items`, including any model default.
    pub fn clear_max_selected_items(mut self) -> Self {
        self.max_selected_items = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<StaticMultiSelectElement, ValidationError> {
        let action_id = self.action_id;
        let options = self.options;
        let option_groups = self.option_groups;
        let initial_options = self.initial_options;
        let confirm = self.confirm;
        let max_selected_items = self.max_selected_items;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("StaticMultiSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "options",
                "option_groups",
                "initial_options",
                "confirm",
                "max_selected_items",
                "focus_on_load",
                "placeholder",
            ],
            "StaticMultiSelectElement",
        )?;
        let value = StaticMultiSelectElement {
            action_id,
            options,
            option_groups,
            initial_options,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "StaticMultiSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "StaticMultiSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("options") {
            crate::rules::limits(v, "select.options", "StaticMultiSelectElement.options")?;
        }
        if let Some(v) = wire.get("option_groups") {
            crate::rules::limits(
                v,
                "select.option_groups",
                "StaticMultiSelectElement.option_groups",
            )?;
        }
        if let Some(v) = wire.get("max_selected_items") {
            crate::rules::limits(
                v,
                "multi_select.max_selected_items",
                "StaticMultiSelectElement.max_selected_items",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "select.placeholder",
                "StaticMultiSelectElement.placeholder",
            )?;
        }
        crate::rules::validate(
            "StaticMultiSelectElement",
            &wire,
            "StaticMultiSelectElement",
        )?;
        Ok(value)
    }
}
impl Serialize for StaticMultiSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for StaticMultiSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "multi_static_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let options = wire::field::<Vec<SelectOption>>(&mut map, "options", path, false)?;
        let option_groups =
            wire::field::<Vec<SelectOptionGroup>>(&mut map, "option_groups", path, false)?;
        let initial_options =
            wire::field::<Vec<SelectOption>>(&mut map, "initial_options", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let max_selected_items = wire::field::<i64>(&mut map, "max_selected_items", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        StaticMultiSelectElementBuilder {
            action_id,
            options,
            option_groups,
            initial_options,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for StaticMultiSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "StaticMultiSelectElement")
    }
}
impl<'de> Deserialize<'de> for StaticMultiSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting one option defined in the payload.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element).
/// Use options or option groups, not both.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticSelectElement {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    option_groups: Option<Vec<SelectOptionGroup>>,
    initial_option: Option<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`StaticSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct StaticSelectElementBuilder {
    action_id: Option<String>,
    options: Option<Vec<SelectOption>>,
    option_groups: Option<Vec<SelectOptionGroup>>,
    initial_option: Option<SelectOption>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for StaticSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            options: None,
            option_groups: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl StaticSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> StaticSelectElementBuilder {
        StaticSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> StaticSelectElementBuilder {
        StaticSelectElementBuilder {
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `options`. Adds selectable options in display order.
    pub fn options(&self) -> Option<&[SelectOption]> {
        self.options.as_deref()
    }
    /// Borrows or copies `option_groups`. Adds labelled groups of options in display order. Cannot be combined with ungrouped options.
    pub fn option_groups(&self) -> Option<&[SelectOptionGroup]> {
        self.option_groups.as_deref()
    }
    /// Borrows or copies `initial_option`. Sets the option selected when the element loads. It must match an option in the element.
    pub fn initial_option(&self) -> Option<&SelectOption> {
        self.initial_option.as_ref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "static_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.options {
            map.serialize_entry("options", value)?;
        }
        if let Some(value) = &self.option_groups {
            map.serialize_entry("option_groups", value)?;
        }
        if let Some(value) = &self.initial_option {
            map.serialize_entry("initial_option", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticSelectElementBuilder {
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
    /// Adds selectable options in display order.
    /// Replaces the collection; order is preserved.
    pub fn options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option(mut self, value: impl Into<SelectOption>) -> Self {
        self.options.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `options`, including any model default.
    pub fn clear_options(mut self) -> Self {
        self.options = None;
        self
    }
    /// Adds labelled groups of options in display order. Cannot be combined with ungrouped options.
    /// Replaces the collection; order is preserved.
    pub fn option_groups<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOptionGroup>,
    {
        self.option_groups = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option_group(mut self, value: impl Into<SelectOptionGroup>) -> Self {
        self.option_groups
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `option_groups`, including any model default.
    pub fn clear_option_groups(mut self) -> Self {
        self.option_groups = None;
        self
    }
    /// Sets the option selected when the element loads. It must match an option in the element.
    pub fn initial_option(mut self, value: impl Into<SelectOption>) -> Self {
        self.initial_option = Some(value.into());
        self
    }
    /// Omits `initial_option`, including any model default.
    pub fn clear_initial_option(mut self) -> Self {
        self.initial_option = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<StaticSelectElement, ValidationError> {
        let action_id = self.action_id;
        let options = self.options;
        let option_groups = self.option_groups;
        let initial_option = self.initial_option;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("StaticSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "options",
                "option_groups",
                "initial_option",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "StaticSelectElement",
        )?;
        let value = StaticSelectElement {
            action_id,
            options,
            option_groups,
            initial_option,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "StaticSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "StaticSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("options") {
            crate::rules::limits(v, "select.options", "StaticSelectElement.options")?;
        }
        if let Some(v) = wire.get("option_groups") {
            crate::rules::limits(
                v,
                "select.option_groups",
                "StaticSelectElement.option_groups",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(v, "select.placeholder", "StaticSelectElement.placeholder")?;
        }
        crate::rules::validate("StaticSelectElement", &wire, "StaticSelectElement")?;
        Ok(value)
    }
}
impl Serialize for StaticSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for StaticSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "static_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let options = wire::field::<Vec<SelectOption>>(&mut map, "options", path, false)?;
        let option_groups =
            wire::field::<Vec<SelectOptionGroup>>(&mut map, "option_groups", path, false)?;
        let initial_option = wire::field::<SelectOption>(&mut map, "initial_option", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        StaticSelectElementBuilder {
            action_id,
            options,
            option_groups,
            initial_option,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for StaticSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "StaticSelectElement")
    }
}
impl<'de> Deserialize<'de> for StaticSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A time-of-day picker.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element).

#[derive(Clone, Debug, PartialEq)]
pub struct TimePickerElement {
    action_id: Option<String>,
    initial_time: Option<String>,
    timezone: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`TimePickerElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct TimePickerElementBuilder {
    action_id: Option<String>,
    initial_time: Option<String>,
    timezone: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for TimePickerElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_time: None,
            timezone: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl TimePickerElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> TimePickerElementBuilder {
        TimePickerElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> TimePickerElementBuilder {
        TimePickerElementBuilder {
            action_id: self.action_id,
            initial_time: self.initial_time,
            timezone: self.timezone,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_time`. Sets the initially selected time, formatted as HH:mm in 24-hour time.
    pub fn initial_time(&self) -> Option<&str> {
        self.initial_time.as_deref()
    }
    /// Borrows or copies `timezone`. Sets the IANA time zone used to display the time, such as Australia/Brisbane.
    pub fn timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "timepicker")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_time {
            map.serialize_entry("initial_time", value)?;
        }
        if let Some(value) = &self.timezone {
            map.serialize_entry("timezone", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TimePickerElementBuilder {
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
    /// Sets the initially selected time, formatted as HH:mm in 24-hour time.
    pub fn initial_time(mut self, value: impl Into<String>) -> Self {
        self.initial_time = Some(value.into());
        self
    }
    /// Omits `initial_time`, including any model default.
    pub fn clear_initial_time(mut self) -> Self {
        self.initial_time = None;
        self
    }
    /// Sets the IANA time zone used to display the time, such as Australia/Brisbane.
    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }
    /// Omits `timezone`, including any model default.
    pub fn clear_timezone(mut self) -> Self {
        self.timezone = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<TimePickerElement, ValidationError> {
        let action_id = self.action_id;
        let initial_time = self.initial_time;
        let timezone = self.timezone;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("TimePickerElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_time",
                "timezone",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "TimePickerElement",
        )?;
        let value = TimePickerElement {
            action_id,
            initial_time,
            timezone,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "TimePickerElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "TimePickerElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "time_picker.placeholder",
                "TimePickerElement.placeholder",
            )?;
        }
        crate::rules::validate("TimePickerElement", &wire, "TimePickerElement")?;
        Ok(value)
    }
}
impl Serialize for TimePickerElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for TimePickerElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "timepicker", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_time = wire::field::<String>(&mut map, "initial_time", path, false)?;
        let timezone = wire::field::<String>(&mut map, "timezone", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        TimePickerElementBuilder {
            action_id,
            initial_time,
            timezone,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for TimePickerElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "TimePickerElement")
    }
}
impl<'de> Deserialize<'de> for TimePickerElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A single-line input that accepts a URL.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/url-input-element).

#[derive(Clone, Debug, PartialEq)]
pub struct UrlInputElement {
    action_id: Option<String>,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`UrlInputElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct UrlInputElementBuilder {
    action_id: Option<String>,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for UrlInputElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl UrlInputElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> UrlInputElementBuilder {
        UrlInputElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> UrlInputElementBuilder {
        UrlInputElementBuilder {
            action_id: self.action_id,
            initial_value: self.initial_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_value`. Sets the value present when the input loads.
    pub fn initial_value(&self) -> Option<&str> {
        self.initial_value.as_deref()
    }
    /// Borrows or copies `dispatch_action_config`. Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(&self) -> Option<&DispatchActionConfiguration> {
        self.dispatch_action_config.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "url_text_input")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_value {
            map.serialize_entry("initial_value", value)?;
        }
        if let Some(value) = &self.dispatch_action_config {
            map.serialize_entry("dispatch_action_config", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl UrlInputElementBuilder {
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
    /// Sets the value present when the input loads.
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
        self
    }
    /// Omits `initial_value`, including any model default.
    pub fn clear_initial_value(mut self) -> Self {
        self.initial_value = None;
        self
    }
    /// Sets which user interactions send a block_actions payload.
    pub fn dispatch_action_config(mut self, value: impl Into<DispatchActionConfiguration>) -> Self {
        self.dispatch_action_config = Some(value.into());
        self
    }
    /// Omits `dispatch_action_config`, including any model default.
    pub fn clear_dispatch_action_config(mut self) -> Self {
        self.dispatch_action_config = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<UrlInputElement, ValidationError> {
        let action_id = self.action_id;
        let initial_value = self.initial_value;
        let dispatch_action_config = self.dispatch_action_config;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| v.resolve().map_err(|e| e.at("UrlInputElement.placeholder")))
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_value",
                "dispatch_action_config",
                "focus_on_load",
                "placeholder",
            ],
            "UrlInputElement",
        )?;
        let value = UrlInputElement {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "UrlInputElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "UrlInputElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(v, "url_input.placeholder", "UrlInputElement.placeholder")?;
        }
        crate::rules::validate("UrlInputElement", &wire, "UrlInputElement")?;
        Ok(value)
    }
}
impl Serialize for UrlInputElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for UrlInputElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "url_text_input", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_value = wire::field::<String>(&mut map, "initial_value", path, false)?;
        let dispatch_action_config = wire::field::<DispatchActionConfiguration>(
            &mut map,
            "dispatch_action_config",
            path,
            false,
        )?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        UrlInputElementBuilder {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for UrlInputElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "UrlInputElement")
    }
}
impl<'de> Deserialize<'de> for UrlInputElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting multiple workspace users.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct UserMultiSelectElement {
    action_id: Option<String>,
    initial_users: Option<Vec<String>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`UserMultiSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct UserMultiSelectElementBuilder {
    action_id: Option<String>,
    initial_users: Option<Vec<String>>,
    confirm: Option<ConfirmationDialogue>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for UserMultiSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_users: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl UserMultiSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> UserMultiSelectElementBuilder {
        UserMultiSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> UserMultiSelectElementBuilder {
        UserMultiSelectElementBuilder {
            action_id: self.action_id,
            initial_users: self.initial_users,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_users`. Adds user IDs that are selected when the menu loads.
    pub fn initial_users(&self) -> Option<&[String]> {
        self.initial_users.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `max_selected_items`. Sets the maximum number of items a user can select.
    pub fn max_selected_items(&self) -> Option<i64> {
        self.max_selected_items
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "multi_users_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_users {
            map.serialize_entry("initial_users", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.max_selected_items {
            map.serialize_entry("max_selected_items", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl UserMultiSelectElementBuilder {
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
    /// Adds user IDs that are selected when the menu loads.
    /// Replaces the collection; order is preserved.
    pub fn initial_users<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.initial_users = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn initial_user(mut self, value: impl Into<String>) -> Self {
        self.initial_users
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `initial_users`, including any model default.
    pub fn clear_initial_users(mut self) -> Self {
        self.initial_users = None;
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
    /// Sets the maximum number of items a user can select.
    pub fn max_selected_items(mut self, value: i64) -> Self {
        self.max_selected_items = Some(value);
        self
    }
    /// Omits `max_selected_items`, including any model default.
    pub fn clear_max_selected_items(mut self) -> Self {
        self.max_selected_items = None;
        self
    }
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<UserMultiSelectElement, ValidationError> {
        let action_id = self.action_id;
        let initial_users = self.initial_users;
        let confirm = self.confirm;
        let max_selected_items = self.max_selected_items;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("UserMultiSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_users",
                "confirm",
                "max_selected_items",
                "focus_on_load",
                "placeholder",
            ],
            "UserMultiSelectElement",
        )?;
        let value = UserMultiSelectElement {
            action_id,
            initial_users,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "UserMultiSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "UserMultiSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("max_selected_items") {
            crate::rules::limits(
                v,
                "multi_select.max_selected_items",
                "UserMultiSelectElement.max_selected_items",
            )?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(
                v,
                "select.placeholder",
                "UserMultiSelectElement.placeholder",
            )?;
        }
        crate::rules::validate("UserMultiSelectElement", &wire, "UserMultiSelectElement")?;
        Ok(value)
    }
}
impl Serialize for UserMultiSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for UserMultiSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "multi_users_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_users = wire::field::<Vec<String>>(&mut map, "initial_users", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let max_selected_items = wire::field::<i64>(&mut map, "max_selected_items", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        UserMultiSelectElementBuilder {
            action_id,
            initial_users,
            confirm,
            max_selected_items,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for UserMultiSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "UserMultiSelectElement")
    }
}
impl<'de> Deserialize<'de> for UserMultiSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A menu for selecting one workspace user.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element).

#[derive(Clone, Debug, PartialEq)]
pub struct UserSelectElement {
    action_id: Option<String>,
    initial_user: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`UserSelectElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct UserSelectElementBuilder {
    action_id: Option<String>,
    initial_user: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainTextInput>,
    extensions: Map<String, Value>,
}
impl Default for UserSelectElementBuilder {
    fn default() -> Self {
        Self {
            action_id: None,
            initial_user: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            extensions: Map::new(),
        }
    }
}
impl UserSelectElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> UserSelectElementBuilder {
        UserSelectElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> UserSelectElementBuilder {
        UserSelectElementBuilder {
            action_id: self.action_id,
            initial_user: self.initial_user,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder.map(Into::into),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> Option<&str> {
        self.action_id.as_deref()
    }
    /// Borrows or copies `initial_user`. Sets the user ID selected when the menu loads.
    pub fn initial_user(&self) -> Option<&str> {
        self.initial_user.as_deref()
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `focus_on_load`. Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(&self) -> Option<bool> {
        self.focus_on_load
    }
    /// Borrows or copies `placeholder`. Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(&self) -> Option<&PlainText> {
        self.placeholder.as_ref()
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
            map.serialize_entry("type", "users_select")?;
        }
        if let Some(value) = &self.action_id {
            map.serialize_entry("action_id", value)?;
        }
        if let Some(value) = &self.initial_user {
            map.serialize_entry("initial_user", value)?;
        }
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.focus_on_load {
            map.serialize_entry("focus_on_load", value)?;
        }
        if let Some(value) = &self.placeholder {
            map.serialize_entry("placeholder", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl UserSelectElementBuilder {
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
    /// Sets the user ID selected when the menu loads.
    pub fn initial_user(mut self, value: impl Into<String>) -> Self {
        self.initial_user = Some(value.into());
        self
    }
    /// Omits `initial_user`, including any model default.
    pub fn clear_initial_user(mut self) -> Self {
        self.initial_user = None;
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
    /// Sets whether the element receives focus when the view opens. Only one element per view may do so.
    pub fn focus_on_load(mut self, value: bool) -> Self {
        self.focus_on_load = Some(value);
        self
    }
    /// Omits `focus_on_load`, including any model default.
    pub fn clear_focus_on_load(mut self) -> Self {
        self.focus_on_load = None;
        self
    }
    /// Sets the placeholder text shown before a value is chosen.
    pub fn placeholder(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    /// Omits `placeholder`, including any model default.
    pub fn clear_placeholder(mut self) -> Self {
        self.placeholder = None;
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
    pub fn build(self) -> Result<UserSelectElement, ValidationError> {
        let action_id = self.action_id;
        let initial_user = self.initial_user;
        let confirm = self.confirm;
        let focus_on_load = self.focus_on_load;
        let placeholder = self
            .placeholder
            .map(|v| {
                v.resolve()
                    .map_err(|e| e.at("UserSelectElement.placeholder"))
            })
            .transpose()?;
        wire::extensions(
            &self.extensions,
            &[
                "action_id",
                "initial_user",
                "confirm",
                "focus_on_load",
                "placeholder",
            ],
            "UserSelectElement",
        )?;
        let value = UserSelectElement {
            action_id,
            initial_user,
            confirm,
            focus_on_load,
            placeholder,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "UserSelectElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "UserSelectElement.action_id")?;
        }
        if let Some(v) = wire.get("placeholder") {
            crate::rules::limits(v, "select.placeholder", "UserSelectElement.placeholder")?;
        }
        crate::rules::validate("UserSelectElement", &wire, "UserSelectElement")?;
        Ok(value)
    }
}
impl Serialize for UserSelectElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for UserSelectElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "users_select", path)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let initial_user = wire::field::<String>(&mut map, "initial_user", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let focus_on_load = wire::field::<bool>(&mut map, "focus_on_load", path, false)?;
        crate::rules::coerce_text(map.get_mut("placeholder"), "plain_text");
        let placeholder = wire::field::<PlainText>(&mut map, "placeholder", path, false)?;
        UserSelectElementBuilder {
            action_id,
            initial_user,
            confirm,
            focus_on_load,
            placeholder: placeholder.map(Into::into),
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for UserSelectElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "UserSelectElement")
    }
}
impl<'de> Deserialize<'de> for UserSelectElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A button that starts a workflow through a link trigger.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element).

#[derive(Clone, Debug, PartialEq)]
pub struct WorkflowButtonElement {
    text: PlainText,
    workflow: Workflow,
    action_id: String,
    confirm: Option<ConfirmationDialogue>,
    style: Option<ButtonStyle>,
    accessibility_label: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`WorkflowButtonElement`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct WorkflowButtonElementBuilder {
    text: Option<PlainTextInput>,
    workflow: Option<Workflow>,
    action_id: Option<String>,
    confirm: Option<ConfirmationDialogue>,
    style: Option<ButtonStyle>,
    accessibility_label: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for WorkflowButtonElementBuilder {
    fn default() -> Self {
        Self {
            text: None,
            workflow: None,
            action_id: None,
            confirm: None,
            style: None,
            accessibility_label: None,
            extensions: Map::new(),
        }
    }
}
impl WorkflowButtonElement {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> WorkflowButtonElementBuilder {
        WorkflowButtonElementBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> WorkflowButtonElementBuilder {
        WorkflowButtonElementBuilder {
            text: Some(self.text.into()),
            workflow: Some(self.workflow),
            action_id: Some(self.action_id),
            confirm: self.confirm,
            style: self.style,
            accessibility_label: self.accessibility_label,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the button label.
    pub fn text(&self) -> &PlainText {
        &self.text
    }
    /// Borrows or copies `workflow`. Sets the workflow started when the button is clicked.
    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }
    /// Borrows or copies `action_id`. Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(&self) -> &str {
        &self.action_id
    }
    /// Borrows or copies `confirm`. Sets a confirmation dialog shown before the action is sent.
    pub fn confirm(&self) -> Option<&ConfirmationDialogue> {
        self.confirm.as_ref()
    }
    /// Borrows or copies `style`. Sets the button's emphasis. Omit it for the default neutral style.
    pub fn style(&self) -> Option<ButtonStyle> {
        self.style
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
            map.serialize_entry("type", "workflow_button")?;
        }
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("workflow", &self.workflow)?;
        map.serialize_entry("action_id", &self.action_id)?;
        if let Some(value) = &self.confirm {
            map.serialize_entry("confirm", value)?;
        }
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
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
impl WorkflowButtonElementBuilder {
    /// Sets the button label.
    pub fn text(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the workflow started when the button is clicked.
    pub fn workflow(mut self, value: impl Into<Workflow>) -> Self {
        self.workflow = Some(value.into());
        self
    }
    /// Sets the identifier Slack returns in interaction payloads when a user acts on this element. It must be unique among the elements of its block.
    pub fn action_id(mut self, value: impl Into<String>) -> Self {
        self.action_id = Some(value.into());
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
    pub fn build(self) -> Result<WorkflowButtonElement, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve().map_err(|e| e.at("WorkflowButtonElement.text")))
                .transpose()?,
            "WorkflowButtonElement.text",
        )?;
        let workflow = wire::required(self.workflow, "WorkflowButtonElement.workflow")?;
        let action_id = wire::required(self.action_id, "WorkflowButtonElement.action_id")?;
        let confirm = self.confirm;
        let style = self.style;
        let accessibility_label = self.accessibility_label;
        wire::extensions(
            &self.extensions,
            &[
                "text",
                "workflow",
                "action_id",
                "confirm",
                "style",
                "accessibility_label",
            ],
            "WorkflowButtonElement",
        )?;
        let value = WorkflowButtonElement {
            text,
            workflow,
            action_id,
            confirm,
            style,
            accessibility_label,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "WorkflowButtonElement",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "workflow_button.text", "WorkflowButtonElement.text")?;
        }
        if let Some(v) = wire.get("action_id") {
            crate::rules::limits(v, "action_id", "WorkflowButtonElement.action_id")?;
        }
        if let Some(v) = wire.get("accessibility_label") {
            crate::rules::limits(
                v,
                "workflow_button.accessibility_label",
                "WorkflowButtonElement.accessibility_label",
            )?;
        }
        crate::rules::validate("WorkflowButtonElement", &wire, "WorkflowButtonElement")?;
        Ok(value)
    }
}
impl Serialize for WorkflowButtonElement {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for WorkflowButtonElement {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "workflow_button", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "plain_text");
        let text = wire::field::<PlainText>(&mut map, "text", path, false)?;
        let workflow = wire::field::<Workflow>(&mut map, "workflow", path, false)?;
        let action_id = wire::field::<String>(&mut map, "action_id", path, false)?;
        let confirm = wire::field::<ConfirmationDialogue>(&mut map, "confirm", path, false)?;
        let style = wire::field::<ButtonStyle>(&mut map, "style", path, false)?;
        let accessibility_label =
            wire::field::<String>(&mut map, "accessibility_label", path, false)?;
        WorkflowButtonElementBuilder {
            text: text.map(Into::into),
            workflow,
            action_id,
            confirm,
            style,
            accessibility_label,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for WorkflowButtonElement {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "WorkflowButtonElement")
    }
}
impl<'de> Deserialize<'de> for WorkflowButtonElement {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An area chart for a data visualization block.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).
/// Every series needs exactly one data point for each axis category.
#[derive(Clone, Debug, PartialEq)]
pub struct AreaChart {
    series: Vec<DataSeries>,
    axis_config: AxisConfig,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`AreaChart`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct AreaChartBuilder {
    series: Option<Vec<DataSeries>>,
    axis_config: Option<AxisConfig>,
    extensions: Map<String, Value>,
}
impl Default for AreaChartBuilder {
    fn default() -> Self {
        Self {
            series: None,
            axis_config: None,
            extensions: Map::new(),
        }
    }
}
impl AreaChart {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> AreaChartBuilder {
        AreaChartBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> AreaChartBuilder {
        AreaChartBuilder {
            series: Some(self.series),
            axis_config: Some(self.axis_config),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `series`. Adds data series in display order. Series names must be unique.
    pub fn series(&self) -> &[DataSeries] {
        &self.series
    }
    /// Borrows or copies `axis_config`. Sets the axis categories and labels. Every series needs exactly one point per category.
    pub fn axis_config(&self) -> &AxisConfig {
        &self.axis_config
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
            map.serialize_entry("type", "area")?;
        }
        map.serialize_entry("series", &self.series)?;
        map.serialize_entry("axis_config", &self.axis_config)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AreaChartBuilder {
    /// Adds data series in display order. Series names must be unique.
    /// Replaces the collection; order is preserved.
    pub fn series<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<DataSeries>,
    {
        self.series = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn series_entry(mut self, value: impl Into<DataSeries>) -> Self {
        self.series.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Sets the axis categories and labels. Every series needs exactly one point per category.
    pub fn axis_config(mut self, value: impl Into<AxisConfig>) -> Self {
        self.axis_config = Some(value.into());
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
    pub fn build(self) -> Result<AreaChart, ValidationError> {
        let series = wire::required(self.series, "AreaChart.series")?;
        let axis_config = wire::required(self.axis_config, "AreaChart.axis_config")?;
        wire::extensions(&self.extensions, &["series", "axis_config"], "AreaChart")?;
        let value = AreaChart {
            series,
            axis_config,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "AreaChart", e.to_string())
        })?;
        if let Some(v) = wire.get("series") {
            crate::rules::limits(v, "data_visualization.series", "AreaChart.series")?;
        }
        crate::rules::validate("AreaChart", &wire, "AreaChart")?;
        Ok(value)
    }
}
impl Serialize for AreaChart {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for AreaChart {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "area", path)?;
        let series = wire::field::<Vec<DataSeries>>(&mut map, "series", path, false)?;
        let axis_config = wire::field::<AxisConfig>(&mut map, "axis_config", path, false)?;
        AreaChartBuilder {
            series,
            axis_config,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for AreaChart {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "AreaChart")
    }
}
impl<'de> Deserialize<'de> for AreaChart {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// The categories and labels shared by bar, area, and line chart axes.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).

#[derive(Clone, Debug, PartialEq)]
pub struct AxisConfig {
    categories: Vec<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`AxisConfig`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct AxisConfigBuilder {
    categories: Option<Vec<String>>,
    x_label: Option<String>,
    y_label: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for AxisConfigBuilder {
    fn default() -> Self {
        Self {
            categories: None,
            x_label: None,
            y_label: None,
            extensions: Map::new(),
        }
    }
}
impl AxisConfig {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> AxisConfigBuilder {
        AxisConfigBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> AxisConfigBuilder {
        AxisConfigBuilder {
            categories: Some(self.categories),
            x_label: self.x_label,
            y_label: self.y_label,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `categories`. Adds axis categories in display order. Labels must be unique.
    pub fn categories(&self) -> &[String] {
        &self.categories
    }
    /// Borrows or copies `x_label`. Sets the horizontal axis label.
    pub fn x_label(&self) -> Option<&str> {
        self.x_label.as_deref()
    }
    /// Borrows or copies `y_label`. Sets the vertical axis label.
    pub fn y_label(&self) -> Option<&str> {
        self.y_label.as_deref()
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
        map.serialize_entry("categories", &self.categories)?;
        if let Some(value) = &self.x_label {
            map.serialize_entry("x_label", value)?;
        }
        if let Some(value) = &self.y_label {
            map.serialize_entry("y_label", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AxisConfigBuilder {
    /// Adds axis categories in display order. Labels must be unique.
    /// Replaces the collection; order is preserved.
    pub fn categories<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.categories = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.categories
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Sets the horizontal axis label.
    pub fn x_label(mut self, value: impl Into<String>) -> Self {
        self.x_label = Some(value.into());
        self
    }
    /// Omits `x_label`, including any model default.
    pub fn clear_x_label(mut self) -> Self {
        self.x_label = None;
        self
    }
    /// Sets the vertical axis label.
    pub fn y_label(mut self, value: impl Into<String>) -> Self {
        self.y_label = Some(value.into());
        self
    }
    /// Omits `y_label`, including any model default.
    pub fn clear_y_label(mut self) -> Self {
        self.y_label = None;
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
    pub fn build(self) -> Result<AxisConfig, ValidationError> {
        let categories = wire::required(self.categories, "AxisConfig.categories")?;
        let x_label = self.x_label;
        let y_label = self.y_label;
        wire::extensions(
            &self.extensions,
            &["categories", "x_label", "y_label"],
            "AxisConfig",
        )?;
        let value = AxisConfig {
            categories,
            x_label,
            y_label,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "AxisConfig", e.to_string())
        })?;
        if let Some(v) = wire.get("categories") {
            crate::rules::limits(v, "data_visualization.categories", "AxisConfig.categories")?;
        }
        if let Some(v) = wire.get("x_label") {
            crate::rules::limits(v, "data_visualization.axis_label", "AxisConfig.x_label")?;
        }
        if let Some(v) = wire.get("y_label") {
            crate::rules::limits(v, "data_visualization.axis_label", "AxisConfig.y_label")?;
        }
        crate::rules::validate("AxisConfig", &wire, "AxisConfig")?;
        Ok(value)
    }
}
impl Serialize for AxisConfig {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for AxisConfig {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let categories = wire::field::<Vec<String>>(&mut map, "categories", path, false)?;
        let x_label = wire::field::<String>(&mut map, "x_label", path, false)?;
        let y_label = wire::field::<String>(&mut map, "y_label", path, false)?;
        AxisConfigBuilder {
            categories,
            x_label,
            y_label,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for AxisConfig {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "AxisConfig")
    }
}
impl<'de> Deserialize<'de> for AxisConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A bar chart for a data visualization block.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).
/// Every series needs exactly one data point for each axis category.
#[derive(Clone, Debug, PartialEq)]
pub struct BarChart {
    series: Vec<DataSeries>,
    axis_config: AxisConfig,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`BarChart`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct BarChartBuilder {
    series: Option<Vec<DataSeries>>,
    axis_config: Option<AxisConfig>,
    extensions: Map<String, Value>,
}
impl Default for BarChartBuilder {
    fn default() -> Self {
        Self {
            series: None,
            axis_config: None,
            extensions: Map::new(),
        }
    }
}
impl BarChart {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> BarChartBuilder {
        BarChartBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> BarChartBuilder {
        BarChartBuilder {
            series: Some(self.series),
            axis_config: Some(self.axis_config),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `series`. Adds data series in display order. Series names must be unique.
    pub fn series(&self) -> &[DataSeries] {
        &self.series
    }
    /// Borrows or copies `axis_config`. Sets the axis categories and labels. Every series needs exactly one point per category.
    pub fn axis_config(&self) -> &AxisConfig {
        &self.axis_config
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
            map.serialize_entry("type", "bar")?;
        }
        map.serialize_entry("series", &self.series)?;
        map.serialize_entry("axis_config", &self.axis_config)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl BarChartBuilder {
    /// Adds data series in display order. Series names must be unique.
    /// Replaces the collection; order is preserved.
    pub fn series<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<DataSeries>,
    {
        self.series = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn series_entry(mut self, value: impl Into<DataSeries>) -> Self {
        self.series.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Sets the axis categories and labels. Every series needs exactly one point per category.
    pub fn axis_config(mut self, value: impl Into<AxisConfig>) -> Self {
        self.axis_config = Some(value.into());
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
    pub fn build(self) -> Result<BarChart, ValidationError> {
        let series = wire::required(self.series, "BarChart.series")?;
        let axis_config = wire::required(self.axis_config, "BarChart.axis_config")?;
        wire::extensions(&self.extensions, &["series", "axis_config"], "BarChart")?;
        let value = BarChart {
            series,
            axis_config,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "BarChart", e.to_string())
        })?;
        if let Some(v) = wire.get("series") {
            crate::rules::limits(v, "data_visualization.series", "BarChart.series")?;
        }
        crate::rules::validate("BarChart", &wire, "BarChart")?;
        Ok(value)
    }
}
impl Serialize for BarChart {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for BarChart {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "bar", path)?;
        let series = wire::field::<Vec<DataSeries>>(&mut map, "series", path, false)?;
        let axis_config = wire::field::<AxisConfig>(&mut map, "axis_config", path, false)?;
        BarChartBuilder {
            series,
            axis_config,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for BarChart {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "BarChart")
    }
}
impl<'de> Deserialize<'de> for BarChart {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// One labelled segment of a pie chart.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).

#[derive(Clone, Debug, PartialEq)]
pub struct ChartSegment {
    label: String,
    value: JsonNumber,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ChartSegment`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ChartSegmentBuilder {
    label: Option<String>,
    value: Option<JsonNumber>,
    extensions: Map<String, Value>,
}
impl Default for ChartSegmentBuilder {
    fn default() -> Self {
        Self {
            label: None,
            value: None,
            extensions: Map::new(),
        }
    }
}
impl ChartSegment {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ChartSegmentBuilder {
        ChartSegmentBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ChartSegmentBuilder {
        ChartSegmentBuilder {
            label: Some(self.label),
            value: Some(self.value),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `label`. Sets the segment label.
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Borrows or copies `value`. Sets the segment's size. Must be greater than zero.
    pub fn value(&self) -> &JsonNumber {
        &self.value
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
        map.serialize_entry("label", &self.label)?;
        map.serialize_entry("value", &self.value)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ChartSegmentBuilder {
    /// Sets the segment label.
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
    /// Sets the segment's size. Must be greater than zero.
    pub fn value(mut self, value: impl Into<JsonNumber>) -> Self {
        self.value = Some(value.into());
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
    pub fn build(self) -> Result<ChartSegment, ValidationError> {
        let label = wire::required(self.label, "ChartSegment.label")?;
        let value = wire::required(self.value, "ChartSegment.value")?;
        wire::extensions(&self.extensions, &["label", "value"], "ChartSegment")?;
        let value = ChartSegment {
            label,
            value,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ChartSegment", e.to_string())
        })?;
        if let Some(v) = wire.get("label") {
            crate::rules::limits(v, "data_visualization.segment.label", "ChartSegment.label")?;
        }
        crate::rules::validate("ChartSegment", &wire, "ChartSegment")?;
        Ok(value)
    }
}
impl Serialize for ChartSegment {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ChartSegment {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let label = wire::field::<String>(&mut map, "label", path, false)?;
        let value = wire::field::<JsonNumber>(&mut map, "value", path, false)?;
        ChartSegmentBuilder {
            label,
            value,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ChartSegment {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ChartSegment")
    }
}
impl<'de> Deserialize<'de> for ChartSegment {
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

/// Limits which conversations a conversation menu offers.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object).
/// Set at least one filter. Include only im, mpim, private, and public.
#[derive(Clone, Debug, PartialEq)]
pub struct ConversationFilter {
    include: Option<Vec<String>>,
    exclude_external_shared_channels: Option<bool>,
    exclude_bot_users: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ConversationFilter`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ConversationFilterBuilder {
    include: Option<Vec<String>>,
    exclude_external_shared_channels: Option<bool>,
    exclude_bot_users: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for ConversationFilterBuilder {
    fn default() -> Self {
        Self {
            include: None,
            exclude_external_shared_channels: None,
            exclude_bot_users: None,
            extensions: Map::new(),
        }
    }
}
impl ConversationFilter {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ConversationFilterBuilder {
        ConversationFilterBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ConversationFilterBuilder {
        ConversationFilterBuilder {
            include: self.include,
            exclude_external_shared_channels: self.exclude_external_shared_channels,
            exclude_bot_users: self.exclude_bot_users,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `include`. Adds conversation types to offer: im, mpim, private, or public.
    pub fn include(&self) -> Option<&[String]> {
        self.include.as_deref()
    }
    /// Borrows or copies `exclude_external_shared_channels`. Sets whether externally shared channels are excluded.
    pub fn exclude_external_shared_channels(&self) -> Option<bool> {
        self.exclude_external_shared_channels
    }
    /// Borrows or copies `exclude_bot_users`. Sets whether bot users are excluded.
    pub fn exclude_bot_users(&self) -> Option<bool> {
        self.exclude_bot_users
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
        if let Some(value) = &self.include {
            map.serialize_entry("include", value)?;
        }
        if let Some(value) = &self.exclude_external_shared_channels {
            map.serialize_entry("exclude_external_shared_channels", value)?;
        }
        if let Some(value) = &self.exclude_bot_users {
            map.serialize_entry("exclude_bot_users", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ConversationFilterBuilder {
    /// Adds conversation types to offer: im, mpim, private, or public.
    /// Replaces the collection; order is preserved.
    pub fn include<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.include = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn add_include(mut self, value: impl Into<String>) -> Self {
        self.include.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Omits `include`, including any model default.
    pub fn clear_include(mut self) -> Self {
        self.include = None;
        self
    }
    /// Sets whether externally shared channels are excluded.
    pub fn exclude_external_shared_channels(mut self, value: bool) -> Self {
        self.exclude_external_shared_channels = Some(value);
        self
    }
    /// Omits `exclude_external_shared_channels`, including any model default.
    pub fn clear_exclude_external_shared_channels(mut self) -> Self {
        self.exclude_external_shared_channels = None;
        self
    }
    /// Sets whether bot users are excluded.
    pub fn exclude_bot_users(mut self, value: bool) -> Self {
        self.exclude_bot_users = Some(value);
        self
    }
    /// Omits `exclude_bot_users`, including any model default.
    pub fn clear_exclude_bot_users(mut self) -> Self {
        self.exclude_bot_users = None;
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
    pub fn build(self) -> Result<ConversationFilter, ValidationError> {
        let include = self.include;
        let exclude_external_shared_channels = self.exclude_external_shared_channels;
        let exclude_bot_users = self.exclude_bot_users;
        wire::extensions(
            &self.extensions,
            &[
                "include",
                "exclude_external_shared_channels",
                "exclude_bot_users",
            ],
            "ConversationFilter",
        )?;
        let value = ConversationFilter {
            include,
            exclude_external_shared_channels,
            exclude_bot_users,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "ConversationFilter",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("include") {
            crate::rules::limits(
                v,
                "conversation_filter.include",
                "ConversationFilter.include",
            )?;
        }
        crate::rules::validate("ConversationFilter", &wire, "ConversationFilter")?;
        Ok(value)
    }
}
impl Serialize for ConversationFilter {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ConversationFilter {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let include = wire::field::<Vec<String>>(&mut map, "include", path, false)?;
        let exclude_external_shared_channels =
            wire::field::<bool>(&mut map, "exclude_external_shared_channels", path, false)?;
        let exclude_bot_users = wire::field::<bool>(&mut map, "exclude_bot_users", path, false)?;
        ConversationFilterBuilder {
            include,
            exclude_external_shared_channels,
            exclude_bot_users,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ConversationFilter {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ConversationFilter")
    }
}
impl<'de> Deserialize<'de> for ConversationFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// One value of a chart series, keyed by axis category.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).

#[derive(Clone, Debug, PartialEq)]
pub struct DataPoint {
    label: String,
    value: JsonNumber,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DataPoint`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DataPointBuilder {
    label: Option<String>,
    value: Option<JsonNumber>,
    extensions: Map<String, Value>,
}
impl Default for DataPointBuilder {
    fn default() -> Self {
        Self {
            label: None,
            value: None,
            extensions: Map::new(),
        }
    }
}
impl DataPoint {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DataPointBuilder {
        DataPointBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DataPointBuilder {
        DataPointBuilder {
            label: Some(self.label),
            value: Some(self.value),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `label`. Sets the axis category this point belongs to.
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Borrows or copies `value`. Sets the point's value.
    pub fn value(&self) -> &JsonNumber {
        &self.value
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
        map.serialize_entry("label", &self.label)?;
        map.serialize_entry("value", &self.value)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DataPointBuilder {
    /// Sets the axis category this point belongs to.
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
    /// Sets the point's value.
    pub fn value(mut self, value: impl Into<JsonNumber>) -> Self {
        self.value = Some(value.into());
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
    pub fn build(self) -> Result<DataPoint, ValidationError> {
        let label = wire::required(self.label, "DataPoint.label")?;
        let value = wire::required(self.value, "DataPoint.value")?;
        wire::extensions(&self.extensions, &["label", "value"], "DataPoint")?;
        let value = DataPoint {
            label,
            value,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "DataPoint", e.to_string())
        })?;
        if let Some(v) = wire.get("label") {
            crate::rules::limits(v, "data_visualization.point_label", "DataPoint.label")?;
        }
        crate::rules::validate("DataPoint", &wire, "DataPoint")?;
        Ok(value)
    }
}
impl Serialize for DataPoint {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DataPoint {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let label = wire::field::<String>(&mut map, "label", path, false)?;
        let value = wire::field::<JsonNumber>(&mut map, "value", path, false)?;
        DataPointBuilder {
            label,
            value,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DataPoint {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DataPoint")
    }
}
impl<'de> Deserialize<'de> for DataPoint {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A named series of chart data points.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).

#[derive(Clone, Debug, PartialEq)]
pub struct DataSeries {
    name: String,
    data: Vec<DataPoint>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DataSeries`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DataSeriesBuilder {
    name: Option<String>,
    data: Option<Vec<DataPoint>>,
    extensions: Map<String, Value>,
}
impl Default for DataSeriesBuilder {
    fn default() -> Self {
        Self {
            name: None,
            data: None,
            extensions: Map::new(),
        }
    }
}
impl DataSeries {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DataSeriesBuilder {
        DataSeriesBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DataSeriesBuilder {
        DataSeriesBuilder {
            name: Some(self.name),
            data: Some(self.data),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `name`. Sets the series name shown in the legend.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Borrows or copies `data`. Adds data points in axis-category order.
    pub fn data(&self) -> &[DataPoint] {
        &self.data
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
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DataSeriesBuilder {
    /// Sets the series name shown in the legend.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Adds data points in axis-category order.
    /// Replaces the collection; order is preserved.
    pub fn data<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<DataPoint>,
    {
        self.data = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn point(mut self, value: impl Into<DataPoint>) -> Self {
        self.data.get_or_insert_with(Vec::new).push(value.into());
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
    pub fn build(self) -> Result<DataSeries, ValidationError> {
        let name = wire::required(self.name, "DataSeries.name")?;
        let data = wire::required(self.data, "DataSeries.data")?;
        wire::extensions(&self.extensions, &["name", "data"], "DataSeries")?;
        let value = DataSeries {
            name,
            data,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "DataSeries", e.to_string())
        })?;
        if let Some(v) = wire.get("name") {
            crate::rules::limits(v, "data_visualization.series_name", "DataSeries.name")?;
        }
        if let Some(v) = wire.get("data") {
            crate::rules::limits(v, "data_visualization.data", "DataSeries.data")?;
        }
        crate::rules::validate("DataSeries", &wire, "DataSeries")?;
        Ok(value)
    }
}
impl Serialize for DataSeries {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DataSeries {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let name = wire::field::<String>(&mut map, "name", path, false)?;
        let data = wire::field::<Vec<DataPoint>>(&mut map, "data", path, false)?;
        DataSeriesBuilder {
            name,
            data,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DataSeries {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DataSeries")
    }
}
impl<'de> Deserialize<'de> for DataSeries {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Chooses which interactions with an input send a block_actions payload.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object).

#[derive(Clone, Debug, PartialEq)]
pub struct DispatchActionConfiguration {
    trigger_actions_on: Option<Vec<String>>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`DispatchActionConfiguration`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct DispatchActionConfigurationBuilder {
    trigger_actions_on: Option<Vec<String>>,
    extensions: Map<String, Value>,
}
impl Default for DispatchActionConfigurationBuilder {
    fn default() -> Self {
        Self {
            trigger_actions_on: None,
            extensions: Map::new(),
        }
    }
}
impl DispatchActionConfiguration {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> DispatchActionConfigurationBuilder {
        DispatchActionConfigurationBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> DispatchActionConfigurationBuilder {
        DispatchActionConfigurationBuilder {
            trigger_actions_on: self.trigger_actions_on,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `trigger_actions_on`. Adds the interactions that send a payload: on_enter_pressed or on_character_entered.
    pub fn trigger_actions_on(&self) -> Option<&[String]> {
        self.trigger_actions_on.as_deref()
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
        if let Some(value) = &self.trigger_actions_on {
            map.serialize_entry("trigger_actions_on", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DispatchActionConfigurationBuilder {
    /// Adds the interactions that send a payload: on_enter_pressed or on_character_entered.
    /// Replaces the collection; order is preserved.
    pub fn trigger_actions_on<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.trigger_actions_on = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn trigger_action(mut self, value: impl Into<String>) -> Self {
        self.trigger_actions_on
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `trigger_actions_on`, including any model default.
    pub fn clear_trigger_actions_on(mut self) -> Self {
        self.trigger_actions_on = None;
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
    pub fn build(self) -> Result<DispatchActionConfiguration, ValidationError> {
        let trigger_actions_on = self.trigger_actions_on;
        wire::extensions(
            &self.extensions,
            &["trigger_actions_on"],
            "DispatchActionConfiguration",
        )?;
        let value = DispatchActionConfiguration {
            trigger_actions_on,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "DispatchActionConfiguration",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("trigger_actions_on") {
            crate::rules::limits(
                v,
                "dispatch_action_configuration.trigger_actions_on",
                "DispatchActionConfiguration.trigger_actions_on",
            )?;
        }
        crate::rules::validate(
            "DispatchActionConfiguration",
            &wire,
            "DispatchActionConfiguration",
        )?;
        Ok(value)
    }
}
impl Serialize for DispatchActionConfiguration {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for DispatchActionConfiguration {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let trigger_actions_on =
            wire::field::<Vec<String>>(&mut map, "trigger_actions_on", path, false)?;
        DispatchActionConfigurationBuilder {
            trigger_actions_on,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for DispatchActionConfiguration {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "DispatchActionConfiguration")
    }
}
impl<'de> Deserialize<'de> for DispatchActionConfiguration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// One button of a feedback buttons element.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element).

#[derive(Clone, Debug, PartialEq)]
pub struct FeedbackButton {
    text: PlainText,
    value: String,
    accessibility_label: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`FeedbackButton`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct FeedbackButtonBuilder {
    text: Option<PlainTextInput>,
    value: Option<String>,
    accessibility_label: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for FeedbackButtonBuilder {
    fn default() -> Self {
        Self {
            text: None,
            value: None,
            accessibility_label: None,
            extensions: Map::new(),
        }
    }
}
impl FeedbackButton {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> FeedbackButtonBuilder {
        FeedbackButtonBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> FeedbackButtonBuilder {
        FeedbackButtonBuilder {
            text: Some(self.text.into()),
            value: Some(self.value),
            accessibility_label: self.accessibility_label,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `text`. Sets the button label.
    pub fn text(&self) -> &PlainText {
        &self.text
    }
    /// Borrows or copies `value`. Sets the application-defined value sent in interaction payloads.
    pub fn value(&self) -> &str {
        &self.value
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
        let _ = tagged;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("value", &self.value)?;
        if let Some(value) = &self.accessibility_label {
            map.serialize_entry("accessibility_label", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl FeedbackButtonBuilder {
    /// Sets the button label.
    pub fn text(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Sets the application-defined value sent in interaction payloads.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
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
    pub fn build(self) -> Result<FeedbackButton, ValidationError> {
        let text = wire::required(
            self.text
                .map(|v| v.resolve().map_err(|e| e.at("FeedbackButton.text")))
                .transpose()?,
            "FeedbackButton.text",
        )?;
        let value = wire::required(self.value, "FeedbackButton.value")?;
        let accessibility_label = self.accessibility_label;
        wire::extensions(
            &self.extensions,
            &["text", "value", "accessibility_label"],
            "FeedbackButton",
        )?;
        let value = FeedbackButton {
            text,
            value,
            accessibility_label,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "FeedbackButton", e.to_string())
        })?;
        if let Some(v) = wire.get("text") {
            crate::rules::limits(v, "feedback_button.text", "FeedbackButton.text")?;
        }
        if let Some(v) = wire.get("value") {
            crate::rules::limits(v, "feedback_button.value", "FeedbackButton.value")?;
        }
        if let Some(v) = wire.get("accessibility_label") {
            crate::rules::limits(
                v,
                "feedback_button.accessibility_label",
                "FeedbackButton.accessibility_label",
            )?;
        }
        crate::rules::validate("FeedbackButton", &wire, "FeedbackButton")?;
        Ok(value)
    }
}
impl Serialize for FeedbackButton {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for FeedbackButton {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        crate::rules::coerce_text(map.get_mut("text"), "plain_text");
        let text = wire::field::<PlainText>(&mut map, "text", path, false)?;
        let value = wire::field::<String>(&mut map, "value", path, false)?;
        let accessibility_label =
            wire::field::<String>(&mut map, "accessibility_label", path, false)?;
        FeedbackButtonBuilder {
            text: text.map(Into::into),
            value,
            accessibility_label,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for FeedbackButton {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "FeedbackButton")
    }
}
impl<'de> Deserialize<'de> for FeedbackButton {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A named value passed to a workflow trigger.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object).

#[derive(Clone, Debug, PartialEq)]
pub struct InputParameter {
    name: String,
    value: String,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`InputParameter`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct InputParameterBuilder {
    name: Option<String>,
    value: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for InputParameterBuilder {
    fn default() -> Self {
        Self {
            name: None,
            value: None,
            extensions: Map::new(),
        }
    }
}
impl InputParameter {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> InputParameterBuilder {
        InputParameterBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> InputParameterBuilder {
        InputParameterBuilder {
            name: Some(self.name),
            value: Some(self.value),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `name`. Sets the workflow input parameter name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Borrows or copies `value`. Sets the value passed to the workflow input.
    pub fn value(&self) -> &str {
        &self.value
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
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("value", &self.value)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl InputParameterBuilder {
    /// Sets the workflow input parameter name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets the value passed to the workflow input.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
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
    pub fn build(self) -> Result<InputParameter, ValidationError> {
        let name = wire::required(self.name, "InputParameter.name")?;
        let value = wire::required(self.value, "InputParameter.value")?;
        wire::extensions(&self.extensions, &["name", "value"], "InputParameter")?;
        let value = InputParameter {
            name,
            value,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "InputParameter", e.to_string())
        })?;
        crate::rules::validate("InputParameter", &wire, "InputParameter")?;
        Ok(value)
    }
}
impl Serialize for InputParameter {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for InputParameter {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let name = wire::field::<String>(&mut map, "name", path, false)?;
        let value = wire::field::<String>(&mut map, "value", path, false)?;
        InputParameterBuilder {
            name,
            value,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for InputParameter {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "InputParameter")
    }
}
impl<'de> Deserialize<'de> for InputParameter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A line chart for a data visualization block.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).
/// Every series needs exactly one data point for each axis category.
#[derive(Clone, Debug, PartialEq)]
pub struct LineChart {
    series: Vec<DataSeries>,
    axis_config: AxisConfig,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`LineChart`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct LineChartBuilder {
    series: Option<Vec<DataSeries>>,
    axis_config: Option<AxisConfig>,
    extensions: Map<String, Value>,
}
impl Default for LineChartBuilder {
    fn default() -> Self {
        Self {
            series: None,
            axis_config: None,
            extensions: Map::new(),
        }
    }
}
impl LineChart {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> LineChartBuilder {
        LineChartBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> LineChartBuilder {
        LineChartBuilder {
            series: Some(self.series),
            axis_config: Some(self.axis_config),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `series`. Adds data series in display order. Series names must be unique.
    pub fn series(&self) -> &[DataSeries] {
        &self.series
    }
    /// Borrows or copies `axis_config`. Sets the axis categories and labels. Every series needs exactly one point per category.
    pub fn axis_config(&self) -> &AxisConfig {
        &self.axis_config
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
            map.serialize_entry("type", "line")?;
        }
        map.serialize_entry("series", &self.series)?;
        map.serialize_entry("axis_config", &self.axis_config)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl LineChartBuilder {
    /// Adds data series in display order. Series names must be unique.
    /// Replaces the collection; order is preserved.
    pub fn series<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<DataSeries>,
    {
        self.series = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn series_entry(mut self, value: impl Into<DataSeries>) -> Self {
        self.series.get_or_insert_with(Vec::new).push(value.into());
        self
    }
    /// Sets the axis categories and labels. Every series needs exactly one point per category.
    pub fn axis_config(mut self, value: impl Into<AxisConfig>) -> Self {
        self.axis_config = Some(value.into());
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
    pub fn build(self) -> Result<LineChart, ValidationError> {
        let series = wire::required(self.series, "LineChart.series")?;
        let axis_config = wire::required(self.axis_config, "LineChart.axis_config")?;
        wire::extensions(&self.extensions, &["series", "axis_config"], "LineChart")?;
        let value = LineChart {
            series,
            axis_config,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "LineChart", e.to_string())
        })?;
        if let Some(v) = wire.get("series") {
            crate::rules::limits(v, "data_visualization.series", "LineChart.series")?;
        }
        crate::rules::validate("LineChart", &wire, "LineChart")?;
        Ok(value)
    }
}
impl Serialize for LineChart {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for LineChart {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "line", path)?;
        let series = wire::field::<Vec<DataSeries>>(&mut map, "series", path, false)?;
        let axis_config = wire::field::<AxisConfig>(&mut map, "axis_config", path, false)?;
        LineChartBuilder {
            series,
            axis_config,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for LineChart {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "LineChart")
    }
}
impl<'de> Deserialize<'de> for LineChart {
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

/// A labelled group of options in a static select menu.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object).

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOptionGroup {
    label: PlainText,
    options: Vec<SelectOption>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`SelectOptionGroup`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct SelectOptionGroupBuilder {
    label: Option<PlainTextInput>,
    options: Option<Vec<SelectOption>>,
    extensions: Map<String, Value>,
}
impl Default for SelectOptionGroupBuilder {
    fn default() -> Self {
        Self {
            label: None,
            options: None,
            extensions: Map::new(),
        }
    }
}
impl SelectOptionGroup {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> SelectOptionGroupBuilder {
        SelectOptionGroupBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> SelectOptionGroupBuilder {
        SelectOptionGroupBuilder {
            label: Some(self.label.into()),
            options: Some(self.options),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `label`. Sets the group heading.
    pub fn label(&self) -> &PlainText {
        &self.label
    }
    /// Borrows or copies `options`. Adds selectable options in display order.
    pub fn options(&self) -> &[SelectOption] {
        &self.options
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
        map.serialize_entry("label", &self.label)?;
        map.serialize_entry("options", &self.options)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SelectOptionGroupBuilder {
    /// Sets the group heading.
    pub fn label(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.label = Some(value.into());
        self
    }
    /// Adds selectable options in display order.
    /// Replaces the collection; order is preserved.
    pub fn options<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectOption>,
    {
        self.options = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn option(mut self, value: impl Into<SelectOption>) -> Self {
        self.options.get_or_insert_with(Vec::new).push(value.into());
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
    pub fn build(self) -> Result<SelectOptionGroup, ValidationError> {
        let label = wire::required(
            self.label
                .map(|v| v.resolve().map_err(|e| e.at("SelectOptionGroup.label")))
                .transpose()?,
            "SelectOptionGroup.label",
        )?;
        let options = wire::required(self.options, "SelectOptionGroup.options")?;
        wire::extensions(&self.extensions, &["label", "options"], "SelectOptionGroup")?;
        let value = SelectOptionGroup {
            label,
            options,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "SelectOptionGroup",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("label") {
            crate::rules::limits(v, "option_group.label", "SelectOptionGroup.label")?;
        }
        if let Some(v) = wire.get("options") {
            crate::rules::limits(v, "option_group.options", "SelectOptionGroup.options")?;
        }
        crate::rules::validate("OptionGroup", &wire, "SelectOptionGroup")?;
        Ok(value)
    }
}
impl Serialize for SelectOptionGroup {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for SelectOptionGroup {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        crate::rules::coerce_text(map.get_mut("label"), "plain_text");
        let label = wire::field::<PlainText>(&mut map, "label", path, false)?;
        let options = wire::field::<Vec<SelectOption>>(&mut map, "options", path, false)?;
        SelectOptionGroupBuilder {
            label: label.map(Into::into),
            options,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for SelectOptionGroup {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "SelectOptionGroup")
    }
}
impl<'de> Deserialize<'de> for SelectOptionGroup {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A pie chart for a data visualization block.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).

#[derive(Clone, Debug, PartialEq)]
pub struct PieChart {
    segments: Vec<ChartSegment>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`PieChart`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct PieChartBuilder {
    segments: Option<Vec<ChartSegment>>,
    extensions: Map<String, Value>,
}
impl Default for PieChartBuilder {
    fn default() -> Self {
        Self {
            segments: None,
            extensions: Map::new(),
        }
    }
}
impl PieChart {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> PieChartBuilder {
        PieChartBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> PieChartBuilder {
        PieChartBuilder {
            segments: Some(self.segments),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `segments`. Adds pie segments in display order.
    pub fn segments(&self) -> &[ChartSegment] {
        &self.segments
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
            map.serialize_entry("type", "pie")?;
        }
        map.serialize_entry("segments", &self.segments)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PieChartBuilder {
    /// Adds pie segments in display order.
    /// Replaces the collection; order is preserved.
    pub fn segments<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ChartSegment>,
    {
        self.segments = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn segment(mut self, value: impl Into<ChartSegment>) -> Self {
        self.segments
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
    pub fn build(self) -> Result<PieChart, ValidationError> {
        let segments = wire::required(self.segments, "PieChart.segments")?;
        wire::extensions(&self.extensions, &["segments"], "PieChart")?;
        let value = PieChart {
            segments,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "PieChart", e.to_string())
        })?;
        if let Some(v) = wire.get("segments") {
            crate::rules::limits(v, "data_visualization.segments", "PieChart.segments")?;
        }
        crate::rules::validate("PieChart", &wire, "PieChart")?;
        Ok(value)
    }
}
impl Serialize for PieChart {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for PieChart {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "pie", path)?;
        let segments = wire::field::<Vec<ChartSegment>>(&mut map, "segments", path, false)?;
        PieChartBuilder {
            segments,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for PieChart {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "PieChart")
    }
}
impl<'de> Deserialize<'de> for PieChart {
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

/// A channel mention inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextChannel {
    channel_id: String,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextChannel`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextChannelBuilder {
    channel_id: Option<String>,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
impl Default for RichTextChannelBuilder {
    fn default() -> Self {
        Self {
            channel_id: None,
            style: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextChannel {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextChannelBuilder {
        RichTextChannelBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextChannelBuilder {
        RichTextChannelBuilder {
            channel_id: Some(self.channel_id),
            style: self.style,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `channel_id`. Sets the ID of the mentioned channel.
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }
    /// Borrows or copies `style`. Sets the mention style.
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
            map.serialize_entry("type", "channel")?;
        }
        map.serialize_entry("channel_id", &self.channel_id)?;
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextChannelBuilder {
    /// Sets the ID of the mentioned channel.
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }
    /// Sets the mention style.
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
    pub fn build(self) -> Result<RichTextChannel, ValidationError> {
        let channel_id = wire::required(self.channel_id, "RichTextChannel.channel_id")?;
        let style = self.style;
        wire::extensions(
            &self.extensions,
            &["channel_id", "style"],
            "RichTextChannel",
        )?;
        let value = RichTextChannel {
            channel_id,
            style,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "RichTextChannel",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("style") {
            crate::rules::style(
                v,
                &[
                    "bold",
                    "italic",
                    "strike",
                    "highlight",
                    "client_highlight",
                    "unlink",
                ],
                "RichTextChannel.style",
            )?;
        }
        crate::rules::validate("RichTextChannel", &wire, "RichTextChannel")?;
        Ok(value)
    }
}
impl Serialize for RichTextChannel {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextChannel {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "channel", path)?;
        let channel_id = wire::field::<String>(&mut map, "channel_id", path, false)?;
        let style = wire::field::<RichTextStyle>(&mut map, "style", path, false)?;
        RichTextChannelBuilder {
            channel_id,
            style,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextChannel {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextChannel")
    }
}
impl<'de> Deserialize<'de> for RichTextChannel {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A preformatted code block inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextCodeBlock {
    elements: Vec<RichTextSectionElement>,
    border: Option<i64>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextCodeBlock`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextCodeBlockBuilder {
    elements: Option<Vec<RichTextSectionElement>>,
    border: Option<i64>,
    extensions: Map<String, Value>,
}
impl Default for RichTextCodeBlockBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            border: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextCodeBlock {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextCodeBlockBuilder {
        RichTextCodeBlockBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextCodeBlockBuilder {
        RichTextCodeBlockBuilder {
            elements: Some(self.elements),
            border: self.border,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds inline rich text elements in display order.
    pub fn elements(&self) -> &[RichTextSectionElement] {
        &self.elements
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
            map.serialize_entry("type", "rich_text_preformatted")?;
        }
        map.serialize_entry("elements", &self.elements)?;
        if let Some(value) = &self.border {
            map.serialize_entry("border", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextCodeBlockBuilder {
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
    pub fn build(self) -> Result<RichTextCodeBlock, ValidationError> {
        let elements = wire::required(self.elements, "RichTextCodeBlock.elements")?;
        let border = self.border;
        wire::extensions(
            &self.extensions,
            &["elements", "border"],
            "RichTextCodeBlock",
        )?;
        let value = RichTextCodeBlock {
            elements,
            border,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "RichTextCodeBlock",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("border") {
            crate::rules::limits(
                v,
                "rich_text_preformatted.border",
                "RichTextCodeBlock.border",
            )?;
        }
        crate::rules::validate("RichTextCodeBlock", &wire, "RichTextCodeBlock")?;
        Ok(value)
    }
}
impl Serialize for RichTextCodeBlock {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextCodeBlock {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "rich_text_preformatted", path)?;
        let elements =
            wire::field::<Vec<RichTextSectionElement>>(&mut map, "elements", path, false)?;
        let border = wire::field::<i64>(&mut map, "border", path, false)?;
        RichTextCodeBlockBuilder {
            elements,
            border,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextCodeBlock {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextCodeBlock")
    }
}
impl<'de> Deserialize<'de> for RichTextCodeBlock {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// An emoji inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextEmoji {
    name: String,
    skin_tone: Option<i64>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextEmoji`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextEmojiBuilder {
    name: Option<String>,
    skin_tone: Option<i64>,
    extensions: Map<String, Value>,
}
impl Default for RichTextEmojiBuilder {
    fn default() -> Self {
        Self {
            name: None,
            skin_tone: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextEmoji {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextEmojiBuilder {
        RichTextEmojiBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextEmojiBuilder {
        RichTextEmojiBuilder {
            name: Some(self.name),
            skin_tone: self.skin_tone,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `name`. Sets the emoji shortcode name, without colons.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Borrows or copies `skin_tone`. Sets the emoji skin tone, from 1 to 6.
    pub fn skin_tone(&self) -> Option<i64> {
        self.skin_tone
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
            map.serialize_entry("type", "emoji")?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(value) = &self.skin_tone {
            map.serialize_entry("skin_tone", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextEmojiBuilder {
    /// Sets the emoji shortcode name, without colons.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets the emoji skin tone, from 1 to 6.
    pub fn skin_tone(mut self, value: i64) -> Self {
        self.skin_tone = Some(value);
        self
    }
    /// Omits `skin_tone`, including any model default.
    pub fn clear_skin_tone(mut self) -> Self {
        self.skin_tone = None;
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
    pub fn build(self) -> Result<RichTextEmoji, ValidationError> {
        let name = wire::required(self.name, "RichTextEmoji.name")?;
        let skin_tone = self.skin_tone;
        wire::extensions(&self.extensions, &["name", "skin_tone"], "RichTextEmoji")?;
        let value = RichTextEmoji {
            name,
            skin_tone,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextEmoji", e.to_string())
        })?;
        crate::rules::validate("RichTextEmoji", &wire, "RichTextEmoji")?;
        Ok(value)
    }
}
impl Serialize for RichTextEmoji {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextEmoji {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "emoji", path)?;
        let name = wire::field::<String>(&mut map, "name", path, false)?;
        let skin_tone = wire::field::<i64>(&mut map, "skin_tone", path, false)?;
        RichTextEmojiBuilder {
            name,
            skin_tone,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextEmoji {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextEmoji")
    }
}
impl<'de> Deserialize<'de> for RichTextEmoji {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A hyperlink inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextLink {
    url: String,
    text: Option<String>,
    style: Option<RichTextStyle>,
    r#unsafe: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextLink`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextLinkBuilder {
    url: Option<String>,
    text: Option<String>,
    style: Option<RichTextStyle>,
    r#unsafe: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for RichTextLinkBuilder {
    fn default() -> Self {
        Self {
            url: None,
            text: None,
            style: None,
            r#unsafe: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextLink {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextLinkBuilder {
        RichTextLinkBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextLinkBuilder {
        RichTextLinkBuilder {
            url: Some(self.url),
            text: self.text,
            style: self.style,
            r#unsafe: self.r#unsafe,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `url`. Sets the link target.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Borrows or copies `text`. Sets the visible link text. Slack shows the URL when omitted.
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
    /// Borrows or copies `style`. Sets the visual style.
    pub fn style(&self) -> Option<&RichTextStyle> {
        self.style.as_ref()
    }
    /// Borrows or copies `unsafe`. Sets whether Slack treats the link as unsafe.
    pub fn r#unsafe(&self) -> Option<bool> {
        self.r#unsafe
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
            map.serialize_entry("type", "link")?;
        }
        map.serialize_entry("url", &self.url)?;
        if let Some(value) = &self.text {
            map.serialize_entry("text", value)?;
        }
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        if let Some(value) = &self.r#unsafe {
            map.serialize_entry("unsafe", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextLinkBuilder {
    /// Sets the link target.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Sets the visible link text. Slack shows the URL when omitted.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
    /// Omits `text`, including any model default.
    pub fn clear_text(mut self) -> Self {
        self.text = None;
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
    /// Sets whether Slack treats the link as unsafe.
    pub fn r#unsafe(mut self, value: bool) -> Self {
        self.r#unsafe = Some(value);
        self
    }
    /// Omits `unsafe`, including any model default.
    pub fn clear_unsafe(mut self) -> Self {
        self.r#unsafe = None;
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
    pub fn build(self) -> Result<RichTextLink, ValidationError> {
        let url = wire::required(self.url, "RichTextLink.url")?;
        let text = self.text;
        let style = self.style;
        let r#unsafe = self.r#unsafe;
        wire::extensions(
            &self.extensions,
            &["url", "text", "style", "unsafe"],
            "RichTextLink",
        )?;
        let value = RichTextLink {
            url,
            text,
            style,
            r#unsafe,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextLink", e.to_string())
        })?;
        if let Some(v) = wire.get("style") {
            crate::rules::style(
                v,
                &["bold", "italic", "strike", "code"],
                "RichTextLink.style",
            )?;
        }
        crate::rules::validate("RichTextLink", &wire, "RichTextLink")?;
        Ok(value)
    }
}
impl Serialize for RichTextLink {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextLink {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "link", path)?;
        let url = wire::field::<String>(&mut map, "url", path, false)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let style = wire::field::<RichTextStyle>(&mut map, "style", path, false)?;
        let r#unsafe = wire::field::<bool>(&mut map, "unsafe", path, false)?;
        RichTextLinkBuilder {
            url,
            text,
            style,
            r#unsafe,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextLink {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextLink")
    }
}
impl<'de> Deserialize<'de> for RichTextLink {
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

/// A quotation inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextQuote {
    elements: Vec<RichTextSectionElement>,
    border: Option<i64>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextQuote`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextQuoteBuilder {
    elements: Option<Vec<RichTextSectionElement>>,
    border: Option<i64>,
    extensions: Map<String, Value>,
}
impl Default for RichTextQuoteBuilder {
    fn default() -> Self {
        Self {
            elements: None,
            border: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextQuote {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextQuoteBuilder {
        RichTextQuoteBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextQuoteBuilder {
        RichTextQuoteBuilder {
            elements: Some(self.elements),
            border: self.border,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `elements`. Adds inline rich text elements in display order.
    pub fn elements(&self) -> &[RichTextSectionElement] {
        &self.elements
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
            map.serialize_entry("type", "rich_text_quote")?;
        }
        map.serialize_entry("elements", &self.elements)?;
        if let Some(value) = &self.border {
            map.serialize_entry("border", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextQuoteBuilder {
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
    pub fn build(self) -> Result<RichTextQuote, ValidationError> {
        let elements = wire::required(self.elements, "RichTextQuote.elements")?;
        let border = self.border;
        wire::extensions(&self.extensions, &["elements", "border"], "RichTextQuote")?;
        let value = RichTextQuote {
            elements,
            border,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextQuote", e.to_string())
        })?;
        if let Some(v) = wire.get("border") {
            crate::rules::limits(v, "rich_text_quote.border", "RichTextQuote.border")?;
        }
        crate::rules::validate("RichTextQuote", &wire, "RichTextQuote")?;
        Ok(value)
    }
}
impl Serialize for RichTextQuote {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextQuote {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "rich_text_quote", path)?;
        let elements =
            wire::field::<Vec<RichTextSectionElement>>(&mut map, "elements", path, false)?;
        let border = wire::field::<i64>(&mut map, "border", path, false)?;
        RichTextQuoteBuilder {
            elements,
            border,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextQuote {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextQuote")
    }
}
impl<'de> Deserialize<'de> for RichTextQuote {
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

/// A user mention inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextUser {
    user_id: String,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextUser`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextUserBuilder {
    user_id: Option<String>,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
impl Default for RichTextUserBuilder {
    fn default() -> Self {
        Self {
            user_id: None,
            style: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextUser {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextUserBuilder {
        RichTextUserBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextUserBuilder {
        RichTextUserBuilder {
            user_id: Some(self.user_id),
            style: self.style,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `user_id`. Sets the ID of the mentioned user.
    pub fn user_id(&self) -> &str {
        &self.user_id
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
            map.serialize_entry("type", "user")?;
        }
        map.serialize_entry("user_id", &self.user_id)?;
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextUserBuilder {
    /// Sets the ID of the mentioned user.
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
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
    pub fn build(self) -> Result<RichTextUser, ValidationError> {
        let user_id = wire::required(self.user_id, "RichTextUser.user_id")?;
        let style = self.style;
        wire::extensions(&self.extensions, &["user_id", "style"], "RichTextUser")?;
        let value = RichTextUser {
            user_id,
            style,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "RichTextUser", e.to_string())
        })?;
        if let Some(v) = wire.get("style") {
            crate::rules::style(
                v,
                &[
                    "bold",
                    "italic",
                    "strike",
                    "highlight",
                    "client_highlight",
                    "unlink",
                ],
                "RichTextUser.style",
            )?;
        }
        crate::rules::validate("RichTextUser", &wire, "RichTextUser")?;
        Ok(value)
    }
}
impl Serialize for RichTextUser {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextUser {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "user", path)?;
        let user_id = wire::field::<String>(&mut map, "user_id", path, false)?;
        let style = wire::field::<RichTextStyle>(&mut map, "style", path, false)?;
        RichTextUserBuilder {
            user_id,
            style,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextUser {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextUser")
    }
}
impl<'de> Deserialize<'de> for RichTextUser {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A user group mention inside rich text.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block).

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextUserGroup {
    usergroup_id: String,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`RichTextUserGroup`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct RichTextUserGroupBuilder {
    usergroup_id: Option<String>,
    style: Option<RichTextStyle>,
    extensions: Map<String, Value>,
}
impl Default for RichTextUserGroupBuilder {
    fn default() -> Self {
        Self {
            usergroup_id: None,
            style: None,
            extensions: Map::new(),
        }
    }
}
impl RichTextUserGroup {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> RichTextUserGroupBuilder {
        RichTextUserGroupBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> RichTextUserGroupBuilder {
        RichTextUserGroupBuilder {
            usergroup_id: Some(self.usergroup_id),
            style: self.style,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `usergroup_id`. Sets the ID of the mentioned user group.
    pub fn usergroup_id(&self) -> &str {
        &self.usergroup_id
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
            map.serialize_entry("type", "usergroup")?;
        }
        map.serialize_entry("usergroup_id", &self.usergroup_id)?;
        if let Some(value) = &self.style {
            map.serialize_entry("style", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RichTextUserGroupBuilder {
    /// Sets the ID of the mentioned user group.
    pub fn usergroup_id(mut self, value: impl Into<String>) -> Self {
        self.usergroup_id = Some(value.into());
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
    pub fn build(self) -> Result<RichTextUserGroup, ValidationError> {
        let usergroup_id = wire::required(self.usergroup_id, "RichTextUserGroup.usergroup_id")?;
        let style = self.style;
        wire::extensions(
            &self.extensions,
            &["usergroup_id", "style"],
            "RichTextUserGroup",
        )?;
        let value = RichTextUserGroup {
            usergroup_id,
            style,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "RichTextUserGroup",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("style") {
            crate::rules::style(
                v,
                &[
                    "bold",
                    "italic",
                    "strike",
                    "highlight",
                    "client_highlight",
                    "unlink",
                ],
                "RichTextUserGroup.style",
            )?;
        }
        crate::rules::validate("RichTextUserGroup", &wire, "RichTextUserGroup")?;
        Ok(value)
    }
}
impl Serialize for RichTextUserGroup {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for RichTextUserGroup {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "usergroup", path)?;
        let usergroup_id = wire::field::<String>(&mut map, "usergroup_id", path, false)?;
        let style = wire::field::<RichTextStyle>(&mut map, "style", path, false)?;
        RichTextUserGroupBuilder {
            usergroup_id,
            style,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for RichTextUserGroup {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "RichTextUserGroup")
    }
}
impl<'de> Deserialize<'de> for RichTextUserGroup {
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

/// A Slack-provided icon, referenced by name.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/blocks/card-block).

#[derive(Clone, Debug, PartialEq)]
pub struct SlackIcon {
    name: String,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`SlackIcon`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct SlackIconBuilder {
    name: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for SlackIconBuilder {
    fn default() -> Self {
        Self {
            name: None,
            extensions: Map::new(),
        }
    }
}
impl SlackIcon {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> SlackIconBuilder {
        SlackIconBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> SlackIconBuilder {
        SlackIconBuilder {
            name: Some(self.name),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `name`. Sets the name of the Slack-provided icon, such as rocket.
    pub fn name(&self) -> &str {
        &self.name
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
            map.serialize_entry("type", "icon")?;
        }
        map.serialize_entry("name", &self.name)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SlackIconBuilder {
    /// Sets the name of the Slack-provided icon, such as rocket.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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
    pub fn build(self) -> Result<SlackIcon, ValidationError> {
        let name = wire::required(self.name, "SlackIcon.name")?;
        wire::extensions(&self.extensions, &["name"], "SlackIcon")?;
        let value = SlackIcon {
            name,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "SlackIcon", e.to_string())
        })?;
        crate::rules::validate("SlackIcon", &wire, "SlackIcon")?;
        Ok(value)
    }
}
impl Serialize for SlackIcon {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for SlackIcon {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "icon", path)?;
        let name = wire::field::<String>(&mut map, "name", path, false)?;
        SlackIconBuilder {
            name,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for SlackIcon {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "SlackIcon")
    }
}
impl<'de> Deserialize<'de> for SlackIcon {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A link trigger that starts a workflow.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object).

#[derive(Clone, Debug, PartialEq)]
pub struct Trigger {
    url: String,
    customizable_input_parameters: Option<Vec<InputParameter>>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`Trigger`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct TriggerBuilder {
    url: Option<String>,
    customizable_input_parameters: Option<Vec<InputParameter>>,
    extensions: Map<String, Value>,
}
impl Default for TriggerBuilder {
    fn default() -> Self {
        Self {
            url: None,
            customizable_input_parameters: None,
            extensions: Map::new(),
        }
    }
}
impl Trigger {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> TriggerBuilder {
        TriggerBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> TriggerBuilder {
        TriggerBuilder {
            url: Some(self.url),
            customizable_input_parameters: self.customizable_input_parameters,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `url`. Sets the link trigger URL.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Borrows or copies `customizable_input_parameters`. Adds the input values passed to the workflow's trigger.
    pub fn customizable_input_parameters(&self) -> Option<&[InputParameter]> {
        self.customizable_input_parameters.as_deref()
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
        map.serialize_entry("url", &self.url)?;
        if let Some(value) = &self.customizable_input_parameters {
            map.serialize_entry("customizable_input_parameters", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TriggerBuilder {
    /// Sets the link trigger URL.
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    /// Adds the input values passed to the workflow's trigger.
    /// Replaces the collection; order is preserved.
    pub fn customizable_input_parameters<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<InputParameter>,
    {
        self.customizable_input_parameters = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Appends one item to the collection.
    pub fn customizable_input_parameter(mut self, value: impl Into<InputParameter>) -> Self {
        self.customizable_input_parameters
            .get_or_insert_with(Vec::new)
            .push(value.into());
        self
    }
    /// Omits `customizable_input_parameters`, including any model default.
    pub fn clear_customizable_input_parameters(mut self) -> Self {
        self.customizable_input_parameters = None;
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
    pub fn build(self) -> Result<Trigger, ValidationError> {
        let url = wire::required(self.url, "Trigger.url")?;
        let customizable_input_parameters = self.customizable_input_parameters;
        wire::extensions(
            &self.extensions,
            &["url", "customizable_input_parameters"],
            "Trigger",
        )?;
        let value = Trigger {
            url,
            customizable_input_parameters,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "Trigger", e.to_string())
        })?;
        crate::rules::validate("Trigger", &wire, "Trigger")?;
        Ok(value)
    }
}
impl Serialize for Trigger {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for Trigger {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let url = wire::field::<String>(&mut map, "url", path, false)?;
        let customizable_input_parameters = wire::field::<Vec<InputParameter>>(
            &mut map,
            "customizable_input_parameters",
            path,
            false,
        )?;
        TriggerBuilder {
            url,
            customizable_input_parameters,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for Trigger {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Trigger")
    }
}
impl<'de> Deserialize<'de> for Trigger {
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

/// The workflow started by a workflow button.
///
/// [Slack reference](https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object).

#[derive(Clone, Debug, PartialEq)]
pub struct Workflow {
    trigger: Trigger,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`Workflow`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct WorkflowBuilder {
    trigger: Option<Trigger>,
    extensions: Map<String, Value>,
}
impl Default for WorkflowBuilder {
    fn default() -> Self {
        Self {
            trigger: None,
            extensions: Map::new(),
        }
    }
}
impl Workflow {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> WorkflowBuilder {
        WorkflowBuilder {
            trigger: Some(self.trigger),
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `trigger`. Sets the link trigger that starts the workflow.
    pub fn trigger(&self) -> &Trigger {
        &self.trigger
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
        map.serialize_entry("trigger", &self.trigger)?;
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl WorkflowBuilder {
    /// Sets the link trigger that starts the workflow.
    pub fn trigger(mut self, value: impl Into<Trigger>) -> Self {
        self.trigger = Some(value.into());
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
    pub fn build(self) -> Result<Workflow, ValidationError> {
        let trigger = wire::required(self.trigger, "Workflow.trigger")?;
        wire::extensions(&self.extensions, &["trigger"], "Workflow")?;
        let value = Workflow {
            trigger,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "Workflow", e.to_string())
        })?;
        crate::rules::validate("Workflow", &wire, "Workflow")?;
        Ok(value)
    }
}
impl Serialize for Workflow {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for Workflow {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let trigger = wire::field::<Trigger>(&mut map, "trigger", path, false)?;
        WorkflowBuilder {
            trigger,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for Workflow {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "Workflow")
    }
}
impl<'de> Deserialize<'de> for Workflow {
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

/// The payload for publishing an App Home tab with views.publish.
///
/// [Slack reference](https://docs.slack.dev/surfaces/app-home).
/// Only blocks supported on the App Home tab are accepted.
#[derive(Clone, Debug, PartialEq)]
pub struct HomeTabView {
    blocks: Vec<Block>,
    private_metadata: Option<String>,
    callback_id: Option<String>,
    external_id: Option<String>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`HomeTabView`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct HomeTabViewBuilder {
    blocks: Option<Vec<Block>>,
    private_metadata: Option<String>,
    callback_id: Option<String>,
    external_id: Option<String>,
    extensions: Map<String, Value>,
}
impl Default for HomeTabViewBuilder {
    fn default() -> Self {
        Self {
            blocks: None,
            private_metadata: None,
            callback_id: None,
            external_id: None,
            extensions: Map::new(),
        }
    }
}
impl HomeTabView {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> HomeTabViewBuilder {
        HomeTabViewBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> HomeTabViewBuilder {
        HomeTabViewBuilder {
            blocks: Some(self.blocks),
            private_metadata: self.private_metadata,
            callback_id: self.callback_id,
            external_id: self.external_id,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `blocks`. Adds the blocks shown on the App Home tab, in display order.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    /// Borrows or copies `private_metadata`. Sets application-defined data returned in view payloads.
    pub fn private_metadata(&self) -> Option<&str> {
        self.private_metadata.as_deref()
    }
    /// Borrows or copies `callback_id`. Sets an identifier returned in view payloads so your app can recognize the view.
    pub fn callback_id(&self) -> Option<&str> {
        self.callback_id.as_deref()
    }
    /// Borrows or copies `external_id`. Sets a workspace-unique identifier you can use to update the view later.
    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
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
            map.serialize_entry("type", "home")?;
        }
        map.serialize_entry("blocks", &self.blocks)?;
        if let Some(value) = &self.private_metadata {
            map.serialize_entry("private_metadata", value)?;
        }
        if let Some(value) = &self.callback_id {
            map.serialize_entry("callback_id", value)?;
        }
        if let Some(value) = &self.external_id {
            map.serialize_entry("external_id", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl HomeTabViewBuilder {
    /// Adds the blocks shown on the App Home tab, in display order.
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
    /// Sets application-defined data returned in view payloads.
    pub fn private_metadata(mut self, value: impl Into<String>) -> Self {
        self.private_metadata = Some(value.into());
        self
    }
    /// Omits `private_metadata`, including any model default.
    pub fn clear_private_metadata(mut self) -> Self {
        self.private_metadata = None;
        self
    }
    /// Sets an identifier returned in view payloads so your app can recognize the view.
    pub fn callback_id(mut self, value: impl Into<String>) -> Self {
        self.callback_id = Some(value.into());
        self
    }
    /// Omits `callback_id`, including any model default.
    pub fn clear_callback_id(mut self) -> Self {
        self.callback_id = None;
        self
    }
    /// Sets a workspace-unique identifier you can use to update the view later.
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }
    /// Omits `external_id`, including any model default.
    pub fn clear_external_id(mut self) -> Self {
        self.external_id = None;
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
    pub fn build(self) -> Result<HomeTabView, ValidationError> {
        let blocks = wire::required(self.blocks, "HomeTabView.blocks")?;
        let private_metadata = self.private_metadata;
        let callback_id = self.callback_id;
        let external_id = self.external_id;
        wire::extensions(
            &self.extensions,
            &["blocks", "private_metadata", "callback_id", "external_id"],
            "HomeTabView",
        )?;
        let value = HomeTabView {
            blocks,
            private_metadata,
            callback_id,
            external_id,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "HomeTabView", e.to_string())
        })?;
        if let Some(v) = wire.get("blocks") {
            crate::rules::limits(v, "view.blocks", "HomeTabView.blocks")?;
        }
        if let Some(v) = wire.get("private_metadata") {
            crate::rules::limits(v, "view.private_metadata", "HomeTabView.private_metadata")?;
        }
        if let Some(v) = wire.get("callback_id") {
            crate::rules::limits(v, "view.callback_id", "HomeTabView.callback_id")?;
        }
        if let Some(v) = wire.get("external_id") {
            crate::rules::limits(v, "view.external_id", "HomeTabView.external_id")?;
        }
        crate::rules::validate("HomeTabView", &wire, "HomeTabView")?;
        Ok(value)
    }
}
impl Serialize for HomeTabView {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for HomeTabView {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "home", path)?;
        let blocks = wire::field::<Vec<Block>>(&mut map, "blocks", path, false)?;
        let private_metadata = wire::field::<String>(&mut map, "private_metadata", path, false)?;
        let callback_id = wire::field::<String>(&mut map, "callback_id", path, false)?;
        let external_id = wire::field::<String>(&mut map, "external_id", path, false)?;
        HomeTabViewBuilder {
            blocks,
            private_metadata,
            callback_id,
            external_id,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for HomeTabView {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "HomeTabView")
    }
}
impl<'de> Deserialize<'de> for HomeTabView {
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

/// A response body for slash commands and interaction response URLs.
///
/// [Slack reference](https://docs.slack.dev/interactivity/handling-user-interaction#message_responses).
/// Only blocks supported in messages are accepted. Markdown block text is limited to 12,000 characters, and data table cell text to 20,000 characters, across the whole message.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageResponse {
    blocks: Option<Vec<Block>>,
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
    mrkdwn: Option<bool>,
    replace_original: Option<bool>,
    response_type: Option<ResponseType>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`MessageResponse`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct MessageResponseBuilder {
    blocks: Option<Vec<Block>>,
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
    mrkdwn: Option<bool>,
    replace_original: Option<bool>,
    response_type: Option<ResponseType>,
    extensions: Map<String, Value>,
}
impl Default for MessageResponseBuilder {
    fn default() -> Self {
        Self {
            blocks: None,
            attachments: None,
            text: Some("".into()),
            mrkdwn: Some(true),
            replace_original: Some(false),
            response_type: Some(ResponseType::InChannel),
            extensions: Map::new(),
        }
    }
}
impl MessageResponse {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> MessageResponseBuilder {
        MessageResponseBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> MessageResponseBuilder {
        MessageResponseBuilder {
            blocks: self.blocks,
            attachments: self.attachments,
            text: self.text,
            mrkdwn: self.mrkdwn,
            replace_original: self.replace_original,
            response_type: self.response_type,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `blocks`. Adds the response blocks in display order.
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
    /// Borrows or copies `replace_original`. Sets whether the response replaces the message that triggered it.
    pub fn replace_original(&self) -> Option<bool> {
        self.replace_original
    }
    /// Borrows or copies `response_type`. Sets who can see the response.
    pub fn response_type(&self) -> Option<ResponseType> {
        self.response_type
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
        if let Some(value) = &self.replace_original {
            map.serialize_entry("replace_original", value)?;
        }
        if let Some(value) = &self.response_type {
            map.serialize_entry("response_type", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl MessageResponseBuilder {
    /// Adds the response blocks in display order.
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
    /// Sets whether the response replaces the message that triggered it.
    pub fn replace_original(mut self, value: bool) -> Self {
        self.replace_original = Some(value);
        self
    }
    /// Omits `replace_original`, including any model default.
    pub fn clear_replace_original(mut self) -> Self {
        self.replace_original = None;
        self
    }
    /// Sets who can see the response.
    pub fn response_type(mut self, value: ResponseType) -> Self {
        self.response_type = Some(value);
        self
    }
    /// Omits `response_type`, including any model default.
    pub fn clear_response_type(mut self) -> Self {
        self.response_type = None;
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
    pub fn build(self) -> Result<MessageResponse, ValidationError> {
        let blocks = self.blocks;
        let attachments = self.attachments;
        let text = self.text;
        let mrkdwn = self.mrkdwn;
        let replace_original = self.replace_original;
        let response_type = self.response_type;
        wire::extensions(
            &self.extensions,
            &[
                "blocks",
                "attachments",
                "text",
                "mrkdwn",
                "replace_original",
                "response_type",
            ],
            "MessageResponse",
        )?;
        let value = MessageResponse {
            blocks,
            attachments,
            text,
            mrkdwn,
            replace_original,
            response_type,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(
                ErrorCategory::TypeMismatch,
                "MessageResponse",
                e.to_string(),
            )
        })?;
        if let Some(v) = wire.get("blocks") {
            crate::rules::limits(v, "message.blocks", "MessageResponse.blocks")?;
        }
        if let Some(v) = wire.get("attachments") {
            crate::rules::limits(v, "message.attachments", "MessageResponse.attachments")?;
        }
        crate::rules::validate("MessageResponse", &wire, "MessageResponse")?;
        Ok(value)
    }
}
impl Serialize for MessageResponse {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for MessageResponse {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let blocks = wire::field::<Vec<Block>>(&mut map, "blocks", path, false)?;
        let attachments = wire::field::<Vec<Attachment>>(&mut map, "attachments", path, false)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let mrkdwn = wire::field::<bool>(&mut map, "mrkdwn", path, false)?;
        let replace_original = wire::field::<bool>(&mut map, "replace_original", path, false)?;
        let response_type = wire::field::<ResponseType>(&mut map, "response_type", path, false)?;
        MessageResponseBuilder {
            blocks,
            attachments,
            text,
            mrkdwn,
            replace_original,
            response_type,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for MessageResponse {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "MessageResponse")
    }
}
impl<'de> Deserialize<'de> for MessageResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// The view payload for views.open, views.push, and views.update.
///
/// [Slack reference](https://docs.slack.dev/surfaces/modals).
/// Only blocks supported in modals are accepted. A modal that contains an input block must have a submit label.
#[derive(Clone, Debug, PartialEq)]
pub struct ModalView {
    title: PlainText,
    blocks: Vec<Block>,
    close: Option<PlainText>,
    submit: Option<PlainText>,
    private_metadata: Option<String>,
    callback_id: Option<String>,
    clear_on_close: Option<bool>,
    notify_on_close: Option<bool>,
    external_id: Option<String>,
    submit_disabled: Option<bool>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`ModalView`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct ModalViewBuilder {
    title: Option<PlainTextInput>,
    blocks: Option<Vec<Block>>,
    close: Option<PlainTextInput>,
    submit: Option<PlainTextInput>,
    private_metadata: Option<String>,
    callback_id: Option<String>,
    clear_on_close: Option<bool>,
    notify_on_close: Option<bool>,
    external_id: Option<String>,
    submit_disabled: Option<bool>,
    extensions: Map<String, Value>,
}
impl Default for ModalViewBuilder {
    fn default() -> Self {
        Self {
            title: None,
            blocks: None,
            close: None,
            submit: None,
            private_metadata: None,
            callback_id: None,
            clear_on_close: None,
            notify_on_close: None,
            external_id: None,
            submit_disabled: None,
            extensions: Map::new(),
        }
    }
}
impl ModalView {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> ModalViewBuilder {
        ModalViewBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> ModalViewBuilder {
        ModalViewBuilder {
            title: Some(self.title.into()),
            blocks: Some(self.blocks),
            close: self.close.map(Into::into),
            submit: self.submit.map(Into::into),
            private_metadata: self.private_metadata,
            callback_id: self.callback_id,
            clear_on_close: self.clear_on_close,
            notify_on_close: self.notify_on_close,
            external_id: self.external_id,
            submit_disabled: self.submit_disabled,
            extensions: self.extensions,
        }
    }
    /// Borrows or copies `title`. Sets the title in the modal's top bar.
    pub fn title(&self) -> &PlainText {
        &self.title
    }
    /// Borrows or copies `blocks`. Adds the blocks shown in the modal, in display order.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    /// Borrows or copies `close`. Sets the label of the button that closes the modal.
    pub fn close(&self) -> Option<&PlainText> {
        self.close.as_ref()
    }
    /// Borrows or copies `submit`. Sets the label of the button that submits the modal.
    pub fn submit(&self) -> Option<&PlainText> {
        self.submit.as_ref()
    }
    /// Borrows or copies `private_metadata`. Sets application-defined data returned in view payloads.
    pub fn private_metadata(&self) -> Option<&str> {
        self.private_metadata.as_deref()
    }
    /// Borrows or copies `callback_id`. Sets an identifier returned in view payloads so your app can recognize the view.
    pub fn callback_id(&self) -> Option<&str> {
        self.callback_id.as_deref()
    }
    /// Borrows or copies `clear_on_close`. Sets whether closing this modal closes every view in its stack.
    pub fn clear_on_close(&self) -> Option<bool> {
        self.clear_on_close
    }
    /// Borrows or copies `notify_on_close`. Sets whether Slack sends a view_closed event when the user closes the modal.
    pub fn notify_on_close(&self) -> Option<bool> {
        self.notify_on_close
    }
    /// Borrows or copies `external_id`. Sets a workspace-unique identifier you can use to update the view later.
    pub fn external_id(&self) -> Option<&str> {
        self.external_id.as_deref()
    }
    /// Borrows or copies `submit_disabled`. Sets whether the submit button starts disabled. Only valid in workflow configuration modals.
    pub fn submit_disabled(&self) -> Option<bool> {
        self.submit_disabled
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
            map.serialize_entry("type", "modal")?;
        }
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("blocks", &self.blocks)?;
        if let Some(value) = &self.close {
            map.serialize_entry("close", value)?;
        }
        if let Some(value) = &self.submit {
            map.serialize_entry("submit", value)?;
        }
        if let Some(value) = &self.private_metadata {
            map.serialize_entry("private_metadata", value)?;
        }
        if let Some(value) = &self.callback_id {
            map.serialize_entry("callback_id", value)?;
        }
        if let Some(value) = &self.clear_on_close {
            map.serialize_entry("clear_on_close", value)?;
        }
        if let Some(value) = &self.notify_on_close {
            map.serialize_entry("notify_on_close", value)?;
        }
        if let Some(value) = &self.external_id {
            map.serialize_entry("external_id", value)?;
        }
        if let Some(value) = &self.submit_disabled {
            map.serialize_entry("submit_disabled", value)?;
        }
        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ModalViewBuilder {
    /// Sets the title in the modal's top bar.
    pub fn title(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Adds the blocks shown in the modal, in display order.
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
    /// Sets the label of the button that closes the modal.
    pub fn close(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.close = Some(value.into());
        self
    }
    /// Omits `close`, including any model default.
    pub fn clear_close(mut self) -> Self {
        self.close = None;
        self
    }
    /// Sets the label of the button that submits the modal.
    pub fn submit(mut self, value: impl Into<PlainTextInput>) -> Self {
        self.submit = Some(value.into());
        self
    }
    /// Omits `submit`, including any model default.
    pub fn clear_submit(mut self) -> Self {
        self.submit = None;
        self
    }
    /// Sets application-defined data returned in view payloads.
    pub fn private_metadata(mut self, value: impl Into<String>) -> Self {
        self.private_metadata = Some(value.into());
        self
    }
    /// Omits `private_metadata`, including any model default.
    pub fn clear_private_metadata(mut self) -> Self {
        self.private_metadata = None;
        self
    }
    /// Sets an identifier returned in view payloads so your app can recognize the view.
    pub fn callback_id(mut self, value: impl Into<String>) -> Self {
        self.callback_id = Some(value.into());
        self
    }
    /// Omits `callback_id`, including any model default.
    pub fn clear_callback_id(mut self) -> Self {
        self.callback_id = None;
        self
    }
    /// Sets whether closing this modal closes every view in its stack.
    pub fn clear_on_close(mut self, value: bool) -> Self {
        self.clear_on_close = Some(value);
        self
    }
    /// Omits `clear_on_close`, including any model default.
    pub fn clear_clear_on_close(mut self) -> Self {
        self.clear_on_close = None;
        self
    }
    /// Sets whether Slack sends a view_closed event when the user closes the modal.
    pub fn notify_on_close(mut self, value: bool) -> Self {
        self.notify_on_close = Some(value);
        self
    }
    /// Omits `notify_on_close`, including any model default.
    pub fn clear_notify_on_close(mut self) -> Self {
        self.notify_on_close = None;
        self
    }
    /// Sets a workspace-unique identifier you can use to update the view later.
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }
    /// Omits `external_id`, including any model default.
    pub fn clear_external_id(mut self) -> Self {
        self.external_id = None;
        self
    }
    /// Sets whether the submit button starts disabled. Only valid in workflow configuration modals.
    pub fn submit_disabled(mut self, value: bool) -> Self {
        self.submit_disabled = Some(value);
        self
    }
    /// Omits `submit_disabled`, including any model default.
    pub fn clear_submit_disabled(mut self) -> Self {
        self.submit_disabled = None;
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
    pub fn build(self) -> Result<ModalView, ValidationError> {
        let title = wire::required(
            self.title
                .map(|v| v.resolve().map_err(|e| e.at("ModalView.title")))
                .transpose()?,
            "ModalView.title",
        )?;
        let blocks = wire::required(self.blocks, "ModalView.blocks")?;
        let close = self
            .close
            .map(|v| v.resolve().map_err(|e| e.at("ModalView.close")))
            .transpose()?;
        let submit = self
            .submit
            .map(|v| v.resolve().map_err(|e| e.at("ModalView.submit")))
            .transpose()?;
        let private_metadata = self.private_metadata;
        let callback_id = self.callback_id;
        let clear_on_close = self.clear_on_close;
        let notify_on_close = self.notify_on_close;
        let external_id = self.external_id;
        let submit_disabled = self.submit_disabled;
        wire::extensions(
            &self.extensions,
            &[
                "title",
                "blocks",
                "close",
                "submit",
                "private_metadata",
                "callback_id",
                "clear_on_close",
                "notify_on_close",
                "external_id",
                "submit_disabled",
            ],
            "ModalView",
        )?;
        let value = ModalView {
            title,
            blocks,
            close,
            submit,
            private_metadata,
            callback_id,
            clear_on_close,
            notify_on_close,
            external_id,
            submit_disabled,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "ModalView", e.to_string())
        })?;
        if let Some(v) = wire.get("title") {
            crate::rules::limits(v, "view.title", "ModalView.title")?;
        }
        if let Some(v) = wire.get("blocks") {
            crate::rules::limits(v, "view.blocks", "ModalView.blocks")?;
        }
        if let Some(v) = wire.get("close") {
            crate::rules::limits(v, "view.close", "ModalView.close")?;
        }
        if let Some(v) = wire.get("submit") {
            crate::rules::limits(v, "view.submit", "ModalView.submit")?;
        }
        if let Some(v) = wire.get("private_metadata") {
            crate::rules::limits(v, "view.private_metadata", "ModalView.private_metadata")?;
        }
        if let Some(v) = wire.get("callback_id") {
            crate::rules::limits(v, "view.callback_id", "ModalView.callback_id")?;
        }
        if let Some(v) = wire.get("external_id") {
            crate::rules::limits(v, "view.external_id", "ModalView.external_id")?;
        }
        crate::rules::validate("ModalView", &wire, "ModalView")?;
        Ok(value)
    }
}
impl Serialize for ModalView {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for ModalView {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "modal", path)?;
        crate::rules::coerce_text(map.get_mut("title"), "plain_text");
        let title = wire::field::<PlainText>(&mut map, "title", path, false)?;
        let blocks = wire::field::<Vec<Block>>(&mut map, "blocks", path, false)?;
        crate::rules::coerce_text(map.get_mut("close"), "plain_text");
        let close = wire::field::<PlainText>(&mut map, "close", path, false)?;
        crate::rules::coerce_text(map.get_mut("submit"), "plain_text");
        let submit = wire::field::<PlainText>(&mut map, "submit", path, false)?;
        let private_metadata = wire::field::<String>(&mut map, "private_metadata", path, false)?;
        let callback_id = wire::field::<String>(&mut map, "callback_id", path, false)?;
        let clear_on_close = wire::field::<bool>(&mut map, "clear_on_close", path, false)?;
        let notify_on_close = wire::field::<bool>(&mut map, "notify_on_close", path, false)?;
        let external_id = wire::field::<String>(&mut map, "external_id", path, false)?;
        let submit_disabled = wire::field::<bool>(&mut map, "submit_disabled", path, false)?;
        ModalViewBuilder {
            title: title.map(Into::into),
            blocks,
            close: close.map(Into::into),
            submit: submit.map(Into::into),
            private_metadata,
            callback_id,
            clear_on_close,
            notify_on_close,
            external_id,
            submit_disabled,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for ModalView {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "ModalView")
    }
}
impl<'de> Deserialize<'de> for ModalView {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// A payload for incoming webhooks.
///
/// [Slack reference](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks).
/// Only blocks supported in messages are accepted. Markdown block text is limited to 12,000 characters, and data table cell text to 20,000 characters, across the whole message.
#[derive(Clone, Debug, PartialEq)]
pub struct WebhookMessage {
    blocks: Option<Vec<Block>>,
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
    response_type: Option<ResponseType>,
    replace_original: Option<bool>,
    delete_original: Option<bool>,
    unfurl_links: Option<bool>,
    unfurl_media: Option<bool>,
    metadata: Option<Map<String, Value>>,
    extensions: Map<String, Value>,
}
/// Consuming builder for [`WebhookMessage`]. Validation runs in [`Self::build`].
#[derive(Clone, Debug)]
pub struct WebhookMessageBuilder {
    blocks: Option<Vec<Block>>,
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
    response_type: Option<ResponseType>,
    replace_original: Option<bool>,
    delete_original: Option<bool>,
    unfurl_links: Option<bool>,
    unfurl_media: Option<bool>,
    metadata: Option<Map<String, Value>>,
    extensions: Map<String, Value>,
}
impl Default for WebhookMessageBuilder {
    fn default() -> Self {
        Self {
            blocks: None,
            attachments: None,
            text: None,
            response_type: None,
            replace_original: None,
            delete_original: None,
            unfurl_links: None,
            unfurl_media: None,
            metadata: None,
            extensions: Map::new(),
        }
    }
}
impl WebhookMessage {
    /// Starts a builder with the documented model defaults.
    pub fn builder() -> WebhookMessageBuilder {
        WebhookMessageBuilder::default()
    }
    /// Moves every field into an editable builder, preserving explicit omissions.
    pub fn into_builder(self) -> WebhookMessageBuilder {
        WebhookMessageBuilder {
            blocks: self.blocks,
            attachments: self.attachments,
            text: self.text,
            response_type: self.response_type,
            replace_original: self.replace_original,
            delete_original: self.delete_original,
            unfurl_links: self.unfurl_links,
            unfurl_media: self.unfurl_media,
            metadata: self.metadata,
            extensions: self.extensions,
        }
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
    /// Borrows or copies `response_type`. Sets who can see the response.
    pub fn response_type(&self) -> Option<ResponseType> {
        self.response_type
    }
    /// Borrows or copies `replace_original`. Sets whether the response replaces the message that triggered it.
    pub fn replace_original(&self) -> Option<bool> {
        self.replace_original
    }
    /// Borrows or copies `delete_original`. Sets whether the response deletes the message that triggered it.
    pub fn delete_original(&self) -> Option<bool> {
        self.delete_original
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
        if let Some(value) = &self.blocks {
            map.serialize_entry("blocks", value)?;
        }
        if let Some(value) = &self.attachments {
            map.serialize_entry("attachments", value)?;
        }
        if let Some(value) = &self.text {
            map.serialize_entry("text", value)?;
        }
        if let Some(value) = &self.response_type {
            map.serialize_entry("response_type", value)?;
        }
        if let Some(value) = &self.replace_original {
            map.serialize_entry("replace_original", value)?;
        }
        if let Some(value) = &self.delete_original {
            map.serialize_entry("delete_original", value)?;
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
impl WebhookMessageBuilder {
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
    /// Sets who can see the response.
    pub fn response_type(mut self, value: ResponseType) -> Self {
        self.response_type = Some(value);
        self
    }
    /// Omits `response_type`, including any model default.
    pub fn clear_response_type(mut self) -> Self {
        self.response_type = None;
        self
    }
    /// Sets whether the response replaces the message that triggered it.
    pub fn replace_original(mut self, value: bool) -> Self {
        self.replace_original = Some(value);
        self
    }
    /// Omits `replace_original`, including any model default.
    pub fn clear_replace_original(mut self) -> Self {
        self.replace_original = None;
        self
    }
    /// Sets whether the response deletes the message that triggered it.
    pub fn delete_original(mut self, value: bool) -> Self {
        self.delete_original = Some(value);
        self
    }
    /// Omits `delete_original`, including any model default.
    pub fn clear_delete_original(mut self) -> Self {
        self.delete_original = None;
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
    pub fn build(self) -> Result<WebhookMessage, ValidationError> {
        let blocks = self.blocks;
        let attachments = self.attachments;
        let text = self.text;
        let response_type = self.response_type;
        let replace_original = self.replace_original;
        let delete_original = self.delete_original;
        let unfurl_links = self.unfurl_links;
        let unfurl_media = self.unfurl_media;
        let metadata = self.metadata;
        wire::extensions(
            &self.extensions,
            &[
                "blocks",
                "attachments",
                "text",
                "response_type",
                "replace_original",
                "delete_original",
                "unfurl_links",
                "unfurl_media",
                "metadata",
            ],
            "WebhookMessage",
        )?;
        let value = WebhookMessage {
            blocks,
            attachments,
            text,
            response_type,
            replace_original,
            delete_original,
            unfurl_links,
            unfurl_media,
            metadata,
            extensions: self.extensions,
        };
        let wire = serde_json::to_value(&value).map_err(|e| {
            ValidationError::new(ErrorCategory::TypeMismatch, "WebhookMessage", e.to_string())
        })?;
        if let Some(v) = wire.get("blocks") {
            crate::rules::limits(v, "message.blocks", "WebhookMessage.blocks")?;
        }
        if let Some(v) = wire.get("attachments") {
            crate::rules::limits(v, "message.attachments", "WebhookMessage.attachments")?;
        }
        crate::rules::validate("WebhookMessage", &wire, "WebhookMessage")?;
        Ok(value)
    }
}
impl Serialize for WebhookMessage {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.serialize_value(s, true)
    }
}
impl FromWire for WebhookMessage {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let blocks = wire::field::<Vec<Block>>(&mut map, "blocks", path, false)?;
        let attachments = wire::field::<Vec<Attachment>>(&mut map, "attachments", path, false)?;
        let text = wire::field::<String>(&mut map, "text", path, false)?;
        let response_type = wire::field::<ResponseType>(&mut map, "response_type", path, false)?;
        let replace_original = wire::field::<bool>(&mut map, "replace_original", path, false)?;
        let delete_original = wire::field::<bool>(&mut map, "delete_original", path, false)?;
        let unfurl_links = wire::field::<bool>(&mut map, "unfurl_links", path, false)?;
        let unfurl_media = wire::field::<bool>(&mut map, "unfurl_media", path, false)?;
        let metadata = wire::field::<Map<String, Value>>(&mut map, "metadata", path, false)?;
        WebhookMessageBuilder {
            blocks,
            attachments,
            text,
            response_type,
            replace_original,
            delete_original,
            unfurl_links,
            unfurl_media,
            metadata,
            extensions: map,
        }
        .build()
        .map_err(|e| e.at(path))
    }
}
impl TryFrom<Value> for WebhookMessage {
    type Error = ValidationError;
    fn try_from(value: Value) -> Result<Self, ValidationError> {
        Self::from_wire(value, "WebhookMessage")
    }
}
impl<'de> Deserialize<'de> for WebhookMessage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
