//! Run `validation/compare.py` on Rust-written schema-2 reports for every W11 golden.
//! CPU orchestration: exports through `export` (voxel) and writes the ROI scalars returned by
//! `extract` into a schema-2 report (ROI), then one Python process per golden. Needs NiBabel to read maps, so it runs only when
//! `W8_COMPARE_PYTHON` names an interpreter that has it; otherwise it prints a notice and
//! passes without comparing.
//! Status: both modes, all 96 MLX records of the W11 corpus.

mod parity_support;

use crate::parity_support::{Fixture, Golden, GoldenMode, data_dir, read, records};
use radiomics::io::HostArray;
use radiomics::{ExportOptions, FeatureValue, Output, TransformSettings, export, extract};
use std::path::Path;
use std::process::Command;

/// A scalar as the schema-2 report writes it: non-finite values are strings.
fn report_number(value: f64) -> serde_json::Value {
    if value.is_nan() {
        "NaN".into()
    } else if value.is_infinite() {
        if value > 0.0 { "Infinity" } else { "-Infinity" }.into()
    } else {
        value.into()
    }
}

#[test]
fn rust_reports_pass_parity_compare_against_every_mlx_golden() {
    let Some(python) = std::env::var_os("W8_COMPARE_PYTHON") else {
        eprintln!("W8_COMPARE_PYTHON not set: skipping validation/compare.py");
        return;
    };
    let compare = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../validation/compare.py");
    let root = std::env::temp_dir().join(format!("radiomics-w8-compare-{}", std::process::id()));
    let mut compared = 0;
    let (mut samples, mut worst_abs, mut worst_rel) = (0usize, 0f64, 0f64);
    for path in records("golden") {
        if !path.to_string_lossy().ends_with(".mlx.json") {
            continue;
        }
        let golden: Golden = read(&path);
        if golden.expected_error.is_some() {
            continue;
        }
        let case: Fixture = read(
            &data_dir()
                .join("cases")
                .join(format!("{}.json", golden.case)),
        );
        let (pixels, labels) = case.arrays();
        let settings = golden.settings();
        let out = root.join(path.file_stem().unwrap());
        let report = match golden.mode {
            GoldenMode::Voxel => {
                let saved = export(
                    &HostArray::F64(pixels),
                    &HostArray::I64(labels),
                    &out,
                    &ExportOptions {
                        settings: Some(settings),
                        spacing: Some(case.spacing.clone()),
                        allow_synthetic_geometry: true,
                        ..Default::default()
                    },
                )
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                saved.report
            }
            GoldenMode::Roi => {
                let result = extract(
                    &HostArray::F64(pixels),
                    &HostArray::I64(labels),
                    &settings,
                    &TransformSettings::default(),
                    Some(&case.spacing),
                    Output::Roi,
                )
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                std::fs::create_dir_all(&out).unwrap();
                let report = out.join("features.json");
                let document = serde_json::json!({
                    "schema_version": 2,
                    "mode": "roi",
                    "features": result.features.iter().map(|(key, value)| {
                        let FeatureValue::Scalar(x) = value else {
                            panic!("{key}: expected a scalar")
                        };
                        (key.clone(), report_number(*x))
                    }).collect::<serde_json::Map<_, _>>(),
                });
                std::fs::write(&report, serde_json::to_vec(&document).unwrap()).unwrap();
                report
            }
        };
        let output = Command::new(&python)
            .arg(&compare)
            .arg(&report)
            .arg(&path)
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success(),
            "{}:\n{text}\n{}",
            path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        // Last line: "... N compared values, ... max absolute error A, max relative error R".
        let summary = text.lines().last().unwrap();
        let number = |after: &str| -> f64 {
            summary
                .split(after)
                .nth(1)
                .unwrap()
                .split(&[',', ' '][..])
                .next()
                .unwrap()
                .parse()
                .unwrap()
        };
        samples += summary
            .split(", ")
            .nth(1)
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        worst_abs = worst_abs.max(number("max absolute error "));
        worst_rel = worst_rel.max(number("max relative error "));
        compared += 1;
    }
    std::fs::remove_dir_all(&root).ok();
    eprintln!(
        "{compared} reports, {samples} values: max absolute error {worst_abs:.3e}, \
         max relative error {worst_rel:.3e} (infinite where a zero reference deviates within atol)"
    );
    assert!(compared > 80);
}
