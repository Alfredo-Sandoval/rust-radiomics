# Parameters

This page covers PyRadiomics-style parameter files, the `Settings` fields, the spatial
options for file input and the 16 image types.

## Parameter files

A parameter file is a YAML mapping with three supported sections: `setting`, `imageType`
and `featureClass`. Unsupported keys, image types and per-image overrides raise
`InvalidInputError` (Python) or `Error::InvalidInput` (Rust). They are rejected, not
ignored. Mapping keys must be strings at every level. Numeric settings require numbers,
and the YAML values `true`, `false`, `yes`, `no`, `on` and `off` are accepted only for
boolean settings.

```yaml
setting:
  binWidth: 25
  label: 1
  resampledPixelSpacing: [1.0, 1.0, 1.0]
  interpolator: sitkLinear
  padDistance: 1
imageType:
  Original: {}
  Wavelet:
    level: 1
featureClass:
  firstorder: [Mean, Entropy]
  glcm: [Contrast, JointEntropy]
```

`resampledPixelSpacing` is in ITK x-first order, as in PyRadiomics. The Python
`ImageSettings.resampled_spacing` and the Rust `ImageSettings` use NumPy axis order
instead. The parser reverses the file value.

The `featureClass` lists must name features explicitly. A family without a list is
rejected. Omitting `featureClass` selects the 93 default features.

### From Python

```python
from rustrad import extract_files
from rustrad.config.parameters import load_params, parse_params

parsed = parse_params(load_params("params.yaml"), ndim=3)
result = extract_files(
    "image.nii.gz", "mask.nii.gz",
    settings=parsed.settings,
    image_settings=parsed.image_settings,
    transforms=parsed.transforms,
)
```

`load_params` reads a UTF-8 YAML file into an ordered mapping, with anchors, merge keys
and plain-scalar typing. `parse_params(params, ndim=...)` validates all three sections for
a 2D or 3D image and returns an object with `settings`, `image_settings` and `transforms`.
`settings_from_params(params, base=None)` overlays only the feature settings on an
existing `Settings`.

### From Rust

```rust
use std::path::Path;

use radiomics::params::ExtractionParameters;
use radiomics::{Output, extract_files};

fn main() -> radiomics::Result<()> {
    let params = ExtractionParameters::from_file("params.yaml", 3)?;
    let result = extract_files(
        Path::new("image.nii.gz"),
        Path::new("mask.nii.gz"),
        &params.settings,
        &params.image_settings,
        &params.transforms,
        Output::Roi,
    )?;
    println!("{} features", result.features.len());
    Ok(())
}
```

`ExtractionParameters::from_file(path, ndim)` and `from_yaml(text, ndim)` parse and
validate all three sections. `radiomics::params` also has `load_params`, `parse_params` and
`settings_from_params` for the separate steps. See the [Rust API](rust-api.md) for the
other imports.

## Settings

`Settings` is a frozen dataclass in Python and a plain struct in Rust, with the same field
names and defaults. Defaults follow IBSI, even when PyRadiomics feature names or
parameter files are used. `Settings.validate(ndim)` checks the fields for an image with
the given number of axes. Extraction validates them again.

### Discretisation and preprocessing

| Field | Default | Controls |
| --- | --- | --- |
| `features` | the 93 defaults | Selected `<family>_<Name>` features |
| `label` | `1` | Mask value that selects the ROI |
| `bin_width` | `25.0` | Fixed bin width; ignored when `bin_count` is set |
| `bin_count` | `None` | Fixed bin count over the sampled range; overrides `bin_width` |
| `bin_anchor` | `None` | Explicit minimum gray-level anchor; bin 1 starts here |
| `max_gray_levels` | `None` | Cap on the gray-level lattice |
| `normalize` | `False` | Whole-image z-score before resegmentation |
| `normalize_scale` | `1.0` | Scale applied after normalization |
| `remove_outliers` | `None` | Clip normalized intensities to this many standard deviations |
| `resegment_range` | `None` | One or two inclusive resegmentation bounds |
| `resegment_mode` | `"absolute"` | How `resegment_range` is read: `absolute`, `relative` or `sigma` |
| `resegment_outlier_sigma` | `None` | Also drop voxels further than this many standard deviations from the mean |
| `resegment_shape` | `False` | Shape features use the resegmented mask |
| `voxel_array_shift` | `0.0` | Shift added for `Energy`, `TotalEnergy` and `RootMeanSquared` |

