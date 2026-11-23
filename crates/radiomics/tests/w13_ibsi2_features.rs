//! IBSI 2 phase 2: intensity statistics of filtered CT images against the consensus values.
//!
//! The opt-in test reads the IBSI CT radiomics phantom (CC BY-NC, never committed) from
//! `IBSI1_CT_PHANTOM`, the directory holding `image/phantom.nii.gz` and `mask/mask.nii.gz`:
//!
//! ```text
//! IBSI1_CT_PHANTOM=<...>/ct_radiomics_phantom/nifti \
//!   cargo +1.96.0 test --release --test w13_ibsi2_features -- --nocapture
//! ```
//!
//! The consensus table `tests/data/w13/reference_values.csv` (CC0, IBSI 2 reference data) is
//! committed. Configurations follow the IBSI 2 manual (arXiv:2006.05470v9, Tables 6.1, 6.2):
//!
//! * A: per-slice 2-D filtering on the source grid, no interpolation. The loader does not round
//!   source intensities, so this route rounds the HU values (ties to even) and calls `extract`
//!   on the arrays.
//! * B: 3-D filtering after IBSI interpolation to 1 mm (cubic B-spline, nearest-integer
//!   rounding, mask threshold 0.5), through `extract_files`.
//!
//! Both re-segment the ROI to [-1000, 400] HU on the unfiltered image. Pass criterion, as
//! published: |value - consensus| <= tolerance. No tolerance is loosened.

use indexmap::IndexMap;
use ndarray::ArrayD;
use radiomics::io::{HostArray, Volume, read_image, read_mask};
use radiomics::settings::{
    FilterPadding, FourierGrid, GaborResponse, LogKernel, OrientationPooling,
};
use radiomics::{FeatureValue, Output};
use radiomics::{
    IbsiInterpolation, ImageSettings, ImageType, Interpolator, Settings, TransformSettings,
    extract, extract_files,
};
use std::collections::BTreeSet;
use std::f64::consts::PI;
use std::path::{Path, PathBuf};

/// Source intensities rounded to the nearest integer, ties to even (the IBSI 2 route A
/// rounds the CT numbers before filtering).
fn rounded_intensities(image: &HostArray) -> HostArray {
    let values: ArrayD<f64> = match image {
        HostArray::U8(a) => a.mapv(f64::from),
        HostArray::I16(a) => a.mapv(f64::from),
        HostArray::U16(a) => a.mapv(f64::from),
        HostArray::I32(a) => a.mapv(f64::from),
        HostArray::F32(a) => a.mapv(f64::from),
        HostArray::F64(a) => a.clone(),
        other => panic!("unsupported CT pixel type {}", other.dtype_name()),
    };
    HostArray::from(values.mapv(f64::round_ties_even))
}

const REFERENCE: &str = include_str!("data/w13/reference_values.csv");

/// IBSI statistic tag -> public feature name.
const FEATURES: [(&str, &str); 18] = [
    ("stat_mean", "firstorder_Mean"),
    ("stat_var", "firstorder_Variance"),
    ("stat_skew", "firstorder_Skewness"),
    ("stat_kurt", "firstorder_Kurtosis"),
    ("stat_median", "firstorder_Median"),
    ("stat_min", "firstorder_Minimum"),
    ("stat_p10", "firstorder_10Percentile"),
    ("stat_p90", "firstorder_90Percentile"),
    ("stat_max", "firstorder_Maximum"),
    ("stat_iqr", "firstorder_InterquartileRange"),
    ("stat_range", "firstorder_Range"),
    ("stat_mad", "firstorder_MeanAbsoluteDeviation"),
    ("stat_rmad", "firstorder_RobustMeanAbsoluteDeviation"),
    ("stat_medad", "firstorder_MedianAbsoluteDeviation"),
    ("stat_cov", "firstorder_CoefficientOfVariation"),
    ("stat_qcod", "firstorder_QuartileCoefficientOfDispersion"),
    ("stat_energy", "firstorder_Energy"),
    ("stat_rms", "firstorder_RootMeanSquared"),
];

