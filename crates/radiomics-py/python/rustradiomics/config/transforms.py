"""Frozen options for the Rust CPU image filters."""

from dataclasses import dataclass
from enum import StrEnum

from rustradiomics import _native
from rustradiomics.config.filtering import (
    FilterPadding,
    FourierGrid,
    GaborResponse,
    LBPMethod,
    LogKernel,
    OrientationPooling,
    RieszBase,
)


class ImageType(StrEnum):
    """Derived images available in extraction requests."""

    ORIGINAL = "original"
    SQUARE = "square"
    SQUARE_ROOT = "squareroot"
    LOGARITHM = "logarithm"
    EXPONENTIAL = "exponential"
    GRADIENT = "gradient"
    WAVELET = "wavelet"
    LOG = "log"
    LBP2D = "lbp-2D"
    LBP3D = "lbp-3D"
    MEAN = "mean"
    LAWS = "laws"
    GABOR = "gabor"
    SIMONCELLI = "simoncelli"
    RIESZ = "riesz"
    SEPARABLE_WAVELET = "separablewavelet"


@dataclass(frozen=True)
class WaveletSettings:
    """Stationary wavelet decomposition and output levels."""

    name: str = "coif1"
    level: int = 1
    start_level: int = 0

    def __post_init__(self) -> None:
        _native.validate_block("WaveletSettings", self)


@dataclass(frozen=True)
class LoGSettings:
    """Laplacian-of-Gaussian scales and boundary conditions."""

    sigma: tuple[float, ...] = ()
    kernel: LogKernel = LogKernel.RECURSIVE
    cutoff: float = 4.0
    padding: FilterPadding = FilterPadding.MIRROR
    constant: float = 0.0
    planar: bool = False

    def __post_init__(self) -> None:
        _native.validate_block("LoGSettings", self)


@dataclass(frozen=True)
class LBP2DSettings:
    """Planar local binary patterns and their coding method."""

    radius: float = 1.0
    samples: int = 8
    method: LBPMethod = LBPMethod.UNIFORM

    def __post_init__(self) -> None:
        _native.validate_block("LBP2DSettings", self)


@dataclass(frozen=True)
class LBP3DSettings:
    """Spherical local binary patterns and harmonic sampling."""

    levels: int = 2
    radius: float = 1.0
    subdivision: int = 1

    def __post_init__(self) -> None:
        _native.validate_block("LBP3DSettings", self)


@dataclass(frozen=True)
class GradientSettings:
    """Physical-unit scaling for image gradients."""

    use_spacing: bool = True

    def __post_init__(self) -> None:
        _native.validate_block("GradientSettings", self)


@dataclass(frozen=True)
class MeanSettings:
    """Mean-filter support and boundary conditions."""

    support: int = 3
    padding: FilterPadding = FilterPadding.MIRROR
    constant: float = 0.0
    planar: bool = False

    def __post_init__(self) -> None:
        _native.validate_block("MeanSettings", self)


@dataclass(frozen=True)
class LawsSettings:
    """Laws texture kernels, orientation pooling and local energy."""

    kernel: str = "L5S5"
    padding: FilterPadding = FilterPadding.MIRROR
    constant: float = 0.0
    pooling: OrientationPooling = OrientationPooling.NONE
    energy_distance: int | None = None

    def __post_init__(self) -> None:
        _native.validate_block("LawsSettings", self)


@dataclass(frozen=True)
class GaborSettings:
    """Gabor scales, orientations and complex-response selection."""

    sigma: tuple[float, ...] = ()
    wavelength: tuple[float, ...] = ()
    gamma: float = 1.0
    theta: tuple[float, ...] = ()
    response: GaborResponse = GaborResponse.MODULUS
    pooling: OrientationPooling = OrientationPooling.NONE
    slice_axis: int | None = None
    padding: FilterPadding = FilterPadding.MIRROR
    constant: float = 0.0

    def __post_init__(self) -> None:
        _native.validate_block("GaborSettings", self)


@dataclass(frozen=True)
class SimoncelliSettings:
    """Simoncelli levels and Fourier sampling."""

    levels: tuple[int, ...] = (1,)
    padding: FilterPadding = FilterPadding.PERIODIC
    grid: FourierGrid = FourierGrid.DFT
    planar: bool = False

    def __post_init__(self) -> None:
        _native.validate_block("SimoncelliSettings", self)


@dataclass(frozen=True)
class RieszSettings:
    """Riesz components, base filters and optional tensor steering."""

    base: RieszBase = RieszBase.LOG
    orders: tuple[tuple[int, ...], ...] = ()
    log_sigma: tuple[float, ...] = ()
    log_cutoff: float = 4.0
    simoncelli_levels: tuple[int, ...] = (1,)
    padding: FilterPadding = FilterPadding.MIRROR
    constant: float = 0.0
    grid: FourierGrid = FourierGrid.DFT
    steering: bool = False
    tensor_sigma: float | None = None

    def __post_init__(self) -> None:
        _native.validate_block("RieszSettings", self)


@dataclass(frozen=True)
class SeparableWaveletSettings:
    """IBSI separable wavelet bands and boundary conditions."""

    name: str = "haar"
    bands: tuple[str, ...] = ()
    level: int = 1
    padding: FilterPadding = FilterPadding.MIRROR
    constant: float = 0.0
    pooling: OrientationPooling = OrientationPooling.NONE
    planar: bool = False

    def __post_init__(self) -> None:
        _native.validate_block("SeparableWaveletSettings", self)


@dataclass(frozen=True)
class TransformSettings:
    """Ordered image selection with independently validated options for each filter."""

    image_types: tuple[ImageType, ...] = (ImageType.ORIGINAL,)
    scratch_mib: int = 64
    wavelet: WaveletSettings = WaveletSettings()
    log: LoGSettings = LoGSettings()
    lbp2d: LBP2DSettings = LBP2DSettings()
    lbp3d: LBP3DSettings = LBP3DSettings()
    gradient: GradientSettings = GradientSettings()
    mean: MeanSettings = MeanSettings()
    laws: LawsSettings = LawsSettings()
    gabor: GaborSettings = GaborSettings()
    simoncelli: SimoncelliSettings = SimoncelliSettings()
    riesz: RieszSettings = RieszSettings()
    separable_wavelet: SeparableWaveletSettings = SeparableWaveletSettings()

    def __post_init__(self) -> None:
        _native.validate_block("TransformSettings", self)
