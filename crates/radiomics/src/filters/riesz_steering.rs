//! Local Riesz alignment from a regularised structure tensor (IBSI 2 `1SD3`).
//!
//! Mirrors `mlxradiomics/filters/riesz_steering.py` (742 lines at mlx-radiomics b58783c).
//! Status: done. The IBSI 2 manual (arXiv:2006.05470v9, section 4.7.1, Eqs. 4.12-4.15)
//! aligns the order-`L` Riesz bank with the eigenvectors of a Gaussian-regularised
//! structure tensor of the first-order responses. The bank is steerable, so aligned
//! responses are a per-voxel linear combination of the axis-aligned ones. The manual has
//! no consensus maps for aligned Riesz; correctness rests on the algebra of the steering
//! matrix and the closed-form eigendecomposition.
//!
//! Conventions (as in mlx-radiomics): multi-indices follow IBSI axis order `(k1, ..., kD)`
//! in descending lexicographic order; `U` has the analysis directions as rows; eigenvectors
//! are ordered by descending Rayleigh quotient; `u1` and `u2` have their largest-magnitude
//! component positive and `u3 = u1 x u2`; a tensor whose spectral spread is at most
//! `2**-17` of its eigenvalue magnitude (float32 rounding level in mlx-radiomics, kept as the
//! contract constant) returns the identity.
//!
//! Execution: CPU numerical work in float64, single-threaded per voxel loop (the
//! Gaussian window uses the rayon-backed separable convolution of `convolution.rs`).
//! mlx-radiomics' blocked batching is a Metal memory device and is not ported. Input
//! contract: the `D` first-order response maps of one base-filtered 2-D or 3-D image.

use crate::config::filtering::{FilterPadding, MAX_STEERING_ORDER};
use crate::error::{Error, Result};
use crate::filters::convolution::convolve_separable_native;
use ndarray::{ArrayD, IxDyn};
use std::collections::HashMap;

/// A near-isotropic tensor has no meaningful orientation (spread at most `2**-17`).
const ISOTROPIC_TOLERANCE: f64 = 7.629_394_531_25e-6;
const SMALLEST_NORM: f64 = 1e-30;

fn multi_indices_inner(ndim: usize, order: u32) -> Vec<Vec<u32>> {
    if ndim == 1 {
        return vec![vec![order]];
    }
    let mut result = Vec::new();
    for head in (0..=order).rev() {
        for tail in multi_indices_inner(ndim - 1, order - head) {
            let mut item = vec![head];
            item.extend(tail);
            result.push(item);
        }
    }
    result
}

/// Multi-indices of the order-`order` Riesz components in canonical order.
///
/// The enumeration is descending lexicographic over IBSI axis order; for three dimensions
/// and order two it is `(2,0,0), (1,1,0), (1,0,1), (0,2,0), (0,1,1), (0,0,2)`.
///
/// Errors: `ndim` not two or three, or `order` below one.
pub fn riesz_multi_indices(ndim: usize, order: u32) -> Result<Vec<Vec<u32>>> {
    if ndim != 2 && ndim != 3 {
        return Err(Error::InvalidInput(
            "Riesz multi-indices are defined for two or three dimensions".into(),
        ));
    }
    if order < 1 {
        return Err(Error::InvalidInput(
            "Riesz order must be at least one".into(),
        ));
    }
    Ok(multi_indices_inner(ndim, order))
}

/// Index tables that expand the steering polynomials for one bank.
struct SteeringPlan {
    count: usize,
    order: usize,
    ndim: usize,
    /// `shifts[step][axis][target]`: source monomial of the previous layer, if any.
    shifts: Vec<Vec<Vec<Option<usize>>>>,
    /// Rotation row used by each of the `order` factors of every component.
    rows: Vec<Vec<usize>>,
    /// `sqrt(m! / n!)` normalisation, `[n * count + m]`.
    factors: Vec<f64>,
}

fn factorial_of(exponents: &[u32]) -> f64 {
    exponents
        .iter()
        .map(|&e| (2..=e).map(f64::from).product::<f64>())
        .product()
}

