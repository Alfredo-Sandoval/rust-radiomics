# Release notes

## 1.1.0

**New package name.** The Python package is now `rustradiomics`, imported with
`import rustradiomics`. Version 1.0.0 was published as `rustrad`. The functions, settings
and results are unchanged.

**Moment invariants.** The new `momentinvariant` family adds ten 3D ROI features: five
moments of the ROI shape and five of its intensity that do not change under translation,
rotation or uniform scaling, as defined in PySERA. The catalog now lists 200 names.

## 1.0.0 (2026-11-23)

`rustrad` measures the intensity, texture and shape of a region in a medical image, from
Python or Rust, on the CPU. It reads NIfTI and NRRD files or NumPy arrays, accepts
PyRadiomics feature names and parameter files, and follows IBSI definitions by default.
Its results are validated against the IBSI 1 and IBSI 2 reference values.

The wheels are for Apple silicon Macs with Python 3.13, 3.14 or 3.15. Download the one for
your Python version from the
[release page](https://github.com/Alfredo-Sandoval/rust-radiomics/releases/tag/v1.0.0)
and install it. For Python 3.13:

```sh
python -m pip install rustrad-1.0.0-cp313-cp313-macosx_11_0_arm64.whl
```

[Quick start](https://github.com/Alfredo-Sandoval/rust-radiomics/blob/v1.0.0/README.md#run) · [Feature catalog](https://github.com/Alfredo-Sandoval/rust-radiomics/blob/v1.0.0/docs/guide/features.md) · [Validation](https://github.com/Alfredo-Sandoval/rust-radiomics/blob/v1.0.0/docs/technical.md)
