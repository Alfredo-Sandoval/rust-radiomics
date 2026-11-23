//! Derived images do not depend on the thread count: every filter gives the same feature
//! bits in a one-thread pool and in the default pool.
#![allow(missing_docs)]

use ndarray::{ArrayD, IxDyn};
use radiomics::io::HostArray;
use radiomics::settings::{
    FilterPadding, FourierGrid, GaborResponse, LbpMethod, LogKernel, OrientationPooling, RieszBase,
};
use radiomics::{
    FeatureValue, ImageType, Mode, Output, Settings, TransformSettings, extract, list_features,
};
use std::f64::consts::PI;

const SHAPE: [usize; 3] = [9, 22, 25];

fn image() -> ArrayD<f64> {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    ArrayD::from_shape_fn(IxDyn(&SHAPE), |index| {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let noise = (state >> 40) as f64 / (1u64 << 24) as f64;
        100.0 * (0.3 * index[2] as f64).sin()
            + 60.0 * (0.2 * index[1] as f64 + 0.5 * index[0] as f64).cos()
            + 30.0 * noise
            + 200.0
    })
}

fn roi() -> ArrayD<bool> {
    ArrayD::from_shape_fn(IxDyn(&SHAPE), |i| {
        (2..7).contains(&i[0]) && (4..18).contains(&i[1]) && (5..20).contains(&i[2])
    })
}

fn settings(kind: ImageType) -> TransformSettings {
    let mut t = TransformSettings {
        image_types: vec![kind],
        ..TransformSettings::default()
    };
    match kind {
        ImageType::Log => t.log.sigma = vec![1.5],
        ImageType::Mean => t.mean.support = 5,
        ImageType::Laws => {
            t.laws.kernel = "L5E5E5".into();
            t.laws.pooling = OrientationPooling::Max;
            t.laws.energy_distance = Some(2);
        }
        ImageType::Gabor => {
            t.gabor.sigma = vec![3.0];
            t.gabor.wavelength = vec![2.0];
            t.gabor.gamma = 1.5;
            t.gabor.theta = (0..4).map(|k| f64::from(k) * PI / 4.0).collect();
        }
        ImageType::SeparableWavelet => {
            t.separable_wavelet.bands = vec!["LHH".into()];
            t.separable_wavelet.pooling = OrientationPooling::Average;
        }
        ImageType::Simoncelli => t.simoncelli.grid = FourierGrid::Centered,
        ImageType::Riesz => {
            t.riesz.orders = vec![vec![1, 0, 0], vec![0, 1, 1]];
            t.riesz.log_sigma = vec![1.5];
        }
        _ => {}
    }
    t
}

/// Bits of every feature computed on every derived image of one configuration.
fn derived(options: &TransformSettings) -> Vec<(String, u64)> {
    let settings = Settings {
        features: list_features(&["firstorder"], Mode::Roi, Some(3)).unwrap(),
        ..Settings::default()
    };
    extract(
        &HostArray::F64(image()),
        &HostArray::U8(roi().mapv(u8::from)),
        &settings,
        options,
        Some(&[1.0, 1.0, 1.0]),
        Output::Roi,
    )
    .expect("derived-image features")
    .features
    .into_iter()
    .map(|(name, value)| match value {
        FeatureValue::Scalar(x) => (name, x.to_bits()),
        _ => unreachable!(),
    })
    .collect()
}

fn all_configurations() -> Vec<(String, TransformSettings)> {
    let mut out = Vec::new();
    for kind in [
        ImageType::Square,
        ImageType::SquareRoot,
        ImageType::Logarithm,
        ImageType::Exponential,
        ImageType::Gradient,
        ImageType::Wavelet,
        ImageType::Log,
        ImageType::Lbp2D,
        ImageType::Lbp3D,
        ImageType::Mean,
        ImageType::Laws,
        ImageType::Gabor,
        ImageType::Simoncelli,
        ImageType::Riesz,
        ImageType::SeparableWavelet,
    ] {
        out.push((kind.as_str().to_string(), settings(kind)));
    }
    let mut t = settings(ImageType::Gabor);
    t.gabor.slice_axis = Some(0);
    t.gabor.response = GaborResponse::Real;
    t.gabor.pooling = OrientationPooling::Average;
    t.gabor.padding = FilterPadding::Periodic;
    out.push(("gabor planar".into(), t));
    let mut t = settings(ImageType::Log);
    t.log.kernel = LogKernel::Sampled;
    t.log.padding = FilterPadding::Nearest;
    out.push(("log sampled".into(), t));
    let mut t = settings(ImageType::Riesz);
    t.riesz.base = RieszBase::Simoncelli;
    t.riesz.simoncelli_levels = vec![1];
    t.riesz.padding = FilterPadding::Periodic;
    out.push(("riesz simoncelli".into(), t));
    let mut t = settings(ImageType::Lbp2D);
    t.lbp2d.method = LbpMethod::Variance;
    out.push(("lbp-2D var".into(), t));
    out
}

#[test]
fn derived_images_are_bit_identical_across_thread_counts() {
    let one = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    for (name, options) in all_configurations() {
        let serial = one.install(|| derived(&options));
        let parallel = derived(&options);
        assert_eq!(serial, parallel, "{name}");
    }
}
