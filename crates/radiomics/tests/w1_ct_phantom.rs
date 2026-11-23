//! Opt-in IBSI 1 CT radiomics phantom check (CC BY-NC data: never committed).
//!
//! Set `W1_IBSI_CT_PHANTOM` to the phantom's `nifti` directory (containing `image/phantom.nii.gz`
//! and `mask/mask.nii.gz`) and `W1_IBSI_CT_GOLDENS` to the JSON written by
//! `validation/w1_ct_phantom_goldens.py`. Without both variables the test reports a skip.
//! The check follows the public `extract_files`: the interpolated grid and the gray-level count
//! in the result metadata must match mlx-radiomics.
#![allow(missing_docs)]

use radiomics::{
    IbsiInterpolation, ImageSettings, Interpolator, Output, Settings, TransformSettings,
    extract_files,
};
use serde_json::Value;
use std::path::PathBuf;

fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

#[test]
fn ibsi_ct_phantom_interpolation_and_binning_match_mlx() {
    let (Some(phantom), Some(goldens)) = (
        std::env::var_os("W1_IBSI_CT_PHANTOM"),
        std::env::var_os("W1_IBSI_CT_GOLDENS"),
    ) else {
        eprintln!("skipped: set W1_IBSI_CT_PHANTOM and W1_IBSI_CT_GOLDENS to run");
        return;
    };
    let root = PathBuf::from(phantom);
    let cases: Value = serde_json::from_str(&std::fs::read_to_string(goldens).unwrap()).unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut settings = Settings {
            features: vec!["firstorder_Entropy".into()],
            ..Settings::default()
        };
        for (key, value) in case["settings"].as_object().unwrap() {
            match key.as_str() {
                "bin_width" => settings.bin_width = value.as_f64().unwrap(),
                "bin_count" => settings.bin_count = Some(value.as_u64().unwrap() as u32),
                "resegment_range" => settings.resegment_range = Some(numbers(value)),
                other => panic!("unmapped setting {other}"),
            }
        }
        let mut ibsi = IbsiInterpolation::new(vec![2.0, 2.0, 2.0]);
        ibsi.image_interpolator = match case["interpolator"].as_str().unwrap() {
            "linear" => Interpolator::Linear,
            _ => Interpolator::Bspline,
        };
        let spatial = ImageSettings {
            ibsi_interpolation: Some(ibsi),
            ..ImageSettings::default()
        };
        let result = extract_files(
            &root.join("image/phantom.nii.gz"),
            &root.join("mask/mask.nii.gz"),
            &settings,
            &spatial,
            &TransformSettings::default(),
            Output::Roi,
        )
        .unwrap();
        let geometry = result.metadata.image_geometry.as_ref().unwrap();
        let expected_shape: Vec<usize> = numbers(&case["image"]["shape"])
            .iter()
            .map(|&n| n as usize)
            .collect();
        assert_eq!(geometry.shape, expected_shape, "{name}: shape");
        for (actual, golden, what) in [
            (&geometry.spacing, &case["spacing"], "spacing"),
            (&geometry.origin, &case["origin"], "origin"),
        ] {
            for (a, e) in actual.iter().zip(numbers(golden)) {
                assert!((a - e).abs() < 1e-9, "{name}: {what}");
            }
        }
        let gray_levels = result.metadata.gray_levels_by_image.as_ref().unwrap()["original"];
        assert_eq!(
            u64::from(gray_levels),
            case["gray_levels"].as_u64().unwrap(),
            "{name}: gray levels"
        );
        println!(
            "{name}: grid {:?}, {} gray levels",
            geometry.shape, gray_levels
        );
    }
}
