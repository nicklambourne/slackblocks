use crate::{MarkdownText, PlainText, Text, ValidationError};

/// A plain-text builder argument: either an owned string or validated plain text.
#[derive(Clone, Debug)]
pub struct PlainTextInput(PlainInput);
#[derive(Clone, Debug)]
enum PlainInput {
    String(String),
    Value(PlainText),
}
impl From<String> for PlainTextInput {
    fn from(v: String) -> Self {
        Self(PlainInput::String(v))
    }
}
impl From<&str> for PlainTextInput {
    fn from(v: &str) -> Self {
        v.to_owned().into()
    }
}
impl From<PlainText> for PlainTextInput {
    fn from(v: PlainText) -> Self {
        Self(PlainInput::Value(v))
    }
}
impl PlainTextInput {
    pub(crate) fn resolve(self) -> Result<PlainText, ValidationError> {
        match self.0 {
            PlainInput::String(s) => PlainText::new(s),
            PlainInput::Value(v) => Ok(v),
        }
    }
}

/// A text builder argument. Strings use the receiving field's documented text kind.
/// There is deliberately no global string-to-`Text` conversion.
#[derive(Clone, Debug)]
pub struct TextInput(Input);
#[derive(Clone, Debug)]
enum Input {
    String(String),
    Value(Text),
}
impl From<String> for TextInput {
    fn from(v: String) -> Self {
        Self(Input::String(v))
    }
}
impl From<&str> for TextInput {
    fn from(v: &str) -> Self {
        v.to_owned().into()
    }
}
impl From<Text> for TextInput {
    fn from(v: Text) -> Self {
        Self(Input::Value(v))
    }
}
impl From<PlainText> for TextInput {
    fn from(v: PlainText) -> Self {
        Text::from(v).into()
    }
}
impl From<MarkdownText> for TextInput {
    fn from(v: MarkdownText) -> Self {
        Text::from(v).into()
    }
}
impl TextInput {
    pub(crate) fn resolve(self, plain: bool) -> Result<Text, ValidationError> {
        match self.0 {
            Input::Value(v) => Ok(v),
            Input::String(s) if plain => PlainText::new(s).map(Into::into),
            Input::String(s) => MarkdownText::new(s).map(Into::into),
        }
    }
}
