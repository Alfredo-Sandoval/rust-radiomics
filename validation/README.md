# Reference validation

The committed corpus contains 12 image/mask pairs, 487 IBSI 1 digital-phantom reference rows, 96 MLX records and 26 PyRadiomics float64 records. The Rust catalog test reads them without Python, a GPU or an external dataset.

## Regenerate

Run from the Rust worktree root on an Apple silicon Mac. Set `MLXRAD_CHECKOUT` to the read-only mlx-radiomics v1.1.0 checkout at commit `b58783c`. Set `MLXRAD_REPO` to the mlx-radiomics repository containing the local IBSI files, the pinned fixture commit, and the Python environments. The MLX environment needs the v1.1.0 dependencies, pytest and SimpleITK. The PyRadiomics environment needs NumPy, SimpleITK and PyRadiomics. Each JSON record includes the generating command and version metadata.

```sh
export MLXRAD_CHECKOUT=/path/to/mlx-radiomics-v1.1.0
export MLXRAD_REPO=/path/to/mlx-radiomics
export PYTHONDONTWRITEBYTECODE=1 MLX_ENABLE_TF32=0
export PYTHONPATH="$MLXRAD_CHECKOUT/src"
mlx_python="$MLXRAD_REPO/.venv/bin/python"
pyr_python="$MLXRAD_REPO/work/lung1-benchmark/venv/bin/python"

"$mlx_python" validation/fixtures.py
"$mlx_python" validation/golden.py
"$pyr_python" validation/golden.py --engine pyradiomics
```

Both golden commands accept `--case NAME`, for example `--case npz_force2d`. The PyRadiomics generator uses the corresponding MLX record to select shared public feature names. The committed runs used MLX 0.32.4 and PyRadiomics 3.1.1.dev111+g8ed579383. For MLX the source version comes from the checkout, since the environment's installed distribution metadata can report a different version.

## Inputs and settings

