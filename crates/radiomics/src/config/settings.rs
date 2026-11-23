//! Feature catalog and extraction settings.
//!
//! Mirrors `mlxradiomics/config/settings.py` (500 lines at mlx-radiomics b58783c).
//! Single-threaded CPU orchestration.
//! Status: catalog, settings and validation ported.

use crate::error::{Error, Result};

mod catalog;

/// Feature families, in mlx-radiomics `FAMILIES` order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Family {
    /// `firstorder`
    FirstOrder,
    /// `glcm`
    Glcm,
    /// `gldm` (NGLDM)
    Gldm,
    /// `ngtdm`
    Ngtdm,
    /// `glrlm`
    Glrlm,
    /// `glszm`
    Glszm,
    /// `shape` (3-D ROI only)
    Shape,
    /// `shape2D` (2-D ROI, or one slice of a 3-D ROI)
    Shape2D,
    /// `gldzm`
    Gldzm,
    /// `intensityhistogram`
    IntensityHistogram,
    /// `localintensity` (3-D ROI only)
    LocalIntensity,
    /// `ivh`
    Ivh,
}

impl Family {
    /// Every family, in `FAMILIES` order.
    pub const ALL: [Self; 12] = [
        Self::FirstOrder,
        Self::Glcm,
        Self::Gldm,
        Self::Ngtdm,
        Self::Glrlm,
        Self::Glszm,
        Self::Shape,
        Self::Shape2D,
        Self::Gldzm,
        Self::IntensityHistogram,
        Self::LocalIntensity,
        Self::Ivh,
    ];

    /// Selection prefix, e.g. `firstorder`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstOrder => "firstorder",
            Self::Glcm => "glcm",
            Self::Gldm => "gldm",
            Self::Ngtdm => "ngtdm",
            Self::Glrlm => "glrlm",
            Self::Glszm => "glszm",
            Self::Shape => "shape",
            Self::Shape2D => "shape2D",
            Self::Gldzm => "gldzm",
            Self::IntensityHistogram => "intensityhistogram",
            Self::LocalIntensity => "localintensity",
            Self::Ivh => "ivh",
        }
    }

    /// Parse a selection prefix.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|family| family.as_str() == text)
    }

    /// Families that voxel mode cannot compute.
    pub const fn roi_only(self) -> bool {
        matches!(self, Self::Shape | Self::Shape2D | Self::LocalIntensity)
    }
}

impl std::fmt::Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Family {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        Self::parse(text)
            .ok_or_else(|| Error::InvalidInput(format!("unknown feature family {text:?}")))
    }
}

/// One public feature: `<family>_<name>`, with its IBSI mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeatureSpec {
    /// Family.
    pub family: Family,
    /// Name within the family, e.g. `Contrast`.
    pub name: &'static str,
    /// Member of the 93-name default selection.
    pub default: bool,
    /// IBSI permanent identifier, when IBSI defines the feature.
    pub ibsi_id: Option<&'static str>,
    /// IBSI definition title.
    pub ibsi_title: Option<&'static str>,
    /// IBSI units.
    pub units: Option<&'static str>,
}

impl FeatureSpec {
    /// Selectable name, `<family>_<Name>`. Result keys add an image prefix such as `original_`.
    pub fn key(&self) -> String {
        format!("{}_{}", self.family.as_str(), self.name)
    }
}

/// All 190 selectable features, in mlx-radiomics `FEATURES` order.
///
/// ```
/// let all = radiomics::features();
/// assert_eq!(all.len(), 190);
/// assert_eq!(all[0].key(), format!("{}_{}", all[0].family, all[0].name));
/// ```
#[must_use]
pub fn features() -> &'static [FeatureSpec] {
    &crate::config::settings::catalog::FEATURES
}

/// The 93-name default selection: PyRadiomics intensity and texture features.
///
/// ```
/// assert_eq!(radiomics::default_features().len(), 93);
/// ```
#[must_use]
pub fn default_features() -> Vec<String> {
    features()
        .iter()
        .filter(|spec| spec.default)
        .map(FeatureSpec::key)
        .collect()
}

/// Extraction mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    /// One value per ROI.
    #[default]
    Roi,
    /// One map per feature over local neighbourhoods.
    Voxel,
}

