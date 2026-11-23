//! Shape, morphology, intensity-histogram and IVH values do not depend on the thread count.
#![allow(missing_docs)]

use ndarray::{ArrayD, IxDyn};
use radiomics::Settings;
use radiomics::io::HostArray;
use radiomics::{FeatureValue, Output, TransformSettings, extract};
use radiomics::{Mode, list_features};

/// An anisotropic blob with a notch, a hole and a stray voxel, over a smooth image.
fn blob() -> (HostArray, HostArray) {
    let shape = [18usize, 30, 34];
    let mut image = ArrayD::<f64>::zeros(IxDyn(&shape));
    let mut mask = ArrayD::<u8>::zeros(IxDyn(&shape));
    for (z, y, x) in (0..shape[0])
        .flat_map(|z| (0..shape[1]).flat_map(move |y| (0..shape[2]).map(move |x| (z, y, x))))
    {
        let (fz, fy, fx) = (z as f64 - 8.5, y as f64 - 14.0, x as f64 - 16.0);
        let radius = (fz / 7.0).powi(2) + (fy / 12.0).powi(2) + (fx / 14.0).powi(2);
        image[[z, y, x]] =
            40.0 * (0.7 * fx).sin() + 25.0 * (0.4 * fy + 0.9 * fz).cos() + (x + y) as f64;
        let notch = fz > 2.0 && fy > 4.0 && fx > 6.0;
        let hole = fz.abs() < 1.5 && fy.abs() < 2.5 && fx.abs() < 2.5;
        if (radius <= 1.0 && !notch && !hole) || (z, y, x) == (16, 28, 32) {
            mask[[z, y, x]] = 1;
        }
    }
    (HostArray::F64(image), HostArray::U8(mask))
}

fn run(features: &[String]) -> Vec<(String, u64)> {
    let (image, mask) = blob();
    let settings = Settings {
        features: features.to_vec(),
        bin_width: 5.0,
        ivh_bin_width: Some(2.5),
        ..Settings::default()
    };
    let result = extract(
        &image,
        &mask,
        &settings,
        &TransformSettings::default(),
        Some(&[2.5, 0.8, 0.7]),
        Output::Roi,
    )
    .expect("extraction succeeds");
    result
        .features
        .iter()
        .map(|(key, value)| match value {
            FeatureValue::Scalar(v) => (key.clone(), v.to_bits()),
            other => panic!("{key}: unexpected {other:?}"),
        })
        .collect()
}

#[test]
fn shape_and_histogram_values_are_bit_identical_across_thread_counts() {
    let mut features = Vec::new();
    for family in ["shape", "intensityhistogram", "ivh"] {
        features.extend(list_features(&[family], Mode::Roi, None).unwrap());
    }
    let single = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap()
        .install(|| run(&features));
    let many = rayon::ThreadPoolBuilder::new()
        .num_threads(7)
        .build()
        .unwrap()
        .install(|| run(&features));
    let default = run(&features);
    assert_eq!(single.len(), features.len());
    assert_eq!(single, many);
    assert_eq!(single, default);
}
