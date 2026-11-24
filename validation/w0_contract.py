"""Export contract cases using the v1.1.0 source selected by PYTHONPATH.

Run with the mlx-radiomics Python environment. No extraction or image data is required.
The parameterized rejection cases come directly from the same checkout's test_parameters.py.
"""

import ast
import json
from dataclasses import asdict
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import NotRequired, get_origin, get_type_hints

import mlxradiomics
import numpy as np
import yaml
from mlxradiomics.config.parameters import load_params, parse_params
from mlxradiomics.contracts.json import json_safe
from mlxradiomics.contracts.metadata import transform_payload
from mlxradiomics.contracts.types import ExecutionMetadata, RuntimeMetadata
from mlxradiomics.contracts.validity import empty_validity, accumulate_validity


def upstream_rejections():
    root = Path(mlxradiomics.__file__).resolve().parents[2]
    tests = ast.parse((root / "tests/test_parameters.py").read_text())
    selected = {
        "test_parameter_files_reject_unsupported_controls",
        "test_parameter_parsers_reject_non_string_keys",
        "test_filter_parameters_refuse_unsupported_scientific_controls",
    }
    functions = [node for node in tests.body if getattr(node, "name", "") in selected]
    assert len(functions) == len(selected)
    for test in functions:
        rows = ast.literal_eval(test.decorator_list[0].args[1])
        for i, row in enumerate(rows):
            if test.name.startswith("test_filter"):
                kinds, ndim, _ = row
                params = {"imageType": kinds}
            elif test.name.startswith("test_parameter_parsers"):
                params, _ = row
                ndim = 3
            else:
                _, params = row
                ndim = 3
            yield f"{test.name}/{i}", params, ndim


