//! Strict JSON report values, mirroring `mlxradiomics/contracts/json.py`.
//! Single-threaded CPU orchestration.
//! Status: ported.

use crate::error::{Error, Result};
use serde::{Serialize, Serializer};
use serde_json::{Number, Value};
use yaml_serde::Value as Raw;

/// Convert a serializable value to JSON, preserving nested non-finite floats as strings.
/// Convert before calling a JSON serializer, which otherwise maps non-finite numbers to null.
pub fn json_safe<T: Serialize + ?Sized>(value: &T) -> Result<Value> {
    // This in-memory Serde value preserves IEEE floats and scalar mapping keys.
    // No text/YAML round trip occurs.
    let raw = yaml_serde::to_value(value).map_err(|e| Error::InvalidInput(e.to_string()))?;
    convert(raw)
}

fn convert(raw: Raw) -> Result<Value> {
    Ok(match raw {
        Raw::Null => Value::Null,
        Raw::Bool(value) => Value::Bool(value),
        Raw::String(value) => Value::String(value),
        Raw::Number(value) => {
            if let Some(n) = value.as_u64() {
                Value::from(n)
            } else if let Some(n) = value.as_i64() {
                Value::from(n)
            } else {
                let n = value.as_f64().expect("floating point number");
                Number::from_f64(n).map(Value::Number).unwrap_or_else(|| {
                    Value::String(
                        if n.is_nan() {
                            "NaN"
                        } else if n.is_sign_positive() {
                            "Infinity"
                        } else {
                            "-Infinity"
                        }
                        .into(),
                    )
                })
            }
        }
        Raw::Sequence(items) => {
            Value::Array(items.into_iter().map(convert).collect::<Result<_>>()?)
        }
        Raw::Mapping(items) => {
            let mut object = serde_json::Map::with_capacity(items.len());
            for (key, value) in items {
                object.insert(key_string(key)?, convert(value)?);
            }
            Value::Object(object)
        }
        Raw::Tagged(tagged) => {
            return Err(Error::InvalidInput(format!(
                "cannot serialize value of type {}",
                tagged.tag
            )));
        }
    })
}

fn key_string(key: Raw) -> Result<String> {
    match key {
        Raw::String(s) => Ok(s),
        Raw::Null => Ok("None".into()),
        Raw::Bool(b) => Ok(if b { "True" } else { "False" }.into()),
        Raw::Number(n) if n.is_f64() => {
            let value = n.as_f64().expect("float");
            let repr = format!("{value:?}").to_ascii_lowercase();
            if let Some((mantissa, exponent)) = repr.split_once('e') {
                let exponent: i32 = exponent.parse().expect("decimal exponent");
                Ok(format!("{mantissa}e{exponent:+03}"))
            } else {
                Ok(repr)
            }
        }
        Raw::Number(n) => Ok(n.to_string()),
        Raw::Sequence(items) => {
            // Of Python's sequence types only tuples can be mapping keys.
            let singleton = items.len() == 1;
            let parts: Result<Vec<_>> = items
                .into_iter()
                .map(|item| match item {
                    Raw::String(s) => Ok(crate::config::validation::quoted(&s)),
                    value => key_string(value),
                })
                .collect();
            Ok(format!(
                "({}{})",
                parts?.join(", "),
                if singleton { "," } else { "" }
            ))
        }
        _ => Err(Error::InvalidInput(
            "cannot serialize non-scalar mapping key".into(),
        )),
    }
}

pub(crate) fn serialize_safe<T: Serialize + ?Sized, S: Serializer>(
    value: &T,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    json_safe(value)
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}
