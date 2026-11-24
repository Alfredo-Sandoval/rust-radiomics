//! IBSI 2 reference response maps (opt-in).
//!
//! The reference maps (CC0) and the phase-1 digital phantoms (CC BY 4.0) are large, so this
//! test reads them from the environment and is skipped (passing) when they are absent:
//!
//! ```text
//! IBSI2_REFERENCE_DIR=<...>/ibsi_2_reference_data \
//! IBSI2_PHANTOM_DIR=<...>/ibsi_2_digital_phantom/nifti \
//!   cargo +1.96.0 test --release -p radiomics --lib ibsi2_maps -- --nocapture
//! ```
//!
//! Pass criterion (IBSI 2): every voxel within 1 % of the reference map's intensity range.
//! `--release` is advised: the Gabor maps convolve 64^3 voxels with a 65x65 kernel.
//! The IBSI CT phantoms (CC BY-NC) are never read. Axis mapping: NIfTI (k1, k2, k3) is the
//! native array (k3, k2, k1), no flips.

#![allow(clippy::field_reassign_with_default)] // settings are filled field by field from JSON specs

use crate::config::filtering::{
    FilterPadding, FourierGrid, GaborResponse, LogKernel, OrientationPooling, RieszBase,
};
use crate::imaging::transforms::feature_images;
use crate::{ImageType, TransformSettings};
use flate2::read::GzDecoder;
use ndarray::{ArrayD, IxDyn};
use std::f64::consts::PI;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Official IBSI 2 criterion: fraction of the reference range.
const IBSI2_RANGE_FRACTION: f64 = 0.01;

struct Nifti {
    /// Native (k3, k2, k1) array.
    data: ArrayD<f64>,
    /// Spacing in mm per native axis.
    spacing: Vec<f64>,
}

fn read_nifti(path: &Path) -> Nifti {
    let mut bytes = Vec::new();
    let raw = std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    if path.extension().is_some_and(|e| e == "gz") {
        GzDecoder::new(&raw[..]).read_to_end(&mut bytes).unwrap();
    } else {
        bytes = raw;
    }
    let i16_at = |o: usize| i16::from_le_bytes([bytes[o], bytes[o + 1]]);
    let f32_at = |o: usize| f32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
    assert_eq!(
        i32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        348,
        "little-endian NIfTI-1"
    );
    let ndim = i16_at(40) as usize;
    assert!(ndim >= 3);
    let (nx, ny, nz) = (
        i16_at(42) as usize,
        i16_at(44) as usize,
        i16_at(46) as usize,
    );
    let datatype = i16_at(70);
    let pixdim = [f32_at(80), f32_at(84), f32_at(88)];
    let offset = f32_at(108) as usize;
    let (slope, inter) = (f32_at(112), f32_at(116));
    let count = nx * ny * nz;
    let body = &bytes[offset..];
    let values: Vec<f64> = (0..count)
        .map(|i| match datatype {
            2 => f64::from(body[i]),
            4 => f64::from(i16::from_le_bytes(
                body[2 * i..2 * i + 2].try_into().unwrap(),
            )),
            8 => f64::from(i32::from_le_bytes(
                body[4 * i..4 * i + 4].try_into().unwrap(),
            )),
            16 => f64::from(f32::from_le_bytes(
                body[4 * i..4 * i + 4].try_into().unwrap(),
            )),
            64 => f64::from_le_bytes(body[8 * i..8 * i + 8].try_into().unwrap()),
            256 => f64::from(body[i] as i8),
            512 => f64::from(u16::from_le_bytes(
                body[2 * i..2 * i + 2].try_into().unwrap(),
            )),
            other => panic!("unsupported NIfTI datatype {other}"),
        })
        .map(|v| {
            if slope != 0.0 {
                v * f64::from(slope) + f64::from(inter)
            } else {
                v
            }
        })
        .collect();
    Nifti {
        data: ArrayD::from_shape_vec(IxDyn(&[nz, ny, nx]), values).unwrap(),
        spacing: vec![
            f64::from(pixdim[2]),
            f64::from(pixdim[1]),
            f64::from(pixdim[0]),
        ],
    }
}

