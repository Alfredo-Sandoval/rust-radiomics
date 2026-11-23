//! W8 end-to-end parity: `extract` and `extract_files` against mlx-radiomics v1.1.0 goldens.
//! CPU numerical work through the public workflow; each comparison runs on the test thread.
//! Status: ROI and voxel mode, original and derived images, both convention sets.
//!
//! The goldens in `tests/data/w8` come from `validation/w8_goldens.py`: mlx `extract_files` on
//! the inputs next to them, with the settings, spatial options and derived images parsed
//! from one PyRadiomics-style parameter mapping stored in the golden. The test parses the
//! same mapping with `parse_params`, so both sides run the same configuration.
//! Tolerances are the W11 MLX ones: rtol 2e-5 and atol 2e-6 (rtol 3e-6 for shape).

mod parity_support;

use crate::parity_support::{FeatureValue as GoldenValue, MLX_ATOL, data_dir, mlx_accepts, read};
use indexmap::IndexMap;
use radiomics::MapDtype;
use radiomics::io::{read_image, read_mask};
use radiomics::params::parse_params;
use radiomics::{FeatureValue, Output};
use radiomics::{Settings, TransformSettings};
use radiomics::{extract, extract_files};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct Case {
    case: String,
    image: String,
    mask: String,
    mode: String,
    params: serde_json::Value,
    gray_levels_by_image: IndexMap<String, u32>,
    image_geometry: BTreeMap<String, Vec<f64>>,
    array_shape: Vec<usize>,
    roi_voxels: usize,
    feature_validity: IndexMap<String, Validity>,
    image_scale: IndexMap<String, f64>,
    features: IndexMap<String, GoldenValue>,
    cpu_lbp_m2_local_peak: Option<f64>,
}

#[derive(Deserialize)]
struct Validity {
    finite: usize,
    valid_zero: usize,
    nan: usize,
    positive_infinity: usize,
    negative_infinity: usize,
    excluded: usize,
}

fn w8() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/w8")
}

fn cases(mode: &str) -> Vec<(PathBuf, Case)> {
    let mut found: Vec<_> = std::fs::read_dir(w8())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            let name = p.to_string_lossy();
            name.ends_with(".mlx.json") && !name.contains("export_report")
        })
        .map(|p| {
            let case: Case = crate::parity_support::read(&p);
            (p, case)
        })
        .filter(|(_, c)| c.mode == mode)
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!found.is_empty());
    found
}

struct Setup {
    settings: Settings,
    image_settings: radiomics::ImageSettings,
    transforms: TransformSettings,
}

fn setup(case: &Case) -> Setup {
    let image = w8().join(&case.image);
    let ndim = radiomics::io::read_image(&image).unwrap().array.ndim();
    let params: radiomics::params::Value = yaml_serde::to_value(&case.params).unwrap();
    let parsed = parse_params(&params, ndim).unwrap();
    Setup {
        settings: parsed.settings,
        image_settings: parsed.image_settings,
        transforms: parsed.transforms,
    }
}

#[derive(Default)]
struct Worst {
    abs: f64,
    rel: f64,
    // Largest |error| / (atol + rtol |reference|); at most 1 inside the tolerance.
    budget: f64,
    values: usize,
}

impl Worst {
    fn add(&mut self, key: &str, actual: f64, expected: f64) {
        self.values += 1;
        if actual.is_finite() && expected.is_finite() {
            let error = (actual - expected).abs();
            self.abs = self.abs.max(error);
            if expected.abs() >= 1e-3 {
                self.rel = self.rel.max(error / expected.abs());
            }
            let rtol = if key.contains("_shape_") || key.contains("_shape2D_") {
                crate::parity_support::MLX_SHAPE_RTOL
            } else {
                crate::parity_support::MLX_RTOL
            };
            self.budget = self.budget.max(error / (MLX_ATOL + rtol * expected.abs()));
        }
    }
}

type Table = BTreeMap<String, Worst>;

fn family_of(key: &str) -> String {
    key.split('_').nth(1).unwrap().to_string()
}

fn image_of(key: &str) -> String {
    key.split('_').next().unwrap().to_string()
}

