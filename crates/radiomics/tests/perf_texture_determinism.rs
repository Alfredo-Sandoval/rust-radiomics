//! Texture features are bit-identical on one thread and on the default Rayon pool, for every
//! family and for every GLCM and GLRLM aggregation, on dense-count and sparse-count lattices.
//! CPU numerical work in float64 through the public workflow on a synthetic volume.
#![allow(missing_docs)]

use ndarray::ArrayD;
use radiomics::io::HostArray;
use radiomics::settings::DirectionalAggregation as Mode;
use radiomics::{FeatureValue, Output, TransformSettings, extract};
use radiomics::{Settings, features};

fn volume() -> (HostArray, HostArray) {
    let shape = [14usize, 21, 19];
    let mut state = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let image = ArrayD::from_shape_fn(shape.to_vec(), |idx| {
        let smooth = (idx[0] * 7 + idx[1] * 5 + idx[2] * 3) as f64;
        smooth + (next() % 4000) as f64 / 10.
    });
    let mask = ArrayD::from_shape_fn(shape.to_vec(), |idx| {
        let d = |i: usize, c: f64| (idx[i] as f64 - c).powi(2) / (c * c);
        u8::from(d(0, 6.5) + d(1, 10.) + d(2, 9.) < 0.9)
    });
    (HostArray::F64(image), HostArray::U8(mask))
}

fn texture_names() -> Vec<String> {
    features()
        .iter()
        .filter(|s| {
            matches!(
                s.family.as_str(),
                "glcm" | "glrlm" | "glszm" | "gldzm" | "gldm" | "ngtdm"
            )
        })
        .map(|s| s.key())
        .collect()
}

fn bits(settings: &Settings) -> Vec<(String, u64)> {
    let (image, mask) = volume();
    let result = extract(
        &image,
        &mask,
        settings,
        &TransformSettings::default(),
        None,
        Output::Roi,
    )
    .unwrap();
    result
        .features
        .into_iter()
        .map(|(k, v)| match v {
            FeatureValue::Scalar(x) => (k, x.to_bits()),
            _ => panic!("scalar expected"),
        })
        .collect()
}

#[test]
fn texture_is_identical_on_one_thread_and_many() {
    let one = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    let many = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let base = Settings {
        features: texture_names(),
        ..Settings::default()
    };
    let mut cases = vec![("native".to_string(), base.clone())];
    for mode in [
        Mode::Average2D,
        Mode::SliceMerge2D,
        Mode::DirectionMerge25D,
        Mode::Merge25D,
        Mode::Average3D,
        Mode::Merge3D,
    ] {
        cases.push((
            format!("{mode:?}"),
            Settings {
                glcm_aggregation: mode,
                glrlm_aggregation: mode,
                ..base.clone()
            },
        ));
    }
    cases.push((
        "force2d".into(),
        Settings {
            force2d_axis: Some(1),
            ..base.clone()
        },
    ));
    // More than 512 gray levels counts into the sparse maps on one thread.
    cases.push((
        "sparse".into(),
        Settings {
            bin_width: 1.0,
            scratch_mib: 2048,
            ..base.clone()
        },
    ));
    for (name, settings) in cases {
        let serial = one.install(|| bits(&settings));
        let parallel = many.install(|| bits(&settings));
        let default_pool = bits(&settings);
        assert_eq!(serial, parallel, "{name}: 1 thread vs 8 threads");
        assert_eq!(serial, default_pool, "{name}: 1 thread vs default pool");
    }
}
