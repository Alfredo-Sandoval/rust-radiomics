//! Size and tone-difference statistics, mirroring `mlxradiomics/features/distributions.py`.
//! Status: ported. Single-threaded CPU numerical work on occupied integer cells and
//! per-level absolute neighborhood difference sums.

use crate::config::settings::NgtdmSingularCases;
use crate::features::matrices::TextureMatrix;
use crate::numerics::sparse_counts::{Marginal, entropy, marginals, variance};
use crate::{Error, Result};

pub(super) fn size_statistics(matrix: &TextureMatrix, names: &[&str]) -> Result<Vec<f64>> {
    let (gray, size) = marginals(&matrix.counts);
    let n: u64 = gray.values().sum();
    let total = n as f64;
    let nonuniformity = |m: &Marginal| m.values().map(|&c| (c as f64).powi(2)).sum::<f64>() / total;
    let emphasis = |gray_power: i32, size_power: i32| {
        matrix
            .counts
            .iter()
            .map(|(&(i, j), &c)| {
                c as f64 * (i as f64).powi(gray_power) * (j as f64).powi(size_power)
            })
            .sum::<f64>()
            / total
    };
    names
        .iter()
        .map(|&name| {
            Ok(match name {
                "SmallDependenceEmphasis"
                | "ShortRunEmphasis"
                | "SmallAreaEmphasis"
                | "SmallDistanceEmphasis" => emphasis(0, -2),
                "LargeDependenceEmphasis"
                | "LongRunEmphasis"
                | "LargeAreaEmphasis"
                | "LargeDistanceEmphasis" => emphasis(0, 2),
                "LowGrayLevelEmphasis" | "LowGrayLevelRunEmphasis" | "LowGrayLevelZoneEmphasis" => {
                    emphasis(-2, 0)
                }
                "HighGrayLevelEmphasis"
                | "HighGrayLevelRunEmphasis"
                | "HighGrayLevelZoneEmphasis" => emphasis(2, 0),
                "SmallDependenceLowGrayLevelEmphasis"
                | "ShortRunLowGrayLevelEmphasis"
                | "SmallAreaLowGrayLevelEmphasis"
                | "SmallDistanceLowGrayLevelEmphasis" => emphasis(-2, -2),
                "SmallDependenceHighGrayLevelEmphasis"
                | "ShortRunHighGrayLevelEmphasis"
                | "SmallAreaHighGrayLevelEmphasis"
                | "SmallDistanceHighGrayLevelEmphasis" => emphasis(2, -2),
                "LargeDependenceLowGrayLevelEmphasis"
                | "LongRunLowGrayLevelEmphasis"
                | "LargeAreaLowGrayLevelEmphasis"
                | "LargeDistanceLowGrayLevelEmphasis" => emphasis(-2, 2),
                "LargeDependenceHighGrayLevelEmphasis"
                | "LongRunHighGrayLevelEmphasis"
                | "LargeAreaHighGrayLevelEmphasis"
                | "LargeDistanceHighGrayLevelEmphasis" => emphasis(2, 2),
                "GrayLevelNonUniformity" => nonuniformity(&gray),
                "GrayLevelNonUniformityNormalized" => nonuniformity(&gray) / total,
                "DependenceNonUniformity"
                | "RunLengthNonUniformity"
                | "SizeZoneNonUniformity"
                | "ZoneDistanceNonUniformity" => nonuniformity(&size),
                "DependenceNonUniformityNormalized"
                | "RunLengthNonUniformityNormalized"
                | "SizeZoneNonUniformityNormalized"
                | "ZoneDistanceNonUniformityNormalized" => nonuniformity(&size) / total,
                "GrayLevelVariance" => variance(&gray, n),
                "DependenceVariance" | "RunVariance" | "ZoneVariance" | "ZoneDistanceVariance" => {
                    variance(&size, n)
                }
                "DependenceEntropy" | "RunEntropy" | "ZoneEntropy" | "ZoneDistanceEntropy" => {
                    entropy(matrix.counts.values(), n)
                }
                "DependenceCountEnergy" => matrix
                    .counts
                    .values()
                    .map(|&c| (c as f64 / total).powi(2))
                    .sum(),
                "DependencePercentage" | "RunPercentage" | "ZonePercentage" => {
                    total / matrix.voxels as f64
                }
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "unknown size statistic: {name}"
                    )));
                }
            })
        })
        .collect()
}

