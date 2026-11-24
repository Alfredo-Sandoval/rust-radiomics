//! Source precision and linear quantile ranks for first-order statistics.
//!
//! Mirrors `mlxradiomics/numerics/precision.py`. All operations run on the CPU.

use crate::numerics::pair_arithmetic::FloatPair;

fn visible(value: f32) -> f64 {
    if value.is_subnormal() {
        0.
    } else {
        f64::from(value)
    }
}

/// Upper bound on the absolute error of the two float32 words after subnormal flushing.
pub fn pair_representation_error(value: f64) -> f64 {
    let FloatPair { high, low } = FloatPair::from_source(value);
    if f64::from(high) == value && !high.is_subnormal() {
        return 0.;
    }
    let gap = ((value - visible(high)) - visible(low)).abs();
    let rounded = gap as f32;
    if f64::from(rounded) < gap {
        f64::from(f32::from_bits(rounded.to_bits() + 1))
    } else {
        f64::from(rounded)
    }
}

/// Exact order-statistic indices and interpolation weight for an integer percentile.
#[derive(Clone, Copy)]
pub(crate) struct QuantileRank {
    lower: usize,
    remainder: u8,
}

impl QuantileRank {
    pub(crate) fn new(len: usize, percent: u8) -> Self {
        debug_assert!(len > 0 && percent <= 100);
        // Divide first to keep (len - 1) * percent exact even at usize::MAX.
        let last = len - 1;
        let percent = usize::from(percent);
        let tail = (last % 100) * percent;
        Self {
            lower: (last / 100) * percent + tail / 100,
            remainder: (tail % 100) as u8,
        }
    }
    pub(crate) fn lower(self) -> usize {
        self.lower
    }
    pub(crate) fn upper(self) -> usize {
        self.lower + usize::from(self.remainder != 0)
    }
    /// Use the split-word interpolation of the reference reporting format.
    pub(crate) fn reported_pair(self, sorted: &[f64]) -> FloatPair {
        let a = FloatPair::from_source(sorted[self.lower()]);
        let b = FloatPair::from_source(sorted[(self.lower() + 1).min(sorted.len() - 1)]);
        let weight = FloatPair::from_source(self.remainder as f64 / 100.);
        if a.high.is_sign_negative() != b.high.is_sign_negative() {
            let one = FloatPair { high: 1., low: 0. };
            a.times(one.plus(weight.negated())).plus(b.times(weight))
        } else {
            a.plus(b.plus(a.negated()).times(weight))
        }
    }

    pub(crate) fn interpolate<T: Copy>(self, sorted: &[T]) -> f64
    where
        f64: From<T>,
    {
        let lo = f64::from(sorted[self.lower()]);
        if self.remainder == 0 {
            return lo;
        }
        let hi = f64::from(sorted[self.upper()]);
        let left = (100 - self.remainder) as f64 / 100.;
        let right = self.remainder as f64 / 100.;
        if lo.is_sign_negative() != hi.is_sign_negative() {
            lo.mul_add(left, hi * right)
        } else if self.remainder < 50 {
            lo + (hi - lo) * right
        } else {
            hi - (hi - lo) * left
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::numerics::precision::*;

    #[test]
    fn ranks_keep_exact_endpoints_and_handle_extreme_opposite_signs() {
        for len in [1, 27, 100, 101, usize::MAX] {
            for p in 0..=100 {
                let rank = QuantileRank::new(len, p);
                let exact = (len - 1) as u128 * u128::from(p);
                assert_eq!(
                    (rank.lower, rank.remainder),
                    ((exact / 100) as usize, (exact % 100) as u8)
                );
            }
        }
        let sorted = [-f64::MAX, f64::MAX];
        assert_eq!(QuantileRank::new(2, 50).interpolate(&sorted), 0.);
        let rank = QuantileRank::new(11, 90);
        assert_eq!((rank.lower(), rank.upper()), (9, 9));
        assert_eq!(QuantileRank::new(2, 0).interpolate(&[4., 5.]), 4.);
    }

    #[test]
    fn pair_error_is_zero_for_exact_offsets_and_positive_for_lost_source_bits() {
        assert_eq!(pair_representation_error(1e8 + 1.), 0.);
        assert!(pair_representation_error(0.1) > 0.);
        assert!(pair_representation_error(f64::from(f32::from_bits(1))) > 0.);
    }
}