fn dirs() -> Option<(PathBuf, PathBuf)> {
    let reference = std::env::var_os("IBSI2_REFERENCE_DIR")?;
    let phantoms = std::env::var_os("IBSI2_PHANTOM_DIR")?;
    Some((reference.into(), phantoms.into()))
}

fn phantom(dir: &Path, name: &str) -> Nifti {
    read_nifti(&dir.join(name).join("image").join(format!("{name}.nii.gz")))
}

fn settings(kind: ImageType) -> TransformSettings {
    let mut s = TransformSettings::default();
    s.image_types = vec![kind];
    s
}

/// One reference map: id, candidate phantoms, settings and the output index to compare.
struct MapCase {
    id: &'static str,
    phantoms: &'static [&'static str],
    settings: TransformSettings,
    output: usize,
}

/// Fallback search list when a map has no pinned phantom.
const ALL: &[&str] = &[
    "checkerboard",
    "impulse",
    "sphere",
    "pattern_1",
    "pattern_2",
    "pattern_3",
    "noise",
];

/// Phantom each reference map was computed on, as established by running every candidate
/// (only the matching phantom reproduces the map; the others miss it by whole ranges).
fn pinned_phantom(id: &str) -> Option<&'static str> {
    Some(match id {
        "1_a_1" | "1_a_2" | "1_a_3" | "1_a_4" | "2_b" | "2_c" | "3_b_1" | "3_b_2" | "3_b_3"
        | "3_c_1" | "3_c_2" | "3_c_3" | "7_a_1" | "7_a_2" | "8_a_1" | "8_a_2" | "8_a_3" => {
            "checkerboard"
        }
        "1_b_1" | "2_a" | "3_a_1" | "3_a_2" | "3_a_3" | "4_a_1" | "4_a_2" | "5_a_1" | "5_a_2"
        | "9_a" => "impulse",
        "4_b_1" | "4_b_2" | "6_a_1" | "6_a_2" | "9_b_1" => "sphere",
        "10_b_1" => "pattern_1",
        _ => return None,
    })
}

