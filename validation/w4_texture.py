"""Regenerate W4 static fixtures. Run with v1.1.0 on PYTHONPATH and MLX_ENABLE_TF32=0.

Usage: python validation/w4_texture.py /path/to/mlx-radiomics-v1.1.0 /path/to/data/ibsi
The JSON has one flat object per extraction. Nonfinite results use named strings.
"""

import argparse
import csv
import json
import math
import sys
from collections import Counter, defaultdict
from dataclasses import replace
from pathlib import Path

import mlxradiomics
import nibabel as nib
import numpy as np

from mlxradiomics import Settings, extract
from mlxradiomics.config.settings import FAMILIES
from mlxradiomics.imaging.preprocessing import prepare

OUT = Path(__file__).resolve().parents[1] / "crates/radiomics/tests/data/w4"
ORACLE_CROSSCHECK_RTOL = 1e-12
ORACLE_CROSSCHECK_ATOL = 1e-13
FAMILY = ("glcm", "glrlm", "glszm", "gldzm", "gldm", "ngtdm")
ALL = tuple(f"{f}_{n}" for f in FAMILY for n in FAMILIES[f])
NATIVE = tuple(n for n in ALL if not n.startswith("gldzm_"))
MODES = (
    ("2d_average", "2d_average"),
    ("2d_slice_merge", "2.5d_merge"),
    ("2.5d_direction_merge", "2d_average"),
    ("2.5d_merge", "2.5d_merge"),
    ("3d_average", "3d"),
    ("3d_merge", "3d"),
)


def write_cases(file, cases):
    # One property per line, including flat arrays, for dependency-free Rust tests.
    objects = []
    for case in cases:
        fields = []
        for key, value in case.items():
            if isinstance(value, float) and not math.isfinite(value):
                value = str(value)
            fields.append("    " + json.dumps(key) + ": " + json.dumps(value, allow_nan=False))
        objects.append("  {\n" + ",\n".join(fields) + "\n  }")
    (OUT / file).write_text("[\n" + ",\n".join(objects) + "\n]\n")


def domain(label, data, profile, settings):
    return dict(
        label=label,
        profile=profile,
        shape=list(data.bins.shape),
        bins=data.bins.ravel().tolist(),
        morphology=data.shape_roi.astype(int).ravel().tolist(),
        ng=data.gray_levels,
        distances=list(settings.distances),
        alpha=settings.gldm_alpha,
        radius=settings.neighborhood_radius,
        symmetric=settings.symmetrical_glcm,
        force=settings.force2d_axis,
        axis=settings.slice_axis,
        directional=settings.glcm_aggregation,
        matrix=settings.gldm_aggregation,
    )


def mode_settings(settings, directional, matrix):
    return replace(
        settings,
        glcm_aggregation=directional,
        glrlm_aggregation=directional,
        glszm_aggregation=matrix,
        gldzm_aggregation=matrix,
        gldm_aggregation=matrix,
        ngtdm_aggregation=matrix,
    )


def native_oracle(data, current, profile):
    from radiomics_reference import catalogue, mathematics
    from test_aggregation import _zone_counts

    b, v = data.bins, data.sampling_mask
    expected = {}
    for f, vals in (
        ("glcm", catalogue.glcm(b, v, current, data.gray_levels)),
        ("glrlm", mathematics.glrlm_features(b, v, current.force2d_axis)),
        (
            "ngtdm",
            mathematics.ngtdm_features(
                b, v, current.distances, current.force2d_axis, ibsi_caps=profile == "ibsi"
            ),
        ),
    ):
        expected.update({f"{f}_{k}": x for k, x in vals.items()})
    expected.update(catalogue.dependence_zones(b, v, current))
    mcc = mathematics.glcm_features(b, v, current)
    for n, x in mcc.items():
        np.testing.assert_allclose(
            x, expected[f"glcm_{n}"], rtol=ORACLE_CROSSCHECK_RTOL, atol=ORACLE_CROSSCHECK_ATOL
        )
    if current.force2d_axis is None and "gldzm_ZonePercentage" in current.features:
        counts = _zone_counts(b, v, data.shape_roi, True)
        vals = catalogue.size_features(counts, "gldzm", voxel_count=int(v.sum()))
        expected.update({f"gldzm_{k}": x for k, x in vals.items()})
    return expected


