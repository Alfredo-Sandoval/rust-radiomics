//! 3-D spherical-harmonic LBP (PyRadiomics `lbp-3D`).
//!
//! Mirrors `mlxradiomics/filters/lbp3d.py` (242 lines at mlx-radiomics b58783c); cubic
//! B-spline coefficients and sampling come from [`crate::numerics::spline`].
//! Status: done: icosphere, orthonormal spherical harmonics, spherical Voronoi solid
//! angles, cubic B-spline sampling, band norms and Fisher excess kurtosis. The Python
//! code takes the icosphere from `trimesh`, the harmonics from SciPy `sph_harm_y` and the
//! Voronoi cells from `scipy.spatial.SphericalVoronoi`; here all three are computed in
//! pure Rust (icosahedron and midpoint subdivision re-projected to the sphere; the
//! standard recurrence for associated Legendre functions; the Voronoi polygon of every
//! vertex from the face circumcentres, triangulated from the vertex and summed with the
//! Van Oosterom-Strackee formula).
//!
//! Execution: CPU numerical work in float64; sampling runs per masked voxel on rayon
//! worker threads; the geometry set-up is single-threaded. Differences from the Metal
//! code, all float32 devices: no exact-power-of-two rescaling for huge float images, and
//! the roundoff ambiguity bounds use the float64 epsilon. The integer dtype limit is
//! `2**53` instead of `2**24`.
//! Input contract: a 3-D image and an aligned boolean mask, both in (z, y, x) order.

use crate::config::transforms::Lbp3DSettings;
use crate::error::{Error, Result};
use crate::filters::kernels::copy_flat;
use crate::numerics::spline;
use ndarray::ArrayD;
use rayon::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;

/// Source dtype of the original image, which decides how LBP3D samples are thresholded.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceDtype {
    /// Floating-point source: samples are compared as interpolated.
    Float,
    /// Integer source: samples round to the nearest integer (halves away from zero) and
    /// clip to `[min, max]`, mimicking integer assignment in PyRadiomics.
    Integer {
        /// Smallest representable value.
        min: i64,
        /// Largest representable value.
        max: i64,
    },
}

type V3 = [f64; 3];

fn norm(v: V3) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn unit(v: V3) -> V3 {
    let n = norm(v);
    [v[0] / n, v[1] / n, v[2] / n]
}

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Unit icosphere vertices and triangular faces after `subdivision` midpoint splits.
fn unit_icosphere(subdivision: u32) -> (Vec<V3>, Vec<[usize; 3]>) {
    let t = (1.0 + 5f64.sqrt()) / 2.0;
    let mut vertices: Vec<V3> = [
        [-1.0, t, 0.0],
        [1.0, t, 0.0],
        [-1.0, -t, 0.0],
        [1.0, -t, 0.0],
        [0.0, -1.0, t],
        [0.0, 1.0, t],
        [0.0, -1.0, -t],
        [0.0, 1.0, -t],
        [t, 0.0, -1.0],
        [t, 0.0, 1.0],
        [-t, 0.0, -1.0],
        [-t, 0.0, 1.0],
    ]
    .iter()
    .map(|&v| unit(v))
    .collect();
    let mut faces: Vec<[usize; 3]> = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    for _ in 0..subdivision {
        let mut midpoints: HashMap<(usize, usize), usize> = HashMap::new();
        let mut next_faces = Vec::with_capacity(faces.len() * 4);
        let mut midpoint = |a: usize, b: usize, vertices: &mut Vec<V3>| -> usize {
            let key = (a.min(b), a.max(b));
            *midpoints.entry(key).or_insert_with(|| {
                let (p, q) = (vertices[a], vertices[b]);
                vertices.push(unit([
                    (p[0] + q[0]) / 2.0,
                    (p[1] + q[1]) / 2.0,
                    (p[2] + q[2]) / 2.0,
                ]));
                vertices.len() - 1
            })
        };
        for &[a, b, c] in &faces {
            let ab = midpoint(a, b, &mut vertices);
            let bc = midpoint(b, c, &mut vertices);
            let ca = midpoint(c, a, &mut vertices);
            next_faces.extend([[a, ab, ca], [b, bc, ab], [c, ca, bc], [ab, bc, ca]]);
        }
        faces = next_faces;
    }
    (vertices, faces)
}

/// Area of the spherical triangle with unit-vector corners (Van Oosterom-Strackee).
fn spherical_triangle_area(a: V3, b: V3, c: V3) -> f64 {
    let triple = dot(a, cross(b, c)).abs();
    2.0 * triple.atan2(1.0 + dot(a, b) + dot(b, c) + dot(c, a))
}

