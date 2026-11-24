//! Per-center scratch admission for voxel maps. The logical working sets follow
//! mlx-radiomics v1.1.0 so callers can use the same feature selections and budgets.

use crate::config::settings::{DirectionalAggregation, MatrixAggregation};
use crate::features::selection::explicit_texture;
use crate::voxel::extraction::VoxelMemory;
use crate::{Error, Result, Settings};

fn too_large() -> Error {
    Error::InvalidInput(
        "scratch_mib cannot hold one local histogram; reduce bins/kernel radius".into(),
    )
}

#[derive(Default)]
struct PairPopulation {
    directions: u128,
    total: u128,
    largest: u128,
}

impl PairPopulation {
    fn new(side: u128, axes: u32, distances: &[u32]) -> Self {
        // In a radius-d offset box each axis contributes side - |offset| pairs.
        // Subtract the inner box to select the Chebyshev shell, then take one
        // direction from each opposite pair. Forced planes have no offset on their fixed axis.
        let box_pairs = |d: u128| ((2 * d + 1) * side - d * (d + 1)).pow(axes);
        let mut pairs = Self::default();
        for &d in distances {
            let d = u128::from(d);
            if d == 0 || d >= side {
                continue;
            }
            pairs.directions += ((2 * d + 1).pow(axes) - (2 * d - 1).pow(axes)) / 2;
            pairs.total += (box_pairs(d) - box_pairs(d - 1)) / 2;
            pairs.largest = pairs.largest.max((side - d) * side.pow(axes - 1));
        }
        pairs
    }
}

fn sparse_glcm_bytes(levels: u128, features: &[&str], kernel: u128, pairs: u128) -> u128 {
    // Marginal, difference and sum arrays span the gray lattice only for consumers of those arrays.
    let has = |name: &str| features.contains(&name);
    let bytes_per_level = 256 * u128::from(has("Imc1") || has("Imc2"))
        + 32 * u128::from(has("Correlation"))
        + 64 * u128::from(has("DifferenceEntropy") || has("DifferenceVariance"))
        + 128 * u128::from(has("SumEntropy"))
        + 1024 * u128::from(has("ClusterShade"));
    let eigen = if has("MCC") {
        kernel.min(levels).pow(2) * 128
    } else {
        0
    };
    256 * pairs.max(1) + levels * bytes_per_level + eigen
}

fn glcm_working_set(
    settings: &Settings,
    levels: u128,
    kernel: u128,
    names: &[&str],
    pairs: &PairPopulation,
    explicit: bool,
) -> (u128, u128) {
    if names.is_empty() {
        return (16 * levels, 0);
    }
    let pair_means = names.iter().all(|name| {
        matches!(
            *name,
            "Autocorrelation"
                | "JointAverage"
                | "Contrast"
                | "DifferenceAverage"
                | "Dissimilarity"
                | "SumAverage"
        )
    });
    let native_pairs = !explicit
        || (pair_means
            && settings.glcm_aggregation == DirectionalAggregation::Native
            && settings.force2d_axis.is_none());
    let symmetry = if settings.symmetrical_glcm { 2 } else { 1 };
    let n = pairs.largest * symmetry;
    let fused = native_pairs
        && pairs.directions != 0
        && if pair_means {
            kernel <= 125 && n * (levels * levels).max(2 * levels) < 1 << 62
        } else {
            kernel <= 27 && n.pow(3) < 1 << 24 && 4 * n.pow(3) * (2 * levels).pow(3) < 1 << 62
        };
    if fused {
        return (
            if pair_means {
                4 * names.len() as u128
            } else {
                (16 * levels).max(128 * pairs.directions)
            },
            0,
        );
    }
    let bytes = if levels > 512 {
        let population = if explicit { pairs.total } else { pairs.largest };
        sparse_glcm_bytes(levels, names, kernel, population * symmetry)
    } else {
        128 * levels * levels
    };
    (bytes, if native_pairs { 8 * pairs.total } else { 0 })
}

#[derive(Clone, Copy)]
pub(crate) struct FirstOrderGroup {
    pub(crate) rows: usize,
    pub(crate) scratch_bytes: usize,
    pub(crate) cache_bytes: usize,
}