/// List selectable names, optionally restricted to families, mode and dimension.
///
/// With no families, list the full catalog, not the default selection. Voxel mode
/// excludes shape, shape2D and local intensity; `ndim = Some(2)` excludes 3-D shape and
/// local intensity; `ndim = Some(3)` excludes shape2D. Names keep catalog order and occur
/// once even when a family is repeated.
///
/// # Errors
/// [`Error::InvalidInput`] for an unknown family or an `ndim` other than 2 or 3.
///
/// # Examples
///
/// ```
/// use radiomics::{list_features, Mode};
///
/// # fn main() -> radiomics::Result<()> {
/// let first_order = list_features(&["firstorder"], Mode::Roi, Some(3))?;
/// assert_eq!(first_order.len(), 21);
/// assert!(first_order.contains(&"firstorder_Mean".to_string()));
/// // Shape features have no voxel-map form.
/// assert!(list_features(&["shape"], Mode::Voxel, None)?.is_empty());
/// # Ok(())
/// # }
/// ```
pub fn list_features(families: &[&str], mode: Mode, ndim: Option<u8>) -> Result<Vec<String>> {
    if let Some(n) = ndim
        && n != 2
        && n != 3
    {
        return Err(Error::InvalidInput("ndim must be 2, 3 or None".into()));
    }
    let mut unknown: Vec<&str> = families
        .iter()
        .copied()
        .filter(|name| Family::parse(name).is_none())
        .collect();
    if !unknown.is_empty() {
        unknown.sort_unstable();
        unknown.dedup();
        let choices: Vec<&str> = Family::ALL.iter().map(|family| family.as_str()).collect();
        return Err(Error::InvalidInput(format!(
            "unknown feature families: {unknown:?}; choose from {choices:?}"
        )));
    }
    let selected = |family: Family| {
        let requested = families.is_empty() || families.contains(&family.as_str());
        let mode_ok = mode == Mode::Roi || !family.roi_only();
        let ndim_ok = match ndim {
            Some(2) => !matches!(family, Family::Shape | Family::LocalIntensity),
            Some(3) => family != Family::Shape2D,
            _ => true,
        };
        requested && mode_ok && ndim_ok
    };
    Ok(features()
        .iter()
        .filter(|spec| selected(spec.family))
        .map(FeatureSpec::key)
        .collect())
}

string_enum! {
    /// Directional matrix aggregation for GLCM and GLRLM.
    DirectionalAggregation {
        Native => "native",
        Average2D => "2d_average",
        SliceMerge2D => "2d_slice_merge",
        DirectionMerge25D => "2.5d_direction_merge",
        Merge25D => "2.5d_merge",
        Average3D => "3d_average",
        Merge3D => "3d_merge",
    }
    default Native
}

string_enum! {
    /// Matrix aggregation for GLSZM, GLDZM, NGTDM and GLDM.
    MatrixAggregation { Native => "native", Average2D => "2d_average", Merge25D => "2.5d_merge", ThreeD => "3d" }
    default Native
}

string_enum! {
    /// Shape PCA and enclosing-ellipsoid covariance divisor (IBSI `n-1`, PyRadiomics `n`).
    CovarianceDivisor { NMinusOne => "n-1", N => "n" }
    default NMinusOne
}

string_enum! {
    /// Approximate enclosing ellipsoid area: series as printed in IBSI, or converged.
    EllipsoidArea { IbsiSeries => "ibsi-series", Exact => "exact" }
    default IbsiSeries
}

string_enum! {
    /// Kurtosis convention (IBSI excess, PyRadiomics raw).
    Kurtosis { Excess => "excess", Raw => "raw" }
    default Excess
}

string_enum! {
    /// Skewness and kurtosis of a constant ROI (IBSI 0, PyRadiomics NaN).
    ZeroVarianceMoments { Zero => "zero", Nan => "nan" }
    default Zero
}

string_enum! {
    /// NGTDM singular cases (IBSI caps, PyRadiomics unadjusted).
    NgtdmSingularCases { IbsiCaps => "ibsi-caps", Unadjusted => "unadjusted" }
    default IbsiCaps
}

string_enum! {
    /// NGLDM/NGTDM neighbourhood (IBSI closed radius, PyRadiomics distance shells).
    NeighborhoodSupport { ClosedRadius => "closed-radius", Distances => "distances" }
    default ClosedRadius
}