`bin_count` takes precedence over `bin_width`. Set only the discretizer you intend to use.

### Texture and neighborhood

| Field | Default | Controls |
| --- | --- | --- |
| `distances` | `(1,)` | Chebyshev distances for GLCM, GLDM and NGTDM |
| `symmetrical_glcm` | `True` | Add the transpose to each GLCM |
| `gldm_alpha` | `0` | GLDM dependence tolerance in bins |
| `force2d_axis` | `None` | NumPy axis for PyRadiomics `force2D`; also lets `shape2D` run on a single-slice 3D ROI |
| `slice_axis` | `0` | Planar aggregation axis when `force2d_axis` is unset |
| `neighborhood_radius` | `1` | Radius of the closed-radius NGTDM and NGLDM neighborhood |
| `ivh_bin_width`, `ivh_bin_count`, `ivh_intensity_range` | `None` | Intensity-volume histogram lattice |

### Voxel mode

| Field | Default | Controls |
| --- | --- | --- |
| `kernel_radius` | `1` | Neighborhood half-width around each ROI voxel |
| `masked_kernel` | `True` | Neighborhoods sample ROI voxels only |
| `scratch_mib` | `64` | Scratch budget in MiB; it sets the delivery batch size |

### Aggregation

`glcm_aggregation` and `glrlm_aggregation` accept `native`, `2d_average`,
`2d_slice_merge`, `2.5d_direction_merge`, `2.5d_merge`, `3d_average` and `3d_merge`.
`glszm_aggregation`, `gldzm_aggregation`, `ngtdm_aggregation` and `gldm_aggregation`
accept `native`, `2d_average`, `2.5d_merge` and `3d`. The default is `native` for all six.
`3d` aggregation requires 3D input without `force2d_axis`.

### Conventions

Each calculation convention is its own field with a short list of named values. Defaults
follow IBSI. Select an alternative explicitly for the method you intend.

| Field | IBSI default | Alternative | Controls |
| --- | --- | --- | --- |
| `covariance_divisor` | `"n-1"` | `"n"` | Divisor of the voxel-center covariance behind the shape axes and the approximate enclosing ellipsoid |
| `ellipsoid_area` | `"ibsi-series"` | `"exact"` | Surface area of the approximate enclosing ellipsoid in its area density |
| `kurtosis` | `"excess"` | `"raw"` | First-order kurtosis with or without subtracting three |
| `zero_variance_moments` | `"zero"` | `"nan"` | Skewness and kurtosis of a zero-variance ROI |
| `ngtdm_singular_cases` | `"ibsi-caps"` | `"unadjusted"` | Coarseness capped at 1e6, busyness 0 with one occupied level, strength 0 for zero differences |
| `neighborhood_support` | `"closed-radius"` | `"distances"` | NGLDM and NGTDM neighborhood: every voxel within `neighborhood_radius`, or the shells in `distances` |
| `bin_origin` | `"resegment-or-minimum"` | `"zero-lattice"` | Origin of a fixed-bin-size lattice when `bin_anchor` is `None` |
| `constant_roi_bin` | `"first"` | `"middle"` | Gray level of a constant ROI under `bin_count` |
| `mask_extrapolation` | `"zero"` | `"nearest"` | Mask outside the outermost source voxel centers under IBSI-grid interpolation |
| `spline_boundary` | `"nearest"` | `"mirror"` | Boundary of the cubic B-spline image interpolator under IBSI-grid interpolation: edge replication or ITK's mirror condition |

