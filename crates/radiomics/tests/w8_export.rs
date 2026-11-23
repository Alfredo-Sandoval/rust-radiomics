//! W8 exports and reports: `export` and `export_files`, the schema-2 `features.json`, map
//! files, publication safety and the ports of mlx's `test_export*.py` cases that apply
//! without the MedMLX runner. CPU orchestration; each test runs on its own thread.
//! Status: voxel exports, ROI report, staging, refusal and rollback cases.

mod parity_support;

use ndarray::Array3;
use radiomics::MapDtype;
use radiomics::extract_files;
use radiomics::io::{HostArray, read_image};
use radiomics::params::parse_params;
use radiomics::{Error, ImageType, Settings, TransformSettings};
use radiomics::{ExportFilesOptions, ExportOptions, export, export_files};
use radiomics::{FeatureValue, Output};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn w8() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/w8")
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "radiomics-w8-export-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir.canonicalize().unwrap()
}

fn files() -> (PathBuf, PathBuf) {
    (
        w8().join("synth3d_image.nii.gz"),
        w8().join("synth3d_mask.nii.gz"),
    )
}

fn staging(dir: &Path) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".mlxradiomics-"))
        .collect()
}

fn golden_params() -> (Value, Settings, radiomics::ImageSettings, TransformSettings) {
    let golden: Value =
        serde_json::from_slice(&fs::read(w8().join("synth3d.voxel.ibsi.mlx.json")).unwrap())
            .unwrap();
    let params: radiomics::params::Value = yaml_serde::to_value(&golden["params"]).unwrap();
    let parsed = parse_params(&params, 3).unwrap();
    (
        golden,
        parsed.settings,
        parsed.image_settings,
        parsed.transforms,
    )
}

fn small_settings(features: &[&str]) -> Settings {
    Settings {
        features: features.iter().map(|s| s.to_string()).collect(),
        ..Settings::default()
    }
}

fn paths(value: &Value, pointer: &str, prefix: &str, out: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                out.insert(format!("{prefix}{pointer}={{}}"));
            }
            for (k, v) in map {
                paths(v, &format!("{pointer}/{k}"), prefix, out);
            }
        }
        _ => {
            out.insert(format!("{prefix}{pointer}"));
        }
    }
}

