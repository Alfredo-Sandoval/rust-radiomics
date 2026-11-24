//! Local and global intensity peaks over a physical 1 cm³ sphere (IBSI VJGA and 0F91).
//!
//! CPU port of `mlxradiomics/features/local_intensity.py`. Large global-peak
//! searches distribute independent image planes across the Rayon thread pool.
//! Sphere means accumulate in double precision. Final values follow the reference
//! implementation's single-precision output.

use ndarray::{ArrayView3, ArrayViewD, Axis, Ix3, s};
use rayon::prelude::*;
use std::ops::Range;

use crate::{Error, Features, Result};

const LOCAL: &str = "LocalIntensityPeak";
const GLOBAL: &str = "GlobalIntensityPeak";

/// Compute full `localintensity_*` ROI keys in caller order on the CPU.
pub fn compute(
    data: &crate::Prepared,
    spacing: &[f64],
    settings: &crate::Settings,
    keys: &[&str],
) -> Result<Features> {
    if keys.is_empty() {
        return Ok(Vec::new());
    }
    let names: Result<Vec<_>> = keys
        .iter()
        .map(|key| {
            key.strip_prefix("localintensity_").ok_or_else(|| {
                Error::InvalidInput(format!("expected a localintensity_ feature, got {key}"))
            })
        })
        .collect();
    local_intensity_statistics(
        data.input.image.view(),
        data.input.roi.view(),
        spacing,
        settings.scratch_mib,
        &names?,
    )
}

/// Compute the selected peaks in request order using millimetre spacing in (z, y, x).
/// Supply the short feature names `LocalIntensityPeak` or `GlobalIntensityPeak`.
/// The ROI selects sphere centers. Each mean includes every in-image sample of
/// its sphere, including voxels outside the ROI. An empty ROI yields negative infinity.
pub fn local_intensity_statistics(
    image: ArrayViewD<'_, f64>,
    mask: ArrayViewD<'_, bool>,
    spacing: &[f64],
    scratch_mib: u32,
    selected: &[&str],
) -> Result<Features> {
    if image.shape() != mask.shape() || image.ndim() != 3 || image.is_empty() {
        return Err(Error::InvalidInput(
            "local intensity requires matching, nonempty 3D image and mask shapes".into(),
        ));
    }
    let spacing: [f64; 3] = spacing
        .try_into()
        .ok()
        .filter(|steps: &[f64; 3]| steps.iter().all(|s| s.is_finite() && *s > 0.0))
        .ok_or_else(|| {
            Error::InvalidInput("expected three positive finite millimetre spacings".into())
        })?;
    let mut names = Vec::with_capacity(2);
    for &name in selected {
        if !matches!(name, LOCAL | GLOBAL) {
            return Err(Error::InvalidInput(format!(
                "unknown local intensity feature {name}"
            )));
        }
        if !names.contains(&name) {
            names.push(name);
        }
    }
    if names.is_empty() {
        return Ok(Vec::new());
    }
    let image = image
        .into_dimensionality::<Ix3>()
        .expect("validated 3D image");
    let mask = mask
        .into_dimensionality::<Ix3>()
        .expect("validated matching mask");
    let (depth, height, width) = image.dim();
    let sphere = Sphere::new([depth, height, width], spacing, scratch_mib)?;
    let want_local = names.contains(&LOCAL);
    let want_global = names.contains(&GLOBAL);
    let mut brightest = f64::NEG_INFINITY;
    if want_local {
        for (index, &inside) in mask.indexed_iter() {
            if inside {
                let value = image[index];
                if !value.is_finite() {
                    return Err(invalid_sample());
                }
                brightest = brightest.max(value);
            }
        }
    }
    let prefix = if want_global && mask.len() >= 32 * 1024 {
        mask.as_slice().and_then(|centers| {
            let values = image.as_slice()?;
            let rows = sphere.integer_row_sums(&image, scratch_mib)?;
            Some((centers, values, rows))
        })
    } else {
        None
    };
    let scan = |planes: Range<usize>| -> Result<[f64; 2]> {
        if let Some((centers, values, rows)) = &prefix {
            return Ok(sphere.integer_peaks(
                rows,
                centers,
                values,
                [height, width],
                planes,
                brightest,
            ));
        }
        let (mut local, mut global) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for z in planes {
            for ((y, x), &inside) in mask.index_axis(Axis(0), z).indexed_iter() {
                if !inside {
                    continue;
                }
                let index = (z, y, x);
                let is_brightest = want_local && image[index] == brightest;
                if want_global || is_brightest {
                    let mean = sphere.mean(&image, index)?;
                    global = global.max(mean);
                    if is_brightest {
                        local = local.max(mean);
                    }
                }
            }
        }
        Ok([local, global])
    };
    // Local-only searches usually evaluate very few spheres. Keep them and small
    // volumes on the caller's thread to avoid scheduling work for each plane.
    let [local, global] = if want_global && depth > 1 && mask.len() >= 32 * 1024 {
        (0..depth)
            .into_par_iter()
            .map(|z| scan(z..z + 1))
            .try_reduce(
                || [f64::NEG_INFINITY; 2],
                |[local, global], [other_local, other_global]| {
                    Ok([local.max(other_local), global.max(other_global)])
                },
            )?
    } else {
        scan(0..depth)?
    };
    Ok(names
        .into_iter()
        .map(|name| {
            let peak = if name == LOCAL { local } else { global };
            (format!("localintensity_{name}"), f64::from(peak as f32))
        })
        .collect())
}