`Settings(bin_width=25.0, kurtosis="raw", covariance_divisor="n")` selects the raw
fourth-moment ratio and the population covariance and keeps every other default. The
matching parameter-file keys are `covarianceDivisor`, `ellipsoidArea`, `kurtosis`,
`zeroVarianceMoments`, `ngtdmSingularCases`, `neighborhoodSupport`, `binOrigin`,
`constantRoiBin`, `maskExtrapolation` and `splineBoundary`. In Rust,
`Settings::pyradiomics_conventions()` returns the PyRadiomics alternative for every
convention above.

The gray-level count of normalized inverse difference and moment is the number of
discretization bins, and histogram mode ties choose the bin closest to the mean and then
the lower bin. Neither has an alternative setting.

### Parameter-file keys

The `setting` section accepts `binWidth`, `binCount`, `binAnchor`, `normalize`,
`normalizeScale`, `removeOutliers`, `resegmentRange`, `resegmentMode`,
`resegmentOutlierSigma`, `resegmentShape`, `label`, `distances`, `symmetricalGLCM`,
`force2D` with `force2Ddimension`, `maskedKernel`, `kernelRadius`, `voxelArrayShift`,
`gldm_a`, `sliceAxis`, `neighborhoodRadius`, `ivhBinWidth`, `ivhBinCount`,
`ivhIntensityRange`, the aggregation keys (`glcmAggregation`, `glrlmAggregation`,
`glszmAggregation`, `gldzmAggregation`, `ngtdmAggregation`, `gldmAggregation`), the
convention keys above, and the spatial and filter keys in the next sections.

## Spatial options for files

File extraction reads the physical grid from the image. `ImageSettings` controls
resampling and mask correction:

| Field | Default | Controls |
| --- | --- | --- |
| `resampled_spacing` | `None` | Target spacing in NumPy axis order |
| `interpolator` | `Interpolator.BSPLINE` | `nearest`, `linear` or `bspline` |
| `correct_mask` | `False` | Correct the mask when it does not match the image grid |
| `pad_distance` | `5` | Padding around the ROI before resampling |
| `ibsi_interpolation` | `None` | An `IbsiInterpolation` for the IBSI resampling protocol |

`IbsiInterpolation(spacing, round_intensities=True, mask_threshold=0.5,
image_interpolator=Interpolator.LINEAR)` selects the IBSI protocol. In a parameter file,
`interpolator` takes `sitkNearestNeighbor`, `sitkLinear` or `sitkBSpline` (or the short
names above), and `padDistance` and `correctMask` set the other two fields.

## Image types

`TransformSettings.image_types` selects the images that features are computed on, in the
order given. Each derived image is discretized separately, and its feature keys carry its
name as a prefix, such as `wavelet-LLH`, `log-sigma-1-0-mm-3D` or `laws-L5S5-mirror`.

| `ImageType` | Parameter-file key | Options class | Notes |
| --- | --- | --- | --- |
| `ORIGINAL` | `Original` | | |
| `SQUARE` | `Square` | | |
| `SQUARE_ROOT` | `SquareRoot` | | |
| `LOGARITHM` | `Logarithm` | | |
| `EXPONENTIAL` | `Exponential` | | |
| `GRADIENT` | `Gradient` | `GradientSettings` | `use_spacing` scales by voxel size |
| `WAVELET` | `Wavelet` | `WaveletSettings` | Stationary wavelet; default `coif1`, level 1 |
| `LOG` | `LoG` | `LoGSettings` | Laplacian of Gaussian; `sigma` is required |
| `LBP2D` | `LBP2D` | `LBP2DSettings` | Planar local binary patterns |
| `LBP3D` | `LBP3D` | `LBP3DSettings` | Spherical local binary patterns; 3D input |
| `MEAN` | `Mean` | `MeanSettings` | Mean filter, support 3 |
| `LAWS` | `Laws` | `LawsSettings` | Laws kernels, default `L5S5` |
| `GABOR` | `Gabor` | `GaborSettings` | Gabor filter bank |
| `SIMONCELLI` | `Simoncelli`, `NonseparableWavelet` | `SimoncelliSettings` | Non-separable wavelet in the Fourier domain |
| `RIESZ` | `Riesz` | `RieszSettings` | Riesz transform of a LoG or Simoncelli base, with optional steering |
| `SEPARABLE_WAVELET` | `SeparableWavelet` | `SeparableWaveletSettings` | IBSI separable wavelet bands |

