//! Every voxel map preserves its binary64 samples across thread counts. Exercise
//! multiple blocks, clipped edges, a sparse mask, and both benchmark radii.
use ndarray::Array3;
use radiomics::MapDtype;
use radiomics::io::HostArray;
use radiomics::{FeatureValue, Mode, Output, Settings, TransformSettings, extract, list_features};
use rayon::ThreadPoolBuilder;

#[test]
fn all_maps_have_identical_bits_on_one_thread_and_the_default_pool() {
    let shape = (9, 8, 17);
    let image = HostArray::I16(
        Array3::from_shape_fn(shape, |(z, y, x)| {
            ((z * 13 + y * x + 3 * x) % 111) as i16 - 54
        })
        .into_dyn(),
    );
    let mask = HostArray::U8(
        Array3::from_shape_fn(shape, |(z, y, x)| u8::from((z + 7 * y + 2 * x) % 13 != 0))
            .into_dyn(),
    );
    let single = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    for radius in [1, 2] {
        let settings = Settings {
            kernel_radius: radius,
            scratch_mib: 1,
            features: list_features(&[], Mode::Voxel, Some(3)).unwrap(),
            ..Default::default()
        };
        let run = || {
            extract(
                &image,
                &mask,
                &settings,
                &TransformSettings::default(),
                None,
                Output::Maps {
                    dtype: MapDtype::Float64,
                    max_bytes: None,
                },
            )
            .unwrap()
        };
        let one = single.install(run);
        let many = run();
        assert_eq!(one.features.len(), 147);
        for ((key, a), (other, b)) in one.features.iter().zip(&many.features) {
            assert_eq!(key, other);
            let (FeatureValue::Map64(a), FeatureValue::Map64(b)) = (a, b) else {
                panic!("expected float64 maps")
            };
            assert!(
                a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits()),
                "{key}, radius {radius}"
            );
        }
    }
}