fn invalid_sample() -> Error {
    Error::InvalidInput("local-intensity spheres require finite float32-range samples".into())
}

struct Sphere {
    half: [usize; 3],
    // The x reach is symmetric in z and y. A missing row uses usize::MAX.
    reaches: Vec<usize>,
}

impl Sphere {
    fn new(shape: [usize; 3], spacing: [f64; 3], scratch_mib: u32) -> Result<Self> {
        let radius = (3000.0 / (4.0 * std::f64::consts::PI)).powf(1.0 / 3.0);
        let half: [usize; 3] =
            std::array::from_fn(|i| ((radius / spacing[i]) as usize).min(shape[i] - 1));
        let budget = u128::from(scratch_mib) << 20;
        let box_size = half.iter().map(|&h| 2 * h as u128 + 1).product::<u128>();
        if box_size * 8 > budget {
            return Err(Error::InvalidInput(
                "scratch_mib cannot hold the physical local-intensity sphere".into(),
            ));
        }
        let mut reaches = Vec::with_capacity((half[0] + 1) * (half[1] + 1));
        for dz in 0..=half[0] {
            for dy in 0..=half[1] {
                let yz_squared =
                    (dz as f64 * spacing[0]).powi(2) + (dy as f64 * spacing[1]).powi(2);
                if yz_squared > radius * radius {
                    reaches.push(usize::MAX);
                    continue;
                }
                // Binary search tests the same squared distance at the sphere boundary.
                let (mut lo, mut hi) = (0, half[2] + 1);
                while hi - lo > 1 {
                    let mid = lo + (hi - lo) / 2;
                    if yz_squared + (mid as f64 * spacing[2]).powi(2) <= radius * radius {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                reaches.push(lo);
            }
        }
        Ok(Self { half, reaches })
    }

    // A range sum of an integer row is exact when every partial sum in the image
    // fits in the 53-bit significand. Otherwise use the compensated sphere sum.
    fn integer_row_sums(&self, image: &ArrayView3<'_, f64>, scratch_mib: u32) -> Option<Vec<f64>> {
        let values = image.as_slice()?;
        let (nz, ny, nx) = image.dim();
        let len = nz.checked_mul(ny)?.checked_mul(nx.checked_add(1)?)?;
        let workers = nz.min(rayon::current_num_threads()) as u128;
        let bytes = (len as u128 + self.reaches.len() as u128 + 2 * nx as u128 * workers) * 8;
        if bytes > u128::from(scratch_mib) << 20 {
            return None;
        }
        // Integer division rounds down before conversion, including at the exactness limit.
        let limit = ((1u64 << 53) / values.len() as u64) as f64;
        if !values.iter().all(|&x| x.abs() <= limit && x.trunc() == x) {
            return None;
        }
        let mut rows = Vec::with_capacity(len);
        for row in values.chunks_exact(nx) {
            let mut sum = 0.;
            rows.push(sum);
            for &value in row {
                sum += value;
                rows.push(sum);
            }
        }
        Some(rows)
    }

    // Neighboring centers use shifted windows of the same prefix row. Process a
    // whole run together so the interior needs only two linear loads per center.
    fn integer_peaks(
        &self,
        rows: &[f64],
        mask: &[bool],
        image: &[f64],
        [height, width]: [usize; 2],
        planes: Range<usize>,
        brightest: f64,
    ) -> [f64; 2] {
        let depth = mask.len() / (width * height);
        let [hz, hy, _] = self.half;
        let mut sums = vec![0.; width];
        let mut clipped_counts = vec![0usize; width];
        let (mut local, mut global) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for z in planes {
            for y in 0..height {
                let base = (z * height + y) * width;
                let centers = &mask[base..base + width];
                let Some(first) = centers.iter().position(|&x| x) else {
                    continue;
                };
                let end = centers.iter().rposition(|&x| x).unwrap() + 1;
                let sums = &mut sums[..end - first];
                let counts = &mut clipped_counts[..end - first];
                sums.fill(0.);
                counts.fill(0);
                let mut common_count = 0;
                for zz in z.saturating_sub(hz)..=z.saturating_add(hz).min(depth - 1) {
                    let reach_row = zz.abs_diff(z) * (hy + 1);
                    for yy in y.saturating_sub(hy)..=y.saturating_add(hy).min(height - 1) {
                        let r = self.reaches[reach_row + yy.abs_diff(y)];
                        if r == usize::MAX {
                            continue;
                        }
                        let row = (zz * height + yy) * (width + 1);
                        if first >= r && end <= width - r {
                            let left = &rows[row + first - r..row + end - r];
                            let right = &rows[row + first + r + 1..row + end + r + 1];
                            for ((sum, &lo), &hi) in sums.iter_mut().zip(left).zip(right) {
                                *sum += hi - lo;
                            }
                            common_count += 2 * r + 1;
                        } else {
                            for (i, x) in (first..end).enumerate() {
                                let lo = x.saturating_sub(r);
                                let hi = x.saturating_add(r).min(width - 1) + 1;
                                sums[i] += rows[row + hi] - rows[row + lo];
                                counts[i] += hi - lo;
                            }
                        }
                    }
                }
                for (i, x) in (first..end).enumerate() {
                    if centers[x] {
                        let mean = sums[i] / (common_count + counts[i]) as f64;
                        global = global.max(mean);
                        if image[base + x] == brightest {
                            local = local.max(mean);
                        }
                    }
                }
            }
        }
        [local, global]
    }

    fn mean(&self, image: &ArrayView3<'_, f64>, (z, y, x): (usize, usize, usize)) -> Result<f64> {
        let (depth, height, width) = image.dim();
        let [hz, hy, _] = self.half;
        let flat = image.as_slice();
        let mut sum = crate::numerics::reduction::CompensatedSum::default();
        let mut count = 0usize;
        let mut add = |value: f64| -> Result<()> {
            if !(value as f32).is_finite() {
                return Err(invalid_sample());
            }
            sum.add(value);
            Ok(())
        };
        for zz in z.saturating_sub(hz)..=z.saturating_add(hz).min(depth - 1) {
            let row = zz.abs_diff(z) * (hy + 1);
            for yy in y.saturating_sub(hy)..=y.saturating_add(hy).min(height - 1) {
                let reach = self.reaches[row + yy.abs_diff(y)];
                if reach == usize::MAX {
                    continue;
                }
                let first = x.saturating_sub(reach);
                let last = x.saturating_add(reach).min(width - 1);
                count += last - first + 1;
                if let Some(values) = flat {
                    let start = (zz * height + yy) * width;
                    for &value in &values[start + first..=start + last] {
                        add(value)?;
                    }
                } else {
                    for &value in image.slice(s![zz, yy, first..=last]) {
                        add(value)?;
                    }
                }
            }
        }
        Ok(sum.total() / count as f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::local_intensity::*;
    use ndarray::{Array3, ArrayView1};

    fn line(values: &[f64], mask: &[bool], selected: &[&str]) -> Result<Features> {
        let image = ArrayView1::from(values)
            .into_shape_with_order((1, 1, values.len()))
            .unwrap();
        let mask = ArrayView1::from(mask)
            .into_shape_with_order((1, 1, mask.len()))
            .unwrap();
        local_intensity_statistics(
            image.into_dyn(),
            mask.into_dyn(),
            &[10., 10., 6.],
            1,
            selected,
        )
    }

    #[test]
    fn matches_mlx_1_1_0_on_triaxial_grid() {
        let mut image = Array3::from_shape_fn((4, 5, 6), |(z, y, x)| {
            let i = (z * 5 + y) * 6 + x;
            (i * 37 % 113) as f64 - 56.0
        });
        let mask = Array3::from_shape_fn(image.dim(), |(z, y, x)| {
            let i = (z * 5 + y) * 6 + x;
            i % 7 == 0 || i == 119
        });
        // Gold values from mlxradiomics.extract 1.1.0 on this same fixture.
        for (tied, expected) in [
            (false, [-7.307692527770996, 8.833333015441895]),
            (true, [13.230769157409668; 2]),
        ] {
            if tied {
                image[[0, 0, 0]] = 60.;
                image[[3, 4, 5]] = 60.;
            }
            let features = local_intensity_statistics(
                image.view().into_dyn(),
                mask.view().into_dyn(),
                &[3., 2., 4.],
                1,
                &[LOCAL, GLOBAL],
            )
            .unwrap();
            assert_eq!(
                features,
                vec![
                    ("localintensity_LocalIntensityPeak".into(), expected[0]),
                    ("localintensity_GlobalIntensityPeak".into(), expected[1]),
                ]
            );
        }
    }

    #[test]
    fn brightest_uses_source_precision_and_checks_all_ties() {
        let center = [true, false, false, true];
        let peaks = line(
            &[1. + f64::EPSILON, -9., 7., 1.],
            &center,
            &[GLOBAL, LOCAL, GLOBAL],
        )
        .unwrap();
        assert_eq!(
            peaks,
            vec![
                ("localintensity_GlobalIntensityPeak".into(), 4.),
                ("localintensity_LocalIntensityPeak".into(), -4.)
            ]
        );
        let tied = line(&[-0., -8., 6., 0.], &center, &[LOCAL]).unwrap();
        assert_eq!(tied[0].1, 3.);
    }

    #[test]
    fn validates_only_selected_spheres_and_retains_cancellation_residuals() {
        let mask = [false, true, false, false, true, false];
        for invalid in [f64::NAN, f64::INFINITY, 1e100] {
            let image = [1e30, 1., -1e30, 2., 0., invalid];
            let peak = line(&image, &mask, &[LOCAL]).unwrap();
            assert_eq!(peak[0].1, f64::from(1_f32 / 3.));
            assert!(line(&image, &mask, &[GLOBAL]).is_err());
        }
    }

    #[test]
    fn checks_spacing_geometry_budget_and_empty_mask() {
        let image = Array3::from_elem((1, 1, 2), 1.);
        let mask = Array3::from_elem(image.dim(), false);
        let run = |spacing: &[f64], budget| {
            local_intensity_statistics(
                image.view().into_dyn(),
                mask.view().into_dyn(),
                spacing,
                budget,
                &[LOCAL],
            )
        };
        for spacing in [&[1., 0., 1.][..], &[1., f64::NAN, 1.], &[1., 1.]] {
            assert!(run(spacing, 1).is_err());
        }
        assert!(run(&[1.; 3], 0).is_err());
        assert_eq!(run(&[1.; 3], 1).unwrap()[0].1, f64::NEG_INFINITY);
    }

    #[test]
    fn global_search_reduces_tied_centers_and_clipped_spheres_across_planes() {
        let mut image = Array3::from_elem((32, 32, 32), -6.);
        let mut mask = Array3::from_elem(image.dim(), false);
        for (z, left, right) in [(0, 1., 2.), (15, 6., 6.)] {
            image[[z, 1, 1]] = left;
            image[[z, 1, 2]] = 9.;
            image[[z, 1, 3]] = right;
            mask[[z, 1, 2]] = true;
        }
        image[[31, 1, 30]] = 18.;
        image[[31, 1, 31]] = 8.;
        mask[[31, 1, 31]] = true;
        let peaks = local_intensity_statistics(
            image.slice(s![..;-1, .., ..;-1]).into_dyn(),
            mask.slice(s![..;-1, .., ..;-1]).into_dyn(),
            &[8., 8., 6.],
            1,
            &[LOCAL, GLOBAL],
        )
        .unwrap();
        assert_eq!([peaks[0].1, peaks[1].1], [7., 13.]);
    }

    #[test]
    fn strided_views_follow_voxel_coordinates() {
        let image = Array3::from_shape_vec((1, 1, 5), vec![1., 20., 4., 0., -3.]).unwrap();
        let mask =
            Array3::from_shape_vec((1, 1, 5), vec![false, true, false, false, true]).unwrap();
        let peaks = local_intensity_statistics(
            image.slice(s![..,..,..;-1]).into_dyn(),
            mask.slice(s![..,..,..;-1]).into_dyn(),
            &[10., 10., 6.],
            1,
            &[LOCAL, GLOBAL],
        )
        .unwrap();
        assert_eq!([peaks[0].1, peaks[1].1], [f64::from(25_f32 / 3.); 2]);
    }
}
