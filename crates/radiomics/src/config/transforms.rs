//! Typed transform configuration and validation without a Metal dependency.
//!
//! Mirrors `mlxradiomics/config/transforms.py` (616 lines at mlx-radiomics b58783c).
//! Single-threaded CPU orchestration.
//! Status: validation ported except W6 wavelet-name lookup.
//! IBSI filter tuples (Laws kernels, bands, Riesz orders) use IBSI
//! (k1, k2, k3) order, where k1 is the last NumPy axis.

use crate::config::filtering::{
    FilterPadding, FourierGrid, GaborResponse, LbpMethod, LogKernel, OrientationPooling, RieszBase,
};

string_enum! {
    /// Derived image types, in mlx-radiomics declaration order.
    ImageType {
        Original => "original",
        Square => "square",
        SquareRoot => "squareroot",
        Logarithm => "logarithm",
        Exponential => "exponential",
        Gradient => "gradient",
        Wavelet => "wavelet",
        Log => "log",
        Lbp2D => "lbp-2D",
        Lbp3D => "lbp-3D",
        Mean => "mean",
        Laws => "laws",
        Gabor => "gabor",
        Simoncelli => "simoncelli",
        Riesz => "riesz",
        SeparableWavelet => "separablewavelet",
    }
    default Original
}

/// Undecimated PyRadiomics-style wavelet decomposition.
#[derive(Clone, Debug, PartialEq)]
pub struct WaveletSettings {
    /// PyWavelets name, e.g. `coif1`.
    pub name: String,
    /// Number of levels produced.
    pub level: u32,
    /// Levels skipped before the first output.
    pub start_level: u32,
}

impl Default for WaveletSettings {
    fn default() -> Self {
        Self {
            name: "coif1".into(),
            level: 1,
            start_level: 0,
        }
    }
}

/// Laplacian of Gaussian.
#[derive(Clone, Debug, PartialEq)]
pub struct LogSettings {
    /// Scales in mm; required when LoG is requested.
    pub sigma: Vec<f64>,
    /// Kernel construction.
    pub kernel: LogKernel,
    /// Sampled-kernel truncation in standard deviations.
    pub cutoff: f64,
    /// Sampled-kernel boundary handling.
    pub padding: FilterPadding,
    /// Value for constant padding.
    pub constant: f64,
    /// Filter each slice separately.
    pub planar: bool,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            sigma: Vec::new(),
            kernel: LogKernel::Recursive,
            cutoff: 4.0,
            padding: FilterPadding::Mirror,
            constant: 0.0,
            planar: false,
        }
    }
}

/// Two-dimensional local binary patterns.
#[derive(Clone, Debug, PartialEq)]
pub struct Lbp2DSettings {
    /// Sampling radius in voxels.
    pub radius: f64,
    /// Number of samples on the circle (1-24).
    pub samples: u32,
    /// Pattern coding.
    pub method: LbpMethod,
}

impl Default for Lbp2DSettings {
    fn default() -> Self {
        Self {
            radius: 1.0,
            samples: 8,
            method: LbpMethod::Uniform,
        }
    }
}

/// Three-dimensional spherical-harmonic local binary patterns.
#[derive(Clone, Debug, PartialEq)]
pub struct Lbp3DSettings {
    /// Spherical-harmonic levels.
    pub levels: u32,
    /// Icosphere radius in voxels.
    pub radius: f64,
    /// Icosphere subdivision (0-4).
    pub subdivision: u32,
}

impl Default for Lbp3DSettings {
    fn default() -> Self {
        Self {
            levels: 2,
            radius: 1.0,
            subdivision: 1,
        }
    }
}

/// Gradient magnitude.
#[derive(Clone, Debug, PartialEq)]
pub struct GradientSettings {
    /// Divide differences by the voxel spacing.
    pub use_spacing: bool,
}

impl Default for GradientSettings {
    fn default() -> Self {
        Self { use_spacing: true }
    }
}

/// IBSI 2 mean filter.
#[derive(Clone, Debug, PartialEq)]
pub struct MeanSettings {
    /// Odd support width in voxels.
    pub support: u32,
    /// Boundary handling.
    pub padding: FilterPadding,
    /// Value for constant padding.
    pub constant: f64,
    /// Filter each slice separately.
    pub planar: bool,
}

impl Default for MeanSettings {
    fn default() -> Self {
        Self {
            support: 3,
            padding: FilterPadding::Mirror,
            constant: 0.0,
            planar: false,
        }
    }
}

