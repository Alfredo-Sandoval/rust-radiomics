"""Generate the redistributable parity inputs and the IBSI digital-phantom table."""

import csv
import hashlib
import io
import json
import math
import os
import platform
import re
import subprocess
from importlib.metadata import version
from pathlib import Path
from tempfile import NamedTemporaryFile

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "crates/radiomics/tests/data/reference"
MLX_REV = "b58783cc10d02d69533e67443123afa423760113"
NPZ_REV = "7964f3047069aee9dd3da577d91b7ba24d4fa722"
NPZ_CASES = (
    "bin_boundaries_2d",
    "flat",
    "force2d",
    "larger_shifted_kernel",
    "negative_ct_multilabel",
    "singleton",
    "unmasked_neighborhood",
)
ATTRIBUTION = (
    "IBSI 1 digital phantom, designed by Alex Zwanenburg, Image Biomarker "
    "Standardisation Initiative. NIfTI image and mask converted to JSON. "
    "CC BY 4.0, https://creativecommons.org/licenses/by/4.0/ . "
    "https://github.com/theibsi/data_sets/tree/master/ibsi_1_digital_phantom"
)


def checkout():
    path = Path(os.environ["MLXRAD_CHECKOUT"]).resolve()
    if git(path, "rev-parse", "HEAD").decode().strip() != MLX_REV:
        raise ValueError(f"MLXRAD_CHECKOUT must point to mlx-radiomics v1.1.0 ({MLX_REV})")
    return path


def git(repo, *args):
    return subprocess.check_output(["git", "-C", repo, *args])


def metadata(command, **source):
    return {
        "schema_version": 1,
        "command": command,
        "python": platform.python_version(),
        "numpy": np.__version__,
        **source,
    }


def numbers(value):
    """Strict JSON representation of an ndarray, list, or scalar."""
    if isinstance(value, np.ndarray):
        return numbers(value.tolist())
    if isinstance(value, list):
        return [numbers(item) for item in value]
    if isinstance(value, float) and not math.isfinite(value):
        return "NaN" if math.isnan(value) else ("Infinity" if value > 0 else "-Infinity")
    return value


def write_json(path, record):
    # Keep fixture diffs readable: one feature or table row per line, arrays in C order.
    compact = json.JSONEncoder(separators=(",", ":"), allow_nan=False).encode
    lines = []
    for key, value in record.items():
        if key in {"features", "recording"} and value:
            entries = [f"  {compact(name)}:{compact(item)}" for name, item in value.items()]
            encoded = "{\n" + ",\n".join(entries) + "\n}"
        elif key == "rows":
            encoded = "[\n  " + ",\n  ".join(map(compact, value)) + "\n]"
        else:
            encoded = compact(value)
        lines.append(f"{compact(key)}:{encoded}")
    payload = ("{\n" + ",\n".join(lines) + "\n}\n").encode()
    if len(payload) >= 1_000_000:
        raise ValueError(f"{path.name} exceeds the 1 MB fixture limit")
    path.parent.mkdir(parents=True, exist_ok=True)
    with NamedTemporaryFile(dir=path.parent, delete=False) as stream:
        stream.write(payload)
    Path(stream.name).replace(path)


def read_pair(image_path, mask_path):
    import SimpleITK as sitk

    image, mask = (sitk.ReadImage(str(path)) for path in (image_path, mask_path))
    if not image.IsSameImageGeometryAs(mask):
        raise ValueError(f"inconsistent image/mask geometry for {image_path.name}")
    return (
        sitk.GetArrayFromImage(image),
        sitk.GetArrayFromImage(mask),
        list(reversed(image.GetSpacing())),
        image.GetOrigin(),
        image.GetDirection(),
    )


def write_case(
    name, image, mask, spacing, *, label=1, origin=None, direction=None, recording=None, **source
):
    ndim = image.ndim
    if image.shape != mask.shape or ndim not in (2, 3) or not np.any(mask == label):
        raise ValueError(name)
    case = {
        "metadata": metadata("python validation/fixtures.py", **source),
        "case": name,
        "shape": image.shape,
        "image": numbers(image.ravel()),
        "mask": mask.astype(int).ravel().tolist(),
        "label": label,
        "spacing": list(map(float, spacing)),
        "origin": [0.0] * ndim if origin is None else origin,
        "direction": np.eye(ndim).ravel().tolist() if direction is None else direction,
    }
    if recording is not None:
        case["recording"] = recording
    write_json(DATA / "cases" / f"{name}.json", case)


def convert_npz(repo, name):
    path = f"tests/fixtures/darwin-arm64/{name}.npz"
    blob = git(repo, "show", f"{NPZ_REV}:{path}")
    with np.load(io.BytesIO(blob), allow_pickle=False) as archive:
        original = json.loads(archive["metadata"].item())
        recording = {"metadata": original}
        for key in archive.files:
            if key in {"image", "mask", "spacing", "metadata"}:
                continue
            arr = archive[key]
            if key.endswith("_counts"):
                # Lossless sparse representation of the upstream integer tensors.
                offsets = np.flatnonzero(arr)
                recording[key] = {
                    "shape": arr.shape,
                    "offsets": offsets.tolist(),
                    "values": arr.ravel()[offsets].tolist(),
                }
            else:
                recording[key] = numbers(arr)
        write_case(
            "npz_" + name,
            archive["image"],
            archive["mask"],
            archive["spacing"],
            label=original["settings"]["label"],
            recording=recording,
            mlx_fixture_commit=NPZ_REV,
            source=path,
            sha256=hashlib.sha256(blob).hexdigest(),
        )


