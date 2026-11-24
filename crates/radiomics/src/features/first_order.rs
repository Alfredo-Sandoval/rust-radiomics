//! First-order ROI statistics and voxel maps from aligned arrays.
//!

pub use crate::native_first_order::{FirstOrderFeature, FirstOrderSettings, Kurtosis};

use crate::config::settings::{Kurtosis as ConfigKurtosis, ZeroVarianceMoments};
use crate::features::first_order_source::{check_samples, uses_shift};
use crate::native_first_order::Calculation;
use crate::numerics::pair_arithmetic::{FloatPair, subtract_pairs};
use crate::numerics::precision::QuantileRank;
use crate::numerics::reduction::reported_mean;
use crate::{Error, Features, Prepared, Result, Settings};
use ndarray::{ArrayD, ArrayView2, ArrayViewD, Slice};
use std::collections::BTreeMap;

/// Scratch histogram for a single ROI or neighborhood. Allocate dense counts once
/// when the lattice is reasonably compact, otherwise store only the observed levels.
#[derive(Clone)]
struct LevelCounts {
    levels: u32,
    dense: Vec<usize>,
    occupied_low: u64,
    occupied_high: Vec<usize>,
    sparse: BTreeMap<i32, usize>,
    total: usize,
}

impl LevelCounts {
    fn new(levels: u32, dense_limit: usize) -> Self {
        let dense = if levels as usize <= dense_limit {
            vec![0; levels as usize]
        } else {
            Vec::new()
        };
        Self {
            levels,
            dense,
            occupied_low: 0,
            occupied_high: Vec::new(),
            sparse: BTreeMap::new(),
            total: 0,
        }
    }
    fn add(&mut self, level: i32) -> Result<()> {
        if level <= 0 || level as u32 > self.levels {
            return Err(Error::InvalidInput(format!(
                "selected gray level {level} is outside 1..={}",
                self.levels
            )));
        }
        if self.dense.is_empty() {
            *self.sparse.entry(level).or_default() += 1;
        } else {
            let i = level as usize - 1;
            if i < u64::BITS as usize {
                self.occupied_low |= 1 << i;
            } else if self.dense[i] == 0 {
                self.occupied_high.push(i);
            }
            self.dense[i] += 1;
        }
        self.total += 1;
        Ok(())
    }
    fn drain_dense(&mut self, mut count: impl FnMut(usize)) {
        // Iterate occupied levels in ascending order to preserve floating-point reductions.
        let mut low = std::mem::take(&mut self.occupied_low);
        while low != 0 {
            let i = low.trailing_zeros() as usize;
            low &= low - 1;
            count(std::mem::take(&mut self.dense[i]));
        }
        self.occupied_high.sort_unstable();
        for i in self.occupied_high.drain(..) {
            count(std::mem::take(&mut self.dense[i]));
        }
    }
    fn clear(&mut self) {
        self.drain_dense(|_| ());
        self.sparse.clear();
        self.total = 0;
    }
    fn finish(&mut self, entropy: bool, uniformity: bool) -> [f64; 2] {
        let total = self.total;
        // The reference applies the minus sign after reducing p * log2(p).
        // A constant (or empty) histogram therefore produces IEEE negative zero.
        let (mut sum_p_log_p, mut u) = (0., 0.);
        let mut count = |n: usize| {
            if entropy {
                sum_p_log_p -= crate::numerics::count_statistics::bin_entropy(n, total);
            }
            if uniformity {
                u += (n as f64 / total as f64).powi(2);
            }
        };
        self.drain_dense(&mut count);
        for &n in self.sparse.values() {
            count(n);
        }
        self.sparse.clear();
        self.total = 0;
        [-sum_p_log_p, if total == 0 { f64::NAN } else { u }]
    }
}

fn family_names<'a>(keys: &[&'a str]) -> Result<Vec<&'a str>> {
    keys.iter()
        .map(|key| {
            key.strip_prefix("firstorder_").ok_or_else(|| {
                Error::InvalidInput(format!("expected a firstorder_ feature, got {key}"))
            })
        })
        .collect()
}

/// Compute the selected full `firstorder_*` ROI keys in caller order.
pub fn compute(
    data: &Prepared,
    spacing: &[f64],
    settings: &Settings,
    keys: &[&str],
) -> Result<Features> {
    first_order_roi(data, spacing, settings, &family_names(keys)?)
}

