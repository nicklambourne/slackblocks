use crate::{
    ValidationError,
    wire::{self, FromWire},
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
/// Optional rich-text style flags. Each receiving element validates its allowed subset.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct RichTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strike: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlink: Option<bool>,
}
impl RichTextStyle {
    /// Starts an empty style, with every flag omitted.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the `bold` flag, retaining explicit false.
    pub fn bold(mut self, value: bool) -> Self {
        self.bold = Some(value);
        self
    }
    /// Returns the supplied `bold` flag.
    pub fn is_bold(&self) -> Option<bool> {
        self.bold
    }
    /// Sets the `client_highlight` flag, retaining explicit false.
    pub fn client_highlight(mut self, value: bool) -> Self {
        self.client_highlight = Some(value);
        self
    }
    /// Returns the supplied `client_highlight` flag.
    pub fn is_client_highlight(&self) -> Option<bool> {
        self.client_highlight
    }
    /// Sets the `code` flag, retaining explicit false.
    pub fn code(mut self, value: bool) -> Self {
        self.code = Some(value);
        self
    }
    /// Returns the supplied `code` flag.
    pub fn is_code(&self) -> Option<bool> {
        self.code
    }
    /// Sets the `highlight` flag, retaining explicit false.
    pub fn highlight(mut self, value: bool) -> Self {
        self.highlight = Some(value);
        self
    }
    /// Returns the supplied `highlight` flag.
    pub fn is_highlight(&self) -> Option<bool> {
        self.highlight
    }
    /// Sets the `italic` flag, retaining explicit false.
    pub fn italic(mut self, value: bool) -> Self {
        self.italic = Some(value);
        self
    }
    /// Returns the supplied `italic` flag.
    pub fn is_italic(&self) -> Option<bool> {
        self.italic
    }
    /// Sets the `strike` flag, retaining explicit false.
    pub fn strike(mut self, value: bool) -> Self {
        self.strike = Some(value);
        self
    }
    /// Returns the supplied `strike` flag.
    pub fn is_strike(&self) -> Option<bool> {
        self.strike
    }
    /// Sets the `unlink` flag, retaining explicit false.
    pub fn unlink(mut self, value: bool) -> Self {
        self.unlink = Some(value);
        self
    }
    /// Returns the supplied `unlink` flag.
    pub fn is_unlink(&self) -> Option<bool> {
        self.unlink
    }
}
impl FromWire for RichTextStyle {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let mut map = wire::object(value, "", path)?;
        let value = Self {
            bold: wire::field(&mut map, "bold", path, false)?,
            client_highlight: wire::field(&mut map, "client_highlight", path, false)?,
            code: wire::field(&mut map, "code", path, false)?,
            highlight: wire::field(&mut map, "highlight", path, false)?,
            italic: wire::field(&mut map, "italic", path, false)?,
            strike: wire::field(&mut map, "strike", path, false)?,
            unlink: wire::field(&mut map, "unlink", path, false)?,
        };
        if let Some(key) = map.keys().next() {
            return Err(wire::mismatch(
                &format!("{path}.{key}"),
                "a known style flag",
            ));
        }
        Ok(value)
    }
}
impl TryFrom<Value> for RichTextStyle {
    type Error = ValidationError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        Self::from_wire(v, "RichTextStyle")
    }
}
impl<'de> Deserialize<'de> for RichTextStyle {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::try_from(Value::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
