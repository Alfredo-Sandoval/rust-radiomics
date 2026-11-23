# Limits

## Input requirements

**Modes.** Shape and local-intensity features are ROI-only. Local intensity and
intensity-dependent morphology need 3D input. `shape2D` on 3D input needs `force2d_axis`
(a NumPy axis) and an ROI in one slice on that axis. Voxel mode needs at least two ROI
voxels.

**Filters.** The default recursive LoG needs 3D input with at least four samples per axis
and `sigma / spacing` of at most 200. LoG requires explicit `sigma` values. LBP3D needs 3D
input, and in voxel mode a masked kernel. It accepts 8- and 16-bit integer images and
floating-point images, refuses wider integer types, and raises an error when a
floating-point source makes a threshold comparison ambiguous. LBP2D refuses sources whose sign
cannot be resolved from three float32 words.

**Files.** Inputs are NIfTI and NRRD. Read voxel maps with NiBabel, because SimpleITK reads
stored NaN voxels as zero. NIfTI/Analyze header pairs named `.nii` or `.nii.gz` are
rejected. When an input has detached data, such as `.nhdr`, existing outputs are never
replaced, even with `overwrite=True`. Use a new output directory.

**Platform.** The Python package supports Python 3.13, 3.14 and 3.15. The release wheels
and performance measurements use Apple silicon macOS.

## Numerics

Feature arithmetic is float64 on the CPU, except that first-order statistics and the
intensity peaks are returned as float32 values, as in mlx-radiomics. Voxel maps hold the
float32 samples that the engine delivers, so float64 maps do not carry extra precision. Undefined features are NaN,
and overflow gives infinity. [Technical notes](../technical.md) give the agreement
tolerances against mlx-radiomics.

## Differences from mlx-radiomics

This port mirrors mlx-radiomics v1.1.0: feature names and order, `Settings` fields and
defaults, parameter-file keys, derived-image names, report schema 2 and error classes. These
differences are deliberate.

- **Precision.** mlx-radiomics computes features in float32 on Metal. This port computes in
  float64 on the CPU and returns first-order statistics and the intensity peaks as float32. Values agree with mlx-radiomics within `rtol=2e-5` and `atol=2e-6` for
  intensity and texture features, and `rtol=3e-6` and `atol=2e-6` for shape features. The
  tolerances are measured constants and are not loosened to pass a test. Where the float32
  result sits on a decision boundary, the two implementations can differ by more. Two
  synthetic LBP3D cases select a different local peak, because the degree-one band has two
  competing maxima one float32 step apart. Rust matches the CPU harmonic reference of
  mlx-radiomics and not its Metal result on those cases.
- **Simoncelli and Riesz are enabled.** mlx-radiomics 1.1.0 disables both because the
  batched Metal FFT in its pinned MLX release can write past its output buffer. This port
  computes them with a CPU FFT. All 16 image types run.
- **No MedMLX runner.** `mlxradiomics.medmlx.run` is not ported, and neither is the
  multi-case `run_study` that sits with it. Use the four extraction functions.
- **No figures.** The `viz` module is not ported. Voxel maps are NIfTI files that any
  viewer can render.
- **Export limit counting.** `max_scratch_mib` limits the memory an export holds. This port
  counts 4 bytes per selected row for shared offsets, 4 bytes per row for each float32 map
  column, and a fixed 459,104-byte writer allowance. mlx-radiomics counts its disk spool
  plus one float32 grid. A given limit can therefore admit slightly different exports in
  the two implementations.
- **Memory errors.** Allocation refusals raise Python `MemoryError`.
- **Sink errors.** If a context manager suppresses an error raised during a batch, the
  original error propagates. On the same inputs mlx-radiomics 1.1.0 raised
  `UnboundLocalError` for the first image and reused the previous image's validity counts
  for the second.
- **`UserWarning` on arrays.** Array `export` records a zero origin and identity direction.
  The Python binding emits the warning, and `allow_synthetic_geometry=True` acknowledges it.
  The Rust `export` does not warn.

## Not a medical device

This software is for research and education. It is not a medical device, and its outputs
require human review. Feature values depend on discretization, resampling and the
conventions selected in `Settings`; compare studies only when these match.

## Reporting differences

If a result differs from mlx-radiomics 1.1.0 by more than the tolerances above, include
the input, settings and both outputs in your bug report.
