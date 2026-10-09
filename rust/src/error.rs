use std::fmt;

/// The shared validation category; message wording is not a compatibility promise.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ErrorCategory {
    /// A string or collection violates a length boundary.
    LengthExceeded,
    /// A number is outside its supported range.
    OutOfRange,
    /// Fields that cannot coexist were provided together.
    MutuallyExclusive,
    /// A value has the wrong type, vocabulary, or receiving role.
    TypeMismatch,
    /// Required input is missing.
    MissingRequired,
    /// A combination or context is invalid.
    InvalidUsage,
}

impl ErrorCategory {
    /// Returns the normative spelling from the shared contract.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LengthExceeded => "length-exceeded",
            Self::OutOfRange => "out-of-range",
            Self::MutuallyExclusive => "mutually-exclusive",
            Self::TypeMismatch => "type-mismatch",
            Self::MissingRequired => "missing-required",
            Self::InvalidUsage => "invalid-usage",
        }
    }
}
impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A checked-construction or parsing error, with a full Slack wire-field path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    category: ErrorCategory,
    path: String,
    message: String,
}
impl ValidationError {
    pub(crate) fn new(
        category: ErrorCategory,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            category,
            path: path.into(),
            message: message.into(),
        }
    }
    /// Returns the stable error category.
    pub const fn category(&self) -> ErrorCategory {
        self.category
    }
    /// Returns the root type, Slack wire fields, and array indexes of the failure.
    pub fn path(&self) -> &str {
        &self.path
    }
    /// Returns a human-readable explanation.
    pub fn message(&self) -> &str {
        &self.message
    }
    pub(crate) fn at(mut self, path: &str) -> Self {
        let suffix = self
            .path
            .find(['.', '['])
            .map(|i| &self.path[i..])
            .unwrap_or("");
        self.path = format!("{path}{suffix}");
        self
    }
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}: {}", self.category, self.path, self.message)
    }
}
impl std::error::Error for ValidationError {}
