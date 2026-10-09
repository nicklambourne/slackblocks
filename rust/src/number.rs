use crate::{ErrorCategory, ValidationError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Number;

/// A JSON integer, unsigned integer, or finite floating-point number.
///
/// Integers retain their precision through `i64::MIN..=u64::MAX`. Arbitrary
/// precision values that cannot round-trip through the supported representation
/// are rejected, including when a consumer enables serde_json's optional features.
#[derive(Clone, Debug, PartialEq)]
pub struct JsonNumber(Number);
impl JsonNumber {
    /// Borrows the validated Serde JSON number.
    pub fn as_number(&self) -> &Number {
        &self.0
    }
}
impl From<i8> for JsonNumber {
    fn from(value: i8) -> Self {
        Self(Number::from(value))
    }
}
impl From<i16> for JsonNumber {
    fn from(value: i16) -> Self {
        Self(Number::from(value))
    }
}
impl From<i32> for JsonNumber {
    fn from(value: i32) -> Self {
        Self(Number::from(value))
    }
}
impl From<i64> for JsonNumber {
    fn from(value: i64) -> Self {
        Self(Number::from(value))
    }
}
impl From<u8> for JsonNumber {
    fn from(value: u8) -> Self {
        Self(Number::from(value))
    }
}
impl From<u16> for JsonNumber {
    fn from(value: u16) -> Self {
        Self(Number::from(value))
    }
}
impl From<u32> for JsonNumber {
    fn from(value: u32) -> Self {
        Self(Number::from(value))
    }
}
impl From<u64> for JsonNumber {
    fn from(value: u64) -> Self {
        Self(Number::from(value))
    }
}
impl TryFrom<f64> for JsonNumber {
    type Error = ValidationError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Number::from_f64(value).map(Self).ok_or_else(|| {
            ValidationError::new(
                ErrorCategory::OutOfRange,
                "JsonNumber",
                "expected a finite number",
            )
        })
    }
}
impl TryFrom<Number> for JsonNumber {
    type Error = ValidationError;
    fn try_from(value: Number) -> Result<Self, Self::Error> {
        if let Some(n) = value.as_i64() {
            return Ok(n.into());
        }
        if let Some(n) = value.as_u64() {
            return Ok(n.into());
        }
        if let Some(n) = value.as_f64().and_then(Number::from_f64) {
            if decimal(&n.to_string()) == decimal(&value.to_string()) {
                return Ok(Self(n));
            }
        }
        Err(ValidationError::new(
            ErrorCategory::OutOfRange,
            "JsonNumber",
            "number cannot be represented without rounding",
        ))
    }
}
// Compare decimal values without rounding through a float. Normalizes exponent
// notation and insignificant zeroes for consumers using arbitrary_precision.
fn decimal(text: &str) -> Option<(bool, String, i64)> {
    let negative = text.starts_with('-');
    let text = text.trim_start_matches('-');
    let mut parts = text.split(['e', 'E']);
    let mantissa = parts.next()?;
    let exponent = parts
        .next()
        .map(str::parse::<i64>)
        .transpose()
        .ok()?
        .unwrap_or(0);
    let scale = mantissa.split_once('.').map_or(0, |(_, f)| f.len() as i64);
    let mut digits = mantissa.replace('.', "").trim_start_matches('0').to_owned();
    if digits.is_empty() {
        return Some((false, "0".into(), 0));
    }
    let mut exponent = exponent.checked_sub(scale)?;
    while digits.ends_with('0') {
        digits.pop();
        exponent = exponent.checked_add(1)?;
    }
    Some((negative, digits, exponent))
}
impl Serialize for JsonNumber {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for JsonNumber {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Number::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
