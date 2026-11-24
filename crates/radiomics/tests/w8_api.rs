//! Ports of mlx's `test_api.py` cases that apply to the Rust public workflow: arrays, files
//! and streamed delivery share values, compact maps keep values and background, and sinks
//! see each image's lifecycle. CPU orchestration on the test thread.
//! Status: ROI, retained maps (float32 and float64) and streaming through a per-image factory.

use indexmap::IndexMap;
use ndarray::{ArrayD, ArrayView1, ArrayView2};
use radiomics::io::{read_image, read_mask};
use radiomics::{Error, ImageSettings, ImageType, Settings, TransformSettings};
use radiomics::{FeatureValue, Output, SinkFactory};
use radiomics::{MapDtype, VoxelSink};
use radiomics::{extract, extract_files};
use std::path::{Path, PathBuf};

fn w8() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/w8")
}

fn settings() -> Settings {
    Settings {
        features: vec!["firstorder_Mean".into(), "firstorder_Entropy".into()],
        bin_count: Some(8),
        ..Settings::default()
    }
}

fn transforms() -> TransformSettings {
    TransformSettings {
        image_types: vec![ImageType::Original, ImageType::Mean, ImageType::Square],
        ..Default::default()
    }
}

#[derive(Default)]
struct Collect {
    events: Vec<(String, String)>,
    maps: IndexMap<String, ArrayD<f32>>,
    shape: Vec<usize>,
    open: Option<String>,
}

struct Sink<'a>(&'a mut Collect);

impl VoxelSink for Sink<'_> {
    fn consume(
        &mut self,
        centers: ArrayView2<'_, i32>,
        features: &IndexMap<String, ArrayView1<'_, f32>>,
    ) -> radiomics::Result<()> {
        for (name, values) in features {
            let map = self
                .0
                .maps
                .entry(name.clone())
                .or_insert_with(|| ArrayD::zeros(self.0.shape.clone()));
            for (row, &value) in centers.rows().into_iter().zip(values) {
                let at: Vec<usize> = row.iter().map(|&c| c as usize).collect();
                map[at.as_slice()] = value;
            }
        }
        Ok(())
    }
}

impl SinkFactory for Collect {
    fn open(&mut self, image: &str) -> radiomics::Result<Box<dyn VoxelSink + '_>> {
        self.events.push(("open".into(), image.into()));
        self.open = Some(image.into());
        Ok(Box::new(Sink(self)))
    }

    fn close(&mut self, image: &str, error: Option<&Error>) -> radiomics::Result<()> {
        assert!(error.is_none());
        assert_eq!(self.open.take().as_deref(), Some(image));
        self.events.push(("close".into(), image.into()));
        Ok(())
    }
}

