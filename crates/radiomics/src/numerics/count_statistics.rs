//! Integer-count Shannon entropy on the calling CPU thread.
//!
//! Mirrors `mlxradiomics/numerics/count_statistics.py`.

pub(crate) fn entropy_term(count: u64, total: u128) -> f64 {
    if count == 0 || u128::from(count) == total {
        return 0.0;
    }
    let probability = count as f64 / total as f64;
    if u128::from(count) > total / 2 {
        // A probability near one may round to exactly one. Form its deficit in integers.
        let deficit = (total - u128::from(count)) as f64 / total as f64;
        -probability * (-deficit).ln_1p() * std::f64::consts::LOG2_E
    } else {
        -probability * probability.log2()
    }
}

use std::sync::LazyLock;

// A radius-one 3D window has at most 27 samples. Every center with the same
// count and total uses the same entropy contribution, independent of gray level.
const SMALL_WINDOW: usize = 27;
static WINDOW_ENTROPY: LazyLock<[[f64; SMALL_WINDOW + 1]; SMALL_WINDOW + 1]> =
    LazyLock::new(|| {
        let mut terms = [[0.; SMALL_WINDOW + 1]; SMALL_WINDOW + 1];
        for (total, row) in terms.iter_mut().enumerate() {
            for (count, term) in row[..=total].iter_mut().enumerate() {
                *term = entropy_term(count as u64, total as u128);
            }
        }
        terms
    });

pub(crate) fn bin_entropy(count: usize, total: usize) -> f64 {
    if total <= SMALL_WINDOW {
        WINDOW_ENTROPY[total][count]
    } else {
        entropy_term(count as u64, total as u128)
    }
}
