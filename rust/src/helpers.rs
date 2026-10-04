//! Convenience APIs built from the same checked native values as ordinary builders.
use crate::{
    Block, HomeTabView, InputParameter, MessagePayload, MessageResponse, ModalView, Trigger,
    ValidationError, WebhookMessage, Workflow,
};
use serde::Serialize;
use serde_json::{Map, Value};
use std::borrow::Cow;

/// A semantic or six-digit hexadecimal attachment color.
///
/// Named constants use Slack's semantic strings or documented hexadecimal values.
/// Parsing accepts hexadecimal strings with or without one leading `#`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
pub struct AttachmentColor(Cow<'static, str>);
impl AttachmentColor {
    /// Slack's positive semantic color.
    pub const GOOD: Self = Self(Cow::Borrowed("good"));
    /// Slack's warning semantic color.
    pub const WARNING: Self = Self(Cow::Borrowed("warning"));
    /// Slack's negative semantic color.
    pub const DANGER: Self = Self(Cow::Borrowed("danger"));
    /// Red (`#ff0000`).
    pub const RED: Self = Self(Cow::Borrowed("#ff0000"));
    /// Blue (`#0000ff`).
    pub const BLUE: Self = Self(Cow::Borrowed("#0000ff"));
    /// Yellow (`#ffff00`).
    pub const YELLOW: Self = Self(Cow::Borrowed("#ffff00"));
    /// Green (`#00ff00`).
    pub const GREEN: Self = Self(Cow::Borrowed("#00ff00"));
    /// Orange (`#ff8800`).
    pub const ORANGE: Self = Self(Cow::Borrowed("#ff8800"));
    /// Purple (`#8800ff`).
    pub const PURPLE: Self = Self(Cow::Borrowed("#8800ff"));
    /// Black (`#000000`).
    pub const BLACK: Self = Self(Cow::Borrowed("#000000"));
    /// Returns the normalized Slack wire spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::str::FromStr for AttachmentColor {
    type Err = ValidationError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "good" => Ok(Self::GOOD),
            "warning" => Ok(Self::WARNING),
            "danger" => Ok(Self::DANGER),
            _ => {
                let hex = value.strip_prefix('#').unwrap_or(value);
                if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                    return Err(ValidationError::new(
                        crate::ErrorCategory::TypeMismatch,
                        "AttachmentColor",
                        "expected a semantic color or six hexadecimal digits",
                    ));
                }
                Ok(Self(Cow::Owned(format!("#{hex}"))))
            }
        }
    }
}
impl std::fmt::Display for AttachmentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl From<AttachmentColor> for String {
    fn from(value: AttachmentColor) -> Self {
        value.0.into_owned()
    }
}
impl<'de> serde::Deserialize<'de> for AttachmentColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}
impl Workflow {
    /// Builds a workflow from a trigger URL and ordered, validated input parameters.
    /// An empty iterator omits `customizable_input_parameters`.
    ///
    /// # Errors
    /// Returns the same validation errors as [`Trigger::builder`] and [`Workflow::builder`].
    pub fn from_url(
        url: impl Into<String>,
        parameters: impl IntoIterator<Item = InputParameter>,
    ) -> Result<Self, ValidationError> {
        let parameters: Vec<_> = parameters.into_iter().collect();
        let mut trigger = Trigger::builder().url(url);
        if !parameters.is_empty() {
            trigger = trigger.customizable_input_parameters(parameters);
        }
        Self::builder().trigger(trigger.build()?).build()
    }
}

/// Borrowed input shapes for a Block Kit Builder preview URL.
///
/// Blocks are wrapped in a `blocks` object. Payloads are serialized as-is. `Raw`
/// is an explicit, unvalidated escape hatch for an existing JSON object.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum BuilderPayload<'a> {
    /// One validated block.
    Block(&'a Block),
    /// Ordered validated blocks.
    Blocks(&'a [Block]),
    /// A chat message payload.
    Message(&'a MessagePayload),
    /// An incoming webhook payload.
    Webhook(&'a WebhookMessage),
    /// An interaction response payload.
    Response(&'a MessageResponse),
    /// A modal view.
    Modal(&'a ModalView),
    /// A Home tab view.
    Home(&'a HomeTabView),
    /// A JSON object constructed outside the validated API.
    Raw(&'a Map<String, Value>),
}
impl<'a> From<&'a Block> for BuilderPayload<'a> {
    fn from(v: &'a Block) -> Self {
        Self::Block(v)
    }
}
impl<'a> From<&'a [Block]> for BuilderPayload<'a> {
    fn from(v: &'a [Block]) -> Self {
        Self::Blocks(v)
    }
}
impl<'a> From<&'a MessagePayload> for BuilderPayload<'a> {
    fn from(v: &'a MessagePayload) -> Self {
        Self::Message(v)
    }
}
impl<'a> From<&'a WebhookMessage> for BuilderPayload<'a> {
    fn from(v: &'a WebhookMessage) -> Self {
        Self::Webhook(v)
    }
}
impl<'a> From<&'a MessageResponse> for BuilderPayload<'a> {
    fn from(v: &'a MessageResponse) -> Self {
        Self::Response(v)
    }
}
impl<'a> From<&'a ModalView> for BuilderPayload<'a> {
    fn from(v: &'a ModalView) -> Self {
        Self::Modal(v)
    }
}
impl<'a> From<&'a HomeTabView> for BuilderPayload<'a> {
    fn from(v: &'a HomeTabView) -> Self {
        Self::Home(v)
    }
}
fn encode(value: &str) -> String {
    use std::fmt::Write;
    let mut result = String::new();
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            result.push(char::from(b));
        } else {
            write!(result, "%{b:02X}").expect("writing to a String is infallible");
        }
    }
    result
}
/// Creates a Slack Block Kit Builder URL without performing any network requests.
/// Optional team IDs are encoded as a single path segment. JSON is encoded as UTF-8.
///
/// # Errors
/// Returns a Serde error if the supplied payload cannot be serialized.
pub fn block_kit_builder_url<'a>(
    payload: impl Into<BuilderPayload<'a>>,
    team_id: Option<&str>,
) -> Result<String, serde_json::Error> {
    let value = match payload.into() {
        BuilderPayload::Block(v) => serde_json::json!({"blocks":[v]}),
        BuilderPayload::Blocks(v) => serde_json::json!({"blocks":v}),
        BuilderPayload::Message(v) => serde_json::to_value(v)?,
        BuilderPayload::Webhook(v) => serde_json::to_value(v)?,
        BuilderPayload::Response(v) => serde_json::to_value(v)?,
        BuilderPayload::Modal(v) => serde_json::to_value(v)?,
        BuilderPayload::Home(v) => serde_json::to_value(v)?,
        BuilderPayload::Raw(v) => Value::Object(v.clone()),
    };
    Ok(format!(
        "https://app.slack.com/block-kit-builder/{}#{}",
        encode(team_id.unwrap_or("")),
        encode(&serde_json::to_string(&value)?)
    ))
}