/// First-order ROI values on the calling CPU thread from a prepared 2D or 3D image.
/// Spacing follows the image axes, in millimetres. Supply short feature names in
/// output order. Histogram features reuse the supplied lattice. All other features
/// operate on the original float64 ROI samples and report float32-compatible values.
pub fn first_order_roi(
    data: &crate::Prepared,
    spacing: &[f64],
    settings: &crate::Settings,
    selected: &[&str],
) -> Result<Features> {
    let mut kernel = RegionKernel::new(data, spacing, settings, selected)?;
    let values = kernel.evaluate(
        data.input.image.view(),
        data.input.roi.view(),
        data.bins.as_ref().map(ArrayD::view),
    )?;
    Ok(kernel
        .selected
        .iter()
        .map(|&f| (format!("firstorder_{}", f.short_name()), values[f as usize]))
        .collect())
}

// Each worker owns the same contiguous center range across all feature columns.

#[derive(Clone)]
struct FlatGrid<'a> {
    image: &'a [f64],
    mask: &'a [bool],
    bins: Option<&'a [i32]>,
}

/// CPU first-order evaluator for streaming voxel batches. It borrows the prepared
/// image and retains working storage for one neighborhood. Results occupy one
/// column per selected feature and one row per supplied ROI center.
#[derive(Clone)]
pub struct FirstOrderMapKernel<'a> {
    data: &'a Prepared,
    region: RegionKernel,
    lattice: Option<&'a ArrayD<i32>>,
    flat: Option<FlatGrid<'a>>,
    axes: [usize; 3],
    pad: usize,
    radius: [usize; 3],
}

impl std::fmt::Debug for FirstOrderMapKernel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirstOrderMapKernel")
            .finish_non_exhaustive()
    }
}

impl<'a> FirstOrderMapKernel<'a> {
    /// Prepare a reusable kernel from full `firstorder_*` feature keys.
    pub fn new(
        data: &'a Prepared,
        spacing: &[f64],
        settings: &Settings,
        keys: &[&str],
    ) -> Result<Self> {
        Self::from_names(data, spacing, settings, &family_names(keys)?)
    }

    fn from_names(
        data: &'a Prepared,
        spacing: &[f64],
        settings: &Settings,
        names: &[&str],
    ) -> Result<Self> {
        let region = RegionKernel::new(data, spacing, settings, names)?;
        let shape = data.input.image.shape();
        if data.input.sampling_mask.shape() != shape
            || settings.force2d_axis.is_some_and(|a| a >= shape.len())
        {
            return Err(Error::InvalidInput(
                "invalid sampling mask or force2d axis for first-order maps".into(),
            ));
        }
        let mut axes = [1; 3];
        let pad = 3 - shape.len();
        axes[pad..].copy_from_slice(shape);
        let radius = std::array::from_fn(|a| {
            if a < pad || settings.force2d_axis == Some(a - pad) {
                0
            } else {
                settings.kernel_radius as usize
            }
        });
        let lattice = data.bins.as_ref().filter(|_| region.histogram.is_some());
        let flat = data
            .input
            .image
            .as_slice()
            .zip(data.input.sampling_mask.as_slice())
            .and_then(|(image, mask)| {
                let bins = lattice.and_then(|b| b.as_slice());
                (bins.is_some() || lattice.is_none()).then_some(FlatGrid { image, mask, bins })
            });
        Ok(Self {
            data,
            region,
            lattice,
            flat,
            axes,
            pad,
            radius,
        })
    }

    /// Evaluate rows of (y, x) or (z, y, x) voxel coordinates in caller order.
    /// Duplicate centers give duplicate values. An invalid center rejects the batch.
    pub fn compute_batch(&mut self, centers: ArrayView2<'_, i32>) -> Result<Vec<Vec<f64>>> {
        if centers.ncols() != 3 - self.pad {
            return Err(Error::InvalidInput(
                "batch center dimension does not match the image".into(),
            ));
        }
        let mut columns: Vec<_> = self
            .region
            .selected
            .iter()
            .map(|_| Vec::with_capacity(centers.nrows()))
            .collect();
        for row in centers.rows() {
            let mut center = [0; 3];
            for (axis, &x) in row.iter().enumerate() {
                if x < 0 || x as usize >= self.axes[self.pad + axis] {
                    return Err(Error::InvalidInput(
                        "first-order batch center is outside the image".into(),
                    ));
                }
                center[self.pad + axis] = x as usize;
            }
            if !self.data.input.roi[&center[self.pad..]] {
                return Err(Error::InvalidInput(
                    "first-order batch center is outside the ROI".into(),
                ));
            }
            match self.at(center) {
                Ok(values) => {
                    for (&f, col) in self.region.selected.iter().zip(&mut columns) {
                        col.push(values[f as usize]);
                    }
                }
                Err(error) => {
                    self.region.samples.clear();
                    if let Some(counts) = &mut self.region.histogram {
                        counts.clear();
                    }
                    return Err(error);
                }
            }
        }
        Ok(columns)
    }