/// IBSI 2 Laws filter.
#[derive(Clone, Debug, PartialEq)]
pub struct LawsSettings {
    /// Two or three kernel names from L3, L5, E3, E5, S3, S5, W5, R5, in IBSI order.
    pub kernel: String,
    /// Boundary handling.
    pub padding: FilterPadding,
    /// Value for constant padding.
    pub constant: f64,
    /// Pooling over the 24 (3-D) or 4 (2-D) right-angle rotations.
    pub pooling: OrientationPooling,
    /// Chebyshev radius of the energy image, if any.
    pub energy_distance: Option<u32>,
}

impl Default for LawsSettings {
    fn default() -> Self {
        Self {
            kernel: "L5S5".into(),
            padding: FilterPadding::Mirror,
            constant: 0.0,
            pooling: OrientationPooling::None,
            energy_distance: None,
        }
    }
}

/// IBSI 2 Gabor filter.
#[derive(Clone, Debug, PartialEq)]
pub struct GaborSettings {
    /// Gaussian scales in mm.
    pub sigma: Vec<f64>,
    /// Wavelengths in mm.
    pub wavelength: Vec<f64>,
    /// Spatial aspect ratio.
    pub gamma: f64,
    /// Orientations in radians.
    pub theta: Vec<f64>,
    /// Part of the complex response kept.
    pub response: GaborResponse,
    /// Pooling over orientations.
    pub pooling: OrientationPooling,
    /// Plane normal axis in 3-D; `None` averages the three plane families.
    pub slice_axis: Option<usize>,
    /// Boundary handling.
    pub padding: FilterPadding,
    /// Value for constant padding.
    pub constant: f64,
}

impl Default for GaborSettings {
    fn default() -> Self {
        Self {
            sigma: Vec::new(),
            wavelength: Vec::new(),
            gamma: 1.0,
            theta: Vec::new(),
            response: GaborResponse::Modulus,
            pooling: OrientationPooling::None,
            slice_axis: None,
            padding: FilterPadding::Mirror,
            constant: 0.0,
        }
    }
}

/// IBSI 2 non-separable (Simoncelli) wavelet.
#[derive(Clone, Debug, PartialEq)]
pub struct SimoncelliSettings {
    /// Decomposition levels produced.
    pub levels: Vec<u32>,
    /// Boundary handling; must be periodic.
    pub padding: FilterPadding,
    /// Frequency grid.
    pub grid: FourierGrid,
    /// Filter each slice separately.
    pub planar: bool,
}

impl Default for SimoncelliSettings {
    fn default() -> Self {
        Self {
            levels: vec![1],
            padding: FilterPadding::Periodic,
            grid: FourierGrid::Dft,
            planar: false,
        }
    }
}

/// IBSI 2 Riesz transform.
#[derive(Clone, Debug, PartialEq)]
pub struct RieszSettings {
    /// Base filter.
    pub base: RieszBase,
    /// Riesz orders, one tuple per output, in IBSI order.
    pub orders: Vec<Vec<u32>>,
    /// LoG base scales in mm.
    pub log_sigma: Vec<f64>,
    /// LoG base truncation in standard deviations.
    pub log_cutoff: f64,
    /// Simoncelli base levels.
    pub simoncelli_levels: Vec<u32>,
    /// Boundary handling.
    pub padding: FilterPadding,
    /// Value for constant padding.
    pub constant: f64,
    /// Frequency grid.
    pub grid: FourierGrid,
    /// Steer orders to the local structure tensor.
    pub steering: bool,
    /// Structure-tensor scale in mm; required exactly when `steering`.
    pub tensor_sigma: Option<f64>,
}

impl Default for RieszSettings {
    fn default() -> Self {
        Self {
            base: RieszBase::Log,
            orders: Vec::new(),
            log_sigma: Vec::new(),
            log_cutoff: 4.0,
            simoncelli_levels: vec![1],
            padding: FilterPadding::Mirror,
            constant: 0.0,
            grid: FourierGrid::Dft,
            steering: false,
            tensor_sigma: None,
        }
    }
}

/// IBSI 2 separable wavelet.
#[derive(Clone, Debug, PartialEq)]
pub struct SeparableWaveletSettings {
    /// PyWavelets name, e.g. `haar`.
    pub name: String,
    /// Bands as L/H strings, one letter per filtered axis in IBSI order.
    pub bands: Vec<String>,
    /// Decomposition level.
    pub level: u32,
    /// Boundary handling.
    pub padding: FilterPadding,
    /// Value for constant padding.
    pub constant: f64,
    /// Pooling over rotations.
    pub pooling: OrientationPooling,
    /// Filter each slice separately.
    pub planar: bool,
}