fn steering_plan(ndim: usize, order: u32) -> Result<SteeringPlan> {
    let indices = riesz_multi_indices(ndim, order)?;
    let layers: Vec<Vec<Vec<u32>>> = (0..=order).map(|d| multi_indices_inner(ndim, d)).collect();
    let mut shifts = Vec::new();
    for degree in 0..order as usize {
        let sources: HashMap<&Vec<u32>, usize> = layers[degree]
            .iter()
            .enumerate()
            .map(|(i, m)| (m, i))
            .collect();
        let mut per_axis = Vec::new();
        for axis in 0..ndim {
            let column = layers[degree + 1]
                .iter()
                .map(|monomial| {
                    if monomial[axis] == 0 {
                        None
                    } else {
                        let mut reduced = monomial.clone();
                        reduced[axis] -= 1;
                        Some(sources[&reduced])
                    }
                })
                .collect();
            per_axis.push(column);
        }
        shifts.push(per_axis);
    }
    let rows = indices
        .iter()
        .map(|monomial| {
            monomial
                .iter()
                .enumerate()
                .flat_map(|(axis, &power)| std::iter::repeat_n(axis, power as usize))
                .collect()
        })
        .collect();
    let count = indices.len();
    let mut factors = vec![0.0; count * count];
    for (row, output) in indices.iter().enumerate() {
        for (column, source) in indices.iter().enumerate() {
            factors[row * count + column] = (factorial_of(source) / factorial_of(output)).sqrt();
        }
    }
    Ok(SteeringPlan {
        count,
        order: order as usize,
        ndim,
        shifts,
        rows,
        factors,
    })
}

/// Evaluate the steering polynomials for one rotation matrix (row-major `D x D`).
fn steer(rotation: &[f64], plan: &SteeringPlan) -> Vec<f64> {
    let mut matrix = vec![0.0; plan.count * plan.count];
    for n in 0..plan.count {
        let mut poly = vec![1.0];
        for step in 0..plan.order {
            let row = plan.rows[n][step];
            let targets = plan.shifts[step][0].len();
            let mut next = vec![0.0; targets];
            for (axis, column) in plan.shifts[step].iter().enumerate() {
                let weight = rotation[row * plan.ndim + axis];
                for (target, source) in column.iter().enumerate() {
                    if let Some(source) = source {
                        next[target] += weight * poly[*source];
                    }
                }
            }
            poly = next;
        }
        for m in 0..plan.count {
            matrix[n * plan.count + m] = poly[m] * plan.factors[n * plan.count + m];
        }
    }
    matrix
}

/// Unit-sum sampled Gaussian taps truncated at four standard deviations.
fn gaussian_taps(sigma_mm: f64, spacing_mm: f64) -> Vec<f64> {
    let sigma = sigma_mm / spacing_mm;
    let radius = (4.0 * sigma).ceil() as i64;
    let taps: Vec<f64> = (-radius..=radius)
        .map(|i| {
            let offset = i as f64 / sigma;
            (-0.5 * offset * offset).exp()
        })
        .collect();
    let total: f64 = taps.iter().sum();
    taps.into_iter().map(|t| t / total).collect()
}

/// Separable Gaussian smoothing with per-axis spacing in millimetres.
fn smooth_gaussian(
    image: &ArrayD<f64>,
    spacing: &[f64],
    sigma_mm: f64,
    padding: FilterPadding,
) -> Result<ArrayD<f64>> {
    let taps: Vec<Vec<f64>> = spacing
        .iter()
        .map(|&s| gaussian_taps(sigma_mm, s))
        .collect();
    convolve_separable_native(image, &taps, padding, 0.0)
}

type V3 = [f64; 3];

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Euclidean norm from a largest-component scaling (avoids overflow of the squares).
fn scaled_norm(v: &[f64]) -> f64 {
    let scale = v.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    let safe = if scale > 0.0 { scale } else { 1.0 };
    scale
        * v.iter()
            .map(|x| (x / safe) * (x / safe))
            .sum::<f64>()
            .sqrt()
}

fn normalised(v: V3) -> V3 {
    let norm = scaled_norm(&v).max(SMALLEST_NORM);
    v.map(|x| x / norm)
}

/// Make the largest-magnitude component positive, keeping the lowest index on ties.
fn positive_leading(v: &[f64]) -> Vec<f64> {
    let mut best = 0;
    for (i, x) in v.iter().enumerate() {
        if x.abs() > v[best].abs() {
            best = i;
        }
    }
    if v[best] < 0.0 {
        v.iter().map(|x| -x).collect()
    } else {
        v.to_vec()
    }
}

type M3 = [[f64; 3]; 3];

/// Unit null vector of `J - eigenvalue I` from the largest row cross product.
fn null_vector(m: &M3, eigenvalue: f64) -> V3 {
    let first = [m[0][0] - eigenvalue, m[0][1], m[0][2]];
    let second = [m[0][1], m[1][1] - eigenvalue, m[1][2]];
    let third = [m[0][2], m[1][2], m[2][2] - eigenvalue];
    let candidates = [
        cross(first, second),
        cross(first, third),
        cross(second, third),
    ];
    let norms = candidates.map(|c| scaled_norm(&c));
    let use_second = norms[1] > norms[0] && norms[1] >= norms[2];
    let use_third = norms[2] > norms[0] && norms[2] > norms[1];
    normalised(if use_second {
        candidates[1]
    } else if use_third {
        candidates[2]
    } else {
        candidates[0]
    })
}

