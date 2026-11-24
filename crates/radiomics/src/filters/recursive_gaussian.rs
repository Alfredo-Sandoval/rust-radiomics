//! ITK's recursive Gaussian recurrence (the LoG image type).
//!
//! Mirrors `mlxradiomics/filters/recursive_gaussian.py` (376 lines at mlx-radiomics
//! b58783c). Coefficient equations follow ITK 5.4.4 (Copyright NumFOCUS, Apache-2.0),
//! `Modules/Filtering/Smoothing/include/itkRecursiveGaussianImageFilter.hxx`; see
//! `THIRD_PARTY_NOTICES.md`.
//! Status: done.
//!
//! Execution: CPU numerical work in float64. The coefficients are prepared exactly as in
//! mlx-radiomics (factored denominator, `fsum`-style exact sums). The recurrence is the
//! same centred form (each row's deviation from its boundary value is filtered, and the
//! analytic constant response is added back), evaluated in plain float64 instead of
//! float-float pairs. Blocks of eight lines are filtered together on rayon worker threads,
//! so the recursion's inner loops run across lines; strided axes are not transposed. Input contract: 3-D
//! [`ArrayD`] in NumPy order with every axis at least four voxels.

use crate::error::{Error, Result};
use crate::filters::kernels::{copy_flat, par_column_blocks};
use ndarray::{ArrayD, IxDyn};
use rayon::prelude::*;

/// The recurrence is qualified through 200 voxels of `sigma / spacing`.
const MAX_SIGMA_RATIO: f64 = 200.0;
/// Reserve roughly half the host significand after cancellation (`sqrt(f64::EPSILON)`).
const MIN_RELATIVE_SECOND_ORDER_RESPONSE: f64 = 1.490_116_119_384_765_6e-8;

/// Coefficients of the centred causal and anticausal recursions for one sigma and order.
///
/// `causal_dc` and `anticausal_dc` are the constant gains added after filtering each row's
/// deviation from its left or right boundary value; their sum is the analytic constant
/// response (one for order 0, zero for order 2).
#[derive(Clone, Debug, PartialEq)]
pub struct RecursiveCoefficients {
    /// Numerator `N0..N3`.
    pub numerator: [f64; 4],
    /// Denominator `D1..D4`.
    pub denominator: [f64; 4],
    /// Anticausal numerator `M1..M4`.
    pub anticausal: [f64; 4],
    /// Causal constant gain.
    pub causal_dc: f64,
    /// Anticausal constant gain.
    pub anticausal_dc: f64,
}

/// Exactly rounded sum (Shewchuk's algorithm, as Python `math.fsum`).
fn fsum(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut partials: Vec<f64> = Vec::new();
    for mut x in values {
        let mut kept = 0;
        for index in 0..partials.len() {
            let mut y = partials[index];
            if x.abs() < y.abs() {
                std::mem::swap(&mut x, &mut y);
            }
            let high = x + y;
            let low = y - (high - x);
            if low != 0.0 {
                partials[kept] = low;
                kept += 1;
            }
            x = high;
        }
        partials.truncate(kept);
        partials.push(x);
    }
    let mut total = 0.0;
    let mut index = partials.len();
    let mut low = 0.0;
    while index > 0 {
        index -= 1;
        let x = total;
        let y = partials[index];
        total = x + y;
        low = y - (total - x);
        if low != 0.0 {
            break;
        }
    }
    if index > 0
        && ((low < 0.0 && partials[index - 1] < 0.0) || (low > 0.0 && partials[index - 1] > 0.0))
    {
        let y = low * 2.0;
        let x = total + y;
        if y == x - total {
            total = x;
        }
    }
    total
}

