# Python API

Use `extract` or `extract_files` to compute ROI features or voxel maps. Use `export` or
`export_files` to write voxel maps to disk. Use `extract_lesions` or `extract_lesions_files`
to compute ROI features for each connected lesion of a mask label.

| Operation | Arrays | NIfTI/NRRD files | Result |
| --- | --- | --- | --- |
| ROI features | `extract(image, mask)` | `extract_files(image_path, mask_path)` | `Result[float]` |
| Voxel maps in memory | `extract(image, mask, mode="voxel")` | `extract_files(image_path, mask_path, mode="voxel")` | `Result[NDArray[np.float64]]` |
| Voxel maps on disk | `export(image, mask, output_dir)` | `export_files(image_path, mask_path, output_dir)` | `MapExport(report, maps, metadata)` |
| ROI features per connected lesion | `extract_lesions(image, mask)` | `extract_lesions_files(image_path, mask_path)` | `tuple[Lesion, ...]` |

All options are keyword-only.

- Every operation accepts `settings=` (`Settings`) and `transforms=` (`TransformSettings`).
  Both default to the 93 default features on the original image.
- Array operations accept `spacing=` in millimeters, in NumPy axis order.
- File operations read spacing from the image. `extract_files` and `export_files` accept
  `image_settings=` (`ImageSettings`) for resampling and mask correction.
- Array `export` accepts `allow_synthetic_geometry=` to acknowledge its synthetic grid.
- Exports accept `overwrite=`, `scratch_dir=` and `max_scratch_mib=`.
- Extraction accepts `mode=` (`"roi"` or `"voxel"`), `map_dtype=` (`"float64"` or
  `"float32"`), `sink=` and `sink_factory=`.

## Signatures

```python
def extract(image, mask, *, settings=None, transforms=None, spacing=None,
            mode="roi", map_dtype="float64", sink=None, sink_factory=None) -> Result
def extract_files(image_path, mask_path, *, settings=None, image_settings=None,
                  transforms=None, mode="roi", map_dtype="float64",
                  sink=None, sink_factory=None) -> Result
def export(image, mask, output_dir, *, settings=None, transforms=None, spacing=None,
           allow_synthetic_geometry=False, overwrite=False, scratch_dir=None,
           max_scratch_mib=None) -> MapExport
def export_files(image_path, mask_path, output_dir, *, settings=None,
                 image_settings=None, transforms=None, overwrite=False,
                 scratch_dir=None, max_scratch_mib=None) -> MapExport
def extract_lesions(image, mask, *, settings=None, transforms=None, spacing=None,
                    min_voxels=1, max_lesions=None) -> tuple[Lesion, ...]
def extract_lesions_files(image_path, mask_path, *, settings=None, image_settings=None,
                          transforms=None, min_voxels=1,
                          max_lesions=None) -> tuple[Lesion, ...]
def label_lesions(mask, *, label=1, min_voxels=1, max_lesions=None) -> NDArray[np.int32]
def list_features(*families, mode="roi", ndim=None) -> tuple[str, ...]
```

`image` and `mask` are NumPy arrays of any integer, boolean or floating dtype, with 2 or 3
axes. The `.pyi` stubs shipped with the package give the exact overloads, and `mode` and
`map_dtype` select the return type for static type checkers.

## Example

```python
from rustradiomics import Settings, extract, extract_files, export_files

settings = Settings(label=2, features=("firstorder_Mean", "glcm_Contrast"))

roi = extract(image, mask, spacing=(3.0, 1.0, 1.0), settings=settings)
mean = roi.features["original_firstorder_Mean"]  # float

voxel = extract_files("image.nii.gz", "mask.nii.gz", settings=settings, mode="voxel")
mean_map = voxel.features["original_firstorder_Mean"]  # float64 NumPy array

saved = export_files("image.nii.gz", "mask.nii.gz", "outputs/maps", settings=settings)
saved.report                            # pathlib.Path to features.json
saved.maps["original_firstorder_Mean"]  # pathlib.Path to a float32 NIfTI
saved.metadata["settings"]              # effective extraction settings
```

## Results

Extraction returns `Result(features, metadata)`. Keys are `<image>_<family>_<name>`, such
as `original_firstorder_Mean`. ROI values are Python floats. Undefined values are NaN, and
infinities keep their sign. `metadata["feature_validity"]` counts finite, zero, NaN,
infinite and excluded samples per feature. `metadata` also records the effective settings
and transforms, `device` (`"cpu"`), `precision` (`"float64"`), the array dtype and shape,
and, for files, the source and processed geometry.

In-memory voxel maps are NumPy arrays on the source grid, zero outside the ROI. They are
float64 by default. Pass `map_dtype="float32"` to retain float32 maps with half the storage.
The values are the same in both cases, because maps hold the float32 samples that the
engine delivers. Saved NIfTI maps and streamed batches are float32. Use `export` or a sink
instead of retaining every map of a large image.