string_enum! {
    /// Fixed-bin-width origin (IBSI resegment-or-minimum, PyRadiomics zero lattice).
    BinOrigin { ResegmentOrMinimum => "resegment-or-minimum", ZeroLattice => "zero-lattice" }
    default ResegmentOrMinimum
}

string_enum! {
    /// Gray level of a constant ROI under a fixed bin count.
    ConstantRoiBin { First => "first", Middle => "middle" }
    default First
}

string_enum! {
    /// IBSI-grid mask outside the source centre hull.
    MaskExtrapolation { Zero => "zero", Nearest => "nearest" }
    default Zero
}

string_enum! {
    /// IBSI B-spline boundary.
    SplineBoundary { Nearest => "nearest", Mirror => "mirror" }
    default Nearest
}

string_enum! {
    /// Resegmentation range interpretation.
    ResegmentMode { Absolute => "absolute", Relative => "relative", Sigma => "sigma" }
    default Absolute
}

/// Feature selection, discretisation and calculation conventions. Defaults follow IBSI.
///
/// Field names, types and defaults match `mlxradiomics.Settings`; axes are NumPy axes.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Settings {
    /// GLDM dependence tolerance in bins.
    pub gldm_alpha: u32,
    /// Fixed bin width; ignored when `bin_count` is set.
    pub bin_width: f64,
    /// Fixed bin count over the sampled range; overrides `bin_width`.
    pub bin_count: Option<u32>,
    /// Whole-image z-score before resegmentation.
    pub normalize: bool,
    /// Scale applied after normalisation.
    pub normalize_scale: f64,
    /// Clip normalised intensities to plus or minus this many standard deviations.
    pub remove_outliers: Option<f64>,
    /// One or two inclusive resegmentation bounds.
    pub resegment_range: Option<Vec<f64>>,
    /// How `resegment_range` is read.
    pub resegment_mode: ResegmentMode,
    /// Shape features use the resegmented mask.
    pub resegment_shape: bool,
    /// Mask label selecting the ROI.
    pub label: i64,
    /// Chebyshev distances for GLCM, GLDM and NGTDM.
    pub distances: Vec<u32>,
    /// Add the transpose to each GLCM.
    pub symmetrical_glcm: bool,
    /// NumPy axis for PyRadiomics `force2D`.
    pub force2d_axis: Option<usize>,
    /// Voxel mode: neighbourhoods sample ROI voxels only.
    pub masked_kernel: bool,
    /// Voxel mode: neighbourhood half-width.
    pub kernel_radius: u32,
    /// Shift added for Energy, TotalEnergy and RootMeanSquared.
    pub voxel_array_shift: f64,
    /// Scratch memory budget in MiB.
    pub scratch_mib: u32,
    /// Cap on the gray-level lattice.
    pub max_gray_levels: Option<u32>,
    /// Selected `<family>_<Name>` features.
    pub features: Vec<String>,
    /// Explicit IBSI minimum gray level anchor; gray level 1 starts here.
    pub bin_anchor: Option<f64>,
    /// Also drop voxels further than this many standard deviations from the mean.
    pub resegment_outlier_sigma: Option<f64>,
    /// Shape covariance divisor.
    pub covariance_divisor: CovarianceDivisor,
    /// Enclosing-ellipsoid area formula.
    pub ellipsoid_area: EllipsoidArea,
    /// Kurtosis convention.
    pub kurtosis: Kurtosis,
    /// Constant-ROI moments.
    pub zero_variance_moments: ZeroVarianceMoments,
    /// NGTDM singular cases.
    pub ngtdm_singular_cases: NgtdmSingularCases,
    /// NGLDM/NGTDM neighbourhood.
    pub neighborhood_support: NeighborhoodSupport,
    /// Fixed-bin-width origin.
    pub bin_origin: BinOrigin,
    /// Constant ROI under a fixed bin count.
    pub constant_roi_bin: ConstantRoiBin,
    /// IBSI-grid mask extrapolation.
    pub mask_extrapolation: MaskExtrapolation,
    /// IBSI B-spline boundary.
    pub spline_boundary: SplineBoundary,
    /// GLCM aggregation.
    pub glcm_aggregation: DirectionalAggregation,
    /// GLRLM aggregation.
    pub glrlm_aggregation: DirectionalAggregation,
    /// GLSZM aggregation.
    pub glszm_aggregation: MatrixAggregation,
    /// GLDZM aggregation.
    pub gldzm_aggregation: MatrixAggregation,
    /// NGTDM aggregation.
    pub ngtdm_aggregation: MatrixAggregation,
    /// GLDM aggregation.
    pub gldm_aggregation: MatrixAggregation,
    /// Planar aggregation axis when `force2d_axis` is unset.
    pub slice_axis: usize,
    /// Closed-radius NGTDM/NGLDM radius.
    pub neighborhood_radius: u32,
    /// IVH lattice bin width.
    pub ivh_bin_width: Option<f64>,
    /// IVH lattice bin count.
    pub ivh_bin_count: Option<u32>,
    /// IVH intensity range.
    pub ivh_intensity_range: Option<(f64, f64)>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gldm_alpha: 0,
            bin_width: 25.0,
            bin_count: None,
            normalize: false,
            normalize_scale: 1.0,
            remove_outliers: None,
            resegment_range: None,
            resegment_mode: ResegmentMode::Absolute,
            resegment_shape: false,
            label: 1,
            distances: vec![1],
            symmetrical_glcm: true,
            force2d_axis: None,
            masked_kernel: true,
            kernel_radius: 1,
            voxel_array_shift: 0.0,
            scratch_mib: 64,
            max_gray_levels: None,
            features: default_features(),
            bin_anchor: None,
            resegment_outlier_sigma: None,
            covariance_divisor: CovarianceDivisor::NMinusOne,
            ellipsoid_area: EllipsoidArea::IbsiSeries,
            kurtosis: Kurtosis::Excess,
            zero_variance_moments: ZeroVarianceMoments::Zero,
            ngtdm_singular_cases: NgtdmSingularCases::IbsiCaps,
            neighborhood_support: NeighborhoodSupport::ClosedRadius,
            bin_origin: BinOrigin::ResegmentOrMinimum,
            constant_roi_bin: ConstantRoiBin::First,
            mask_extrapolation: MaskExtrapolation::Zero,
            spline_boundary: SplineBoundary::Nearest,
            glcm_aggregation: DirectionalAggregation::Native,
            glrlm_aggregation: DirectionalAggregation::Native,
            glszm_aggregation: MatrixAggregation::Native,
            gldzm_aggregation: MatrixAggregation::Native,
            ngtdm_aggregation: MatrixAggregation::Native,
            gldm_aggregation: MatrixAggregation::Native,
            slice_axis: 0,
            neighborhood_radius: 1,
            ivh_bin_width: None,
            ivh_bin_count: None,
            ivh_intensity_range: None,
        }
    }
}

