//! Parameter-file and settings parity with mlx-radiomics v1.1.0.

use radiomics::params::{Value, load_params, parse_params, settings_from_params};
use radiomics::settings::{DirectionalAggregation, MatrixAggregation};
use radiomics::{
    Error, IbsiInterpolation, ImageSettings, Interpolator, Mode, Settings, list_features,
};
use serde::Deserialize;
use serde_json::Value as Json;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
struct Case {
    case: String,
    yaml: String,
    ndim: usize,
    error: Option<String>,
    settings: Option<Json>,
    image_settings: Option<Json>,
    transforms: Option<Json>,
}

/// serde_json writes non-finite floats as null; the Python reference writes them as strings.
fn non_finite_as_null(value: Json) -> Json {
    match value {
        Json::String(text) if matches!(text.as_str(), "NaN" | "Infinity" | "-Infinity") => {
            Json::Null
        }
        Json::Array(items) => Json::Array(items.into_iter().map(non_finite_as_null).collect()),
        Json::Object(map) => Json::Object(
            map.into_iter()
                .map(|(key, value)| (key, non_finite_as_null(value)))
                .collect(),
        ),
        other => other,
    }
}

struct TempFile(PathBuf);

impl TempFile {
    fn new(name: &str) -> Self {
        Self(std::env::temp_dir().join(format!("radiomics-w0-{name}-{}.yaml", std::process::id())))
    }
}
impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

#[test]
fn python_parameter_files_match_messages_and_all_payload_fields() {
    let file = TempFile::new("golden");
    let cases: Vec<Case> = serde_json::from_str(include_str!("data/w0/parameters.json")).unwrap();
    for row in cases {
        fs::write(&file.0, row.yaml).unwrap();
        let result = load_params(&file.0).and_then(|params| parse_params(&params, row.ndim));
        if let Some(error) = row.error {
            assert_eq!(
                result.unwrap_err(),
                Error::InvalidInput(error),
                "{}",
                row.case
            );
        } else {
            let parsed = result.unwrap_or_else(|e| panic!("{}: {e}", row.case));
            assert_eq!(
                serde_json::to_value(&parsed.settings).unwrap(),
                non_finite_as_null(row.settings.unwrap()),
                "{}",
                row.case
            );
            assert_eq!(
                serde_json::to_value(&parsed.image_settings).unwrap(),
                non_finite_as_null(row.image_settings.unwrap()),
                "{}",
                row.case
            );
            assert_eq!(
                serde_json::to_value(&parsed.transforms).unwrap(),
                non_finite_as_null(row.transforms.unwrap()),
                "{}",
                row.case
            );
        }
    }
}

#[test]
fn malformed_missing_or_non_mapping_yaml_is_invalid_input() {
    let file = TempFile::new("load");
    assert_eq!(
        load_params(&file.0).unwrap_err(),
        Error::InvalidInput(format!("missing params file: {}", file.0.display()))
    );
    fs::write(&file.0, "setting: [").unwrap();
    let Error::InvalidInput(message) = load_params(&file.0).unwrap_err() else {
        panic!("YAML error class")
    };
    assert!(
        message.contains("invalid params YAML") && message.contains("line 1"),
        "{message}"
    );
    fs::write(&file.0, [0xff]).unwrap();
    assert!(matches!(load_params(&file.0), Err(Error::InvalidInput(_))));
    for yaml in ["", "[]", "false", "null"] {
        fs::write(&file.0, yaml).unwrap();
        assert_eq!(
            load_params(&file.0).unwrap_err(),
            Error::InvalidInput("'params' must be a mapping".into())
        );
    }
}

fn yaml(text: &str) -> Value {
    yaml_serde::from_str(text).unwrap()
}

#[test]
fn overlay_preserves_base_and_only_reads_an_explicitly_enabled_axis() {
    let base = Settings {
        force2d_axis: Some(1),
        scratch_mib: 5,
        bin_count: Some(3),
        features: vec!["firstorder_Mean".to_owned()],
        ..Settings::pyradiomics_conventions()
    };
    for (mapping, axis) in [
        ("{}", Some(1)),
        ("{force2Ddimension: bad}", Some(1)),
        ("{force2D: false, force2Ddimension: bad}", None),
        ("{force2D: true}", Some(0)),
        ("{force2D: true, force2Ddimension: 2}", Some(2)),
    ] {
        let params = yaml(&format!("setting: {mapping}"));
        let settings = settings_from_params(&params, Some(&base)).unwrap();
        assert_eq!(
            settings,
            Settings {
                force2d_axis: axis,
                ..base.clone()
            }
        );
    }
    let params = yaml("setting: {binWidth: -1, binCount: 0}");
    let s = settings_from_params(&params, None).unwrap();
    assert_eq!(
        s.validate(3).unwrap_err().to_string(),
        "bin_count must be a positive integer below 2**31"
    );
    assert_eq!(
        settings_from_params(&yaml("setting: {correctMask: true}"), None)
            .unwrap_err()
            .to_string(),
        "unsupported setting keys for this runner: ['correctMask']"
    );
    assert_eq!(
        settings_from_params(&yaml("imageType: {Unknown: {}}"), None).unwrap(),
        Settings::default()
    );
}