fn report(title: &str, by_family: &Table, by_image: &Table) {
    eprintln!("{title}");
    for (label, table) in [("family", by_family), ("image", by_image)] {
        for (name, w) in table {
            eprintln!(
                "  {label:6} {name:34} values {:6} max abs {:.3e} max rel {:.3e} budget {:.3}",
                w.values, w.abs, w.rel, w.budget
            );
        }
    }
}

/// Float32 rounding that mlx's filters leave in a derived image, in units of the float32
/// epsilon times the image's peak magnitude. Measured: the largest need is reported by the
/// test and never exceeds this.
const FILTER_ULPS: f64 = 4.0;

fn local_key(key: &str) -> &str {
    // `mlx_accepts` recognizes shape keys by prefix, so pass the feature part.
    key.split_once('_').map_or(key, |(_, rest)| rest)
}

fn compare_validity(
    label: &str,
    key: &str,
    actual: &radiomics::metadata::FeatureValidity,
    expected: &Validity,
) {
    assert_eq!(
        (
            actual.finite,
            actual.valid_zero,
            actual.nan,
            actual.positive_infinity,
            actual.negative_infinity,
            actual.excluded
        ),
        (
            expected.finite,
            expected.valid_zero,
            expected.nan,
            expected.positive_infinity,
            expected.negative_infinity,
            expected.excluded
        ),
        "{label} {key}: validity counts"
    );
}

#[allow(clippy::too_many_arguments)]
fn check_result(
    label: &str,
    case: &Case,
    mask: &radiomics::io::Volume,
    result: &radiomics::ExtractionResult<FeatureValue>,
    by_family: &mut Table,
    by_image: &mut Table,
    failures: &mut Vec<String>,
    outside: &mut Vec<String>,
) {
    let voxel = case.mode == "voxel";
    assert_eq!(
        result.features.keys().collect::<Vec<_>>(),
        case.features.keys().collect::<Vec<_>>(),
        "{label}: feature keys and order"
    );
    assert_eq!(
        result.metadata.gray_levels_by_image.as_ref().unwrap(),
        &case.gray_levels_by_image,
        "{label}: gray levels by image"
    );
    assert_eq!(result.metadata.roi_voxels, case.roi_voxels, "{label}");
    assert_eq!(result.metadata.array_shape, case.array_shape, "{label}");
    let geometry = result
        .metadata
        .image_geometry
        .as_ref()
        .expect("file geometry");
    assert_eq!(
        geometry.shape.iter().map(|&s| s as f64).collect::<Vec<_>>(),
        case.image_geometry["shape"],
        "{label}"
    );
    for (name, values) in [
        ("spacing", &geometry.spacing),
        ("origin", &geometry.origin),
        ("direction", &geometry.direction),
    ] {
        assert_eq!(values, &case.image_geometry[name], "{label}: {name}");
    }
    assert_eq!(
        result.metadata.feature_validity.keys().collect::<Vec<_>>(),
        case.feature_validity.keys().collect::<Vec<_>>(),
        "{label}: validity keys"
    );
    let inside = mask.array.eq_label(label_of(case));
    for ((key, found), expected) in result.features.iter().zip(case.features.values()) {
        let image = image_of(key);
        let allowance = if image == "original" || image.starts_with("wavelet") {
            0.0
        } else {
            FILTER_ULPS * f64::from(f32::EPSILON) * case.image_scale[&image]
        };
        let mut check = |actual: f64, reference: f64| {
            let strict = mlx_accepts(local_key(key), actual, reference);
            let scaled = strict
                || (actual.is_finite()
                    && reference.is_finite()
                    && (actual - reference).abs()
                        <= MLX_ATOL
                            + crate::parity_support::MLX_RTOL * reference.abs()
                            + allowance);
            if !strict {
                outside.push(format!("{label} {key}: {actual} vs {reference}"));
            }
            // The CPU and Metal harmonic reductions select different maximal voxels
            // on this fixture. Each reported peak must still match a measured sphere mean.
            let cpu_peak_matches = key == "lbp-3D-m2_localintensity_LocalIntensityPeak"
                && case
                    .cpu_lbp_m2_local_peak
                    .is_some_and(|peak| mlx_accepts(local_key(key), actual, peak));
            if !scaled && !cpu_peak_matches {
                failures.push(format!("{label} {key}: {actual} vs {reference}"));
            }
            by_family
                .entry(family_of(key))
                .or_default()
                .add(key, actual, reference);
            by_image
                .entry(image_of(key))
                .or_default()
                .add(key, actual, reference);
        };
        match (found, expected, voxel) {
            (FeatureValue::Scalar(x), GoldenValue::Scalar(y), false) => check(*x, y.value()),
            (FeatureValue::Map32(map), GoldenValue::Map(samples), true) => {
                assert_eq!(map.shape(), case.array_shape.as_slice(), "{label} {key}");
                let flat: Vec<f32> = map.iter().copied().collect();
                let mut at = 0;
                for (&value, &selected) in flat.iter().zip(inside.iter()) {
                    if selected {
                        check(f64::from(value), samples[at].value());
                        at += 1;
                    } else {
                        assert_eq!(value, 0.0, "{label} {key}: background must be zero");
                    }
                }
                assert_eq!(at, samples.len(), "{label} {key}");
            }
            _ => panic!("{label} {key}: unexpected value kind"),
        }
    }
    for (key, counts) in &result.metadata.feature_validity {
        compare_validity(label, key, counts, &case.feature_validity[key]);
    }
}