/// ITK's Deriche numerator coefficients `(N0..N3)` for `order` 0 or 2.
fn numerator(sigma_voxels: f64, order: u32) -> [f64; 4] {
    let (a1, b1, a2, b2) = if order == 0 {
        (1.3530, 1.8151, -0.3531, 0.0902)
    } else {
        (-1.3563, 5.2318, 0.3446, -2.2355)
    };
    let (theta1, theta2) = (0.6681 / sigma_voxels, 2.0787 / sigma_voxels);
    let (s1, s2) = (theta1.sin(), theta2.sin());
    let (c1, c2) = (theta1.cos(), theta2.cos());
    let (e1, e2) = (
        (-1.3932 / sigma_voxels).exp(),
        (-1.3732 / sigma_voxels).exp(),
    );
    [
        a1 + a2,
        e2 * (b2 * s2 - (a2 + 2.0 * a1) * c2) + e1 * (b1 * s1 - (a1 + 2.0 * a2) * c1),
        2.0 * e1 * e2 * ((a1 + a2) * c2 * c1 - b1 * c2 * s1 - b2 * c1 * s2)
            + a2 * e1.powi(2)
            + a1 * e2.powi(2),
        e2 * e1.powi(2) * (b2 * s2 - a2 * c2) + e1 * e2.powi(2) * (b1 * s1 - a1 * c1),
    ]
}

/// `(D1..D4, SD, DD, ED)` with the DC normalisation evaluated in factored form.
fn denominator(sigma_voxels: f64) -> ([f64; 4], f64, f64, f64) {
    let (e1, e2) = (
        (-1.3932 / sigma_voxels).exp(),
        (-1.3732 / sigma_voxels).exp(),
    );
    let (theta1, theta2) = (0.6681 / sigma_voxels, 2.0787 / sigma_voxels);
    let (c1, c2) = (theta1.cos(), theta2.cos());
    let (p1, p2) = (-2.0 * e1 * c1, -2.0 * e2 * c2);
    let (q1, q2) = (e1.powi(2), e2.powi(2));
    let f1 = (1.0 - e1).powi(2) + 4.0 * e1 * (theta1 / 2.0).sin().powi(2);
    let f2 = (1.0 - e2).powi(2) + 4.0 * e2 * (theta2 / 2.0).sin().powi(2);
    let sd = f1 * f2;
    let (dp1, dp2) = (p1 + 2.0 * q1, p2 + 2.0 * q2);
    let second = 2.0 * q1 * f2 + 2.0 * dp1 * dp2 + 2.0 * q2 * f1;
    let dd = dp1 * f2 + f1 * dp2;
    let ed = dd + second;
    (
        [p1 + p2, q1 + q2 + p1 * p2, p1 * q2 + p2 * q1, q1 * q2],
        sd,
        dd,
        ed,
    )
}

fn singular(sigma_voxels: f64) -> Error {
    Error::InvalidInput(format!(
        "recursive Gaussian coefficient normalization is singular for sigma / spacing = {sigma_voxels:?}"
    ))
}

fn not_finite(sigma_voxels: f64) -> Error {
    Error::InvalidInput(format!(
        "recursive Gaussian coefficients are not finite for sigma / spacing = {sigma_voxels:?}"
    ))
}