def extra_rejections():
    for key, values in {
        "force2Ddimension": [True, 1.0, "1", -1, 3],
        "binWidth": [True, "10", 0, float("inf")],
        "binCount": [True, 0, -1, 2**31],
        "label": [False, "1", 1.5, 0], "gldm_a": [-1],
        "normalize": [0], "normalizeScale": [0], "removeOutliers": [1],
        "resegmentRange": [[], [True], [1, 2, 3], [float("nan")]],
        "resegmentMode": ["other"], "resegmentShape": [1],
        "distances": [0, [], [1, 1], [0], [True], [1.0]],
        "symmetricalGLCM": [1], "maskedKernel": ["yes"], "kernelRadius": [0],
        "voxelArrayShift": [float("nan")], "sliceAxis": [3], "neighborhoodRadius": [0],
        "ivhBinCount": [0], "ivhBinWidth": [-1],
        "ivhIntensityRange": [[1], [2, 1], [0, float("inf")]],
        "binAnchor": [float("inf")], "resegmentOutlierSigma": [-1, 2],
        "covarianceDivisor": [1, "other"], "ellipsoidArea": ["other"], "kurtosis": ["other"],
        "zeroVarianceMoments": ["other"], "ngtdmSingularCases": ["other"],
        "neighborhoodSupport": ["other"], "binOrigin": ["other"], "constantRoiBin": ["other"],
        "maskExtrapolation": ["other"], "splineBoundary": ["other"],
        "glcmAggregation": ["3d"], "gldmAggregation": ["3d_merge"],
        "lbp2DSamples": [0, 25], "lbp3DLevels": [0], "lbp3DIcosphereSubdivision": [5],
        "lbp2DRadius": [True, "1", None, 0, -1, float("nan"), float("inf")],
        "lbp3DIcosphereRadius": [True, "1", None, 0, -1, float("nan"), float("inf")],
        "padDistance": [-1], "resampledPixelSpacing": [[0, 1, -1]], "correctMask": ["false"],
    }.items():
        for i, value in enumerate(values):
            setting = {key: value}
            if key == "force2Ddimension":
                setting["force2D"] = True
            yield f"setting/{key}/{i}", {"setting": setting}, 3

    for kind, options in [
        ("Wavelet", {"level": 0}), ("Wavelet", {"start_level": -1}),
        ("Wavelet", {"wavelet": 1}), ("Wavelet", {"other": 0}),
        ("LoG", {"sigma": [0]}), ("LoG", {"sigma": [float("inf")]}),
        ("LoG", {"sigma": [1, 1]}), ("LoG", {"sigma": [1], "kernel": "other"}),
        ("LoG", {"sigma": [1], "constant": float("inf")}),
        ("LoG", {"sigma": [1], "padding": 1}), ("LoG", {"sigma": [1], "cutoff": 0}),
        ("Mean", {"constant": float("inf")}), ("Mean", {"support": -3}),
        ("Laws", {"kernel": "L3XX"}), ("Laws", {"energy_distance": -1}),
        ("Laws", {"pooling": "other"}), ("Laws", {"kernel": 123}),
        ("Gabor", {}), ("Gabor", {"gamma": 0}), ("Gabor", {"wavelength": [0]}),
        ("Gabor", {"theta": [float("inf")]}), ("Gabor", {"slice_axis": 3}),
        ("Gabor", {"sigma": [1, 2], "wavelength": [1, 2, 3], "theta": [0]}),
        ("Gabor", {"sigma": [1], "wavelength": [2], "theta": [0.0, -0.0]}),
        ("Gabor", {"sigma": [1, 1], "wavelength": [2], "theta": [0]}),
        ("Simoncelli", {"levels": [1, 1]}), ("Simoncelli", {"levels": [0]}),
        ("Riesz", {"orders": [[1, 0, 0], [1, 0, 0]]}),
        ("Riesz", {"log_sigma": [0]}), ("Riesz", {"log_cutoff": 0}),
        ("Riesz", {"simoncelli_levels": [0]}), ("Riesz", {"simoncelli_levels": [2, 2]}),
        ("Riesz", {"tensor_sigma": 0}), ("Riesz", {"steering": True, "tensor_sigma": 1,
                                                   "orders": [[16, 1, 0]]}),
        ("SeparableWavelet", {"bands": ["XL"]}), ("SeparableWavelet", {"bands": [1]}),
        ("SeparableWavelet", {"bands": ["LL", "LL"]}), ("SeparableWavelet", {"level": 0}),
    ]:
        yield f"{kind}/{options}", {"imageType": {kind: options}}, 3

    for i, value in enumerate([{}, {"glcm": None}, {"glcm": "Contrast"}, {"glcm": [1]},
                               {"glcm": ["Contrast", "Contrast"]},
                               {"bad": ["Zulu", "Alpha", "a'b", 'a"b']}]):
        yield f"featureClass/{i}", {"featureClass": value}, 3
    for i, params in enumerate([
        {"imageType": {}}, {"imageType": []}, {"imageType": {"Square": 1}},
        {"imageType": {"NonseparableWavelet": None, "Simoncelli": {}}},
        {"setting": {"binCount": 2, "binAnchor": 0}},
        {"setting": {"ivhBinCount": 1, "ivhBinWidth": 1}},
        {"setting": {"resegmentMode": "sigma", "resegmentRange": [1], "resegmentOutlierSigma": 1}},
        {"setting": {"force2D": True, "glcmAggregation": "3d_merge"}},
    ]):
        yield f"combined/{i}", params, 3
    yield "dimension", {}, 1
    yield "lbp dimension", {"imageType": {"LBP3D": None}}, 2
    yield "planar dimension", {"imageType": {"SeparableWavelet": {"bands": ["HL"], "planar": True}}}, 2


