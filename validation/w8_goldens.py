"""Regenerate the W8 end-to-end goldens: mlx-radiomics v1.1.0 `extract_files` on files.

Usage, on an Apple silicon Mac with the v1.1.0 checkout on PYTHONPATH:

    PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/path/to/mlx-radiomics-v1.1.0/src MLX_ENABLE_TF32=0 \
        MLXRAD_REPO=/path/to/mlx-radiomics python validation/w8_goldens.py

It writes small NIfTI and NRRD inputs and one golden per case under
`crates/radiomics/tests/data/w8/`. Each golden stores the PyRadiomics-style parameter
mapping that both implementations parse (`parse_params`), so the settings, spatial options
and derived images cannot drift apart, the expected feature values (ROI scalars, or flat
C-order voxel maps of float32 values) and the metadata that the Rust test compares.
Non-finite values use "NaN", "Infinity" and "-Infinity".

Inputs: `phantom_*.nii.gz` is the IBSI 1 digital phantom (Alex Zwanenburg, IBSI, CC BY 4.0,
https://github.com/theibsi/data_sets/tree/master/ibsi_1_digital_phantom) copied from
`$MLXRAD_REPO/data/ibsi/digital_phantom/nifti`; the other images are synthetic and seeded.
"""

import json
import math
import os
import shutil
from pathlib import Path

import numpy as np
import SimpleITK as sitk

from mlxradiomics import extract_files, list_features
from mlxradiomics.config.parameters import parse_params
from mlxradiomics.workflows import extraction

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "crates/radiomics/tests/data/w8"

PYRADIOMICS_CONVENTIONS = {
    "covarianceDivisor": "n",
    "ellipsoidArea": "exact",
    "kurtosis": "raw",
    "zeroVarianceMoments": "nan",
    "ngtdmSingularCases": "unadjusted",
    "neighborhoodSupport": "distances",
    "binOrigin": "zero-lattice",
    "constantRoiBin": "middle",
    "maskExtrapolation": "nearest",
    "splineBoundary": "mirror",
}
VOXEL_FEATURES = (
    "firstorder_Mean", "firstorder_Entropy", "firstorder_Skewness", "glcm_Contrast",
    "glcm_Idm", "glrlm_ShortRunEmphasis", "glszm_ZoneEntropy", "gldm_DependenceEntropy",
    "ngtdm_Coarseness", "intensityhistogram_Mean", "ivh_VolumeAtIntensityFraction10",
)


def number(value: float) -> float | str:
    if math.isfinite(value):
        return value
    return "NaN" if math.isnan(value) else ("Infinity" if value > 0 else "-Infinity")


def synthetic(shape: tuple[int, ...], seed: int, dtype: str) -> tuple[np.ndarray, np.ndarray]:
    rng = np.random.default_rng(seed)
    grids = np.meshgrid(*[np.linspace(-1, 1, n) for n in shape], indexing="ij")
    radius = sum(g**2 for g in grids)
    smooth = 90 * np.sin(2.5 * grids[0]) + 60 * np.cos(3.0 * grids[-1]) * grids[1]
    values = smooth + rng.normal(0, 25, size=shape) - 40 * radius
    mask = np.zeros(shape, dtype=np.uint8)
    mask[radius < 0.55] = 1
    mask[(radius >= 0.55) & (radius < 0.7)] = 2
    if dtype == "int16":
        return np.round(values).astype(np.int16), mask
    return values.astype(np.float32), mask


def write_pair(name: str, image: np.ndarray, mask: np.ndarray, spacing, origin, suffix: str):
    """Write an image/mask pair; `spacing` is in ITK (x, y, z) order."""
    for kind, array in (("image", image), ("mask", mask)):
        volume = sitk.GetImageFromArray(array)
        volume.SetSpacing(spacing)
        volume.SetOrigin(origin)
        sitk.WriteImage(volume, str(OUT / f"{name}_{kind}{suffix}"))


def features_for(ndim: int, mode: str) -> dict[str, list[str]]:
    names = list_features(ndim=ndim, mode=mode)
    if mode == "voxel":
        names = tuple(n for n in names if n in VOXEL_FEATURES)
    families: dict[str, list[str]] = {}
    for name in names:
        family, feature = name.split("_", 1)
        families.setdefault(family, []).append(feature)
    return families


