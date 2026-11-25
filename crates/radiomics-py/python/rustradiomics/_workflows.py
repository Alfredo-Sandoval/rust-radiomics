"""Array and file workflow entry points for the CPU engine."""

import os
import warnings
from collections.abc import Generator
from contextlib import ExitStack, contextmanager
from pathlib import Path
from typing import Literal

import numpy as np
from numpy.typing import NDArray

from rustradiomics import _native
from rustradiomics._results import Lesion, MapExport, Result
from rustradiomics._types import HostArray, MapDtype, VoxelSink, VoxelSinkFactory
from rustradiomics.config.settings import Settings
from rustradiomics.config.spatial import ImageSettings
from rustradiomics.config.transforms import TransformSettings
from rustradiomics.errors import InvalidInputError


@contextmanager
def _sink_context(factory: VoxelSinkFactory, name: str) -> Generator[VoxelSink | None]:
    # Enter and unwind user resources in Python so cleanup runs with the active exception.
    with ExitStack() as resources:
        sink = resources.enter_context(factory(name))
        yield sink if sink else None


def _validate_request(mode: str, map_dtype: str, sink: object, sink_factory: object) -> None:
    if mode not in ("roi", "voxel"):
        raise InvalidInputError("mode must be 'roi' or 'voxel'")
    if sink is not None and sink_factory is not None:
        raise InvalidInputError("pass only one of sink or sink_factory")
    if mode == "roi" and (sink is not None or sink_factory is not None):
        raise InvalidInputError("sink and sink_factory require mode='voxel'")
    if sink is not None and not callable(sink):
        raise InvalidInputError("sink must be callable")
    if sink_factory is not None and not callable(sink_factory):
        raise InvalidInputError("sink_factory must be callable")
    if map_dtype not in ("float64", "float32"):
        raise InvalidInputError("map_dtype must be 'float64' or 'float32'")


def extract(
    image: HostArray,
    mask: HostArray,
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    mode: Literal["roi", "voxel"] = "roi",
    map_dtype: MapDtype = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result:
    """Extract CPU features from aligned arrays with spacing in NumPy axis order."""
    _validate_request(mode, map_dtype, sink, sink_factory)
    options = transforms if transforms is not None else TransformSettings()
    if spacing is None:
        warnings.warn(
            "Array extraction assumes 1 mm spacing on every axis. Pass spacing explicitly "
            "in NumPy axis order, or use extract_files to read physical geometry.",
            UserWarning,
            stacklevel=2,
        )
    features, metadata = _native.extract(
        image,
        mask,
        settings if settings is not None else Settings(),
        options,
        spacing,
        mode == "voxel",
        map_dtype,
        sink,
        sink_factory,
    )
    return Result(features, metadata)


def extract_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    mode: Literal["roi", "voxel"] = "roi",
    map_dtype: MapDtype = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result:
    """Extract CPU features from a NIfTI or NRRD image and mask."""
    _validate_request(mode, map_dtype, sink, sink_factory)
    features, metadata = _native.extract_files(
        Path(image_path).expanduser(),
        Path(mask_path).expanduser(),
        settings if settings is not None else Settings(),
        image_settings if image_settings is not None else ImageSettings(),
        transforms if transforms is not None else TransformSettings(),
        mode == "voxel",
        map_dtype,
        sink,
        sink_factory,
    )
    return Result(features, metadata)


def _check_lesion_selection(min_voxels: int, max_lesions: int | None) -> None:
    if type(min_voxels) is not int or min_voxels < 1:
        raise InvalidInputError("min_voxels must be a positive integer")
    if max_lesions is not None and (type(max_lesions) is not int or max_lesions < 1):
        raise InvalidInputError("max_lesions must be a positive integer or None")


def label_lesions(
    mask: HostArray,
    *,
    label: int = 1,
    min_voxels: int = 1,
    max_lesions: int | None = None,
) -> NDArray[np.int32]:
    """Number the connected lesions of one mask label, largest first.

    The result has the mask's shape: 0 outside the kept lesions, 1 for the largest
    lesion, 2 for the next. Voxels that touch by a face, an edge or a corner belong to
    one lesion. Equal-sized lesions keep the order of their first voxel in C order. Pass
    the map as ``mask`` with ``Settings(label=k)`` to extract or export lesion ``k``.
    ``min_voxels`` drops smaller lesions before ``max_lesions`` keeps the largest ones.
    """
    _check_lesion_selection(min_voxels, max_lesions)
    return _native.label_lesions(mask, label, min_voxels, max_lesions)


def extract_lesions(
    image: HostArray,
    mask: HostArray,
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    min_voxels: int = 1,
    max_lesions: int | None = None,
) -> tuple[Lesion, ...]:
    """Extract ROI features for each connected lesion of ``settings.label``.

    Every kept lesion is extracted as its own ROI, exactly as ``extract`` would with a
    mask holding only that lesion. Lesions are returned largest first.
    """
    _check_lesion_selection(min_voxels, max_lesions)
    if spacing is None:
        warnings.warn(
            "Array extraction assumes 1 mm spacing on every axis. Pass spacing explicitly "
            "in NumPy axis order, or use extract_lesions_files to read physical geometry.",
            UserWarning,
            stacklevel=2,
        )
    lesions = _native.extract_lesions(
        image,
        mask,
        settings if settings is not None else Settings(),
        transforms if transforms is not None else TransformSettings(),
        spacing,
        min_voxels,
        max_lesions,
    )
    return tuple(
        Lesion(index, voxels, tuple(lower), tuple(upper), Result(features, metadata))
        for index, voxels, lower, upper, features, metadata in lesions
    )


def extract_lesions_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    min_voxels: int = 1,
    max_lesions: int | None = None,
) -> tuple[Lesion, ...]:
    """Extract ROI features for each connected lesion of a NIfTI or NRRD mask label.

    The pair is loaded and spatially processed once, and the label is split on the
    processed grid described by ``metadata["image_geometry"]``.
    """
    _check_lesion_selection(min_voxels, max_lesions)
    lesions = _native.extract_lesions_files(
        Path(image_path).expanduser(),
        Path(mask_path).expanduser(),
        settings if settings is not None else Settings(),
        image_settings if image_settings is not None else ImageSettings(),
        transforms if transforms is not None else TransformSettings(),
        min_voxels,
        max_lesions,
    )
    return tuple(
        Lesion(index, voxels, tuple(lower), tuple(upper), Result(features, metadata))
        for index, voxels, lower, upper, features, metadata in lesions
    )


