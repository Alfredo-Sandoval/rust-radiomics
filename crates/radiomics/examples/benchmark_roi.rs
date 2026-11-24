//! In-memory, original-image ROI benchmark. Build with `--release`.
//! Read both images once, warm the extraction, then time only the native CPU call.

use radiomics::io::{read_image, read_mask};
use radiomics::{Error, FeatureValue, Output, Result, Settings, TransformSettings, extract};
use std::hint::black_box;
use std::path::Path;
use std::time::{Duration, Instant};

/// A scalar as the schema-2 report writes it: non-finite values are strings.
fn json_number(value: f64) -> serde_json::Value {
    if value.is_nan() {
        "NaN".into()
    } else if value.is_infinite() {
        if value > 0.0 { "Infinity" } else { "-Infinity" }.into()
    } else {
        value.into()
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let usage = || {
        Error::InvalidInput("usage: benchmark_roi IMAGE MASK REPEATS|profile [FEATURE ...]".into())
    };
    let image = read_image(Path::new(&args.next().ok_or_else(usage)?))?;
    let mask = read_mask(Path::new(&args.next().ok_or_else(usage)?))?;
    let count = args.next().ok_or_else(usage)?;
    let profile = count == "profile";
    let repeats = if profile {
        1
    } else {
        count
            .parse::<usize>()
            .ok()
            .filter(|&n| n > 0)
            .ok_or_else(usage)?
    };
    let mut settings = Settings::default();
    let selected: Vec<_> = args.collect();
    if !selected.is_empty() {
        settings.features = selected;
    }
    let transforms = TransformSettings::default();
    let run = || {
        extract(
            &image.array,
            &mask.array,
            &settings,
            &transforms,
            Some(&image.spacing),
            Output::Roi,
        )
    };
    let reference = run()?;
    let mut elapsed = Vec::with_capacity(repeats);
    let start = Instant::now();
    while elapsed.len() < repeats || (profile && start.elapsed() < Duration::from_secs(6)) {
        let t = Instant::now();
        let result = black_box(run()?);
        elapsed.push(t.elapsed().as_secs_f64() * 1000.);
        drop(result);
    }
    elapsed.sort_by(f64::total_cmp);
    let count = elapsed.len();
    let median_ms = 0.5 * (elapsed[(count - 1) / 2] + elapsed[count / 2]);
    if profile {
        println!(
            "{}",
            serde_json::json!({"median_ms": median_ms, "runs": count, "feature_count": reference.features.len()})
        );
        return Ok(());
    }
    let values = reference
        .features
        .into_iter()
        .map(|(k, v)| match v {
            FeatureValue::Scalar(value) => Ok((k, json_number(value))),
            _ => Err(Error::Execution("expected an ROI scalar".into())),
        })
        .collect::<Result<serde_json::Map<_, _>>>()?;
    println!(
        "{}",
        serde_json::json!({"samples_ms":elapsed,"median_ms":median_ms,"features":values,"roi_voxels":reference.metadata.roi_voxels})
    );
    Ok(())
}