#[test]
fn spatial_interpolators_and_zero_spacing_axes() {
    for (name, expected) in [
        ("sitkNearestNeighbor", Interpolator::Nearest),
        ("nearest", Interpolator::Nearest),
        ("sitkLinear", Interpolator::Linear),
        ("linear", Interpolator::Linear),
        ("sitkBSpline", Interpolator::Bspline),
        ("bspline", Interpolator::Bspline),
    ] {
        let parsed = parse_params(
            &yaml(&format!(
                "setting: {{interpolator: {name}, resampledPixelSpacing: [0, 2]}}"
            )),
            2,
        )
        .unwrap();
        assert_eq!(parsed.image_settings.interpolator, expected);
        assert_eq!(parsed.image_settings.resampled_spacing.unwrap(), [2.0, 0.0]);
    }
    let ibsi = IbsiInterpolation::new(vec![0.0, 2.0, 3.0]);
    ibsi.validate().unwrap();
    let invalid = |message: &str| Error::InvalidInput(message.into());
    for spacing in [
        vec![],
        vec![1.0; 4],
        vec![f64::INFINITY, 1.0],
        vec![-0.1, 1.0],
        vec![0.0, f64::NAN],
    ] {
        assert_eq!(
            IbsiInterpolation::new(spacing.clone())
                .validate()
                .unwrap_err(),
            invalid("IBSI spacing requires 2 or 3 finite nonnegative values")
        );
        assert_eq!(
            ImageSettings {
                resampled_spacing: Some(spacing),
                ..Default::default()
            }
            .validate()
            .unwrap_err(),
            invalid("resampled_spacing requires 2 or 3 finite nonnegative values")
        );
    }
    for mask_threshold in [0.0, 1.1, f64::NAN, f64::INFINITY] {
        assert_eq!(
            IbsiInterpolation {
                mask_threshold,
                ..ibsi.clone()
            }
            .validate()
            .unwrap_err(),
            invalid("mask_threshold must be finite and in (0, 1]")
        );
    }
    IbsiInterpolation {
        mask_threshold: 1.0,
        image_interpolator: Interpolator::Bspline,
        ..ibsi.clone()
    }
    .validate()
    .unwrap();
    assert_eq!(
        IbsiInterpolation {
            image_interpolator: Interpolator::Nearest,
            ..ibsi.clone()
        }
        .validate()
        .unwrap_err(),
        invalid("image_interpolator must be linear or bspline")
    );
    let image = ImageSettings {
        ibsi_interpolation: Some(ibsi),
        ..Default::default()
    };
    image.validate().unwrap();
    assert_eq!(
        ImageSettings {
            resampled_spacing: Some(vec![1.0; 2]),
            ..image.clone()
        }
        .validate()
        .unwrap_err(),
        invalid("ibsi_interpolation and resampled_spacing are mutually exclusive")
    );
    assert_eq!(
        ImageSettings {
            correct_mask: true,
            ..image
        }
        .validate()
        .unwrap_err(),
        invalid("ibsi_interpolation interpolates the mask; correct_mask must be False")
    );
}

#[test]
fn direct_settings_and_selected_families_follow_the_same_rules() {
    let family_names = ["firstorder", "shape", "shape2D", "localintensity"];
    for (mode, ndim) in [(Mode::Roi, Some(2)), (Mode::Voxel, None)] {
        let s = Settings {
            features: list_features(&family_names, mode, ndim).unwrap(),
            ..Default::default()
        };
        s.validate(2).unwrap();
    }
    for (s, message) in [
        (
            Settings {
                distances: vec![],
                ..Default::default()
            },
            "distances must contain positive integers",
        ),
        (
            Settings {
                features: vec![],
                ..Default::default()
            },
            "features must be a nonempty unique selection",
        ),
        (
            Settings {
                scratch_mib: 0,
                ..Default::default()
            },
            "scratch_mib must be a positive integer",
        ),
        (
            Settings {
                max_gray_levels: Some(0),
                ..Default::default()
            },
            "max_gray_levels must be a positive integer or None",
        ),
        (
            Settings {
                resegment_range: Some(vec![]),
                ..Default::default()
            },
            "resegment_range must contain one or two finite thresholds",
        ),
    ] {
        assert_eq!(
            s.validate(3).unwrap_err(),
            Error::InvalidInput(message.into())
        );
    }

    for s in [
        Settings {
            glcm_aggregation: DirectionalAggregation::Merge3D,
            ..Default::default()
        },
        Settings {
            glrlm_aggregation: DirectionalAggregation::Average3D,
            ..Default::default()
        },
        Settings {
            glszm_aggregation: MatrixAggregation::ThreeD,
            ..Default::default()
        },
        Settings {
            gldzm_aggregation: MatrixAggregation::ThreeD,
            features: vec!["gldzm_SmallDistanceEmphasis".into()],
            ..Default::default()
        },
        Settings {
            ngtdm_aggregation: MatrixAggregation::ThreeD,
            ..Default::default()
        },
        Settings {
            gldm_aggregation: MatrixAggregation::ThreeD,
            ..Default::default()
        },
    ] {
        s.validate(3).unwrap();
        assert_eq!(
            s.validate(2).unwrap_err().to_string(),
            "3d aggregation requires 3D input without force2d_axis"
        );
        assert!(
            Settings {
                force2d_axis: Some(0),
                ..s
            }
            .validate(3)
            .is_err()
        );
    }
}