def aggregated_oracle(data, current):
    from radiomics_reference import catalogue, mathematics
    from test_aggregation import _dependence_counts, _reference_directional, _zone_counts

    b, v = data.bins, data.sampling_mask
    expected = {}
    # Independent unit-direction oracle includes unequal and empty slices.
    control = replace(
        current, distances=(1,), symmetrical_glcm=True, neighborhood_radius=1, gldm_alpha=0
    )
    # The enumeration helper takes a 3-D stack for every planar mode.
    stack, valid, axis = (b[None], v[None], 0) if b.ndim == 2 else (b, v, current.slice_axis)
    for f in ("glcm", "glrlm"):
        vals = _reference_directional(stack, valid, f, current.glcm_aggregation, axis)
        expected.update({f"{f}_{k}": x for k, x in vals.items()})
    mode = current.gldm_aggregation
    if mode == "3d" or b.ndim == 2:
        domains = [(b, v, data.shape_roi)]
    else:
        domains = [
            tuple(np.take(a, i, axis=current.slice_axis) for a in (b, v, data.shape_roi))
            for i in range(b.shape[current.slice_axis])
            if np.take(v, i, axis=current.slice_axis).any()
        ]
    for f in ("glszm", "gldzm", "gldm"):
        matrices = [
            (
                _dependence_counts(plane, mask)
                if f == "gldm"
                else _zone_counts(plane, mask, morph, f == "gldzm"),
                int(mask.sum()),
            )
            for plane, mask, morph in domains
        ]
        if mode == "2.5d_merge":
            matrices = [(sum((c for c, _ in matrices), Counter()), sum(n for _, n in matrices))]
        values = [catalogue.size_features(c, f, voxel_count=n) for c, n in matrices]
        expected.update(
            {f"{f}_{k}": math.fsum(x[k] for x in values) / len(values) for k in values[0]}
        )
    if mode == "2.5d_merge":
        # Empty separators prevent cross-plane neighbors at unit radius.
        separated = np.zeros((2 * len(domains) - 1, *domains[0][0].shape), np.int32)
        separated[::2] = [plane for plane, _, _ in domains]
        domains = [(separated, separated > 0, None)]
    caps = current.ngtdm_singular_cases == "ibsi-caps"
    tones = [mathematics.ngtdm_features(plane, mask, ibsi_caps=caps) for plane, mask, _ in domains]
    expected.update({f"ngtdm_{k}": math.fsum(x[k] for x in tones) / len(tones) for k in tones[0]})
    return control, expected


def synthetic():
    from conftest import settings_for
    from radiomics_reference import catalogue

    gpu, oracles = [], []
    rng = np.random.default_rng(401)
    for shape in ((5, 6), (4, 5, 6)):
        image = rng.integers(0, 8, size=shape, dtype=np.int32) * 21 + 7
        mask = (np.indices(shape).sum(axis=0) % 5 != 1).astype(np.uint8)
        if len(shape) == 3:
            mask[:, 2, :] = 0
            mask[1:, 0, :] = 0
        for profile, names in (("ibsi", ALL), ("pyradiomics", ALL), ("pyradiomics", NATIVE)):
            settings = settings_for(profile, features=names)
            variants = [("default", settings)]
            if names == NATIVE:
                variants.append(
                    (
                        "multiple_distances",
                        replace(settings, distances=(3, 1), symmetrical_glcm=False, gldm_alpha=2),
                    )
                )
                variants.append(
                    (
                        "shells",
                        replace(
                            settings,
                            distances=(2,),
                            symmetrical_glcm=False,
                            gldm_alpha=1,
                            force2d_axis=len(shape) - 1,
                        ),
                    )
                )
            if names == ALL:
                variants.extend(
                    (
                        d,
                        mode_settings(
                            replace(
                                settings,
                                slice_axis=1,
                                distances=(2,),
                                neighborhood_radius=2,
                                symmetrical_glcm=False,
                                gldm_alpha=1,
                                resegment_range=(28.0,),
                            ),
                            d,
                            m,
                        ),
                    )
                    for d, m in MODES
                    if len(shape) == 3 or not d.startswith("3d")
                )
            if names == ALL and len(shape) == 3:
                variants.append(("force", replace(settings, slice_axis=0, force2d_axis=2)))
            for label, current in variants:
                label = f"{len(shape)}d_{profile}_{len(names)}_{label}"
                data = prepare(image, mask, current, voxel=False)
                case = domain(label, data, profile, current)
                spacing = (2.5, 1.5, 0.75)[-len(shape) :]
                result = extract(image, mask, settings=current, spacing=spacing)
                gpu.append(
                    {
                        **case,
                        **{
                            n.removeprefix("original_"): float(x)
                            for n, x in result.features.items()
                        },
                    }
                )
                if current.glcm_aggregation == "native":
                    expected = native_oracle(data, current, profile)
                else:
                    control, expected = aggregated_oracle(data, current)
                    case = domain(label + "_unit", data, profile, control)
                oracles.append({**case, **expected})
    # Sparse GLCM test fixtures exercise global Ng normalization with missing levels.
    for shape in ((4, 5), (3, 4, 5)):
        levels = np.array([1, 16001, 16002, 25001], np.int32)
        image = levels[np.random.default_rng(417).integers(0, 4, shape)]
        mask = np.indices(shape).sum(axis=0) % 4 != 0
        for symmetric in (False, True):
            settings = settings_for(
                "pyradiomics",
                features=tuple(n for n in ALL if n.startswith("glcm_")),
                bin_width=1,
                bin_anchor=1,
                symmetrical_glcm=symmetric,
            )
            data = prepare(image, mask, settings, voxel=False)
            case = domain(f"sparse_{len(shape)}d_{symmetric}", data, "pyradiomics", settings)
            values = catalogue.glcm(data.bins, mask, settings, data.gray_levels)
            oracles.append({**case, **{f"glcm_{n}": x for n, x in values.items()}})
    write_cases("mlx.json", gpu)
    write_cases("oracles.json", oracles)