def params_for(
    ndim: int, mode: str, convention: str, image_types: dict, label: int, **setting: object,
) -> dict:
    base = {"binCount": 32, "label": label, "ivhBinCount": 32}
    if convention == "pyradiomics":
        base.update(PYRADIOMICS_CONVENTIONS)
    base.update(setting)
    return {
        "setting": base,
        "imageType": image_types,
        "featureClass": features_for(ndim, mode),
    }


def cpu_lbp_m2_local_peak(image_path: Path, mask: np.ndarray, options) -> float:
    """Evaluate the synthetic integer fixture's degree-one band with MLX CPU arithmetic."""
    import mlx.core as mx
    from scipy.ndimage import map_coordinates

    from mlxradiomics.filters.lbp3d import _sphere_coefficients

    volume = sitk.ReadImage(str(image_path))
    image = sitk.GetArrayFromImage(volume)
    assert image.dtype == np.int16
    centers = np.argwhere(mask)
    vertices, harmonics, areas = _sphere_coefficients(options.subdivision, options.radius, 2)
    positions = (centers[:, None, :] + vertices).transpose(2, 0, 1)
    samples = map_coordinates(image, positions, order=3, mode="constant", cval=0)
    with mx.stream(mx.cpu):
        bits = mx.array(samples >= image[mask, None], dtype=mx.complex64)
        basis = mx.conj(mx.array(harmonics[1], dtype=mx.complex64))
        coefficients = bits @ (basis * mx.array(areas, dtype=mx.float32)[:, None])
        power = mx.real(coefficients) ** 2 + mx.imag(coefficients) ** 2
        band = np.asarray(mx.sqrt(mx.sum(power, axis=1)))

    # LocalIntensityPeak takes the greatest in-image sphere mean at any maximal ROI center.
    values = np.zeros(image.shape, dtype=np.float32)
    values[mask] = band
    grid = np.indices(image.shape).reshape(3, -1).T
    spacing = np.array(volume.GetSpacing()[::-1])
    radius_squared = (3000 / (4 * np.pi)) ** (2 / 3)
    means = []
    for center in centers[band == band.max()]:
        sphere = np.sum(((grid - center) * spacing) ** 2, axis=1) <= radius_squared
        means.append(values.ravel()[sphere].mean(dtype=np.float64))
    return float(np.float32(max(means)))


