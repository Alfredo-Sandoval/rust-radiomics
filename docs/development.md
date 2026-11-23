# Development

## Checks

Run the checks from the repository root with the make targets:

| Target | Runs |
| --- | --- |
| `make env` | Creates `crates/radiomics-py/.venv` from the lock file and builds the extension into it in release mode |
| `make check-env` | Verifies that the environment exists and imports `rustrad` |
| `make lint` | `cargo fmt --check`, Clippy with warnings denied, and Ruff |
| `make typecheck` | Pyright on the Python package, and a Ruff pass that bans `Any` |
| `make test` | The Rust test suite in release mode, then the Python tests |
| `make qa` | `make lint` and `make typecheck` (the default target) |
| `make ibsi` | The opt-in IBSI 1 and IBSI 2 suites and the parity suites |
| `make build` | An sdist and one release wheel for each of Python 3.13, 3.14 and 3.15, written to `dist/` |
| `make clean` | Removes build and cache output |
| `make loc` | Counts lines of source |

Every target except `env` and `clean` needs the environment from `make env`.

`make build` finds an interpreter for each wheel with `uv python find`. Run
`uv python install 3.13 3.14 3.15` once before the first build. Set `WHEEL_PYTHONS` to build
for other versions.

`make ibsi` reads its data locations from environment variables and stops with a message
if one of the first four is unset. The IBSI 1 CT phantom and the IBSI 2 reference data are
not distributed with the repository.

| Variable | Points to |
| --- | --- |
| `IBSI1_CT_PHANTOM` | The CT phantom NIfTI directory, with `image/phantom.nii.gz` and `mask/mask.nii.gz` |
| `IBSI1_REFERENCE_DIR` | The directory with `config A.csv` to `config E.csv` |
| `IBSI2_REFERENCE_DIR` | The IBSI 2 reference data, with the response maps |
| `IBSI2_PHANTOM_DIR` | The IBSI 2 phase 1 digital phantom NIfTI directory |
| `W1_IBSI_CT_PHANTOM`, `W1_IBSI_CT_GOLDENS` | Optional. The CT phantom directory and the golden file from `validation/w1_ct_phantom_goldens.py`, for the CT interpolation check |
| `IBSI2_MAPS` | Optional. Limits the IBSI 2 map checks to the named maps |

The CT phantom and the LUNG1 images are CC BY-NC data. Keep them outside the repository.

Without make, the Rust checks are:

```bash
cargo +1.96.0 fmt --all --check
cargo +1.96.0 clippy --locked --all-targets
cargo +1.96.0 test --locked --release
```

and the Python tests run with `python -m pytest` in `crates/radiomics-py` once the package
is installed in the environment.

## Reference fixtures

The committed parity corpus is under `crates/radiomics/tests/data/`. It holds IBSI digital
phantom inputs (CC BY 4.0), synthetic cases, and the goldens that mlx-radiomics v1.1.0 and
PyRadiomics produced for them. Rust tests read the corpus without Python, a GPU or any
external dataset.

The scripts in `validation/` regenerate it. They run on an Apple silicon Mac with the
mlx-radiomics v1.1.0 checkout on `PYTHONPATH`, `MLX_ENABLE_TF32=0` and
`PYTHONDONTWRITEBYTECODE=1`. The mlx-radiomics environment needs pytest and SimpleITK, and
the PyRadiomics environment needs NumPy, SimpleITK and PyRadiomics.

| Script | Regenerates |
| --- | --- |
| `fixtures.py` | The redistributable inputs and the IBSI 1 digital-phantom table |
| `golden.py` | mlx-radiomics goldens, and PyRadiomics goldens with `--engine pyradiomics`; `--case NAME` selects one case |
| `export_catalog.py` | The Rust feature catalog (`catalog.rs`) from an installed mlx-radiomics |
| `w0_contract.py` | Test cases for settings, parameter files and results |
| `w1_io_goldens.py`, `w1_ct_phantom_goldens.py` | Reader goldens, and the CT phantom grid and gray-level goldens |
| `w4_texture.py`, `w5_goldens.py` | Texture fixtures and the IBSI digital-phantom morphology fixture |
| `w7_voxel.py`, `w8_goldens.py` | Voxel-map and end-to-end file goldens |
| `compare.py` | Compares a schema-2 report with a golden or an IBSI reference table |

Every JSON record stores the command and version metadata that produced it. See
`validation/README.md` for the arguments of each script and for the comparison rules.

## Upstream

The reference tests use [mlx-radiomics](https://github.com/MedMLX/mlx-radiomics) v1.1.0,
commit `b58783c`. When updating the reference version, regenerate the expected results
from the new tag.

The public Rust API is the crate root and the `io`, `params`,
`settings` and `metadata` modules. The hidden `__private` module carries the few items the
Python binding needs and may change in any release.

The integration tests in `crates/radiomics/tests/` use only the public API. A check of an
internal detail lives in a `#[cfg(test)]` module beside the code it covers.
