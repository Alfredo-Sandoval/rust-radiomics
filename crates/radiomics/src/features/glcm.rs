//! Co-occurrence features, mirroring `mlxradiomics/features/glcm.py`.
//! Status: ported. Single-threaded CPU numerical work on occupied integer cells.

use crate::numerics::eigen::maximal_correlation;
use crate::numerics::sparse_counts::{
    Counts, Marginal, centered, entropy, marginals, variance, weighted_sum,
};
use crate::{Error, Result};

/// Integer central third moment of the sum distribution. If the mean is m + f/n
/// and A_r = sum c*(k-m)^r, the numerator over n^3 is A_3*n² - 3*f*n*A_2 + 2*f³.
/// Divide out integral parts before multiplying: A_3 fits i128 under the GLCM
/// count bound, while the unreduced common numerator can need 190 bits.
fn cluster_shade(sums: &Marginal, total: u64) -> f64 {
    let n = total as i128;
    let s = weighted_sum(sums);
    let (m, f) = (s / n, s % n);
    let (mut a2, mut a3) = (0i128, 0i128);
    for (k, c) in sums.iter() {
        let offset = k as i128 - m;
        let square = c as i128 * offset * offset;
        a2 += square;
        a3 += square * offset;
    }
    let r = a3 % n - 3 * f * (a2 / n);
    let mut whole = a3 / n + r / n;
    let cube = n * n * n;
    let remainder = (r % n) * n * n - 3 * f * n * (a2 % n) + 2 * f * f * f;
    whole += remainder / cube;
    let mut fraction = remainder % cube;
    // Make both terms the same sign so their conversion cannot cancel.
    if whole > 0 && fraction < 0 {
        whole -= 1;
        fraction += cube;
    }
    if whole < 0 && fraction > 0 {
        whole += 1;
        fraction -= cube;
    }
    whole as f64 + fraction as f64 / cube as f64
}

/// KL divergence in bits. Removing the zero-sum linear residual gives each
/// cell a nonnegative contribution, including unoccupied cells of the product support.
fn mutual_information_bits(counts: &Counts, rows: &Marginal, cols: &Marginal, n: u64) -> f64 {
    let square = n as i128 * n as i128;
    let mut missing = square;
    let mut sum = 0.;
    // n < 2**31, so the products of counts below are under 2**62 and fit 64 bits.
    for (&(i, j), &c) in counts {
        let expected = (rows.get(i).unwrap() * cols.get(j).unwrap()) as i64;
        missing -= i128::from(expected);
        let observed = (c * n) as i64;
        let r = (observed - expected) as f64 / expected as f64;
        let divergence = if r.abs() < 0.01 {
            // (1+r)*ln(1+r)-r = r²/2 - r³/6 + r⁴/12 - ...
            let mut poly = 0.;
            for k in (2..=12).rev() {
                poly = 1. / f64::from(k * (k - 1)) - r * poly;
            }
            r * r * poly
        } else {
            let q = observed as f64 / expected as f64;
            let log_ratio = if q < 0.5 { q.ln() } else { r.ln_1p() };
            q * log_ratio - r
        };
        sum += expected as f64 * divergence;
    }
    (sum + missing as f64) / (square as f64 * std::f64::consts::LN_2)
}

