"""Python adapters for the core parameter-file contract."""

import re
from collections.abc import Callable, Mapping, Sequence
from dataclasses import dataclass, fields, replace
from datetime import date, datetime, timedelta, timezone
from enum import StrEnum
from pathlib import Path
from typing import cast

from rustrad import _native
from rustrad.config.settings import Settings
from rustrad.config.spatial import IbsiInterpolation, ImageSettings, Interpolator
from rustrad.config.transforms import ImageType, TransformSettings
from rustrad.errors import InvalidInputError


@dataclass(frozen=True)
class ExtractionParameters:
    settings: Settings
    image_settings: ImageSettings
    transforms: TransformSettings


def _build[T](factory: Callable[..., T], values: Mapping[str, object]) -> T:
    # Construct a frozen dataclass from a validated mapping of field values.
    return factory(**values)


def _transform_settings(flat: Mapping[str, object]) -> TransformSettings:
    # The core serializes transforms as the flat schema-2 report payload.
    options: dict[str, object] = {
        "image_types": tuple(map(ImageType, cast("Sequence[str]", flat["image_types"]))),
        "scratch_mib": flat["scratch_mib"],
    }
    special_keys = {
        ("wavelet", "name"): "wavelet",
        ("log", "sigma"): "sigma",
        ("separable_wavelet", "name"): "separable_wavelet",
    }
    defaults = TransformSettings()
    for field in fields(defaults):
        if field.name in options:
            continue
        block = getattr(defaults, field.name)
        values: dict[str, object] = {}
        for child in fields(block):
            key = special_keys.get((field.name, child.name), f"{field.name}_{child.name}")
            value = flat[key]
            default = getattr(block, child.name)
            values[child.name] = type(default)(value) if isinstance(default, StrEnum) else value
        options[field.name] = type(block)(**values)
    return _build(TransformSettings, options)


def load_params(path: str | Path) -> dict[str, object]:
    """Load an ordered parameter mapping from a UTF-8 YAML file."""
    return _native.load_params(path)


def _timestamp(raw: str) -> date | datetime:
    # YAML allows one-digit months, days and hours, and whitespace before offsets.
    match = re.fullmatch(
        r"(\d{4})-(\d{1,2})-(\d{1,2})"
        r"(?:(?:[Tt]|[ \t]+)(\d{1,2}):(\d{2}):(\d{2})(?:\.(\d*))?"
        r"(?:[ \t]*(?:(Z)|([+-])(\d{1,2})(?::(\d{2}))?))?)?",
        raw,
    )
    if match is None:
        raise InvalidInputError(f"invalid YAML timestamp: {raw}")
    year, month, day = (int(match[i]) for i in (1, 2, 3))
    if match[4] is None:
        return date(year, month, day)
    zone = None
    if match[8] or match[9]:
        offset = timedelta(hours=int(match[10] or 0), minutes=int(match[11] or 0))
        zone = timezone(-offset if match[9] == "-" else offset)
    micros = int((match[7] or "")[:6].ljust(6, "0"))
    return datetime(year, month, day, int(match[4]), int(match[5]), int(match[6]), micros, zone)


# The Rust extension looks this converter up by name when it meets a YAML timestamp.
_NATIVE_HOOKS = (_timestamp,)


def _mapping(value: object, name: str) -> Mapping[str, object]:
    if not isinstance(value, Mapping):
        raise InvalidInputError(f"'{name}' must be a mapping")
    mapping = cast("Mapping[object, object]", value)
    if any(not isinstance(key, str) for key in mapping):
        raise InvalidInputError(f"'{name}' keys must be strings")
    return cast("Mapping[str, object]", mapping)


def _check_feature_lists(selection: object) -> None:
    for family, names in _mapping(selection, "featureClass").items():
        if not isinstance(names, list):
            raise InvalidInputError(
                f"featureClass '{family}' must list feature names explicitly for this runner"
            )


