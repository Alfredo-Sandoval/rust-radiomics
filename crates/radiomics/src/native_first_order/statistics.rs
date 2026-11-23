use crate::native_first_order::ExtractionError;
use crate::numerics::count_statistics::bin_entropy;
use crate::numerics::precision::QuantileRank;
use crate::numerics::reduction::CompensatedSum;

macro_rules! first_order_features {
    ($($feature:ident => $name:literal),+ $(,)?) => {
        /// Supported first-order features for the original image.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum FirstOrderFeature {
            $(#[doc = $name] $feature),+
        }

        impl FirstOrderFeature {
            /// All supported features in catalogue order.
            pub const ALL: [Self; first_order_features!(@count $($feature),+)] = [
                $(Self::$feature),+
            ];

            /// Name within the first-order family.
            pub const fn short_name(self) -> &'static str {
                match self { $(Self::$feature => $name),+ }
            }

            /// Look up a short feature name, such as `"10Percentile"`.
            pub fn from_short_name(name: &str) -> Option<Self> {
                match name { $($name => Some(Self::$feature)),+, _ => None }
            }

            /// Name used in PyRadiomics-compatible feature tables.
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$feature => concat!("original_firstorder_", $name)),+
                }
            }
        }
    };
    (@count $($feature:ident),+) => {
        <[&str]>::len(&[$(stringify!($feature)),+])
    };
}

first_order_features! {
    Energy => "Energy",
    TotalEnergy => "TotalEnergy",
    Entropy => "Entropy",
    Minimum => "Minimum",
    Percentile10 => "10Percentile",
    Percentile90 => "90Percentile",
    Maximum => "Maximum",
    Mean => "Mean",
    Median => "Median",
    InterquartileRange => "InterquartileRange",
    Range => "Range",
    MeanAbsoluteDeviation => "MeanAbsoluteDeviation",
    RobustMeanAbsoluteDeviation => "RobustMeanAbsoluteDeviation",
    RootMeanSquared => "RootMeanSquared",
    Skewness => "Skewness",
    Kurtosis => "Kurtosis",
    Variance => "Variance",
    Uniformity => "Uniformity",
    MedianAbsoluteDeviation => "MedianAbsoluteDeviation",
    CoefficientOfVariation => "CoefficientOfVariation",
    QuartileCoefficientOfDispersion => "QuartileCoefficientOfDispersion",
}

use FirstOrderFeature::*;

/// Convention for the standardized fourth moment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kurtosis {
    /// Subtract three from the standardized fourth moment.
    Excess,
    /// Return the standardized fourth moment.
    Raw,
}

/// Intensity discretization and first-order statistic conventions.
#[derive(Clone, Copy, Debug)]
pub struct FirstOrderSettings {
    /// Fixed intensity-bin width for entropy and uniformity.
    pub bin_width: f64,
    /// Spacing in millimeters in `[z, y, x]` order for TotalEnergy.
    pub spacing: [f64; 3],
    /// Shift applied only to Energy, TotalEnergy and RootMeanSquared.
    pub voxel_array_shift: f64,
    /// Fourth moment convention. Constant neighborhoods return zero.
    pub kurtosis: Kurtosis,
}

impl Default for FirstOrderSettings {
    fn default() -> Self {
        Self {
            bin_width: 25.0,
            spacing: [1.0; 3],
            voxel_array_shift: 0.0,
            kurtosis: Kurtosis::Excess,
        }
    }
}

#[derive(Clone, Default)]
struct Statistics {
    count: f64,
    mean: f64,
    minimum: f64,
    maximum: f64,
    energy: f64,
    variance: f64,
    skewness: f64,
    kurtosis: f64,
    absolute_deviation: f64,
    robust_absolute_deviation: f64,
    median_absolute_deviation: f64,
    quartile_coefficient: f64,
    p10: f64,
    median: f64,
    p90: f64,
    interquartile_range: f64,
    sum_p_log_p: f64,
    uniformity: f64,
}