impl Default for SeparableWaveletSettings {
    fn default() -> Self {
        Self {
            name: "haar".into(),
            bands: Vec::new(),
            level: 1,
            padding: FilterPadding::Mirror,
            constant: 0.0,
            pooling: OrientationPooling::None,
            planar: false,
        }
    }
}

/// Derived images to extract, with one settings block per filter.
#[derive(Clone, Debug, PartialEq)]
pub struct TransformSettings {
    /// Image types in output order; unique.
    pub image_types: Vec<ImageType>,
    /// Scratch memory budget in MiB.
    pub scratch_mib: u32,
    /// Wavelet settings.
    pub wavelet: WaveletSettings,
    /// LoG settings.
    pub log: LogSettings,
    /// LBP2D settings.
    pub lbp2d: Lbp2DSettings,
    /// LBP3D settings.
    pub lbp3d: Lbp3DSettings,
    /// Gradient settings.
    pub gradient: GradientSettings,
    /// Mean filter settings.
    pub mean: MeanSettings,
    /// Laws filter settings.
    pub laws: LawsSettings,
    /// Gabor filter settings.
    pub gabor: GaborSettings,
    /// Simoncelli wavelet settings.
    pub simoncelli: SimoncelliSettings,
    /// Riesz transform settings.
    pub riesz: RieszSettings,
    /// Separable wavelet settings.
    pub separable_wavelet: SeparableWaveletSettings,
}

impl Default for TransformSettings {
    fn default() -> Self {
        Self {
            image_types: vec![ImageType::Original],
            scratch_mib: 64,
            wavelet: WaveletSettings::default(),
            log: LogSettings::default(),
            lbp2d: Lbp2DSettings::default(),
            lbp3d: Lbp3DSettings::default(),
            gradient: GradientSettings::default(),
            mean: MeanSettings::default(),
            laws: LawsSettings::default(),
            gabor: GaborSettings::default(),
            simoncelli: SimoncelliSettings::default(),
            riesz: RieszSettings::default(),
            separable_wavelet: SeparableWaveletSettings::default(),
        }
    }
}

use crate::config::filtering::{MAX_STEERING_ORDER, validate_laws_kernel};
use crate::config::validation::{invalid, positive, unique};
use crate::error::{Error, Result};

fn validate_constant(value: f64, name: &str) -> Result<()> {
    if !value.is_finite() {
        return invalid(format!("{name} must be a finite number"));
    }
    Ok(())
}

fn validate_levels(levels: &[u32], name: &str) -> Result<()> {
    if levels.is_empty() || levels.contains(&0) {
        return invalid(format!("{name} must contain positive integers"));
    }
    if !unique(levels) {
        return invalid(format!("{name} must be unique"));
    }
    Ok(())
}

// Validation has excluded NaNs. Both signs of zero compare equal in Python.
fn float_key(value: f64) -> u64 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

impl WaveletSettings {
    /// Check the decomposition levels.
    pub fn validate(&self) -> Result<()> {
        if self.level == 0 {
            return invalid("wavelet_level must be a positive integer");
        }
        Ok(())
    }
}

impl LogSettings {
    /// Check LoG scales and reject sampled-kernel options on the recursive kernel.
    pub fn validate(&self) -> Result<()> {
        if self.sigma.iter().any(|n| !positive(*n)) {
            return invalid("sigma values must be finite and positive");
        }
        if !unique(self.sigma.iter().copied().map(float_key)) {
            return invalid("sigma values must be unique");
        }
        if !positive(self.cutoff) {
            return invalid("log_cutoff must be finite and positive");
        }
        validate_constant(self.constant, "log_constant")?;
        if self.kernel == LogKernel::Recursive
            && (self.planar
                || self.cutoff != 4.0
                || self.padding != FilterPadding::Mirror
                || self.constant != 0.0)
        {
            return invalid(
                "log_cutoff, log_padding, log_constant and log_planar require log_kernel=LogKernel.SAMPLED",
            );
        }
        Ok(())
    }
}

impl Lbp2DSettings {
    /// Check the circular sampling radius and sample count.
    pub fn validate(&self) -> Result<()> {
        if !(1..=24).contains(&self.samples) {
            return invalid("lbp2d_samples must be an integer from 1 to 24");
        }
        if !positive(self.radius) {
            return invalid("LBP radii must be finite and positive");
        }
        Ok(())
    }
}

