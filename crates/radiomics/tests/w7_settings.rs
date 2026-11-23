//! Voxel maps for settings outside the W11 goldens, against mlx-radiomics v1.1.0 maps
//! (`validation/w7_voxel.py`): explicit aggregation, closed-radius support with NGTDM caps,
//! asymmetric unmasked GLCM with two distances, 2-D radius 2, and a resegmented ROI
//! inside a larger shape mask. CPU numerical work, rayon over centres.
//! Status: all five cases and every voxel feature of their dimensionality.

mod parity_support;

use crate::parity_support::{MLX_ATOL, MLX_RTOL, Number, mlx_accepts};
use ndarray::{ArrayD, IxDyn};
use radiomics::{FeatureValue, HostArray, MapDtype, Output, Settings, TransformSettings, extract};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Record {
    name: String,
    shape: Vec<usize>,
    image: Vec<f64>,
    roi: Vec<u8>,
    settings: BTreeMap<String, serde_json::Value>,
    features: Vec<String>,
    maps: BTreeMap<String, Vec<Number>>,
}

fn settings(record: &Record) -> Settings {
    let s = &record.settings;
    macro_rules! text {
        ($k:literal) => {
            s[$k]
                .as_str()
                .unwrap()
                .parse()
                .unwrap_or_else(|_| panic!($k))
        };
    }
    let optional = |k: &str| s[k].as_f64();
    Settings {
        bin_width: s["bin_width"].as_f64().unwrap(),
        bin_count: s["bin_count"].as_u64().map(|v| v as u32),
        distances: s["distances"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u32)
            .collect(),
        force2d_axis: s["force2d_axis"].as_u64().map(|v| v as usize),
        masked_kernel: s["masked_kernel"].as_bool().unwrap(),
        kernel_radius: s["kernel_radius"].as_u64().unwrap() as u32,
        symmetrical_glcm: s["symmetrical_glcm"].as_bool().unwrap(),
        voxel_array_shift: s["voxel_array_shift"].as_f64().unwrap(),
        gldm_alpha: s["gldm_alpha"].as_u64().unwrap() as u32,
        glcm_aggregation: text!("glcm_aggregation"),
        glrlm_aggregation: text!("glrlm_aggregation"),
        glszm_aggregation: text!("glszm_aggregation"),
        gldzm_aggregation: text!("gldzm_aggregation"),
        gldm_aggregation: text!("gldm_aggregation"),
        ngtdm_aggregation: text!("ngtdm_aggregation"),
        neighborhood_support: text!("neighborhood_support"),
        neighborhood_radius: s["neighborhood_radius"].as_u64().unwrap() as u32,
        ngtdm_singular_cases: text!("ngtdm_singular_cases"),
        slice_axis: s["slice_axis"].as_u64().unwrap() as usize,
        resegment_shape: s["resegment_shape"].as_bool().unwrap(),
        resegment_range: s["resegment_range"]
            .as_array()
            .map(|a| a.iter().map(|v| v.as_f64().unwrap()).collect()),
        ivh_bin_width: optional("ivh_bin_width"),
        ivh_bin_count: s["ivh_bin_count"].as_u64().map(|v| v as u32),
        kurtosis: text!("kurtosis"),
        zero_variance_moments: text!("zero_variance_moments"),
        features: record.features.clone(),
        ..Settings::default()
    }
}

#[test]
fn voxel_maps_match_mlx_for_non_default_settings() {
    let records: Vec<Record> = crate::parity_support::read(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/w7/voxel_settings.mlx.json"),
    );
    assert_eq!(records.len(), 5);
    let _ = (MLX_ATOL, MLX_RTOL);
    for record in &records {
        let dim = IxDyn(&record.shape);
        let flag = |v: &[u8]| {
            ArrayD::from_shape_vec(dim.clone(), v.iter().map(|&b| b != 0).collect()).unwrap()
        };
        let roi = flag(&record.roi);
        let image = ArrayD::from_shape_vec(dim.clone(), record.image.clone()).unwrap();
        let settings = settings(record);
        let result = extract(
            &HostArray::from(image),
            &HostArray::from(roi.mapv(u8::from)),
            &settings,
            &TransformSettings::default(),
            None,
            Output::Maps {
                dtype: MapDtype::Float32,
                max_bytes: None,
            },
        );
        let maps = match result {
            Ok(result) => result.features,
            Err(err) => panic!("{}: {err}", record.name),
        };
        assert_eq!(maps.len(), record.features.len(), "{}", record.name);
        let mut worst = 0.0_f64;
        for ((key, map), name) in maps.iter().zip(&record.features) {
            assert_eq!(key.strip_prefix("original_"), Some(name.as_str()));
            let FeatureValue::Map32(map) = map else {
                panic!("{} {key}: expected a float32 map", record.name)
            };
            let key = name;
            for (&actual, expected) in map.iter().zip(&record.maps[key]) {
                let (actual, expected) = (f64::from(actual), expected.value());
                assert!(
                    mlx_accepts(key, actual, expected),
                    "{} {key}: got {actual}, expected {expected}",
                    record.name
                );
                if actual.is_finite() && expected.is_finite() {
                    worst = worst
                        .max((actual - expected).abs() / (MLX_ATOL + MLX_RTOL * expected.abs()));
                }
            }
        }
        eprintln!(
            "{}: {} maps, max fraction of tolerance {worst:.3}",
            record.name,
            maps.len()
        );
    }
}
