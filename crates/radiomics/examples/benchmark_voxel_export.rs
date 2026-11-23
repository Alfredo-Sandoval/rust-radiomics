//! Measure public NIfTI export of three sparse maps on a deterministic synthetic CT grid.
use ndarray::Array3;
use radiomics::HostArray;
use radiomics::{ExportOptions, Result, Settings, export};
use std::hint::black_box;
use std::path::Path;
use std::time::{Duration, Instant};

fn benchmark(dir: &Path) -> Result<()> {
    let shape = (48, 256, 256);
    let image = HostArray::I16(
        Array3::from_shape_fn(shape, |(z, y, x)| ((5 * z + 7 * y + x) % 250) as i16).into_dyn(),
    );
    let mask = HostArray::Bool(
        Array3::from_shape_fn(shape, |(z, y, x)| {
            (17..31).contains(&z) && (82..104).contains(&y) && (123..145).contains(&x)
        })
        .into_dyn(),
    );
    let options = ExportOptions {
        settings: Some(Settings {
            features: [
                "firstorder_Mean",
                "firstorder_Variance",
                "firstorder_Entropy",
            ]
            .map(str::to_owned)
            .to_vec(),
            ..Settings::default()
        }),
        allow_synthetic_geometry: true,
        overwrite: true,
        ..ExportOptions::default()
    };
    black_box(export(&image, &mask, dir, &options)?);
    let start = Instant::now();
    let mut times = Vec::new();
    while times.len() < 5 || start.elapsed() < Duration::from_secs(6) {
        let t = Instant::now();
        black_box(export(&image, &mask, dir, &options)?);
        times.push(t.elapsed().as_secs_f64() * 1000.);
    }
    times.sort_by(f64::total_cmp);
    let n = times.len();
    println!(
        "{}",
        serde_json::json!({"median_ms": (times[n/2]+times[(n-1)/2])*0.5, "runs":n,"maps":3})
    );
    Ok(())
}
fn main() -> Result<()> {
    let dir = std::env::temp_dir().join(format!(
        "radiomics-voxel-export-bench-{}",
        std::process::id()
    ));
    std::fs::create_dir(&dir)?;
    let result = benchmark(&dir);
    let cleanup = std::fs::remove_dir_all(dir);
    result?;
    Ok(cleanup?)
}
