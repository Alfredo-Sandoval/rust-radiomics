//! CPU arithmetic for float32 values with a separate low-order residual.
//!
//! Mirrors `mlxradiomics/numerics/pair_arithmetic.py`. Native extraction keeps
//! float64 source samples. These operations cover consumers of split source words.

/// A leading float32 value and a trailing correction.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FloatPair {
    /// High word.
    pub high: f32,
    /// Low word.
    pub low: f32,
}

impl FloatPair {
    /// Split a binary64 source using the nearest representable high and residual.
    pub fn from_source(value: f64) -> Self {
        let high = value as f32;
        Self {
            high,
            low: (value - f64::from(high)) as f32,
        }
    }

    /// Fold the two words into a float32 result.
    pub fn folded(self) -> f32 {
        self.high + self.low
    }

    pub(crate) fn plus(self, other: Self) -> Self {
        let sum = self.high + other.high;
        let back = sum - self.high;
        let error = ((self.high - (sum - back)) + (other.high - back)) + self.low + other.low;
        let high = sum + error;
        Self {
            high,
            low: error - (high - sum),
        }
    }

    pub(crate) fn negated(self) -> Self {
        Self {
            high: -self.high,
            low: -self.low,
        }
    }

    pub(crate) fn times(self, other: Self) -> Self {
        let value = self.high * other.high;
        let error =
            self.high.mul_add(other.high, -value) + self.high * other.low + self.low * other.high;
        let high = value + error;
        Self {
            high,
            low: error - (high - value),
        }
    }

    pub(crate) fn scaled(self, scale: RowScale) -> Self {
        match scale {
            RowScale::Multiply(k) => Self {
                high: self.high * k,
                low: self.low * k,
            },
        }
    }
}

/// Scale each word before subtracting a row center.
#[derive(Clone, Copy, Debug)]
pub enum RowScale {
    /// Multiply by the supplied factor.
    Multiply(f32),
}

/// Subtract two pairs using error-free high-word subtraction and low-word correction.
/// A finite high-word subtraction that overflows is evaluated at half scale.
pub fn subtract_pairs(left: FloatPair, right: FloatPair) -> FloatPair {
    let span = left.high - right.high;
    if span.is_infinite() && left.high.is_finite() && right.high.is_finite() {
        let half = subtract_pairs(
            left.scaled(RowScale::Multiply(0.5)),
            right.scaled(RowScale::Multiply(0.5)),
        );
        let high = half.high * 2.;
        return FloatPair {
            high,
            low: if high.is_infinite() {
                0.
            } else {
                half.low * 2.
            },
        };
    }
    let back = span - left.high;
    let residual = ((left.high - (span - back)) + (-right.high - back)) + (left.low - right.low);
    let high = span + residual;
    FloatPair {
        high,
        low: residual - (high - span),
    }
}
