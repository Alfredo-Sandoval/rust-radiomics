# Rust API

The `radiomics` crate is the CPU engine behind the Python package. It is not published to
crates.io. Depend on it by git:

```toml
[dependencies]
radiomics = { git = "https://github.com/Alfredo-Sandoval/rust-radiomics" }
```

The crate re-exports `ndarray` and `indexmap`, so `radiomics::ndarray` and
`radiomics::indexmap` always match the versions in its signatures. The crate declares Rust
1.89 as its minimum version, and the repository is built and tested with Rust 1.96.0.
Computation uses Rayon. Set `RAYON_NUM_THREADS` to limit the worker count.

Arrays are `ndarray::ArrayD` values with 2 or 3 axes in NumPy order, `(z, y, x)`. Spacing is
in millimeters in the same order. `HostArray` wraps an array of any supported element type
(`bool`, `u8` to `u64`, `i8` to `i64`, `f32`, `f64`) and converts with `HostArray::from`.

## Public surface

Everything is reachable from the crate root or from four modules. The rest of the crate is
private. The hidden `__private` module supports the Python binding and may change in any
release.

| Path | Contents |
| --- | --- |
| `radiomics::{extract, extract_files}` | Features or maps from arrays or from a NIfTI or NRRD pair |
| `radiomics::{extract_lesions, extract_lesions_files, label_lesions, Lesion, LesionOptions}` | ROI features for each connected lesion of a mask label |
| `radiomics::{export, export_files}` | Voxel maps written as NIfTI files with a `features.json` report |
| `radiomics::{Output, VoxelSink, SinkFactory, MapDtype}` | Delivery mode and streaming consumers |
| `radiomics::{ExtractionResult, FeatureValue, MapExport}` | Results |
| `radiomics::{Settings, ImageSettings, IbsiInterpolation, Interpolator, TransformSettings, ImageType}` | Settings |
| `radiomics::{ExportOptions, ExportFilesOptions}` | Export options |
| `radiomics::{list_features, features, default_features, Family, FeatureSpec, Mode}` | Feature catalog |
| `radiomics::{HostArray, Error, Result}` | Input arrays and errors |
| `radiomics::io` | `HostArray`, `Volume`, `ArrayKind`, `read_image`, `read_mask`, `VOLUME_SUFFIXES` |
| `radiomics::params` | `ExtractionParameters` (`from_yaml`, `from_file`), `load_params`, `parse_params`, `settings_from_params`, `Value` |
| `radiomics::settings` | Convention enums of `Settings` and the per-filter settings structs of `TransformSettings` |
| `radiomics::metadata` | `ExecutionMetadata`, `RuntimeMetadata`, `ImageGeometryPayload`, `FeatureValidity`, `Device`, `Precision` |

Enums that can gain variants (`Error`, `Output`, `FeatureValue`, `Family`, `ImageType`,
`MapDtype` and the convention enums) are `#[non_exhaustive]`, so a `match` on them needs a
wildcard arm. Settings and option structs have public fields and implement `Default`. Build
them with `..Default::default()`. They are validated when they are used.

## Entry points

| Function | Purpose |
| --- | --- |
| `extract(image, mask, settings, transforms, spacing, output)` | Features or maps from aligned arrays |
| `extract_files(image_path, mask_path, settings, image_settings, transforms, output)` | Features or maps from a NIfTI or NRRD pair |
| `extract_lesions(image, mask, settings, transforms, spacing, &LesionOptions)` | ROI features for each connected lesion of `settings.label` |
| `extract_lesions_files(image_path, mask_path, settings, image_settings, transforms, &LesionOptions)` | The same from a NIfTI or NRRD pair, split on the processed grid |
| `label_lesions(mask, label, &LesionOptions)` | Lesion map of one label, 1 for the largest lesion |
| `export(image, mask, output_dir, &ExportOptions)` | Write voxel maps and a schema-2 report from arrays |
| `export_files(image_path, mask_path, output_dir, &ExportFilesOptions)` | Write voxel maps and a report from files |
| `list_features(families, mode, ndim)`, `features()`, `default_features()` | Inspect the catalog |

`extract` and `extract_files` return `Result<ExtractionResult<FeatureValue>>`. The
`ExtractionResult` has `features`, an ordered map keyed `<image>_<family>_<name>`, and
`metadata`. A `FeatureValue` is `Scalar(f64)`, `Map32(ArrayD<f32>)` or `Map64(ArrayD<f64>)`.
`export` and `export_files` return a `MapExport` with `report`, `maps` and `metadata`.
`extract_lesions` and `extract_lesions_files` return a `Vec<Lesion>`, largest lesion first.
A `Lesion` has `index`, `voxels`, the index bounds `lower` and `upper`, and the `result` of
extracting that lesion alone. Voxels that touch by a face, an edge or a corner belong to
one lesion. `LesionOptions` drops lesions below `min_voxels`, then keeps the `max_lesions`
largest.
`radiomics::Error` has the variants `InvalidInput`, `Execution`, `Memory`,
`MissingDependency`, `Io` and `NotImplemented`. It implements `std::error::Error`, `Send`
and `Sync`.

`Output` chooses the delivery mode:

| Variant | Delivery |
| --- | --- |
| `Output::Roi` | One scalar per feature |
| `Output::Maps { dtype, max_bytes }` | Retain zero-background maps, with an optional byte limit |
| `Output::Stream { dtype, sink }` | Send float32 batches to a `VoxelSink` |
| `Output::PerImage { dtype, factory }` | Open a separate sink for each original or derived image through a `SinkFactory` |

