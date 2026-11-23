//! W11 reference inventory and local-intensity comparisons, CPU orchestration on each test thread.
//! Status: runs in the default build.

mod parity_support;

use crate::parity_support::*;
use radiomics::{Mode, features, list_features};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn every_case_and_golden_matches_the_catalog_and_grid() {
    let keys: BTreeSet<_> = features().iter().map(|f| f.key()).collect();
    let mut cases = BTreeMap::new();
    let mut inventory = BTreeSet::new();
    for path in records("cases") {
        let case: Fixture = read(&path);
        assert_eq!(path.file_stem().unwrap().to_str(), Some(case.case.as_str()));
        let (image, mask) = case.arrays();
        assert!(matches!(image.ndim(), 2 | 3));
        assert_eq!(case.spacing.len(), image.ndim());
        assert!(case.spacing.iter().all(|s| s.is_finite() && *s > 0.));
        assert_eq!(case.origin.len(), image.ndim());
        assert_eq!(case.direction.len(), image.ndim().pow(2));
        assert!(mask.iter().any(|v| *v == case.label));
        let singleton = mask.iter().filter(|v| **v == case.label).count() == 1;
        for profile in ["width25", "count32"] {
            for convention in ["ibsi", "pyradiomics"] {
                for mode in ["roi", "voxel"] {
                    let stem = format!("{}.{profile}.{convention}.{mode}", case.case);
                    inventory.insert(format!("{stem}.mlx.json"));
                    if case.case.starts_with("npz_")
                        && convention == "pyradiomics"
                        && (mode == "roi" || !singleton)
                    {
                        inventory.insert(format!("{stem}.pyradiomics.json"));
                    }
                }
            }
        }
        cases.insert(case.case.clone(), case);
    }
    // The source archives, both synthetic dimensions and the IBSI digital phantom.
    assert_eq!(
        cases.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "ibsi_digital",
            "npz_bin_boundaries_2d",
            "npz_flat",
            "npz_force2d",
            "npz_larger_shifted_kernel",
            "npz_negative_ct_multilabel",
            "npz_singleton",
            "npz_unmasked_neighborhood",
            "synthetic_constant",
            "synthetic_fractional_2d",
            "synthetic_multilabel_3d",
            "synthetic_single_voxel",
        ]
    );

    for path in records("golden") {
        let filename = path.file_name().unwrap().to_str().unwrap();
        assert!(inventory.remove(filename), "unexpected golden {filename}");
        let golden: Golden = read(&path);
        let case = &cases[&golden.case];
        assert_eq!(golden.shape, case.shape, "{filename}");
        // Older records retain the generator path used to produce them.
        assert!(matches!(
            golden.metadata.command.split_whitespace().nth(1),
            Some("parity/golden.py" | "validation/golden.py")
        ));
        let mode = match golden.mode {
            GoldenMode::Roi => Mode::Roi,
            GoldenMode::Voxel => Mode::Voxel,
        };
        let settings = golden.settings();
        let profile = if settings.bin_count == Some(32) {
            "count32"
        } else {
            assert_eq!((settings.bin_count, settings.bin_width), (None, 25.));
            "width25"
        };
        let convention = match golden.settings.convention {
            Convention::Ibsi => "ibsi",
            Convention::Pyradiomics => "pyradiomics",
        };
        let mode_name = if mode == Mode::Voxel { "voxel" } else { "roi" };
        let mlx = golden.metadata.engine == "mlxradiomics";
        let engine = if mlx {
            assert_eq!(golden.metadata.version, "1.1.0");
            "mlx"
        } else {
            assert_eq!(golden.metadata.engine, "pyradiomics");
            "pyradiomics"
        };
        assert_eq!(
            filename,
            format!(
                "{}.{profile}.{convention}.{mode_name}.{engine}.json",
                case.case
            )
        );
        assert_eq!(settings.label, case.label);
        if let Some(error) = &golden.expected_error {
            assert!(mlx && mode == Mode::Voxel);
            assert_eq!(
                case.arrays().1.iter().filter(|v| **v == case.label).count(),
                1
            );
            assert_eq!(
                error,
                "voxel extraction requires at least two selected voxels"
            );
            assert!(golden.features.is_empty());
            continue;
        }
        let selected = list_features(&[], mode, Some(case.shape.len() as u8)).unwrap();
        assert!(!golden.features.is_empty());
        if mlx {
            assert_eq!(selected, settings.features, "{filename}");
        }
        for (key, value) in &golden.features {
            assert!(
                keys.contains(key) && selected.contains(key),
                "{filename} {key}"
            );
            match (mode, value) {
                (Mode::Roi, FeatureValue::Scalar(v)) => {
                    v.value();
                }
                (Mode::Voxel, FeatureValue::Map(values)) => {
                    assert_eq!(
                        values.len(),
                        case.shape.iter().product::<usize>(),
                        "{filename} {key}"
                    );
                }
                _ => panic!("{filename} {key} has the wrong mode"),
            }
        }
    }
    assert!(inventory.is_empty(), "missing goldens: {inventory:?}");
}

#[test]
fn digital_reference_preserves_tags_and_published_tolerances() {
    let table: IbsiTable = read(&data_dir().join("ibsi_1_reference.json"));
    assert_eq!(table.rows.len(), 487);
    let catalog: BTreeMap<_, _> = features().iter().map(|f| (f.key(), f.ibsi_id)).collect();
    let mut tags = BTreeSet::new();
    let mut mapped = BTreeSet::new();
    for row in &table.rows {
        assert!(tags.insert(&row.tag), "{}", row.tag);
        assert!(!row.family.is_empty() && !row.name.is_empty());
        if let Some(key) = &row.key {
            assert!(
                mapped.insert((&row.family, key)),
                "duplicate IBSI mapping for {}",
                row.tag
            );
            assert_eq!(catalog[key], row.ibsi_id.as_deref(), "{}", row.tag);
        }
        assert_eq!(row.value.is_some(), row.tolerance.is_some(), "{}", row.tag);
        assert!(row.tolerance.is_none_or(|v| v >= 0.));
    }
    let phantom: Fixture = read(&data_dir().join("cases/ibsi_digital.json"));
    let count = phantom
        .arrays()
        .1
        .iter()
        .filter(|v| **v == phantom.label)
        .count();
    let volume = count as f64 * phantom.spacing.iter().product::<f64>();
    let row = table
        .rows
        .iter()
        .find(|r| r.tag == "morph_vol_approx")
        .unwrap();
    assert_eq!(row.accepts(volume), Some(true));
}