def accepted():
    settings = {
        "binWidth": 2.5, "label": 2**40, "distances": [2, 1], "symmetricalGLCM": False,
        "force2D": True, "force2Ddimension": 2, "maskedKernel": False, "kernelRadius": 3,
        "voxelArrayShift": -15.5, "gldm_a": 3, "normalize": True, "normalizeScale": 2,
        "removeOutliers": 3, "resegmentRange": [-2, 5], "resegmentMode": "absolute",
        "resegmentShape": True, "resegmentOutlierSigma": 2, "binAnchor": -50, "sliceAxis": 1,
        "neighborhoodRadius": 2, "glcmAggregation": "2d_slice_merge",
        "glrlmAggregation": "2.5d_direction_merge", "glszmAggregation": "2.5d_merge",
        "gldzmAggregation": "2d_average", "ngtdmAggregation": "2.5d_merge",
        "gldmAggregation": "2d_average", "ivhBinWidth": 4, "ivhIntensityRange": [-5, 5],
        "covarianceDivisor": "n", "ellipsoidArea": "exact", "kurtosis": "raw",
        "zeroVarianceMoments": "nan", "ngtdmSingularCases": "unadjusted",
        "neighborhoodSupport": "distances", "binOrigin": "zero-lattice",
        "constantRoiBin": "middle", "maskExtrapolation": "nearest", "splineBoundary": "mirror",
        "resampledPixelSpacing": [0, 2, 3.5], "interpolator": "sitkNearestNeighbor",
        "correctMask": True, "padDistance": 0, "lbp2DRadius": 3, "lbp2DSamples": 12,
        "lbp2DMethod": "ror", "lbp3DLevels": 3, "lbp3DIcosphereRadius": 2,
        "lbp3DIcosphereSubdivision": 2, "gradientUseSpacing": False,
    }
    kinds = {
        "Square": None, "Original": {}, "SquareRoot": {}, "Exponential": {}, "Logarithm": {},
        "LBP2D": {}, "LBP3D": {}, "Gradient": {},
        "LoG": {"sigma": [1, 1.5], "kernel": "sampled", "cutoff": 2, "padding": "nearest",
                "constant": 1, "planar": True},
        "Wavelet": {"wavelet": "db3", "level": 3, "start_level": 1},
        "Mean": {"support": 5, "padding": "constant", "constant": 3, "planar": True},
        "Laws": {"kernel": "E3L3S3", "padding": "nearest", "constant": -1, "pooling": "max",
                 "energy_distance": 3},
        "Gabor": {"sigma": [1], "wavelength": [2, 3], "theta": [0, 2], "gamma": 2,
                  "response": "imaginary", "pooling": "average", "slice_axis": 1,
                  "padding": "constant", "constant": -2},
        "Riesz": {"base": "log", "orders": [[1, 0, 0], [0, 2, 0]], "log_sigma": [3],
                  "log_cutoff": 3.5, "simoncelli_levels": [2, 3], "padding": "constant",
                  "constant": 1, "grid": "centered", "steering": True, "tensor_sigma": 2},
        "NonseparableWavelet": {"levels": [2], "padding": "periodic", "grid": "centered",
                                "planar": True},
        "SeparableWavelet": {"wavelet": "db2", "bands": ["LH", "LL"], "level": 3,
                             "padding": "periodic", "constant": 2, "pooling": "min", "planar": True},
    }
    yield "defaults", {}, 3
    yield "all overrides", {"imageType": kinds, "setting": settings,
                            "featureClass": {"glcm": ["Contrast"], "firstorder": ["Mean", "Range"]}}, 3
    yield "unused bin width", {"setting": {"binCount": 2, "binWidth": float("nan"),
                                          "ivhBinCount": 3, "ivhIntensityRange": [3, 3]}}, 2
    yield "unselected 3d family", {"featureClass": {"firstorder": ["Mean"]},
                                   "setting": {"glcmAggregation": "3d_merge"}}, 2
    yield "null overrides", {"imageType": {"Laws": {"energy_distance": None}, "Gabor": {
        "sigma": [1, 2], "wavelength": [2], "theta": [0], "slice_axis": None}}}, 3
    # The three test_input_domains.py requests stop here before image execution.
    for width, anchor, bounds, features, aggregation in [
        (10, -200, [50, 100], {"firstorder": ["Mean", "Variance", "Minimum", "Maximum"],
                               "shape": ["VoxelVolume"]}, "native"),
        (0.25, 0, [0], {"firstorder": ["Mean"],
                        "shape": ["IntegratedIntensity", "CentreOfMassShift"]}, "native"),
        (0.05, 0, [0], {"intensityhistogram": ["Minimum", "Maximum", "Mean", "Variance"],
                        "glcm": ["JointAverage", "Contrast", "ClusterShade", "Idmn"]}, "3d_merge"),
    ]:
        yield f"input domains/{width}", {
            "setting": {"binWidth": width, "binAnchor": anchor, "resegmentRange": bounds,
                        "glcmAggregation": aggregation}, "featureClass": features,
        }, 3


