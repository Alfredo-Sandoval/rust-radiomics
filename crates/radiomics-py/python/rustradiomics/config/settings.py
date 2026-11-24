"""Feature selection and extraction settings for the CPU runtime."""

from dataclasses import dataclass
from typing import Literal

from rustradiomics import _native
from rustradiomics.errors import InvalidInputError


def list_features(
    *families: str, mode: Literal["roi", "voxel"] = "roi", ndim: int | None = None
) -> tuple[str, ...]:
    """Return selectable names in catalog order for the requested mode and NumPy rank."""
    if mode not in ("roi", "voxel"):
        raise InvalidInputError("mode must be 'roi' or 'voxel'")
    if ndim is not None and (type(ndim) is not int or ndim not in (2, 3)):
        raise InvalidInputError("ndim must be 2, 3 or None")
    return tuple(_native.list_features(families, mode, ndim))


FEATURES = list_features()
DEFAULT_FEATURES = tuple(_native.default_features())


type DirectionalAggregation = Literal[
    "native",
    "2d_average",
    "2d_slice_merge",
    "2.5d_direction_merge",
    "2.5d_merge",
    "3d_average",
    "3d_merge",
]
type MatrixAggregation = Literal["native", "2d_average", "2.5d_merge", "3d"]
type CovarianceDivisor = Literal["n-1", "n"]
type EllipsoidArea = Literal["ibsi-series", "exact"]
type Kurtosis = Literal["excess", "raw"]
type ZeroVarianceMoments = Literal["zero", "nan"]
type NgtdmSingularCases = Literal["ibsi-caps", "unadjusted"]
type NeighborhoodSupport = Literal["closed-radius", "distances"]
type BinOrigin = Literal["resegment-or-minimum", "zero-lattice"]
type ConstantRoiBin = Literal["first", "middle"]
type MaskExtrapolation = Literal["zero", "nearest"]
type SplineBoundary = Literal["nearest", "mirror"]


@dataclass(frozen=True)
class Settings:
    """Feature selection and preprocessing options for one extraction."""

    gldm_alpha: int = 0
    bin_width: float = 25.0
    bin_count: int | None = None
    normalize: bool = False
    normalize_scale: float = 1.0
    remove_outliers: float | None = None
    resegment_range: tuple[float, ...] | None = None
    resegment_mode: Literal["absolute", "relative", "sigma"] = "absolute"
    resegment_shape: bool = False
    label: int = 1
    distances: tuple[int, ...] = (1,)
    symmetrical_glcm: bool = True
    force2d_axis: int | None = None
    masked_kernel: bool = True
    kernel_radius: int = 1
    voxel_array_shift: float = 0.0
    scratch_mib: int = 64
    max_gray_levels: int | None = None
    features: tuple[str, ...] = DEFAULT_FEATURES
    bin_anchor: float | None = None
    resegment_outlier_sigma: float | None = None
    covariance_divisor: CovarianceDivisor = "n-1"
    ellipsoid_area: EllipsoidArea = "ibsi-series"
    kurtosis: Kurtosis = "excess"
    zero_variance_moments: ZeroVarianceMoments = "zero"
    ngtdm_singular_cases: NgtdmSingularCases = "ibsi-caps"
    neighborhood_support: NeighborhoodSupport = "closed-radius"
    bin_origin: BinOrigin = "resegment-or-minimum"
    constant_roi_bin: ConstantRoiBin = "first"
    mask_extrapolation: MaskExtrapolation = "zero"
    spline_boundary: SplineBoundary = "nearest"
    glcm_aggregation: DirectionalAggregation = "native"
    glrlm_aggregation: DirectionalAggregation = "native"
    glszm_aggregation: MatrixAggregation = "native"
    gldzm_aggregation: MatrixAggregation = "native"
    ngtdm_aggregation: MatrixAggregation = "native"
    gldm_aggregation: MatrixAggregation = "native"
    slice_axis: int = 0
    neighborhood_radius: int = 1
    ivh_bin_width: float | None = None
    ivh_bin_count: int | None = None
    ivh_intensity_range: tuple[float, float] | None = None

    def validate(self, ndim: int) -> None:
        """Validate these settings for an image with the given NumPy rank."""
        _native.validate_settings(self, ndim)
