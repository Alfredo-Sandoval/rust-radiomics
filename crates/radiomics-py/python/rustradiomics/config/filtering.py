"""String-valued choices used by CPU image filters."""

from enum import StrEnum


class FilterPadding(StrEnum):
    """IBSI 2 boundary-extension methods (manual section 2.2)."""

    CONSTANT = "constant"
    NEAREST = "nearest"
    PERIODIC = "periodic"
    MIRROR = "mirror"


class FourierGrid(StrEnum):
    """Sampling convention for full-image Fourier-domain filters."""

    DFT = "dft"
    CENTERED = "centered"


class LogKernel(StrEnum):
    """Laplacian-of-Gaussian construction for ``ImageType.LOG``."""

    RECURSIVE = "recursive"
    SAMPLED = "sampled"


class RieszBase(StrEnum):
    """Base filter to which the Riesz transform is applied."""

    LOG = "log"
    SIMONCELLI = "simoncelli"


class GaborResponse(StrEnum):
    """Which component of the complex Gabor response map to return."""

    MODULUS = "modulus"
    REAL = "real"
    IMAGINARY = "imaginary"


class OrientationPooling(StrEnum):
    """Pooling across rotated versions of a directionally sensitive filter."""

    NONE = "none"
    AVERAGE = "average"
    MAX = "max"
    MIN = "min"


class LBPMethod(StrEnum):
    """Coding method for local binary patterns."""

    DEFAULT = "default"
    ROTATION_INVARIANT = "ror"
    UNIFORM = "uniform"
    NONROTATION_UNIFORM = "nri_uniform"
    VARIANCE = "var"
