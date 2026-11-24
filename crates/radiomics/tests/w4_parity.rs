//! Replays the static W4 MLX, independent-oracle and IBSI texture fixtures through the public
//! `extract`, without Python. The fixtures give gray-level lattices; each lattice is fed as the
//! image with unit bin width. Cases whose morphology mask differs from the ROI are skipped.
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::str::FromStr;

use ndarray::{ArrayD, IxDyn};
use radiomics::{FeatureValue, HostArray, Output, Settings, TransformSettings, extract};

const MLX_RTOL: f64 = 2e-5;
const MLX_ATOL: f64 = 2e-6;
const ORACLE_RTOL: f64 = 2e-12;
const ORACLE_ATOL: f64 = 2e-13;
const IBSI_SIGNIFICANT_DIGITS: i32 = 3;
const IBSI_PUBLISHED_TOLERANCE: f64 = 0.;

type Fields<'a> = BTreeMap<&'a str, &'a str>;

// The generator writes a flat JSON object per domain with one field per line.
// Scalars and numeric lists in these fixtures contain no escaped characters.
fn cases(json: &str) -> Vec<Fields<'_>> {
    let mut result = Vec::new();
    let mut current = Fields::new();
    for line in json.lines().map(|line| line.trim().trim_end_matches(',')) {
        match line {
            "[" | "]" | "{" => continue,
            "}" => result.push(std::mem::take(&mut current)),
            _ => {
                let (key, value) = line.split_once(": ").unwrap();
                assert!(
                    current
                        .insert(key.trim_matches('"'), value.trim_matches('"'))
                        .is_none()
                );
            }
        }
    }
    result
}

fn array<T: FromStr>(values: &str) -> Vec<T>
where
    T::Err: Debug,
{
    values
        .trim_matches(['[', ']'])
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Reference {
    Mlx,
    Oracle,
    Ibsi,
}

fn check(json: &str, reference: Reference, expected_count: usize, expected_skipped: usize) {
    let mut count = 0;
    let mut skipped = 0;
    let (mut max_absolute, mut max_relative) = (0f64, 0f64);
    let mut largest = (String::new(), 0.);
    for fields in cases(json) {
        let shape: Vec<usize> = array(fields["shape"]);
        let bins: ArrayD<i32> =
            ArrayD::from_shape_vec(IxDyn(&shape), array(fields["bins"])).unwrap();
        let mask = bins.mapv(|b| u8::from(b > 0));
        // The public API takes one mask. Cases whose morphology mask differs from the ROI
        // (distance zones, shape-mask support) cannot be expressed through it.
        let morphology: Vec<u8> = array(fields["morphology"]);
        if morphology
            .iter()
            .zip(mask.iter())
            .any(|(m, r)| (*m != 0) != (*r != 0))
        {
            skipped += 1;
            continue;
        }
        let image = bins.mapv(f64::from);
        let profile = match fields["profile"] {
            "ibsi" => Settings::default(),
            "pyradiomics" => Settings::pyradiomics_conventions(),
            _ => panic!("unknown profile"),
        };
        let directional = fields["directional"].parse().unwrap();
        let matrix = fields["matrix"].parse().unwrap();
        let settings = Settings {
            bin_width: 1.0,
            features: Vec::new(),
            gldm_alpha: fields["alpha"].parse().unwrap(),
            symmetrical_glcm: fields["symmetric"].parse().unwrap(),
            distances: array(fields["distances"]),
            neighborhood_radius: fields["radius"].parse().unwrap(),
            force2d_axis: (fields["force"] != "null").then(|| fields["force"].parse().unwrap()),
            slice_axis: fields["axis"].parse().unwrap(),
            glcm_aggregation: directional,
            glrlm_aggregation: directional,
            glszm_aggregation: matrix,
            gldzm_aggregation: matrix,
            gldm_aggregation: matrix,
            ngtdm_aggregation: matrix,
            ..profile
        };
        let names: Vec<_> = fields
            .keys()
            .rev()
            .copied()
            .filter(|key| key.contains('_'))
            .collect();
        let spacing = &[2.5, 1.5, 0.75][3 - shape.len()..];
        let settings = Settings {
            features: names.iter().map(|n| (*n).into()).collect(),
            ..settings
        };
        let result = extract(
            &HostArray::from(image),
            &HostArray::from(mask),
            &settings,
            &TransformSettings::default(),
            Some(spacing),
            Output::Roi,
        );
        let actual: Vec<_> = result
            .unwrap_or_else(|e| panic!("{}: {e}", fields["label"]))
            .features
            .into_iter()
            .map(|(key, value)| match value {
                FeatureValue::Scalar(x) => (key.strip_prefix("original_").unwrap().to_owned(), x),
                _ => panic!("scalar expected"),
            })
            .collect();
        assert_eq!(actual.len(), names.len());
        for ((key, value), expected_name) in actual.into_iter().zip(names) {
            assert_eq!(key, expected_name);
            let target: f64 = fields[expected_name].parse().unwrap();
            count += 1;
            if !target.is_finite() {
                assert!(
                    value == target || (value.is_nan() && target.is_nan()),
                    "{} {key} {value} vs {target}",
                    fields["label"]
                );
                continue;
            }
            let error = (value - target).abs();
            if error > max_absolute {
                max_absolute = error;
                largest = (format!("{} {key}", fields["label"]), target);
            }
            if target != 0. {
                max_relative = max_relative.max(error / target.abs());
            }
            let matches = match reference {
                Reference::Mlx => error <= MLX_ATOL + MLX_RTOL * target.abs(),
                Reference::Oracle => error <= ORACLE_ATOL + ORACLE_RTOL * target.abs(),
                Reference::Ibsi => {
                    // Submission workbook column H truncates the absolute difference
                    // at the precision of the benchmark before comparing with column F.
                    let place = if target == 0. {
                        1.
                    } else {
                        10f64
                            .powi(target.abs().log10().floor() as i32 + 1 - IBSI_SIGNIFICANT_DIGITS)
                    };
                    (error / place).trunc() * place <= IBSI_PUBLISHED_TOLERANCE
                }
            };
            assert!(
                matches,
                "{} {key}: {value:.17e} vs {target:.17e} (error {error:e})",
                fields["label"]
            );
        }
    }
    assert_eq!(count, expected_count);
    assert_eq!(skipped, expected_skipped);
    eprintln!(
        "{reference:?}: {count} features, max absolute {max_absolute:.6e} at {} (reference {:.6e}), max relative (nonzero references) {max_relative:.6e}",
        largest.0, largest.1
    );
}

#[test]
fn mlx_v1_1_0_texture_runs() {
    check(include_str!("data/w4/mlx.json"), Reference::Mlx, 1056, 20);
}

#[test]
fn independent_texture_oracles() {
    check(
        include_str!("data/w4/oracles.json"),
        Reference::Oracle,
        1128,
        20,
    );
}

#[test]
fn ibsi_digital_phantom_texture_rows() {
    check(include_str!("data/w4/ibsi.json"), Reference::Ibsi, 408, 0);
}