def phantom(checkout, ibsi):
    families = dict(cm="glcm", rlm="glrlm", szm="glszm", dzm="gldzm", ngt="ngtdm", ngl="gldm")
    modes = {
        "2D, averaged": "2d_average",
        "2D, slice-merged": "2d_slice_merge",
        "2.5D, direction-merged": "2.5d_direction_merge",
        "2.5D, merged": "2.5d_merge",
        "3D, averaged": "3d_average",
        "3D, merged": "3d_merge",
        "2D": "2d_average",
        "2.5D": "2.5d_merge",
        "3D": "3d",
    }
    root = ibsi / "digital_phantom/nifti"

    def read(file):
        return np.asarray(nib.load(root / file).dataobj, dtype=np.int32).T

    image, mask = read("image/phantom.nii.gz"), read("mask/mask.nii.gz")
    settings = Settings(features=ALL, bin_width=1, bin_anchor=1)
    data = prepare(image, mask, settings, voxel=False)
    definitions = json.loads((checkout / "docs/ibsi-features.json").read_text())["definitions"]
    names = {
        (n.split("_")[0], d["title"].lower()): n for d in definitions for n in d["public_names"]
    }
    grouped = defaultdict(dict)
    with (ibsi / "ibsi_1_reference/digital phantom.csv").open() as handle:
        for row in csv.DictReader(handle):
            family = families.get(row["tag"].split("_")[0])
            if family is None:
                continue
            mode = modes[row["family"].split("(")[1][:-1]]
            title = row["feature"].lower()
            if title in ("low grey level emphasis", "high grey level emphasis"):
                title = title.replace("level emphasis", "level zone emphasis")
            assert float(row["tolerance"]) == 0
            name = names[family, title]
            assert name not in grouped[mode]
            grouped[mode][name] = float(row["reference value"])
    assert sum(map(len, grouped.values())) == 408
    cases = []
    for mode, rows in grouped.items():
        directional = mode if mode != "3d" else "native"
        matrix = mode if mode in ("2d_average", "2.5d_merge", "3d") else "native"
        cases.append(
            {**domain(mode, data, "ibsi", mode_settings(settings, directional, matrix)), **rows}
        )
    write_cases("ibsi.json", cases)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("checkout", type=Path)
    parser.add_argument("ibsi", type=Path)
    args = parser.parse_args()
    if Path(mlxradiomics.__file__).resolve().parents[2] != args.checkout.resolve():
        parser.error("set PYTHONPATH to the v1.1.0 checkout's src directory")
    sys.path.insert(0, str(args.checkout / "tests"))
    OUT.mkdir(parents=True, exist_ok=True)
    phantom(args.checkout, args.ibsi)
    synthetic()
