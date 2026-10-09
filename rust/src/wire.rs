use crate::{ErrorCategory, JsonNumber, ValidationError};
use serde_json::{Map, Value};

pub(crate) trait FromWire: Sized {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError>;
}
pub(crate) fn mismatch(path: &str, expected: &str) -> ValidationError {
    ValidationError::new(
        ErrorCategory::TypeMismatch,
        path,
        format!("expected {expected}"),
    )
}
pub(crate) fn object(
    value: Value,
    tag: &str,
    path: &str,
) -> Result<Map<String, Value>, ValidationError> {
    let Value::Object(mut map) = value else {
        return Err(mismatch(path, "an object"));
    };
    if !tag.is_empty() && map.remove("type").as_ref().and_then(Value::as_str) != Some(tag) {
        return Err(mismatch(&format!("{path}.type"), tag));
    }
    Ok(map)
}
pub(crate) fn field<T: FromWire>(
    map: &mut Map<String, Value>,
    name: &str,
    path: &str,
    required: bool,
) -> Result<Option<T>, ValidationError> {
    let path = format!("{path}.{name}");
    match map.remove(name) {
        None | Some(Value::Null) if required => Err(ValidationError::new(
            ErrorCategory::MissingRequired,
            path,
            "required field is missing",
        )),
        None | Some(Value::Null) => Ok(None),
        Some(value) => T::from_wire(value, &path).map(Some),
    }
}
pub(crate) fn required<T>(value: Option<T>, path: &str) -> Result<T, ValidationError> {
    value.ok_or_else(|| {
        ValidationError::new(
            ErrorCategory::MissingRequired,
            path,
            "required field is missing",
        )
    })
}
pub(crate) fn extensions(
    map: &Map<String, Value>,
    fields: &[&str],
    path: &str,
) -> Result<(), ValidationError> {
    for key in map.keys() {
        if key.is_empty() || key == "type" || fields.contains(&key.as_str()) {
            return Err(ValidationError::new(
                ErrorCategory::InvalidUsage,
                format!("{path}.{key}"),
                "extension collides with a reserved field",
            ));
        }
    }
    Ok(())
}
impl FromWire for String {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(mismatch(path, "a string")),
        }
    }
}
impl FromWire for bool {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        value.as_bool().ok_or_else(|| mismatch(path, "a boolean"))
    }
}
impl FromWire for i64 {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        if let Some(n) = value.as_i64() {
            return Ok(n);
        }
        if value.as_number().is_some_and(|n| n.is_u64()) {
            return Err(ValidationError::new(
                ErrorCategory::OutOfRange,
                path,
                "integer exceeds i64",
            ));
        }
        Err(mismatch(path, "a signed integer"))
    }
}
impl FromWire for f64 {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let number = JsonNumber::from_wire(value, path)?;
        number
            .as_number()
            .as_f64()
            .filter(|n| n.is_finite())
            .ok_or_else(|| mismatch(path, "a finite number"))
    }
}
impl FromWire for JsonNumber {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value {
            Value::Number(n) => Self::try_from(n).map_err(|e| e.at(path)),
            _ => Err(mismatch(path, "a number")),
        }
    }
}
impl<T: FromWire> FromWire for Vec<T> {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        let Value::Array(values) = value else {
            return Err(mismatch(path, "an array"));
        };
        values
            .into_iter()
            .enumerate()
            .map(|(i, v)| T::from_wire(v, &format!("{path}[{i}]")))
            .collect()
    }
}
impl FromWire for Map<String, Value> {
    fn from_wire(value: Value, path: &str) -> Result<Self, ValidationError> {
        match value {
            Value::Object(m) => Ok(m),
            _ => Err(mismatch(path, "an object")),
        }
    }
}