#[test]
fn export_files_writes_derived_maps_equal_to_extraction_with_source_geometry() {
    let (golden, settings, image_settings, transforms) = golden_params();
    let (image, mask) = files();
    let dir = scratch("maps");
    let out = dir.join("out");
    let saved = export_files(
        &image,
        &mask,
        &out,
        &ExportFilesOptions {
            settings: Some(settings.clone()),
            image_settings: Some(image_settings.clone()),
            transforms: Some(transforms.clone()),
            ..Default::default()
        },
    )
    .unwrap();
    let expected = extract_files(
        &image,
        &mask,
        &settings,
        &image_settings,
        &transforms,
        Output::Maps {
            dtype: MapDtype::Float32,
            max_bytes: None,
        },
    )
    .unwrap();
    assert_eq!(
        saved.maps.keys().collect::<Vec<_>>(),
        expected.features.keys().collect::<Vec<_>>()
    );
    assert!(saved.maps.keys().any(|k| k.starts_with("wavelet-LLH_")));
    assert!(saved.maps.keys().any(|k| k.starts_with("log-sigma-")));
    assert!(saved.maps.keys().any(|k| k.starts_with("laws-")));
    let source = read_image(&image).unwrap();
    for (name, path) in &saved.maps {
        assert_eq!(path, &out.join("maps").join(format!("{name}.nii.gz")));
        let written = read_image(path).unwrap();
        let FeatureValue::Map32(map) = &expected.features[name] else {
            panic!("float32 map expected")
        };
        let HostArray::F32(values) = &written.array else {
            panic!("{name}: written maps are float32")
        };
        assert_eq!(values.shape(), map.shape());
        for (a, b) in values.iter().zip(map.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "{name}");
        }
        assert_eq!(written.spacing, source.spacing);
        assert_eq!(written.origin, source.origin);
        assert_eq!(written.direction, source.direction);
    }
    // Report: schema 2, details and map paths.
    let report: Value = serde_json::from_slice(&fs::read(&saved.report).unwrap()).unwrap();
    assert_eq!(report["schema_version"], 2);
    assert_eq!(report["mode"], "voxel");
    assert_eq!(report["details"]["feature_count"], saved.maps.len());
    assert_eq!(report["details"]["label"], 1);
    // `features.json` sorts its keys; the mapping equals the returned one.
    let listed = report["maps"].as_object().unwrap();
    assert_eq!(listed.len(), saved.maps.len());
    for (name, path) in &saved.maps {
        assert_eq!(Path::new(listed[name].as_str().unwrap()), path.as_path());
    }
    assert!(listed.keys().is_sorted());
    assert_eq!(
        report["details"]["execution"]["settings"]["bin_count"],
        golden["params"]["setting"]["binCount"]
    );
    assert!(staging(&out).is_empty());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn report_structure_and_provenance_match_mlx_features_json() {
    let (_, settings, image_settings, transforms) = golden_params();
    let (image, mask) = files();
    let dir = scratch("report");
    let saved = export_files(
        &image,
        &mask,
        &dir.join("out"),
        &ExportFilesOptions {
            settings: Some(settings),
            image_settings: Some(image_settings),
            transforms: Some(transforms),
            ..Default::default()
        },
    )
    .unwrap();
    let rust: Value = serde_json::from_slice(&fs::read(&saved.report).unwrap()).unwrap();
    let mlx: Value =
        serde_json::from_slice(&fs::read(w8().join("synth3d.export_report.mlx.json")).unwrap())
            .unwrap();
    // Every key path of mlx's report exists here and vice versa. The `runtime.versions`
    // entries name different libraries, which is the contract, so only the section is kept.
    let (mut a, mut b) = (BTreeSet::new(), BTreeSet::new());
    let mut rust_trim = rust.clone();
    let mut mlx_trim = mlx.clone();
    for report in [&mut rust_trim, &mut mlx_trim] {
        report["details"]["execution"]["runtime"]["versions"] = Value::Null;
    }
    paths(&rust_trim, "", "", &mut a);
    paths(&mlx_trim, "", "", &mut b);
    let only_rust: Vec<_> = a.difference(&b).collect();
    let only_mlx: Vec<_> = b.difference(&a).collect();
    assert!(
        only_rust.is_empty() && only_mlx.is_empty(),
        "report keys differ. only Rust: {only_rust:#?} only mlx: {only_mlx:#?}"
    );
    // Values that must be identical: layout, settings, spatial options, geometry, counts.
    let (r, m) = (&rust["details"], &mlx["details"]);
    for key in ["voxel", "label", "shape", "spacing", "feature_count"] {
        assert_eq!(r[key], m[key], "details.{key}");
    }
    assert_eq!(rust["schema_version"], mlx["schema_version"]);
    assert_eq!(rust["mode"], mlx["mode"]);
    let (re, me) = (&r["execution"], &m["execution"]);
    for key in [
        "settings",
        "image_settings",
        "transforms",
        "image_geometry",
        "source_geometry",
        "gray_levels_by_image",
        "array_shape",
        "array_dtype",
        "roi_voxels",
        "excluded_voxels",
        "spacing",
        "voxel",
        "aggregation",
        "map_dtype",
        "feature_validity",
        "gray_levels",
    ] {
        assert_eq!(re[key], me[key], "execution.{key}");
    }
    // This port states its own execution.
    assert_eq!(re["device"], "cpu");
    assert_eq!(re["precision"], "float64");
    assert_eq!(me["device"], "gpu");
    assert_eq!(
        rust["maps"].as_object().unwrap().keys().collect::<Vec<_>>(),
        mlx["maps"].as_object().unwrap().keys().collect::<Vec<_>>()
    );
    for (name, path) in rust["maps"].as_object().unwrap() {
        assert!(
            path.as_str()
                .unwrap()
                .ends_with(&format!("maps/{name}.nii.gz")),
            "{name}"
        );
    }
    // Strict JSON, sorted keys, two-space indentation.
    let text = fs::read_to_string(&saved.report).unwrap();
    assert!(text.starts_with("{\n  \"details\": {\n"));
    assert!(!text.contains("NaN\n") && !text.contains(": NaN"));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn export_refuses_overwrite_keeps_previous_outputs_and_replaces_on_request() {
    let (image, mask) = files();
    let dir = scratch("overwrite");
    let out = dir.join("out");
    let options = ExportFilesOptions {
        settings: Some(small_settings(&["firstorder_Mean"])),
        transforms: Some(TransformSettings {
            image_types: vec![ImageType::Original, ImageType::Square],
            ..Default::default()
        }),
        ..Default::default()
    };
    let first = export_files(&image, &mask, &out, &options).unwrap();
    let previous: Vec<_> = first.maps.values().map(|p| fs::read(p).unwrap()).collect();
    let error = export_files(&image, &mask, &out, &options).unwrap_err();
    assert!(matches!(&error, Error::InvalidInput(m) if m.contains("overwrite=True")));
    assert_eq!(
        first
            .maps
            .values()
            .map(|p| fs::read(p).unwrap())
            .collect::<Vec<_>>(),
        previous
    );
    let replaced = export_files(
        &image,
        &mask,
        &out,
        &ExportFilesOptions {
            overwrite: true,
            ..options.clone()
        },
    )
    .unwrap();
    assert_eq!(replaced.maps, first.maps);
    assert!(staging(&out).is_empty());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn a_different_selection_in_an_occupied_directory_is_refused() {
    let (image, mask) = files();
    let dir = scratch("mixed");
    let out = dir.join("out");
    let first = ExportFilesOptions {
        settings: Some(small_settings(&["firstorder_Mean"])),
        ..Default::default()
    };
    export_files(&image, &mask, &out, &first).unwrap();
    let report = fs::read(out.join("features.json")).unwrap();
    let other = ExportFilesOptions {
        settings: Some(small_settings(&["firstorder_Variance"])),
        ..Default::default()
    };
    // features.json exists, so the new selection is refused even though no map collides.
    let error = export_files(&image, &mask, &out, &other).unwrap_err();
    assert!(matches!(&error, Error::InvalidInput(m) if m.contains("overwrite=True")));
    assert_eq!(fs::read(out.join("features.json")).unwrap(), report);
    assert!(
        !out.join("maps/original_firstorder_Variance.nii.gz")
            .exists()
    );
    fs::remove_dir_all(dir).unwrap();
}

fn small_arrays() -> (HostArray, HostArray) {
    let image = Array3::from_shape_fn((6, 7, 8), |(z, y, x)| {
        ((z * 31 + y * 17 + x * 5) % 23) as i16 * 10 - 40
    });
    let mask = Array3::from_shape_fn((6, 7, 8), |(z, y, x)| {
        u8::from((1..5).contains(&z) && (1..6).contains(&y) && (1..7).contains(&x))
    });
    (image.into_dyn().into(), mask.into_dyn().into())
}

#[test]
fn export_rejects_shape_features_before_creating_anything() {
    let (image, mask) = small_arrays();
    let dir = scratch("shape");
    let out = dir.join("out");
    let error = export(
        &image,
        &mask,
        &out,
        &ExportOptions {
            settings: Some(small_settings(&["shape_MeshVolume"])),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "shape features are unavailable in voxel mode"
    );
    assert!(!out.exists());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn export_writer_failure_leaves_no_report_staging_or_partial_outputs() {
    use std::os::unix::fs::PermissionsExt;
    let (image, mask) = small_arrays();
    let dir = scratch("failure");
    let out = dir.join("out");
    fs::create_dir_all(out.join("maps")).unwrap();
    // The staged files are written, but publication cannot link into a read-only map directory.
    fs::set_permissions(out.join("maps"), fs::Permissions::from_mode(0o555)).unwrap();
    let probe = fs::write(out.join("maps/probe"), b"x");
    if probe.is_ok() {
        // Running as a user for whom permissions are not enforced (for example root).
        fs::remove_dir_all(&dir).ok();
        return;
    }
    let error = export(
        &image,
        &mask,
        &out,
        &ExportOptions {
            settings: Some(small_settings(&["firstorder_Mean"])),
            spacing: Some(vec![1.5, 1.0, 0.8]),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::Io(_)), "{error}");
    assert!(!out.join("features.json").exists());
    assert!(staging(&out).is_empty());
    assert_eq!(fs::read_dir(out.join("maps")).unwrap().count(), 0);
    fs::set_permissions(out.join("maps"), fs::Permissions::from_mode(0o755)).unwrap();
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn invalid_requests_are_input_errors() {
    let (image, mask) = small_arrays();
    let dir = scratch("invalid");
    for (spacing, settings) in [
        (Some(vec![-1.0, 1.0, 1.0]), Settings::default()),
        (
            Some(vec![1.0, 1.0, 1.0]),
            Settings {
                bin_width: 0.0,
                ..Settings::default()
            },
        ),
        (Some(vec![1.0, 1.0]), Settings::default()),
    ] {
        let error = export(
            &image,
            &mask,
            &dir.join("out"),
            &ExportOptions {
                settings: Some(settings),
                spacing,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(matches!(error, Error::InvalidInput(_)), "{error}");
    }
    // The output path must be a directory.
    let file = dir.join("occupied");
    fs::write(&file, b"x").unwrap();
    let error = export(&image, &mask, &file, &ExportOptions::default()).unwrap_err();
    assert!(
        matches!(&error, Error::InvalidInput(m) if m.contains("output_dir is not a directory")),
        "{error}"
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn scratch_refusals_leave_nothing_behind() {
    let image = Array3::<i16>::from_shape_fn((65, 65, 65), |(z, y, x)| (z + y + x) as i16);
    let mask = Array3::<u8>::from_elem((65, 65, 65), 1);
    let (image, mask): (HostArray, HostArray) = (image.into_dyn().into(), mask.into_dyn().into());
    let dir = scratch("scratch");
    let existing = dir.join("scratch");
    fs::create_dir(&existing).unwrap();
    let cases = [
        // A missing scratch directory, a budget the retained rows exhaust and an invalid budget.
        (Some(dir.join("missing")), Some(1)),
        (Some(existing.clone()), Some(1)),
        (Some(existing.clone()), Some(0)),
    ];
    for (scratch_dir, max_scratch_mib) in cases {
        let out = dir.join("out");
        let error = export(
            &image,
            &mask,
            &out,
            &ExportOptions {
                settings: Some(small_settings(&["firstorder_Mean"])),
                transforms: Some(TransformSettings {
                    image_types: vec![ImageType::Square],
                    ..Default::default()
                }),
                scratch_dir,
                max_scratch_mib,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(matches!(error, Error::InvalidInput(_)), "{error}");
        assert!(!out.join("features.json").exists());
        if out.exists() {
            assert!(staging(&out).is_empty());
            assert!(
                fs::read_dir(out.join("maps"))
                    .map(|d| d.count() == 0)
                    .unwrap_or(true)
            );
        }
        assert_eq!(fs::read_dir(&existing).unwrap().count(), 0);
    }
    fs::remove_dir_all(dir).unwrap();
}

fn two_voxel_case(side: usize) -> (HostArray, HostArray) {
    let image =
        Array3::<i16>::from_shape_fn((side, side, side), |(z, y, x)| (z + 2 * y + x) as i16);
    let mut mask = Array3::<u8>::zeros((side, side, side));
    mask[[10, 10, 10]] = 1;
    mask[[20, 21, 22]] = 1;
    (image.into_dyn().into(), mask.into_dyn().into())
}

fn export_with_limit(
    image: &HostArray,
    mask: &HostArray,
    name: &str,
    mib: u64,
) -> radiomics::Result<()> {
    let dir = scratch(name);
    let result = export(
        image,
        mask,
        &dir.join("out"),
        &ExportOptions {
            settings: Some(small_settings(&["firstorder_Mean", "firstorder_Minimum"])),
            max_scratch_mib: Some(mib),
            ..Default::default()
        },
    );
    fs::remove_dir_all(dir).unwrap();
    result.map(|_| ())
}

#[test]
fn export_admission_charges_retained_rows_not_the_dense_grid() {
    // 56^3 grid, two selected voxels, two maps, 1 MiB: dense float64 storage would need
    // 2,809,856 bytes; the two retained rows and the writer buffers fit.
    let (image, mask) = two_voxel_case(56);
    export_with_limit(&image, &mask, "admit-sparse", 1).unwrap();
}

#[test]
fn export_over_the_retained_requirement_is_refused_with_the_budget_error() {
    // Every voxel of a 43^3 grid selected: 79,507 rows of one map need 636,056 bytes of
    // columns, 318,028 of offsets and the writer allowance, more than 1 MiB.
    let side = 43;
    let image: HostArray =
        Array3::<i16>::from_shape_fn((side, side, side), |(z, y, x)| (z + 2 * y + x) as i16)
            .into_dyn()
            .into();
    let mask: HostArray = Array3::<u8>::from_elem((side, side, side), 1)
        .into_dyn()
        .into();
    let dir = scratch("admit-over");
    let error = export(
        &image,
        &mask,
        &dir.join("out"),
        &ExportOptions {
            settings: Some(small_settings(&["firstorder_Mean"])),
            max_scratch_mib: Some(1),
            ..Default::default()
        },
    )
    .unwrap_err();
    fs::remove_dir_all(dir).unwrap();
    let Error::InvalidInput(message) = error else {
        panic!("expected InvalidInput, got {error}");
    };
    assert!(
        message.contains("voxel export temporary storage needs"),
        "{message}"
    );
    assert!(
        message.contains("the temporary budget of 1048576 bytes"),
        "{message}"
    );
}

#[test]
fn array_export_carries_synthetic_geometry_and_the_array_values() {
    let (image, mask) = small_arrays();
    let dir = scratch("array");
    let saved = export(
        &image,
        &mask,
        &dir.join("out"),
        &ExportOptions {
            settings: Some(small_settings(&["firstorder_Mean", "glcm_Contrast"])),
            spacing: Some(vec![2.0, 3.0, 4.0]),
            allow_synthetic_geometry: true,
            ..Default::default()
        },
    )
    .unwrap();
    let volume = read_image(&saved.maps["original_firstorder_Mean"]).unwrap();
    assert_eq!(volume.origin, [0.0; 3]);
    assert_eq!(volume.spacing, [2.0, 3.0, 4.0]);
    assert_eq!(
        volume.direction,
        [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
    );
    // No file geometry in the metadata: neither source geometry nor image settings.
    assert!(saved.metadata.source_geometry.is_none());
    assert!(saved.metadata.image_settings.is_none());
    assert_eq!(
        saved.metadata.image_geometry.as_ref().unwrap().shape,
        [6, 7, 8]
    );
    // Maps agree with the in-memory extraction.
    let expected = radiomics::extract(
        &image,
        &mask,
        &small_settings(&["firstorder_Mean", "glcm_Contrast"]),
        &TransformSettings::default(),
        Some(&[2.0, 3.0, 4.0]),
        Output::Maps {
            dtype: MapDtype::Float32,
            max_bytes: None,
        },
    )
    .unwrap();
    let HostArray::F32(values) = &volume.array else {
        panic!("float32 map")
    };
    let FeatureValue::Map32(map) = &expected.features["original_firstorder_Mean"] else {
        panic!("float32 map")
    };
    assert_eq!(values, &map.clone().into_dyn().mapv(|v| v));
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn outputs_may_not_alias_inputs_and_overlapping_runs_are_refused() {
    let (image, mask) = files();
    let dir = scratch("alias");
    let out = dir.join("out");
    fs::create_dir_all(out.join("maps")).unwrap();
    // A map path is an input file: refused, and the input is untouched.
    let copy_image = out.join("maps/original_firstorder_Mean.nii.gz");
    fs::copy(&image, &copy_image).unwrap();
    let before = fs::read(&copy_image).unwrap();
    let error = export_files(&copy_image, &mask, &out, &ExportFilesOptions::default());
    assert!(
        matches!(&error, Err(Error::InvalidInput(m)) if m.contains("output aliases an input")),
        "{error:?}"
    );
    assert_eq!(fs::read(&copy_image).unwrap(), before);
    assert!(staging(&out).is_empty());
    // An overlapping run on the same directory holds the lock.
    // The run takes an advisory lock on the output directory itself.
    fs::create_dir_all(dir.join("locked")).unwrap();
    let held = fs::File::open(dir.join("locked")).unwrap();
    held.try_lock().unwrap();
    let error = export_files(
        &image,
        &mask,
        &dir.join("locked"),
        &ExportFilesOptions::default(),
    );
    assert!(
        matches!(&error, Err(Error::InvalidInput(m)) if m.contains("another run is using output directory")),
        "{error:?}"
    );
    drop(held);
    // Stale staging from a killed run is reclaimed by the next run.
    let stale = dir.join("locked/.mlxradiomics-abcd0123");
    fs::create_dir_all(&stale).unwrap();
    fs::write(stale.join(".mlxradiomics-staging"), "1\n").unwrap();
    export_files(
        &image,
        &mask,
        &dir.join("locked"),
        &ExportFilesOptions::default(),
    )
    .unwrap();
    assert!(!stale.exists());
    fs::remove_dir_all(dir).unwrap();
}
