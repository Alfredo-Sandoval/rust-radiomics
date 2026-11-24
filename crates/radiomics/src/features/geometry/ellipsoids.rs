//! Minimum-volume and covariance ellipsoids, areas and densities.
//!
//! Mirrors `mlxradiomics/features/geometry/ellipsoids.py` (719 lines at mlx-radiomics
//! b58783c): Khachiyan barycentric coordinate ascent for the minimum-volume enclosing
//! ellipsoid (IBSI tolerance 0.001, at most 100,000 steps), the exact ellipsoid surface
//! area (spheres and spheroids in closed form, triaxial axes by Carlson's `R_F` and
//! `R_D` duplication), the IBSI manual's printed Legendre series, and the volume and area
//! densities. The double-single Metal ascent and spectrum kernels are replaced by
//! float64 arithmetic on points whitened by their principal spread (Khachiyan's weights
//! are invariant under an invertible affine map, so conditioning is all that matters).
//!
//! Execution: CPU numerical work in float64, single-threaded. Input contract: physical
//! hull vertices `(n, 3)` for MVEE and the descending semi-axes `a >= b >= c` for areas.
//!
//! Status: done.

use std::f64::consts::PI;

use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::symmetric_eigen;
use crate::features::geometry::boxes::centroid;
use crate::features::geometry::mesh::MeshGeometry;

/// IBSI Khachiyan stopping tolerance on the weight update norm.
pub const MVEE_TOLERANCE: f64 = 1e-3;
/// Khachiyan iteration cap.
pub const MVEE_MAX_ITERATIONS: usize = 100_000;
/// Highest Legendre order of the IBSI manual's printed area series.
pub const IBSI_SERIES_ORDER: usize = 20;

/// Inverse of a symmetric positive-definite 3x3 matrix by cofactors.
///
/// Returns `None` unless the determinant is positive and every entry finite.
fn symmetric_inverse(m: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let (a, b, c) = (m[0][0], m[0][1], m[0][2]);
    let (d, e, f) = (m[1][1], m[1][2], m[2][2]);
    let co00 = d * f - e * e;
    let co01 = c * e - b * f;
    let co02 = b * e - c * d;
    let co11 = a * f - c * c;
    let co12 = b * c - a * e;
    let co22 = a * d - b * b;
    let determinant = a * co00 + b * co01 + c * co02;
    if !determinant.is_finite() || determinant <= 0.0 {
        return None;
    }
    let inverse = [[co00, co01, co02], [co01, co11, co12], [co02, co12, co22]]
        .map(|row| row.map(|value| value / determinant));
    inverse
        .iter()
        .flatten()
        .all(|value| value.is_finite())
        .then_some(inverse)
}

