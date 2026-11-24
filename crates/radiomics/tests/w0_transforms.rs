//! Direct construction and dimension checks have the same boundaries as parameter parsing.

use ndarray::{ArrayD, Dimension};
use radiomics::settings::*;
use radiomics::{
    Error, HostArray, ImageType, Output, Result, Settings, TransformSettings, extract,
};

/// Extract the mean over a small ROI, so only the transform settings can fail.
fn run(options: &TransformSettings, ndim: usize) -> Result<usize> {
    let shape = vec![5; ndim];
    let image = ArrayD::from_shape_fn(shape.clone(), |ix| ix.slice().iter().sum::<usize>() as f64);
    let mask = ArrayD::from_elem(shape, 1_u8);
    let settings = Settings {
        features: vec!["firstorder_Mean".into()],
        ..Settings::default()
    };
    let result = extract(
        &HostArray::from(image),
        &HostArray::from(mask),
        &settings,
        options,
        None,
        Output::Roi,
    )?;
    Ok(result
        .metadata
        .gray_levels_by_image
        .map_or(0, |images| images.len()))
}

#[test]
fn only_selected_filters_need_options_for_the_requested_dimension() {
    let mut options = TransformSettings {
        image_types: vec![ImageType::Laws],
        riesz: RieszSettings {
            orders: vec![vec![1, 0]],
            ..Default::default()
        },
        separable_wavelet: SeparableWaveletSettings {
            bands: vec!["LH".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    options.validate().unwrap();
    run(&options, 3).unwrap();
    options.image_types = vec![ImageType::SeparableWavelet];
    options.validate().unwrap();
    assert_eq!(
        run(&options, 3).unwrap_err().to_string(),
        "separable wavelet bands must have one letter per filtered axis"
    );
    options.separable_wavelet.planar = true;
    run(&options, 3).unwrap();
}

#[test]
fn invalid_nested_settings_are_rejected_even_with_original_only() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let log = LogSettings {
            sigma: vec![value],
            ..Default::default()
        };
        assert_eq!(
            log.validate().unwrap_err().to_string(),
            "sigma values must be finite and positive"
        );
        let s = TransformSettings {
            lbp3d: Lbp3DSettings {
                radius: value,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            s.validate().unwrap_err().to_string(),
            "LBP radii must be finite and positive"
        );
        assert_eq!(
            Lbp2DSettings {
                radius: value,
                ..Default::default()
            }
            .validate()
            .unwrap_err()
            .to_string(),
            "LBP radii must be finite and positive"
        );
    }
    for (t, msg) in [
        (
            TransformSettings {
                scratch_mib: 0,
                ..Default::default()
            },
            "scratch_mib must be a positive integer",
        ),
        (
            TransformSettings {
                image_types: vec![],
                ..Default::default()
            },
            "image_types must contain typed ImageType entries",
        ),
        (
            TransformSettings {
                image_types: vec![ImageType::Original; 2],
                ..Default::default()
            },
            "image_types must be unique",
        ),
        (
            TransformSettings {
                image_types: vec![ImageType::Log],
                ..Default::default()
            },
            "LoG requires explicit positive sigma values",
        ),
        (
            TransformSettings {
                simoncelli: SimoncelliSettings {
                    levels: vec![],
                    ..Default::default()
                },
                ..Default::default()
            },
            "simoncelli_levels must contain positive integers",
        ),
    ] {
        assert_eq!(t.validate().unwrap_err(), Error::InvalidInput(msg.into()));
    }
}

#[test]
fn gabor_singletons_broadcast_in_scale_order() {
    // One derived image per (sigma, wavelength) pair and orientation.
    for (sigma, wavelength, pairs) in [
        (vec![1.], vec![2., 3.], 2),
        (vec![2., 1.], vec![3.], 2),
        (vec![2., 1.], vec![3., 4.], 2),
    ] {
        let t = TransformSettings {
            image_types: vec![ImageType::Gabor],
            gabor: GaborSettings {
                sigma,
                wavelength,
                theta: vec![0.0],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(run(&t, 3).unwrap(), pairs);
    }
}

#[test]
fn selected_wavelets_must_name_a_packaged_wavelet() {
    // Messages from mlxradiomics v1.1.0 `TransformSettings(...)` with name="nope".
    let wavelet = TransformSettings {
        image_types: vec![ImageType::Wavelet],
        wavelet: WaveletSettings {
            name: "nope".into(),
            ..Default::default()
        },
        ..Default::default()
    };
    let separable = TransformSettings {
        image_types: vec![ImageType::SeparableWavelet],
        separable_wavelet: SeparableWaveletSettings {
            name: "nope".into(),
            bands: vec!["LLH".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    for options in [wavelet, separable] {
        assert_eq!(
            options.validate(),
            Err(Error::InvalidInput(
                "unsupported discrete wavelet: nope".into()
            ))
        );
    }
    let unselected = TransformSettings {
        wavelet: WaveletSettings {
            name: "nope".into(),
            ..Default::default()
        },
        ..Default::default()
    };
    unselected.validate().unwrap();
    TransformSettings {
        image_types: vec![ImageType::Wavelet],
        ..Default::default()
    }
    .validate()
    .unwrap();
}
