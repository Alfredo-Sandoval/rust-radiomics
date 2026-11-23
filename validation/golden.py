"""Generate reference values with mlx-radiomics v1.1.0 or the PyRadiomics interpreter."""

import argparse
import json
import logging
import os
import subprocess
import sys
from importlib.metadata import version
from pathlib import Path

import numpy as np

from fixtures import (
    ROOT,
    DATA,
    MLX_REV,
    NPZ_CASES,
    checkout,
    metadata,
    numbers,
    read_pair,
    write_json,
)

PROFILES = ("width25", "count32")


def settings_record(case, profile, convention):
    recorded = case.get("recording", {}).get("metadata", {}).get("settings", {})
    count = profile == "count32"
    axis = 0 if count and len(case["shape"]) == 3 else recorded.get("force2d_axis")
    return {
        "convention": convention,
        "label": case["label"],
        "bin_width": 25.0,
        "bin_count": 32 if count else None,
        "ivh_bin_count": 32 if count else None,
        "ivh_bin_width": None if count else 25.0,
        "distances": [1, 2] if count else [1],
        "force2d_axis": axis,
        "masked_kernel": recorded.get("masked_kernel", True),
        "kernel_radius": recorded.get("kernel_radius", 1),
        "symmetrical_glcm": recorded.get("symmetrical_glcm", True),
        "voxel_array_shift": recorded.get("voxel_array_shift", 0.0),
    }


def mlx_source():
    source = checkout()
    import mlxradiomics as mlx

    if (
        Path(mlx.__file__).resolve().parents[2] != source
        or os.environ.get("MLX_ENABLE_TF32") != "0"
    ):
        raise ValueError("set PYTHONPATH=$MLXRAD_CHECKOUT/src and MLX_ENABLE_TF32=0")
    sys.path.insert(0, str(source / "tests"))
    from conftest import settings_for

    return mlx, settings_for


def mlx_golden(mlx, settings_for, image, mask, case, profile, mode, convention, external):
    from mlxradiomics.imaging.preprocessing import prepare

    record = settings_record(case, profile, convention)
    fields = {k: v for k, v in record.items() if k != "convention"}
    fields["distances"] = tuple(fields["distances"])
    names = mlx.list_features(ndim=image.ndim, mode=mode)
    settings = settings_for(convention, features=names, **fields)
    command = "python validation/golden.py" + (" --external" if external else "")
    golden = {
        "metadata": metadata(
            command,
            engine="mlxradiomics",
            version="1.1.0",
            mlx_commit=MLX_REV,
            mlx=version("mlx"),
            mlx_enable_tf32=0,
        ),
        "case": case["case"],
        "shape": image.shape,
        "mode": mode,
        "settings": record,
    }
    try:
        result = mlx.extract(
            image, mask, spacing=tuple(case["spacing"]), settings=settings, mode=mode
        )
    except ValueError as error:
        if (
            mode != "voxel"
            or np.count_nonzero(mask == case["label"]) != 1
            or str(error) != "voxel extraction requires at least two selected voxels"
        ):
            raise
        return {**golden, "expected_error": str(error), "features": {}}
    if set(result.features) != {"original_" + name for name in names}:
        raise ValueError("mlx-radiomics returned a different feature selection")
    if not external:
        prepared = prepare(image, mask, settings, voxel=mode == "voxel")
        golden["prepared"] = {
            "gray_levels": prepared.gray_levels,
            "bins": prepared.bins.ravel().tolist(),
        }
    golden["features"] = {
        key.removeprefix("original_"): numbers(value.ravel() if mode == "voxel" else float(value))
        for key, value in result.features.items()
    }
    return golden


def pyradiomics_values(extractor, family, voxel, shape):
    # execute() fills failed voxel batches with its background value. Read the
    # same batch iterator with its success flag to keep those out of the oracle.
    maps = {}
    features = {}
    unavailable = {}
    batches = [None]
    if voxel:
        coords = extractor.labelledVoxelCoordinates
        batches = [coords[:, i : i + 32] for i in range(0, coords.shape[1], 32)]
    for batch in batches:
        for success, name, value in extractor._calculateFeatures(batch):
            key = f"{family}_{name}"
            if not success:
                if not voxel or key != "glcm_MCC":
                    raise ValueError(f"PyRadiomics failed to compute {key}")
                unavailable[key] = "PyRadiomics MCC eigensolver failed for a voxel batch"
                continue
            if np.iscomplexobj(value):
                if np.all(np.isnan(value)):
                    value = np.full(np.shape(value), np.nan)
                else:
                    value = np.real_if_close(value)
                    if np.iscomplexobj(value):
                        raise ValueError(f"{key} has a non-real result")
            if voxel:
                if key not in maps:
                    maps[key] = np.zeros(shape)
                maps[key][tuple(batch)] = value
            else:
                features[key] = numbers(float(np.asarray(value).item()))
    for key, array in maps.items():
        if key not in unavailable:
            features[key] = numbers(array.ravel())
    return features, unavailable