fn cases() -> Vec<MapCase> {
    let mut cases = Vec::new();
    for (id, padding) in [
        ("1_a_1", FilterPadding::Constant),
        ("1_a_2", FilterPadding::Nearest),
        ("1_a_3", FilterPadding::Periodic),
        ("1_a_4", FilterPadding::Mirror),
    ] {
        let mut s = settings(ImageType::Mean);
        s.mean.support = 15;
        s.mean.padding = padding;
        cases.push(MapCase {
            id,
            phantoms: ALL,
            settings: s,
            output: 0,
        });
    }
    let mut s = settings(ImageType::Mean);
    s.mean.support = 15;
    s.mean.planar = true;
    cases.push(MapCase {
        id: "1_b_1",
        phantoms: ALL,
        settings: s,
        output: 0,
    });

    for (id, sigma, padding, planar) in [
        ("2_a", 3.0, FilterPadding::Constant, false),
        ("2_b", 5.0, FilterPadding::Mirror, false),
        ("2_c", 5.0, FilterPadding::Mirror, true),
    ] {
        let mut s = settings(ImageType::Log);
        s.log.kernel = LogKernel::Sampled;
        s.log.sigma = vec![sigma];
        s.log.padding = padding;
        s.log.planar = planar;
        cases.push(MapCase {
            id,
            phantoms: ALL,
            settings: s,
            output: 0,
        });
    }
    for (kernel, padding, prefix, planar) in [
        ("E5L5S5", FilterPadding::Constant, "3_a", false),
        ("E3W5R5", FilterPadding::Mirror, "3_b", false),
        ("L5S5", FilterPadding::Mirror, "3_c", true),
    ] {
        let _ = planar;
        for (n, pooling, energy) in [
            (1, OrientationPooling::None, None),
            (2, OrientationPooling::Max, None),
            (3, OrientationPooling::Max, Some(7)),
        ] {
            let mut s = settings(ImageType::Laws);
            s.laws.kernel = kernel.into();
            s.laws.padding = padding;
            s.laws.pooling = pooling;
            s.laws.energy_distance = energy;
            let id: &'static str = Box::leak(format!("{prefix}_{n}").into_boxed_str());
            cases.push(MapCase {
                id,
                phantoms: ALL,
                settings: s,
                output: 0,
            });
        }
    }
    let gabor =
        |sigma: f64, wavelength: f64, gamma: f64, thetas: Vec<f64>, padding, pooling, axis| {
            let mut s = settings(ImageType::Gabor);
            s.gabor.sigma = vec![sigma];
            s.gabor.wavelength = vec![wavelength];
            s.gabor.gamma = gamma;
            s.gabor.theta = thetas;
            s.gabor.response = GaborResponse::Modulus;
            s.gabor.padding = padding;
            s.gabor.pooling = pooling;
            s.gabor.slice_axis = axis;
            s
        };
    cases.push(MapCase {
        id: "4_a_1",
        phantoms: ALL,
        settings: gabor(
            10.0,
            4.0,
            0.5,
            vec![PI / 3.0],
            FilterPadding::Constant,
            OrientationPooling::None,
            Some(0),
        ),
        output: 0,
    });
    cases.push(MapCase {
        id: "4_b_1",
        phantoms: ALL,
        settings: gabor(
            20.0,
            8.0,
            2.5,
            vec![5.0 * PI / 4.0],
            FilterPadding::Mirror,
            OrientationPooling::None,
            Some(0),
        ),
        output: 0,
    });
    // 4_a_2 / 4_b_2: orientations span 2 pi in steps of pi/4 (from 0) and pi/8 (from the
    // 4_b_1 angle 5 pi/4), averaged over orientations and the three orthogonal plane families.
    let ring = |base: f64, count: usize| -> Vec<f64> {
        (0..count)
            .map(|k| base + 2.0 * PI * k as f64 / count as f64)
            .collect()
    };
    cases.push(MapCase {
        id: "4_a_2",
        phantoms: ALL,
        settings: gabor(
            10.0,
            4.0,
            0.5,
            ring(0.0, 8),
            FilterPadding::Constant,
            OrientationPooling::Average,
            None,
        ),
        output: 0,
    });
    cases.push(MapCase {
        id: "4_b_2",
        phantoms: ALL,
        settings: gabor(
            20.0,
            8.0,
            2.5,
            ring(5.0 * PI / 4.0, 16),
            FilterPadding::Mirror,
            OrientationPooling::Average,
            None,
        ),
        output: 0,
    });
    for (id, bands, name, padding, level, pooling) in [
        (
            "5_a_1",
            "LHL",
            "db2",
            FilterPadding::Constant,
            1,
            OrientationPooling::None,
        ),
        (
            "5_a_2",
            "LHL",
            "db2",
            FilterPadding::Constant,
            1,
            OrientationPooling::Average,
        ),
        (
            "6_a_1",
            "HHL",
            "coif1",
            FilterPadding::Periodic,
            1,
            OrientationPooling::None,
        ),
        (
            "6_a_2",
            "HHL",
            "coif1",
            FilterPadding::Periodic,
            1,
            OrientationPooling::Average,
        ),
        (
            "7_a_1",
            "LLL",
            "haar",
            FilterPadding::Mirror,
            2,
            OrientationPooling::Average,
        ),
        (
            "7_a_2",
            "HHH",
            "haar",
            FilterPadding::Mirror,
            2,
            OrientationPooling::Average,
        ),
    ] {
        let mut s = settings(ImageType::SeparableWavelet);
        s.separable_wavelet.name = name.into();
        s.separable_wavelet.bands = vec![bands.into()];
        s.separable_wavelet.level = level;
        s.separable_wavelet.padding = padding;
        s.separable_wavelet.pooling = pooling;
        cases.push(MapCase {
            id,
            phantoms: ALL,
            settings: s,
            output: 0,
        });
    }
    for (id, level) in [("8_a_1", 1), ("8_a_2", 2), ("8_a_3", 3)] {
        let mut s = settings(ImageType::Simoncelli);
        s.simoncelli.levels = vec![level];
        s.simoncelli.grid = FourierGrid::Centered;
        cases.push(MapCase {
            id,
            phantoms: ALL,
            settings: s,
            output: 0,
        });
    }
    for (id, order) in [("9_a", vec![1, 0, 0]), ("9_b_1", vec![0, 2, 0])] {
        let mut s = settings(ImageType::Riesz);
        s.riesz.base = RieszBase::Log;
        s.riesz.orders = vec![order];
        s.riesz.log_sigma = vec![3.0];
        s.riesz.padding = FilterPadding::Constant;
        s.riesz.grid = FourierGrid::Centered;
        cases.push(MapCase {
            id,
            phantoms: ALL,
            settings: s,
            output: 0,
        });
    }
    let mut s = settings(ImageType::Riesz);
    s.riesz.base = RieszBase::Simoncelli;
    s.riesz.orders = vec![vec![0, 2, 0]];
    s.riesz.simoncelli_levels = vec![1];
    s.riesz.padding = FilterPadding::Periodic;
    s.riesz.grid = FourierGrid::Centered;
    cases.push(MapCase {
        id: "10_b_1",
        phantoms: ALL,
        settings: s,
        output: 0,
    });
    cases
}