/// Stable ITK recurrence coefficients for one physical sigma and voxel spacing.
///
/// Errors (`ValueError`): invalid parameters, a ratio above 200, or a singular,
/// ill-conditioned or non-finite normalisation.
pub fn coefficients(sigma: f64, spacing: f64, order: u32) -> Result<RecursiveCoefficients> {
    if order != 0 && order != 2 {
        return Err(Error::InvalidInput("Gaussian order must be 0 or 2".into()));
    }
    if !sigma.is_finite() || sigma <= 0.0 {
        return Err(Error::InvalidInput(
            "sigma must be finite and positive".into(),
        ));
    }
    if !spacing.is_finite() || spacing <= 0.0 {
        return Err(Error::InvalidInput(
            "spacing must be finite and positive".into(),
        ));
    }
    let sigma_voxels = sigma / spacing;
    if !sigma_voxels.is_finite() || sigma_voxels <= 0.0 {
        return Err(Error::InvalidInput(
            "sigma / spacing must be finite and positive".into(),
        ));
    }
    if sigma_voxels > MAX_SIGMA_RATIO {
        return Err(Error::InvalidInput(
            "recursive Gaussian requires sigma / spacing <= 200".into(),
        ));
    }
    if sigma_voxels < 1.3732 / 746.0 {
        if order == 2 {
            return Err(Error::InvalidInput(
                "second-order recursive Gaussian normalization is singular".into(),
            ));
        }
        return Ok(RecursiveCoefficients {
            numerator: [1.0, 0.0, 0.0, 0.0],
            denominator: [0.0; 4],
            anticausal: [0.0; 4],
            causal_dc: 1.0,
            anticausal_dc: 0.0,
        });
    }
    let (d, sd, dd, ed) = denominator(sigma_voxels);
    if !sd.is_finite() || sd <= 0.0 {
        return Err(singular(sigma_voxels));
    }
    let n0 = numerator(sigma_voxels, 0);
    let n: [f64; 4];
    if order == 0 {
        let normalizer = 2.0 * fsum(n0) / sd - n0[0];
        if !normalizer.is_finite() || normalizer == 0.0 {
            return Err(singular(sigma_voxels));
        }
        n = n0.map(|value| value / normalizer);
    } else {
        let n2 = numerator(sigma_voxels, 2);
        let denom = 2.0 * fsum(n0) - sd * n0[0];
        if !denom.is_finite() || denom == 0.0 {
            return Err(singular(sigma_voxels));
        }
        let beta = -(2.0 * fsum(n2) - sd * n2[0]) / denom;
        let mut combined = [0.0; 4];
        for k in 0..4 {
            combined[k] = n2[k] + beta * n0[k];
        }
        let max_abs = |values: &[f64; 4]| values.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        let scale = max_abs(&n2).max(beta.abs() * max_abs(&n0));
        if max_abs(&combined) <= MIN_RELATIVE_SECOND_ORDER_RESPONSE * scale {
            return Err(Error::InvalidInput(
                "second-order recursive Gaussian normalization is ill-conditioned".into(),
            ));
        }
        let sigma_squared = sigma * sigma;
        if !sigma_squared.is_finite() || sigma_squared == 0.0 {
            return Err(Error::InvalidInput(
                "second-order recursive Gaussian scale is outside the numeric range".into(),
            ));
        }
        let sn = fsum(combined);
        let dn = fsum(combined.iter().enumerate().map(|(k, v)| k as f64 * v));
        let en = fsum(combined.iter().enumerate().map(|(k, v)| (k * k) as f64 * v));
        let sd3 = sd * sd * sd;
        if !sd3.is_finite() || sd3 == 0.0 {
            return Err(singular(sigma_voxels));
        }
        let alpha = (en * sd * sd - ed * sn * sd - 2.0 * dn * dd * sd + 2.0 * dd * dd * sn) / sd3;
        if !alpha.is_finite() || alpha == 0.0 {
            return Err(singular(sigma_voxels));
        }
        n = combined.map(|value| value * sigma_squared / alpha);
    }
    let m = [
        n[1] - d[0] * n[0],
        n[2] - d[1] * n[0],
        n[3] - d[2] * n[0],
        -d[3] * n[0],
    ];
    if n.iter().chain(&d).chain(&m).any(|v| !v.is_finite()) {
        return Err(not_finite(sigma_voxels));
    }
    let causal_dc = fsum(n) / sd;
    let anticausal_dc = (if order == 0 { 1.0 } else { 0.0 }) - causal_dc;
    if !causal_dc.is_finite() || !anticausal_dc.is_finite() {
        return Err(not_finite(sigma_voxels));
    }
    Ok(RecursiveCoefficients {
        numerator: n,
        denominator: d,
        anticausal: m,
        causal_dc,
        anticausal_dc,
    })
}

/// Lines filtered together: the recursion runs once over `LANES` lines held sample-major, so
/// its inner loops over the lines vectorise.
const LANES: usize = 8;

