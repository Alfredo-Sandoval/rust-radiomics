"""Compare a schema-2 original-image ROI or voxel report with a parity reference."""

import argparse
import json
import math
import sys
from pathlib import Path

MLX_RTOL = 2e-5
MLX_SHAPE_RTOL = 3e-6
MLX_ATOL = 2e-6
PYR_RTOL = 1e-8
PYR_ATOL = 1e-10
SPECIAL = {"NaN": math.nan, "Infinity": math.inf, "-Infinity": -math.inf}


def number(value):
    if type(value) in (float, int):
        return float(value)
    if isinstance(value, str) and value in SPECIAL:
        return SPECIAL[value]
    raise ValueError(f"invalid scalar {value!r}")


def errors(actual, reference):
    if actual == reference or (math.isnan(actual) and math.isnan(reference)):
        return 0.0, 0.0
    if not math.isfinite(actual) or not math.isfinite(reference):
        return math.inf, math.inf
    delta = abs(actual - reference)
    return delta, delta / abs(reference) if reference else math.inf


def read_map(report_path, value, shape):
    import nibabel as nib

    if not isinstance(value, str):
        raise ValueError("schema-2 maps must name NIfTI files")
    # NIfTI stores (x, y, z). NiBabel preserves NaN and signed infinity.
    array = nib.load(report_path.parent / value).get_fdata().T
    if list(array.shape) != shape:
        raise ValueError(f"map grid {list(array.shape)} differs from golden grid {shape}")
    return array.ravel()


def references(golden, family):
    if family is not None:
        rows = [
            row
            for row in golden["rows"]
            if row["family"] == family
            and row["key"]
            and row["value"] is not None
            and row["tolerance"] is not None
        ]
        if not rows:
            raise ValueError(f"no published values mapped for IBSI family {family!r}")
        return "roi", {r["key"]: (r["value"], 0.0, r["tolerance"]) for r in rows}
    if "rows" in golden:
        raise ValueError("select a table family with --ibsi-family (include the aggregation)")
    if "expected_error" in golden:
        raise ValueError("this case expects an extraction error: " + golden["expected_error"])
    mode = golden["mode"]
    engine = golden["metadata"]["engine"]
    if engine not in {"mlxradiomics", "pyradiomics"} or mode not in {"roi", "voxel"}:
        raise ValueError(f"unknown engine or mode: {engine} {mode}")
    expected = {}
    for key, value in golden["features"].items():
        if engine == "pyradiomics":
            rtol, atol = PYR_RTOL, PYR_ATOL
        else:
            rtol = MLX_SHAPE_RTOL if key.startswith(("shape_", "shape2D_")) else MLX_RTOL
            atol = MLX_ATOL
        expected[key] = (value, rtol, atol)
    return mode, expected


def compare(report_path, golden_path, *, ibsi_family=None, allow_extra=False):
    report = json.loads(report_path.read_text())
    golden = json.loads(golden_path.read_text())
    mode, expected = references(golden, ibsi_family)
    if report["schema_version"] != 2 or report["mode"] != mode:
        raise ValueError(f"expected a schema-2 {mode} report")
    actual = report["features" if mode == "roi" else "maps"]
    if not expected or not isinstance(actual, dict):
        raise ValueError("expected a reference selection and a report feature dictionary")
    print("feature\tmax_abs\tmax_rel\trtol\tatol\tfailed/samples\tstatus")
    failures = samples = 0
    max_abs = max_rel = 0.0
    for key, (reference, rtol, atol) in sorted(expected.items()):
        report_key = "original_" + key
        if report_key not in actual:
            print(f"{key}\t-\t-\t{rtol:g}\t{atol:g}\t-\tMISSING")
            failures += 1
            continue
        if mode == "roi":
            values, refs = [number(actual[report_key])], [reference]
        else:
            values = read_map(report_path, actual[report_key], golden["shape"])
            refs = reference
        failed = 0
        worst_abs = worst_rel = 0.0
        for value, ref in zip(values, refs, strict=True):
            ref = number(ref)
            absolute, relative = errors(float(value), ref)
            if math.isinf(absolute) or (absolute and absolute > atol + rtol * abs(ref)):
                failed += 1
            worst_abs, worst_rel = max(worst_abs, absolute), max(worst_rel, relative)
        samples += len(refs)
        max_abs, max_rel = max(max_abs, worst_abs), max(max_rel, worst_rel)
        failures += int(failed > 0)
        status = "FAIL" if failed else "PASS"
        print(
            f"{key}\t{worst_abs:.9g}\t{worst_rel:.9g}\t{rtol:g}\t{atol:g}\t{failed}/{len(refs)}\t{status}"
        )
    extra = sorted(actual.keys() - {"original_" + key for key in expected})
    if not allow_extra and ibsi_family is None:
        failures += len(extra)
        for key in extra:
            print(f"{key}\t-\t-\t-\t-\t-\tUNEXPECTED")
    print(
        f"{len(expected)} reference features, {samples} compared values, {len(extra)} extra keys, "
        f"{failures} feature failures, max absolute error {max_abs:.9g}, max relative error {max_rel:.9g}"
    )
    return int(failures > 0)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", type=Path, help="Rust features.json (schema 2)")
    parser.add_argument(
        "golden", type=Path, help="parity golden or IBSI digital-phantom reference table"
    )
    parser.add_argument("--ibsi-family", help="exact family column in the IBSI table")
    parser.add_argument(
        "--allow-extra", action="store_true", help="compare only the reference selection"
    )
    args = parser.parse_args()
    try:
        return compare(
            args.report, args.golden, ibsi_family=args.ibsi_family, allow_extra=args.allow_extra
        )
    except (KeyError, OSError, TypeError, ValueError, RuntimeError, ImportError) as error:
        parser.exit(2, f"comparison error: {error}\n")


if __name__ == "__main__":
    sys.exit(main())
