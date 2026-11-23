import os
from typing import Literal, overload

import numpy as np
from numpy.typing import NDArray

from rustrad._results import MapExport, Result
from rustrad._types import FloatArray, HostArray, MapDtype, VoxelSink, VoxelSinkFactory
from rustrad.config.settings import Settings
from rustrad.config.spatial import ImageSettings
from rustrad.config.transforms import TransformSettings

@overload
def extract(
    image: HostArray,
    mask: HostArray,
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    mode: Literal["roi"] = "roi",
    map_dtype: MapDtype = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result[float]: ...
@overload
def extract(
    image: HostArray,
    mask: HostArray,
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    mode: Literal["voxel"],
    map_dtype: Literal["float64"] = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result[FloatArray]: ...
@overload
def extract(
    image: HostArray,
    mask: HostArray,
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    mode: Literal["voxel"],
    map_dtype: Literal["float32"],
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result[NDArray[np.float32]]: ...
@overload
def extract(
    image: HostArray,
    mask: HostArray,
    *,
    settings: Settings | None = None,
    transforms: TransformSettings | None = None,
    spacing: tuple[float, ...] | None = None,
    mode: Literal["roi", "voxel"],
    map_dtype: MapDtype = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result: ...
@overload
def extract_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    mode: Literal["roi"] = "roi",
    map_dtype: MapDtype = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result[float]: ...
@overload
def extract_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    mode: Literal["voxel"],
    map_dtype: Literal["float64"] = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result[FloatArray]: ...
@overload
def extract_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    mode: Literal["voxel"],
    map_dtype: Literal["float32"],
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result[NDArray[np.float32]]: ...
@overload
def extract_files(
    image_path: str | os.PathLike[str],
    mask_path: str | os.PathLike[str],
    *,
    settings: Settings | None = None,
    image_settings: ImageSettings | None = None,
    transforms: TransformSettings | None = None,
    mode: Literal["roi", "voxel"],
    map_dtype: MapDtype = "float64",
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> Result: ...
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
) -> MapExport: ...
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
) -> MapExport: ...
