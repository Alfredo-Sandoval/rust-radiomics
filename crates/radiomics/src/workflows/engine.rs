//! Original-image CPU extraction, voxel delivery and execution provenance.
//!
//! ROI features run on the CPU. Large global-intensity peak searches and voxel texture
//! batches use Rayon. Coordinates and spacing follow NumPy axis order, and the public voxel
//! sink receives float32 samples for each requested feature in selection order.

mod delivery;
mod inputs;
pub use crate::workflows::engine::inputs::extract;

use indexmap::IndexMap;
use ndarray::{ArrayD, ArrayView1, IxDyn};
use serde_json::Number;

use crate::config::settings::DirectionalAggregation;
use crate::contracts::results::ExtractionResult;
use crate::contracts::types::{ExecutionMetadata, MapDtype, VoxelSink};
use crate::contracts::validity::{accumulate_validity, empty_validity};
use crate::features::{aggregation::resolved_aggregation, ivh, morphology};
use crate::numerics::runtime::{require_retained_capacity, zeroed_map};
use crate::voxel::extraction::extract_scheduled;
use crate::workflows::engine::delivery::{
    masked_samples, source_runs, store_column, write_samples,
};
use crate::{Error, Family, Prepared, Result, Settings};

/// Feature output from the original or derived-image workflows.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum FeatureValue {
    /// One scalar over the entire intensity or morphology mask.
    Scalar(f64),
    /// One value per voxel, stored as float32.
    Map32(ArrayD<f32>),
    /// The same evaluated voxel values stored as float64.
    Map64(ArrayD<f64>),
}

/// Open and release a streaming consumer independently for each feature image.
///
/// Pass a factory to [`Output::PerImage`]. `open` is called once for each original or
/// derived image (for example `original` or `wavelet-LLH`) before its first batch, and
/// `close` after the sink is dropped.
///
/// ```
/// use radiomics::indexmap::IndexMap;
/// use radiomics::ndarray::{ArrayD, ArrayView1, ArrayView2};
/// use radiomics::{
///     extract, HostArray, MapDtype, Output, SinkFactory, VoxelSink, Settings, TransformSettings,
/// };
///
/// /// Counts the voxels delivered for each image.
/// #[derive(Default)]
/// struct Counts(Vec<(String, usize)>);
///
/// struct CountingSink<'a> {
///     slot: &'a mut usize,
/// }
///
/// impl VoxelSink for CountingSink<'_> {
///     fn consume(
///         &mut self,
///         centers: ArrayView2<'_, i32>,
///         _features: &IndexMap<String, ArrayView1<'_, f32>>,
///     ) -> radiomics::Result<()> {
///         *self.slot += centers.nrows();
///         Ok(())
///     }
/// }
///
/// impl SinkFactory for Counts {
///     fn open(&mut self, image: &str) -> radiomics::Result<Box<dyn VoxelSink + '_>> {
///         self.0.push((image.to_string(), 0));
///         let slot = &mut self.0.last_mut().expect("just pushed").1;
///         Ok(Box::new(CountingSink { slot }))
///     }
/// }
///
/// # fn main() -> radiomics::Result<()> {
/// let image = HostArray::from(ArrayD::from_shape_fn(vec![4, 5, 5], |ix| (ix[1] + ix[2]) as i16));
/// let mask = HostArray::from(ArrayD::from_elem(vec![4, 5, 5], 1_u8));
/// let settings = Settings {
///     features: vec!["firstorder_Mean".into()],
///     kernel_radius: 1,
///     ..Settings::default()
/// };
/// let mut factory = Counts::default();
/// extract(
///     &image,
///     &mask,
///     &settings,
///     &TransformSettings::default(),
///     None,
///     Output::PerImage { dtype: MapDtype::Float32, factory: &mut factory },
/// )?;
/// assert_eq!(factory.0, [("original".to_string(), 100)]);
/// # Ok(())
/// # }
/// ```
pub trait SinkFactory {
    /// Called once before the first batch of an image.
    fn open(&mut self, image: &str) -> Result<Box<dyn VoxelSink + '_>>;
    /// Select delivery for one image. Returning `None` retains its maps with the
    /// usual byte limit and dtype. Streaming factories use their existing `open`.
    fn open_optional(&mut self, image: &str) -> Result<Option<Box<dyn VoxelSink + '_>>> {
        self.open(image).map(Some)
    }
    /// Called after the consumer is dropped, including after a batch error.
    fn close(&mut self, _image: &str, _error: Option<&Error>) -> Result<()> {
        Ok(())
    }
}

