//! IBSI intensity-histogram features from 1-based gray-level bins.
//!
//! Mirrors `mlxradiomics/features/intensity_histogram.py` (v1.1.0, b58783c).
//! Status: all 23 ROI features ported. Single-threaded CPU numerical work.
//! The ROI selects samples from the prepared lattice. Empty gray levels count
//! toward the domain of the histogram gradient.

use std::borrow::Cow;
use std::cell::LazyCell;

use ndarray::ArrayD;

use crate::config::settings::Settings;
use crate::contracts::types::{Features, Prepared};
use crate::error::{Error, Result};

/// The values of an array in C order: a borrowed slice when it is contiguous, else a copy.
pub(crate) fn contiguous<T: Clone>(array: &ArrayD<T>) -> Cow<'_, [T]> {
    array.as_slice().map_or_else(
        || Cow::Owned(array.iter().cloned().collect()),
        Cow::Borrowed,
    )
}

fn integer_histogram(bins: &ArrayD<i32>, valid: &ArrayD<bool>, ng: u32) -> Result<Vec<usize>> {
    if ng == 0 {
        return Err(Error::InvalidInput("ng must be a positive integer".into()));
    }
    if bins.shape() != valid.shape() {
        return Err(Error::InvalidInput(
            "bins and ROI must share a shape".into(),
        ));
    }
    let mut counts = vec![0; ng as usize];
    let (bins, valid) = (contiguous(bins), contiguous(valid));
    for (&level, &inside) in bins.iter().zip(valid.iter()) {
        if inside {
            if level < 1 || level as u32 > ng {
                return Err(Error::InvalidInput(
                    "ROI bin outside the gray-level lattice".into(),
                ));
            }
            counts[level as usize - 1] += 1;
        }
    }
    Ok(counts)
}

struct Bin {
    level: i64,
    count: usize,
    cumulative: usize,
}

fn linear_percentile(bins: &[Bin], total: usize, numerator: u128, denominator: u128) -> f64 {
    let position = (total - 1) as u128 * numerator;
    let rank = (position / denominator) as usize;
    let remainder = (position % denominator) as i64;
    let lower = bins.partition_point(|bin| bin.cumulative <= rank);
    let left = bins[lower].level;
    if remainder == 0 || rank + 1 < bins[lower].cumulative {
        return left as f64;
    }
    let step = (bins[lower + 1].level - left) * remainder;
    left as f64 + step as f64 / denominator as f64
}

// Subtract the exact rational mean before converting to f64. This retains the
// residual of a rare neighboring level even far from the lattice origin.
fn centered(level: i64, total: usize, weighted: i128) -> f64 {
    (level as i128 * total as i128 - weighted) as f64 / total as f64
}

fn weighted_sum(bins: &[Bin]) -> i128 {
    bins.iter().map(|b| b.level as i128 * b.count as i128).sum()
}

struct Moments {
    variance: f64,
    skewness: f64,
    kurtosis: f64,
    mean_ad: f64,
}

fn moments(bins: &[Bin], total: usize, weighted: i128) -> Moments {
    let (mut second, mut third, mut fourth, mut absolute) = (0., 0., 0., 0.);
    for bin in bins {
        let d = centered(bin.level, total, weighted);
        let square = d * d;
        let probability = bin.count as f64 / total as f64;
        second += probability * square;
        third += probability * square * d;
        fourth += probability * square * square;
        absolute += probability * d.abs();
    }
    Moments {
        variance: second,
        skewness: if second == 0. {
            0.
        } else {
            third / (second * second.sqrt())
        },
        kurtosis: if second == 0. {
            0.
        } else {
            fourth / (second * second) - 3.
        },
        mean_ad: absolute,
    }
}

fn robust_mean_absolute_deviation(bins: &[Bin], p10: f64, p90: f64) -> f64 {
    let first = bins.partition_point(|bin| (bin.level as f64) < p10);
    let end = bins.partition_point(|bin| bin.level as f64 <= p90);
    let retained = &bins[first..end];
    let total = retained.iter().map(|bin| bin.count).sum();
    if total == 0 {
        return f64::NAN;
    }
    let weighted = weighted_sum(retained);
    retained
        .iter()
        .map(|bin| bin.count as f64 * centered(bin.level, total, weighted).abs())
        .sum::<f64>()
        / total as f64
}

struct Gradients {
    maximum: f64,
    maximum_level: usize,
    minimum: f64,
    minimum_level: usize,
}