`crates/radiomics/tests/data/reference/cases/` holds the IBSI 1 digital phantom, seven synthetic NPZ cases from mlx-radiomics and four additional synthetic cases. The latter cover 2-D and 3-D input, fractional and negative intensities, constant intensity, multiple labels, a singleton ROI and anisotropic spacing. Alex Zwanenburg designed the IBSI digital phantom, distributed under CC BY 4.0. The JSON retains the image, mask, spacing, origin, direction and attribution. [IBSI source and license](https://github.com/theibsi/data_sets/tree/master/ibsi_1_digital_phantom)

Images, integer labels and goldens' voxel maps are flat arrays in NumPy C order. Reshape with the record's `shape`, using `(z, y, x)` or `(y, x)`. Spacing follows those axes in millimetres. Origin and row-major direction follow ITK physical-axis order. The NPZ and synthetic inputs use zero origin and identity direction.

The NPZ conversion preserves the original settings, version metadata, feature values, maps and intermediate arrays under `recording`. Count tensors store their shape and nonzero flat offsets and values. All other array fields under `recording` preserve the NPZ dimensions. The NPZs were recorded with PyRadiomics `6a761c4` and retrieved from mlx-radiomics commit `7964f30` on `dev`. Its `negative_ct_multilabel` case is synthetic.

For each case, MLX uses `tests/conftest.py::settings_for` with both `ibsi` and `pyradiomics`. Each preset runs both profiles.

| Profile | Discretisation | IVH discretisation | Distances | Force 2-D axis |
| --- | --- | --- | --- | --- |
| `width25` | Width 25 | Width 25 | 1 | Recorded NPZ axis if present |
| `count32` | 32 bins | 32 bins | 1, 2 | Axis 0 for 3-D, recorded axis if present for 2-D |

Each profile uses the case label. NPZ cases also preserve `masked_kernel`, `kernel_radius`, `symmetrical_glcm` and `voxel_array_shift` from the recording. Other fields take their preset defaults. The profile and conventions apply to the fresh goldens. Historical NPZ values retain the settings in their own metadata.

`golden/<case>.<profile>.<convention>.<mode>.<engine>.json` stores one run. MLX selects all names from `list_features(ndim=..., mode=...)`, giving 156 ROI features in 2-D, 181 in 3-D and 147 voxel features. MLX rejects voxel mode for the two singleton ROIs. All eight such records contain the observed `expected_error`. Successful MLX records include the reference bin lattice. Nonfinite values use the strings `"NaN"`, `"Infinity"` and `"-Infinity"`.

PyRadiomics computes the shared, non-deprecated features under its own conventions. Its classes receive 2-D cases as single-slice volumes with 1 mm through-plane spacing. The shape2D class uses the in-plane axes. Singleton cases have ROI references. Seven PyRadiomics voxel runs report `glcm_MCC` under `unavailable` because its eigensolver failed for a batch. Those maps have no reference values. Each engine is a separate numerical target.

## Rust tests

```sh
~/.cargo/bin/cargo +1.96.0 test --test parity_catalog -- --nocapture
~/.cargo/bin/cargo +1.96.0 test --release --test parity_families -- --nocapture
```

The default test checks the case inventory, feature selection, dimensions and IBSI identifiers. `parity_families` runs the ROI goldens through the public `extract`, one test per family, and `w7_parity` runs the voxel goldens through `extract` with retained maps. Both compare against the MLX values at `rtol=2e-5` and `atol=2e-6` (`rtol=3e-6` for shape), and check the `expected_error` records. Four intensity-dependent morphology outputs use the `shape_` prefix.

## Compare a report

```sh
data_dir=crates/radiomics/tests/data/reference
python3 validation/compare.py /path/to/features.json \
    "$data_dir/golden/npz_force2d.width25.ibsi.roi.mlx.json"
"$mlx_python" validation/compare.py /path/to/voxel/features.json \
    "$data_dir/golden/npz_force2d.width25.ibsi.voxel.mlx.json"
"$mlx_python" -m pytest -q -p no:cacheprovider validation/test_compare.py
```

Schema-2 ROI reports contain `features` keyed `original_<family>_<Name>`. Voxel reports supply `maps` with absolute or report-relative NIfTI paths. The voxel reader uses NiBabel to preserve nonfinite values. The CLI prints maximum absolute and relative errors per feature, tolerance, failed sample counts and overall maxima. Every voxel, including the background, is compared. Missing and extra keys fail the comparison. Use `--allow-extra` to check a reference subset. Exit statuses are 0 for a match, 1 for numerical or feature-set differences, and 2 for invalid input.

MLX comparisons use `abs(actual - reference) <= atol + rtol * abs(reference)`. The intensity and texture limits are `rtol=2e-5` and `atol=2e-6`. Shape, shape2D and intensity-dependent morphology use `rtol=3e-6` and `atol=2e-6`. The CLI uses `rtol=1e-8` and `atol=1e-10` for the separate float64 PyRadiomics comparisons. NaNs match each other. Infinities must match in sign. A nonzero deviation from a zero reference has infinite relative error and is evaluated against the absolute tolerance.

### IBSI 1 reference rows

`ibsi_1_reference.json` preserves the CSV family (including aggregation), feature name, tag, value and absolute tolerance. It joins the row to the v1.1.0 definition catalog for its public key and permanent IBSI identifier. The generator retains rows without a catalog match with a null key. All 487 current rows map to 174 public keys. Five rows have no published numerical reference. [IBSI 1 reference table](https://theibsi.github.io/ibsi1/)

Choose the exact CSV family when comparing an IBSI report. The input report must use the published digital-phantom preprocessing and the chosen aggregation. The two golden profiles exercise the port's settings. The IBSI comparison uses the published absolute tolerance on the supplied values, including zero tolerances. Extra report keys and rows without reference values are excluded.

```sh
python3 validation/compare.py /path/to/ibsi/features.json "$data_dir/ibsi_1_reference.json" \
    --ibsi-family 'Co-occurrence matrix (3D, averaged)'
```

## External CT data

The IBSI 1 CT phantom and LUNG1-001 have noncommercial licenses. Their inputs remain in `$MLXRAD_REPO/data/ibsi/ct_radiomics_phantom/nifti/` and `$MLXRAD_REPO/data/nsclc-radiomics/LUNG1-001/`. The opt-in command writes ROI values for all 181 features in both profiles and both convention sets. It requires an output directory outside Git and writes only the result JSONs there.

```sh
export PARITY_EXTERNAL_OUT=/path/outside/the/repository
"$mlx_python" validation/golden.py --external
```
