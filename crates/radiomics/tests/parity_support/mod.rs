//! CPU orchestration for reading W11 reference records and comparing family outputs.
//! Each comparison runs on the calling test thread.

#![allow(dead_code)]

use ndarray::{ArrayD, IxDyn};
use radiomics::{Mode, Settings};
use serde::{Deserialize, de::DeserializeOwned};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const MLX_RTOL: f64 = 2e-5;
pub const MLX_SHAPE_RTOL: f64 = 3e-6;
pub const MLX_ATOL: f64 = 2e-6;

pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/reference")
}

pub fn records(directory: &str) -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(data_dir().join(directory))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

pub fn read<T: DeserializeOwned>(path: &Path) -> T {
    let bytes = std::fs::read(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    assert!(bytes.len() < 1_000_000, "{}", path.display());
    serde_json::from_slice(&bytes).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

#[derive(Deserialize)]
pub struct Fixture {
    pub case: String,
    pub shape: Vec<usize>,
    pub spacing: Vec<f64>,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
    pub label: i64,
    image: Vec<f64>,
    mask: Vec<i64>,
}

impl Fixture {
    pub fn arrays(&self) -> (ArrayD<f64>, ArrayD<i64>) {
        let dim = IxDyn(&self.shape);
        (
            ArrayD::from_shape_vec(dim.clone(), self.image.clone()).unwrap(),
            ArrayD::from_shape_vec(dim, self.mask.clone()).unwrap(),
        )
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GoldenMode {
    Roi,
    Voxel,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Convention {
    Ibsi,
    Pyradiomics,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingsRecord {
    pub convention: Convention,
    pub label: i64,
    bin_width: f64,
    bin_count: Option<u32>,
    ivh_bin_width: Option<f64>,
    ivh_bin_count: Option<u32>,
    distances: Vec<u32>,
    force2d_axis: Option<usize>,
    masked_kernel: bool,
    kernel_radius: u32,
    symmetrical_glcm: bool,
    voxel_array_shift: f64,
}

#[derive(Deserialize)]
pub struct Source {
    pub engine: String,
    pub version: String,
    pub command: String,
}

#[derive(Deserialize)]
pub struct Golden {
    pub metadata: Source,
    pub case: String,
    pub shape: Vec<usize>,
    pub settings: SettingsRecord,
    pub mode: GoldenMode,
    pub features: BTreeMap<String, FeatureValue>,
    pub expected_error: Option<String>,
}

impl Golden {
    pub fn settings(&self) -> Settings {
        let record = &self.settings;
        let base = match record.convention {
            Convention::Ibsi => Settings::default(),
            Convention::Pyradiomics => Settings::pyradiomics_conventions(),
        };
        let mode = match self.mode {
            GoldenMode::Roi => Mode::Roi,
            GoldenMode::Voxel => Mode::Voxel,
        };
        let mut features =
            radiomics::list_features(&[], mode, Some(self.shape.len() as u8)).unwrap();
        if self.expected_error.is_none() {
            features.retain(|key| self.features.contains_key(key));
        }
        Settings {
            label: record.label,
            bin_width: record.bin_width,
            bin_count: record.bin_count,
            distances: record.distances.clone(),
            force2d_axis: record.force2d_axis,
            ivh_bin_width: record.ivh_bin_width,
            ivh_bin_count: record.ivh_bin_count,
            kernel_radius: record.kernel_radius,
            masked_kernel: record.masked_kernel,
            symmetrical_glcm: record.symmetrical_glcm,
            voxel_array_shift: record.voxel_array_shift,
            features,
            ..base
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FeatureValue {
    Scalar(Number),
    Map(Vec<Number>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Number {
    Finite(f64),
    Special(Special),
}

#[derive(Deserialize)]
pub enum Special {
    #[serde(rename = "NaN")]
    Nan,
    Infinity,
    #[serde(rename = "-Infinity")]
    NegInfinity,
}

impl Number {
    pub fn value(&self) -> f64 {
        match self {
            Self::Finite(v) => *v,
            Self::Special(Special::Nan) => f64::NAN,
            Self::Special(Special::Infinity) => f64::INFINITY,
            Self::Special(Special::NegInfinity) => f64::NEG_INFINITY,
        }
    }
}

#[derive(Deserialize)]
pub struct IbsiTable {
    pub rows: Vec<IbsiRow>,
}

#[derive(Deserialize)]
pub struct IbsiRow {
    pub family: String,
    pub name: String,
    pub tag: String,
    pub key: Option<String>,
    pub ibsi_id: Option<String>,
    pub value: Option<f64>,
    pub tolerance: Option<f64>,
}

impl IbsiRow {
    pub fn accepts(&self, actual: f64) -> Option<bool> {
        Some((actual - self.value?).abs() <= self.tolerance?)
    }
}

pub fn mlx_accepts(key: &str, actual: f64, expected: f64) -> bool {
    if !actual.is_finite() || !expected.is_finite() {
        return actual == expected || (actual.is_nan() && expected.is_nan());
    }
    let rtol = if key.starts_with("shape_") || key.starts_with("shape2D_") {
        MLX_SHAPE_RTOL
    } else {
        MLX_RTOL
    };
    (actual - expected).abs() <= MLX_ATOL + rtol * expected.abs()
}