def run_case(name: str, image: str, mask: str, mode: str, params: dict) -> None:
    ndim = sitk.ReadImage(str(OUT / image)).GetDimension()
    parsed = parse_params(params, ndim=ndim)
    # Record the peak magnitude of every feature image as mlx filtered it (float32). The Rust
    # test scales the float32 rounding allowance of derived images with it.
    scales: dict[str, float] = {}
    real = extraction.feature_arrays

    def spy(image_array, data, *args):
        for key, array in real(image_array, data, *args):
            peak = data.image if array is None else array
            scales[key] = float(np.abs(np.asarray(peak, dtype=np.float64)).max())
            yield key, array

    extraction.feature_arrays = spy
    try:
        result = extract_files(
            OUT / image, OUT / mask, settings=parsed.settings,
            image_settings=parsed.image_settings, transforms=parsed.transforms, mode=mode,
        )
    finally:
        extraction.feature_arrays = real
    scales.setdefault("original", float(np.abs(sitk.GetArrayFromImage(
        sitk.ReadImage(str(OUT / image)))).max()))
    features = {}
    inside = (sitk.GetArrayFromImage(sitk.ReadImage(str(OUT / mask))) == parsed.settings.label)
    for key, value in result.features.items():
        if mode == "voxel":
            array = np.asarray(value)
            assert not np.any(array[~inside]), "maps are zero outside the ROI"
            # Shortest float32 spelling: the Rust test reads it back as the same float32.
            features[key] = [
                number(float(str(np.float32(v)))) for v in array[inside].astype(np.float32)
            ]
        else:
            features[key] = number(float(value))
    meta = result.metadata
    record = {
        "case": name,
        "image": image,
        "mask": mask,
        "mode": mode,
        "params": params,
        "gray_levels_by_image": meta["gray_levels_by_image"],
        "image_geometry": {k: list(v) for k, v in meta["image_geometry"].items()},
        "array_shape": list(meta["array_shape"]),
        "roi_voxels": meta["roi_voxels"],
        "image_scale": scales,
        "feature_validity": meta["feature_validity"],
        "voxel_values": "C-order values at the voxels of the selected label" if mode == "voxel" else None,
        "features": features,
    }
    if name in ("synth3d.roi.ibsi", "synth3d.roi.pyradiomics"):
        record["cpu_lbp_m2_local_peak"] = cpu_lbp_m2_local_peak(
            OUT / image, inside, parsed.transforms.lbp3d,
        )
    compact = json.JSONEncoder(separators=(",", ":"), allow_nan=False).encode
    lines = []
    for key, value in record.items():
        if key in ("features", "feature_validity", "image_scale"):
            body = ",\n".join(f"  {compact(k)}:{compact(v)}" for k, v in value.items())
            lines.append(f"{compact(key)}:{{\n{body}\n}}")
        else:
            lines.append(f"{compact(key)}:{compact(value)}")
    text = "{\n" + ",\n".join(lines) + "\n}\n"
    assert len(text) < 1_000_000, name
    (OUT / f"{name}.mlx.json").write_text(text)
    print(name, len(features), "features", len(text) // 1024, "KiB")


def export_report(name: str, image: str, mask: str, params: dict) -> None:
    """Store mlx's schema-2 `features.json` for an export, maps relative to the report."""
    import tempfile

    from mlxradiomics import export_files

    ndim = sitk.ReadImage(str(OUT / image)).GetDimension()
    parsed = parse_params(params, ndim=ndim)
    with tempfile.TemporaryDirectory() as scratch:
        saved = export_files(
            OUT / image, OUT / mask, Path(scratch) / "out", settings=parsed.settings,
            image_settings=parsed.image_settings, transforms=parsed.transforms,
        )
        report = json.loads(saved.report.read_text())
        report["maps"] = {k: f"maps/{k}.nii.gz" for k in report["maps"]}
        # Distinguish Rust from mlx by values only: keep every key of the report.
        (OUT / f"{name}.mlx.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
        print(name, len(report["maps"]), "maps")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    source = Path(os.environ["MLXRAD_REPO"]) / "data/ibsi/digital_phantom/nifti"
    shutil.copy(source / "image/phantom.nii.gz", OUT / "phantom_image.nii.gz")
    shutil.copy(source / "mask/mask.nii.gz", OUT / "phantom_mask.nii.gz")
    image3, mask3 = synthetic((12, 14, 16), 11, "int16")
    write_pair("synth3d", image3, mask3, (1.5, 1.5, 2.0), (3.0, -2.0, 5.0), ".nii.gz")
    image2, mask2 = synthetic((22, 26), 12, "float32")
    write_pair("synth2d", image2, mask2, (0.8, 1.2), (-4.0, 6.0), ".nrrd")

    types3 = {
        "Original": {}, "Wavelet": {}, "LoG": {"sigma": [1.5, 3.0]}, "Mean": {},
        "Laws": {}, "LBP3D": {},
    }
    types3_voxel = {
        "Original": {}, "Wavelet": {}, "LoG": {"sigma": [1.5]}, "Mean": {}, "Laws": {},
    }
    types2 = {
        "Original": {}, "Wavelet": {}, "Mean": {}, "Laws": {},
        "LBP2D": {}, "Square": {},
    }
    types_phantom = {"Original": {}, "Mean": {}, "Square": {}}
    for convention in ("ibsi", "pyradiomics"):
        run_case(
            f"synth3d.roi.{convention}", "synth3d_image.nii.gz", "synth3d_mask.nii.gz", "roi",
            params_for(3, "roi", convention, types3, 1),
        )
        run_case(
            f"synth3d.voxel.{convention}", "synth3d_image.nii.gz", "synth3d_mask.nii.gz", "voxel",
            params_for(3, "voxel", convention, types3_voxel, 1),
        )
        if convention == "ibsi":
            export_report(
                "synth3d.export_report", "synth3d_image.nii.gz", "synth3d_mask.nii.gz",
                params_for(3, "voxel", convention, types3_voxel, 1),
            )
        run_case(
            f"synth2d.roi.{convention}", "synth2d_image.nrrd", "synth2d_mask.nrrd", "roi",
            params_for(2, "roi", convention, types2, 1),
        )
        run_case(
            f"synth2d.voxel.{convention}", "synth2d_image.nrrd", "synth2d_mask.nrrd", "voxel",
            params_for(2, "voxel", convention, {k: v for k, v in types2.items() if k != "LBP2D"}, 2),
        )
        run_case(
            f"phantom.roi.{convention}", "phantom_image.nii.gz", "phantom_mask.nii.gz", "roi",
            params_for(3, "roi", convention, types_phantom, 1),
        )


if __name__ == "__main__":
    main()
