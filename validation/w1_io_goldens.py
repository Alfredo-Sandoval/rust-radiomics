"""Generate the I/O fixtures and goldens (mlx ``read_image``/``read_mask``/headers).

Writes tiny NIfTI and NRRD files plus ``io_goldens.json`` into
``crates/radiomics/tests/data/w1`` and copies the IBSI 1 digital phantom (CC BY 4.0) from
``$MLXRAD_REPO/data/ibsi/digital_phantom``.
"""

from __future__ import annotations

import gzip
import os
import re
import shutil
import sys
from pathlib import Path

import nibabel as nib
import numpy as np
import SimpleITK as sitk

sys.path.insert(0, str(Path(__file__).parent))
from w1_common import DATA, array_record, craft_nifti, quaternion_of, write_json  # noqa: E402

from mlxradiomics.imaging.io import (  # noqa: E402
    has_detached_data, read_image, read_image_dimension, read_mask, write_feature_map, Volume,
)

IBSI_PHANTOM = Path(os.environ["MLXRAD_REPO"]) / "data" / "ibsi" / "digital_phantom"
FILES = DATA / "io"


def rotation_z(angle: float) -> np.ndarray:
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[c, -s, 0], [s, c, 0], [0, 0, 1.0]])


def rotation_x(angle: float) -> np.ndarray:
    c, s = np.cos(angle), np.sin(angle)
    return np.array([[1.0, 0, 0], [0, c, -s], [0, s, c]])


def ramp(shape, dtype, seed=3):
    rng = np.random.default_rng(seed)
    if np.dtype(dtype).kind == "f":
        return (rng.normal(size=shape) * 100).astype(dtype)
    info = np.iinfo(dtype)
    low = max(info.min, -(2**40))
    high = min(info.max, 2**40)
    values = rng.integers(low if low < 0 else 0, min(high, 5000) if high > 5000 else high, size=shape)
    return values.astype(dtype)


# SimpleITK messages name source files under its build machine's directories.
_ABSOLUTE_DIRECTORY = re.compile(r"(?<!\S)/(?:[\w.+-]+/)+")


def _message(error: Exception, path: Path) -> str:
    """Exception text with the input path as ``{path}`` and other absolute directories dropped."""
    return _ABSOLUTE_DIRECTORY.sub("", str(error).replace(str(path), "{path}"))


def record(path: Path) -> dict:
    """What mlx-radiomics does with ``path`` (results or exception class and message)."""
    out: dict = {"file": str(path.relative_to(DATA))}
    for key, call in (("has_detached", has_detached_data), ("dimension", read_image_dimension)):
        try:
            out[key] = {"ok": call(path)}
        except Exception as error:  # noqa: BLE001 - recording behaviour
            out[key] = {"error": {"class": type(error).__name__, "message": _message(error, path)}}
    for key, call in (("image", read_image), ("mask", read_mask)):
        try:
            volume = call(path)
            out[key] = {"ok": {
                "spacing": list(volume.spacing), "origin": list(volume.origin),
                "direction": list(volume.direction), "array": array_record(volume.array),
            }}
        except Exception as error:  # noqa: BLE001
            out[key] = {"error": {"class": type(error).__name__, "message": _message(error, path).splitlines()[0]}}
    return out


