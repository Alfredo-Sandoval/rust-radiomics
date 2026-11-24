//! Inverse-distance pair sums over the voxels of a regular grid.
//!
//! Moran's I and Geary's C need, over every unordered voxel pair with physical distance
//! `r`, the sums of `1/r`, `(1/r) c_i c_j` and `(1/r) (c_i - c_j)^2`, where `c` are the
//! centred intensities. A direct loop evaluates one square root and one division per
//! pair. Voxel centres lie on a lattice, so the pair distance depends only on the integer
//! offset: voxels are grouped into runs of consecutive x positions in one `(z, y)` row, the
//! weights of one run pair are computed once per distinct x offset (`m + n - 1` values for
//! runs of length `m` and `n`) and the `m * n` pair terms are a contiguous
//! multiply-accumulate against that weight vector.
//!
//! Execution: CPU numerical work in float64, rayon over the first run of each run pair.
//! Within one run pair the terms are added with four independent accumulators per sum and
//! the block totals are added with Neumaier compensation in a fixed run order, so the result
//! does not depend on the thread count. Input contract: 3-D integer voxel coordinates in C
//! order (as [`Coordinates::from_mask`] produces) and one centred value per voxel.
//!
//! Status: done.

use std::sync::Arc;

use rayon::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};

use crate::features::geometry::arithmetic::NeumaierSum;
use crate::features::geometry::mesh::Coordinates;

/// Accumulator lanes per sum: four 128-bit float64 registers, which measured fastest.
const LANES: usize = 4;

/// A maximal x-run of consecutive voxels in one `(z, y)` row.
#[derive(Clone, Copy)]
struct Run {
    z: i64,
    y: i64,
    x: i64,
    start: usize,
    len: usize,
}

fn runs_of(coordinates: &Coordinates) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    for (index, row) in coordinates.rows().enumerate() {
        match runs.last_mut() {
            Some(run) if run.z == row[0] && run.y == row[1] && run.x + run.len as i64 == row[2] => {
                run.len += 1;
            }
            _ => runs.push(Run {
                z: row[0],
                y: row[1],
                x: row[2],
                start: index,
                len: 1,
            }),
        }
    }
    runs
}

/// Pairwise sum of the accumulator lanes.
fn lane_sum(lanes: &[f64; LANES]) -> f64 {
    let mut work = *lanes;
    let mut half = LANES / 2;
    while half > 0 {
        for k in 0..half {
            work[k] += work[k + half];
        }
        half /= 2;
    }
    work[0]
}

/// `[sum w, sum w c_i c_j, sum w (c_i - c_j)^2]` of the pairs inside one run, against the
/// reversed weights `weights[t + n - 1 - u]` of the pair `(values[t], values[u])`, `t < u`.
fn triangle(values: &[f64], weights: &[f64]) -> [f64; 3] {
    let n = values.len();
    let (mut sw, mut sm, mut sg) = (0.0f64, 0.0f64, 0.0f64);
    for (u, &cu) in values.iter().enumerate() {
        let w = &weights[n - 1 - u..n - 1];
        for (&wk, &ck) in w.iter().zip(&values[..u]) {
            let d = ck - cu;
            sw += wk;
            sm = (wk * ck).mul_add(cu, sm);
            sg = (wk * d).mul_add(d, sg);
        }
    }
    [sw, sm, sg]
}