/// Deterministic unit vector orthogonal to `v`, from the axis of its smallest component.
fn complement(v: V3) -> V3 {
    let mut axis = 0;
    for i in 1..3 {
        if v[i].abs() < v[axis].abs() {
            axis = i;
        }
    }
    let mut basis = [0.0; 3];
    basis[axis] = 1.0;
    normalised(cross(v, basis))
}

fn bilinear(m: &M3, left: V3, right: V3) -> f64 {
    let mut total = 0.0;
    for i in 0..3 {
        let mut row = 0.0;
        for j in 0..3 {
            row += left[j] * m[j][i];
        }
        total += row * right[i];
    }
    total
}

/// Sort three orthonormal rows by descending Rayleigh quotient.
fn sorted_by_rayleigh(m: &M3, mut rows: [V3; 3]) -> [V3; 3] {
    let mut q = [
        bilinear(m, rows[0], rows[0]),
        bilinear(m, rows[1], rows[1]),
        bilinear(m, rows[2], rows[2]),
    ];
    if q[1] > q[0] {
        rows.swap(0, 1);
        q.swap(0, 1);
    }
    if q[2] > q[1] {
        rows.swap(1, 2);
        q.swap(1, 2);
    }
    if q[1] > q[0] {
        rows.swap(0, 1);
    }
    rows
}

