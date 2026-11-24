"""Shared helpers for the W1 (imaging) golden generators.

Goldens are produced by running mlx-radiomics v1.1.0 (``PYTHONPATH=<checkout>/src``) and
SimpleITK/NiBabel on small deterministic inputs. The Rust tests under
``crates/radiomics/tests/w1_*.rs`` read the JSON written here; nothing in the Rust crate
imports Python.

Run with the mlx-radiomics virtual environment::

    PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=<mlx-radiomics>/src MLX_ENABLE_TF32=0 \\
        <mlx-radiomics>/.venv/bin/python validation/w1_<name>.py
"""

from __future__ import annotations

import json
import struct
from pathlib import Path

import numpy as np

REPOSITORY = Path(__file__).resolve().parents[1]
DATA = REPOSITORY / "crates" / "radiomics" / "tests" / "data" / "w1"


def jsonable(value):
    """Convert NumPy scalars and arrays to plain JSON values (floats keep full precision)."""
    if isinstance(value, np.ndarray):
        return [jsonable(v) for v in value.reshape(-1).tolist()]
    if isinstance(value, (np.integer,)):
        return int(value)
    if isinstance(value, (np.floating,)):
        return jsonable(float(value))
    if isinstance(value, (list, tuple)):
        return [jsonable(v) for v in value]
    if isinstance(value, dict):
        return {k: jsonable(v) for k, v in value.items()}
    if isinstance(value, float) and not np.isfinite(value):
        return {"nonfinite": repr(value)}
    return value


def array_record(array: np.ndarray) -> dict:
    """Shape, dtype name and flat C-order values of ``array``."""
    flat = array.reshape(-1)
    if array.dtype.kind == "f":
        values = [float(v) if np.isfinite(v) else {"nonfinite": repr(float(v))} for v in flat]
    elif array.dtype.kind == "b":
        values = [bool(v) for v in flat]
    else:
        values = [int(v) for v in flat]
    return {"dtype": array.dtype.name, "shape": list(array.shape), "values": values}


def write_json(name: str, payload: object) -> None:
    DATA.mkdir(parents=True, exist_ok=True)
    (DATA / name).write_text(json.dumps(jsonable(payload), separators=(",", ":"), allow_nan=False) + "\n")


def craft_nifti(
    path: Path, array: np.ndarray, *, qcode=0, scode=0, pixdim=(1.0, 1.0, 1.0), quatern=(0.0, 0.0, 0.0),
    qoffset=(0.0, 0.0, 0.0), srow=None, qfac=1.0, units=2, datatype=None, endian="<",
    slope=1.0, inter=0.0, extra_dims=(), magic=b"n+1\0", raw_bytes=None,
) -> None:
    """Write a single-file NIfTI-1 with exact header control.

    ``array`` is in NumPy order (z, y, x); NIfTI stores x fastest, which is the C order of
    that array. ``extra_dims`` appends trailing NIfTI dimensions (for example ``(1,)``).
    """
    codes = {np.uint8: 2, np.int16: 4, np.int32: 8, np.float32: 16, np.complex64: 32, np.float64: 64,
             np.int8: 256, np.uint16: 512, np.uint32: 768, np.int64: 1024, np.uint64: 1280,
             np.complex128: 1792}
    header = bytearray(352)
    sizes = list(array.shape[::-1]) + list(extra_dims)
    dims = [len(sizes)] + sizes + [1] * (7 - len(sizes))
    struct.pack_into(endian + "i", header, 0, 348)
    struct.pack_into(endian + "8h", header, 40, *dims)
    code = datatype if datatype is not None else codes[array.dtype.type]
    struct.pack_into(endian + "h", header, 70, code)
    bits = array.dtype.itemsize * 8 if raw_bytes is None else 8
    struct.pack_into(endian + "h", header, 72, bits)
    pd = ([qfac] + list(pixdim) + [1.0] * 4)[:8]
    struct.pack_into(endian + "8f", header, 76, *pd)
    struct.pack_into(endian + "f", header, 108, 352.0)
    struct.pack_into(endian + "f", header, 112, slope)
    struct.pack_into(endian + "f", header, 116, inter)
    header[123] = units
    struct.pack_into(endian + "hh", header, 252, qcode, scode)
    struct.pack_into(endian + "6f", header, 256, *quatern, *qoffset)
    if srow is not None:
        struct.pack_into(endian + "12f", header, 280, *np.asarray(srow, dtype=float).reshape(-1)[:12])
    header[344:348] = magic
    body = raw_bytes if raw_bytes is not None else np.ascontiguousarray(
        array.astype(array.dtype.newbyteorder(endian)),
    ).tobytes()
    path.write_bytes(bytes(header) + body)


def quaternion_of(rotation: np.ndarray) -> tuple[float, float, float]:
    """(b, c, d) of a proper rotation matrix, the NIfTI qform convention."""
    r = rotation
    a = 0.5 * np.sqrt(max(0.0, 1 + r[0, 0] + r[1, 1] + r[2, 2]))
    if a > 1e-6:
        return (float(0.25 * (r[2, 1] - r[1, 2]) / a), float(0.25 * (r[0, 2] - r[2, 0]) / a),
                float(0.25 * (r[1, 0] - r[0, 1]) / a))
    raise ValueError("rotation too close to a half turn for this helper")
