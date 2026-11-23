"""Generate the IBSI 1 digital-phantom morphology fixture for the Rust W5 test.

Writes ``crates/radiomics/tests/data/w5/ibsi_phantom.json``. Run it with the Python
environment that has ``mlxradiomics`` v1.1.0 importable (it runs on the Metal GPU in
float32) and ``SimpleITK``::

    PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=<mlx-radiomics>/src MLX_ENABLE_TF32=0 \
        ../mlx-radiomics/.venv/bin/python validation/w5_goldens.py \
        --phantom ../mlx-radiomics/data/ibsi \
        --out crates/radiomics/tests/data/w5

The fixture holds the IBSI 1 digital phantom (CC BY 4.0), its morphology reference rows and
the mlx-radiomics values of every geometric ``shape`` feature (one name per call) and of the
intensity morphology features under both convention sets (``ibsi`` = ``Settings()``
defaults, ``pyradiomics`` = ``Settings.pyradiomics_conventions()``).

mlx-radiomics is Apache-2.0 and the IBSI phantom is CC BY 4.0.
"""

import argparse
import csv
import json
from math import isnan
from pathlib import Path

import numpy as np

parser = argparse.ArgumentParser()
parser.add_argument("--phantom", required=True, type=Path, help="mlx-radiomics data/ibsi")
parser.add_argument("--out", required=True, type=Path)
args = parser.parse_args()

from mlxradiomics.config.settings import SHAPE  # noqa: E402
from mlxradiomics.features.morphology import INTENSITY_MORPHOLOGY, morphology_statistics  # noqa: E402
from mlxradiomics.features.shape import shape_statistics  # noqa: E402

GEOMETRIC_3D = [name for name in SHAPE if name not in INTENSITY_MORPHOLOGY]
CONVENTIONS = {
    "ibsi": {"covariance_divisor": "n-1", "ellipsoid_area": "ibsi-series"},
    "pyradiomics": {"covariance_divisor": "n", "ellipsoid_area": "exact"},
}


def encode(value: float) -> float | str:
    """JSON has no NaN or infinity; encode them as strings."""
    value = float(value)
    if isnan(value):
        return "nan"
    if value in (float("inf"), float("-inf")):
        return "inf" if value > 0 else "-inf"
    return value


def run_shape(mask: np.ndarray, spacing: tuple[float, ...], names: list[str],
              convention: dict[str, str]) -> dict:
    result: dict = {}
    for name in names:
        try:
            value = shape_statistics(mask, spacing, 64, (name,), **convention)
            (key, array), = value.items()
            result[name] = encode(np.asarray(array)[0])
            del key
        except (ValueError, RuntimeError) as error:
            result[name] = {"error": type(error).__name__, "message": str(error)}
    return result


def main() -> None:
    args.out.mkdir(parents=True, exist_ok=True)

    # IBSI 1 digital phantom (CC BY 4.0) and its morphology reference rows.
    import SimpleITK as sitk

    base = args.phantom / "digital_phantom" / "nifti"
    image_itk = sitk.ReadImage(str(base / "image" / "phantom.nii.gz"))
    mask_itk = sitk.ReadImage(str(base / "mask" / "mask.nii.gz"))
    image = sitk.GetArrayFromImage(image_itk).astype(np.float64)
    mask = sitk.GetArrayFromImage(mask_itk) > 0
    spacing = tuple(reversed(image_itk.GetSpacing()))
    rows = []
    with (args.phantom / "ibsi_1_reference" / "digital phantom.csv").open() as handle:
        for row in csv.DictReader(handle):
            if row["family"] == "Morphology" and row["reference value"]:
                rows.append({"tag": row["tag"], "feature": row["feature"],
                             "reference": float(row["reference value"]),
                             "tolerance": float(row["tolerance"])})
    names = [n for n in GEOMETRIC_3D]
    mlx = {label: run_shape(mask, spacing, names, convention)
           for label, convention in CONVENTIONS.items()}
    morph = {}
    for name in INTENSITY_MORPHOLOGY:
        value = morphology_statistics(image, mask, mask, spacing, 64, (name,))
        morph[name] = encode(np.asarray(next(iter(value.values())))[0])
    phantom = {
        "shape": list(mask.shape), "spacing": list(spacing),
        "image": image.ravel().tolist(), "mask": mask.astype(int).ravel().tolist(),
        "rows": rows, "mlx": mlx, "mlx_morphology": morph,
    }
    (args.out / "ibsi_phantom.json").write_text(json.dumps(phantom))


if __name__ == "__main__":
    main()