impl Lbp3DSettings {
    /// Check the spherical sampling radius, levels and subdivision.
    pub fn validate(&self) -> Result<()> {
        if !positive(self.radius) {
            return invalid("LBP radii must be finite and positive");
        }
        if self.levels == 0 {
            return invalid("lbp3d_levels must be a positive integer");
        }
        if self.subdivision > 4 {
            return invalid("lbp3d_subdivision must be an integer from zero to four");
        }
        Ok(())
    }
}

impl MeanSettings {
    /// Check kernel support and the padding constant.
    pub fn validate(&self) -> Result<()> {
        if self.support.is_multiple_of(2) {
            return invalid("mean_support must be a positive odd integer");
        }
        validate_constant(self.constant, "mean_constant")
    }
}

impl LawsSettings {
    /// Check the concatenated IBSI kernel names and the padding constant.
    pub fn validate(&self) -> Result<()> {
        validate_laws_kernel(&self.kernel).map_err(|err| {
            // The shared filter helper uses Rust debug quotes for this fixed list.
            Error::InvalidInput(err.to_string().replace('"', "'"))
        })?;
        validate_constant(self.constant, "laws_constant")
    }
}

impl GaborSettings {
    /// Check the Gabor scales, orientations and optional 3-D slice axis.
    pub fn validate(&self) -> Result<()> {
        for (name, values) in [
            ("gabor_sigma", &self.sigma),
            ("gabor_wavelength", &self.wavelength),
        ] {
            if values.iter().any(|n| !positive(*n)) {
                return invalid(format!("{name} must contain finite positive values"));
            }
        }
        if !positive(self.gamma) {
            return invalid("gabor_gamma must be finite and positive");
        }
        if self.theta.iter().any(|n| !n.is_finite()) {
            return invalid("gabor_theta must contain finite values");
        }
        if self.slice_axis.is_some_and(|axis| axis > 2) {
            return invalid("gabor_slice_axis must be None or an array axis from zero to two");
        }
        validate_constant(self.constant, "gabor_constant")
    }
}

impl SimoncelliSettings {
    /// Check periodic padding and the selected band levels.
    pub fn validate(&self) -> Result<()> {
        if self.padding != FilterPadding::Periodic {
            return invalid("Simoncelli filtering is defined with periodic boundaries only");
        }
        validate_levels(&self.levels, "simoncelli_levels")
    }
}

impl RieszSettings {
    /// Check the base filter, component orders and optional steering controls.
    pub fn validate(&self) -> Result<()> {
        if self.base == RieszBase::Simoncelli && self.padding != FilterPadding::Periodic {
            return invalid("Simoncelli-based Riesz filtering requires periodic padding");
        }
        if self
            .orders
            .iter()
            .any(|order| order.iter().all(|&p| p == 0))
        {
            return invalid(
                "riesz_orders must contain nonempty tuples of nonnegative integers with a positive sum",
            );
        }
        if !unique(&self.orders) {
            return invalid("riesz_orders must be unique");
        }
        if self.log_sigma.iter().any(|n| !positive(*n)) {
            return invalid("riesz_log_sigma must contain finite positive values");
        }
        if !positive(self.log_cutoff) {
            return invalid("riesz_log_cutoff must be finite and positive");
        }
        validate_levels(&self.simoncelli_levels, "riesz_simoncelli_levels")?;
        validate_constant(self.constant, "riesz_constant")?;
        if self.tensor_sigma.is_some_and(|n| !positive(n)) {
            return invalid("riesz_tensor_sigma must be finite and positive");
        }
        if self.steering && self.tensor_sigma.is_none() {
            return invalid("riesz_steering requires riesz_tensor_sigma");
        }
        if self.steering
            && self.orders.iter().any(|order| {
                order.iter().copied().fold(0u32, u32::saturating_add) > MAX_STEERING_ORDER
            })
        {
            return invalid(format!(
                "riesz_steering supports Riesz orders up to {MAX_STEERING_ORDER}"
            ));
        }
        if !self.steering && self.tensor_sigma.is_some() {
            return invalid("riesz_tensor_sigma requires riesz_steering");
        }
        Ok(())
    }
}

impl SeparableWaveletSettings {
    /// Check the band alphabet, uniqueness, level and padding constant.
    pub fn validate(&self) -> Result<()> {
        if self
            .bands
            .iter()
            .any(|s| s.is_empty() || !s.bytes().all(|c| matches!(c, b'L' | b'H')))
        {
            return invalid("separable_wavelet_bands must contain L/H band codes");
        }
        if !unique(&self.bands) {
            return invalid("separable_wavelet_bands must be unique");
        }
        if self.level == 0 {
            return invalid("separable_wavelet_level must be a positive integer");
        }
        validate_constant(self.constant, "separable_wavelet_constant")
    }
}