#[derive(Clone)]
pub(crate) struct Calculation {
    settings: FirstOrderSettings,
    voxel_volume: f64,
    energy: bool,
    moments: bool,
    higher_moments: bool,
    deviation: bool,
    robust: bool,
    median_deviation: bool,
    quartile_coefficient: bool,
    quantiles: bool,
    histogram: bool,
    origin: f64,
    bins: Vec<i64>,
    stats: Statistics,
}

impl Calculation {
    pub(crate) fn new(
        features: &[FirstOrderFeature],
        settings: FirstOrderSettings,
    ) -> Result<Self, ExtractionError> {
        if features.is_empty() {
            return Err(ExtractionError::EmptyFeatures);
        }
        if !settings.bin_width.is_finite() || settings.bin_width <= 0.0 {
            return Err(ExtractionError::InvalidBinWidth);
        }
        let voxel_volume = settings.spacing.iter().product::<f64>();
        if !voxel_volume.is_finite()
            || voxel_volume <= 0.0
            || settings.spacing.iter().any(|&spacing| spacing <= 0.0)
        {
            return Err(ExtractionError::InvalidSpacing);
        }
        if !settings.voxel_array_shift.is_finite() {
            return Err(ExtractionError::InvalidIntensityShift);
        }

        let wants = |group: &[FirstOrderFeature]| group.iter().any(|f| features.contains(f));
        let higher_moments = wants(&[Skewness, Kurtosis]);
        let robust = features.contains(&RobustMeanAbsoluteDeviation);
        let median_deviation = features.contains(&MedianAbsoluteDeviation);
        let quartile_coefficient = features.contains(&QuartileCoefficientOfDispersion);
        Ok(Self {
            settings,
            voxel_volume,
            energy: wants(&[Energy, TotalEnergy, RootMeanSquared]),
            moments: higher_moments || wants(&[Variance, CoefficientOfVariation]),
            higher_moments,
            deviation: features.contains(&MeanAbsoluteDeviation),
            robust,
            median_deviation,
            quartile_coefficient,
            quantiles: robust
                || median_deviation
                || quartile_coefficient
                || wants(&[Percentile10, Percentile90, Median, InterquartileRange]),
            histogram: wants(&[Entropy, Uniformity]),
            origin: 0.0,
            bins: Vec::new(),
            stats: Statistics::default(),
        })
    }

    pub(crate) fn measure<T: Copy>(&mut self, samples: &mut [T]) -> Result<(), ExtractionError>
    where
        f64: From<T>,
    {
        let mut stats = Statistics {
            count: samples.len() as f64,
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            ..Statistics::default()
        };
        let mut sum = CompensatedSum::default();
        for &sample in samples.iter() {
            let value = f64::from(sample);
            if !value.is_finite() {
                return Err(ExtractionError::NonFiniteIntensity);
            }
            sum.add(value);
            stats.minimum = stats.minimum.min(value);
            stats.maximum = stats.maximum.max(value);
        }
        stats.mean = sum.total() / stats.count;

        if self.energy {
            stats.energy = samples
                .iter()
                .map(|&sample| {
                    let shifted = f64::from(sample) + self.settings.voxel_array_shift;
                    shifted * shifted
                })
                .sum();
        }
        if self.moments || self.deviation {
            self.measure_deviations(samples, &mut stats);
        }
        if self.histogram {
            self.measure_histogram(samples, &mut stats)?;
        }
        if self.quantiles {
            sort_quantile_samples(samples, stats.minimum, stats.maximum);
            let ranks =
                [10, 25, 50, 75, 90].map(|percent| QuantileRank::new(samples.len(), percent));
            let [p10, lower_quartile, median, upper_quartile, p90] =
                ranks.map(|rank| rank.interpolate(samples));
            stats.p10 = p10;
            stats.median = median;
            stats.p90 = p90;
            stats.interquartile_range = upper_quartile - lower_quartile;
            if self.quartile_coefficient {
                stats.quartile_coefficient =
                    stats.interquartile_range / (upper_quartile + lower_quartile);
            }
            if self.median_deviation {
                stats.median_absolute_deviation = samples
                    .iter()
                    .map(|&sample| (f64::from(sample) - stats.median).abs())
                    .sum::<f64>()
                    / stats.count;
            }
            if self.robust {
                // The first included sample is at or above the upper endpoint of the
                // 10th-percentile interpolation. Comparing source values retains all ties.
                let low = f64::from(samples[ranks[0].upper()]);
                let high = f64::from(samples[ranks[4].lower()]);
                let lo = samples.partition_point(|&v| f64::from(v) < low);
                let hi = samples.partition_point(|&v| f64::from(v) <= high);
                let middle = &samples[lo..hi];
                let anchor = if low.signum() == high.signum() {
                    low
                } else {
                    0.
                };
                let mean = average_offset(middle, anchor);
                stats.robust_absolute_deviation = middle
                    .iter()
                    .map(|&sample| ((f64::from(sample) - anchor) - mean).abs())
                    .sum::<f64>()
                    / middle.len() as f64;
            }
        }
        self.stats = stats;
        Ok(())
    }

