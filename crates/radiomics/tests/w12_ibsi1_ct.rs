//! W12: IBSI 1 CT radiomics phantom, configurations A to E, through the public file API,
//! scored like mlx-radiomics' `scripts/qualify_ibsi_1.py`. mlx reports 346 rows each for
//! 2-D A and B and 210 each for 3-D C, D and E; all 1,322 pass the declared rules and
//! 1,321 are within the literal published tolerances.
//!
//! The phantom is CC BY-NC data and is never committed. Opt in with
//! `IBSI1_CT_PHANTOM` (directory with `image/phantom.nii.gz` and `mask/mask.nii.gz`) and
//! `IBSI1_REFERENCE_DIR` (directory with `config A.csv` to `config E.csv`). Without both,
//! the test prints a skip notice and passes.
#![allow(missing_docs)]

mod w12_common;

use crate::w12_common::*;
use radiomics::settings::ResegmentMode;
use radiomics::{IbsiInterpolation, ImageSettings, Interpolator, Settings};
use std::path::PathBuf;

/// Expected scored rows per configuration in the pinned workbook export.
const EXPECTED_ROWS: [(&str, usize); 5] =
    [("A", 346), ("B", 346), ("C", 210), ("D", 210), ("E", 210)];

fn interpolation(spacing: [f64; 3], interpolator: Interpolator) -> ImageSettings {
    let mut ibsi = IbsiInterpolation::new(spacing.to_vec());
    ibsi.image_interpolator = interpolator;
    ImageSettings {
        ibsi_interpolation: Some(ibsi),
        ..ImageSettings::default()
    }
}

fn configuration(name: &str) -> (ImageSettings, Settings) {
    let base = Settings {
        slice_axis: 0,
        ..Settings::default()
    };
    match name {
        // 2-D, no interpolation, [-500, 400] HU, 25 HU bins from the lower bound.
        "A" => (
            ImageSettings::default(),
            Settings {
                bin_width: 25.0,
                bin_anchor: Some(-500.0),
                resegment_range: Some(vec![-500.0, 400.0]),
                ..base
            },
        ),
        // 2-D, bilinear to 2 x 2 mm in plane, [-500, 400] HU, 32 bins.
        "B" => (
            interpolation([0.0, 2.0, 2.0], Interpolator::Linear),
            Settings {
                bin_count: Some(32),
                resegment_range: Some(vec![-500.0, 400.0]),
                ..base
            },
        ),
        // 3-D, trilinear to 2 mm, [-1000, 400] HU, 25 HU bins, IVH 2.5 HU.
        "C" => (
            interpolation([2.0; 3], Interpolator::Linear),
            Settings {
                bin_width: 25.0,
                bin_anchor: Some(-1000.0),
                resegment_range: Some(vec![-1000.0, 400.0]),
                ivh_bin_width: Some(2.5),
                ..base
            },
        ),
        // 3-D, trilinear to 2 mm, 3-sigma outlier filtering, 32 bins.
        "D" => (
            interpolation([2.0; 3], Interpolator::Linear),
            Settings {
                bin_count: Some(32),
                resegment_range: Some(vec![-3.0, 3.0]),
                resegment_mode: ResegmentMode::Sigma,
                ..base
            },
        ),
        // 3-D, tricubic spline to 2 mm, [-1000, 400] HU and 3-sigma filtering, 32 bins, IVH 1000 bins.
        "E" => (
            interpolation([2.0; 3], Interpolator::Bspline),
            Settings {
                bin_count: Some(32),
                resegment_range: Some(vec![-1000.0, 400.0]),
                resegment_outlier_sigma: Some(3.0),
                ivh_bin_count: Some(1000),
                ..base
            },
        ),
        other => panic!("unknown configuration {other}"),
    }
}

#[test]
fn ibsi_ct_phantom_configurations_match_the_published_values() {
    let (Some(phantom), Some(reference)) = (
        std::env::var_os("IBSI1_CT_PHANTOM"),
        std::env::var_os("IBSI1_REFERENCE_DIR"),
    ) else {
        eprintln!("skipped: set IBSI1_CT_PHANTOM and IBSI1_REFERENCE_DIR to run");
        return;
    };
    let (phantom, reference) = (PathBuf::from(phantom), PathBuf::from(reference));
    let (image, mask) = (
        phantom.join("image/phantom.nii.gz"),
        phantom.join("mask/mask.nii.gz"),
    );
    let mut table = Vec::new();
    let mut failures = Vec::new();
    for (name, expected) in EXPECTED_ROWS {
        let rows = read_rows(&reference.join(format!("config {name}.csv")));
        assert_eq!(rows.len(), expected, "configuration {name}: scored rows");
        let (image_settings, settings) = configuration(name);
        let outcomes = qualify(false, &image, &mask, &settings, &image_settings, &rows);
        let counts = count(&outcomes);
        print_failures(name, &outcomes);
        record_rows(&format!("config-{name}"), &outcomes);
        for o in outcomes.iter().filter(|o| o.declared && !o.literal) {
            println!(
                "  declared-rule pass outside the literal tolerance, {name} {} [{}]: {:.12e} vs {} +/- {} ({:?})",
                o.tag, o.feature, o.value, o.reference, o.tolerance, o.rule
            );
        }
        for o in outcomes.iter().filter(|o| !o.declared) {
            failures.push(format!(
                "{name} {}: {:.12e} vs {} +/- {}",
                o.tag, o.value, o.reference, o.tolerance
            ));
        }
        table.push((format!("config {name}"), counts));
    }
    println!();
    print_table(&table);
    let total = table.iter().fold((0, 0, 0), |a, (_, c)| {
        (a.0 + c.compared, a.1 + c.declared, a.2 + c.literal)
    });
    println!(
        "total: {} compared, {} pass the declared rules, {} within the literal tolerances",
        total.0, total.1, total.2
    );
    assert!(failures.is_empty(), "failures:\n{}", failures.join("\n"));
}