FAMILIES = {
    "Morphology": "shape",
    "Local intensity": "localintensity",
    "Statistics": "firstorder",
    "Intensity histogram": "intensityhistogram",
    "Intensity volume histogram": "ivh",
    "Co-occurrence matrix": "glcm",
    "Run length matrix": "glrlm",
    "Size zone matrix": "glszm",
    "Distance zone matrix": "gldzm",
    "Neighbourhood grey tone difference matrix": "ngtdm",
    "Neighbouring grey level dependence matrix": "gldm",
}


def title_key(family, title):
    title = title.lower().replace("’", "'").replace(r"\ :sup:`th`", "th")
    for abbr, full in {
        "aabb": "axis-aligned bounding box",
        "ombb": "oriented minimum bounding box",
        "aee": "approximate enclosing ellipsoid",
        "mvee": "minimum volume enclosing ellipsoid",
    }.items():
        title = title.replace(f"({abbr})", f"({full})")
    if family in {"firstorder", "intensityhistogram"} and "histogram gradient" not in title:
        title = re.sub(
            r"\b(discretised |intensity-based |intensity histogram |intensity\b)", "", title
        )
    if family in {"glszm", "gldzm"}:
        title = title.replace("level zone emphasis", "level emphasis")
    if family == "glrlm":
        title = title.replace(" runs ", " run ")
    if family == "gldm":
        title = title.replace(" count percentage", " percentage")
    return " ".join(title.split())


IVH_TITLES = {
    "ivh_v10": "ivh_VolumeAtIntensityFraction10",
    "ivh_v90": "ivh_VolumeAtIntensityFraction90",
    "ivh_i10": "ivh_IntensityAtVolumeFraction10",
    "ivh_i90": "ivh_IntensityAtVolumeFraction90",
    "ivh_diff_v10_v90": "ivh_VolumeFractionDifference",
    "ivh_diff_i10_i90": "ivh_IntensityFractionDifference",
}


def reference_table(path, source):
    definitions = json.loads((source / "docs/ibsi-features.json").read_text())["definitions"]
    titles = {}
    ids = {}
    for definition in definitions:
        for key in definition["public_names"]:
            family = key.split("_")[0]
            ids[key] = definition["id"]
            if len(definition["public_names"]) == 1:
                index = (family, title_key(family, definition["title"]))
                if index in titles:
                    raise ValueError(f"ambiguous IBSI definition {index}")
                titles[index] = key

    rows = []
    with path.open(newline="") as stream:
        for row in csv.DictReader(stream):
            family = FAMILIES.get(row["family"].split(" (")[0])
            tag = row["tag"]
            key = IVH_TITLES.get(tag) or titles.get((family, title_key(family, row["feature"])))
            rows.append(
                {
                    "family": row["family"],
                    "name": row["feature"],
                    "tag": tag,
                    "key": key,
                    "ibsi_id": ids[key] if key else None,
                    "value": float(row["reference value"]) if row["reference value"] else None,
                    "tolerance": float(row["tolerance"]) if row["tolerance"] else None,
                }
            )
    write_json(
        DATA / "ibsi_1_reference.json",
        {
            "metadata": metadata(
                "python validation/fixtures.py",
                source="IBSI 1, digital phantom.csv",
                attribution="Image Biomarker Standardisation Initiative, https://theibsi.github.io/ibsi1/",
                sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                mlx_commit=MLX_REV,
            ),
            "rows": rows,
        },
    )


def main():
    repo = Path(os.environ["MLXRAD_REPO"])
    ibsi = repo / "data/ibsi"
    source = checkout()
    nifti = ibsi / "digital_phantom/nifti"
    image, mask, spacing, origin, direction = read_pair(
        nifti / "image/phantom.nii.gz", nifti / "mask/mask.nii.gz"
    )
    write_case(
        "ibsi_digital",
        image,
        mask,
        spacing,
        origin=origin,
        direction=direction,
        attribution=ATTRIBUTION,
        simpleitk=version("SimpleITK"),
    )
    reference_table(ibsi / "ibsi_1_reference/digital phantom.csv", source)
    for name in NPZ_CASES:
        convert_npz(repo, name)

    y, x = np.indices((4, 5))
    write_case(
        "synthetic_fractional_2d",
        (9 * x - 15 * y + 0.25) / 2,
        np.where((x + y) % 3 == 0, 1, 2),
        [1.75, 0.625],
        label=2,
    )
    z, y, x = np.indices((3, 4, 5))
    write_case(
        "synthetic_multilabel_3d",
        22.5 * z + 13 * x - 17 * y - 10.25,
        np.where((x + 2 * y + z) % 4 == 0, 1, 2),
        [2.25, 1.5, 0.75],
        label=2,
    )
    write_case("synthetic_constant", np.full((3, 4), -18.5), np.ones((3, 4)), [1, 1])
    write_case(
        "synthetic_single_voxel",
        x + 4 * y + 17 * z,
        (x == 2) & (y == 1) & (z == 1),
        [3, 1.25, 0.75],
    )


if __name__ == "__main__":
    main()