/// Filter a sample-major block of `LANES` lines with the centred causal and anticausal
/// recursions.
///
/// Per line the floating-point operations and their order are those of the scalar
/// recursion: the interior of each recursion is unrolled over the four taps in tap order,
/// and only the first and last four samples take the edge-clamped path.
fn filter_block(
    x: &mut [[f64; LANES]],
    c: &RecursiveCoefficients,
    causal: &mut [[f64; LANES]],
    anti: &mut [[f64; LANES]],
) {
    let length = x.len();
    let (x0, xn) = (x[0], x[length - 1]);
    let (n, d, m) = (&c.numerator, &c.denominator, &c.anticausal);
    for i in 0..length.min(4) {
        for l in 0..LANES {
            let mut numerator = 0.0;
            let mut feedback = 0.0;
            for k in 0..4 {
                numerator += (x[i.saturating_sub(k)][l] - x0[l]) * n[k];
                if i > k {
                    feedback += causal[i - k - 1][l] * d[k];
                }
            }
            causal[i][l] = numerator - feedback;
        }
    }
    for i in 4..length {
        for l in 0..LANES {
            let mut numerator = 0.0;
            numerator += (x[i][l] - x0[l]) * n[0];
            numerator += (x[i - 1][l] - x0[l]) * n[1];
            numerator += (x[i - 2][l] - x0[l]) * n[2];
            numerator += (x[i - 3][l] - x0[l]) * n[3];
            let mut feedback = 0.0;
            feedback += causal[i - 1][l] * d[0];
            feedback += causal[i - 2][l] * d[1];
            feedback += causal[i - 3][l] * d[2];
            feedback += causal[i - 4][l] * d[3];
            causal[i][l] = numerator - feedback;
        }
    }
    for i in (0..length).rev() {
        if i + 4 < length {
            for l in 0..LANES {
                let mut numerator = 0.0;
                numerator += (x[i + 1][l] - xn[l]) * m[0];
                numerator += (x[i + 2][l] - xn[l]) * m[1];
                numerator += (x[i + 3][l] - xn[l]) * m[2];
                numerator += (x[i + 4][l] - xn[l]) * m[3];
                let mut feedback = 0.0;
                feedback += anti[i + 1][l] * d[0];
                feedback += anti[i + 2][l] * d[1];
                feedback += anti[i + 3][l] * d[2];
                feedback += anti[i + 4][l] * d[3];
                anti[i][l] = numerator - feedback;
            }
        } else {
            for l in 0..LANES {
                let mut numerator = 0.0;
                let mut feedback = 0.0;
                for k in 0..4 {
                    numerator += (x[(i + k + 1).min(length - 1)][l] - xn[l]) * m[k];
                    if i + k + 1 < length {
                        feedback += anti[i + k + 1][l] * d[k];
                    }
                }
                anti[i][l] = numerator - feedback;
            }
        }
    }
    for l in 0..LANES {
        let offset = c.causal_dc * x0[l] + c.anticausal_dc * xn[l];
        for i in 0..length {
            x[i][l] = causal[i][l] + anti[i][l] + offset;
        }
    }
}

/// Filter `image` along one axis with a recursive Gaussian of order 0 or 2.
///
/// `sigma` and `spacing` share a unit (mm); the kernel width in voxels is
/// `sigma / spacing`. Order 0 smooths; order 2 applies the ITK second-derivative filter
/// ([`laplacian_gaussian`] divides its result by the squared spacing). Boundaries continue
/// the edge value as a constant.
///
/// Errors: `axis` outside the image, invalid `order`, `sigma` or `spacing`, fewer than four
/// samples along `axis`, or an unsupported second-order ratio.
pub fn recursive_gaussian(
    image: &ArrayD<f64>,
    sigma: f64,
    spacing: f64,
    axis: usize,
    order: u32,
) -> Result<ArrayD<f64>> {
    if axis >= image.ndim() {
        return Err(Error::InvalidInput(
            "axis must index the image's dimensions".into(),
        ));
    }
    let c = coefficients(sigma, spacing, order)?;
    let shape = image.shape().to_vec();
    let length = shape[axis];
    if length < 4 {
        return Err(Error::InvalidInput(
            "recursive Gaussian requires at least four samples along the axis".into(),
        ));
    }
    let inner: usize = shape[axis + 1..].iter().product();
    let mut data = copy_flat(image);
    let buffers = || {
        (
            vec![[0.0; LANES]; length],
            vec![[0.0; LANES]; length],
            vec![[0.0; LANES]; length],
        )
    };
    if inner == 1 {
        // Contiguous lines: gather `LANES` of them sample-major, filter, scatter back.
        data.par_chunks_mut(LANES * length).for_each_init(
            buffers,
            |(block, causal, anti), chunk| {
                let lines = chunk.len() / length;
                for i in 0..length {
                    for l in 0..LANES {
                        // Unused lines repeat line 0 so that no lane holds uninitialised noise.
                        let line = if l < lines { l } else { 0 };
                        block[i][l] = chunk[line * length + i];
                    }
                }
                filter_block(block, &c, causal, anti);
                for i in 0..length {
                    for l in 0..lines {
                        chunk[l * length + i] = block[i][l];
                    }
                }
            },
        );
    } else {
        // Strided lines: the row pieces of neighbouring columns already are sample-major.
        par_column_blocks(&mut data, length, inner, LANES, |rows| {
            let width = rows[0].len();
            let (mut block, mut causal, mut anti) = buffers();
            for (k, row) in rows.iter().enumerate() {
                for l in 0..LANES {
                    block[k][l] = row[l.min(width - 1)];
                }
            }
            filter_block(&mut block, &c, &mut causal, &mut anti);
            for (k, row) in rows.iter_mut().enumerate() {
                for (l, slot) in row.iter_mut().enumerate() {
                    *slot = block[k][l];
                }
            }
        });
    }
    Ok(ArrayD::from_shape_vec(IxDyn(&shape), data).expect("shape"))
}

