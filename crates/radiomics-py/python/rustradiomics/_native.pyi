from collections.abc import Mapping, Sequence
from os import PathLike

import numpy as np
from numpy.typing import NDArray

from rustradiomics._types import (
    ExecutionMetadata,
    HostArray,
    MapDtype,
    VoxelArray,
    VoxelSink,
    VoxelSinkFactory,
)
from rustradiomics.config.settings import Settings
from rustradiomics.config.spatial import ImageSettings
from rustradiomics.config.transforms import TransformSettings

def default_features() -> list[str]: ...
def list_features(families: Sequence[str], mode: str, ndim: int | None = None) -> list[str]: ...
def validate_settings(obj: Settings, ndim: int) -> None: ...
def validate_block(class_name: str, obj: object) -> None: ...
def load_params(path: str | PathLike[str]) -> dict[str, object]: ...
def settings_from_params(params: Mapping[str, object]) -> dict[str, object]: ...
def parse_params(
    params: Mapping[str, object], ndim: int
) -> tuple[dict[str, object], dict[str, object], dict[str, object]]: ...
def extract(
    image: HostArray,
    mask: HostArray,
    settings: Settings,
    transforms: TransformSettings,
    spacing: Sequence[float] | None,
    voxel: bool,
    map_dtype: MapDtype,
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> tuple[dict[str, float | VoxelArray], ExecutionMetadata]: ...
def extract_files(
    image_path: str | PathLike[str],
    mask_path: str | PathLike[str],
    settings: Settings,
    image_settings: ImageSettings,
    transforms: TransformSettings,
    voxel: bool,
    map_dtype: MapDtype,
    sink: VoxelSink | None = None,
    sink_factory: VoxelSinkFactory | None = None,
) -> tuple[dict[str, float | VoxelArray], ExecutionMetadata]: ...
def label_lesions(
    mask: HostArray, label: int, min_voxels: int, max_lesions: int | None
) -> NDArray[np.int32]: ...
def extract_lesions(
    image: HostArray,
    mask: HostArray,
    settings: Settings,
    transforms: TransformSettings,
    spacing: Sequence[float] | None,
    min_voxels: int,
    max_lesions: int | None,
) -> list[
    tuple[int, int, list[int], list[int], dict[str, float], ExecutionMetadata]
]: ...
def extract_lesions_files(
    image_path: str | PathLike[str],
    mask_path: str | PathLike[str],
    settings: Settings,
    image_settings: ImageSettings,
    transforms: TransformSettings,
    min_voxels: int,
    max_lesions: int | None,
) -> list[
    tuple[int, int, list[int], list[int], dict[str, float], ExecutionMetadata]
]: ...
def export(
    image: HostArray,
    mask: HostArray,
    output_dir: str | PathLike[str],
    settings: Settings,
    transforms: TransformSettings,
    spacing: Sequence[float] | None,
    allow_synthetic_geometry: bool,
    overwrite: bool,
    scratch_dir: str | PathLike[str] | None,
    max_scratch_mib: int | None,
) -> tuple[str, dict[str, str], ExecutionMetadata]: ...
def export_files(
    image_path: str | PathLike[str],
    mask_path: str | PathLike[str],
    output_dir: str | PathLike[str],
    settings: Settings,
    image_settings: ImageSettings,
    transforms: TransformSettings,
    overwrite: bool,
    scratch_dir: str | PathLike[str] | None,
    max_scratch_mib: int | None,
) -> tuple[str, dict[str, str], ExecutionMetadata]: ...
