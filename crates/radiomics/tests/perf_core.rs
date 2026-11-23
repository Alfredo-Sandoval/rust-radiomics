//! Thread-count determinism of the ROI engine, first-order maps and physical sphere sums.

use ndarray::{Array3, ArrayD};
use radiomics::io::HostArray;
use radiomics::{
    FeatureValue, MapDtype, Mode, Output, Settings, TransformSettings, extract, list_features,
};
use rayon::ThreadPoolBuilder;

fn run(
    image: &HostArray,
    mask: &HostArray,
    settings: &Settings,
    spacing: &[f64],
    output: Output<'_>,
) -> Vec<(String, Vec<u64>)> {
    let result = extract(
        image,
        mask,
        settings,
        &TransformSettings::default(),
        Some(spacing),
        output,
    )
    .unwrap();
    result
        .features
        .into_iter()
        .map(|(key, value)| {
            let words = match value {
                FeatureValue::Scalar(x) => vec![x.to_bits()],
                FeatureValue::Map32(map) => map.iter().map(|v| u64::from(v.to_bits())).collect(),
                _ => unreachable!(),
            };
            (key, words)
        })
        .collect()
}

#[test]
fn core_results_identical_in_one_and_default_pool() {
    let image = Array3::from_shape_fn((20, 40, 48), |(z, y, x)| {
        ((x * 5 + y * 3 + z * 7) % 53) as i16 - 25
    });
    let mask = image.mapv(|v| u8::from(v % 5 != 0));
    let image = HostArray::I16(image.into_dyn());
    let mask = HostArray::U8(mask.into_dyn());
    let spacing = [1.1, 1.5, 1.9];
    let roi = Settings {
        features: list_features(&["firstorder", "localintensity"], Mode::Roi, Some(3)).unwrap(),
        ..Settings::default()
    };
    let maps = Settings {
        features: list_features(&["firstorder"], Mode::Voxel, Some(3)).unwrap(),
        ..Settings::default()
    };
    let all = || {
        let scalars = run(&image, &mask, &roi, &spacing, Output::Roi);
        let maps = run(
            &image,
            &mask,
            &maps,
            &spacing,
            Output::Maps {
                dtype: MapDtype::Float32,
                max_bytes: None,
            },
        );
        (scalars, maps)
    };
    let serial = ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap()
        .install(all);
    assert_eq!(serial, all());
}

#[test]
fn integer_sphere_rows_ignore_the_memory_layout() {
    let shape = (33, 34, 35);
    let data = Array3::from_shape_fn(shape, |(z, y, x)| ((z + 3 * y + 7 * x) % 39) as f64 - 17.);
    let roi = data.mapv(|x| u8::from(x != 0.));
    let settings = Settings {
        features: vec![
            "localintensity_GlobalIntensityPeak".into(),
            "localintensity_LocalIntensityPeak".into(),
        ],
        ..Settings::default()
    };
    let compute = |image: ArrayD<f64>| {
        run(
            &HostArray::F64(image),
            &HostArray::U8(roi.clone().into_dyn()),
            &settings,
            &[1.2, 1.4, 2.5],
            Output::Roi,
        )
    };
    let image = data.into_dyn();
    let strided = image
        .clone()
        .reversed_axes()
        .as_standard_layout()
        .to_owned()
        .reversed_axes();
    assert!(strided.as_slice().is_none());
    assert_eq!(compute(image), compute(strided));
}

#[test]
fn sparse_full_grid_search_has_identical_geometry_across_threads() {
    let dims = (64, 256, 512);
    let mut image = Array3::<i16>::zeros(dims);
    let mut labels = Array3::<u8>::zeros(dims);
    for (z, y, x, v) in [(60, 245, 465, 2), (61, 246, 466, 9), (62, 247, 467, -8)] {
        image[[z, y, x]] = v;
        labels[[z, y, x]] = 7;
    }
    let (image, mask) = (
        HostArray::I16(image.into_dyn()),
        HostArray::U8(labels.into_dyn()),
    );
    let settings = Settings {
        label: 7,
        features: vec!["firstorder_Mean".into()],
        ..Settings::default()
    };
    let run = || {
        let result = extract(
            &image,
            &mask,
            &settings,
            &TransformSettings::default(),
            None,
            Output::Roi,
        )
        .unwrap();
        let FeatureValue::Scalar(mean) = result.features["original_firstorder_Mean"] else {
            unreachable!()
        };
        (
            result.metadata.roi_voxels,
            result.metadata.excluded_voxels,
            mean.to_bits(),
        )
    };
    let serial = ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap()
        .install(run);
    assert_eq!(serial, (3, 64 * 256 * 512 - 3, 1f64.to_bits()));
    assert_eq!(run(), serial);
}