    fn at(&mut self, center: [usize; 3]) -> Result<[f64; FirstOrderFeature::ALL.len()]> {
        let ranges: [_; 3] = std::array::from_fn(|a| {
            center[a].saturating_sub(self.radius[a])
                ..center[a]
                    .saturating_add(self.radius[a])
                    .saturating_add(1)
                    .min(self.axes[a])
        });
        if let Some(grid) = &self.flat {
            let [_, ny, nx] = self.axes;
            let [zs, ys, xs] = ranges;
            for z in zs {
                for y in ys.clone() {
                    let base = (z * ny + y) * nx;
                    let row = base + xs.start..base + xs.end;
                    self.region.gather_flat(
                        &grid.image[row.clone()],
                        &grid.mask[row.clone()],
                        grid.bins.map(|b| &b[row]),
                    )?;
                }
            }
            self.region.finish()
        } else {
            let window = |a: ndarray::AxisDescription| {
                Slice::from(ranges[self.pad + a.axis.index()].clone())
            };
            self.region.evaluate(
                self.data.input.image.slice_each_axis(window),
                self.data.input.sampling_mask.slice_each_axis(window),
                self.lattice.map(|b| b.slice_each_axis(window)),
            )
        }
    }
}

// Use slice search to skip long background runs on full-volume masks.
fn selected_indices(mask: &[bool]) -> impl Iterator<Item = usize> + '_ {
    const BLOCK: usize = 32;
    mask.chunks(BLOCK)
        .enumerate()
        .filter(|(_, block)| block.contains(&true))
        .flat_map(|(k, block)| {
            block
                .iter()
                .enumerate()
                .filter_map(move |(i, &inside)| inside.then_some(k * BLOCK + i))
        })
}

#[derive(Clone)]
struct RegionKernel {
    selected: Vec<FirstOrderFeature>,
    arithmetic: Option<SampleStatistics>,
    entropy: bool,
    uniformity: bool,
    histogram: Option<LevelCounts>,
    samples: Vec<f64>,
}

impl RegionKernel {
    fn new(data: &Prepared, spacing: &[f64], settings: &Settings, names: &[&str]) -> Result<Self> {
        let image = &data.input.image;
        if image.shape() != data.input.roi.shape()
            || !(2..=3).contains(&image.ndim())
            || spacing.len() != image.ndim()
            || spacing.iter().any(|v| !v.is_finite() || *v <= 0.)
        {
            return Err(Error::InvalidInput("first-order extraction requires aligned 2D or 3D arrays and positive finite spacing".into()));
        }
        let mut selected = Vec::new();
        for &name in names {
            let feature = FirstOrderFeature::from_short_name(name).ok_or_else(|| {
                Error::InvalidInput(format!("unknown first-order feature {name}"))
            })?;
            if !selected.contains(&feature) {
                selected.push(feature);
            }
        }
        let entropy = selected.contains(&FirstOrderFeature::Entropy);
        let uniformity = selected.contains(&FirstOrderFeature::Uniformity);
        if entropy || uniformity {
            let bins = data.bins.as_ref().ok_or_else(|| {
                Error::InvalidInput("first-order histogram bins are required".into())
            })?;
            if bins.shape() != image.shape() || data.gray_levels == 0 {
                return Err(Error::InvalidInput(
                    "invalid first-order histogram lattice".into(),
                ));
            }
        }
        let features: Vec<_> = selected
            .iter()
            .copied()
            .filter(|f| {
                !matches!(
                    f,
                    FirstOrderFeature::Entropy | FirstOrderFeature::Uniformity
                )
            })
            .collect();
        let arithmetic = if features.is_empty() {
            None
        } else {
            Some(SampleStatistics::new(features, spacing, settings)?)
        };
        Ok(Self {
            selected,
            entropy,
            uniformity,
            arithmetic,
            histogram: (entropy || uniformity).then(|| {
                LevelCounts::new(
                    data.gray_levels,
                    image.len().min(
                        (settings.scratch_mib as usize).saturating_mul(1024 * 1024)
                            / size_of::<usize>(),
                    ),
                )
            }),
            samples: Vec::new(),
        })
    }