def build() -> list[Path]:
    FILES.mkdir(parents=True, exist_ok=True)
    paths: list[Path] = []

    def add(path: Path) -> None:
        paths.append(path)

    shape = (4, 3, 5)  # numpy (z, y, x): x = 5, y = 3, z = 4
    base = ramp(shape, np.int16)
    rot = rotation_z(0.37) @ rotation_x(-0.21)
    q = quaternion_of(rot)
    spacing = (0.7, 1.3, 2.2)
    translate = (11.0, -7.0, 4.0)
    srow = np.hstack([rot @ np.diag(spacing), np.array(translate)[:, None]])

    cases = {
        "nifti_qform": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate),
        "nifti_sform": dict(scode=1, pixdim=spacing, srow=srow),
        "nifti_sform_wins_at_code1": dict(qcode=1, scode=1, pixdim=spacing, quatern=(0, 0, 0), qoffset=(1, 2, 3), srow=srow),
        "nifti_qform_wins_at_sform2": dict(qcode=1, scode=2, pixdim=spacing, quatern=q, qoffset=translate, srow=srow * 2),
        "nifti_no_orientation": dict(pixdim=spacing),
        "nifti_qfac_negative": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, qfac=-1.0),
        "nifti_units_meter": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, units=1),
        "nifti_units_micron": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, units=3),
        "nifti_zero_pixdim": dict(qcode=1, pixdim=(0.0, 2.5, 3.5), quatern=q, qoffset=translate),
        "nifti_big_endian": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, endian=">"),
        "nifti_scaled_int16": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, slope=2.5, inter=-1024.0),
        "nifti_slope_zero_inter": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, slope=0.0, inter=5.0),
        "nifti_trailing_unit_dim": dict(qcode=1, pixdim=spacing, quatern=q, qoffset=translate, extra_dims=(1,)),
        "nifti_oblique_sform_shear": dict(scode=1, pixdim=spacing, srow=np.array([[1, 0.3, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0.0]])),
    }
    for name, kwargs in cases.items():
        path = FILES / f"{name}.nii"
        craft_nifti(path, base, **kwargs)
        add(path)

    # 2-D, with a tilted slab to check the 2x2 block rule.
    flat = ramp((6, 5), np.int16)
    craft_nifti(FILES / "nifti_2d_qform.nii", flat, qcode=1, pixdim=(1.5, 2.5, 1.0), quatern=q, qoffset=translate)
    add(FILES / "nifti_2d_qform.nii")
    craft_nifti(FILES / "nifti_2d_inplane.nii", flat, qcode=1, pixdim=(1.5, 2.5, 1.0), quatern=quaternion_of(rotation_z(0.3)), qoffset=translate)
    add(FILES / "nifti_2d_inplane.nii")

    # Pixel types, with values at the type limits.
    for dtype in (np.uint8, np.int8, np.uint16, np.int16, np.uint32, np.int32, np.uint64, np.int64, np.float32, np.float64):
        data = ramp((3, 4, 2), dtype)
        if np.dtype(dtype).kind in "iu":
            data[0, 0, 0] = np.iinfo(dtype).max
            data[0, 0, 1] = np.iinfo(dtype).min
        path = FILES / f"nifti_dtype_{np.dtype(dtype).name}.nii"
        craft_nifti(path, data, qcode=1, pixdim=(1.0, 2.0, 3.0), quatern=(0, 0, 0), qoffset=(1, 2, 3))
        add(path)
    # Scaled types.
    for dtype, slope, inter in ((np.uint8, 0.5, 3.0), (np.int32, 0.1, 3.3), (np.float32, 0.1, 3.3), (np.float64, 0.1, 3.3)):
        data = ramp((3, 4, 2), dtype)
        path = FILES / f"nifti_scaled_{np.dtype(dtype).name}.nii"
        craft_nifti(path, data, qcode=1, pixdim=(1.0, 2.0, 3.0), slope=slope, inter=inter)
        add(path)

    # Non-finite values, complex, RGB, 4-D, 1-D, paired headers, NIfTI-2, truncated.
    for label, bad in (("nan", np.nan), ("inf", np.inf), ("ninf", -np.inf)):
        data = ramp((3, 4, 2), np.float32)
        data[1, 1, 1] = bad
        craft_nifti(FILES / f"nifti_nonfinite_{label}.nii", data, qcode=1, pixdim=(1, 1, 1))
        add(FILES / f"nifti_nonfinite_{label}.nii")
    data = ramp((3, 4, 2), np.float64)
    data[0, 0, 0] = np.nan
    craft_nifti(FILES / "nifti_nonfinite_nan_f64.nii", data)
    add(FILES / "nifti_nonfinite_nan_f64.nii")
    craft_nifti(FILES / "nifti_complex64.nii", np.zeros((3, 4, 2), np.complex64), qcode=1)
    add(FILES / "nifti_complex64.nii")
    craft_nifti(FILES / "nifti_rgb.nii", np.zeros((3, 4, 2), np.uint8), qcode=1, datatype=128, raw_bytes=bytes(3 * 24))
    add(FILES / "nifti_rgb.nii")
    craft_nifti(FILES / "nifti_4d.nii", ramp((2, 4, 3), np.int16), qcode=1, extra_dims=(2,))
    add(FILES / "nifti_4d.nii")
    craft_nifti(FILES / "nifti_paired_ni1.nii", base, qcode=1, magic=b"ni1\0")
    add(FILES / "nifti_paired_ni1.nii")
    analyze = bytearray(352)
    analyze[0:4] = (348).to_bytes(4, "little")
    nib.save(nib.Nifti2Image(base.T.copy(), np.eye(4)), str(FILES / "nifti2.nii"))
    add(FILES / "nifti2.nii")
    (FILES / "not_a_volume.txt").write_text("text")
    add(FILES / "not_a_volume.txt")

    # Gzip-compressed copies.
    for name in ("nifti_qform", "nifti_dtype_uint64", "nifti_scaled_int16"):
        source = FILES / f"{name}.nii"
        target = FILES / f"{name}.nii.gz"
        with source.open("rb") as raw, gzip.open(target, "wb") as zipped:
            zipped.write(raw.read())
        add(target)

    # NRRD written by SimpleITK: raw, gzip, 2-D, oblique, detached header.
    img = sitk.GetImageFromArray(ramp((4, 3, 5), np.int16))
    img.SetSpacing((0.7, 1.3, 2.2))
    img.SetOrigin((11.0, -7.0, 4.0))
    img.SetDirection(tuple((rotation_z(0.37) @ rotation_x(-0.21)).reshape(-1)))
    sitk.WriteImage(img, str(FILES / "itk_oblique_raw.nrrd"))
    add(FILES / "itk_oblique_raw.nrrd")
    sitk.WriteImage(img, str(FILES / "itk_oblique_gzip.nrrd"), useCompression=True)
    add(FILES / "itk_oblique_gzip.nrrd")
    float_img = sitk.GetImageFromArray(ramp((4, 3, 5), np.float32))
    float_img.SetSpacing((0.5, 0.75, 1.25))
    sitk.WriteImage(float_img, str(FILES / "itk_float32.nrrd"))
    add(FILES / "itk_float32.nrrd")
    flat_img = sitk.GetImageFromArray(ramp((6, 5), np.uint16))
    flat_img.SetSpacing((1.5, 2.5))
    flat_img.SetOrigin((3.0, -2.0))
    th = 0.4
    flat_img.SetDirection((np.cos(th), -np.sin(th), np.sin(th), np.cos(th)))
    sitk.WriteImage(flat_img, str(FILES / "itk_2d.nrrd"))
    add(FILES / "itk_2d.nrrd")
    u64 = sitk.GetImageFromArray(np.array([[0, 2**64 - 1, 2**53 + 1], [7, 2**32, 5]], dtype=np.uint64))
    sitk.WriteImage(u64, str(FILES / "itk_uint64.nrrd"))
    add(FILES / "itk_uint64.nrrd")
    sitk.WriteImage(img, str(FILES / "itk_detached.nhdr"))
    add(FILES / "itk_detached.nhdr")
    vector = sitk.GetImageFromArray(np.stack((ramp((3, 4, 5), np.float32), ramp((3, 4, 5), np.float32))).transpose(1, 2, 3, 0), isVector=True)
    sitk.WriteImage(vector, str(FILES / "itk_vector.nrrd"))
    add(FILES / "itk_vector.nrrd")

    # Hand-written NRRD: text encoding, big-endian raw, RAS space, spacings only, int16.
    def nrrd(name, header: str, body: bytes) -> None:
        (FILES / name).write_bytes(header.encode() + b"\n" + body)
        add(FILES / name)

    values = np.arange(24, dtype=np.int16).reshape(4, 3, 2) - 5  # numpy (z, y, x)
    text_body = (" ".join(str(int(v)) for v in values.reshape(-1)) + "\n").encode()
    common = "NRRD0004\ntype: short\ndimension: 3\nsizes: 2 3 4\n"
    nrrd("hand_text.nrrd", common + "encoding: ascii\nspace: left-posterior-superior\nspace directions: (2,0,0) (0,3,0) (0,0,4)\nspace origin: (1,2,3)\n", text_body)
    nrrd("hand_ras.nrrd", common + "encoding: raw\nendian: little\nspace: right-anterior-superior\nspace directions: (2,0,0) (0,3,0) (0,0,4)\nspace origin: (1,2,3)\n", values.astype("<i2").tobytes())
    nrrd("hand_big_endian.nrrd", common + "encoding: raw\nendian: big\nspace: left-posterior-superior\nspace directions: (0,2,0) (-3,0,0) (0,0,4)\nspace origin: (1,2,3)\n", values.astype(">i2").tobytes())
    nrrd("hand_spacings_only.nrrd", common + "encoding: raw\nendian: little\nspacings: 0.5 0.25 2\n", values.astype("<i2").tobytes())
    nrrd("hand_no_geometry.nrrd", common + "encoding: raw\nendian: little\n", values.astype("<i2").tobytes())
    four_values = np.arange(120, dtype="<i2")
    nrrd("hand_4d.nrrd", "NRRD0004\ntype: short\ndimension: 4\nsizes: 2 3 4 5\nkinds: domain domain domain domain\nencoding: raw\nendian: little\n", four_values.tobytes())
    nrrd("hand_truncated.nrrd", common + "encoding: raw\nendian: little\n", values.astype("<i2").tobytes()[:10])
    nrrd("hand_oblique.nrrd", common + "encoding: raw\nendian: little\nspace: left-posterior-superior\nspace directions: (1,1,0) (1,2,0) (0,0,1)\n", values.astype("<i2").tobytes())
    (FILES / "hand_detached.nhdr").write_text(common + "encoding: raw\nendian: little\ndata file: hand_detached.raw\nspace: left-posterior-superior\nspace directions: (2,0,0) (0,3,0) (0,0,4)\n\n")
    (FILES / "hand_detached.raw").write_bytes(values.astype("<i2").tobytes())
    add(FILES / "hand_detached.nhdr")

    # IBSI 1 digital phantom, converted by the reference data (CC BY 4.0).
    shutil.copy(IBSI_PHANTOM / "nifti" / "image" / "phantom.nii.gz", FILES / "ibsi_digital_phantom_image.nii.gz")
    shutil.copy(IBSI_PHANTOM / "nifti" / "mask" / "mask.nii.gz", FILES / "ibsi_digital_phantom_mask.nii.gz")
    add(FILES / "ibsi_digital_phantom_image.nii.gz")
    add(FILES / "ibsi_digital_phantom_mask.nii.gz")
    return paths


