"""Regenerate the W7 voxel fixtures: mlx-radiomics v1.1.0 voxel maps for settings the
parity goldens do not cover (explicit aggregation, closed-radius support with NGTDM caps,
asymmetric unmasked GLCM with several distances, 2-D radius 2, resegmented shape mask).

Usage, on an Apple silicon Mac with the v1.1.0 checkout on PYTHONPATH:

    PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/path/to/mlx-radiomics-v1.1.0/src MLX_ENABLE_TF32=0 \
        python validation/w7_voxel.py

Each case stores the arrays that mlx prepared (image, ROI, shape ROI, sampling mask, global
bins) so the Rust test needs no preprocessing, the settings that matter for voxel mode,
and one flat C-order map per voxel feature. Nonfinite values use "NaN", "Infinity" and
"-Infinity".
"""

import json
import math
from dataclasses import replace
from pathlib import Path

import numpy as np

from mlxradiomics import Settings, extract, list_features
from mlxradiomics.imaging.preprocessing import prepare

OUT = Path(__file__).resolve().parents[1] / "crates/radiomics/tests/data/w7/voxel_settings.mlx.json"
KEYS = (
    "bin_width", "bin_count", "distances", "force2d_axis", "masked_kernel", "kernel_radius",
    "symmetrical_glcm", "voxel_array_shift", "gldm_alpha", "glcm_aggregation",
    "glrlm_aggregation", "glszm_aggregation", "gldzm_aggregation", "gldm_aggregation",
    "ngtdm_aggregation", "neighborhood_support", "neighborhood_radius",
    "ngtdm_singular_cases", "slice_axis", "resegment_shape", "resegment_range",
    "ivh_bin_width", "ivh_bin_count", "kurtosis", "zero_variance_moments",
)


def number(value: float) -> float | str:
    if math.isfinite(value):
        return value
    return "NaN" if math.isnan(value) else ("Infinity" if value > 0 else "-Infinity")


def case(name: str, shape: tuple[int, ...], seed: int, **fields: object) -> dict[str, object]:
    rng = np.random.default_rng(seed)
    image = np.round(rng.normal(0, 40, size=shape) * 4) / 4
    mask = (rng.random(shape) < 0.6).astype(np.uint8)
    mask.flat[:3] = 1
    if "ivh_bin_count" not in fields:
        fields["ivh_bin_width"] = 20.0  # continuous intensities need an IVH lattice
    settings = replace(Settings(), **fields)
    settings = replace(settings, features=tuple(list_features(mode="voxel", ndim=len(shape))))
    data = prepare(image, mask, settings, voxel=True)
    assert data.bins is not None
    result = extract(image, mask, settings=settings, spacing=(1.0,) * len(shape), mode="voxel")
    maps = {
        key.removeprefix("original_"): [number(float(v)) for v in np.asarray(m).ravel()]
        for key, m in result.features.items()
    }
    return {
        "name": name,
        "shape": list(shape),
        "image": image.ravel().tolist(),
        "roi": data.roi.astype(int).ravel().tolist(),
        "shape_roi": data.shape_roi.astype(int).ravel().tolist(),
        "sampling_mask": data.sampling_mask.astype(int).ravel().tolist(),
        "bins": data.bins.ravel().tolist(),
        "gray_levels": data.gray_levels,
        "settings": {k: getattr(settings, k) for k in KEYS},
        "features": list(settings.features),
        "maps": maps,
    }


def main() -> None:
    cases = [
        case("explicit-merge", (4, 5, 6), 701, bin_width=20.0, glcm_aggregation="2d_slice_merge",
             glrlm_aggregation="2.5d_merge", glszm_aggregation="2.5d_merge",
             gldzm_aggregation="2.5d_merge", gldm_aggregation="2.5d_merge",
             ngtdm_aggregation="2.5d_merge"),
        case("closed-radius-caps", (4, 5, 6), 702, bin_width=20.0, neighborhood_support="closed-radius",
             neighborhood_radius=1, ngtdm_singular_cases="ibsi-caps", gldm_alpha=1),
        case("asymmetric-unmasked-distances", (4, 5, 6), 703, bin_count=10, symmetrical_glcm=False,
             masked_kernel=False, distances=(1, 2)),
        case("plane-radius2-average", (8, 9), 704, bin_count=8, kernel_radius=2,
             glcm_aggregation="2d_average", glrlm_aggregation="2d_average",
             glszm_aggregation="2d_average", gldzm_aggregation="2d_average",
             gldm_aggregation="2d_average", ngtdm_aggregation="2d_average"),
        case("resegmented-shape-mask", (4, 5, 6), 705, bin_width=20.0, resegment_shape=False,
             resegment_range=[-30.0, 30.0], ivh_bin_count=6),
    ]
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(cases, allow_nan=False, separators=(",", ":")) + "\n")
    print(f"wrote {OUT} ({OUT.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
