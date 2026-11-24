from collections.abc import Callable, Mapping
from datetime import date, datetime, timedelta, timezone
from pathlib import Path
from types import MappingProxyType

import numpy as np
import pytest
from rustradiomics import Settings, extract
from rustradiomics.config.parameters import (
    load_params,
    parse_params,
    settings_from_params,
)


def test_load_parse_and_extract(tmp_path: Path) -> None:
    path = tmp_path / "params.yaml"
    path.write_text(
        "featureClass: {firstorder: [Mean]}\n"
        "setting: {<<: [{binCount: 4, normalize: no}, {binCount: 16}], label: 0x02}\n",
        encoding="utf-8",
    )
    params = load_params(path)
    assert list(params) == ["featureClass", "setting"]
    assert params == {
        "featureClass": {"firstorder": ["Mean"]},
        "setting": {"binCount": 4, "normalize": False, "label": 2},
    }
    parsed = parse_params(params, ndim=2)
    assert settings_from_params(params) == parsed.settings
    result = extract(
        np.array([[2, 8], [10, 16]]),
        np.array([[2, 1], [2, 1]]),
        settings=parsed.settings,
        spacing=(1.0, 1.0),
    )
    assert result.features == {"original_firstorder_Mean": 6.0}


def test_loader_preserves_plain_scalar_types(tmp_path: Path) -> None:
    path = tmp_path / "types.yaml"
    path.write_text(
        "names: &names ['on', null, 012, 1:02]\ncopy: *names\n"
        "date: 2025-06-04\ninstant: 2025-6-4 3:02:01.1234567 -5:30\n"
        "blob: !!binary YWJj\n",
        encoding="utf-8",
    )
    result = load_params(str(path))
    assert result["names"] == result["copy"] == ["on", None, 10, 62]
    assert result["date"] == date(2025, 6, 4)
    assert result["instant"] == datetime(
        2025, 6, 4, 3, 2, 1, 123456, tzinfo=timezone(-timedelta(hours=5, minutes=30))
    )
    assert result["blob"] == b"abc"


@pytest.mark.parametrize(
    ("contents", "message"),
    [
        (b"setting: [", "line 1"),
        (b"[]", "mapping"),
        (b"1: test", "keys must be strings"),
        (b"setting: {[a, b]: 2}", "unhashable"),
        (b"bad: \xff", "UTF-8"),
        (None, "missing params file"),
    ],
)
def test_invalid_parameter_file(tmp_path: Path, contents: bytes | None, message: str) -> None:
    path = tmp_path / "params.yaml"
    if contents is not None:
        path.write_bytes(contents)
    with pytest.raises(ValueError, match=message):
        load_params(path)


def test_overlay_preserves_base_ignores_unrelated_sections_and_defers_field_validation() -> None:
    base = Settings(force2d_axis=1, scratch_mib=23, ivh_bin_count=31)
    assert settings_from_params({"imageType": object()}, base=base) == base
    params = MappingProxyType(
        {
            "setting": MappingProxyType(
                {
                    "force2D": False,
                    "force2Ddimension": object(),
                    "distances": [1, 2],
                    "ivhIntensityRange": [-2, 4],
                }
            ),
            "featureClass": {"firstorder": ["Mean", "Range"]},
        }
    )
    result = settings_from_params(params, base=base)
    assert result == Settings(
        scratch_mib=23,
        ivh_bin_count=31,
        distances=(1, 2),
        ivh_intensity_range=(-2.0, 4.0),
        features=("firstorder_Mean", "firstorder_Range"),
    )
    assert base.force2d_axis == 1
    cases: list[tuple[str, str, object]] = [
        ("binCount", "bin_count", -1),
        ("kurtosis", "kurtosis", "other"),
    ]
    for key, field, value in cases:
        unvalidated = settings_from_params({"setting": {key: value}}, base=base)
        assert getattr(unvalidated, field) == value
        with pytest.raises(ValueError, match=field):
            unvalidated.validate(3)


@pytest.mark.parametrize(
    ("params", "message"),
    [
        ({"setting": {"binWidth": True}}, "must be a number"),
        ({"setting": {"symmetricalGLCM": 1}}, "must be a boolean"),
        ({"setting": {"ivhIntensityRange": [1]}}, "two bounds"),
        ({"setting": {"distances": [True]}}, "list of integers"),
        ({"setting": {"resegmentMode": "unknown"}}, "absolute, relative, sigma"),
        ({"setting": {"resampledPixelSpacing": [1, 1, 1]}}, "unsupported setting keys"),
        ({"featureClass": {"firstorder": []}}, "select at least one"),
        ({"featureClass": {"firstorder": ["Mean", 1]}}, "entries must be strings"),
    ],
)
def test_overlay_checks_parameter_types(params: Mapping[str, object], message: str) -> None:
    with pytest.raises(ValueError, match=message):
        settings_from_params(params)


def test_feature_selection_requires_a_list_before_serde_conversion() -> None:
    params = {"featureClass": {"firstorder": ("Mean",)}}
    parsers: tuple[Callable[[Mapping[str, object]], object], ...] = (
        settings_from_params,
        lambda p: parse_params(p, ndim=2),
    )
    for parse in parsers:
        with pytest.raises(ValueError, match="list feature names explicitly"):
            parse(params)