/// Delivery mode for a single image. A sink receives batches synchronously.
/// The retained-output byte limit applies only when the workflow allocates maps.
#[non_exhaustive]
pub enum Output<'a> {
    /// One scalar per feature.
    Roi,
    /// Retain zero-background maps, with an optional caller-provided byte limit.
    Maps {
        /// Storage dtype.
        dtype: MapDtype,
        /// Maximum total payload, in bytes. Defaults to a quarter of physical RAM
        /// when the platform supplies its size.
        max_bytes: Option<usize>,
    },
    /// Send float32 batches to the consumer, preserving the requested map dtype in metadata.
    Stream {
        /// Dtype recorded for the requested run.
        dtype: MapDtype,
        /// Evaluated batch consumer.
        sink: &'a mut dyn VoxelSink,
    },
    /// Open a separate consumer for each original or filtered image.
    PerImage {
        /// Dtype recorded in provenance.
        dtype: MapDtype,
        /// Factory for per-image resources.
        factory: &'a mut dyn SinkFactory,
    },
}

impl std::fmt::Debug for Output<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Roi => f.write_str("Roi"),
            Self::Maps { dtype, max_bytes } => f
                .debug_struct("Maps")
                .field("dtype", dtype)
                .field("max_bytes", max_bytes)
                .finish(),
            Self::Stream { dtype, .. } => f
                .debug_struct("Stream")
                .field("dtype", dtype)
                .finish_non_exhaustive(),
            Self::PerImage { dtype, .. } => f
                .debug_struct("PerImage")
                .field("dtype", dtype)
                .finish_non_exhaustive(),
        }
    }
}

impl Output<'_> {
    pub(super) fn map_dtype(&self) -> Option<MapDtype> {
        match self {
            Self::Roi => None,
            Self::Maps { dtype, .. }
            | Self::Stream { dtype, .. }
            | Self::PerImage { dtype, .. } => Some(*dtype),
        }
    }

    pub(super) fn reborrow(&mut self) -> Output<'_> {
        match self {
            Self::Roi => Output::Roi,
            Self::Maps { dtype, max_bytes } => Output::Maps {
                dtype: *dtype,
                max_bytes: *max_bytes,
            },
            Self::Stream { dtype, sink } => Output::Stream {
                dtype: *dtype,
                sink: *sink,
            },
            Self::PerImage { dtype, factory } => Output::PerImage {
                dtype: *dtype,
                factory: *factory,
            },
        }
    }

    pub(super) fn admit(&self, voxels: usize, features: usize) -> Result<()> {
        let Self::Maps { dtype, max_bytes } = self else {
            return Ok(());
        };
        let limit = max_bytes.unwrap_or_else(retained_budget);
        match dtype {
            MapDtype::Float32 => require_retained_capacity::<f32>(voxels, features, limit),
            MapDtype::Float64 => require_retained_capacity::<f64>(voxels, features, limit),
        }
    }
}

fn retained_budget() -> usize {
    static BYTES: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *BYTES.get_or_init(|| {
        #[cfg(target_os = "linux")]
        let ram = std::fs::read_to_string("/proc/meminfo").ok().and_then(|s| {
            s.lines()
                .find(|s| s.starts_with("MemTotal:"))?
                .split_whitespace()
                .nth(1)?
                .parse::<usize>()
                .ok()?
                .checked_mul(1024)
        });
        #[cfg(target_os = "macos")]
        let ram = macos_physical_memory_bytes();
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let ram: Option<usize> = None;
        ram.map_or(usize::MAX, |n| n / 4)
    })
}