/// MVEE semi-axes of float64 hull points in their own coordinate unit.
///
/// Khachiyan coordinate ascent on whitened points; semi-axes follow `sqrt(3 * eig(C))` with
/// `C` the weighted covariance at the converged weights, graded back by the whitening
/// scales. Returns the three semi-axes in descending order.
///
/// # Errors
///
/// [`Error::Execution`] if there are fewer than four points, the points do not span three
/// dimensions, the weighted covariance loses positive definiteness, an iteration becomes
/// non-finite or does not converge within [`MVEE_MAX_ITERATIONS`] steps, or the smallest
/// semi-axis cannot be resolved above the roundoff bound.
pub fn mvee_semi_axes(host: &[[f64; 3]]) -> Result<[f64; 3]> {
    let count = host.len();
    if count < 4 {
        return Err(Error::Execution(
            "MVEE requires at least four hull vertices".into(),
        ));
    }
    // Whiten: centre, rotate onto the principal axes, divide by each spread.
    let mean = centroid(host);
    let centered: Vec<[f64; 3]> = host
        .iter()
        .map(|p| [p[0] - mean[0], p[1] - mean[1], p[2] - mean[2]])
        .collect();
    let mut second = [[0.0f64; 3]; 3];
    for p in &centered {
        for i in 0..3 {
            for j in 0..3 {
                second[i][j] += p[i] * p[j];
            }
        }
    }
    let (_, rotation) = symmetric_eigen(second);
    let rotated: Vec<[f64; 3]> = centered
        .iter()
        .map(|p| {
            let mut r = [0.0; 3];
            for (axis, value) in r.iter_mut().enumerate() {
                *value =
                    p[0] * rotation[0][axis] + p[1] * rotation[1][axis] + p[2] * rotation[2][axis];
            }
            r
        })
        .collect();
    let mut scales = [0.0f64; 3];
    for axis in 0..3 {
        let mean_square = rotated.iter().map(|p| p[axis] * p[axis]).sum::<f64>() / count as f64;
        scales[axis] = mean_square.sqrt();
    }
    if scales.iter().any(|s| !s.is_finite() || *s <= 0.0) {
        return Err(Error::Execution(
            "MVEE hull points do not span three dimensions".into(),
        ));
    }
    let points: Vec<[f64; 3]> = rotated
        .iter()
        .map(|p| [p[0] / scales[0], p[1] / scales[1], p[2] / scales[2]])
        .collect();
    let mut weights = vec![1.0 / count as f64; count];
    let covariance_of = |weights: &[f64]| -> ([f64; 3], [[f64; 3]; 3]) {
        let total: f64 = weights.iter().sum();
        let mut center = [0.0; 3];
        let mut moment = [[0.0; 3]; 3];
        for (p, w) in points.iter().zip(weights) {
            for i in 0..3 {
                center[i] += w * p[i];
                for j in i..3 {
                    moment[i][j] += w * p[i] * p[j];
                }
            }
        }
        for i in 0..3 {
            center[i] /= total;
        }
        let mut covariance = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in i..3 {
                let value = moment[i][j] / total - center[i] * center[j];
                covariance[i][j] = value;
                covariance[j][i] = value;
            }
        }
        (center, covariance)
    };
    let mut converged = false;
    for _ in 0..MVEE_MAX_ITERATIONS {
        let (center, covariance) = covariance_of(&weights);
        let Some(inverse) = symmetric_inverse(&covariance) else {
            return Err(Error::Execution(
                "MVEE covariance is not positive definite".into(),
            ));
        };
        let mut maximum = 0.0f64;
        let mut index = 0usize;
        for (i, p) in points.iter().enumerate() {
            let x = [p[0] - center[0], p[1] - center[1], p[2] - center[2]];
            let mut value = 0.0;
            for a in 0..3 {
                let y = inverse[a][0] * x[0] + inverse[a][1] * x[1] + inverse[a][2] * x[2];
                value += x[a] * y;
            }
            if value > maximum {
                maximum = value;
                index = i;
            }
        }
        // Khachiyan's lifted value is M = 1 + values, so the step (M - d - 1) /
        // ((d + 1)(M - 1)) becomes (values - d) / ((d + 1) values).
        let step = if maximum > 3.0 {
            (maximum - 3.0) / (4.0 * maximum)
        } else {
            0.0
        };
        let mut change = 0.0;
        let mut finite = true;
        for (i, weight) in weights.iter_mut().enumerate() {
            let indicator = if i == index { 1.0 } else { 0.0 };
            let updated = *weight * (1.0 - step) + step * indicator;
            finite &= updated.is_finite();
            let delta = updated - *weight;
            change += delta * delta;
            *weight = updated;
        }
        if !finite {
            return Err(Error::Execution(
                "MVEE iteration produced a non-finite weight".into(),
            ));
        }
        if change.sqrt() <= MVEE_TOLERANCE {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(Error::Execution(format!(
            "MVEE did not converge within {MVEE_MAX_ITERATIONS} iterations"
        )));
    }
    let (_, whitened) = covariance_of(&weights);
    let mut graded = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            graded[i][j] = whitened[i][j] * scales[i] * scales[j];
        }
    }
    let peak = graded
        .iter()
        .flatten()
        .fold(0.0f64, |acc, v| acc.max(v.abs()));
    let (eigen, _) = symmetric_eigen(graded);
    let tolerance = 8.0 * 3.0 * f64::EPSILON * peak;
    if eigen.iter().any(|v| !v.is_finite()) {
        return Err(Error::Execution(
            "MVEE covariance spectrum is not finite".into(),
        ));
    }
    if eigen.iter().any(|v| *v < -tolerance) {
        return Err(Error::Execution(
            "MVEE covariance has an eigenvalue below its PSD roundoff bound".into(),
        ));
    }
    if eigen[0] <= tolerance {
        return Err(Error::Execution(
            "MVEE covariance is too ill-conditioned to resolve the smallest semi-axis".into(),
        ));
    }
    let semi = eigen.map(|value| (3.0 * value).sqrt());
    if semi.iter().any(|v| !v.is_finite()) {
        return Err(Error::Execution("MVEE semi-axes are not finite".into()));
    }
    Ok([semi[2], semi[1], semi[0]])
}