def file_cases():
    for name, params, ndim in (*upstream_rejections(), *extra_rejections(), *accepted()):
        yield name, yaml.safe_dump(params, sort_keys=False), ndim
    yield "duplicate family order", (
        "featureClass: {glcm: [MCC], firstorder: [Mean], 'glcm': [Contrast], glcm: [Id], ivh: [AreaUnderIVHCurve]}"
    ), 3
    yield "duplicate image order", "imageType: {Square: {}, Original: {}, Square: null}", 3
    selected = "featureClass: {firstorder: [Mean]}\n"
    for text in [
        "setting: {normalize: ON, maskedKernel: no, covarianceDivisor: n, label: 021}",
        "setting: {normalize: 'ON'}",
        "setting: {label: 0x1F_FF, kernelRadius: 0b10, normalizeScale: 1:00.5}",
        "setting: {label: 1:01, voxelArrayShift: -0.5e+2}",
        "setting: {label: 08}", "setting: {label: 0o10}",
        "setting: {binWidth: 1e-3}", "setting: {binWidth: .25}",
        "setting: {label: 2, label: 3}",
        "setting: {normalize: !!bool 'yes', normalizeScale: !!float 1e2}",
        "imageType: {Mean: &m {padding: nearest, constant: 2}, Laws: {<<: *m, constant: 3}}",
        "setting: {!!str '<<': 3}",
        "imageType: {Wavelet: {wavelet: 2025-01-01}}",
        "imageType: {Wavelet: {wavelet: !!binary aGFhcg==}}",
        "setting: {2025-01-01: true}",
    ]:
        yield text, selected + text, 3


def main():
    rows = []
    with TemporaryDirectory() as tmp:
        path = Path(tmp) / "params.yaml"
        for name, text, ndim in file_cases():
            row = {"case": name, "yaml": text, "ndim": ndim}
            path.write_text(text, encoding="utf-8")
            try:
                parsed = parse_params(load_params(path), ndim=ndim)
            except ValueError as error:
                row["error"] = str(error)
            else:
                row["settings"] = json_safe(asdict(parsed.settings))
                row["image_settings"] = json_safe(asdict(parsed.image_settings))
                row["transforms"] = json_safe(transform_payload(parsed.transforms))
            rows.append(row)

    target = Path(__file__).resolve().parents[1] / "crates/radiomics/tests/data/w0"
    target.mkdir(parents=True, exist_ok=True)
    (target / "parameters.json").write_text(json.dumps(rows, indent=2, allow_nan=False) + "\n")
    keys = get_type_hints(ExecutionMetadata, include_extras=True)
    counts = empty_validity(5)
    for dtype in (np.float64, np.float32):
        accumulate_validity(counts, np.array([0., -0., np.nan, -np.inf, np.inf, 2.5], dtype=dtype))
    contract = {
        "required": sorted(k for k, v in keys.items() if get_origin(v) is not NotRequired),
        "optional": sorted(k for k, v in keys.items() if get_origin(v) is NotRequired),
        "runtime": sorted(get_type_hints(RuntimeMetadata)),
        "counts": counts,
        "nested": json_safe({2: [(float("nan"), -float("inf"), float("inf"), None)], 1: [False]}),
    }
    (target / "contracts.json").write_text(json.dumps(contract, indent=2, allow_nan=False) + "\n")
    print(f"{len(rows)} cases, {sum('error' in row for row in rows)} expected refusals")


if __name__ == "__main__":
    main()