/// Pair each Gabor sigma with a wavelength, broadcasting either singleton.
pub fn gabor_scale_pairs(options: &TransformSettings) -> Result<Vec<(f64, f64)>> {
    let g = &options.gabor;
    let (ns, nw) = (g.sigma.len(), g.wavelength.len());
    if ns == nw {
        return Ok(g
            .sigma
            .iter()
            .copied()
            .zip(g.wavelength.iter().copied())
            .collect());
    }
    if ns == 1 && nw > 1 {
        return Ok(g.wavelength.iter().map(|&w| (g.sigma[0], w)).collect());
    }
    if nw == 1 && ns > 1 {
        return Ok(g.sigma.iter().map(|&s| (s, g.wavelength[0])).collect());
    }
    invalid("gabor_sigma and gabor_wavelength must have equal lengths or one value")
}

impl TransformSettings {
    /// Validate filter settings and the inputs required by the enabled image types.
    pub fn validate(&self) -> Result<()> {
        self.wavelet.validate()?;
        self.log.validate()?;
        self.lbp2d.validate()?;
        self.lbp3d.validate()?;
        self.mean.validate()?;
        self.laws.validate()?;
        self.gabor.validate()?;
        self.simoncelli.validate()?;
        self.riesz.validate()?;
        self.separable_wavelet.validate()?;
        if self.scratch_mib == 0 {
            return invalid("scratch_mib must be a positive integer");
        }
        if self.image_types.is_empty() {
            return invalid("image_types must contain typed ImageType entries");
        }
        if !unique(&self.image_types) {
            return invalid("image_types must be unique");
        }
        if self.image_types.contains(&ImageType::Wavelet) {
            crate::config::wavelets::wavelet_filters(&self.wavelet.name)?;
        }
        if self.image_types.contains(&ImageType::Log) && self.log.sigma.is_empty() {
            return invalid("LoG requires explicit positive sigma values");
        }
        if self.image_types.contains(&ImageType::SeparableWavelet) {
            crate::config::wavelets::wavelet_filters(&self.separable_wavelet.name)?;
            if self.separable_wavelet.bands.is_empty() {
                return invalid("separable wavelets require explicit separable_wavelet_bands");
            }
        }
        if self.image_types.contains(&ImageType::Gabor) {
            let g = &self.gabor;
            if g.sigma.is_empty() || g.wavelength.is_empty() || g.theta.is_empty() {
                return invalid(
                    "Gabor requires explicit gabor_sigma, gabor_wavelength and gabor_theta",
                );
            }
            let pairs = gabor_scale_pairs(self)?;
            if !unique(g.theta.iter().copied().map(float_key)) {
                return invalid("gabor_theta values must be unique");
            }
            if !unique(pairs.into_iter().map(|(s, w)| (float_key(s), float_key(w)))) {
                return invalid("gabor sigma/wavelength pairs must be unique");
            }
        }
        if self.image_types.contains(&ImageType::Riesz) {
            if self.riesz.orders.is_empty() {
                return invalid("Riesz requires explicit riesz_orders");
            }
            if self.riesz.base == RieszBase::Log && self.riesz.log_sigma.is_empty() {
                return invalid("a LoG Riesz base requires explicit riesz_log_sigma values");
            }
        }
        Ok(())
    }
}

/// Check a selected filter's axis-dependent options against an image's dimension.
pub fn validate_filter_shape(
    ndim: usize,
    options: &TransformSettings,
    kind: ImageType,
) -> Result<()> {
    match kind {
        ImageType::Laws if ndim == 2 && options.laws.kernel.len() == 6 => {
            invalid("a three-axis Laws kernel requires a 3-D image")
        }
        ImageType::Gabor if ndim == 2 && options.gabor.slice_axis.is_some() => {
            invalid("gabor_slice_axis is only valid for a 3-D image")
        }
        ImageType::Riesz if options.riesz.orders.iter().any(|o| o.len() != ndim) => {
            invalid("Riesz order tuples must have one entry per image axis")
        }
        ImageType::SeparableWavelet => {
            let wavelet = &options.separable_wavelet;
            if wavelet.planar && ndim != 3 {
                return invalid("planar separable wavelets require a 3-D image");
            }
            let axes = if wavelet.planar { 2 } else { ndim };
            if wavelet.bands.iter().any(|s| s.len() != axes) {
                return invalid("separable wavelet bands must have one letter per filtered axis");
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