    fn evaluate(
        &mut self,
        image: ArrayViewD<'_, f64>,
        mask: ArrayViewD<'_, bool>,
        bins: Option<ArrayViewD<'_, i32>>,
    ) -> Result<[f64; FirstOrderFeature::ALL.len()]> {
        if let (Some(im), Some(m)) = (image.as_slice(), mask.as_slice())
            && (self.histogram.is_none() || bins.as_ref().and_then(|b| b.as_slice()).is_some())
        {
            self.gather_flat(im, m, bins.as_ref().and_then(|b| b.as_slice()))?;
            return self.finish();
        }
        if let Some(counts) = &mut self.histogram {
            let levels = bins.expect("validated histogram lattice");
            for (&level, &inside) in levels.iter().zip(mask.iter()) {
                if inside {
                    counts.add(level)?;
                }
            }
        }
        if self.arithmetic.is_some() {
            self.samples.extend(
                image
                    .iter()
                    .zip(mask.iter())
                    .filter_map(|(&v, &m)| m.then_some(v)),
            );
        }
        self.finish()
    }

    fn gather_flat(&mut self, image: &[f64], mask: &[bool], bins: Option<&[i32]>) -> Result<()> {
        if self.histogram.is_none() && self.arithmetic.is_some() {
            for (block, values) in mask.chunks(64).zip(image.chunks(64)) {
                if !block.contains(&false) {
                    self.samples.extend_from_slice(values);
                } else if block.contains(&true) {
                    self.samples.extend(
                        values
                            .iter()
                            .zip(block)
                            .filter_map(|(&v, &inside)| inside.then_some(v)),
                    );
                }
            }
            return Ok(());
        }
        for i in selected_indices(mask) {
            if self.arithmetic.is_some() {
                self.samples.push(image[i]);
            }
            if let (Some(counts), Some(levels)) = (&mut self.histogram, bins) {
                counts.add(levels[i])?;
            }
        }
        Ok(())
    }

    fn finish(&mut self) -> Result<[f64; FirstOrderFeature::ALL.len()]> {
        let mut values = [0.; FirstOrderFeature::ALL.len()];
        if let Some(counts) = &mut self.histogram {
            let [e, u] = counts.finish(self.entropy, self.uniformity);
            values[FirstOrderFeature::Entropy as usize] = f64::from(e as f32);
            values[FirstOrderFeature::Uniformity as usize] = f64::from(u as f32);
        }
        if let Some(arithmetic) = &mut self.arithmetic {
            arithmetic.evaluate(&mut self.samples)?;
            for &f in &arithmetic.features {
                values[f as usize] = arithmetic.value(f)?;
            }
        }
        self.samples.clear();
        Ok(values)
    }
}

#[derive(Clone)]
struct SampleStatistics {
    features: Vec<FirstOrderFeature>,
    calculation: Calculation,
    shift: f64,
    nan_when_constant: bool,
    constant: bool,
    shifted_overflow: bool,
    source_overflow: bool,
    mean: f32,
    mean_scratch: Vec<f32>,
    quantiles: [FloatPair; 5],
}

impl SampleStatistics {
    fn new(
        features: Vec<FirstOrderFeature>,
        spacing: &[f64],
        settings: &crate::Settings,
    ) -> Result<Self> {
        let mut axes = [1.; 3];
        axes[3 - spacing.len()..].copy_from_slice(spacing);
        let calculation = Calculation::new(
            &features,
            FirstOrderSettings {
                // The arithmetic kernel receives only unbinned statistics.
                spacing: axes,
                voxel_array_shift: settings.voxel_array_shift,
                kurtosis: match settings.kurtosis {
                    ConfigKurtosis::Excess => Kurtosis::Excess,
                    ConfigKurtosis::Raw => Kurtosis::Raw,
                },
                ..Default::default()
            },
        )
        .map_err(|e| Error::InvalidInput(e.to_string()))?;
        Ok(Self {
            features,
            calculation,
            shift: settings.voxel_array_shift,
            nan_when_constant: settings.zero_variance_moments == ZeroVarianceMoments::Nan,
            constant: false,
            shifted_overflow: false,
            source_overflow: false,
            mean: 0.,
            mean_scratch: Vec::new(),
            quantiles: [FloatPair { high: 0., low: 0. }; 5],
        })
    }

