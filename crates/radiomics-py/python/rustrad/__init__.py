"""Radiomics on the CPU. Array coordinates and spacing follow NumPy (z, y, x) order."""

from importlib.metadata import version

from rustrad._results import MapExport, Result
from rustrad._types import VoxelSink, VoxelSinkFactory
from rustrad._workflows import export, export_files, extract, extract_files
from rustrad.config.settings import FEATURES, Settings, list_features
from rustrad.config.spatial import IbsiInterpolation, ImageSettings, Interpolator
from rustrad.config.transforms import ImageType, TransformSettings

__version__ = version("rustrad")
__device__ = "cpu"

__all__ = [
    "FEATURES",
    "IbsiInterpolation",
    "ImageSettings",
    "ImageType",
    "Interpolator",
    "MapExport",
    "Result",
    "Settings",
    "TransformSettings",
    "VoxelSink",
    "VoxelSinkFactory",
    "__version__",
    "export",
    "export_files",
    "extract",
    "extract_files",
    "list_features",
]


def __dir__() -> list[str]:
    return sorted(__all__)