/// Exact ellipsoid surface area for semi-axes `a >= b >= c`.
///
/// This is the sum of the IBSI Legendre series with eccentricities referenced to the
/// shortest axis, `1-c^2/a^2` and `1-c^2/b^2` (Tee, Surface Area of Ellipsoid Segment,
/// equations 11-14). The IBSI manual's displayed eccentricities refer to the longest axis
/// instead; that transcription does not recover spheroid areas. Spheres and spheroids use
/// closed forms; triaxial axes use Carlson's `R_G`, to which the corrected series converges.
#[must_use]
pub fn ellipsoid_area_normalized(a: f64, b: f64, c: f64) -> f64 {
    if c <= 0.0 {
        return 2.0 * PI * a * b;
    }
    if a == c {
        return 4.0 * PI * a * a;
    }
    if b == c {
        let eccentricity = (1.0 - (b / a).powi(2)).sqrt();
        return 2.0 * PI * b * (b + a * eccentricity.asin() / eccentricity);
    }
    if a == b {
        let ratio = c / a;
        // q = e^2 = (1 - r)(1 + r); atanh(e)/e = sum q^k / (2k + 1).
        let squared = (1.0 - ratio) * (1.0 + ratio);
        let atanh_ratio = if squared <= 0.25 {
            let mut total = 0.0;
            for k in (0..=40).rev() {
                total = total * squared + 1.0 / (2 * k + 1) as f64;
            }
            total
        } else {
            let eccentricity = squared.sqrt();
            ((1.0 + eccentricity) / ratio).ln() / eccentricity
        };
        return 2.0 * PI * a * a * (1.0 + ratio * ratio * atanh_ratio);
    }
    // S = 4 pi a b c R_G(a^-2, b^-2, c^-2) (DLMF 19.33.2). Scaling by b^2 puts the middle
    // argument at one, where DLMF 19.21.10 has only non-negative terms:
    // S = 2 pi (a c R_F(x, y, 1) + a c (1-x)(y-1)/3 R_D(x, y, 1) + b^2).
    let (inner, outer) = (b / a, b / c);
    let (x, y) = (inner * inner, outer * outer);
    let one_minus_x = (1.0 - inner) * (1.0 + inner);
    let y_minus_one = (outer - 1.0) * (outer + 1.0);
    let (rf, rd) = carlson_rf_rd(x, y, (a / c).powi(2));
    2.0 * PI * (a * c * (rf + one_minus_x * y_minus_one / 3.0 * rd) + b * b)
}

/// Carlson's `R_F(x, y, 1)` and `R_D(x, y, 1)` for `x <= 1 <= y`.
///
/// Duplication (Carlson 1995) keeps every argument at least `x`; `spread` is the host-known
/// ratio `y / x` bound. Five steps beyond the stop rule `4^m > 19 * spread` let both
/// seventh-order closing series truncate below float64 roundoff.
fn carlson_rf_rd(mut x: f64, mut y: f64, spread: f64) -> (f64, f64) {
    let steps = ((19.0 * spread).ln() / 4f64.ln()).ceil().max(0.0) as usize + 1 + 5;
    let mut z: f64 = 1.0;
    let (mut total, mut scale) = (0.0, 1.0);
    for _ in 0..steps {
        let (root_x, root_y, root_z) = (x.sqrt(), y.sqrt(), z.sqrt());
        let lambda = root_x * (root_y + root_z) + root_y * root_z;
        total += scale / (root_z * (z + lambda));
        scale /= 4.0;
        x = (x + lambda) / 4.0;
        y = (y + lambda) / 4.0;
        z = (z + lambda) / 4.0;
    }
    let mean = (x + y + z) / 3.0;
    let (dx, dy) = (1.0 - x / mean, 1.0 - y / mean);
    let dz = -(dx + dy);
    let (e2, e3) = (dx * dy - dz * dz, dx * dy * dz);
    let rf = (1.0 - e2 / 10.0 + e3 / 14.0 + e2 * e2 / 24.0 - 3.0 * e2 * e3 / 44.0) / mean.sqrt();
    let mean = (x + y + 3.0 * z) / 5.0;
    let (dx, dy) = ((mean - x) / mean, (mean - y) / mean);
    let dz = -(dx + dy) / 3.0;
    let (xy, zz) = (dx * dy, dz * dz);
    let (e2, e3) = (xy - 6.0 * zz, (3.0 * xy - 8.0 * zz) * dz);
    let (e4, e5) = (3.0 * (xy - zz) * zz, xy * zz * dz);
    let closing = 1.0 - 3.0 * e2 / 14.0 + e3 / 6.0 + 9.0 * e2 * e2 / 88.0
        - 3.0 * e4 / 22.0
        - 9.0 * e2 * e3 / 52.0
        + 3.0 * e5 / 26.0;
    let rd = 3.0 * total + scale * closing / (mean * mean.sqrt());
    (rf, rd)
}