    fn evaluate(&mut self, samples: &mut [f64]) -> Result<()> {
        let Some(&first) = samples.first() else {
            return Err(Error::InvalidInput("no first-order ROI samples".into()));
        };
        check_samples(samples, &self.features, self.shift)?;
        if self.features.as_slice() == [FirstOrderFeature::Mean] {
            self.mean = reported_mean(samples, &mut self.mean_scratch);
            return Ok(());
        }
        self.constant = self.nan_when_constant && samples.iter().all(|&v| v == first);
        self.shifted_overflow = self.features.iter().copied().any(uses_shift)
            && samples
                .iter()
                .any(|v| !((v + self.shift) as f32).is_finite());
        use FirstOrderFeature::*;
        self.source_overflow = samples.iter().any(|&v| !(v as f32).is_finite());
        if self
            .features
            .iter()
            .any(|f| matches!(f, Mean | CoefficientOfVariation))
        {
            self.mean = reported_mean(samples, &mut self.mean_scratch);
        }
        self.calculation
            .measure(samples)
            .map_err(|e| Error::InvalidInput(e.to_string()))?;
        // The numerical kernel sorts in source order whenever a quantile is selected.
        if self.features.contains(&QuartileCoefficientOfDispersion)
            || (self.source_overflow
                && self.features.iter().any(|f| {
                    matches!(
                        f,
                        Median
                            | Percentile10
                            | Percentile90
                            | InterquartileRange
                            | RobustMeanAbsoluteDeviation
                    )
                }))
        {
            self.quantiles = [10, 25, 50, 75, 90]
                .map(|p| QuantileRank::new(samples.len(), p).reported_pair(samples));
        }
        Ok(())
    }

    fn value(&self, feature: FirstOrderFeature) -> Result<f64> {
        use FirstOrderFeature::*;
        if self.constant && matches!(feature, Skewness | Kurtosis) {
            return Ok(f64::NAN);
        }
        if self.shifted_overflow && uses_shift(feature) {
            return Ok(f64::INFINITY);
        }
        if self.source_overflow {
            match feature {
                Range
                | Variance
                | MeanAbsoluteDeviation
                | MedianAbsoluteDeviation
                | CoefficientOfVariation => return Ok(f64::NAN),
                Skewness | Kurtosis => {
                    return Ok(if self.nan_when_constant { f64::NAN } else { 0. });
                }
                Percentile10 | Median | Percentile90 => {
                    let rank = match feature {
                        Percentile10 => 0,
                        Median => 2,
                        _ => 4,
                    };
                    return Ok(f64::from(self.quantiles[rank].folded()));
                }
                InterquartileRange => {
                    return Ok(f64::from(
                        subtract_pairs(self.quantiles[3], self.quantiles[1]).folded(),
                    ));
                }
                RobustMeanAbsoluteDeviation
                    if !self.quantiles[0].folded().is_finite()
                        || !self.quantiles[4].folded().is_finite() =>
                {
                    return Ok(f64::NAN);
                }
                _ => {}
            }
        }
        let value = match feature {
            Mean => f64::from(self.mean),
            CoefficientOfVariation => {
                self.calculation.value(Variance).sqrt() / f64::from(self.mean)
            }
            QuartileCoefficientOfDispersion => {
                let [_, mut lo, _, mut hi, _] = self.quantiles;
                if lo.high.abs().max(hi.high.abs()) > f32::MAX * 0.5 {
                    lo = lo.scaled(crate::numerics::pair_arithmetic::RowScale::Multiply(0.5));
                    hi = hi.scaled(crate::numerics::pair_arithmetic::RowScale::Multiply(0.5));
                }
                f64::from(
                    subtract_pairs(hi, lo).folded() / subtract_pairs(hi, lo.negated()).folded(),
                )
            }
            _ => self.calculation.value(feature),
        };
        let reported = value as f32;
        if value != 0.
            && (reported == 0. || reported.is_subnormal())
            && matches!(
                feature,
                Energy
                    | TotalEnergy
                    | RootMeanSquared
                    | Range
                    | InterquartileRange
                    | Variance
                    | MeanAbsoluteDeviation
                    | MedianAbsoluteDeviation
                    | RobustMeanAbsoluteDeviation
            )
        {
            return Err(Error::Execution(format!(
                "first-order {} underflows the float32 range",
                feature.short_name()
            )));
        }
        Ok(if reported.is_subnormal() {
            0.
        } else {
            f64::from(reported)
        })
    }
}