Exports return `MapExport(report, maps, metadata)`. `report` is the absolute path to the
schema-2 `features.json`, and `maps` maps each output name to the absolute path of its
float32 NIfTI under `<output_dir>/maps`. Map files carry the image-type prefix, for example
`wavelet-LLH_glcm_Contrast`. `export` and `export_files` always write voxel maps. For ROI
values, use `Result.features`.

## Geometry

File operations preserve the processed image's physical geometry. For arrays, pass
`spacing` in NumPy axis order, `(z, y, x)` for a conventional 3D image loaded with
SimpleITK. Omitted spacing emits a warning and assumes 1 mm on every axis. Array `export`
has no source origin or direction. It writes a zero origin and an identity direction and
warns unless `allow_synthetic_geometry=True`. Use `export_files` for scanner-aligned maps.

## Feature selection

```python
from rustradiomics import Settings, list_features

settings = Settings(features=list_features("firstorder", "glcm"))
voxel_names = list_features(mode="voxel", ndim=3)
```

`list_features` returns catalog names in catalog order, filtered by family, mode and
optional input rank. It does not change the 93-feature default. Unknown family names raise
`InvalidInputError`. See [Features](features.md) for the catalog.

## Derived images

```python
from rustradiomics import ImageType, Settings, TransformSettings, export_files
from rustradiomics.config.transforms import WaveletSettings

result = export_files(
    "image.nii.gz", "mask.nii.gz", "outputs/maps",
    settings=Settings(features=("firstorder_Mean", "glcm_Contrast")),
    transforms=TransformSettings(
        image_types=(ImageType.ORIGINAL, ImageType.WAVELET),
        wavelet=WaveletSettings(level=1),
    ),
)
# result.maps keys: original_glcm_Contrast, wavelet-LLH_glcm_Contrast, ...
```

[Parameters](parameters.md#image-types) lists all 16 image types and their options.

## Connected lesions

A mask label can hold several separate lesions. `extract_lesions` and
`extract_lesions_files` split the label into connected lesions and extract each one as
its own ROI:

```python
from rustradiomics import extract_lesions_files

lesions = extract_lesions_files(
    "image.nii.gz", "mask.nii.gz", settings=settings, min_voxels=10, max_lesions=5,
)
for lesion in lesions:
    lesion.index, lesion.voxels  # 1 is the largest lesion; size in voxels
    lesion.lower, lesion.upper   # array index bounds per axis, upper exclusive
    lesion.result.features["original_firstorder_Mean"]
```

Voxels that touch by a face, an edge or a corner belong to one lesion, as in PySERA.
Lesions are ordered by voxel count, largest first. `min_voxels` drops smaller lesions,
then `max_lesions` keeps the largest ones. Each `lesion.result` is the `Result` that
`extract` returns for a mask holding only that lesion. The file function splits the label
on the processed grid, after any resampling. Both functions return ROI features. For
voxel maps of one lesion, number the lesions with `label_lesions(mask)` and pass that map
as the mask with `Settings(label=k)`.

## Streaming voxel batches

To consume voxel results as they are computed, pass `mode="voxel"` and a sink:

```python
def consume(centers, features):
    # centers: int32 (batch, ndim) in NumPy axis order
    # features: name -> float32 (batch,)
    ...

result = extract(image, mask, spacing=(3.0, 1.0, 1.0), mode="voxel", sink=consume)
```

The arrays are borrowed for the duration of the call, so copy any you keep. The returned
`features` is empty, and `metadata` still holds the validity counts.

To own resources per derived image, pass `sink_factory=` instead. It is a callable that
takes the image name and returns a context manager. The context is entered before the
first batch of that image and exited after its last batch, including after an error:

```python
import contextlib

@contextlib.contextmanager
def per_image(name):
    print("open", name)
    yield consume          # a truthy sink receives the batches
    print("close", name)   # flush or release the image's storage here

extract(image, mask, spacing=(3.0, 1.0, 1.0), mode="voxel", sink_factory=per_image,
        transforms=TransformSettings(image_types=(ImageType.ORIGINAL, ImageType.SQUARE)))
```

If the context yields `None` or another falsey value, that image's maps are retained and
returned in `features`, and the other images are streamed. Supply at most one of `sink` and
`sink_factory`. If a context manager suppresses an error raised during a batch, the
original error still propagates, because the image has no completed result.

## Errors

| Exception | Raised for |
| --- | --- |
| `InvalidInputError` (a `ValueError`) | Invalid inputs, settings, parameter files or unsupported options |
| `ModelExecutionError` (a `RuntimeError`) | A numerical or execution refusal |
| `MissingDependencyError` (an `ImportError`) | An unavailable optional capability |
| `MemoryError` | A requested allocation that exceeds its budget or the addressable index range |
| `OSError` | File-system failures |

The first three live in `rustradiomics.errors`. `TransformSettings`, `ImageSettings` and
`IbsiInterpolation` validate when they are constructed. `Settings` validates when
extraction starts, and `settings.validate(ndim)` checks it earlier:

```python
Settings(bin_width=-1).validate(3)   # raises InvalidInputError
```
