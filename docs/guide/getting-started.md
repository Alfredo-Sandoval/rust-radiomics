# Getting started

## Install

```sh
pip install rustradiomics
```

Needs Python 3.13, 3.14 or 3.15. PyPI has wheels for Apple silicon Macs. On other
systems pip builds the extension from source, which needs a Rust toolchain.

To install from a checkout instead, run this at the repository root:

```sh
python -m pip install ./crates/radiomics-py
```

Check the version and device that Python loaded:

```sh
python -c "import rustradiomics as r; print(r.__version__, r.__device__)"
```

The output is `1.1.0 cpu`. For contributor setup and wheel builds, see
[Development](../development.md).

## Extract ROI features

From the repository root, run the example below with the included IBSI digital phantom.
For your own data, replace the two paths with an aligned NIfTI or NRRD image and
integer label mask. By default, `extract_files` reads the spacing from the image
and measures the 93 default features over voxels whose mask label is 1.

```python
from pathlib import Path
from rustradiomics import extract_files

data = Path("crates/radiomics/tests/data/w1/io")
image_path = data / "ibsi_digital_phantom_image.nii.gz"
mask_path = data / "ibsi_digital_phantom_mask.nii.gz"
result = extract_files(image_path, mask_path)

print(len(result.features))  # 93
print(round(result.features["original_firstorder_Mean"], 3))  # 2.149
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
one map per feature. `export_files` writes the maps to disk as float32 NIfTI files. To
export a local mean map for the phantom above, reuse `image_path` and `mask_path` and
choose an empty output directory.

```python
from rustradiomics import Settings, export_files

saved = export_files(image_path, mask_path, "outputs/maps",
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
