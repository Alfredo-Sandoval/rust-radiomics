//! ROI family parity against W11's MLX goldens, through the public `extract`.
//! CPU orchestration on each test thread.

mod parity_support;

use crate::parity_support::{
    FeatureValue as Reference, Fixture, Golden, GoldenMode, data_dir, mlx_accepts, read, records,
};
use radiomics::{FeatureValue, HostArray, Output, Settings, TransformSettings, extract};

fn check_family(accepts: impl Fn(&str) -> bool) {
    let mut count = 0;
    for path in records("golden") {
        if !path.to_str().unwrap().ends_with(".roi.mlx.json") {
            continue;
        }
        let golden: Golden = read(&path);
        assert_eq!(golden.mode, GoldenMode::Roi);
        let mut settings = golden.settings();
        settings.features.retain(|name| accepts(name));
        if settings.features.is_empty() {
            continue;
        }
        let case: Fixture = read(
            &data_dir()
                .join("cases")
                .join(format!("{}.json", golden.case)),
        );
        let (image, labels) = case.arrays();
        let result = run(image.into(), labels.into(), &settings, &case.spacing);
        match (&golden.expected_error, result) {
            (Some(expected), Err(error)) => {
                assert_eq!(error.to_string(), *expected, "{}", path.display());
            }
            (None, Ok(result)) => {
                assert_eq!(result.features.len(), settings.features.len());
                for (key, value) in &result.features {
                    let name = key.strip_prefix("original_").unwrap();
                    let (FeatureValue::Scalar(actual), Reference::Scalar(expected)) =
                        (value, &golden.features[name])
                    else {
                        panic!("{} {name}: expected a scalar", path.display())
                    };
                    assert!(
                        mlx_accepts(name, *actual, expected.value()),
                        "{} {name}: got {actual}, expected {}",
                        path.display(),
                        expected.value()
                    );
                    count += 1;
                }
            }
            (expected, result) => panic!(
                "{}: expected error {expected:?}, got {:?}",
                path.display(),
                result.map(|r| r.features.len())
            ),
        }
    }
    assert!(count > 0, "no matching MLX golden values");
}

fn run(
    image: HostArray,
    mask: HostArray,
    settings: &Settings,
    spacing: &[f64],
) -> radiomics::Result<radiomics::ExtractionResult<FeatureValue>> {
    extract(
        &image,
        &mask,
        settings,
        &TransformSettings::default(),
        Some(spacing),
        Output::Roi,
    )
}

macro_rules! family_test {
    ($test:ident, $prefix:literal) => {
        #[test]
        fn $test() {
            check_family(|key| key.starts_with($prefix));
        }
    };
}

family_test!(glcm, "glcm_");
family_test!(glrlm, "glrlm_");
family_test!(glszm, "glszm_");
family_test!(gldzm, "gldzm_");
family_test!(gldm, "gldm_");
family_test!(ngtdm, "ngtdm_");
family_test!(shape_2d, "shape2D_");
family_test!(shape_3d, "shape_");
family_test!(intensity_histogram_roi, "intensityhistogram_");
family_test!(ivh_roi, "ivh_");
family_test!(local_intensity, "localintensity_");
family_test!(first_order, "firstorder_");