def _check_export_options(overwrite: bool, max_scratch_mib: int | None) -> None:
    if type(overwrite) is not bool:
        raise InvalidInputError("overwrite must be a boolean")
    if max_scratch_mib is not None and (type(max_scratch_mib) is not int or max_scratch_mib < 1):
        raise InvalidInputError("max_scratch_mib must be a positive integer")


def export(
    image: HostArray,
    mask: HostArray,
    output_dir: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    allow_synthetic_geometry: bool = False,
    overwrite: bool = False,
    scratch_dir: str | os.PathLike[str] | None = None,
    max_scratch_mib: int | None = None,
) -> MapExport:
    """Export CPU voxel maps from aligned NumPy arrays."""
    if type(allow_synthetic_geometry) is not bool:
        raise InvalidInputError("allow_synthetic_geometry must be a boolean")
    if not allow_synthetic_geometry:
        warnings.warn(
            "Array export writes synthetic geometry (zero origin, identity direction; "
            "1 mm spacing if omitted). Use export_files to retain source geometry, or "
            "set allow_synthetic_geometry=True to acknowledge this coordinate system.",
            UserWarning,
            stacklevel=2,
        )
    _check_export_options(overwrite, max_scratch_mib)
    report, maps, metadata = _native.export(
        image,
        mask,
        Path(output_dir).expanduser().resolve(),
        settings if settings is not None else Settings(),
        transforms if transforms is not None else TransformSettings(),
        spacing,
        allow_synthetic_geometry,
        overwrite,
        Path(scratch_dir).expanduser() if scratch_dir is not None else None,
        max_scratch_mib,
    )
    return MapExport(Path(report), {name: Path(path) for name, path in maps.items()}, metadata)


def export_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    output_dir: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    overwrite: bool = False,
    scratch_dir: str | os.PathLike[str] | None = None,
    max_scratch_mib: int | None = None,
) -> MapExport:
    """Export CPU voxel maps from a NIfTI or NRRD image and mask."""
    _check_export_options(overwrite, max_scratch_mib)
    report, maps, metadata = _native.export_files(
        Path(image_path).expanduser(),
        Path(mask_path).expanduser(),
        Path(output_dir).expanduser().resolve(),
        settings if settings is not None else Settings(),
        image_settings if image_settings is not None else ImageSettings(),
        transforms if transforms is not None else TransformSettings(),
        overwrite,
        Path(scratch_dir).expanduser() if scratch_dir is not None else None,
        max_scratch_mib,
    )
    return MapExport(Path(report), {name: Path(path) for name, path in maps.items()}, metadata)
