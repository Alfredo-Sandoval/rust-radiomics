//! Voxel-map parity against W11's MLX voxel goldens, through the public `extract`. CPU
//! numerical work; the extraction fans out over centres with rayon and each comparison runs
//! on the test thread.

mod parity_support;

use crate::parity_support::{
    FeatureValue as Reference, Fixture, Golden, MLX_ATOL, MLX_RTOL, data_dir, mlx_accepts, read,
    records,
};
use radiomics::{FeatureValue, MapDtype, Output, TransformSettings, extract};
use std::collections::BTreeMap;

#[test]
fn voxel_maps_match_mlx_goldens() {
    let mut cases = 0;
    let mut errors = 0;
    // family -> (max absolute error, max relative error where |reference| >= 1e-3,
    // max error as a fraction of the allowed atol + rtol * |reference|, values compared)
    let mut worst: BTreeMap<String, (f64, f64, f64, usize)> = BTreeMap::new();
    for path in records("golden") {
        if !path.to_str().unwrap().ends_with(".voxel.mlx.json") {
            continue;
        }
        let golden: Golden = read(&path);
        let case: Fixture = read(
            &data_dir()
                .join("cases")
                .join(format!("{}.json", golden.case)),
        );
        let settings = golden.settings();
        let label = path.file_name().unwrap().to_string_lossy().into_owned();
        let (image, labels) = case.arrays();
        let result = extract(
            &image.into(),
            &labels.into(),
            &settings,
            &TransformSettings::default(),
            Some(&case.spacing),
            Output::Maps {
                dtype: MapDtype::Float32,
                max_bytes: None,
            },
        );
        if let Some(expected) = &golden.expected_error {
            // MLX refuses before any map: a single selected voxel.
            assert_eq!(result.unwrap_err().to_string(), *expected, "{label}");
            errors += 1;
            continue;
        }
        let result = result.unwrap_or_else(|err| panic!("{label}: {err}"));
        assert_eq!(result.features.len(), settings.features.len(), "{label}");
        for ((key, map), name) in result.features.iter().zip(&settings.features) {
            assert_eq!(
                key.strip_prefix("original_"),
                Some(name.as_str()),
                "{label}"
            );
            let FeatureValue::Map32(map) = map else {
                panic!("{label} {key}: expected a float32 voxel map")
            };
            assert_eq!(map.shape(), case.shape.as_slice(), "{label} {key}");
            let Reference::Map(expected) = &golden.features[name] else {
                panic!("{label} {key}: expected a voxel map")
            };
            let family = name.split('_').next().unwrap().to_owned();
            let entry = worst.entry(family).or_default();
            for (&actual, expected) in map.iter().zip(expected) {
                let (actual, expected) = (f64::from(actual), expected.value());
                assert!(
                    mlx_accepts(name, actual, expected),
                    "{label} {key}: got {actual}, expected {expected}"
                );
                entry.3 += 1;
                if actual.is_finite() && expected.is_finite() {
                    let error = (actual - expected).abs();
                    entry.0 = entry.0.max(error);
                    if expected.abs() >= 1e-3 {
                        entry.1 = entry.1.max(error / expected.abs());
                    }
                    entry.2 = entry.2.max(error / (MLX_ATOL + MLX_RTOL * expected.abs()));
                }
            }
        }
        cases += 1;
    }
    assert!(cases > 0 && errors == 8, "{cases} cases, {errors} refusals");
    for (family, (abs, rel, fraction, n)) in &worst {
        eprintln!(
            "{family}: {n} values, max abs {abs:e}, max rel {rel:e}, max fraction of tolerance {fraction:.3}"
        );
    }
}