fn histogram_gradients(counts: &[usize]) -> Gradients {
    // Keep twice the derivative integral until after comparing extrema.
    let last = counts.len() - 1;
    let endpoint = |a: usize, b: usize| 2 * (counts[b] as i128 - counts[a] as i128);
    let first = endpoint(0, 1.min(last));
    let (mut maximum, mut minimum) = ((first, 1), (first, 1));
    for i in 1..=last {
        let gradient = if i == last {
            endpoint(i - 1, i)
        } else {
            counts[i + 1] as i128 - counts[i - 1] as i128
        };
        if gradient > maximum.0 {
            maximum = (gradient, i + 1);
        }
        if gradient < minimum.0 {
            minimum = (gradient, i + 1);
        }
    }
    Gradients {
        maximum: maximum.0 as f64 * 0.5,
        maximum_level: maximum.1,
        minimum: minimum.0 as f64 * 0.5,
        minimum_level: minimum.1,
    }
}

fn histogram_statistics(counts: &[usize], names: &[&str]) -> Result<Features> {
    let mut total = 0;
    let mut bins = Vec::new();
    for (index, &count) in counts.iter().enumerate() {
        if count > 0 {
            total += count;
            bins.push(Bin {
                level: index as i64 + 1,
                count,
                cumulative: total,
            });
        }
    }
    let (Some(first), Some(last)) = (bins.first(), bins.last()) else {
        return Err(Error::InvalidInput(
            "selected label is absent from mask".into(),
        ));
    };
    let weighted = weighted_sum(&bins);
    let mean = weighted as f64 / total as f64;
    let moments = LazyCell::new(|| moments(&bins, total, weighted));
    let gradients = LazyCell::new(|| histogram_gradients(counts));
    let percentile = |p, q| linear_percentile(&bins, total, p, q);
    let (p10, q1, median, q3, p90) = (
        percentile(1, 10),
        percentile(1, 4),
        percentile(1, 2),
        percentile(3, 4),
        percentile(9, 10),
    );

    names
        .iter()
        .map(|&key| {
            let name = key.strip_prefix("intensityhistogram_");
            let value = match name {
                Some("Mean") => mean,
                Some("Variance") => moments.variance,
                Some("Skewness") => moments.skewness,
                Some("Kurtosis") => moments.kurtosis,
                Some("Minimum") => first.level as f64,
                Some("Maximum") => last.level as f64,
                Some("Range") => (last.level - first.level) as f64,
                Some("Median") => median,
                Some("10Percentile") => p10,
                Some("90Percentile") => p90,
                Some("InterquartileRange") => q3 - q1,
                Some("QuartileCoefficientOfDispersion") => (q3 - q1) / (q3 + q1),
                Some("MeanAbsoluteDeviation") => moments.mean_ad,
                Some("RobustMeanAbsoluteDeviation") => {
                    robust_mean_absolute_deviation(&bins, p10, p90)
                }
                Some("MedianAbsoluteDeviation") => {
                    bins.iter()
                        .map(|b| b.count as f64 * (b.level as f64 - median).abs())
                        .sum::<f64>()
                        / total as f64
                }
                Some("CoefficientOfVariation") => moments.variance.sqrt() / mean,
                Some("Mode") => {
                    bins.iter()
                        .min_by_key(|b| {
                            (
                                std::cmp::Reverse(b.count),
                                (b.level as i128 * total as i128 - weighted).abs(),
                                b.level,
                            )
                        })
                        .unwrap()
                        .level as f64
                }
                Some("Entropy") => bins
                    .iter()
                    .map(|b| {
                        let p = b.count as f64 / total as f64;
                        // log1p retains the entropy of a dominant level as p approaches 1.
                        if b.count > total / 2 {
                            -p * (-((total - b.count) as f64) / total as f64).ln_1p()
                                / std::f64::consts::LN_2
                        } else {
                            -p * p.log2()
                        }
                    })
                    .sum(),
                Some("Uniformity") => {
                    let squares: u128 = bins.iter().map(|b| (b.count as u128).pow(2)).sum();
                    squares as f64 / (total as u128).pow(2) as f64
                }
                Some("MaximumGradient") => gradients.maximum,
                Some("MaximumGradientGrayLevel") => gradients.maximum_level as f64,
                Some("MinimumGradient") => gradients.minimum,
                Some("MinimumGradientGrayLevel") => gradients.minimum_level as f64,
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "unknown intensity-histogram statistics: {}",
                        name.unwrap_or(key)
                    )));
                }
            };
            Ok((key.to_owned(), value))
        })
        .collect()
}

/// Compute the requested full `intensityhistogram_*` keys in caller order.
pub fn compute(
    prepared: &Prepared,
    _spacing: &[f64],
    _settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    if names.is_empty() {
        return Ok(Vec::new());
    }
    let bins = prepared
        .bins
        .as_ref()
        .ok_or_else(|| Error::InvalidInput("intensity histogram requires prepared bins".into()))?;
    let counts = integer_histogram(bins, &prepared.input.roi, prepared.gray_levels)?;
    histogram_statistics(&counts, names)
}