/// `filter_id -> feature_tag -> (consensus, tolerance)`; `None` where none is published.
type Reference = IndexMap<String, IndexMap<String, Option<(f64, f64)>>>;

fn reference() -> Reference {
    let mut table = Reference::new();
    let mut lines = REFERENCE.lines();
    assert_eq!(
        lines.next(),
        Some("filter_id;feature;feature_tag;consensus_value;tolerance")
    );
    for line in lines.filter(|l| !l.trim().is_empty()) {
        let cells: Vec<&str> = line.split(';').collect();
        assert_eq!(cells.len(), 5, "malformed row: {line}");
        let entry = if cells[3].is_empty() || cells[4].is_empty() {
            None
        } else {
            Some((
                cells[3].parse::<f64>().expect(line),
                cells[4].parse::<f64>().expect(line),
            ))
        };
        let previous = table
            .entry(cells[0].to_string())
            .or_default()
            .insert(cells[2].to_string(), entry);
        assert!(previous.is_none(), "duplicate row: {line}");
    }
    table
}

#[derive(Clone, Copy, PartialEq)]
enum Config {
    A,
    B,
}

impl Config {
    fn letter(self) -> &'static str {
        match self {
            Config::A => "A",
            Config::B => "B",
        }
    }
    fn planar(self) -> bool {
        self == Config::A
    }
}

/// One configured case: a consensus filter id and the Rust settings that realise it.
struct Case {
    /// Consensus id such as `8.A`.
    id: String,
    /// Display label, with the Fourier grid for Simoncelli.
    label: String,
    transforms: TransformSettings,
}

fn transforms(kind: ImageType) -> TransformSettings {
    TransformSettings {
        image_types: vec![kind],
        ..TransformSettings::default()
    }
}

/// Table 6.2 filters 1 to 9 for one configuration; Simoncelli on both Fourier grids.
fn cases(config: Config) -> Vec<Case> {
    let planar = config.planar();
    let mirror = FilterPadding::Mirror;
    let mut out = Vec::new();
    let mut push = |filter: u32, label: String, t: TransformSettings| {
        out.push(Case {
            id: format!("{filter}.{}", config.letter()),
            label,
            transforms: t,
        });
    };
    push(1, "original".into(), TransformSettings::default());

    let mut t = transforms(ImageType::Mean);
    t.mean.support = 5;
    t.mean.padding = mirror;
    t.mean.planar = planar;
    push(2, "mean".into(), t);

    let mut t = transforms(ImageType::Log);
    t.log.sigma = vec![1.5];
    t.log.kernel = LogKernel::Sampled;
    t.log.cutoff = 4.0;
    t.log.padding = mirror;
    t.log.planar = planar;
    push(3, "log".into(), t);

    let mut t = transforms(ImageType::Laws);
    t.laws.kernel = if planar { "L5E5" } else { "L5E5E5" }.into();
    t.laws.padding = mirror;
    t.laws.pooling = OrientationPooling::Max;
    t.laws.energy_distance = Some(7);
    push(4, "laws".into(), t);

    // Delta theta = pi/8 over a full turn; the modulus at theta and theta + pi is equal.
    let mut t = transforms(ImageType::Gabor);
    t.gabor.sigma = vec![5.0];
    t.gabor.wavelength = vec![2.0];
    t.gabor.gamma = 1.5;
    t.gabor.theta = (0..16).map(|k| f64::from(k) * PI / 8.0).collect();
    t.gabor.response = GaborResponse::Modulus;
    t.gabor.pooling = OrientationPooling::Average;
    t.gabor.padding = mirror;
    t.gabor.slice_axis = planar.then_some(0);
    push(5, "gabor".into(), t);

    for (filter, band2d, band3d, level) in [(6, "LH", "LLH", 1), (7, "HH", "HHH", 2)] {
        let mut t = transforms(ImageType::SeparableWavelet);
        t.separable_wavelet.name = "db3".into();
        t.separable_wavelet.bands = vec![(if planar { band2d } else { band3d }).into()];
        t.separable_wavelet.level = level;
        t.separable_wavelet.padding = mirror;
        t.separable_wavelet.pooling = OrientationPooling::Average;
        t.separable_wavelet.planar = planar;
        push(filter, "separable wavelet".into(), t);
    }

    for (filter, level) in [(8, 1), (9, 2)] {
        for grid in [FourierGrid::Centered, FourierGrid::Dft] {
            let mut t = transforms(ImageType::Simoncelli);
            t.simoncelli.levels = vec![level];
            t.simoncelli.grid = grid;
            t.simoncelli.planar = planar;
            let name = if grid == FourierGrid::Centered {
                "simoncelli centred"
            } else {
                "simoncelli dft"
            };
            push(filter, name.into(), t);
        }
    }
    out
}