/// `[sum w, sum w c_i c_j, sum w (c_i - c_j)^2]` of one `inner` x `outer` block against the
/// reversed weight vector (`weights[t + outer.len() - 1 - u]` for the pair
/// `(inner[t], outer[u])`).
fn block(inner: &[f64], outer: &[f64], weights: &[f64]) -> [f64; 3] {
    let n = outer.len();
    let (mut sw, mut sm, mut sg) = ([0.0f64; LANES], [0.0f64; LANES], [0.0f64; LANES]);
    let (mut tail_w, mut tail_m, mut tail_g) = (0.0f64, 0.0f64, 0.0f64);
    for (u, &cu) in outer.iter().enumerate() {
        let w = &weights[n - 1 - u..n - 1 - u + inner.len()];
        let mut pm = [0.0f64; LANES];
        let mut chunks_w = w.chunks_exact(LANES);
        let mut chunks_c = inner.chunks_exact(LANES);
        for (wk, ck) in (&mut chunks_w).zip(&mut chunks_c) {
            for lane in 0..LANES {
                let d = ck[lane] - cu;
                sw[lane] += wk[lane];
                pm[lane] = wk[lane].mul_add(ck[lane], pm[lane]);
                sg[lane] = (wk[lane] * d).mul_add(d, sg[lane]);
            }
        }
        // The product with the outer value is applied once per outer voxel.
        for lane in 0..LANES {
            sm[lane] = pm[lane].mul_add(cu, sm[lane]);
        }
        let (mut rw, mut rm, mut rg) = (0.0, 0.0, 0.0);
        for (&wk, &ck) in chunks_w.remainder().iter().zip(chunks_c.remainder()) {
            let d = ck - cu;
            rw += wk;
            rm = wk.mul_add(ck, rm);
            rg = (wk * d).mul_add(d, rg);
        }
        tail_w += rw;
        tail_m = rm.mul_add(cu, tail_m);
        tail_g += rg;
    }
    [
        lane_sum(&sw) + tail_w,
        lane_sum(&sm) + tail_m,
        lane_sum(&sg) + tail_g,
    ]
}

/// Unordered-pair sums `[sum 1/r, sum c_i c_j / r, sum (c_i - c_j)^2 / r]` over every pair of
/// distinct voxels, with `r` the physical distance.
///
/// `coordinates` must be 3-D, in C order and without duplicates; `centered` has one value per
/// voxel; `spacing` is per axis in (z, y, x) order.
#[must_use]
pub fn exact_inverse_distance_sums(
    centered: &[f64],
    coordinates: &Coordinates,
    spacing: &[f64],
) -> [f64; 3] {
    assert_eq!(coordinates.ndim(), 3, "pair sums are defined on 3-D grids");
    let runs = runs_of(coordinates);
    let (sz, sy, sx) = (spacing[0], spacing[1], spacing[2]);
    let rows: Vec<[f64; 3]> = (0..runs.len())
        .into_par_iter()
        .map(|a| {
            let first = runs[a];
            let mut weight = NeumaierSum::new();
            let mut moran = NeumaierSum::new();
            let mut geary = NeumaierSum::new();
            let mut add = |parts: [f64; 3]| {
                weight.add(parts[0]);
                moran.add(parts[1]);
                geary.add(parts[2]);
            };
            let mut kernel: Vec<f64> = Vec::new();
            let real = |run: &Run| &centered[run.start..run.start + run.len];
            // Pairs inside the run: offsets 1..len, reversed so a fixed outer voxel reads
            // its weights contiguously.
            if first.len > 1 {
                let n = first.len;
                kernel.clear();
                kernel.extend((0..n - 1).map(|j| 1.0 / (sx * (n - 1 - j) as f64)));
                add(triangle(real(&first), &kernel));
            }
            for second in &runs[a + 1..] {
                let dz = (second.z - first.z) as f64 * sz;
                let dy = (second.y - first.y) as f64 * sy;
                let base = dz * dz + dy * dy;
                // The longer run is the contiguous inner loop; the weights are symmetric.
                let (inner, outer) = if first.len >= second.len {
                    (&first, second)
                } else {
                    (second, &first)
                };
                let (m, n) = (inner.len, outer.len);
                let largest = outer.x + n as i64 - 1 - inner.x;
                kernel.clear();
                kernel.extend((0..m + n - 1).map(|j| {
                    let dx = (largest - j as i64) as f64 * sx;
                    1.0 / (base + dx * dx).sqrt()
                }));
                add(block(real(inner), real(outer), &kernel));
            }
            [weight.value(), moran.value(), geary.value()]
        })
        .collect();
    let mut totals = [NeumaierSum::new(), NeumaierSum::new(), NeumaierSum::new()];
    for row in &rows {
        for (total, value) in totals.iter_mut().zip(row) {
            total.add(*value);
        }
    }
    [totals[0].value(), totals[1].value(), totals[2].value()]
}

