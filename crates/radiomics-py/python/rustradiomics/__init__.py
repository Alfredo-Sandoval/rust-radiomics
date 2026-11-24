"""Radiomics on the CPU. Array coordinates and spacing follow NumPy (z, y, x) order."""

from importlib.metadata import version

from rustradiomics._results import MapExport, Result
from rustradiomics._types import VoxelSink, VoxelSinkFactory
from rustradiomics._workflows import export, export_files, extract, extract_files
from rustradiomics.config.settings import FEATURES, Settings, list_features
from rustradiomics.config.spatial import IbsiInterpolation, ImageSettings, Interpolator
from rustradiomics.config.transforms import ImageType, TransformSettings

__version__ = version("rustradiomics")
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