def _option_field(key: str) -> str:
    if key == "symmetricalGLCM":
        return "symmetrical_glcm"
    if key == "force2D":
        return "force2d_axis"
    return re.sub(r"(?=[A-Z])", "_", key).lower()


# A Python Settings object can hold unvalidated enum strings and arbitrary-size integers.
# Keep those values until validate(ndim) converts the dataclass to typed Rust fields.
_TEXT_OPTIONS = (
    "covarianceDivisor",
    "ellipsoidArea",
    "kurtosis",
    "zeroVarianceMoments",
    "ngtdmSingularCases",
    "neighborhoodSupport",
    "binOrigin",
    "constantRoiBin",
    "maskExtrapolation",
    "splineBoundary",
    "glcmAggregation",
    "glrlmAggregation",
    "glszmAggregation",
    "gldzmAggregation",
    "ngtdmAggregation",
    "gldmAggregation",
)
_INTEGER_OPTIONS = (
    "binCount",
    "label",
    "gldm_a",
    "kernelRadius",
    "sliceAxis",
    "neighborhoodRadius",
    "ivhBinCount",
)


def _integer(value: object, key: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise InvalidInputError(f"setting '{key}' must be an integer")
    return value


def settings_from_params(params: Mapping[str, object], *, base: Settings | None = None) -> Settings:
    """Overlay feature settings on a frozen base. Call validate(ndim) to check field ranges."""
    params = _mapping(params, "params")
    options = dict(_mapping(params.get("setting", {}), "setting"))
    source = base if base is not None else Settings()
    updates: dict[str, object] = {}
    for key in _TEXT_OPTIONS:
        field = _option_field(key)
        value = options.pop(key, getattr(source, field))
        if not isinstance(value, str):
            raise InvalidInputError(f"setting '{key}' must be a string")
        updates[field] = value
    for key in _INTEGER_OPTIONS:
        if key in options:
            field = "gldm_alpha" if key == "gldm_a" else _option_field(key)
            updates[field] = _integer(options.pop(key), key)
    if "distances" in options:
        raw = options.pop("distances")
        items = cast("Sequence[object]", raw) if isinstance(raw, (tuple, list)) else ()
        if not items or any(isinstance(item, bool) or not isinstance(item, int) for item in items):
            raise InvalidInputError("setting 'distances' must be a list of integers")
        updates["distances"] = tuple(items)
    axis = options.pop("force2Ddimension", 0)
    if options.get("force2D") is True:
        updates["force2d_axis"] = _integer(axis, "force2Ddimension")
        del options["force2D"]
    native_params: dict[str, object] = {"setting": options}
    if (selection := params.get("featureClass")) is not None:
        _check_feature_lists(selection)
        native_params["featureClass"] = selection
    native = _native.settings_from_params(native_params)
    for key in options:
        field = _option_field(key)
        updates[field] = native[field]
    if selection is not None:
        updates["features"] = native["features"]
    return replace(source, **updates)


def parse_params(params: Mapping[str, object], *, ndim: int) -> ExtractionParameters:
    """Parse a parameter mapping on the CPU into frozen, typed options."""
    if ndim not in (2, 3):
        raise InvalidInputError("parameter files require a 2D or 3D image")
    params = _mapping(params, "params")
    if (selection := params.get("featureClass")) is not None:
        # Serde represents Python tuples and lists as the same sequence.
        _check_feature_lists(selection)
    settings, spatial, transforms = _native.parse_params(params, ndim)
    spatial["interpolator"] = Interpolator(spatial["interpolator"])
    if (ibsi := cast("dict[str, object] | None", spatial["ibsi_interpolation"])) is not None:
        ibsi["image_interpolator"] = Interpolator(ibsi["image_interpolator"])
        spatial["ibsi_interpolation"] = _build(IbsiInterpolation, ibsi)
    return ExtractionParameters(
        _build(Settings, settings),
        _build(ImageSettings, spatial),
        _transform_settings(transforms),
    )
