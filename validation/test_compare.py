"""Regression tests for the schema-2 comparison CLI."""

import json
import subprocess
import sys
from pathlib import Path

import nibabel as nib
import numpy as np
import pytest

SCRIPT = Path(__file__).with_name("compare.py")


def run_cli(directory, report, golden, *args):
    report_path, golden_path = directory / "features.json", directory / "golden.json"
    report_path.write_text(json.dumps(report, allow_nan=False))
    golden_path.write_text(json.dumps(golden, allow_nan=False))
    return subprocess.run(
        [sys.executable, SCRIPT, report_path, golden_path, *args], text=True, capture_output=True
    )


def test_roi_tolerances_missing_keys_and_signed_infinity(tmp_path):
    reference = {
        "firstorder_Mean": 100,
        "shape_MeshVolume": 100,
        "glcm_Id": "-Infinity",
        "ngtdm_Contrast": 0,
    }
    report = {
        "schema_version": 2,
        "mode": "roi",
        "features": {
            "original_firstorder_Mean": 100.001,
            "original_shape_MeshVolume": 100.001,
            "original_glcm_Id": "Infinity",
            "original_glcm_Idm": 1,
        },
    }
    golden = {"metadata": {"engine": "mlxradiomics"}, "mode": "roi", "features": reference}
    result = run_cli(tmp_path, report, golden)
    assert result.returncode == 1, result.stderr
    lines = {line.split("\t")[0]: line.split("\t")[-1] for line in result.stdout.splitlines()}
    assert {k: lines[k] for k in reference} == {
        "firstorder_Mean": "PASS",
        "shape_MeshVolume": "FAIL",
        "glcm_Id": "FAIL",
        "ngtdm_Contrast": "MISSING",
    }
    assert lines["original_glcm_Idm"] == "UNEXPECTED"


def test_nifti_report_preserves_layout_and_nonfinite_values(tmp_path):
    expected = np.array([[5.0, np.nan, np.inf], [-np.inf, 0.0, -1.25]], dtype=np.float32)
    path = tmp_path / "mean.nii.gz"
    nib.save(nib.Nifti1Image(expected.T, np.eye(4)), path)
    report = {"schema_version": 2, "mode": "voxel", "maps": {"original_firstorder_Mean": path.name}}
    golden = {
        "metadata": {"engine": "mlxradiomics"},
        "mode": "voxel",
        "shape": [2, 3],
        "features": {"firstorder_Mean": [5.0, "NaN", "Infinity", "-Infinity", 0.0, -1.25]},
    }
    result = run_cli(tmp_path, report, golden)
    assert result.returncode == 0, result.stderr + result.stdout
    assert "6 compared values" in result.stdout
    golden["shape"] = [3, 2]
    result = run_cli(tmp_path, report, golden)
    assert result.returncode == 2
    assert "differs from golden grid" in result.stderr


@pytest.mark.parametrize(("value", "code"), [(596, 0), (596.001, 1)])
def test_ibsi_published_absolute_tolerance(tmp_path, value, code):
    table = (
        Path(__file__).resolve().parents[1]
        / "crates/radiomics/tests/data/reference/ibsi_1_reference.json"
    )
    rows = [r for r in json.loads(table.read_text())["rows"] if r["tag"] == "morph_vol_approx"]
    report = {"schema_version": 2, "mode": "roi", "features": {"original_shape_VoxelVolume": value}}
    result = run_cli(tmp_path, report, {"rows": rows}, "--ibsi-family", "Morphology")
    assert result.returncode == code, result.stderr + result.stdout
