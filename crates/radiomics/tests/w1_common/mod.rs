//! Shared helpers of the W1 golden tests (`tests/data/w1`).
#![allow(missing_docs)]

use radiomics::io::HostArray;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Mutex;

pub fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/w1")
}

pub fn load_json(name: &str) -> Value {
    let text = std::fs::read_to_string(data_dir().join(name))
        .unwrap_or_else(|e| panic!("missing golden {name}: {e}; run validation/w1_*.py"));
    serde_json::from_str(&text).expect("valid JSON")
}

pub fn as_f64(value: &Value) -> f64 {
    match value {
        Value::Number(n) => n.as_f64().expect("finite number"),
        Value::String(s) if s == "nan" => f64::NAN,
        Value::String(s) if s == "inf" => f64::INFINITY,
        Value::String(s) if s == "-inf" => f64::NEG_INFINITY,
        Value::Object(o) if o.contains_key("nonfinite") => {
            match o["nonfinite"].as_str().expect("repr") {
                "nan" => f64::NAN,
                "inf" => f64::INFINITY,
                _ => f64::NEG_INFINITY,
            }
        }
        other => panic!("not a number: {other}"),
    }
}

pub fn f64_list(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(as_f64)
        .collect()
}

pub fn usize_list(value: &Value) -> Vec<usize> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|v| v.as_u64().expect("unsigned") as usize)
        .collect()
}

/// Flat C-order values of any pixel type as exact `i128` (integers) or bit patterns.
pub enum Flat {
    Int(Vec<i128>),
    Float(Vec<f64>),
}

pub fn flatten(array: &HostArray) -> Flat {
    match array {
        HostArray::Bool(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::U8(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::I8(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::U16(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::I16(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::U32(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::I32(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::U64(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::I64(a) => Flat::Int(a.iter().map(|&v| i128::from(v)).collect()),
        HostArray::F32(a) => Flat::Float(a.iter().map(|&v| f64::from(v)).collect()),
        HostArray::F64(a) => Flat::Float(a.iter().copied().collect()),
    }
}

/// Max absolute difference between the array and a golden record (`dtype`, `shape`,
/// `values`); panics on a dtype or shape mismatch. Integers must be identical.
pub fn compare_record(array: &HostArray, record: &Value, context: &str) -> f64 {
    assert_eq!(
        array.dtype_name(),
        record["dtype"].as_str().unwrap(),
        "{context}: dtype"
    );
    assert_eq!(
        array.shape(),
        usize_list(&record["shape"]).as_slice(),
        "{context}: shape"
    );
    let values = record["values"].as_array().unwrap();
    match flatten(array) {
        Flat::Int(actual) => {
            for (i, (a, e)) in actual.iter().zip(values).enumerate() {
                let expected = match e {
                    Value::Bool(b) => i128::from(*b),
                    other => other
                        .as_i64()
                        .map(i128::from)
                        .unwrap_or_else(|| i128::from(other.as_u64().expect("integer golden"))),
                };
                assert_eq!(*a, expected, "{context}: element {i}");
            }
            0.0
        }
        Flat::Float(actual) => {
            let mut worst = 0.0f64;
            for (i, (a, e)) in actual.iter().zip(values).enumerate() {
                let expected = as_f64(e);
                if a.is_nan() || expected.is_nan() {
                    assert!(
                        a.is_nan() && expected.is_nan(),
                        "{context}: element {i} NaN mismatch"
                    );
                } else if a.is_infinite() || expected.is_infinite() {
                    assert_eq!(*a, expected, "{context}: element {i}");
                } else {
                    worst = worst.max((a - expected).abs());
                }
            }
            worst
        }
    }
}

/// Collect the largest observed error per named quantity and print it at the end of a test.
pub struct Observed(Mutex<Vec<(String, f64)>>);

impl Observed {
    pub const fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }

    pub fn note(&self, name: &str, error: f64) {
        let mut guard = self.0.lock().unwrap();
        match guard.iter_mut().find(|(n, _)| n == name) {
            Some(entry) => entry.1 = entry.1.max(error),
            None => guard.push((name.to_string(), error)),
        }
    }

    pub fn report(&self, title: &str) {
        for (name, error) in self.0.lock().unwrap().iter() {
            println!("{title}: max observed {name} = {error:e}");
        }
    }
}