impl Settings {
    /// The PyRadiomics convention set used by mlx-radiomics' tests (`settings_for("pyradiomics")`).
    #[must_use]
    pub fn pyradiomics_conventions() -> Self {
        Self {
            covariance_divisor: CovarianceDivisor::N,
            ellipsoid_area: EllipsoidArea::Exact,
            kurtosis: Kurtosis::Raw,
            zero_variance_moments: ZeroVarianceMoments::Nan,
            ngtdm_singular_cases: NgtdmSingularCases::Unadjusted,
            neighborhood_support: NeighborhoodSupport::Distances,
            bin_origin: BinOrigin::ZeroLattice,
            constant_roi_bin: ConstantRoiBin::Middle,
            mask_extrapolation: MaskExtrapolation::Nearest,
            spline_boundary: SplineBoundary::Mirror,
            ..Self::default()
        }
    }
}

impl Settings {
    /// Validate preprocessing, selected feature definitions and extraction controls for `ndim`.
    /// The enum and scalar field types enforce mlx's type checks for direct Rust callers.
    pub fn validate(&self, ndim: usize) -> Result<()> {
        use crate::config::validation::{invalid, positive, sorted_names, unique};

        self.validate_preprocessing()?;
        self.validate_definitions(ndim)?;
        if self.bin_count.is_none() && !positive(self.bin_width) {
            return invalid("bin_width must be finite and positive");
        }
        if self.label <= 0 {
            return invalid("label must be a positive integer");
        }
        if self.distances.is_empty() || self.distances.contains(&0) {
            return invalid("distances must contain positive integers");
        }
        if !unique(&self.distances) {
            return invalid("distances must be unique");
        }
        if self.force2d_axis.is_some_and(|axis| axis >= ndim) {
            return invalid("force2d_axis must be an integer indexing a NumPy image axis");
        }
        if self.kernel_radius == 0 {
            return invalid("kernel_radius must be a positive integer");
        }
        if self.scratch_mib == 0 {
            return invalid("scratch_mib must be a positive integer");
        }
        if self.max_gray_levels == Some(0) {
            return invalid("max_gray_levels must be a positive integer or None");
        }
        if !self.voxel_array_shift.is_finite() {
            return invalid("voxel_array_shift must be finite");
        }
        if self.features.is_empty() || !unique(&self.features) {
            return invalid("features must be a nonempty unique selection");
        }
        let unknown: Vec<_> = self
            .features
            .iter()
            .filter(|key| {
                let Some((family, name)) = key.split_once('_') else {
                    return true;
                };
                !features()
                    .iter()
                    .any(|spec| spec.family.as_str() == family && spec.name == name)
            })
            .map(String::as_str)
            .collect();
        if !unknown.is_empty() {
            return invalid(format!("unsupported features: {}", sorted_names(unknown)));
        }
        Ok(())
    }