/// Spherical Voronoi solid angle of every unit-sphere vertex of an icosphere.
fn voronoi_areas(vertices: &[V3], faces: &[[usize; 3]]) -> Vec<f64> {
    // The circumcentre of a face on the unit sphere is its outward unit normal.
    let centres: Vec<V3> = faces
        .iter()
        .map(|&[a, b, c]| {
            let (pa, pb, pc) = (vertices[a], vertices[b], vertices[c]);
            let sub = |p: V3, q: V3| [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
            let normal = unit(cross(sub(pb, pa), sub(pc, pa)));
            let centroid = [
                pa[0] + pb[0] + pc[0],
                pa[1] + pb[1] + pc[1],
                pa[2] + pb[2] + pc[2],
            ];
            if dot(normal, centroid) < 0.0 {
                normal.map(|x| -x)
            } else {
                normal
            }
        })
        .collect();
    let mut incident: Vec<Vec<usize>> = vec![Vec::new(); vertices.len()];
    for (face, corners) in faces.iter().enumerate() {
        for &corner in corners {
            incident[corner].push(face);
        }
    }
    vertices
        .iter()
        .enumerate()
        .map(|(index, &v)| {
            // Tangent basis at v, then order the cell corners by angle about v.
            let helper = if v[0].abs() < 0.9 {
                [1.0, 0.0, 0.0]
            } else {
                [0.0, 1.0, 0.0]
            };
            let e1 = unit(cross(v, helper));
            let e2 = cross(v, e1);
            let mut polygon: Vec<(f64, V3)> = incident[index]
                .iter()
                .map(|&face| {
                    let c = centres[face];
                    (dot(c, e2).atan2(dot(c, e1)), c)
                })
                .collect();
            polygon.sort_by(|x, y| x.0.total_cmp(&y.0));
            (0..polygon.len())
                .map(|i| {
                    spherical_triangle_area(v, polygon[i].1, polygon[(i + 1) % polygon.len()].1)
                })
                .sum()
        })
        .collect()
}

/// Orthonormal complex spherical harmonics `Y_n^m`, `m = -n..=n`, with the Condon-Shortley
/// phase (SciPy `sph_harm_y(n, m, theta, phi)`, `theta` polar and `phi` azimuth).
fn spherical_harmonics(n: usize, theta: f64, phi: f64) -> Vec<(f64, f64)> {
    let x = theta.cos();
    let s = theta.sin();
    // P_n^m(x) for m >= 0 with the Condon-Shortley phase.
    let legendre = |m: usize| -> f64 {
        let mut pmm = 1.0;
        for k in 1..=m {
            pmm *= -((2 * k - 1) as f64) * s;
        }
        if n == m {
            return pmm;
        }
        let mut previous = pmm;
        let mut current = x * (2 * m + 1) as f64 * pmm;
        for l in m + 2..=n {
            let next =
                ((2 * l - 1) as f64 * x * current - (l + m - 1) as f64 * previous) / (l - m) as f64;
            previous = current;
            current = next;
        }
        current
    };
    let mut positive = Vec::with_capacity(n + 1);
    for m in 0..=n {
        // (n - m)! / (n + m)!
        let ratio: f64 = (n - m + 1..=n + m)
            .map(|k| k as f64)
            .product::<f64>()
            .recip();
        let normal = ((2 * n + 1) as f64 / (4.0 * PI) * ratio).sqrt();
        let value = normal * legendre(m);
        positive.push((
            value * (m as f64 * phi).cos(),
            value * (m as f64 * phi).sin(),
        ));
    }
    let mut result = Vec::with_capacity(2 * n + 1);
    for m in (1..=n).rev() {
        // Y_n^{-m} = (-1)^m conj(Y_n^m)
        let sign = if m % 2 == 0 { 1.0 } else { -1.0 };
        result.push((sign * positive[m].0, -sign * positive[m].1));
    }
    result.extend(positive);
    result
}

/// Icosphere sampling geometry and spherical-harmonic basis.
struct Sphere {
    /// Vertex offsets in voxels at `radius`.
    vertices: Vec<V3>,
    /// Conjugated harmonics times the Voronoi solid angle: `[degree][vertex][m]` as (re, im).
    analysis: Vec<Vec<Vec<(f64, f64)>>>,
}

fn sphere_coefficients(subdivision: u32, radius: f64, levels: u32) -> Sphere {
    let (unit_vertices, faces) = unit_icosphere(subdivision);
    let areas = voronoi_areas(&unit_vertices, &faces);
    let angles: Vec<(f64, f64)> = unit_vertices
        .iter()
        .map(|v| {
            (
                v[2].clamp(-1.0, 1.0).acos(),
                v[1].atan2(v[0]).rem_euclid(2.0 * PI),
            )
        })
        .collect();
    let analysis = (0..levels as usize)
        .map(|n| {
            angles
                .iter()
                .zip(&areas)
                .map(|(&(theta, phi), &area)| {
                    spherical_harmonics(n, theta, phi)
                        .into_iter()
                        .map(|(re, im)| (re * area, -im * area))
                        .collect()
                })
                .collect()
        })
        .collect();
    Sphere {
        vertices: unit_vertices
            .iter()
            .map(|v| v.map(|x| x * radius))
            .collect(),
        analysis,
    }
}

/// Sample the spline at a voxel position; zero outside `[0, size - 1]` on any axis.
fn cubic_sample(coefficients: &[f64], dims: [usize; 3], position: V3) -> f64 {
    if (0..3).any(|d| position[d] < 0.0 || position[d] > dims[d] as f64 - 1.0) {
        return 0.0;
    }
    spline::evaluate_flat(coefficients, &dims, &position)
}

/// Per-center result: one band norm per degree plus the excess kurtosis.
struct CenterResult {
    bands: Vec<f64>,
    kurtosis: f64,
}

/// Compute the 3-D LBP images `lbp-3D-m1` ... `lbp-3D-m<levels>` and `lbp-3D-k`.
///
/// For every masked voxel, the image is sampled at the icosphere vertices around it
/// (radius `radius` voxels) by cubic B-spline interpolation, and each sample is compared
/// with the centre intensity (bit set where sample >= centre). With `f_j` that bit,
/// `Y_n^m` the orthonormal spherical harmonics and `dOmega_j` the spherical Voronoi solid
/// angle of the vertex, the degree-`n` coefficients are
/// `c_nm = sum_j dOmega_j f_j conj(Y_n^m(theta_j, phi_j))` and `lbp-3D-m<k>` (`k = 1..levels`,
/// degree `n = k - 1`) is the band norm `sqrt(sum_m |c_nm|^2)`. An all-set bit vector gives
/// `sqrt(4*pi)` for `m1` at every subdivision. `lbp-3D-k` is the Fisher excess kurtosis of
/// the sampled values, NaN for a constant ring. Voxels outside `mask` are 0.
///
/// Errors (`ValueError` in Python): image not 3-D or shapes differ; empty mask; a
/// non-finite spline sample; an integer dtype wider than `2**53`; an integer sample within
/// roundoff of its centre's rounding threshold `centre - 0.5`; a nonconstant floating image
/// with a sample within `64 * eps * max(|sample|, |centre|, 1)` of its centre, where
/// spline roundoff could change the bit. A constant floating image is sampled exactly at
/// its constant inside the source domain and at zero outside it.
pub fn lbp3d_images(
    image: &ArrayD<f64>,
    mask: &ArrayD<bool>,
    options: &Lbp3DSettings,
    source_dtype: SourceDtype,
) -> Result<Vec<(String, ArrayD<f64>)>> {
    if image.ndim() != 3 || image.shape() != mask.shape() {
        return Err(Error::InvalidInput(
            "LBP3D requires a 3D image and an aligned boolean mask".into(),
        ));
    }
    let dims = [image.shape()[0], image.shape()[1], image.shape()[2]];
    let strides = [dims[1] * dims[2], dims[2], 1];
    let mask_flat = mask.as_standard_layout();
    let mask_flat = mask_flat.as_slice().expect("standard layout is contiguous");
    // Masked voxels in row-major order, found one block of the mask per task.
    let centers: Vec<[usize; 3]> = mask_flat
        .par_chunks(1 << 16)
        .enumerate()
        .flat_map_iter(|(block, chunk)| {
            chunk
                .iter()
                .enumerate()
                .filter(|&(_, &masked)| masked)
                .map(move |(offset, _)| {
                    let flat = block * (1 << 16) + offset;
                    [
                        flat / strides[0],
                        flat / strides[1] % dims[1],
                        flat % dims[2],
                    ]
                })
        })
        .collect();
    if centers.is_empty() {
        return Err(Error::InvalidInput("LBP3D mask is empty".into()));
    }
    let standard = image.as_standard_layout();
    let image_flat = standard.as_slice().expect("standard layout is contiguous");
    if image_flat.par_iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput(
            "LBP3D spline samples are not finite for this image".into(),
        ));
    }
    if let SourceDtype::Integer { min, max } = source_dtype
        && min.unsigned_abs().max(max.unsigned_abs()) > 1 << 53
    {
        return Err(Error::InvalidInput(
            "LBP3D integer dtype exceeds exact float64 integer range".into(),
        ));
    }
    let sphere = sphere_coefficients(options.subdivision, options.radius, options.levels);
    let flat = copy_flat(image);
    let coefficients = copy_flat(&spline::spline_coefficients(image));
    let constant_image =
        source_dtype == SourceDtype::Float && flat.par_iter().all(|&v| v == flat[0]);
    let eps = f64::EPSILON;
    let levels = options.levels as usize;
    let results: Vec<CenterResult> = centers
        .par_iter()
        .map(|center| -> Result<CenterResult> {
            let original = flat[center[0] * strides[0] + center[1] * strides[1] + center[2]];
            let mut samples = Vec::with_capacity(sphere.vertices.len());
            let mut binary = Vec::with_capacity(sphere.vertices.len());
            for offset in &sphere.vertices {
                let position = [
                    center[0] as f64 + offset[0],
                    center[1] as f64 + offset[1],
                    center[2] as f64 + offset[2],
                ];
                let mut sample = cubic_sample(&coefficients, dims, position);
                if !sample.is_finite() {
                    return Err(Error::InvalidInput(
                        "LBP3D spline samples are not finite for this image".into(),
                    ));
                }
                let bit;
                if constant_image {
                    let inside = (0..3).all(|d| position[d] >= 0.0 && position[d] <= dims[d] as f64 - 1.0);
                    sample = if inside { original } else { 0.0 };
                    bit = inside || original <= 0.0;
                } else {
                    let bound = 64.0 * eps * sample.abs().max(original.abs()).max(1.0);
                    match source_dtype {
                        SourceDtype::Integer { min, max } => {
                            if (sample - (original - 0.5)).abs() <= bound {
                                return Err(Error::InvalidInput(
                                    "LBP3D integer rounding threshold is ambiguous at float64 precision; spline roundoff can change its binary code"
                                        .into(),
                                ));
                            }
                            sample = (sample.signum() * (sample.abs() + 0.5).floor())
                                .clamp(min as f64, max as f64);
                        }
                        SourceDtype::Float => {
                            if (sample - original).abs() <= bound {
                                return Err(Error::InvalidInput(
                                    "LBP3D threshold is ambiguous at float64 precision; source spline roundoff can change its binary code"
                                        .into(),
                                ));
                            }
                        }
                    }
                    bit = sample >= original;
                }
                samples.push(sample);
                binary.push(f64::from(u8::from(bit)));
            }
            // Centre before reduction so a constant ring has exactly zero variance.
            let first = samples[0];
            let mut offsets: Vec<f64> = samples.iter().map(|s| s - first).collect();
            let amplitude = offsets.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
            if amplitude > 0.0 {
                offsets.iter_mut().for_each(|x| *x /= amplitude);
            }
            let n = offsets.len() as f64;
            let mean = offsets.iter().sum::<f64>() / n;
            let centered: Vec<f64> = offsets.iter().map(|x| x - mean).collect();
            let variance = centered.iter().map(|x| x * x).sum::<f64>() / n;
            let kurtosis = if variance == 0.0 {
                f64::NAN
            } else {
                centered.iter().map(|x| x.powi(4)).sum::<f64>() / n / (variance * variance) - 3.0
            };
            let bands = (0..levels)
                .map(|degree| {
                    let columns = 2 * degree + 1;
                    let (mut power, mut re, mut im) = (0.0, vec![0.0; columns], vec![0.0; columns]);
                    for (j, &bit) in binary.iter().enumerate() {
                        if bit != 0.0 {
                            for m in 0..columns {
                                re[m] += sphere.analysis[degree][j][m].0;
                                im[m] += sphere.analysis[degree][j][m].1;
                            }
                        }
                    }
                    for m in 0..columns {
                        power += re[m] * re[m] + im[m] * im[m];
                    }
                    power.sqrt()
                })
                .collect();
            Ok(CenterResult { bands, kurtosis })
        })
        .collect::<Result<Vec<_>>>()?;
    let scatter = |values: Vec<f64>| -> ArrayD<f64> {
        let mut output = vec![0.0; dims.iter().product()];
        for (center, value) in centers.iter().zip(values) {
            output[center[0] * strides[0] + center[1] * strides[1] + center[2]] = value;
        }
        ArrayD::from_shape_vec(image.raw_dim(), output).expect("shape")
    };
    let mut images = Vec::with_capacity(levels + 1);
    for degree in 0..levels {
        images.push((
            format!("lbp-3D-m{}", degree + 1),
            scatter(results.iter().map(|r| r.bands[degree]).collect()),
        ));
    }
    images.push((
        "lbp-3D-k".to_string(),
        scatter(results.iter().map(|r| r.kurtosis).collect()),
    ));
    Ok(images)
}
