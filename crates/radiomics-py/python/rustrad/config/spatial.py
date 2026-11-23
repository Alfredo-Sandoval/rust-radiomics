"""Physical-grid options, with spacing in NumPy axis order."""

from dataclasses import dataclass
from enum import StrEnum

from rustrad import _native


class Interpolator(StrEnum):
    """Interpolator used by the CPU resampler."""

    NEAREST = "nearest"
    LINEAR = "linear"
    BSPLINE = "bspline"


@dataclass(frozen=True)
class IbsiInterpolation:
    """Explicit IBSI interpolation settings for the file-based extraction and export workflows."""

    spacing: tuple[float, ...]
    round_intensities: bool = True
    mask_threshold: float = 0.5
    image_interpolator: Interpolator = Interpolator.LINEAR

    def __post_init__(self) -> None:
        _native.validate_block("IbsiInterpolation", self)


@dataclass(frozen=True)
class ImageSettings:
    """Spatial preprocessing options for the file-based extraction and export workflows."""

    resampled_spacing: tuple[float, ...] | None = None
    interpolator: Interpolator = Interpolator.BSPLINE
    correct_mask: bool = False
    pad_distance: int = 5
    ibsi_interpolation: IbsiInterpolation | None = None

    def __post_init__(self) -> None:
        _native.validate_block("ImageSettings", self)
