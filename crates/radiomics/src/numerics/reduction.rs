//! Deterministic CPU reductions, mirroring `mlxradiomics/numerics/reduction.py`.
//! Each row is evaluated on the calling thread, independently of batch size.

/// Neumaier summation preserves small intensities between large signed values.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompensatedSum {
    sum: f64,
    correction: f64,
}

impl CompensatedSum {
    /// Accumulate one value and retain its first-order rounding error.
    #[inline]
    pub fn add(&mut self, value: f64) {
        let next = self.sum + value;
        self.correction += if self.sum.abs() >= value.abs() {
            (self.sum - next) + value
        } else {
            (value - next) + self.sum
        };
        self.sum = next;
    }

    /// The corrected sum, preserving IEEE nonfinite totals.
    #[inline]
    pub fn total(self) -> f64 {
        if self.sum.is_finite() {
            self.sum + self.correction
        } else {
            self.sum
        }
    }
}

/// Float32-compatible mean of float64 source samples. A compensated adjacent-pair
/// tree over the high words followed by the residual words matches the reference
/// reduction order, including cancellation between the two parts.
pub(crate) fn reported_mean(samples: &[f64], work: &mut Vec<f32>) -> f32 {
    use crate::numerics::pair_arithmetic::FloatPair;
    let count = samples.len() as f32;
    let amplitude = samples.iter().map(|v| v.abs()).fold(0., f64::max);
    if amplitude <= (1u32 << 24) as f64
        && amplitude * (samples.len() as f64) <= (1u64 << 40) as f64
        && samples.iter().all(|v| v.fract() == 0.)
    {
        return samples.iter().map(|&v| v as i64).sum::<i64>() as f32 / count;
    }
    let scale = if amplitude as f32 > f32::MAX / count {
        amplitude as f32
    } else {
        1.
    };
    work.clear();
    work.extend(samples.iter().map(|&v| (v as f32) / scale));
    work.extend(
        samples
            .iter()
            .map(|&v| FloatPair::from_source(v).low / scale),
    );
    let mut width = work.len();
    let mut correction = 0_f32;
    while width > 1 {
        let mut level_error = 0.;
        for i in 0..width / 2 {
            let left = work[2 * i];
            let right = work[2 * i + 1];
            let sum = left + right;
            let back = sum - left;
            level_error += (left - (sum - back)) + (right - back);
            work[i] = sum;
        }
        if width % 2 == 1 {
            work[width / 2] = work[width - 1];
        }
        correction += level_error;
        width = width.div_ceil(2);
    }
    ((work[0] + correction) / count) * scale
}