def feature_map_golden() -> dict:
    """Header and data bytes that mlx writes for one oblique float feature map."""
    rot = rotation_z(0.37) @ rotation_x(-0.21)
    for suffix in (".nii", ".nii.gz"):
        pass
    reference = Volume(
        array=np.zeros((4, 3, 5), np.int16), spacing=(2.2, 1.3, 0.7), origin=(11.0, -7.0, 4.0),
        direction=tuple(rot.reshape(-1)),
    )
    rng = np.random.default_rng(5)
    values = rng.normal(size=(4, 3, 5)).astype(np.float64)
    values[0, 0, 0] = np.nan
    values[0, 0, 1] = np.inf
    plain = FILES / "map_plain.nii"
    write_feature_map(plain, values, reference=reference)
    zipped = FILES / "map_gz.nii.gz"
    write_feature_map(zipped, values.astype(np.float32), reference=reference)
    decompressed = gzip.open(zipped, "rb").read()
    out = {
        "reference": {"shape": [4, 3, 5], "spacing": list(reference.spacing), "origin": list(reference.origin),
                      "direction": list(reference.direction)},
        "values": [("nan" if np.isnan(v) else "inf" if np.isinf(v) else float(v)) for v in values.reshape(-1)],
        "nifti_bytes_hex": plain.read_bytes().hex(),
        "gz_decompressed_equals_plain_float32": decompressed[352:] == values.astype(np.float32).tobytes(),
    }
    plain.unlink()
    zipped.unlink()
    # Two-dimensional map.
    flat_reference = Volume(array=np.zeros((5, 6), np.int16), spacing=(1.5, 2.5), origin=(3.0, -2.0),
                            direction=(np.cos(0.4), -np.sin(0.4), np.sin(0.4), np.cos(0.4)))
    flat_values = rng.normal(size=(5, 6)).astype(np.float32)
    plain2 = FILES / "map_plain2.nii"
    write_feature_map(plain2, flat_values, reference=flat_reference)
    out["flat"] = {
        "reference": {"shape": [5, 6], "spacing": list(flat_reference.spacing), "origin": list(flat_reference.origin),
                      "direction": list(flat_reference.direction)},
        "values": [float(v) for v in flat_values.reshape(-1)],
        "nifti_bytes_hex": plain2.read_bytes().hex(),
    }
    plain2.unlink()
    return out


def main() -> None:
    paths = build()
    goldens = [record(path) for path in paths]
    write_json("io_goldens.json", {"cases": goldens, "feature_map": feature_map_golden()})
    (FILES / "NOTICE.md").write_text(
        "# Fixture notices\n\n`ibsi_digital_phantom_*.nii.gz` are the IBSI 1 digital phantom image and mask "
        "(Image Biomarker Standardisation Initiative, https://ibsi.readthedocs.io), converted to NIfTI, "
        "licensed CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/). All other files are synthetic "
        "and generated by `validation/w1_io_goldens.py` (Apache-2.0).\n",
    )
    print(f"wrote {len(goldens)} I/O cases")


if __name__ == "__main__":
    main()