/// Smallest 5-smooth integer at least `n`.
fn smooth_size(n: usize) -> usize {
    (n..)
        .find(|&candidate| {
            let mut rest = candidate;
            for factor in [2, 3, 5] {
                while rest % factor == 0 {
                    rest /= factor;
                }
            }
            rest == 1
        })
        .expect("a smooth size exists")
}

/// Transform every contiguous length-`plan.len()` row of `rows` in place, in parallel.
fn fft_rows(rows: &mut [Complex<f64>], plan: &Arc<dyn Fft<f64>>) {
    let len = plan.len();
    rows.par_chunks_mut(len * 16).for_each(|block| {
        let mut scratch = vec![Complex::new(0.0, 0.0); plan.get_inplace_scratch_len()];
        for row in block.chunks_exact_mut(len) {
            plan.process_with_scratch(row, &mut scratch);
        }
    });
}

/// Forward 3-D transform of `data` (`[pz][py][px]`, nonzero only in the leading
/// `extent` corner) into the layout `[ky][kx][kz]`.
fn fft3(data: Vec<Complex<f64>>, padded: [usize; 3], extent: [usize; 3]) -> Vec<Complex<f64>> {
    let [pz, py, px] = padded;
    let mut planner = FftPlanner::<f64>::new();
    let (plan_x, plan_y, plan_z) = (
        planner.plan_fft_forward(px),
        planner.plan_fft_forward(py),
        planner.plan_fft_forward(pz),
    );
    let mut data = data;
    // Along x: only the occupied (z, y) rows are nonzero.
    data.par_chunks_mut(py * px)
        .take(extent[0])
        .for_each(|plane| {
            let mut scratch = vec![Complex::new(0.0, 0.0); plan_x.get_inplace_scratch_len()];
            for row in plane.chunks_exact_mut(px).take(extent[1]) {
                plan_x.process_with_scratch(row, &mut scratch);
            }
        });
    // Along y: columns of each occupied plane, gathered in blocks of x.
    const BLOCK: usize = 8;
    data.par_chunks_mut(py * px)
        .take(extent[0])
        .for_each(|plane| {
            let mut scratch = vec![Complex::new(0.0, 0.0); plan_y.get_inplace_scratch_len()];
            let mut tile = vec![Complex::new(0.0, 0.0); BLOCK * py];
            for x0 in (0..px).step_by(BLOCK) {
                let width = BLOCK.min(px - x0);
                for y in 0..py {
                    for b in 0..width {
                        tile[b * py + y] = plane[y * px + x0 + b];
                    }
                }
                for b in 0..width {
                    plan_y.process_with_scratch(&mut tile[b * py..(b + 1) * py], &mut scratch);
                }
                for y in 0..py {
                    for b in 0..width {
                        plane[y * px + x0 + b] = tile[b * py + y];
                    }
                }
            }
        });
    // Transpose to [y][x][z] so the z transform runs over contiguous rows.
    let mut turned = vec![Complex::new(0.0, 0.0); data.len()];
    turned
        .par_chunks_mut(px * pz)
        .enumerate()
        .for_each(|(y, slab)| {
            for z in 0..pz {
                for x in 0..px {
                    slab[x * pz + z] = data[(z * py + y) * px + x];
                }
            }
        });
    drop(data);
    fft_rows(&mut turned, &plan_z);
    turned
}

