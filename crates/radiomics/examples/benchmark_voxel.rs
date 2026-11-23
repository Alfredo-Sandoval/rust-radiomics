//! Warm, in-memory retained voxel maps through the public extraction entry point.
//! Arguments: IMAGE MASK REPEATS RADIUS FAMILY|all|defaults. Use "synthetic -" for
//! a deterministic 10.1-million-voxel volume. `defaults` selects the 93 default features.
//! Timings exclude input I/O and destruction of the retained result.

use ndarray::Array3;
use radiomics::io::{HostArray, read_image, read_mask};
use radiomics::{
    Error, MapDtype, Mode, Output, Result, Settings, TransformSettings, default_features, extract,
    list_features,
};
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

fn measure<T>(
    mut run: impl FnMut() -> Result<T>,
    repeats: usize,
    radius: u32,
    count: usize,
    roi_voxels: usize,
) -> Result<()> {
    drop(black_box(run()?));
    let mut samples = Vec::new();
    let start = Instant::now();
    while samples.len() < repeats || start.elapsed().as_secs() < 5 {
        let t = Instant::now();
        let result = black_box(run()?);
        samples.push(t.elapsed().as_secs_f64() * 1000.);
        drop(result);
    }
    samples.sort_by(f64::total_cmp);
    let n = samples.len();
    let median = (samples[(n - 1) / 2] + samples[n / 2]) * 0.5;
    println!(
        "{}",
        serde_json::json!({"median_ms":median,"repeats":n,"min_ms":samples[0],"max_ms":samples[n-1],"roi_voxels":roi_voxels,"maps":count,"radius":radius})
    );
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let usage = || {
        Error::InvalidInput(
            "usage: benchmark_voxel IMAGE MASK REPEATS RADIUS FAMILY|all|defaults".into(),
        )
    };
    if args.len() != 5 {
        return Err(usage());
    }
    let (image, mask, spacing) = if args[0] == "synthetic" {
        let shape = (101, 200, 500);
        let image = Array3::from_shape_fn(shape, |(z, y, x)| {
            ((z * 7 + y * 11 + x * 3 + x * y % 47) % 800) as i16 - 400
        });
        (
            HostArray::I16(image.into_dyn()),
            HostArray::U8(Array3::ones(shape).into_dyn()),
            vec![1.; 3],
        )
    } else {
        let image = read_image(Path::new(&args[0]))?;
        (
            image.array,
            read_mask(Path::new(&args[1]))?.array,
            image.spacing,
        )
    };
    let repeats: usize = args[2].parse().ok().filter(|&n| n > 0).ok_or_else(usage)?;
    let radius = args[3].parse().map_err(|_| usage())?;
    let ndim = Some(image.ndim() as u8);
    let features = match args[4].as_str() {
        "defaults" => default_features(),
        "all" => list_features(&[], Mode::Voxel, ndim)?,
        family => list_features(&[family], Mode::Voxel, ndim)?,
    };
    let settings = Settings {
        features,
        kernel_radius: radius,
        ..Default::default()
    };
    let transforms = TransformSettings::default();
    let run = || {
        extract(
            &image,
            &mask,
            &settings,
            &transforms,
            Some(&spacing),
            Output::Maps {
                dtype: MapDtype::Float64,
                max_bytes: Some(usize::MAX),
            },
        )
    };
    let voxels = mask.eq_label(settings.label).iter().filter(|&&v| v).count();
    let count = settings.features.len();
    measure(run, repeats, radius, count, voxels)
}