    fn measure_deviations<T: Copy>(&self, samples: &[T], stats: &mut Statistics)
    where
        f64: From<T>,
    {
        let mut squares = 0.0;
        let mut cubes = 0.0;
        let mut fourths = 0.0;
        let mut absolute = 0.0;
        // A rounded absolute mean can introduce false skew in high-offset float64 data.
        let narrow = size_of::<T>() == size_of::<f64>()
            && stats.maximum - stats.minimum < 0.5 * stats.minimum.abs();
        let (anchor, mean) = if narrow {
            (stats.minimum, average_offset(samples, stats.minimum))
        } else {
            (0., stats.mean)
        };
        for &sample in samples {
            let delta = (f64::from(sample) - anchor) - mean;
            let square = delta * delta;
            squares += square;
            if self.higher_moments {
                cubes += square * delta;
                fourths += square * square;
            }
            if self.deviation {
                absolute += delta.abs();
            }
        }
        stats.absolute_deviation = absolute / stats.count;
        stats.variance = squares / stats.count;
        if self.higher_moments && stats.variance != 0.0 {
            stats.skewness = (cubes / stats.count) / (stats.variance * stats.variance.sqrt());
            stats.kurtosis = (fourths / stats.count) / (stats.variance * stats.variance);
            if self.settings.kurtosis == crate::native_first_order::statistics::Kurtosis::Excess {
                stats.kurtosis -= 3.0;
            }
        }
    }

    fn measure_histogram<T: Copy>(
        &mut self,
        samples: &[T],
        stats: &mut Statistics,
    ) -> Result<(), ExtractionError>
    where
        f64: From<T>,
    {
        let origin = self.origin;
        let width = self.settings.bin_width;
        // Every finite sample lies between these two validated bin coordinates.
        const INDEX_LIMIT: f64 = (1u64 << 52) as f64;
        let bounds = -INDEX_LIMIT..INDEX_LIMIT;
        if !bounds.contains(&((stats.minimum - origin) / width).floor())
            || !bounds.contains(&((stats.maximum - origin) / width).floor())
        {
            return Err(ExtractionError::BinIndexOutOfRange);
        }
        let lower = bin_index(stats.minimum - origin, width);
        let upper = bin_index(stats.maximum - origin, width);
        let span = (upper - lower + 1) as u64;

        self.bins.clear();
        if span <= samples.len() as u64 {
            // Direct counts avoid sorting every overlapping neighborhood on a compact lattice.
            self.bins.resize(span as usize, 0);
            for &sample in samples {
                let bin = bin_index(f64::from(sample) - origin, width);
                self.bins[(bin - lower) as usize] += 1;
            }
            for &count in &self.bins {
                if count != 0 {
                    stats.add_bin(count as usize, samples.len());
                }
            }
        } else {
            for &sample in samples {
                self.bins.push(bin_index(f64::from(sample) - origin, width));
            }
            self.bins.sort_unstable();
            for run in self.bins.chunk_by(|left, right| left == right) {
                stats.add_bin(run.len(), samples.len());
            }
        }
        Ok(())
    }

