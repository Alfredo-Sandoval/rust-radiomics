//! Strict accessors for parameter mappings, mirroring `mlxradiomics/config/validation.py`.
//! Single-threaded CPU orchestration.
//! Status: ported.

use crate::error::{Error, Result};
use yaml_serde::{Mapping, Value};

pub(crate) fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::InvalidInput(message.into()))
}

/// Require a mapping whose keys are all strings, preserving their input order.
pub fn require_mapping<'a>(value: &'a Value, context: &str) -> Result<&'a Mapping> {
    let Value::Mapping(mapping) = value else {
        return invalid(format!("'{context}' must be a mapping"));
    };
    if mapping.keys().any(|key| !matches!(key, Value::String(_))) {
        return invalid(format!("'{context}' keys must be strings"));
    }
    Ok(mapping)
}

// Value accessors ordinarily unwrap YAML tags. At this boundary a tagged object
// (e.g. a Python date) has a different type from its scalar representation.
pub(crate) fn as_text(value: &Value) -> Option<&str> {
    match value {
        Value::String(s) => Some(s),
        _ => None,
    }
}

pub(crate) fn sequence(value: &Value) -> Option<&[Value]> {
    match value {
        Value::Sequence(s) => Some(s),
        _ => None,
    }
}

fn number(value: &Value) -> Option<&yaml_serde::Number> {
    match value {
        Value::Number(n) => Some(n),
        _ => None,
    }
}

/// Read a number, accepting both integer and floating-point scalars.
pub fn require_float(value: &Value, key: &str) -> Result<f64> {
    number(value)
        .and_then(yaml_serde::Number::as_f64)
        .ok_or_else(|| Error::InvalidInput(format!("setting '{key}' must be a number")))
}

/// Read an integer without coercing booleans, floats or strings.
pub fn require_int(value: &Value, key: &str) -> Result<i64> {
    number(value)
        .and_then(yaml_serde::Number::as_i64)
        .ok_or_else(|| Error::InvalidInput(format!("setting '{key}' must be an integer")))
}

/// Read a boolean without coercion.
pub fn require_bool(value: &Value, key: &str) -> Result<bool> {
    match value {
        Value::Bool(b) => Ok(*b),
        _ => invalid(format!("setting '{key}' must be a boolean")),
    }
}

fn require_list<T>(
    value: &Value,
    key: &str,
    kind: &str,
    read: fn(&Value) -> Option<T>,
) -> Result<Vec<T>> {
    sequence(value)
        .filter(|items| !items.is_empty())
        .and_then(|items| items.iter().map(read).collect())
        .ok_or_else(|| Error::InvalidInput(format!("setting '{key}' must be a list of {kind}")))
}

/// Read a nonempty list of integers without coercion.
pub fn require_int_tuple(value: &Value, key: &str) -> Result<Vec<i64>> {
    require_list(value, key, "integers", |v| number(v)?.as_i64())
}

/// Read a nonempty list of numbers, rejecting booleans.
pub fn require_float_tuple(value: &Value, key: &str) -> Result<Vec<f64>> {
    require_list(value, key, "numbers", |v| number(v)?.as_f64())
}

pub(crate) fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

pub(crate) fn unique<T: Eq + std::hash::Hash>(items: impl IntoIterator<Item = T>) -> bool {
    let mut seen = std::collections::HashSet::new();
    items.into_iter().all(|item| seen.insert(item))
}

// Python repr uses single quotes unless double quotes avoid escaping an apostrophe.
pub(crate) fn quoted(value: &str) -> String {
    let quote = if value.contains('\'') && !value.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::from(quote);
    for c in value.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote || c == '\\' => {
                out.push('\\');
                out.push(c);
            }
            c if c.is_control() && (c as u32) < 256 => {
                out.push_str(&format!("\\x{:02x}", c as u32))
            }
            _ => out.push(c),
        }
    }
    out.push(quote);
    out
}

pub(crate) fn sorted_names<'a>(names: impl IntoIterator<Item = &'a str>) -> String {
    let mut names: Vec<_> = names.into_iter().collect();
    names.sort_unstable();
    format!(
        "[{}]",
        names.into_iter().map(quoted).collect::<Vec<_>>().join(", ")
    )
}