fn settings() -> Settings {
    Settings {
        features: FEATURES
            .iter()
            .map(|(_, name)| (*name).to_string())
            .collect(),
        resegment_range: Some(vec![-1000.0, 400.0]),
        ..Settings::default()
    }
}

fn image_settings(config: Config) -> ImageSettings {
    match config {
        Config::A => ImageSettings::default(),
        Config::B => {
            let mut ibsi = IbsiInterpolation::new(vec![1.0, 1.0, 1.0]);
            ibsi.image_interpolator = Interpolator::Bspline;
            ImageSettings {
                ibsi_interpolation: Some(ibsi),
                ..ImageSettings::default()
            }
        }
    }
}

#[test]
fn reference_table_parses_and_every_filter_id_has_a_configured_case() {
    let table = reference();
    let ids: BTreeSet<&str> = table.keys().map(String::as_str).collect();
    let expected: BTreeSet<String> = (1..=9)
        .flat_map(|f| ["A", "B"].map(|c| format!("{f}.{c}")))
        .collect();
    assert_eq!(
        ids,
        expected.iter().map(String::as_str).collect::<BTreeSet<_>>()
    );

    let tags: BTreeSet<&str> = FEATURES.iter().map(|(tag, _)| *tag).collect();
    let mut unpublished = Vec::new();
    for (id, rows) in &table {
        assert_eq!(
            rows.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            tags,
            "{id}"
        );
        for (tag, entry) in rows {
            match entry {
                None => unpublished.push(format!("{id} {tag}")),
                Some((value, tolerance)) => {
                    assert!(value.is_finite() && *tolerance > 0.0, "{id} {tag}");
                }
            }
        }
    }
    // The only entry without a published consensus is 8.B quartile dispersion.
    assert_eq!(unpublished, ["8.B stat_qcod"]);

    // Every id maps to at least one configured case, and filters 8 and 9 to both grids.
    for config in [Config::A, Config::B] {
        let mapped = cases(config);
        for filter in 1..=9 {
            let id = format!("{filter}.{}", config.letter());
            let n = mapped.iter().filter(|c| c.id == id).count();
            assert_eq!(n, if filter >= 8 { 2 } else { 1 }, "{id}");
        }
        assert_eq!(mapped.len(), 11);
    }
}

fn run(
    config: Config,
    phantom: &Path,
    case: &Case,
    arrays: Option<&(Volume, Volume)>,
) -> IndexMap<String, f64> {
    let settings = settings();
    let result = match (config, arrays) {
        (Config::A, Some((image, mask))) => extract(
            &rounded_intensities(&image.array),
            &mask.array,
            &settings,
            &case.transforms,
            Some(&image.spacing),
            Output::Roi,
        ),
        _ => extract_files(
            &phantom.join("image/phantom.nii.gz"),
            &phantom.join("mask/mask.nii.gz"),
            &settings,
            &image_settings(config),
            &case.transforms,
            Output::Roi,
        ),
    }
    .unwrap_or_else(|e| panic!("{} {}: {e}", case.id, case.label));
    result
        .features
        .into_iter()
        .filter_map(|(name, value)| match value {
            FeatureValue::Scalar(v) => Some((name, v)),
            _ => None,
        })
        .collect()
}