/// Closed-form eigenvectors of a symmetric 3x3 matrix (Smith / Eberly), as rows.
fn rotation_3x3(m: &M3) -> [V3; 3] {
    let (a, b, c, d, e, f) = (m[0][0], m[0][1], m[0][2], m[1][1], m[1][2], m[2][2]);
    let mean = (a + d + f) / 3.0;
    let off = b * b + c * c + e * e;
    let spread = (a - mean).powi(2) + (d - mean).powi(2) + (f - mean).powi(2) + 2.0 * off;
    let radius = (spread.max(0.0) / 6.0).sqrt();
    let safe = if radius > 0.0 { radius } else { 1.0 };
    let (sa, sb, sc) = ((a - mean) / safe, b / safe, c / safe);
    let (sd, se, sf) = ((d - mean) / safe, e / safe, (f - mean) / safe);
    let determinant =
        sa * (sd * sf - se * se) - sb * (sb * sf - se * sc) + sc * (sb * se - sd * sc);
    let angle = (determinant / 2.0).clamp(-1.0, 1.0).acos() / 3.0;
    let largest = mean + 2.0 * radius * angle.cos();
    let smallest = mean + 2.0 * radius * (angle + 2.0 * std::f64::consts::PI / 3.0).cos();
    let middle = 3.0 * mean - largest - smallest;

    let top_isolated = (largest - middle) >= (middle - smallest);
    let isolated = null_vector(m, if top_isolated { largest } else { smallest });
    let p = complement(isolated);
    let q = cross(isolated, p);
    let (plane_a, plane_b, plane_c) = (bilinear(m, p, p), bilinear(m, p, q), bilinear(m, q, q));
    let plane_angle = (2.0 * plane_b).atan2(plane_a - plane_c) / 2.0;
    let upper_raw = [
        plane_angle.cos() * p[0] + plane_angle.sin() * q[0],
        plane_angle.cos() * p[1] + plane_angle.sin() * q[1],
        plane_angle.cos() * p[2] + plane_angle.sin() * q[2],
    ];
    let lower = normalised(cross(isolated, upper_raw));
    let upper = normalised(upper_raw);
    let rows = if top_isolated {
        [isolated, upper, lower]
    } else {
        [upper, lower, isolated]
    };
    let rows = sorted_by_rayleigh(m, rows);
    let first: V3 = positive_leading(&rows[0])
        .try_into()
        .expect("three components");
    let second: V3 = positive_leading(&rows[1])
        .try_into()
        .expect("three components");
    let magnitude = largest.abs() + middle.abs() + smallest.abs();
    if (largest - smallest) <= ISOTROPIC_TOLERANCE * magnitude {
        return [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    }
    [first, second, cross(first, second)]
}

/// Closed-form eigenvectors of a symmetric 2x2 matrix, as rows.
fn rotation_2x2(a: f64, b: f64, c: f64) -> [[f64; 2]; 2] {
    let half = (a - c) / 2.0;
    let delta = (half * half + b * b).sqrt();
    let largest = (a + c) / 2.0 + delta;
    let first_row = [a - largest, b];
    let second_row = [b, c - largest];
    let from_first = [b, largest - a];
    let from_second = [c - largest, -b];
    let use_first = first_row[0] * first_row[0] + first_row[1] * first_row[1]
        >= second_row[0] * second_row[0] + second_row[1] * second_row[1];
    let chosen = if use_first { from_first } else { from_second };
    let norm = scaled_norm(&chosen).max(SMALLEST_NORM);
    let unit = [chosen[0] / norm, chosen[1] / norm];
    let first = positive_leading(&unit);
    if delta <= ISOTROPIC_TOLERANCE * (a.abs() + b.abs() + c.abs()) {
        return [[1.0, 0.0], [0.0, 1.0]];
    }
    [[first[0], first[1]], [-first[1], first[0]]]
}

/// Rotation whose rows are the descending-eigenvalue eigenvectors of a symmetric tensor.
///
/// `tensor` is row-major `D x D` (`D` = 2 or 3). The tensor is divided by its largest
/// absolute entry before the solve (positive scale, eigenvectors unchanged). Rows are
/// ordered by descending Rayleigh quotient, `u1` and `u2` have their largest-magnitude
/// component positive and `u3` closes the right-handed frame. Degenerate spectra return
/// the documented deterministic completions; the zero tensor returns the identity.
pub fn symmetric_rotation(tensor: &[f64], ndim: usize) -> Result<Vec<f64>> {
    if (ndim != 2 && ndim != 3) || tensor.len() != ndim * ndim {
        return Err(Error::InvalidInput(
            "structure tensors must be batched 2x2 or 3x3 matrices".into(),
        ));
    }
    let magnitude = tensor.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    let scale = if magnitude > 0.0 { magnitude } else { 1.0 };
    let t: Vec<f64> = tensor.iter().map(|x| x / scale).collect();
    Ok(if ndim == 3 {
        let m: M3 = [[t[0], t[1], t[2]], [t[3], t[4], t[5]], [t[6], t[7], t[8]]];
        rotation_3x3(&m).iter().flatten().copied().collect()
    } else {
        rotation_2x2(t[0], t[1], t[3])
            .iter()
            .flatten()
            .copied()
            .collect()
    })
}

fn validated_responses(
    first_order: &[ArrayD<f64>],
    spacing: &[f64],
    tensor_sigma: f64,
) -> Result<Vec<usize>> {
    if first_order.len() != 2 && first_order.len() != 3 {
        return Err(Error::InvalidInput(
            "Riesz alignment requires the two or three first-order responses".into(),
        ));
    }
    let ndim = first_order.len();
    if spacing.len() != ndim {
        return Err(Error::InvalidInput(
            "spacing must provide one value per array axis".into(),
        ));
    }
    if spacing.iter().any(|s| !s.is_finite() || *s <= 0.0) {
        return Err(Error::InvalidInput(
            "spacing values must be positive and finite".into(),
        ));
    }
    if !tensor_sigma.is_finite() || tensor_sigma <= 0.0 {
        return Err(Error::InvalidInput(
            "tensor_sigma must be positive and finite".into(),
        ));
    }
    let shape = first_order[0].shape().to_vec();
    if shape.len() != ndim {
        return Err(Error::InvalidInput(
            "first-order Riesz responses must have one axis per rotation axis".into(),
        ));
    }
    if first_order[1..]
        .iter()
        .any(|r| r.shape() != shape.as_slice())
    {
        return Err(Error::InvalidInput(
            "first-order Riesz responses must share one spatial shape".into(),
        ));
    }
    Ok(shape)
}

/// Per-voxel rotation whose rows are the structure-tensor eigenvectors.
///
/// The regularised structure tensor of Eq. structureTensor has entries `J[m, n]` equal to
/// the products `R_m R_n` of the first-order responses convolved with a Gaussian window of
/// `tensor_sigma` mm on every axis. `Constant` extends each product by zero. The bank is
/// divided by one common maximum-magnitude scale first (preserves every eigenvector).
///
/// Returns the flat row-major rotations, `D * D` values per voxel in C order of the
/// spatial shape. Errors as in the Python: inconsistent inputs, or a responding voxel
/// whose smoothed tensor underflows to zero.
pub fn structure_tensor_rotations(
    first_order: &[ArrayD<f64>],
    spacing: &[f64],
    tensor_sigma: f64,
    padding: FilterPadding,
) -> Result<Vec<f64>> {
    let shape = validated_responses(first_order, spacing, tensor_sigma)?;
    let ndim = first_order.len();
    let amplitude = first_order
        .iter()
        .flat_map(|r| r.iter())
        .fold(0.0_f64, |m, x| m.max(x.abs()));
    let safe = if amplitude > 0.0 { amplitude } else { 1.0 };
    let scaled: Vec<ArrayD<f64>> = first_order.iter().map(|r| r / safe).collect();
    let mut smoothed: HashMap<(usize, usize), ArrayD<f64>> = HashMap::new();
    for row in 0..ndim {
        for column in row..ndim {
            let product = &scaled[row] * &scaled[column];
            smoothed.insert(
                (row, column),
                smooth_gaussian(&product, spacing, tensor_sigma, padding)?,
            );
        }
    }
    let count: usize = shape.iter().product();
    let flat: HashMap<(usize, usize), Vec<f64>> = smoothed
        .into_iter()
        .map(|(k, v)| (k, v.iter().copied().collect()))
        .collect();
    let bank: Vec<Vec<f64>> = first_order
        .iter()
        .map(|r| r.iter().copied().collect())
        .collect();
    let mut rotations = Vec::with_capacity(count * ndim * ndim);
    let mut tensor = vec![0.0; ndim * ndim];
    for voxel in 0..count {
        let mut trace = 0.0;
        for axis in 0..ndim {
            trace += flat[&(axis, axis)][voxel];
        }
        if trace == 0.0 && bank.iter().any(|r| r[voxel] != 0.0) {
            return Err(Error::InvalidInput(
                "Riesz structure-tensor products exceed float32 dynamic range".into(),
            ));
        }
        for row in 0..ndim {
            for column in 0..ndim {
                tensor[row * ndim + column] = flat[&(row.min(column), row.max(column))][voxel];
            }
        }
        rotations.extend(symmetric_rotation(&tensor, ndim)?);
    }
    Ok(rotations)
}

/// Align the order-`order` Riesz bank with the local structure-tensor frame.
///
/// Implements Eq. steerability, `R{f}_U = S_U R{f}`: response `n` is the combination of
/// the axis-aligned responses weighted by row `n` of the steering matrix. `components` are
/// all `K` order-`order` responses in [`riesz_multi_indices`] order; `first_order` are the
/// `D` first-order responses used for the structure tensor. Returns `K` aligned maps in
/// the order of `components`.
pub fn align_riesz(
    components: &[ArrayD<f64>],
    first_order: &[ArrayD<f64>],
    order: u32,
    spacing: &[f64],
    tensor_sigma: f64,
    padding: FilterPadding,
) -> Result<Vec<ArrayD<f64>>> {
    if order < 1 {
        return Err(Error::InvalidInput(
            "Riesz alignment requires an order of at least one".into(),
        ));
    }
    let shape = validated_responses(first_order, spacing, tensor_sigma)?;
    let ndim = first_order.len();
    let indices = riesz_multi_indices(ndim, order)?;
    if components.len() != indices.len() {
        return Err(Error::InvalidInput(format!(
            "order-{order} alignment in {ndim} dimensions needs {} components, received {}",
            indices.len(),
            components.len()
        )));
    }
    if components.iter().any(|c| c.shape() != shape.as_slice()) {
        return Err(Error::InvalidInput(
            "Riesz components must share the first-order spatial shape".into(),
        ));
    }
    if order > MAX_STEERING_ORDER {
        return Err(Error::InvalidInput(format!(
            "Riesz steering is qualified up to order {MAX_STEERING_ORDER}; order {order} is numerically unstable in float32"
        )));
    }
    let rotations = structure_tensor_rotations(first_order, spacing, tensor_sigma, padding)?;
    let plan = steering_plan(ndim, order)?;
    let count: usize = shape.iter().product();
    let flat: Vec<Vec<f64>> = components
        .iter()
        .map(|c| c.iter().copied().collect())
        .collect();
    let k = plan.count;
    let mut aligned = vec![vec![0.0; count]; k];
    for voxel in 0..count {
        let matrix = steer(
            &rotations[voxel * ndim * ndim..(voxel + 1) * ndim * ndim],
            &plan,
        );
        for n in 0..k {
            let mut total = 0.0;
            for m in 0..k {
                total += matrix[n * k + m] * flat[m][voxel];
            }
            aligned[n][voxel] = total;
        }
    }
    Ok(aligned
        .into_iter()
        .map(|values| ArrayD::from_shape_vec(IxDyn(&shape), values).expect("shape"))
        .collect())
}