#[test]
fn ibsi2_reference_response_maps_within_one_percent_of_range() {
    let Some((reference, phantoms)) = dirs() else {
        eprintln!(
            "skipped: set IBSI2_REFERENCE_DIR and IBSI2_PHANTOM_DIR to run the IBSI 2 map checks"
        );
        return;
    };
    // Optional comma-separated map ids (for example `4_a_2,8_a_1`); unset or `all` runs every map.
    let only = std::env::var("IBSI2_MAPS")
        .ok()
        .filter(|ids| !ids.is_empty() && ids != "all");
    let mut failures = Vec::new();
    for case in cases() {
        if only
            .as_deref()
            .is_some_and(|ids| !ids.split(',').any(|id| id == case.id))
        {
            continue;
        }
        let map = read_nifti(
            &reference
                .join("reference_response_maps")
                .join(format!("{}-ValidCRM.nii", case.id)),
        );
        let (lo, hi) = map
            .data
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
                (lo.min(v), hi.max(v))
            });
        let range = hi - lo;
        let mut best: Option<(f64, &str)> = None;
        let pinned = pinned_phantom(case.id);
        let search: &[&str] = pinned.as_ref().map_or(case.phantoms, std::slice::from_ref);
        for name in search {
            let image = phantom(&phantoms, name);
            if image.data.shape() != map.data.shape() {
                continue;
            }
            let started = std::time::Instant::now();
            let output =
                match feature_images(&image.data, None, &image.spacing, &case.settings, None) {
                    Ok(o) => o,
                    Err(error) => {
                        eprintln!("{} on {name}: {error}", case.id);
                        continue;
                    }
                };
            let values = &output[case.output].1;
            let worst = values
                .iter()
                .zip(map.data.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            let fraction = worst / range;
            eprintln!(
                "{} on {name}: max error {:.3e} of range ({:.2?})",
                case.id,
                fraction,
                started.elapsed()
            );
            if best.is_none_or(|(f, _)| fraction < f) {
                best = Some((fraction, name));
            }
            if fraction <= IBSI2_RANGE_FRACTION {
                // Optional record for figures: the computed map as little-endian f64 in
                // native (k3, k2, k1) order, with its shape and phantom.
                if let Some(dir) = std::env::var_os("IBSI2_MAPS_OUT") {
                    let dir = PathBuf::from(dir);
                    let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
                    std::fs::write(dir.join(format!("{}.f64", case.id)), bytes).unwrap();
                    let shape: Vec<String> = values.shape().iter().map(|n| n.to_string()).collect();
                    std::fs::write(
                        dir.join(format!("{}.json", case.id)),
                        format!(
                            "{{\"id\": \"{}\", \"phantom\": \"{name}\", \"shape_k3_k2_k1\": [{}], \"max_error_fraction\": {fraction:e}}}\n",
                            case.id,
                            shape.join(", ")
                        ),
                    )
                    .unwrap();
                }
                break;
            }
        }
        match best {
            Some((fraction, name)) if fraction <= IBSI2_RANGE_FRACTION => {
                eprintln!("PASS {} ({name}): {:.3e} of range", case.id, fraction);
            }
            Some((fraction, name)) => {
                eprintln!("FAIL {} (best {name}): {:.3e} of range", case.id, fraction);
                failures.push(case.id);
            }
            None => {
                eprintln!("FAIL {}: no phantom of matching shape", case.id);
                failures.push(case.id);
            }
        }
    }
    assert!(
        failures.is_empty(),
        "maps outside 1% of range: {failures:?}"
    );
}