/// Fail unless [`laplacian_gaussian`] accepts this grid and sigma.
///
/// Shared with the derived-image pre-flight so an unsupported request fails before any
/// image is extracted, with the same message the filter would raise.
pub fn validate_log_domain(shape: &[usize], spacing: &[f64], sigma: f64) -> Result<()> {
    if shape.len() != 3 || shape.iter().any(|&n| n < 4) {
        return Err(Error::InvalidInput(
            "recursive LoG requires a 3D image with every axis at least four voxels".into(),
        ));
    }
    if spacing.len() != 3 || spacing.iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err(Error::InvalidInput(
            "LoG spacing must contain three finite positive values".into(),
        ));
    }
    if !sigma.is_finite() || sigma <= 0.0 {
        return Err(Error::InvalidInput(
            "LoG sigma must be finite and positive".into(),
        ));
    }
    for &axis_spacing in spacing {
        coefficients(sigma, axis_spacing, 2)?;
    }
    if shape
        .iter()
        .zip(spacing)
        .any(|(&n, &s)| (n as f64) < (sigma / s).ceil() + 1.0)
    {
        return Err(Error::InvalidInput(
            "LoG sigma exceeds the image's physical support".into(),
        ));
    }
    Ok(())
}

/// Laplacian of Gaussian of a 3-D image (PyRadiomics `log` image type).
///
/// The sum over axes of the second derivative along that axis, with Gaussian smoothing
/// along the other two, each as a recursive filter, divided by the squared spacing of the
/// derivative axis. `spacing` is in mm per NumPy axis.
pub fn laplacian_gaussian(image: &ArrayD<f64>, spacing: &[f64], sigma: f64) -> Result<ArrayD<f64>> {
    validate_log_domain(image.shape(), spacing, sigma)?;
    let mut output = ArrayD::<f64>::zeros(image.raw_dim());
    // ITK traverses physical x, y, z axes, opposite the NumPy array-axis order.
    for derivative_axis in (0..3).rev() {
        let mut filtered =
            recursive_gaussian(image, sigma, spacing[derivative_axis], derivative_axis, 2)?;
        for smooth_axis in (0..3).rev() {
            if smooth_axis != derivative_axis {
                filtered =
                    recursive_gaussian(&filtered, sigma, spacing[smooth_axis], smooth_axis, 0)?;
            }
        }
        let divisor = spacing[derivative_axis].powi(2);
        let filtered = filtered.as_standard_layout();
        let source = filtered.as_slice().expect("standard layout is contiguous");
        output
            .as_slice_mut()
            .expect("zeros are standard layout")
            .par_iter_mut()
            .zip(source)
            .for_each(|(o, &f)| *o += f / divisor);
    }
    Ok(output)
}
