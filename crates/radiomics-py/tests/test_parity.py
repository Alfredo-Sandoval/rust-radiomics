"""Compare the public NumPy extraction with mlx-radiomics v1.1.0 W11 records."""

import json
from collections.abc import Mapping
from pathlib import Path
from typing import Literal, cast

import numpy as np
import pytest
from rustrad import Settings, extract, list_features
from rustrad._types import HostArray, VoxelArray
from rustrad.errors import InvalidInputError

DATA = Path(__file__).resolve().parents[2] / "radiomics/tests/data/reference"
GOLDENS = sorted((DATA / "golden").glob("*.mlx.json"))
PYRADIOMICS = {
    "covariance_divisor": "n",
    "ellipsoid_area": "exact",
    "kurtosis": "raw",
    "zero_variance_moments": "nan",
    "ngtdm_singular_cases": "unadjusted",
    "neighborhood_support": "distances",
    "bin_origin": "zero-lattice",
    "constant_roi_bin": "middle",
    "mask_extrapolation": "nearest",
    "spline_boundary": "mirror",
}


@pytest.mark.parametrize("path", GOLDENS, ids=lambda path: path.name.removesuffix(".mlx.json"))
def test_w11_parity(path: Path) -> None:
    record = json.loads(path.read_text())
    case = json.loads((DATA / "cases" / f"{record['case']}.json").read_text())
    shape = tuple(case["shape"])
    image = np.array(case["image"]).reshape(shape)
    mask = np.array(case["mask"]).reshape(shape)
    voxel = record["mode"] == "voxel"
    kwargs = record["settings"].copy()
    convention = kwargs.pop("convention")
    kwargs["distances"] = tuple(kwargs["distances"])
    features = tuple(record["features"]) or list_features(mode=record["mode"], ndim=image.ndim)
    settings = Settings(
        **(PYRADIOMICS if convention == "pyradiomics" else {}), **kwargs, features=features
    )

    mode = cast("Literal['roi', 'voxel']", record["mode"])
    spacing = cast("tuple[float, ...]", tuple(case["spacing"]))

    def compute() -> Mapping[str, float | VoxelArray]:
        result = extract(image, mask, settings=settings, spacing=spacing, mode=mode)
        return {name.removeprefix("original_"): value for name, value in result.features.items()}

    if expected_error := record.get("expected_error"):
        with pytest.raises(InvalidInputError) as exc:
            compute()
        assert str(exc.value) == expected_error
        return
    result = compute()
    assert tuple(result) == features
    for key, expected in record["features"].items():
        value = result[key]
        if voxel:
            assert isinstance(value, np.ndarray)
            assert value.shape == shape and value.dtype == np.float64
            actual = value.ravel()
        else:
            assert isinstance(value, float)
            actual = value
        rtol = 3e-6 if key.startswith(("shape_", "shape2D_")) else 2e-5
        np.testing.assert_allclose(
            actual,
            np.asarray(expected, dtype=np.float64),
            rtol=rtol,
            atol=2e-6,
            equal_nan=True,
            err_msg=f"{path.name}: {key}",
        )


def mean(image: HostArray, roi: HostArray | None = None) -> float:
    roi = np.ones(image.shape, dtype=bool) if roi is None else roi
    settings = Settings(features=("firstorder_Mean",))
    result = extract(image, roi, settings=settings, spacing=(1.0, 2.0))
    return result.features["original_firstorder_Mean"]


@pytest.mark.parametrize("dtype", ["int8", "uint16", ">i4", ">u8", ">f4", "float16", "float64"])
def test_strided_and_swapped_arrays(dtype: str) -> None:
    image = np.arange(24).astype(dtype).reshape(4, 6)[::-2, 1::2]
    np.testing.assert_allclose(mean(image, roi=np.ones(image.shape, dtype=">i8")), image.mean())


@pytest.mark.parametrize("dtype", ["int64", "uint64", ">i8", ">u8"])
def test_integer_conversion_requires_exact_float64(dtype: str) -> None:
    a = np.full((2, 2), 2**60, dtype=dtype)
    assert mean(a) == 2**60
    a[0, 0] = 2**60 + 1
    with pytest.raises(InvalidInputError, match="float64 cannot represent exactly"):
        mean(a)


def test_mask_and_image_dtype_validation() -> None:
    image = np.arange(4.0).reshape(2, 2)
    with pytest.raises(InvalidInputError, match="integer labels"):
        mean(image, roi=image)
    with pytest.raises(InvalidInputError, match="image must be real numeric"):
        mean(image.astype(np.complex128))


def test_large_readonly_strided_mask_and_unaligned_swapped_image() -> None:
    shape = (4097, 2048)
    backing = bytearray(1 + 2 * shape[0] * shape[1])
    swapped = np.dtype(np.int16).newbyteorder("S")
    image = np.ndarray(shape, dtype=swapped, buffer=backing, offset=1)
    raw_mask = np.zeros(shape, dtype=np.uint8)
    rows = [0, 2048, 4096]
    image[rows, 7] = [-7, 19, 36]
    raw_mask[rows, 7] = [128, 1, 255]
    mask = raw_mask.view(np.bool_)[::-1]
    image = image[::-1]
    image.flags.writeable = mask.flags.writeable = False
    assert not image.flags.aligned
    result = extract(image, mask, settings=Settings(features=("firstorder_Mean",)), spacing=(1, 1))
    assert result.metadata["roi_voxels"] == 3
    assert result.metadata["array_dtype"] == str(swapped)
    assert result.features["original_firstorder_Mean"] == 16.0


def test_noncanonical_true_bytes_in_boolean_mask() -> None:
    data = np.array([[1, 0], [127, 255]], dtype=np.uint8)
    mask = data.view(np.bool_)
    image = np.array([[2.0, 100.0], [4.0, 6.0]])
    result = extract(image, mask, settings=Settings(features=("firstorder_Mean",)), spacing=(1, 1))
    assert result.features["original_firstorder_Mean"] == 4