    pub(crate) fn value(&self, feature: FirstOrderFeature) -> f64 {
        let stats = &self.stats;
        match feature {
            Energy => stats.energy,
            TotalEnergy => stats.energy * self.voxel_volume,
            Entropy => -stats.sum_p_log_p,
            Minimum => stats.minimum,
            Percentile10 => stats.p10,
            Percentile90 => stats.p90,
            Maximum => stats.maximum,
            Mean => stats.mean,
            Median => stats.median,
            InterquartileRange => stats.interquartile_range,
            Range => stats.maximum - stats.minimum,
            MeanAbsoluteDeviation => stats.absolute_deviation,
            RobustMeanAbsoluteDeviation => stats.robust_absolute_deviation,
            RootMeanSquared => (stats.energy / stats.count).sqrt(),
            Skewness => stats.skewness,
            Kurtosis => stats.kurtosis,
            Variance => stats.variance,
            Uniformity => stats.uniformity,
            MedianAbsoluteDeviation => stats.median_absolute_deviation,
            CoefficientOfVariation => stats.variance.sqrt() / stats.mean,
            QuartileCoefficientOfDispersion => stats.quartile_coefficient,
        }
    }
}

/// Count bounded integer CT intensities in linear time. Other samples retain
/// binary64 total order, including the ordering of negative and positive zero.
fn sort_quantile_samples<T: Copy>(samples: &mut [T], minimum: f64, maximum: f64)
where
    f64: From<T>,
{
    let span = maximum - minimum;
    if samples.len() >= 2048
        && span <= (samples.len() / 4).min(65535) as f64
        && minimum.abs().max(maximum.abs()) <= (1u64 << 52) as f64
        && minimum.trunc() == minimum
    {
        let mut counts = vec![(0usize, samples[0]); span as usize + 1];
        let mut integral = true;
        for &value in samples.iter() {
            let x = f64::from(value);
            if x.trunc() != x || x.to_bits() == (-0.0f64).to_bits() {
                integral = false;
                break;
            }
            let entry = &mut counts[(x - minimum) as usize];
            entry.0 += 1;
            entry.1 = value;
        }
        if integral {
            let mut rest = samples;
            for (count, value) in counts {
                let (run, tail) = rest.split_at_mut(count);
                run.fill(value);
                rest = tail;
            }
            return;
        }
    }
    samples.sort_unstable_by(|&a, &b| f64::from(a).total_cmp(&f64::from(b)));
}

fn bin_index(distance: f64, width: f64) -> i64 {
    let floor = (distance / width).floor();
    // Correct quotient rounding for samples immediately next to a bin edge.
    let residual = (-floor).mul_add(width, distance);
    if residual < 0.0 {
        floor as i64 - 1
    } else if residual >= width {
        floor as i64 + 1
    } else {
        floor as i64
    }
}

impl Statistics {
    fn add_bin(&mut self, count: usize, total: usize) {
        let fraction = count as f64 / self.count;
        self.uniformity += fraction * fraction;
        self.sum_p_log_p -= bin_entropy(count, total);
    }
}

fn average_offset<T: Copy>(samples: &[T], anchor: f64) -> f64
where
    f64: From<T>,
{
    let mut sum = CompensatedSum::default();
    for &v in samples {
        sum.add(f64::from(v) - anchor);
    }
    sum.total() / samples.len() as f64
}

#[cfg(test)]
mod sort_tests {
    use crate::native_first_order::statistics::sort_quantile_samples;

    #[test]
    fn compact_integer_sort_and_fallback_preserve_every_source_word() {
        for special in [0., -0., 0.25, 1e19] {
            let mut values: Vec<f64> = (0..4096).map(|i| f64::from(i * 17 % 51 - 25)).collect();
            values[2048] = special;
            let mut sorted = values.clone();
            sorted.sort_unstable_by(f64::total_cmp);
            sort_quantile_samples(&mut values, sorted[0], sorted[sorted.len() - 1]);
            assert!(
                values
                    .iter()
                    .zip(&sorted)
                    .all(|(x, y)| x.to_bits() == y.to_bits())
            );
        }
    }
}