#[test]
fn array_file_and_streamed_features_share_values_and_geometry() {
    let (image_path, mask_path) = (
        w8().join("synth3d_image.nii.gz"),
        w8().join("synth3d_mask.nii.gz"),
    );
    let image = read_image(&image_path).unwrap();
    let mask = read_mask(&mask_path).unwrap();
    let (settings, transforms) = (settings(), transforms());
    let from_arrays = extract(
        &image.array,
        &mask.array,
        &settings,
        &transforms,
        Some(&image.spacing),
        Output::Roi,
    )
    .unwrap();
    let from_files = extract_files(
        &image_path,
        &mask_path,
        &settings,
        &ImageSettings::default(),
        &transforms,
        Output::Roi,
    )
    .unwrap();
    assert_eq!(from_arrays.features.len(), 6);
    assert_eq!(from_arrays.features, from_files.features);
    let geometry = from_files.metadata.image_geometry.as_ref().unwrap();
    assert_eq!(geometry.shape, [12, 14, 16]);
    assert_eq!(geometry.spacing, [2.0, 1.5, 1.5]);
    assert_eq!(geometry.origin, [3.0, -2.0, 5.0]);
    assert_eq!(geometry.direction, [1., 0., 0., 0., 1., 0., 0., 0., 1.]);
    assert_eq!(
        from_files.metadata.source_geometry.as_ref().unwrap(),
        geometry
    );
    // Retained compact maps keep the evaluated values and a zero background.
    let compact = extract_files(
        &image_path,
        &mask_path,
        &settings,
        &ImageSettings::default(),
        &transforms,
        Output::Maps {
            dtype: MapDtype::Float32,
            max_bytes: None,
        },
    )
    .unwrap();
    let wide = extract_files(
        &image_path,
        &mask_path,
        &settings,
        &ImageSettings::default(),
        &transforms,
        Output::Maps {
            dtype: MapDtype::Float64,
            max_bytes: None,
        },
    )
    .unwrap();
    assert_eq!(compact.metadata.map_dtype, Some(MapDtype::Float32));
    assert_eq!(wide.metadata.map_dtype, Some(MapDtype::Float64));
    let inside = mask.array.eq_label(1);
    for (key, found) in &compact.features {
        let (FeatureValue::Map32(small), FeatureValue::Map64(large)) = (found, &wide.features[key])
        else {
            panic!("{key}: map dtypes")
        };
        for ((&a, &b), &selected) in small.iter().zip(large.iter()).zip(inside.iter()) {
            assert_eq!(f64::from(a).to_bits(), b.to_bits(), "{key}");
            if !selected {
                assert_eq!(a, 0.0, "{key}");
            }
        }
    }
    // A per-image factory sees open and close once per image, in order, and the retained
    // result stays empty.
    let mut collect = Collect {
        shape: image.array.shape().to_vec(),
        ..Default::default()
    };
    let streamed = extract_files(
        &image_path,
        &mask_path,
        &settings,
        &ImageSettings::default(),
        &transforms,
        Output::PerImage {
            dtype: MapDtype::Float64,
            factory: &mut collect,
        },
    )
    .unwrap();
    assert!(streamed.features.is_empty());
    let events: Vec<_> = collect
        .events
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    assert_eq!(
        events,
        [
            ("open", "original"),
            ("close", "original"),
            ("open", "mean-M3-mirror"),
            ("close", "mean-M3-mirror"),
            ("open", "square"),
            ("close", "square"),
        ]
    );
    for (key, found) in &compact.features {
        let FeatureValue::Map32(expected) = found else {
            unreachable!()
        };
        assert_eq!(&collect.maps[key], expected, "{key}");
        assert_eq!(
            streamed.metadata.feature_validity[key].finite,
            streamed.metadata.roi_voxels
        );
    }
}

#[test]
fn roi_values_reject_invalid_requests_with_mlx_messages() {
    let image = read_image(&w8().join("synth3d_image.nii.gz")).unwrap();
    let mask = read_mask(&w8().join("synth3d_mask.nii.gz")).unwrap();
    let run = |settings: &Settings, transforms: &TransformSettings, spacing: &[f64], output| {
        extract(
            &image.array,
            &mask.array,
            settings,
            transforms,
            Some(spacing),
            output,
        )
        .unwrap_err()
        .to_string()
    };
    let maps = || Output::Maps {
        dtype: MapDtype::Float64,
        max_bytes: None,
    };
    let shape = Settings {
        features: vec!["shape_MeshVolume".into()],
        ..Settings::default()
    };
    assert_eq!(
        run(&shape, &transforms(), &image.spacing, maps()),
        "shape features are unavailable in voxel mode"
    );
    assert_eq!(
        run(&settings(), &transforms(), &[1.0, 1.0], Output::Roi),
        "spacing must contain one finite positive value per array axis"
    );
    let lbp = TransformSettings {
        image_types: vec![ImageType::Lbp3D],
        ..Default::default()
    };
    let unmasked = Settings {
        masked_kernel: false,
        ..settings()
    };
    assert_eq!(
        run(&unmasked, &lbp, &image.spacing, maps()),
        "pinned LBP3D leaves outside-ROI values undefined; unmasked voxels unsupported"
    );
    // A retained-map budget refuses before any extraction.
    let error = extract(
        &image.array,
        &mask.array,
        &settings(),
        &transforms(),
        Some(&image.spacing),
        Output::Maps {
            dtype: MapDtype::Float64,
            max_bytes: Some(1024),
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::Memory(_)), "{error}");
}
