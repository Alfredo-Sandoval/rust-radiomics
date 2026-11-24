"""Public Python contracts mirrored from mlx-radiomics v1.1.0."""

import json
import os
import pickle
import subprocess
import sys
import textwrap
import warnings
from collections.abc import Callable, Generator, Mapping
from contextlib import AbstractContextManager, contextmanager, suppress
from dataclasses import FrozenInstanceError, asdict, replace
from importlib.metadata import version
from pathlib import Path
from types import TracebackType
from typing import TYPE_CHECKING, Literal, cast

import numpy as np
import pytest
import rustradiomics as r
from numpy.typing import NDArray
from rustradiomics._types import ExecutionMetadata, MapDtype, VoxelArray
from rustradiomics.config.settings import Kurtosis
from rustradiomics.config.transforms import LoGSettings, MeanSettings
from rustradiomics.errors import (
    InvalidInputError,
    MissingDependencyError,
    ModelExecutionError,
)

if TYPE_CHECKING:
    from _typeshed import DataclassInstance


def _noop(centers: NDArray[np.int32], features: Mapping[str, NDArray[np.float32]], /) -> None:
    return None


def scalar(result: r.Result, key: str) -> float:
    value = result.features[key]
    assert isinstance(value, float)
    return value


def grid(result: r.Result, key: str) -> VoxelArray:
    value = result.features[key]
    assert isinstance(value, np.ndarray)
    return value