## ROI features from arrays

```rust
use radiomics::ndarray::ArrayD;
use radiomics::{FeatureValue, HostArray, Output, Settings, TransformSettings, extract};

fn main() -> radiomics::Result<()> {
    let image = ArrayD::from_shape_fn(vec![8, 16, 16], |ix| (ix[0] * 7 + ix[1] * 3 + ix[2]) as i16);
    let mask = ArrayD::from_shape_fn(vec![8, 16, 16], |ix| {
        u8::from(ix[0] > 1 && ix[1] > 3 && ix[1] < 12 && ix[2] > 3 && ix[2] < 12)
    });
    let settings = Settings {
        features: vec!["firstorder_Mean".into(), "glcm_Contrast".into()],
        bin_width: 5.0,
        ..Settings::default()
    };
    let result = extract(
        &HostArray::from(image),
        &HostArray::from(mask),
        &settings,
        &TransformSettings::default(),
        Some(&[3.0, 1.0, 1.0]),
        Output::Roi,
    )?;
    for (name, value) in &result.features {
        if let FeatureValue::Scalar(v) = value {
            println!("{name} = {v}");
        }
    }
    Ok(())
}
```

Passing `None` for `spacing` assumes 1 mm on every axis.

## Retained and streamed voxel maps

```rust
use radiomics::indexmap::IndexMap;
use radiomics::ndarray::{ArrayD, ArrayView1, ArrayView2};
use radiomics::{
    FeatureValue, HostArray, MapDtype, Output, Settings, TransformSettings, extract,
};

fn main() -> radiomics::Result<()> {
    let image = HostArray::from(ArrayD::from_shape_fn(vec![8, 16, 16], |ix| {
        (ix[0] * 7 + ix[1] * 3 + ix[2]) as i16
    }));
    let mask = HostArray::from(ArrayD::from_elem(vec![8, 16, 16], 1_u8));
    let settings = Settings {
        features: vec!["firstorder_Mean".into()],
        ..Settings::default()
    };

    // Retain one float64 map per feature.
    let maps = extract(
        &image,
        &mask,
        &settings,
        &TransformSettings::default(),
        None,
        Output::Maps { dtype: MapDtype::Float64, max_bytes: None },
    )?;
    if let Some(FeatureValue::Map64(map)) = maps.features.get("original_firstorder_Mean") {
        println!("{:?}", map.shape());
    }

    // Stream float32 batches. A closure with this signature is a `VoxelSink`.
    let mut seen = 0usize;
    let mut sink = |centers: ArrayView2<'_, i32>,
                    _features: &IndexMap<String, ArrayView1<'_, f32>>|
     -> radiomics::Result<()> {
        seen += centers.nrows(); // one row of NumPy-order coordinates per voxel
        Ok(())
    };
    extract(
        &image,
        &mask,
        &settings,
        &TransformSettings::default(),
        None,
        Output::Stream { dtype: MapDtype::Float32, sink: &mut sink },
    )?;
    println!("{seen} voxels streamed");
    Ok(())
}
```

`max_bytes: None` limits retained output to a quarter of physical memory where the platform
reports it. For one sink per derived image, implement `SinkFactory` and pass
`Output::PerImage`. Its `open_optional` method may return `None` to retain that image's
maps. The `SinkFactory` documentation has a complete example.

## Files, parameter files and export

```rust
use std::path::Path;

use radiomics::params::ExtractionParameters;
use radiomics::{
    ExportFilesOptions, ImageSettings, Output, Settings, export_files, extract_files,
};

fn main() -> radiomics::Result<()> {
    let image_path = Path::new("image.nii.gz");
    let mask_path = Path::new("mask.nii.gz");

    // Settings from a PyRadiomics parameter file, for a 3D image.
    let params = ExtractionParameters::from_file("params.yaml", 3)?;
    let result = extract_files(
        image_path,
        mask_path,
        &params.settings,
        &params.image_settings,
        &params.transforms,
        Output::Roi,
    )?;
    println!("{} features", result.features.len());

    // Voxel maps and a report on disk.
    let options = ExportFilesOptions {
        settings: Some(Settings {
            features: vec!["firstorder_Mean".into()],
            ..Settings::default()
        }),
        image_settings: Some(ImageSettings::default()),
        ..ExportFilesOptions::default()
    };
    let saved = export_files(image_path, mask_path, Path::new("outputs/maps"), &options)?;
    println!("{:?} {:?}", saved.report, saved.maps.keys().collect::<Vec<_>>());
    Ok(())
}
```

Export refuses an occupied output directory unless `overwrite` is set. Array `export` takes
an `ExportOptions` with the same fields plus `spacing` and `allow_synthetic_geometry`.
`ExtractionParameters::from_yaml` parses a document held in a string. The [parameter
guide](parameters.md) lists the supported keys.

## Catalog

```rust
use radiomics::{Mode, default_features, features, list_features};

fn main() -> radiomics::Result<()> {
    let names = list_features(&["firstorder"], Mode::Roi, Some(3))?;
    assert_eq!(names.len(), 21);
    assert_eq!(features().len(), 200);
    assert_eq!(default_features().len(), 93);
    Ok(())
}
```

`features()` returns all 200 `FeatureSpec` entries with `family`, `name`, `default`,
`ibsi_id`, `ibsi_title` and `units`. `default_features()` returns the 93 default names.
Run `cargo doc --open -p radiomics` for the full reference.
