//! Packaged wavelet coefficients (PyWavelets filter taps).
//!
//! Mirrors `mlxradiomics/config/wavelets.py` (57 lines at mlx-radiomics b58783c); the
//! table is `imaging/wavelet_tables.json`, generated from PyWavelets 1.10.0 (MIT; see
//! `THIRD_PARTY_NOTICES.md`).
//! Status: done. Table parsing is CPU orchestration; there is no numerical work.

use crate::error::{Error, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Decomposition filter pair (low-pass and high-pass taps) of one discrete wavelet.
#[derive(Clone, Debug, PartialEq)]
pub struct Wavelet {
    /// Low-pass decomposition taps (`dec_lo`).
    pub low: Vec<f64>,
    /// High-pass decomposition taps (`dec_hi`).
    pub high: Vec<f64>,
}

const TABLE: &str = include_str!("../imaging/wavelet_tables.json");

/// Parse a decoded JSON filter into a vector of floats.
///
/// Errors (`ValueError` in Python): not a nonempty list of finite numbers.
pub fn wavelet_coefficients(value: &Value) -> Result<Vec<f64>> {
    let Some(list) = value.as_array() else {
        return Err(Error::InvalidInput(
            "wavelet coefficients require a numeric list".into(),
        ));
    };
    let mut result = Vec::with_capacity(list.len());
    for item in list {
        match item.as_f64() {
            Some(v) if v.is_finite() => result.push(v),
            _ => {
                return Err(Error::InvalidInput(
                    "wavelet coefficient must be finite numeric".into(),
                ));
            }
        }
    }
    if result.is_empty() {
        return Err(Error::InvalidInput(
            "wavelet filter must not be empty".into(),
        ));
    }
    Ok(result)
}

fn tables() -> Result<&'static HashMap<String, Value>> {
    static CACHE: OnceLock<Option<HashMap<String, Value>>> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            let raw: Value = serde_json::from_str(TABLE).ok()?;
            if raw.get("schema_version")?.as_i64()? != 1 {
                return None;
            }
            Some(
                raw.get("wavelets")?
                    .as_object()?
                    .clone()
                    .into_iter()
                    .collect(),
            )
        })
        .as_ref()
        .ok_or_else(|| Error::InvalidInput("invalid wavelet table schema".into()))
}

/// Load the named wavelet's low- and high-pass filters from `wavelet_tables.json`.
pub fn wavelet_filters(name: &str) -> Result<Wavelet> {
    let Some(table) = tables()?.get(name) else {
        return Err(Error::InvalidInput(format!(
            "unsupported discrete wavelet: {name}"
        )));
    };
    let field = |key: &str| {
        table
            .get(key)
            .ok_or_else(|| Error::InvalidInput("invalid wavelet table schema".into()))
            .and_then(wavelet_coefficients)
    };
    let wavelet = Wavelet {
        low: field("low")?,
        high: field("high")?,
    };
    if wavelet.low.len() != wavelet.high.len() || !wavelet.low.len().is_multiple_of(2) {
        return Err(Error::InvalidInput(
            "wavelet requires equally sized even-length filters".into(),
        ));
    }
    Ok(wavelet)
}