// Query in-process because a spawned child can keep an export directory lock alive.
#[cfg(target_os = "macos")]
fn macos_physical_memory_bytes() -> Option<usize> {
    let mut bytes = 0_u64;
    let mut len = size_of::<u64>();
    // SAFETY: hw.memsize returns a uint64_t. The name is NUL-terminated, bytes
    // and len are writable for the call, and a null newp requests a read only.
    #[allow(unsafe_code)]
    let status = unsafe {
        libc::sysctlbyname(
            c"hw.memsize".as_ptr(),
            (&raw mut bytes).cast(),
            &raw mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if status == 0 && len == size_of::<u64>() {
        usize::try_from(bytes).ok()
    } else {
        None
    }
}

fn aggregation(settings: &Settings, ndim: usize) -> Result<IndexMap<String, String>> {
    let mut modes = IndexMap::new();
    for family in [
        Family::Glcm,
        Family::Glrlm,
        Family::Glszm,
        Family::Gldzm,
        Family::Ngtdm,
        Family::Gldm,
    ] {
        if !settings
            .features
            .iter()
            .any(|k| k.split_once('_').is_some_and(|(f, _)| f == family.as_str()))
        {
            continue;
        }
        let mode = resolved_aggregation(family, settings, ndim)?;
        let name = if !matches!(family, Family::Glcm | Family::Glrlm)
            && mode == DirectionalAggregation::Average3D
        {
            "3d"
        } else {
            mode.as_str()
        };
        modes.insert(family.as_str().into(), name.into());
    }
    Ok(modes)
}

pub(super) fn checked_source(shape: &[usize], spacing: &[f64]) -> Result<usize> {
    if !(2..=3).contains(&shape.len()) || shape.contains(&0) {
        return Err(Error::InvalidInput(
            "image and mask must share a nonempty 2D or 3D array shape".into(),
        ));
    }
    if spacing.len() != shape.len() || spacing.iter().any(|&x| !x.is_finite() || x <= 0.) {
        return Err(Error::InvalidInput(
            "spacing must contain one finite positive value per array axis".into(),
        ));
    }
    shape
        .iter()
        .try_fold(1usize, |n, &x| n.checked_mul(x))
        .filter(|&n| n < 1 << 30)
        .ok_or_else(|| {
            Error::InvalidInput("image exceeds supported integer indexing/count range".into())
        })
}

// The voxel planner materializes an int32 coordinate for each kernel position.
// Check that mandatory allocation before constructing any offset ranges.
fn admit_kernel_offsets(settings: &Settings, ndim: usize) -> Result<()> {
    let side = 2 * u128::from(settings.kernel_radius) + 1;
    let axes = ndim - usize::from(settings.force2d_axis.is_some());
    let positions = side.pow(axes as u32);
    let bytes = positions * ndim as u128 * std::mem::size_of::<i32>() as u128;
    let budget = u128::from(settings.scratch_mib) << 20;
    if settings.kernel_radius > i32::MAX as u32 || bytes > budget || bytes > isize::MAX as u128 {
        return Err(Error::InvalidInput(
            "scratch_mib cannot hold one local histogram; reduce bins/kernel radius".into(),
        ));
    }
    Ok(())
}

/// Original source coordinates and optional undefined support for a filtered image.
#[derive(Clone, Copy)]
pub(super) struct ImageContext<'a> {
    pub shape: &'a [usize],
    pub dtype: &'a str,
    pub name: &'a str,
    pub undefined: Option<&'a ArrayD<bool>>,
}

impl<'a> ImageContext<'a> {
    fn original(shape: &'a [usize], dtype: &'a str) -> Self {
        Self {
            shape,
            dtype,
            name: "original",
            undefined: None,
        }
    }
}

pub(super) fn run_prepared(
    data: &Prepared,
    settings: &Settings,
    spacing: &[f64],
    context: ImageContext<'_>,
    output: Output<'_>,
) -> Result<ExtractionResult<FeatureValue>> {
    run_prepared_at(data, settings, spacing, context, output, None)
}

// The private window is in source index space. Only the calculation sees local coordinates.
fn run_prepared_at(
    data: &Prepared,
    settings: &Settings,
    spacing: &[f64],
    context: ImageContext<'_>,
    output: Output<'_>,
    voxel_origin: Option<&[usize]>,
) -> Result<ExtractionResult<FeatureValue>> {
    let ImageContext {
        shape: source_shape,
        dtype: source_dtype,
        name: image_name,
        undefined,
    } = context;
    if let Output::PerImage { dtype, factory } = output {
        let mut consumer = factory.open_optional(image_name)?;
        let delivery = match consumer.as_deref_mut() {
            Some(sink) => Output::Stream { dtype, sink },
            None => Output::Maps {
                dtype,
                max_bytes: None,
            },
        };
        let result = run_prepared_at(data, settings, spacing, context, delivery, voxel_origin);
        drop(consumer);
        let closed = factory.close(image_name, result.as_ref().err());
        let value = result?;
        closed?;
        return Ok(value);
    }
    let voxels = checked_source(source_shape, spacing)?;
    let shape = data.input.image.shape();
    let voxel = output.map_dtype().is_some();
    if source_shape.len() != shape.len()
        || shape.contains(&0)
        || shape.iter().zip(source_shape).any(|(a, b)| a > b)
        || (voxel && voxel_origin.is_none() && shape != source_shape)
        || voxel_origin.is_some_and(|origin| {
            origin.len() != shape.len()
                || origin
                    .iter()
                    .zip(shape)
                    .zip(source_shape)
                    .any(|((&lo, &size), &full)| lo > full - size)
        })
        || data.input.roi.shape() != shape
        || data.input.shape_roi.shape() != shape
        || data.input.sampling_mask.shape() != shape
        || data.bins.as_ref().is_some_and(|a| a.shape() != shape)
    {
        return Err(Error::InvalidInput(
            "prepared masks, bins and image must share a compatible grid".into(),
        ));
    }
    settings.validate(shape.len())?;
    if voxel {
        admit_kernel_offsets(settings, shape.len())?;
    }
    output.admit(voxels, settings.features.len())?;
    let roi_voxels = data.input.roi.iter().filter(|&&x| x).count();
    if !voxel {
        let requested: Vec<_> = settings
            .features
            .iter()
            .filter_map(|k| k.strip_prefix("shape_"))
            .collect();
        morphology::require_pairwise_admission(&requested, roi_voxels)?;
    }
    let lattice = settings
        .features
        .iter()
        .any(|k| k.starts_with("ivh_"))
        .then(|| ivh::prepare_ivh(&data.input.image, &data.input.sampling_mask, settings))
        .transpose()?;
    let ivh_lattice = lattice
        .as_ref()
        .map(|l| -> Result<_> {
            let number = |x| {
                Number::from_f64(x).ok_or_else(|| {
                    Error::InvalidInput("IVH lattice calibration must be finite".into())
                })
            };
            Ok(IndexMap::from([
                ("origin".into(), number(l.origin)?),
                ("step".into(), number(l.step)?),
                ("levels".into(), l.levels.into()),
            ]))
        })
        .transpose()?;
    let excluded = voxels - roi_voxels;
    let mut metadata = ExecutionMetadata {
        settings: settings.clone(),
        device: Default::default(),
        precision: Default::default(),
        map_dtype: output.map_dtype(),
        voxel,
        spacing: spacing.to_vec(),
        histogram_method: "cpu".into(),
        preprocessing:
            "native CPU normalization, resegmentation, discretization and feature extraction".into(),
        gray_levels: data.gray_levels,
        runtime: Default::default(),
        array_shape: source_shape.to_vec(),
        array_dtype: source_dtype.into(),
        roi_voxels,
        excluded_voxels: excluded,
        feature_validity: settings
            .features
            .iter()
            .map(|k| {
                (
                    format!("{image_name}_{k}"),
                    empty_validity(if voxel { excluded } else { 0 }),
                )
            })
            .collect(),
        aggregation: Some(aggregation(settings, shape.len())?),
        ivh_lattice,
        gray_levels_by_image: None,
        batch_size: None,
        scratch_estimate_bytes: None,
        first_order_batch_size: None,
        first_order_scratch_estimate_bytes: None,
        first_order_cache_bytes: None,
        image_geometry: None,
        source_geometry: None,
        image_settings: None,
        transforms: None,
    };
    let mut features = IndexMap::new();
    if !voxel {
        for (key, value) in
            crate::workflows::roi::compute_roi(data, settings, spacing, lattice.as_ref())?
        {
            let value = if undefined.is_some() { f64::NAN } else { value };
            let key = format!("{image_name}_{key}");
            accumulate_validity(&mut metadata.feature_validity[&key], [&value]);
            features.insert(key, FeatureValue::Scalar(value));
        }
    } else {
        let mut output = output;
        let mut stream_columns = Vec::<Vec<f32>>::new();
        let mut invalid_rows = Vec::new();
        let mut masked_column = Vec::new();
        let origin = voxel_origin.filter(|lo| lo.iter().any(|&i| i != 0));
        let origin_offset = origin.map_or(0, |lo| {
            lo.iter()
                .zip(source_shape)
                .fold(0, |i, (&x, &size)| i * size + x)
        });
        let mut runs = Vec::new();
        let schedule = extract_scheduled(
            data,
            settings,
            spacing,
            lattice.as_ref(),
            Some(roi_voxels),
            &mut |centers, columns| {
                invalid_rows.clear();
                if let Some(invalid) = undefined {
                    for (i, center) in centers.rows().into_iter().enumerate() {
                        let mut at = [0usize; 3];
                        for (v, &x) in at.iter_mut().zip(center) {
                            *v = x as usize;
                        }
                        if invalid[IxDyn(&at[..shape.len()])] {
                            invalid_rows.push(i);
                        }
                    }
                }
                match &mut output {
                    Output::Maps { dtype, .. } => {
                        source_runs(&mut runs, centers, source_shape, origin_offset);
                        if features.is_empty() {
                            features.extend(metadata.feature_validity.keys().cloned().map(|key| {
                                let map = match dtype {
                                    MapDtype::Float32 => {
                                        FeatureValue::Map32(zeroed_map(source_shape))
                                    }
                                    MapDtype::Float64 => {
                                        FeatureValue::Map64(zeroed_map(source_shape))
                                    }
                                };
                                (key, map)
                            }));
                        }
                        for ((map, counts), column) in features
                            .values_mut()
                            .zip(metadata.feature_validity.values_mut())
                            .zip(columns)
                        {
                            let samples = masked_samples(column, &invalid_rows, &mut masked_column);
                            store_column(map, &runs, samples, counts);
                        }
                        Ok(())
                    }
                    Output::Stream { sink, .. } => {
                        stream_columns.resize_with(columns.len(), Vec::new);
                        for ((buffer, counts), column) in stream_columns
                            .iter_mut()
                            .zip(metadata.feature_validity.values_mut())
                            .zip(columns)
                        {
                            let samples = masked_samples(column, &invalid_rows, &mut masked_column);
                            buffer.resize(samples.len(), 0.0);
                            write_samples(buffer, samples, counts);
                        }
                        let mut translated;
                        let centers = if let Some(origin) = origin {
                            translated = centers.clone();
                            for mut row in translated.rows_mut() {
                                for (coordinate, &start) in row.iter_mut().zip(origin) {
                                    *coordinate += start as i32;
                                }
                            }
                            &translated
                        } else {
                            centers
                        };
                        let batch = metadata
                            .feature_validity
                            .keys()
                            .zip(&stream_columns)
                            .map(|(k, c)| (k.clone(), ArrayView1::from(c.as_slice())))
                            .collect();
                        sink.consume(centers.view(), &batch)
                    }
                    Output::Roi | Output::PerImage { .. } => unreachable!(),
                }
            },
        )?;
        metadata.batch_size = Some(schedule.memory.batch_size);
        metadata.scratch_estimate_bytes = Some(schedule.memory.scratch_estimate_bytes);
        if let Some(group) = schedule.first {
            metadata.first_order_batch_size = Some(group.rows);
            metadata.first_order_scratch_estimate_bytes = Some(group.scratch_bytes);
            metadata.first_order_cache_bytes = Some(group.cache_bytes);
        }
    }
    Ok(ExtractionResult { features, metadata })
}

#[cfg(target_os = "macos")]
#[test]
fn macos_default_map_limit_is_one_quarter_of_physical_memory() {
    let ram = macos_physical_memory_bytes().expect("read hw.memsize");
    let voxels = ram / 4 / size_of::<f64>();
    assert!(voxels > 0);
    let output = Output::Maps {
        dtype: MapDtype::Float64,
        max_bytes: None,
    };
    output.admit(voxels, 1).unwrap();
    assert!(output.admit(voxels + 1, 1).is_err());
}