/// The ellipsoid area series as printed in the IBSI manual, `a >= b >= c`.
///
/// `4 pi a b sum_{nu=0}^{20} (alpha beta)^nu / (1 - 4 nu^2) P_nu(x)` with
/// `alpha = sqrt(1 - b^2/a^2)`, `beta = sqrt(1 - c^2/a^2)` and
/// `x = (alpha^2 + beta^2) / (2 alpha beta)`, truncated at `nu = 20` as the manual states.
/// Its longest-axis eccentricities are not the exact surface (see
/// [`ellipsoid_area_normalized`]), but they reproduce the published IBSI 1 area density of
/// the approximate enclosing ellipsoid, so `ellipsoid_area = "ibsi-series"` uses this form
/// for that feature only.
///
/// `Q_nu = (alpha beta)^nu P_nu(x)` follows from Bonnet's recurrence as
/// `Q_{nu+1} = ((2 nu + 1) s Q_nu - nu p^2 Q_{nu-1}) / (nu + 1)` with
/// `s = (alpha^2 + beta^2) / 2` and `p = alpha beta`, which never divides by `p`; a sphere
/// gives `4 pi a^2`.
#[must_use]
pub fn ellipsoid_area_ibsi_series(a: f64, b: f64, c: f64) -> f64 {
    if c <= 0.0 {
        return 2.0 * PI * a * b;
    }
    let (inner, outer) = (b / a, c / a);
    let alpha2 = (1.0 - inner) * (1.0 + inner);
    let beta2 = (1.0 - outer) * (1.0 + outer);
    let half_sum = (alpha2 + beta2) / 2.0;
    let product2 = alpha2 * beta2;
    let (mut previous, mut current) = (1.0, half_sum);
    let mut total = previous + current / (1.0 - 4.0);
    for nu in 1..IBSI_SERIES_ORDER {
        let following = ((2 * nu + 1) as f64 * half_sum * current
            - nu as f64 * product2 * previous)
            / (nu + 1) as f64;
        previous = current;
        current = following;
        total += current / (1.0 - 4.0 * ((nu + 1) * (nu + 1)) as f64);
    }
    4.0 * PI * a * b * total
}

/// Mesh volume and area divided by an enclosing volume and area (physical units).
///
/// A non-positive enclosing measure gives infinity.
#[must_use]
pub fn enclosing_densities(geometry: &MeshGeometry, volume: f64, area: f64) -> (f64, f64) {
    let volume_density = if volume > 0.0 {
        geometry.enclosed / volume
    } else {
        f64::INFINITY
    };
    let area_density = if area > 0.0 {
        geometry.boundary / area
    } else {
        f64::INFINITY
    };
    (volume_density, area_density)
}

/// Volume and area densities for the approximate enclosing ellipsoid.
///
/// `eigen` is the ascending covariance spectrum in physical units; the semi-axes are
/// `2 sqrt(eigen)`. `printed_series` selects the IBSI manual's printed area series instead
/// of the exact surface; only the area density changes.
#[must_use]
pub fn aee_densities(
    geometry: &MeshGeometry,
    eigen: &[f64],
    needs_area: bool,
    printed_series: bool,
) -> (f64, f64) {
    let semi: Vec<f64> = eigen.iter().map(|value| 2.0 * value.sqrt()).collect();
    let (a, b, c) = (semi[semi.len() - 1], semi[semi.len() - 2], semi[0]);
    let ellipsoid_volume = (4.0 * PI / 3.0) * a * b * c;
    let ellipsoid_area = if !needs_area {
        1.0
    } else if printed_series {
        ellipsoid_area_ibsi_series(a, b, c)
    } else {
        ellipsoid_area_normalized(a, b, c)
    };
    enclosing_densities(geometry, ellipsoid_volume, ellipsoid_area)
}