    fn validate_preprocessing(&self) -> Result<()> {
        use crate::config::validation::{invalid, positive};
        if let Some(anchor) = self.bin_anchor {
            if !anchor.is_finite() {
                return invalid("bin_anchor must be a finite number or None");
            }
            if self.bin_count.is_some() {
                return invalid("bin_anchor cannot be combined with bin_count");
            }
        }
        if self.bin_count.is_some_and(|n| !(1..(1 << 31)).contains(&n)) {
            return invalid("bin_count must be a positive integer below 2**31");
        }
        if !positive(self.normalize_scale) {
            return invalid("normalize_scale must be finite and positive");
        }
        if self
            .remove_outliers
            .is_some_and(|n| !positive(n) || !self.normalize)
        {
            return invalid("remove_outliers requires normalization and a finite positive value");
        }
        if self
            .resegment_range
            .as_ref()
            .is_some_and(|v| !(1..=2).contains(&v.len()) || v.iter().any(|n| !n.is_finite()))
        {
            return invalid("resegment_range must contain one or two finite thresholds");
        }
        if let Some(sigma) = self.resegment_outlier_sigma {
            if !positive(sigma) {
                return invalid("resegment_outlier_sigma must be a finite positive number or None");
            }
            if self.resegment_range.is_none() || self.resegment_mode != ResegmentMode::Absolute {
                return invalid(
                    r#"resegment_outlier_sigma requires resegment_range and resegment_mode="absolute""#,
                );
            }
        }
        Ok(())
    }

    fn validate_definitions(&self, ndim: usize) -> Result<()> {
        use crate::config::validation::{invalid, positive};
        for (prefix, mode) in [
            ("glcm_", self.glcm_aggregation.as_str()),
            ("glrlm_", self.glrlm_aggregation.as_str()),
            ("glszm_", self.glszm_aggregation.as_str()),
            ("gldzm_", self.gldzm_aggregation.as_str()),
            ("ngtdm_", self.ngtdm_aggregation.as_str()),
            ("gldm_", self.gldm_aggregation.as_str()),
        ] {
            if mode.starts_with("3d")
                && (ndim != 3 || self.force2d_axis.is_some())
                && self.features.iter().any(|name| name.starts_with(prefix))
            {
                return invalid("3d aggregation requires 3D input without force2d_axis");
            }
        }
        if self.slice_axis >= ndim {
            return invalid("slice_axis must be an integer indexing an image axis");
        }
        if self.neighborhood_radius == 0 {
            return invalid("neighborhood_radius must be a positive integer");
        }
        if self.ivh_bin_width.is_some_and(|width| !positive(width)) {
            return invalid("ivh_bin_width must be finite and positive");
        }
        if self.ivh_bin_count == Some(0) {
            return invalid("ivh_bin_count must be a positive integer");
        }
        if self.ivh_bin_width.is_some() && self.ivh_bin_count.is_some() {
            return invalid("IVH accepts either bin width or bin count");
        }
        if self
            .ivh_intensity_range
            .is_some_and(|(lo, hi)| !lo.is_finite() || !hi.is_finite() || lo > hi)
        {
            return invalid("ivh_intensity_range must be finite ordered lower/upper bounds");
        }
        Ok(())
    }
}