/// The three sums of [`exact_inverse_distance_sums`] from FFT correlations.
///
/// With `m` the voxel indicator, `c` the centred values and `K(o) = 1 / |o|` (`K(0) = 0`) on
/// the zero-padded bounding box, the ordered-pair sums are `sum m (K*m)`, `sum c (K*c)` and
/// `2 sum c^2 (K*m) - 2 sum c (K*c)`; by Parseval each is a frequency-domain sum of the
/// transform of `K` against the transforms of `m`, `c` and `c^2`, so no inverse transform is
/// needed. `m + i c` and `c^2 + i K` share one complex transform each. The reduction runs in
/// fixed slabs summed in order, so the result does not depend on the thread count.
#[must_use]
pub fn fft_inverse_distance_sums(
    centered: &[f64],
    coordinates: &Coordinates,
    spacing: &[f64],
) -> [f64; 3] {
    assert_eq!(coordinates.ndim(), 3, "pair sums are defined on 3-D grids");
    let mut low = [i64::MAX; 3];
    let mut high = [i64::MIN; 3];
    for row in coordinates.rows() {
        for axis in 0..3 {
            low[axis] = low[axis].min(row[axis]);
            high[axis] = high[axis].max(row[axis]);
        }
    }
    let extent: [usize; 3] = std::array::from_fn(|a| (high[a] - low[a] + 1) as usize);
    let padded: [usize; 3] = std::array::from_fn(|a| smooth_size(2 * extent[a] - 1));
    let [pz, py, px] = padded;
    let total = pz * py * px;
    let zero = Complex::new(0.0, 0.0);
    let mut first = vec![zero; total];
    let mut second = vec![zero; total];
    for (row, &value) in coordinates.rows().zip(centered) {
        let index = (((row[0] - low[0]) as usize * py) + (row[1] - low[1]) as usize) * px
            + (row[2] - low[2]) as usize;
        first[index] = Complex::new(1.0, value);
        second[index] = Complex::new(value * value, 0.0);
    }
    // The kernel is dense over the padded box; its imaginary slot carries K.
    let wrapped = |index: usize, size: usize, step: f64| {
        let offset = if index <= size / 2 {
            index
        } else {
            size - index
        };
        offset as f64 * step
    };
    second
        .par_chunks_mut(py * px)
        .enumerate()
        .for_each(|(z, plane)| {
            let dz = wrapped(z, pz, spacing[0]);
            for y in 0..py {
                let dy = wrapped(y, py, spacing[1]);
                for x in 0..px {
                    let dx = wrapped(x, px, spacing[2]);
                    let distance = (dz * dz + dy * dy + dx * dx).sqrt();
                    plane[y * px + x].im = if distance > 0.0 { 1.0 / distance } else { 0.0 };
                }
            }
        });
    let extent_first = extent;
    let (first, second) = (
        fft3(first, padded, extent_first),
        fft3(second, padded, [pz, py, px]),
    );
    // Slab `y` holds frequencies (ky = y, kx, kz); the mirror is (-ky, -kx, -kz).
    let partial: Vec<[f64; 3]> = (0..py)
        .into_par_iter()
        .map(|y| {
            let my = (py - y) % py;
            let mut sums = [0.0f64; 3];
            for x in 0..px {
                let mx = (px - x) % px;
                for z in 0..pz {
                    let mz = (pz - z) % pz;
                    let at = (y * px + x) * pz + z;
                    let mirror = (my * px + mx) * pz + mz;
                    let (a, b) = (first[at], first[mirror].conj());
                    let (c, d) = (second[at], second[mirror].conj());
                    let m_hat = (a + b) * 0.5;
                    let c_hat = (a - b) * Complex::new(0.0, -0.5);
                    let q_hat = (c + d) * 0.5;
                    let k_hat = ((c - d) * Complex::new(0.0, -0.5)).re;
                    sums[0] += k_hat * m_hat.norm_sqr();
                    sums[1] += k_hat * c_hat.norm_sqr();
                    sums[2] += k_hat * (m_hat.conj() * q_hat).re;
                }
            }
            sums
        })
        .collect();
    let mut totals = [0.0f64; 3];
    for sums in &partial {
        for k in 0..3 {
            totals[k] += sums[k];
        }
    }
    let scale = 1.0 / total as f64;
    let (weight, moran, mixed) = (totals[0] * scale, totals[1] * scale, totals[2] * scale);
    // Ordered-pair sums halved to unordered, as the exact kernel returns.
    [0.5 * weight, 0.5 * moran, mixed - moran]
}