fn finish(failures: &[String], outside: &[String]) {
    eprintln!(
        "  {} values exceed the strict rtol 2e-5 / atol 2e-6; {} remain after the float32 \
         filter allowance and the measured CPU peak comparison",
        outside.len(),
        failures.len()
    );
    for line in outside {
        eprintln!("    strict: {line}");
    }
    let mut groups: BTreeMap<String, usize> = BTreeMap::new();
    for f in failures {
        let mut parts = f.split(' ');
        let (case, key) = (parts.next().unwrap(), parts.next().unwrap_or(""));
        let mut k = key.splitn(3, '_');
        let group = format!(
            "{case} {}_{}",
            k.next().unwrap_or(""),
            k.next().unwrap_or("")
        );
        *groups.entry(group).or_default() += 1;
    }
    for (group, count) in &groups {
        eprintln!("  outside tolerance: {group}: {count}");
    }
    assert!(
        failures.is_empty(),
        "{} values outside tolerance, first: {:#?}",
        failures.len(),
        failures
    );
}

fn label_of(case: &Case) -> i64 {
    case.params["setting"]["label"].as_i64().unwrap()
}

fn run_files(
    case: &Case,
    output: Output<'_>,
) -> radiomics::Result<radiomics::ExtractionResult<FeatureValue>> {
    let s = setup(case);
    extract_files(
        &w8().join(&case.image),
        &w8().join(&case.mask),
        &s.settings,
        &s.image_settings,
        &s.transforms,
        output,
    )
}

#[test]
fn extract_files_roi_matches_mlx_for_original_and_derived_images() {
    let (mut by_family, mut by_image) = (Table::new(), Table::new());
    let (mut failures, mut outside) = (Vec::new(), Vec::new());
    for (path, case) in cases("roi") {
        let mask = read_mask(&w8().join(&case.mask)).unwrap();
        let result = run_files(&case, Output::Roi).unwrap_or_else(|e| panic!("{}: {e}", case.case));
        check_result(
            &case.case,
            &case,
            &mask,
            &result,
            &mut by_family,
            &mut by_image,
            &mut failures,
            &mut outside,
        );
        let _ = path;
    }
    report("ROI, extract_files", &by_family, &by_image);
    finish(&failures, &outside);
}

#[test]
fn extract_files_voxel_maps_match_mlx_for_original_and_derived_images() {
    let (mut by_family, mut by_image) = (Table::new(), Table::new());
    let (mut failures, mut outside) = (Vec::new(), Vec::new());
    for (_, case) in cases("voxel") {
        let mask = read_mask(&w8().join(&case.mask)).unwrap();
        let result = run_files(
            &case,
            Output::Maps {
                dtype: MapDtype::Float32,
                max_bytes: None,
            },
        )
        .unwrap_or_else(|e| panic!("{}: {e}", case.case));
        assert_eq!(result.metadata.map_dtype, Some(MapDtype::Float32));
        check_result(
            &case.case,
            &case,
            &mask,
            &result,
            &mut by_family,
            &mut by_image,
            &mut failures,
            &mut outside,
        );
    }
    report("voxel, extract_files", &by_family, &by_image);
    finish(&failures, &outside);
}