struct ToneLevel {
    level: usize,
    count: u64,
    difference_sum: f64,
}

/// Each NGTDM pair term is symmetric and vanishes on the diagonal.
fn pair_sum(levels: &[ToneLevel], term: impl Fn(&ToneLevel, &ToneLevel, f64) -> f64) -> f64 {
    let mut sum = 0.;
    for (k, left) in levels.iter().enumerate() {
        for right in &levels[k + 1..] {
            let distance = left.level.abs_diff(right.level) as f64;
            sum += term(left, right, distance);
        }
    }
    2. * sum
}

pub(super) fn ngtdm_statistics(
    matrix: &TextureMatrix,
    names: &[&str],
    singular: NgtdmSingularCases,
) -> Result<Vec<f64>> {
    let total = matrix.voxels as f64;
    let levels: Vec<_> = matrix
        .counts
        .iter()
        .map(|(&(level, _), &count)| ToneLevel {
            level,
            count,
            difference_sum: matrix.differences[&level],
        })
        .collect();
    let present = levels.len() as f64;
    let weighted = levels
        .iter()
        .map(|v| v.count as f64 * v.difference_sum)
        .sum::<f64>();
    let sum_s = matrix.differences.values().sum::<f64>();
    let caps = singular == NgtdmSingularCases::IbsiCaps;
    names
        .iter()
        .map(|&name| {
            Ok(match name {
                "Coarseness" => {
                    let coarseness = total / weighted;
                    if caps {
                        coarseness.min(1e6)
                    } else {
                        coarseness
                    }
                }
                "Contrast" if levels.len() == 1 => 0.,
                "Contrast" => {
                    let contrast = pair_sum(&levels, |a, b, distance| {
                        (a.count as f64 / total) * (b.count as f64 / total) * distance.powi(2)
                    });
                    contrast * sum_s / (total * present * (present - 1.))
                }
                "Busyness" if caps && levels.len() == 1 => 0.,
                "Busyness" => {
                    let products = |v: &ToneLevel| v.level as i128 * v.count as i128;
                    let denominator =
                        pair_sum(&levels, |a, b, _| (products(a) - products(b)).abs() as f64);
                    weighted / denominator
                }
                "Complexity" => {
                    pair_sum(&levels, |a, b, distance| {
                        distance
                            * (a.count as f64 * a.difference_sum
                                + b.count as f64 * b.difference_sum)
                            / (a.count + b.count) as f64
                    }) / total
                }
                "Strength" if caps && sum_s == 0. => 0.,
                "Strength" => {
                    pair_sum(&levels, |a, b, distance| {
                        (a.count + b.count) as f64 / total * distance.powi(2)
                    }) / sum_s
                }
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "unknown NGTDM statistics: {name}"
                    )));
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod w4_count_tests {
    use crate::features::distributions::*;
    use crate::numerics::sparse_counts::Counts;

    const RELATIVE_TOLERANCE: f64 = 2e-13;

    #[test]
    fn variance_keeps_original_integer_coordinates() {
        let size = 1 << 25;
        let matrix = TextureMatrix {
            counts: Counts::from([
                ((25000, size), 2),
                ((25001, size + 1), 1),
                ((25001, size), 3),
            ]),
            voxels: (6 * size + 1) as u64,
            ..Default::default()
        };
        for name in [
            "ZoneDistanceVariance",
            "RunVariance",
            "DependenceVariance",
            "ZoneVariance",
        ] {
            let values = size_statistics(&matrix, &["GrayLevelVariance", name]).unwrap();
            for (value, target) in values.into_iter().zip([2. / 9., 5. / 36.]) {
                assert!((value - target).abs() <= RELATIVE_TOLERANCE * target);
            }
        }
    }

    #[test]
    fn busyness_preserves_one_count_difference() {
        let matrix = TextureMatrix {
            counts: Counts::from([((1, 1), (1 << 25) + 1), ((2, 1), 1 << 24)]),
            differences: [(1, 0.5), (2, 0.5)].into(),
            voxels: 3 * (1 << 24) + 1,
        };
        assert_eq!(
            ngtdm_statistics(&matrix, &["Busyness"], NgtdmSingularCases::Unadjusted).unwrap()[0],
            matrix.voxels as f64 / 4.
        );
    }
}