/// Known failures on the manual's DFT Fourier grid, as `(id, grid label, tag, value,
/// consensus, tolerance)`. The consensus used the centred, endpoint-inclusive grid (the
/// manual's figure and MIRP), not the 2 pi / N step of its text; the centred grid passes every
/// row. mlx-radiomics v1.1.0 documents the same two statistics as outside tolerance. The
/// numbers are pinned so a change in either direction is noticed; no tolerance is loosened.
const KNOWN_DFT_GRID_FAILURES: &[(&str, &str, &str, f64, f64, f64)] = &[
    (
        "8.A",
        "simoncelli dft",
        "stat_qcod",
        -314.7793,
        -441.0,
        29.0,
    ),
    ("9.B", "simoncelli dft", "stat_min", -599.4526, -605.0, 2.0),
];

#[test]
fn ibsi2_filtered_ct_statistics_match_consensus() {
    let Some(dir) = std::env::var_os("IBSI1_CT_PHANTOM") else {
        eprintln!("skipped: set IBSI1_CT_PHANTOM to the CT phantom nifti directory to run");
        return;
    };
    let phantom = PathBuf::from(dir);
    let table = reference();
    let mut unexplained = Vec::new();
    let mut scored_total = 0;
    let mut passed_total = 0;
    // Optional per-row record for figures (`IBSI_ROWS_DIR`).
    let mut rows = String::from("filter_id,config,label,tag,value,consensus,tolerance\n");
    for config in [Config::A, Config::B] {
        let pair = (config == Config::A).then(|| {
            (
                read_image(&phantom.join("image/phantom.nii.gz")).unwrap(),
                read_mask(&phantom.join("mask/mask.nii.gz")).unwrap(),
            )
        });
        for case in cases(config) {
            let values = run(config, &phantom, &case, pair.as_ref());
            let (mut count, mut passed) = (0, 0);
            let mut worst = (0.0_f64, "");
            for (tag, name) in FEATURES {
                let Some((consensus, tolerance)) = table[&case.id][tag] else {
                    continue;
                };
                let key = values
                    .keys()
                    .find(|k| k.ends_with(&format!("_{name}")))
                    .unwrap_or_else(|| panic!("{} {}: no {name}", case.id, case.label));
                let value = values[key];
                let ratio = (value - consensus).abs() / tolerance;
                rows.push_str(&format!(
                    "{},{},{},{tag},{value:e},{consensus:e},{tolerance:e}\n",
                    case.id,
                    config.letter(),
                    case.label
                ));
                count += 1;
                if ratio > worst.0 {
                    worst = (ratio, tag);
                }
                if ratio <= 1.0 {
                    passed += 1;
                } else {
                    println!(
                        "  FAIL {} [{}] {tag}: value {value:.6e} consensus {consensus:.6e} \
                         tolerance {tolerance:.3e} (|d|/tol {ratio:.3})",
                        case.id, case.label
                    );
                    let known = KNOWN_DFT_GRID_FAILURES
                        .iter()
                        .any(|&(id, label, t, v, c, tol)| {
                            id == case.id
                                && label == case.label
                                && t == tag
                                && (value - v).abs() < 1e-3 * v.abs()
                                && c == consensus
                                && tol == tolerance
                        });
                    if !known {
                        unexplained.push(format!(
                            "{} [{}] {tag}: {value:.6e} vs {consensus:.6e} +/- {tolerance:.3e}",
                            case.id, case.label
                        ));
                    }
                }
            }
            scored_total += count;
            passed_total += passed;
            println!(
                "{:4} {:19} count {count:2} pass {passed:2} worst {:.3} ({})",
                case.id, case.label, worst.0, worst.1
            );
        }
    }
    println!("total {passed_total}/{scored_total}");
    if let Some(dir) = std::env::var_os("IBSI_ROWS_DIR") {
        let path = PathBuf::from(dir).join("ibsi2-phase2.csv");
        std::fs::write(&path, rows).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    }
    assert_eq!(
        scored_total - passed_total,
        KNOWN_DFT_GRID_FAILURES.len(),
        "the failures are exactly the named DFT-grid exceptions"
    );
    assert!(
        unexplained.is_empty(),
        "unexplained failures:\n{}",
        unexplained.join("\n")
    );
}