#[test]
fn array_extraction_equals_file_extraction_bit_for_bit() {
    for (_, case) in cases("roi")
        .into_iter()
        .filter(|(_, c)| c.case.starts_with("synth"))
    {
        let s = setup(&case);
        let from_files = run_files(&case, Output::Roi).unwrap();
        let image = read_image(&w8().join(&case.image)).unwrap();
        let mask = read_mask(&w8().join(&case.mask)).unwrap();
        let from_arrays = extract(
            &image.array,
            &mask.array,
            &s.settings,
            &s.transforms,
            Some(&image.spacing),
            Output::Roi,
        )
        .unwrap();
        assert_eq!(
            from_arrays.features.keys().collect::<Vec<_>>(),
            from_files.features.keys().collect::<Vec<_>>()
        );
        for (key, a) in &from_arrays.features {
            let (FeatureValue::Scalar(a), FeatureValue::Scalar(b)) = (a, &from_files.features[key])
            else {
                panic!("ROI scalars expected")
            };
            assert_eq!(a.to_bits(), b.to_bits(), "{} {key}", case.case);
        }
        assert!(from_arrays.metadata.image_geometry.is_none());
        assert_eq!(
            from_files.metadata.preprocessing,
            "native CPU file I/O, spatial preprocessing and float64 binning"
        );
        assert_ne!(
            from_arrays.metadata.preprocessing,
            from_files.metadata.preprocessing
        );
        assert_eq!(
            from_arrays.metadata.gray_levels_by_image,
            from_files.metadata.gray_levels_by_image
        );
        assert_eq!(from_arrays.metadata.array_dtype, image.array.dtype_name());
    }
}

#[test]
fn w11_fixtures_through_the_public_extract_match_mlx() {
    use crate::parity_support::{Fixture, Golden, records};
    let mut checked = 0;
    for path in records("golden") {
        if !path.to_string_lossy().ends_with(".mlx.json") {
            continue;
        }
        let golden: Golden = read(&path);
        if golden.expected_error.is_some() {
            continue;
        }
        let case: Fixture = read(
            &data_dir()
                .join("cases")
                .join(format!("{}.json", golden.case)),
        );
        let (pixels, labels) = case.arrays();
        let settings = golden.settings();
        let voxel = golden.mode == crate::parity_support::GoldenMode::Voxel;
        let output = if voxel {
            Output::Maps {
                dtype: MapDtype::Float32,
                max_bytes: None,
            }
        } else {
            Output::Roi
        };
        let result = extract(
            &pixels.into(),
            &labels.into(),
            &settings,
            &TransformSettings::default(),
            Some(&case.spacing),
            output,
        )
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert_eq!(
            result
                .metadata
                .gray_levels_by_image
                .as_ref()
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
            ["original"]
        );
        for ((key, found), name) in result.features.iter().zip(&settings.features) {
            assert_eq!(key, &format!("original_{name}"));
            match (found, &golden.features[name]) {
                (FeatureValue::Scalar(x), GoldenValue::Scalar(y)) => {
                    assert!(
                        mlx_accepts(name, *x, y.value()),
                        "{} {name}",
                        path.display()
                    );
                    checked += 1;
                }
                (FeatureValue::Map32(map), GoldenValue::Map(values)) => {
                    for (&x, y) in map.iter().zip(values) {
                        assert!(
                            mlx_accepts(name, f64::from(x), y.value()),
                            "{} {name}: {x} vs {}",
                            path.display(),
                            y.value()
                        );
                        checked += 1;
                    }
                }
                _ => panic!("{} {name}: value kind", path.display()),
            }
        }
    }
    eprintln!("{checked} W11 values through extract");
    assert!(checked > 100_000);
}