All 16 image types run. Simoncelli and Riesz need an FFT. This port computes them with a
CPU FFT, so they are enabled here, while mlx-radiomics 1.1.0 disables both because of a
fault in the batched Metal FFT it pins.

Each options class validates when it is constructed. `TransformSettings` also validates the
combination:

```python
from rustrad import ImageType, Settings, TransformSettings
from rustrad.config.transforms import LoGSettings, MeanSettings

transforms = TransformSettings(
    image_types=(ImageType.ORIGINAL, ImageType.MEAN, ImageType.LOG),
    mean=MeanSettings(support=5),
    log=LoGSettings(sigma=(1.0, 2.0)),
)
```

In ROI mode, shape features are computed once from the original inputs. Normalization and
resegmentation run once, before any derived image is computed.

### Filter options

Enum values live in `rustrad.config.filtering`.

| Class | Fields and defaults |
| --- | --- |
| `WaveletSettings` | `name="coif1"`, `level=1`, `start_level=0` |
| `LoGSettings` | `sigma=()`, `kernel="recursive"` (or `"sampled"`), `cutoff=4.0`, `padding="mirror"`, `constant=0.0`, `planar=False` |
| `LBP2DSettings` | `radius=1.0`, `samples=8`, `method="uniform"` (also `default`, `ror`, `nri_uniform`, `var`) |
| `LBP3DSettings` | `levels=2`, `radius=1.0`, `subdivision=1` |
| `GradientSettings` | `use_spacing=True` |
| `MeanSettings` | `support=3`, `padding="mirror"`, `constant=0.0`, `planar=False` |
| `LawsSettings` | `kernel="L5S5"`, `padding="mirror"`, `constant=0.0`, `pooling="none"`, `energy_distance=None` |
| `GaborSettings` | `sigma=()`, `wavelength=()`, `gamma=1.0`, `theta=()`, `response="modulus"` (or `real`, `imaginary`), `pooling="none"`, `slice_axis=None`, `padding="mirror"`, `constant=0.0` |
| `SimoncelliSettings` | `levels=(1,)`, `padding="periodic"`, `grid="dft"` (or `"centered"`), `planar=False` |
| `RieszSettings` | `base="log"` (or `"simoncelli"`), `orders=()`, `log_sigma=()`, `log_cutoff=4.0`, `simoncelli_levels=(1,)`, `padding="mirror"`, `constant=0.0`, `grid="dft"`, `steering=False`, `tensor_sigma=None` |
| `SeparableWaveletSettings` | `name="haar"`, `bands=()`, `level=1`, `padding="mirror"`, `constant=0.0`, `pooling="none"`, `planar=False` |

`padding` takes `constant`, `nearest`, `periodic` or `mirror`, and `pooling` takes `none`,
`average`, `max` or `min`. `TransformSettings.scratch_mib` (default 64) bounds the working
memory of the filters.

In a parameter file, an `imageType` entry carries these options as a mapping. The
Simoncelli and Riesz examples below use the field names above:

```yaml
imageType:
  Original: {}
  Simoncelli:
    levels: [1]
  Riesz:
    base: log
    log_sigma: [1.0]
    orders: [[1, 0, 0]]
```

LoG requires explicit `sigma` values. The recursive LoG needs 3D input with at least four
samples per axis. LBP3D needs 3D input and a masked kernel. See [Limits](limits.md) for
the full list of input requirements.
