# Getting started

## Requirements

- Python 3.13, 3.14 or 3.15, and NumPy 2.1 or later.
- For source builds, a Rust toolchain. The repository is tested with Rust 1.96.0.
- For development from a checkout, `uv` installs the build tools, including `maturin`.

The package runs on the CPU in float64 and uses all available cores through Rayon.

## Install

```bash
pip install rustradiomics
```

PyPI has wheels for Apple silicon Macs with Python 3.13, 3.14 and 3.15. On other systems
pip builds the extension from source, which needs a Rust toolchain.

To build the extension from a checkout of this release, run the following from the
repository root.

```bash
uv venv --python 3.13
source .venv/bin/activate
RUSTUP_TOOLCHAIN=1.96.0 uv pip install ./crates/radiomics-py
```

For development, `make env` creates `crates/radiomics-py/.venv` and builds the extension
into it. After that, `make build` writes an sdist and one wheel for each supported Python
version to `dist/`, and `uv pip install dist/rustradiomics-*-cp313-*.whl` installs the Python 3.13
wheel into another environment.

Check the install:

```bash
python -c "import rustradiomics; print(rustradiomics.__version__, rustradiomics.__device__)"
```

The device is always `cpu`.

## Extract ROI features

`extract_files` reads a NIfTI or NRRD image and an integer label mask and returns one
value per feature for the selected label:

```python
from rustradiomics import Settings, extract_files

settings = Settings(label=1)  # the 93 default PyRadiomics intensity and texture features
result = extract_files("image.nii.gz", "mask.nii.gz", settings=settings)

result.features["original_firstorder_Mean"]   # float
result.metadata["feature_validity"]           # finite / NaN / infinity counts per feature
```

Feature keys are `<image>_<family>_<name>`, such as `original_glcm_Contrast` or
`wavelet-HHL_glcm_Contrast`. `rustradiomics.FEATURES` lists all 200 selectable names. See
[Features](features.md).

## Arrays instead of files

`extract` takes aligned NumPy arrays. Spacing is in millimeters and in NumPy axis order:

```python
import numpy as np
from rustradiomics import Settings, extract

image = np.random.default_rng(0).normal(size=(16, 32, 32)).astype(np.float32)
mask = np.zeros(image.shape, dtype=np.uint8)
mask[4:12, 8:24, 8:24] = 1

result = extract(image, mask, spacing=(3.0, 1.0, 1.0),
                 settings=Settings(features=("firstorder_Mean", "glcm_Contrast")))
```

## Voxel maps

`mode="voxel"` computes each feature in a neighborhood around every ROI voxel and returns
one map per feature. `export_files` writes the maps to disk as float32 NIfTI files without
holding them all in memory:

```python
from rustradiomics import Settings, export_files

saved = export_files("image.nii.gz", "mask.nii.gz", "outputs/maps",
                     settings=Settings(features=("firstorder_Mean",)))
saved.maps["original_firstorder_Mean"]  # pathlib.Path to the NIfTI map
```

## Threads

Extraction uses Rayon worker threads. Set `RAYON_NUM_THREADS` before starting Python to
limit them. Values were bit-identical across the thread counts that the test suite compares;
see [Technical notes](../technical.md#threading).

## Next steps

- [Python API](python-api.md): the four core operations, derived images and streaming.
- [Rust API](rust-api.md): the same workflows from Rust.
- [Parameters](parameters.md): PyRadiomics YAML files, `Settings` fields and image types.
- [Features](features.md): the 200 feature names with IBSI identifiers.
- [Limits](limits.md): input requirements and differences from mlx-radiomics.