pub(super) fn glcm_statistics(
    counts: &Counts,
    ng: u32,
    names: &[&str],
    scratch_mib: u32,
) -> Result<Vec<f64>> {
    let n: u64 = counts.values().sum();
    if n >= 1 << 31 {
        return Err(Error::InvalidInput(
            "merged GLCM count must be below 2**31".into(),
        ));
    }
    let (rows, cols) = marginals(counts);
    let (sx, sy) = (weighted_sum(&rows), weighted_sum(&cols));
    // The largest row and column are the last row key and the last column key.
    let (top_row, top_column) = (rows.keys()[rows.len() - 1], cols.keys()[cols.len() - 1]);
    let differences = Marginal::accumulate(
        counts.len(),
        top_row.max(top_column),
        counts.iter().map(|(&(i, j), &c)| (i.abs_diff(j), c)),
    );
    let sums = Marginal::accumulate(
        counts.len(),
        top_row + top_column,
        counts.iter().map(|(&(i, j), &c)| (i + j, c)),
    );
    let total = n as f64;
    let moment = |dist: &Marginal, weight: &dyn Fn(usize) -> f64| {
        dist.iter().map(|(k, c)| c as f64 * weight(k)).sum::<f64>() / total
    };
    let information = std::cell::LazyCell::new(|| mutual_information_bits(counts, &rows, &cols, n));
    names
        .iter()
        .map(|&name| {
            Ok(match name {
                "Autocorrelation" => {
                    counts
                        .iter()
                        .map(|(&(i, j), &c)| c as f64 * i as f64 * j as f64)
                        .sum::<f64>()
                        / total
                }
                "JointAverage" => sx as f64 / total,
                "JointEnergy" => counts.values().map(|&c| (c as f64 / total).powi(2)).sum(),
                "JointEntropy" => entropy(counts.values(), n),
                "MaximumProbability" => *counts.values().max().unwrap() as f64 / total,
                "Contrast" => moment(&differences, &|k| (k as f64).powi(2)),
                "Correlation" => {
                    if rows.len() == 1 || cols.len() == 1 {
                        1.
                    } else {
                        let covariance = counts
                            .iter()
                            .map(|(&(i, j), &c)| c as f64 * centered(i, sx, n) * centered(j, sy, n))
                            .sum::<f64>()
                            / total;
                        covariance
                            / ((variance(&rows, n) * variance(&cols, n)).sqrt() + f64::EPSILON)
                    }
                }
                "DifferenceAverage" | "Dissimilarity" => weighted_sum(&differences) as f64 / total,
                "DifferenceEntropy" => entropy(differences.values(), n),
                "DifferenceVariance" => variance(&differences, n),
                "SumAverage" => (sx + sy) as f64 / total,
                "SumEntropy" => entropy(sums.values(), n),
                "SumSquares" => variance(&rows, n),
                "ClusterTendency" | "SumVariance" => variance(&sums, n),
                "ClusterProminence" => moment(&sums, &|k| centered(k, sx + sy, n).powi(4)),
                "ClusterShade" => cluster_shade(&sums, n),
                "Id" => moment(&differences, &|k| 1. / (1. + k as f64)),
                "Idm" => moment(&differences, &|k| 1. / (1. + (k as f64).powi(2))),
                "Idn" => moment(&differences, &|k| 1. / (1. + k as f64 / ng as f64)),
                "Idmn" => moment(&differences, &|k| {
                    1. / (1. + (k as f64 / ng as f64).powi(2))
                }),
                "InverseVariance" => moment(&differences, &|k| {
                    if k == 0 { 0. } else { (k as f64).powi(-2) }
                }),
                "Imc1" => {
                    let h = entropy(rows.values(), n).max(entropy(cols.values(), n));
                    if h == 0. { 0. } else { -*information / h }
                }
                "Imc2" => (-(-2. * *information).exp_m1()).sqrt(),
                "MCC" => {
                    if ng > 512 {
                        // Preserve the explicit occupied-state admission check in sparse_glcm.py.
                        let levels = rows.len()
                            + cols
                                .keys()
                                .iter()
                                .filter(|&&k| rows.get(k).is_none())
                                .count();
                        if (levels as u128).pow(2) * 128 > (scratch_mib as u128) << 20 {
                            return Err(Error::InvalidInput(
                                "MCC occupied-state eigensystem exceeds scratch_mib".into(),
                            ));
                        }
                    }
                    maximal_correlation(counts, &rows, &cols, n)?
                }
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "unknown glcm statistic: {name}"
                    )));
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod w4_count_tests {
    use crate::features::glcm::*;

    const RELATIVE_TOLERANCE: f64 = 2e-13;

    fn close(value: f64, target: f64) {
        assert!(
            (value - target).abs() <= RELATIVE_TOLERANCE * target.abs(),
            "{value:e} vs {target:e}"
        );
    }

    #[test]
    fn rare_perfect_dependence_and_entropy() {
        for n in [1 << 27, 1 << 28, 2_000_000_000] {
            let counts = Counts::from([((1, 1), n), ((25001, 25001), 1)]);
            let values = glcm_statistics(
                &counts,
                25001,
                &[
                    "MCC",
                    "Imc1",
                    "Imc2",
                    "JointEntropy",
                    "SumEntropy",
                    "DifferenceEntropy",
                ],
                64,
            )
            .unwrap();
            let n = n as f64;
            let h = (n * (1. / n).ln_1p() + (n + 1.).ln()) / ((n + 1.) * std::f64::consts::LN_2);
            close(values[0], 1.);
            close(values[1], -1.);
            close(values[2], (-(-2. * h).exp_m1()).sqrt());
            close(values[3], h);
            assert_eq!(values[3], values[4]);
            assert_eq!(values[5], 0.);
        }
    }

    #[test]
    fn exact_independence_and_single_count_perturbation() {
        let c = 1 << 26;
        for extra in [0, 1] {
            let counts = Counts::from([
                ((1, 1), c + extra),
                ((1, 25001), c),
                ((25001, 1), c),
                ((25001, 25001), c + extra),
            ]);
            let x = extra as f64 / (2 * c + extra) as f64;
            // Here x <= 2^-27. The next term x^4/(12 ln 2) is below one f64 ulp of MI.
            let info = x * x / (2. * std::f64::consts::LN_2);
            let values = glcm_statistics(&counts, 25001, &["MCC", "Imc1", "Imc2"], 64).unwrap();
            close(values[0], x);
            close(values[1], -info);
            close(values[2], (-(-2. * info).exp_m1()).sqrt());
        }
    }

    #[test]
    fn cluster_shade_retains_cancellation_at_large_counts_and_indices() {
        let levels = [1usize, 100001, 200001, 250001];
        for extra in [0, 1] {
            let counts: Counts = levels
                .into_iter()
                .zip([2 << 25, 12 << 25, 2 << 25, (8 << 25) + extra])
                .map(|(k, c)| ((k, k), c))
                .collect();
            let value = glcm_statistics(&counts, 250001, &["ClusterShade"], 64).unwrap()[0];
            close(
                value,
                if extra == 0 {
                    0.
                } else {
                    -9_934_107.462_565_104
                },
            );
        }
    }

    #[test]
    fn sparse_difference_variance_and_singleton_marginal() {
        let counts = Counts::from([((1, 100_000_001), 1), ((1, 100_000_002), 1)]);
        let values = glcm_statistics(
            &counts,
            100_000_002,
            &["DifferenceVariance", "Correlation", "MCC"],
            64,
        )
        .unwrap();
        assert_eq!(values[..2], [0.25, 1.]);
        assert!(values[2].is_nan());
    }
}