def test_public_namespace() -> None:
    assert r.__all__ == [
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
    assert dir(r) == sorted(r.__all__)
    assert not hasattr(r, "run")
    assert r.__version__ == version("rustradiomics")
    assert r.__device__ == "cpu"


@pytest.mark.parametrize(("ndim", "roi_count"), [(None, 200), (2, 156), (3, 191)])
def test_feature_discovery(ndim: int, roi_count: int) -> None:
    names = r.list_features(ndim=ndim)
    assert isinstance(names, tuple) and len(set(names)) == len(names) == roi_count
    assert names == tuple(key for key in r.FEATURES if key in names)
    assert len(r.list_features(ndim=ndim, mode="voxel")) == 147
    assert r.list_features("shape", "shape2D", "localintensity", mode="voxel") == ()
    assert r.list_features("firstorder", "shape", "shape2D", "localintensity", ndim=2) == (
        r.list_features("firstorder", "shape2D")
    )
    assert r.list_features("glcm", "firstorder", "glcm") == r.list_features("firstorder", "glcm")
    r.Settings(features=names).validate(ndim or 3)


def test_unknown_family_message() -> None:
    with pytest.raises(InvalidInputError) as exc:
        r.list_features("glmc", "wrong", "glmc")
    assert str(exc.value) == (
        "unknown feature families: ['glmc', 'wrong']; choose from "
        "('firstorder', 'glcm', 'gldm', 'ngtdm', 'glrlm', 'glszm', 'shape', 'shape2D', "
        "'gldzm', 'intensityhistogram', 'localintensity', 'ivh', 'momentinvariant')"
    )


@pytest.mark.parametrize("ndim", [True, 2.0, -1, 1000])
def test_invalid_dimension(ndim: object) -> None:
    with pytest.raises(InvalidInputError, match="^ndim must be 2, 3 or None$"):
        r.list_features(ndim=cast("int", ndim))


def test_exception_hierarchy() -> None:
    assert issubclass(InvalidInputError, ValueError)
    assert issubclass(ModelExecutionError, RuntimeError)
    assert issubclass(MissingDependencyError, ImportError)
    with pytest.raises(InvalidInputError, match="^mode must be 'roi' or 'voxel'$"):
        r.list_features(mode=cast("Literal['roi']", "volume"))


def test_settings_roundtrip() -> None:
    s = r.Settings()
    assert len(asdict(s)) == 42
    assert len(s.features) == 93 and len(set(s.features)) == 93
    assert s.bin_width == 25.0 and s.label == 1 and s.force2d_axis is None
    assert s.distances == (1,) and s.covariance_divisor == "n-1" and s.kurtosis == "excess"
    assert r.Settings(**asdict(s)) == s
    changed = replace(s, bin_width=3.5, features=("firstorder_Mean",), distances=(1, 2))
    changed.validate(2)
    assert changed.bin_width == 3.5 and s.bin_width == 25
    assert r.Settings(bin_count=16, bin_width=cast("float", "unused")).validate(2) is None


@pytest.mark.parametrize(
    "instance",
    [
        r.Settings(),
        r.IbsiInterpolation((0, 2, 2)),
        r.ImageSettings(),
        r.TransformSettings(),
        r.Result({"original_firstorder_Mean": 5.0}, cast("ExecutionMetadata", {"device": "cpu"})),
        r.MapExport(Path("/report.json"), {}, cast("ExecutionMetadata", {"device": "cpu"})),
    ],
)
def test_frozen_values_and_repr(instance: "DataclassInstance") -> None:
    assert repr(instance).startswith(type(instance).__name__ + "(")
    assert pickle.loads(pickle.dumps(instance)) == instance
    field = next(iter(asdict(instance)))
    with pytest.raises(FrozenInstanceError):
        setattr(instance, field, getattr(instance, field))


def test_typed_spatial_and_transform_options() -> None:
    assert tuple(map(str, r.Interpolator)) == ("nearest", "linear", "bspline")
    assert tuple(map(str, r.ImageType)) == (
        "original",
        "square",
        "squareroot",
        "logarithm",
        "exponential",
        "gradient",
        "wavelet",
        "log",
        "lbp-2D",
        "lbp-3D",
        "mean",
        "laws",
        "gabor",
        "simoncelli",
        "riesz",
        "separablewavelet",
    )
    assert repr(r.IbsiInterpolation((1, 2))) == (
        "IbsiInterpolation(spacing=(1, 2), round_intensities=True, mask_threshold=0.5, "
        "image_interpolator=<Interpolator.LINEAR: 'linear'>)"
    )
    assert r.ImageSettings().interpolator is r.Interpolator.BSPLINE
    opts = r.TransformSettings(image_types=(r.ImageType.LOG,), log=LoGSettings(sigma=(2.5,)))
    assert opts.log.sigma == (2.5,) and opts.wavelet.name == "coif1"


@pytest.mark.parametrize(
    ("build", "message"),
    [
        (
            lambda: r.Settings(bin_count=-1).validate(2),
            "bin_count must be a positive integer below 2**31",
        ),
        (lambda: r.Settings(bin_width=np.nan).validate(3), "bin_width must be finite and positive"),
        (lambda: r.Settings(distances=(1, 1)).validate(2), "distances must be unique"),
        (
            lambda: r.Settings(force2d_axis=cast("int", True)).validate(2),
            "force2d_axis must be an integer indexing a NumPy image axis",
        ),
        (lambda: r.Settings(features=("bad",)).validate(3), "unsupported features: ['bad']"),
        (
            lambda: r.Settings(kurtosis=cast("Kurtosis", "flat")).validate(2),
            "kurtosis must be one of excess, raw",
        ),
        (
            lambda: r.IbsiInterpolation((1,)),
            "IBSI spacing requires 2 or 3 finite nonnegative values",
        ),
        (
            lambda: r.IbsiInterpolation((1, 2), mask_threshold=0),
            "mask_threshold must be finite and in (0, 1]",
        ),
        (
            lambda: r.ImageSettings(interpolator=cast("r.Interpolator", "bspline")),
            "interpolator must be an Interpolator value",
        ),
        (
            lambda: r.TransformSettings(image_types=cast("tuple[r.ImageType, ...]", ("original",))),
            "image_types must contain typed ImageType entries",
        ),
        (
            lambda: r.TransformSettings(image_types=(r.ImageType.LOG,)),
            "LoG requires explicit positive sigma values",
        ),
        (lambda: r.TransformSettings(mean=cast("MeanSettings", {})), "mean must be MeanSettings"),
        (lambda: MeanSettings(support=2), "mean_support must be a positive odd integer"),
    ],
)
def test_validation_errors(build: Callable[[], object], message: str) -> None:
    with pytest.raises(InvalidInputError) as exc:
        build()
    assert str(exc.value) == message


def test_configuration_loads_without_execution_dependencies() -> None:
    program = textwrap.dedent("""
        import importlib.abc
        import sys
        class NoExecutionRuntime(importlib.abc.MetaPathFinder):
            def find_spec(self, fullname, path=None, target=None):
                if fullname.split(".")[0] in {"mlx", "SimpleITK", "mlxradiomics", "radiomics"}:
                    raise AssertionError(fullname)
        sys.meta_path.insert(0, NoExecutionRuntime())
        from rustradiomics import ImageSettings, TransformSettings
        from rustradiomics.config.parameters import parse_params
        parsed = parse_params({"setting": {"label": 2}}, ndim=3)
        assert parsed.settings.label == 2
        assert parsed.image_settings == ImageSettings() and not parsed.image_settings.correct_mask
        assert parsed.transforms == TransformSettings() and parsed.transforms.image_types
    """)
    subprocess.run([sys.executable, "-c", program], check=True)


@pytest.mark.parametrize(
    ("mode", "map_dtype", "sink", "message"),
    [
        ("volume", "float64", None, "mode must be 'roi' or 'voxel'"),
        ("roi", "float64", _noop, "sink and sink_factory require mode='voxel'"),
        ("voxel", "float16", None, "map_dtype must be 'float64' or 'float32'"),
    ],
)
def test_invalid_request_precedes_file_io(
    tmp_path: Path,
    mode: str,
    map_dtype: str,
    sink: r.VoxelSink | None,
    message: str,
) -> None:
    with pytest.raises(InvalidInputError) as exc:
        r.extract_files(
            tmp_path / "absent.nii",
            tmp_path / "absent-mask.nii",
            mode=cast("Literal['roi']", mode),
            map_dtype=cast("MapDtype", map_dtype),
            sink=sink,
        )
    assert str(exc.value) == message


@pytest.mark.parametrize("dtype", ["int64", "float16", ">i2", ">f4"])
def test_array_export_writes_schema2_report_and_nifti(tmp_path: Path, dtype: str) -> None:
    import gzip
    import struct

    image = np.arange(6, dtype=dtype).reshape(2, 3)
    mask = np.ones(image.shape, dtype=bool)
    spacing = (2.0, 3.0)
    settings = r.Settings(features=("firstorder_Mean",))
    with pytest.warns(UserWarning, match="synthetic geometry"):
        saved = r.export(image, mask, tmp_path, spacing=spacing, settings=settings)
    assert saved.report == tmp_path / "features.json"
    assert saved.metadata["device"] == "cpu"
    report = json.loads(saved.report.read_text())
    execution = report["details"]["execution"]
    assert execution["array_dtype"] == saved.metadata["array_dtype"] == str(image.dtype)
    assert execution["runtime"]["python"] == saved.metadata["runtime"]["python"]
    assert saved.metadata["runtime"]["python"] == sys.version.split()[0]
    assert report["schema_version"] == 2
    assert report["maps"] == {key: str(path) for key, path in saved.maps.items()}
    path = saved.maps["original_firstorder_Mean"]
    assert path.is_absolute() and path.parent == tmp_path / "maps"
    with gzip.open(path, "rb") as stream:
        nifti = stream.read()
    assert struct.unpack_from("<h", nifti, 70) == (16,)  # float32
    offset = int(struct.unpack_from("<f", nifti, 108)[0])
    written = np.frombuffer(nifti, dtype="<f4", count=image.size, offset=offset).reshape(
        image.shape
    )
    expected = r.extract(image, mask, mode="voxel", spacing=spacing, settings=settings)
    np.testing.assert_array_equal(written, expected.features["original_firstorder_Mean"])
    with pytest.raises(InvalidInputError):
        r.export(
            image, mask, tmp_path, allow_synthetic_geometry=True, spacing=spacing, settings=settings
        )
    again = r.export(
        image,
        mask,
        tmp_path,
        allow_synthetic_geometry=True,
        overwrite=True,
        spacing=spacing,
        settings=settings,
    )
    assert again.maps == saved.maps


def test_array_extraction_warns_before_assuming_unit_spacing() -> None:
    image = np.arange(6.0).reshape(2, 3)
    mask = np.ones_like(image, dtype=bool)
    settings = r.Settings(features=("firstorder_TotalEnergy",))
    with pytest.warns(UserWarning, match="assumes 1 mm spacing"):
        result = r.extract(image, mask, settings=settings)
    assert result.features["original_firstorder_TotalEnergy"] == np.sum(image**2)
    assert result.metadata["device"] == "cpu"


@pytest.mark.parametrize("ndim", [2, 3])
@pytest.mark.parametrize("mode", ["roi", "voxel"])
@pytest.mark.parametrize("image_type", [r.ImageType.ORIGINAL, r.ImageType.SQUARE])
def test_empty_spacing_uses_unit_spacing(
    ndim: int, mode: Literal["roi", "voxel"], image_type: r.ImageType
) -> None:
    image = np.arange(2**ndim, dtype=np.int16).reshape((2,) * ndim)
    mask = np.ones_like(image, dtype=bool)
    settings = r.Settings(features=("firstorder_TotalEnergy",))
    transforms = r.TransformSettings(image_types=(image_type,))
    units = (1.0,) * ndim
    with warnings.catch_warnings():
        warnings.simplefilter("error", UserWarning)
        actual = r.extract(
            image, mask, spacing=(), settings=settings, transforms=transforms, mode=mode
        )
    expected = r.extract(
        image, mask, spacing=units, settings=settings, transforms=transforms, mode=mode
    )
    assert actual.metadata["spacing"] == units
    key = f"{image_type.value}_firstorder_TotalEnergy"
    np.testing.assert_array_equal(actual.features[key], expected.features[key])


def test_voxel_maps_and_streamed_values() -> None:
    image = np.arange(4).reshape(2, 2)
    mask = np.ones_like(image)
    settings = r.Settings(features=("firstorder_Mean",))
    spacing = (2.0, 3.0)
    retained = r.extract(image, mask, settings=settings, mode="voxel", spacing=spacing)
    mapped = retained.features["original_firstorder_Mean"]
    assert mapped.dtype == np.float64
    np.testing.assert_array_equal(mapped, np.full(image.shape, 1.5))
    collected = np.zeros_like(mapped)

    def sink(centers: NDArray[np.int32], batch: Mapping[str, NDArray[np.float32]]) -> None:
        collected[tuple(centers.T)] = batch["original_firstorder_Mean"]

    streamed = r.extract(image, mask, sink=sink, settings=settings, mode="voxel", spacing=spacing)
    assert streamed.features == {}
    np.testing.assert_array_equal(collected, mapped)


def test_public_array_label_selection_and_geometry() -> None:
    # The uint64 background would alias the positive int64 label after a float64 cast.
    label = 2**60
    image = np.array([[1.25, 2.25, 50.0], [3.25, 4.25, 50.0]], dtype=">f4")[:, ::-1]
    mask = np.array([[label, label, label + 1]] * 2, dtype=">u8")[:, ::-1]
    selection = r.Settings(label=label, features=("firstorder_Mean", "firstorder_TotalEnergy"))
    result = r.extract(image, mask, settings=selection, spacing=(2.0, 3.0))
    assert result.features["original_firstorder_Mean"] == 2.75
    expected_energy = np.sum(image[mask == label].astype(np.float64) ** 2) * 6
    assert result.features["original_firstorder_TotalEnergy"] == pytest.approx(expected_energy)
    assert result.metadata["array_shape"] == (2, 3)
    assert result.metadata["spacing"] == (2.0, 3.0)
    assert result.metadata["array_dtype"] == str(image.dtype)
    assert result.metadata["settings"]["features"] == selection.features
    assert result.metadata["roi_voxels"] == 4
    assert result.metadata["device"] == "cpu" and result.metadata["precision"] == "float64"


def test_sink_factory_lifetime_and_callback_error() -> None:
    events: list[tuple[str, str]] = []
    failing = LookupError("caller stopped")

    @contextmanager
    def factory(image_name: str) -> Generator[r.VoxelSink]:
        events.append(("open", image_name))

        def consume(
            centers: NDArray[np.int32], features: Mapping[str, NDArray[np.float32]]
        ) -> None:
            assert centers.dtype == np.int32
            assert features["original_firstorder_Mean"].dtype == np.float32
            raise failing

        try:
            yield consume
        finally:
            events.append(("close", image_name))

    image = np.arange(4).reshape(2, 2)
    with pytest.raises(LookupError) as exc:
        r.extract(
            image,
            np.ones_like(image),
            settings=r.Settings(features=("firstorder_Mean",)),
            spacing=(1, 2),
            mode="voxel",
            sink_factory=factory,
        )
    assert exc.value is failing
    assert events == [("open", "original"), ("close", "original")]


@pytest.mark.parametrize("dtype", ["float32", "float64"])
@pytest.mark.parametrize("empty_sink", [False, True], ids=["none", "falsey-callable"])
def test_factory_can_retain_one_image_and_stream_another(dtype: MapDtype, empty_sink: bool) -> None:
    class EmptySink(list[object]):
        def __call__(
            self, centers: NDArray[np.int32], features: Mapping[str, NDArray[np.float32]], /
        ) -> None:
            pytest.fail("a falsey per-image sink should retain its map")

    events: list[tuple[str, str]] = []
    batches: list[tuple[NDArray[np.int32], Mapping[str, NDArray[np.float32]]]] = []

    @contextmanager
    def factory(name: str) -> Generator[r.VoxelSink | None]:
        events.append(("open", name))
        try:
            if name == "original":
                yield EmptySink() if empty_sink else None
            else:
                yield lambda centers, features: batches.append((centers, features))
        finally:
            events.append(("close", name))

    image = np.arange(24, dtype=np.int16).reshape(4, 6)
    mask = ((image % 3) != 0).astype(np.uint8)
    settings = r.Settings(features=("firstorder_Mean",))
    transforms = r.TransformSettings(image_types=(r.ImageType.ORIGINAL, r.ImageType.SQUARE))
    expected = (
        r.extract(
            image,
            mask,
            mode="voxel",
            spacing=(1.0, 2.0),
            map_dtype="float64",
            settings=settings,
            transforms=transforms,
        )
        if dtype == "float64"
        else r.extract(
            image,
            mask,
            mode="voxel",
            spacing=(1.0, 2.0),
            map_dtype="float32",
            settings=settings,
            transforms=transforms,
        )
    )
    mixed = r.extract(
        image,
        mask,
        mode="voxel",
        spacing=(1.0, 2.0),
        map_dtype=dtype,
        settings=settings,
        transforms=transforms,
        sink_factory=factory,
    )
    assert events == [
        ("open", "original"),
        ("close", "original"),
        ("open", "square"),
        ("close", "square"),
    ]
    key = "original_firstorder_Mean"
    assert list(mixed.features) == [key]
    assert grid(mixed, key).dtype == dtype
    np.testing.assert_array_equal(mixed.features[key], expected.features[key])
    assert mixed.metadata["feature_validity"] == expected.metadata["feature_validity"]
    coords = np.concatenate([c for c, _ in batches])
    np.testing.assert_array_equal(coords, np.argwhere(mask))
    key = "square_firstorder_Mean"
    samples = np.concatenate([batch[key] for _, batch in batches])
    np.testing.assert_array_equal(samples, grid(expected, key)[tuple(coords.T)])


def test_factory_checks_exit_before_entering_and_chains_cleanup_errors() -> None:
    image = np.arange(4).reshape(2, 2)
    settings = r.Settings(features=("firstorder_Mean",))
    mask = np.ones_like(image)

    class MissingExit:
        def __enter__(self) -> r.VoxelSink:
            pytest.fail("the context manager must have an exit method before taking resources")

    def missing_exit(name: str) -> AbstractContextManager[r.VoxelSink | None]:
        return cast("AbstractContextManager[r.VoxelSink | None]", MissingExit())

    with pytest.raises(TypeError, match="context manager"):
        r.extract(
            image,
            mask,
            sink_factory=missing_exit,
            mode="voxel",
            spacing=(1.0, 1.0),
            settings=settings,
        )

    failed = LookupError("batch failed")
    cleanup = RuntimeError("close failed")

    class Manager:
        def __init__(self) -> None:
            # Python looks up the context methods on the type, even with same-named attributes.
            def unused(*args: object) -> None:
                pytest.fail("used an instance attribute")

            vars(self).update(__enter__=unused, __exit__=unused)

        def __enter__(self) -> r.VoxelSink:
            def consume(
                centers: NDArray[np.int32], features: Mapping[str, NDArray[np.float32]]
            ) -> None:
                raise failed

            return consume

        def __exit__(
            self,
            kind: type[BaseException] | None,
            value: BaseException | None,
            tb: TracebackType | None,
        ) -> None:
            assert sys.exception() is value is failed
            assert kind is LookupError and tb is not None
            raise cleanup

    with pytest.raises(RuntimeError) as exc:
        r.extract(
            image,
            mask,
            sink_factory=lambda name: Manager(),
            mode="voxel",
            spacing=(1.0, 1.0),
            settings=settings,
        )
    assert exc.value is cleanup
    assert cleanup.__context__ is failed


def test_suppressed_callback_failure_cannot_publish_an_incomplete_image() -> None:
    from contextlib import contextmanager

    failed = LookupError("second image failed")

    @contextmanager
    def factory(name: str) -> Generator[r.VoxelSink]:
        def consume(
            centers: NDArray[np.int32], features: Mapping[str, NDArray[np.float32]]
        ) -> None:
            if name == "square":
                raise failed

        with suppress(LookupError):
            yield consume

    image = np.arange(12, dtype=np.int16).reshape(3, 4)
    with pytest.raises(LookupError) as exc:
        r.extract(
            image,
            np.ones_like(image),
            mode="voxel",
            spacing=(1.0, 1.0),
            sink_factory=factory,
            settings=r.Settings(features=("firstorder_Mean",)),
            transforms=r.TransformSettings(image_types=(r.ImageType.ORIGINAL, r.ImageType.SQUARE)),
        )
    assert exc.value is failed


def test_compact_maps_preserve_background_and_nonfinite_values() -> None:
    image = np.full((3, 4), 1e20)
    mask = np.zeros_like(image, dtype=np.uint8)
    mask[1, 1:3] = 1
    settings = r.Settings(
        features=("firstorder_Energy", "firstorder_Skewness", "firstorder_Entropy"),
        bin_count=1,
        zero_variance_moments="nan",
    )
    wide = r.extract(image, mask, settings=settings, spacing=(1.0, 1.0), mode="voxel")
    compact = r.extract(
        image, mask, settings=settings, spacing=(1.0, 1.0), mode="voxel", map_dtype="float32"
    )
    assert wide.metadata["feature_validity"] == compact.metadata["feature_validity"]
    assert np.isposinf(compact.features["original_firstorder_Energy"][mask == 1]).all()
    assert np.isnan(compact.features["original_firstorder_Skewness"][mask == 1]).all()
    for name, values in compact.features.items():
        assert values.dtype == np.float32 and wide.features[name].dtype == np.float64
        np.testing.assert_array_equal(values.astype(np.float64), wide.features[name])
        np.testing.assert_array_equal(np.signbit(values), np.signbit(wide.features[name]))
        assert not np.signbit(values[mask == 0]).any()
        assert (values[mask == 0] == 0).all()


def test_file_and_array_derived_extraction_share_geometry(tmp_path: Path) -> None:
    image = np.arange(6, dtype=np.int16).reshape(2, 3)
    mask = np.ones_like(image)
    # Fastest axis first in the header, last axis first in the NumPy buffer.
    header = (
        "NRRD0005\ntype: short\ndimension: 2\nsizes: 3 2\nspace dimension: 2\n"
        "space directions: (3,0) (0,2)\nspace origin: (7,11)\nencoding: raw\nendian: little\n\n"
    ).encode("ascii")
    paths = tmp_path / "image.nrrd", tmp_path / "mask.nrrd"
    for path, array in zip(paths, (image, mask), strict=True):
        path.write_bytes(header + array.astype("<i2").tobytes())
    settings = r.Settings(features=("firstorder_Mean",))
    transforms = r.TransformSettings(
        image_types=(r.ImageType.ORIGINAL, r.ImageType.MEAN), mean=MeanSettings(support=1)
    )
    via_array = r.extract(image, mask, spacing=(2, 3), settings=settings, transforms=transforms)
    via_file = r.extract_files(str(paths[0]), paths[1], settings=settings, transforms=transforms)
    assert (
        via_array.features
        == via_file.features
        == {
            "original_firstorder_Mean": 2.5,
            "mean-M1-mirror_firstorder_Mean": 2.5,
        }
    )
    assert via_file.metadata.get("image_geometry") == {
        "shape": (2, 3),
        "spacing": (2, 3),
        "origin": (7, 11),
        "direction": (1, 0, 0, 1),
    }

    home_paths = [Path("~") / os.path.relpath(path, Path.home()) for path in paths]
    home_image, home_mask = home_paths
    assert (
        r.extract_files(home_image, home_mask, settings=settings, transforms=transforms).features
        == via_file.features
    )

    saved = r.export_files(
        paths[0], paths[1], tmp_path / "out", settings=settings, transforms=transforms
    )
    execution = json.loads(saved.report.read_text())["details"]["execution"]
    assert execution["array_dtype"] == saved.metadata["array_dtype"]
    assert saved.metadata["array_dtype"] == via_file.metadata["array_dtype"]
    assert execution["runtime"]["python"] == saved.metadata["runtime"]["python"]
    assert saved.metadata["runtime"]["python"] == via_file.metadata["runtime"]["python"]
    assert isinstance(saved, r.MapExport) and all(path.is_file() for path in saved.maps.values())
    assert saved.metadata.get("image_geometry") == via_file.metadata.get("image_geometry")
    assert saved.maps.keys() == via_file.features.keys()

    events: list[tuple[str, str]] = []
    collected: dict[str, NDArray[np.float64]] = {}

    @contextmanager
    def factory(name: str) -> Generator[r.VoxelSink]:
        events.append(("open", name))
        try:

            def consume(
                centers: NDArray[np.int32], batch: Mapping[str, NDArray[np.float32]]
            ) -> None:
                for key, values in batch.items():
                    target = collected.setdefault(key, np.zeros(image.shape))
                    target[tuple(centers.T)] = values

            yield consume
        finally:
            events.append(("close", name))

    retained = r.extract_files(
        paths[0],
        paths[1],
        settings=settings,
        transforms=transforms,
        mode="voxel",
        map_dtype="float32",
    )
    streamed = r.extract_files(
        paths[0],
        paths[1],
        settings=settings,
        transforms=transforms,
        mode="voxel",
        sink_factory=factory,
    )
    assert streamed.features == {} and collected.keys() == retained.features.keys()
    for key, values in retained.features.items():
        assert values.dtype == np.float32
        np.testing.assert_array_equal(collected[key], values)
    assert events == [
        (action, name) for name in ("original", "mean-M1-mirror") for action in ("open", "close")
    ]


@pytest.mark.parametrize("use_files", [False, True])
def test_export_follows_symlink_before_parent_directory(tmp_path: Path, use_files: bool) -> None:
    target = tmp_path / "physical" / "child"
    target.mkdir(parents=True)
    alias = tmp_path / "alias"
    alias.symlink_to(target, target_is_directory=True)
    destination = alias / ".." / "maps-out"
    settings = r.Settings(features=("firstorder_Mean",))
    if use_files:
        source = tmp_path / "input.nrrd"
        header = b"NRRD0005\ntype: uint8\ndimension: 2\nsizes: 2 2\nencoding: raw\n\n"
        source.write_bytes(header + bytes([1]) * 4)
        result = r.export_files(source, source, destination, settings=settings)
    else:
        image = np.ones((2, 2), dtype=np.uint8)
        result = r.export(
            image, image, destination, allow_synthetic_geometry=True, settings=settings
        )
    expected = tmp_path.resolve() / "physical" / "maps-out"
    assert result.report == expected / "features.json"
    assert result.report.is_file()
    assert all(path.parent == expected / "maps" and path.is_file() for path in result.maps.values())
    assert not (tmp_path / "maps-out").exists()


def test_flat_voxel_entropy_has_mlx_zero_sign() -> None:
    image = np.ones((2, 2))
    result = r.extract(
        image,
        np.ones_like(image, dtype=bool),
        spacing=(1, 1),
        mode="voxel",
        settings=r.Settings(features=("firstorder_Entropy",), bin_count=1),
    )
    assert np.signbit(result.features["original_firstorder_Entropy"]).all()


def test_roi_count_buffer_budget_raises_memory_error() -> None:
    image = np.array([[0, 1], [2, 3]])
    settings = r.Settings(features=("firstorder_Entropy",), bin_count=500000, scratch_mib=1)
    with pytest.raises(MemoryError) as exc:
        r.extract(image, np.ones_like(image), settings=settings, spacing=(1, 1))
    assert (
        str(exc.value) == "firstorder ROI count buffer requires 2000000 bytes, "
        "above the 1048576-byte scratch budget"
    )


@pytest.mark.parametrize("params", [[], [("setting", {"label": 4})], ""])
def test_parameter_parser_rejects_nonmapping_inputs(params: object) -> None:
    from rustradiomics.config.parameters import parse_params

    with pytest.raises(InvalidInputError, match="'params' must be a mapping"):
        parse_params(cast("Mapping[str, object]", params), ndim=3)
    with pytest.raises(InvalidInputError, match="parameter files require a 2D or 3D image"):
        parse_params(cast("Mapping[str, object]", params), ndim=-1)


def test_parameter_parser_accepts_readonly_mappings_and_checks_keys_first() -> None:
    from types import MappingProxyType

    from rustradiomics.config.parameters import parse_params

    params = MappingProxyType({"setting": MappingProxyType({"label": 4})})
    assert parse_params(params, ndim=2).settings.label == 4
    with pytest.raises(InvalidInputError, match="'params' keys must be strings"):
        parse_params(cast("Mapping[str, object]", {1: object()}), ndim=2)


def test_parameter_mapping_roundtrip() -> None:
    from rustradiomics.config.parameters import parse_params

    parsed = parse_params(
        {
            "imageType": {"Original": {}, "LoG": {"sigma": [2]}},
            "setting": {
                "resampledPixelSpacing": [2, 3, 4],
                "interpolator": "sitkLinear",
                "distances": [1, 2],
            },
        },
        ndim=3,
    )
    assert parsed.transforms.image_types == (r.ImageType.ORIGINAL, r.ImageType.LOG)
    assert parsed.transforms.log == LoGSettings(sigma=(2,))
    assert parsed.settings.distances == (1, 2)
    assert parsed.image_settings == r.ImageSettings(
        resampled_spacing=(4, 3, 2), interpolator=r.Interpolator.LINEAR
    )


def test_unsupported_core_stage_keeps_its_name(tmp_path: Path) -> None:
    path = tmp_path / "volume.nii"
    path.write_bytes((540).to_bytes(4, "little") + bytes(536))
    with pytest.raises(NotImplementedError, match="^NIfTI-2 reading$"):
        r.extract_files(path, path)


def test_detached_nrrd_read_failure_raises_oserror(tmp_path: Path) -> None:
    path = tmp_path / "missing-payload.nhdr"
    path.write_text(
        "NRRD0005\ntype: uchar\ndimension: 2\nsizes: 2 2\nencoding: raw\ndata file: absent.raw\n\n"
    )
    with pytest.raises(OSError, match="absent.raw"):
        r.extract_files(path, path)


def test_spatial_numeric_protocol_and_invalid_settings_dimension() -> None:
    options = r.IbsiInterpolation(
        cast("tuple[float, ...]", (np.float32(1.5), 0)),
        mask_threshold=cast("float", np.float32(0.5)),
    )
    assert options.spacing == (1.5, 0)
    with pytest.raises(
        InvalidInputError, match="slice_axis must be an integer indexing an image axis"
    ):
        r.Settings().validate(-1)