#[cfg(test)]
mod tests {
    use crate::features::geometry::ellipsoids::*;

    #[test]
    fn sphere_area_all_forms() {
        let exact = 4.0 * PI;
        assert!((ellipsoid_area_normalized(1.0, 1.0, 1.0) - exact).abs() < 1e-14);
        assert!((ellipsoid_area_ibsi_series(1.0, 1.0, 1.0) - exact).abs() < 1e-14);
    }

    #[test]
    fn triaxial_area_matches_scipy_elliprg() {
        // 4 pi a b c R_G(a^-2, b^-2, c^-2) from scipy.special.elliprg (float64).
        let cases = [
            ((3.1, 3.0, 2.9), 113.08059234047874),
            ((10.0, 5.0, 1.0), 333.9460450700347),
            ((2.0, 1.5, 1.45), 34.01894977530843),
            ((30.0, 3.0, 0.3), 574.6317373755777),
            ((1.0001, 1.0, 0.9999), 12.566370597604012),
            ((7.5, 7.2, 1.0), 356.31834981757885),
        ];
        for ((a, b, c), expected) in cases {
            let value = ellipsoid_area_normalized(a, b, c);
            assert!(
                (value - expected).abs() <= 1e-12 * expected,
                "({a}, {b}, {c}): {value} vs {expected}"
            );
        }
    }

    #[test]
    fn triaxial_area_matches_the_spheroid_limit() {
        let spheroid = ellipsoid_area_normalized(2.0, 2.0, 1.0);
        let near = ellipsoid_area_normalized(2.0, 2.0 * (1.0 - 1e-9), 1.0);
        assert!((spheroid - near).abs() < 1e-6, "{spheroid} {near}");
        let prolate = ellipsoid_area_normalized(2.0, 1.0, 1.0);
        let near = ellipsoid_area_normalized(2.0, 1.0 + 1e-9, 1.0);
        assert!((prolate - near).abs() < 1e-6);
    }

    #[test]
    fn mvee_of_sampled_ellipsoid_surface_recovers_known_axes() {
        // Fibonacci-sphere samples of an ellipsoid with axes 3, 2, 1.3 under a fixed rotation
        // (mlx-radiomics test_mvee_of_sampled_ellipsoid_surface_recovers_known_axes, rtol 5e-3).
        let axes = [3.0, 2.0, 1.3];
        let count = 400;
        let (c, s) = (0.6f64.cos(), 0.6f64.sin());
        let rotation = [
            [c, -s, 0.0],
            [s * 0.8, c * 0.8, 0.6],
            [-s * 0.6, -c * 0.6, 0.8],
        ];
        let points: Vec<[f64; 3]> = (0..count)
            .map(|k| {
                let index = k as f64 + 0.5;
                let polar = (1.0 - 2.0 * index / count as f64).acos();
                let azimuth = PI * (1.0 + 5f64.sqrt()) * index;
                let unit = [
                    polar.sin() * azimuth.cos() * axes[0],
                    polar.sin() * azimuth.sin() * axes[1],
                    polar.cos() * axes[2],
                ];
                let mut p = [0.0; 3];
                for i in 0..3 {
                    p[i] = rotation[i][0] * unit[0]
                        + rotation[i][1] * unit[1]
                        + rotation[i][2] * unit[2];
                }
                p
            })
            .collect();
        let semi = mvee_semi_axes(&points).unwrap();
        for (value, expected) in semi.iter().zip([3.0, 2.0, 1.3]) {
            assert!((value / expected - 1.0).abs() < 5e-3, "{semi:?}");
        }
    }

    #[test]
    fn mvee_of_a_cube_is_a_sphere_of_radius_root_three_half() {
        let mut points = Vec::new();
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    points.push([x, y, z]);
                }
            }
        }
        let semi = mvee_semi_axes(&points).unwrap();
        for value in semi {
            assert!((value - 3f64.sqrt()).abs() < 1e-9, "{semi:?}");
        }
    }
}