def pyradiomics_golden(image, mask, case, profile, mode, catalog):
    import SimpleITK as sitk
    import radiomics
    from radiomics import getFeatureClasses

    record = settings_record(case, profile, "pyradiomics")
    plane = image.ndim == 2
    spacing = case["spacing"]
    sitk_image = sitk.GetImageFromArray(image[None] if plane else image)
    sitk_mask = sitk.GetImageFromArray(mask[None] if plane else mask)
    sitk_image.SetSpacing(tuple(reversed([1.0, *spacing] if plane else spacing)))
    sitk_mask.CopyInformation(sitk_image)
    axis = record["force2d_axis"]
    voxel = mode == "voxel"
    options = {
        "binWidth": record["bin_width"],
        "label": record["label"],
        "distances": record["distances"],
        "symmetricalGLCM": record["symmetrical_glcm"],
        "force2D": axis is not None,
        "force2Ddimension": 0 if axis is None else axis + int(plane),
        "maskedKernel": record["masked_kernel"],
        "kernelRadius": record["kernel_radius"],
        "voxelArrayShift": record["voxel_array_shift"],
        "voxelBased": voxel,
        "voxelBatch": 32,
        "initValue": 0,
    }
    if record["bin_count"] is not None:
        options["binCount"] = record["bin_count"]
    features = {}
    unavailable = {}
    for family, cls in sorted(getFeatureClasses().items()):
        if family in ("shape", "shape2D") and (voxel or (family == "shape2D") != plane):
            continue
        names = [
            n
            for n, deprecated in cls.getFeatureNames().items()
            if not deprecated and f"{family}_{n}" in catalog
        ]
        if not names:
            continue
        class_options = (
            dict(options, force2D=True, force2Ddimension=0) if family == "shape2D" else options
        )
        extractor = cls(sitk_image, sitk_mask, **class_options)
        for name in names:
            extractor.enableFeatureByName(name)
        values, failed = pyradiomics_values(extractor, family, voxel, sitk_image.GetSize()[::-1])
        features.update(values)
        unavailable.update(failed)
    return {
        "metadata": metadata(
            "python validation/golden.py --engine pyradiomics",
            engine="pyradiomics",
            version=radiomics.__version__,
            simpleitk=version("SimpleITK"),
        ),
        "case": case["case"],
        "shape": image.shape,
        "mode": mode,
        "settings": record,
        "unavailable": unavailable,
        "features": features,
    }


def external_output():
    value = os.environ.get("PARITY_EXTERNAL_OUT")
    if not value:
        raise ValueError("--external requires PARITY_EXTERNAL_OUT outside any Git worktree")
    path = Path(value).expanduser().resolve()
    ancestor = next(p for p in (path, *path.parents) if p.exists())
    check = subprocess.run(
        ["git", "-C", ancestor, "rev-parse", "--git-dir"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    if path == ROOT or ROOT in path.parents or check.returncode == 0:
        raise ValueError("--external output must be outside any Git worktree")
    return path


def inputs(external, engine, selected_case):
    if external:
        data = Path(os.environ["MLXRAD_REPO"]) / "data"
        phantom = data / "ibsi/ct_radiomics_phantom/nifti"
        lung = data / "nsclc-radiomics/LUNG1-001"
        for name, ipath, mpath in (
            ("ibsi_ct", phantom / "image/phantom.nii.gz", phantom / "mask/mask.nii.gz"),
            ("lung1_001", lung / "image.nrrd", lung / "mask.nrrd"),
        ):
            if selected_case and selected_case != name:
                continue
            image, mask, spacing, _, _ = read_pair(ipath, mpath)
            yield {"case": name, "shape": image.shape, "spacing": spacing, "label": 1}, image, mask
    else:
        paths = (
            sorted((DATA / "cases").glob("*.json"))
            if engine == "mlx"
            else [DATA / "cases" / f"npz_{name}.json" for name in NPZ_CASES]
        )
        if not paths:
            raise ValueError("run validation/fixtures.py first")
        for path in paths:
            if selected_case and selected_case != path.stem:
                continue
            case = json.loads(path.read_text())
            shape = case["shape"]
            yield (
                case,
                np.array(case["image"], dtype=np.float64).reshape(shape),
                np.array(case["mask"], dtype=np.uint32).reshape(shape),
            )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", choices=("mlx", "pyradiomics"), default="mlx")
    parser.add_argument("--external", action="store_true")
    parser.add_argument("--case", help="generate only this case")
    args = parser.parse_args()
    if args.external and args.engine != "mlx":
        parser.error("--external uses the mlx engine")
    directory = external_output() if args.external else DATA / "golden"
    if args.engine == "mlx":
        mlx, settings_for = mlx_source()
    else:
        # Use the MLX golden's catalog even though the two libraries live in separate venvs.
        logging.getLogger("radiomics").setLevel(logging.ERROR)
    found = False
    for case, image, mask in inputs(args.external, args.engine, args.case):
        name = case["case"]
        found = True
        for profile in PROFILES:
            for mode in ("roi",) if args.external else ("roi", "voxel"):
                conventions = ("ibsi", "pyradiomics") if args.engine == "mlx" else ("pyradiomics",)
                for convention in conventions:
                    stem = f"{name}.{profile}.{convention}.{mode}"
                    path = directory / f"{stem}.{args.engine}.json"
                    if (
                        args.engine == "pyradiomics"
                        and mode == "voxel"
                        and np.count_nonzero(mask == case["label"]) == 1
                    ):
                        continue
                    print(path.name, flush=True)
                    if args.engine == "mlx":
                        result = mlx_golden(
                            mlx,
                            settings_for,
                            image,
                            mask,
                            case,
                            profile,
                            mode,
                            convention,
                            args.external,
                        )
                    else:
                        catalog = json.loads((directory / f"{stem}.mlx.json").read_text())[
                            "features"
                        ]
                        result = pyradiomics_golden(image, mask, case, profile, mode, catalog)
                    write_json(path, result)
    if not found:
        parser.error(f"unknown case {args.case!r}")


if __name__ == "__main__":
    main()
