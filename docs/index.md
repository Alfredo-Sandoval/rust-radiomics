# rust radiomics

`rustradiomics` computes 200 radiomics features from images and segmentation masks on the CPU.
It supports PyRadiomics parameter files and uses IBSI conventions by default.

| Input | Output |
| :--- | :--- |
| Aligned image and integer label mask, as NumPy arrays or NIfTI/NRRD files | ROI features, retained or streamed voxel maps, or one float32 NIfTI map per feature with a JSON report |

The Python package is `rustradiomics`. The Rust crate is `radiomics`.

```python
from rustradiomics import Settings, extract_files

result = extract_files(
    "image.nii.gz", "mask.nii.gz",
    settings=Settings(features=("firstorder_Mean", "glcm_Contrast")),
)
result.features["original_glcm_Contrast"]
```

## Guide

- [Getting started](guide/getting-started.md) covers installation and a first extraction.
- [Python API](guide/python-api.md): the four operations, results, streaming and errors.
- [Rust API](guide/rust-api.md): the same workflows from the `radiomics` crate.
- [Parameters](guide/parameters.md): parameter files, `Settings` fields, spatial options and the 16 image types.
- [Features](guide/features.md): the 200 feature names with IBSI identifiers, units and modes.
- [Limits](guide/limits.md): input requirements and differences from mlx-radiomics.

## Reference

- [Technical notes](technical.md): precision, validation results and how values are checked.
- [Performance](performance.md): timings, the comparison with mlx-radiomics and how to reproduce them.
- [Development](development.md): checks, reference fixtures and the upstream revision.
- [Release notes](release-notes.md)
