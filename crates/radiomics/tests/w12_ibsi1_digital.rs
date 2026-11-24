//! W12: IBSI 1 digital phantom through the public file API, scored like mlx-radiomics'
//! `scripts/qualify_ibsi_1.py` (482 rows; mlx v1.1.0 reports 481 passing and AEE area density as its one exception).
//! CPU numerical work through `extract_files`, one extraction per workbook aggregation.
//! The phantom and workbook rows are CC BY 4.0 (see THIRD_PARTY_NOTICES.md).
#![allow(missing_docs)]

mod w12_common;

use crate::w12_common::*;
use radiomics::{ImageSettings, Settings};
use std::path::PathBuf;

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/w12")
}

#[test]
fn digital_phantom_matches_the_published_ibsi_1_values() {
    let rows = read_rows(&data().join("digital_phantom.csv"));
    assert_eq!(rows.len(), 482, "scored rows in the pinned workbook export");
    // Unit bins from the minimum grey level (no anchor, default origin), planes along axis 0.
    let settings = Settings {
        bin_width: 1.0,
        slice_axis: 0,
        ..Settings::default()
    };
    let outcomes = qualify(
        true,
        &data().join("digital_phantom_image.nii.gz"),
        &data().join("digital_phantom_mask.nii.gz"),
        &settings,
        &ImageSettings::default(),
        &rows,
    );
    let counts = count(&outcomes);
    println!();
    print_table(&[("digital".into(), counts)]);
    print_failures("digital", &outcomes);
    record_rows("digital", &outcomes);
    assert_eq!(counts.compared, 482);
    assert_eq!(counts.failures, 0, "unexplained failures");
    let aee = outcomes
        .iter()
        .find(|o| o.tag == "morph_area_dens_aee")
        .unwrap();
    println!("AEE area density (ibsi-series default): {:.9}", aee.value);
}

#[test]
fn three_significant_digit_rounding_ties_to_even() {
    assert!(matches(1.2349, "1.23", "0", Rule::ThreeSignificant));
    assert!(matches(1.235, "1.24", "0", Rule::ThreeSignificant));
    assert!(matches(1.245, "1.24", "0", Rule::ThreeSignificant));
    assert!(matches(0.0012351, "0.00124", "0", Rule::ThreeSignificant));
    assert!(matches(999.5, "1.00e3", "0", Rule::ThreeSignificant));
    assert!(matches(-0.5195, "-0.52", "0", Rule::ThreeSignificant));
    assert!(!matches(1.2251, "1.22", "0", Rule::ThreeSignificant));
    assert!(matches(0.0, "0", "0", Rule::ThreeSignificant));
}

/// The corrected (converged) enclosing-ellipsoid area is not the published 1.36; this is
/// the named exception mlx records. The default `ibsi-series` area is the published one.
#[test]
fn aee_area_density_exact_area_is_the_named_exception() {
    let rows: Vec<Row> = read_rows(&data().join("digital_phantom.csv"))
        .into_iter()
        .filter(|r| r.tag == "morph_area_dens_aee")
        .collect();
    let settings = Settings {
        bin_width: 1.0,
        ellipsoid_area: radiomics::settings::EllipsoidArea::Exact,
        ..Settings::default()
    };
    let outcomes = qualify(
        true,
        &data().join("digital_phantom_image.nii.gz"),
        &data().join("digital_phantom_mask.nii.gz"),
        &settings,
        &ImageSettings::default(),
        &rows,
    );
    let o = &outcomes[0];
    println!(
        "AEE area density (exact): {:.9} vs {} +/- {}",
        o.value, o.reference, o.tolerance
    );
    assert!((o.value - 1.304356217).abs() < 1e-6, "{}", o.value);
    assert!(!o.declared);
}
