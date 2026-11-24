//! Include occupied row spans in the half-open bounds of a 2D or 3D image.

use ndarray::{ArrayD, Dimension};
use rayon::prelude::*;

pub(in crate::workflows) fn include<T: Copy + PartialEq + Sync>(
    array: &ArrayD<T>,
    label: T,
    lower: &mut [usize],
    upper: &mut [usize],
) {
    let shape = array.shape();
    if array.is_empty() {
        return;
    }
    if let Some(values) = array.as_slice() {
        let last = shape.len() - 1;
        let width = shape[last];
        let scan = |start_row: usize, rows: &[T]| {
            let mut lo = [usize::MAX; 3];
            let mut hi = [0; 3];
            for (row, samples) in rows.chunks_exact(width).enumerate() {
                // Slice search can skip background rows a machine word at a time.
                if !samples.contains(&label) {
                    continue;
                }
                if lo[last] != 0 {
                    lo[last] = lo[last].min(samples.iter().position(|&v| v == label).unwrap());
                }
                if hi[last] != width {
                    hi[last] = hi[last].max(1 + samples.iter().rposition(|&v| v == label).unwrap());
                }
                let mut index = start_row + row;
                for axis in (0..last).rev() {
                    let i = index % shape[axis];
                    lo[axis] = lo[axis].min(i);
                    hi[axis] = hi[axis].max(i + 1);
                    index /= shape[axis];
                }
            }
            (lo, hi)
        };
        // A serial row scan is cheaper on small CT phantoms.
        let (lo, hi) = if values.len() >= 8 * 1024 * 1024 {
            let rows_per_chunk = (128 * 1024usize).div_ceil(width);
            values
                .par_chunks(rows_per_chunk * width)
                .enumerate()
                // Most CT mask blocks are entirely background. Search each block
                // once before dispatching individual row searches for its bounds.
                .filter(|(_, rows)| rows.contains(&label))
                .map(|(chunk, rows)| scan(chunk * rows_per_chunk, rows))
                .reduce_with(|(lo, hi), (other_lo, other_hi)| {
                    (
                        std::array::from_fn(|a| lo[a].min(other_lo[a])),
                        std::array::from_fn(|a| hi[a].max(other_hi[a])),
                    )
                })
                .unwrap_or(([usize::MAX; 3], [0; 3]))
        } else {
            scan(0, values)
        };
        for axis in 0..shape.len() {
            lower[axis] = lower[axis].min(lo[axis]);
            upper[axis] = upper[axis].max(hi[axis]);
        }
    } else {
        for (at, &value) in array.indexed_iter() {
            if value == label {
                for (axis, &i) in at.slice().iter().enumerate() {
                    lower[axis] = lower[axis].min(i);
                    upper[axis] = upper[axis].max(i + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workflows::roi::bounds::*;
    use ndarray::{Array3, s};

    #[test]
    fn large_mask_unions_matches_from_separate_chunks() {
        let mut mask = Array3::<u8>::zeros((480, 128, 144));
        mask[[11, 127, 0]] = 9;
        mask[[461, 1, 143]] = 9;
        let mask = mask.into_dyn();
        let mut lower = [7, 100, 52];
        let mut upper = [10, 104, 55];
        include(&mask, 8, &mut lower, &mut upper);
        assert_eq!((lower, upper), ([7, 100, 52], [10, 104, 55]));
        include(&mask, 9, &mut lower, &mut upper);
        assert_eq!((lower, upper), ([7, 1, 0], [462, 128, 144]));
    }

    #[test]
    fn row_spans_and_strided_fallback_match_logical_coordinates() {
        let data = Array3::from_shape_fn((5, 7, 13), |(z, y, x)| {
            ((z * 6 + y * 13 + x * 5) % 17) as i32
        });
        for data in [
            data.clone().into_dyn(),
            data.clone().reversed_axes().into_dyn(),
            data.clone().slice_move(s![..;-1,1..,..;2]).into_dyn(),
        ] {
            for number in -1..=18 {
                let mut lower = data.shape().to_vec();
                let mut upper = vec![0; data.ndim()];
                include(&data, number, &mut lower, &mut upper);
                let mut check_lower = data.shape().to_vec();
                let mut check_upper = vec![0; data.ndim()];
                for (at, &v) in data.indexed_iter() {
                    if v == number {
                        for (d, &i) in at.slice().iter().enumerate() {
                            check_lower[d] = check_lower[d].min(i);
                            check_upper[d] = check_upper[d].max(i + 1);
                        }
                    }
                }
                assert_eq!(lower, check_lower);
                assert_eq!(upper, check_upper);
            }
        }
    }
}