pub(crate) struct Schedule {
    pub(crate) memory: VoxelMemory,
    pub(crate) first: Option<FirstOrderGroup>,
}

pub(super) fn schedule(
    settings: &Settings,
    ndim: usize,
    gray_levels: u32,
    rows: usize,
) -> Result<Schedule> {
    let levels = u128::from(gray_levels);
    let side = 2 * u128::from(settings.kernel_radius) + 1;
    let axes = (ndim - usize::from(settings.force2d_axis.is_some())) as u32;
    let kernel = side.pow(axes);
    let window = kernel * (64 + 16 * ndim as u128) + 4096;
    let budget = u128::from(settings.scratch_mib) << 20;
    if window > budget
        || settings.kernel_radius > i32::MAX as u32
        || kernel > isize::MAX as u128 / (ndim as u128 * 4)
    {
        return Err(too_large());
    }
    let names: Vec<_> = settings.features.iter().map(String::as_str).collect();
    let explicit = explicit_texture(settings, &names);
    let glcm: Vec<_> = names
        .iter()
        .filter_map(|name| name.strip_prefix("glcm_"))
        .collect();
    let pairs = if glcm.is_empty() {
        PairPopulation::default()
    } else {
        PairPopulation::new(side, axes, &settings.distances)
    };
    let (mut bytes, topology) = glcm_working_set(settings, levels, kernel, &glcm, &pairs, explicit);

    let has_distance_zones = names.iter().any(|name| name.starts_with("gldzm_"));
    let small_sizes = kernel <= 125 && kernel.pow(3).saturating_mul(levels.pow(4)) < 1 << 62;
    let runs = (3u128.pow(axes) - 1) / 2;
    let first_features = names
        .iter()
        .filter(|name| name.starts_with("firstorder_"))
        .count() as u128;
    for name in names {
        let family = name.split_once('_').expect("validated feature name").0;
        let fused = small_sizes
            && match family {
                "glrlm" => {
                    !explicit
                        || (settings.force2d_axis.is_none()
                            && settings.glrlm_aggregation == DirectionalAggregation::Native)
                }
                "glszm" => {
                    !explicit
                        || (settings.force2d_axis.is_none()
                            && settings.glszm_aggregation == MatrixAggregation::Native
                            && !has_distance_zones)
                }
                "gldm" | "ngtdm" => !explicit && kernel <= 27,
                _ => false,
            };
        bytes = bytes.max(match family {
            "firstorder" | "intensityhistogram" | "ivh" => 16 * levels,
            "glcm" => 0,
            _ if fused => 17 * 4 * (runs + 2),
            _ => levels * kernel * 64,
        });
    }
    let per_center = window + bytes;
    if per_center > budget {
        return Err(too_large());
    }
    // Retain direction index pairs only if one complete window still fits.
    let retained = if topology <= budget - per_center {
        topology
    } else {
        0
    };
    let batch = (budget - retained) / per_center;
    let scratch = batch * per_center + retained;
    let groups = if !glcm.is_empty() && first_features > 0 && rows as u128 > batch {
        let first_budget = budget.min(8 << 20).saturating_sub(retained);
        let first_per_center = window + levels * 16;
        (first_budget / first_per_center / batch)
            .min((rows as u128).div_ceil(batch))
            .min((budget - scratch) / (batch * first_features * 4))
    } else {
        0
    };
    let mut peak = scratch;
    let first = if groups >= 2 {
        let rows = groups * batch;
        let bytes = rows * (window + levels * 16) + retained;
        let cached = rows * first_features * 4;
        peak = bytes.max(scratch + cached);
        Some(FirstOrderGroup {
            rows: rows as usize,
            scratch_bytes: bytes as usize,
            cache_bytes: cached as usize,
        })
    } else {
        None
    };
    Ok(Schedule {
        memory: VoxelMemory {
            batch_size: usize::try_from(batch).map_err(|_| too_large())?,
            scratch_estimate_bytes: usize::try_from(peak).map_err(|_| too_large())?,
        },
        first,
    })
}
