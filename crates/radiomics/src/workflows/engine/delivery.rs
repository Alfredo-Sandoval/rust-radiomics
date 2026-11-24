//! Store evaluated voxel columns in source-grid order and count their public float32 values.

use crate::contracts::validity::{FeatureValidity, accumulate_validity};
use crate::workflows::engine::FeatureValue;
use ndarray::Array2;
use std::ops::Range;

/// Undefined filter responses are NaN for every feature at that centre.
pub(super) fn masked_samples<'a>(
    column: &'a [f64],
    invalid_rows: &[usize],
    scratch: &'a mut Vec<f64>,
) -> &'a [f64] {
    if invalid_rows.is_empty() {
        return column;
    }
    scratch.clear();
    scratch.extend_from_slice(column);
    for &row in invalid_rows {
        scratch[row] = f64::NAN;
    }
    scratch
}

/// Narrow to the public float32 sample before widening into float64 maps or
/// counting validity. Overflow and underflow therefore match streamed output.
pub(super) fn write_samples<T: From<f32> + Into<f64> + Copy>(
    dest: &mut [T],
    samples: &[f64],
    counts: &mut FeatureValidity,
) {
    let mut zeros = 0;
    let mut finite = 0;
    for (slot, &value) in dest.iter_mut().zip(samples) {
        let sample = value as f32;
        *slot = T::from(sample);
        zeros += usize::from(sample == 0.0);
        finite += usize::from(sample.is_finite());
    }
    if finite == samples.len() {
        counts.finite += finite;
        counts.valid_zero += zeros;
    } else {
        accumulate_validity(counts, dest.iter());
    }
}

/// W7 emits centres in C order. Consecutive source voxels can share one slice
/// assignment, even when the calculation used a cropped image.
pub(super) fn source_runs(
    runs: &mut Vec<Range<usize>>,
    centers: &Array2<i32>,
    shape: &[usize],
    origin_offset: usize,
) {
    runs.clear();
    for row in centers.rows() {
        let offset = origin_offset
            + row
                .iter()
                .zip(shape)
                .fold(0, |i, (&x, &size)| i * size + x as usize);
        if let Some(last) = runs.last_mut()
            && last.end == offset
        {
            last.end += 1;
        } else {
            runs.push(offset..offset + 1);
        }
    }
}

pub(super) fn store_column(
    map: &mut FeatureValue,
    runs: &[Range<usize>],
    samples: &[f64],
    counts: &mut FeatureValidity,
) {
    fn write_runs<T: From<f32> + Into<f64> + Copy>(
        grid: &mut [T],
        runs: &[Range<usize>],
        mut samples: &[f64],
        counts: &mut FeatureValidity,
    ) {
        for run in runs {
            let (batch, rest) = samples.split_at(run.len());
            write_samples(&mut grid[run.clone()], batch, counts);
            samples = rest;
        }
    }
    match map {
        FeatureValue::Map32(map) => write_runs(
            map.as_slice_mut().expect("C-order retained map"),
            runs,
            samples,
            counts,
        ),
        FeatureValue::Map64(map) => write_runs(
            map.as_slice_mut().expect("C-order retained map"),
            runs,
            samples,
            counts,
        ),
        FeatureValue::Scalar(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::contracts::validity::{accumulate_validity, empty_validity};
    use crate::workflows::engine::delivery::write_samples;

    #[test]
    fn stored_values_and_counts_use_float32_rounding() {
        let batches: &[&[f64]] = &[
            &[0.0, -0.0, f64::MIN_POSITIVE, -f64::MIN_POSITIVE, 1.0],
            &[f64::MAX, -f64::MAX, f64::from_bits(0x7ff8_1234_5678_9abc)],
        ];
        let expected: Vec<_> = batches
            .iter()
            .flat_map(|&batch| batch.iter().map(|&v| v as f32))
            .collect();
        let mut counts32 = empty_validity(3);
        let mut counts64 = counts32;
        let mut start = 0;
        for &batch in batches {
            let mut values32 = vec![1.0f32; batch.len()];
            let mut values64 = vec![1.0f64; batch.len()];
            write_samples(&mut values32, batch, &mut counts32);
            write_samples(&mut values64, batch, &mut counts64);
            for ((&a, &b), &sample) in values32.iter().zip(&values64).zip(&expected[start..]) {
                assert_eq!(a.to_bits(), sample.to_bits());
                assert_eq!(b.to_bits(), f64::from(sample).to_bits());
            }
            start += batch.len();
        }
        let mut reference = empty_validity(3);
        accumulate_validity(&mut reference, &expected);
        assert_eq!(counts32, reference);
        assert_eq!(counts64, counts32);
    }
}