/// Pair-sum kernel choice: the FFT correlation when its padded box is much cheaper than the
/// `n^2 / 2` pairs of the exact kernel, the exact run kernel otherwise (small or sparse ROIs).
#[must_use]
pub fn inverse_distance_sums(
    centered: &[f64],
    coordinates: &Coordinates,
    spacing: &[f64],
) -> [f64; 3] {
    let mut low = [i64::MAX; 3];
    let mut high = [i64::MIN; 3];
    for row in coordinates.rows() {
        for axis in 0..3 {
            low[axis] = low[axis].min(row[axis]);
            high[axis] = high[axis].max(row[axis]);
        }
    }
    let padded: usize = (0..3)
        .map(|a| smooth_size(2 * (high[a] - low[a] + 1) as usize - 1))
        .product();
    let pairs = centered.len() * centered.len() / 2;
    if padded <= FFT_MAX_POINTS && padded * FFT_COST_PER_POINT < pairs {
        fft_inverse_distance_sums(centered, coordinates, spacing)
    } else {
        exact_inverse_distance_sums(centered, coordinates, spacing)
    }
}

/// Largest padded box (complex points) the FFT kernel may allocate: two arrays of 16 bytes.
const FFT_MAX_POINTS: usize = 1 << 25;
/// Exact-kernel pairs equivalent to one padded FFT point (calibrated by measurement).
const FFT_COST_PER_POINT: usize = 100;

#[cfg(test)]
mod tests {
    use crate::features::geometry::mesh::Coordinates;
    use crate::features::geometry::pairwise::{
        exact_inverse_distance_sums, fft_inverse_distance_sums,
    };
    use ndarray::{ArrayD, IxDyn};

    fn direct(centered: &[f64], coordinates: &Coordinates, spacing: &[f64]) -> [f64; 3] {
        let mut sums = [0.0; 3];
        for i in 0..centered.len() {
            for j in i + 1..centered.len() {
                let r = (0..3)
                    .map(|a| {
                        ((coordinates.row(i)[a] - coordinates.row(j)[a]) as f64 * spacing[a])
                            .powi(2)
                    })
                    .sum::<f64>()
                    .sqrt();
                sums[0] += 1.0 / r;
                sums[1] += centered[i] * centered[j] / r;
                sums[2] += (centered[i] - centered[j]).powi(2) / r;
            }
        }
        sums
    }

    #[test]
    fn run_sums_match_the_direct_pair_loop() {
        let mut mask = ArrayD::from_elem(IxDyn(&[5, 6, 9]), false);
        let mut state = 12345u64;
        for value in mask.iter_mut() {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *value = !(state >> 33).is_multiple_of(3);
        }
        let coordinates = Coordinates::from_mask(&mask);
        let centered: Vec<f64> = (0..coordinates.len())
            .map(|i| ((i * 37 % 101) as f64 - 50.0) * 0.5)
            .collect();
        let spacing = [2.5, 0.7, 0.9];
        let fast = exact_inverse_distance_sums(&centered, &coordinates, &spacing);
        let fft = fft_inverse_distance_sums(&centered, &coordinates, &spacing);
        let slow0 = direct(&centered, &coordinates, &spacing);
        for k in 0..3 {
            assert!(
                (fft[k] - slow0[k]).abs() <= 1e-11 * slow0[k].abs().max(slow0[0]),
                "fft {k}: {fft:?} {slow0:?}"
            );
        }
        let slow = direct(&centered, &coordinates, &spacing);
        for k in 0..3 {
            assert!(
                (fast[k] - slow[k]).abs() <= 1e-11 * slow[k].abs(),
                "{k}: {fast:?} {slow:?}"
            );
        }
    }
}
