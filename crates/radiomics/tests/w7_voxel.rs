//! Voxel-map behaviour through the public `extract`: refusals with the mlx messages, stream
//! batches against retained maps, and the scratch-budget refusals of mlx-radiomics.
//! CPU numerical work; rayon over centres.

use indexmap::IndexMap;
use ndarray::{Array2, ArrayD, ArrayView1, ArrayView2, Dimension, IxDyn};
use radiomics::settings::DirectionalAggregation;
use radiomics::{
    ExtractionResult, FeatureValue, HostArray, MapDtype, Mode, Output, Settings, TransformSettings,
    extract, list_features,
};

fn lcg(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state >> 33
}

fn image(shape: &[usize], seed: u64, levels: bool) -> ArrayD<f64> {
    let mut state = seed;
    ArrayD::from_shape_fn(IxDyn(shape), |_| {
        let v = (lcg(&mut state) % 7) as f64 * 25.0 + (lcg(&mut state) % 25) as f64;
        if levels {
            (v / 25.0).floor() * 25.0
        } else {
            v - 80.0
        }
    })
}

/// Two blobs with a hole plus one isolated voxel, so windows see gaps and singletons.
fn roi(shape: &[usize]) -> ArrayD<bool> {
    let ndim = shape.len();
    ArrayD::from_shape_fn(IxDyn(shape), |at| {
        let a = |i: usize| at[i];
        let (y, x) = (a(ndim - 2), a(ndim - 1));
        let first = y < 3 && x < 4 && !(y == 1 && x == 1);
        let second = y >= shape[ndim - 2] - 3 && x >= 2 && x < shape[ndim - 1] - 1;
        let lone = y == 4 && x == shape[ndim - 1] - 1;
        let depth = ndim == 2 || a(0) != 0;
        depth && (first || second || lone)
    })
}

const TEXTURE_FAMILIES: [&str; 6] = ["glcm", "glrlm", "glszm", "gldzm", "gldm", "ngtdm"];

fn names(families: &[&str], ndim: usize) -> Vec<String> {
    list_features(families, Mode::Voxel, Some(ndim as u8)).unwrap()
}

fn maps(
    image: &ArrayD<f64>,
    mask: &ArrayD<bool>,
    settings: &Settings,
    spacing: &[f64],
    dtype: MapDtype,
) -> radiomics::Result<ExtractionResult<FeatureValue>> {
    extract(
        &HostArray::from(image.clone()),
        &HostArray::from(mask.mapv(u8::from)),
        settings,
        &TransformSettings::default(),
        Some(spacing),
        Output::Maps {
            dtype,
            max_bytes: None,
        },
    )
}

fn map32(value: &FeatureValue) -> &ArrayD<f32> {
    let FeatureValue::Map32(map) = value else {
        panic!("expected a float32 map")
    };
    map
}

#[test]
fn stream_batches_cover_every_centre_once_in_order_and_match_the_retained_maps() {
    let shape = [5usize, 6, 7];
    let img = image(&shape, 5, true);
    let mask = roi(&shape);
    let mut settings = Settings {
        bin_width: 25.0,
        kernel_radius: 6,
        scratch_mib: 1,
        features: vec![
            "firstorder_Mean".into(),
            "glcm_Contrast".into(),
            "ngtdm_Strength".into(),
        ],
        ..Settings::default()
    };
    let (host_image, host_mask) = (
        HostArray::from(img.clone()),
        HostArray::from(mask.mapv(u8::from)),
    );
    let stream = |settings: &Settings, sink: &mut dyn radiomics::VoxelSink| {
        extract(
            &host_image,
            &host_mask,
            settings,
            &TransformSettings::default(),
            Some(&[1.0; 3]),
            Output::Stream {
                dtype: MapDtype::Float32,
                sink,
            },
        )
    };
    let error = stream(&settings, &mut |_: ArrayView2<'_, i32>,
                                        _: &IndexMap<
        String,
        ArrayView1<'_, f32>,
    >| {
        panic!("a refused window reached the sink")
    })
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("scratch_mib cannot hold one local histogram")
    );
    settings.scratch_mib = 2;
    let mut batches: Vec<(Array2<i32>, Vec<Vec<f32>>)> = Vec::new();
    let mut sink = |centers: ArrayView2<'_, i32>,
                    columns: &IndexMap<String, ArrayView1<'_, f32>>| {
        batches.push((
            centers.to_owned(),
            columns.values().map(|column| column.to_vec()).collect(),
        ));
        Ok(())
    };
    let streamed = stream(&settings, &mut sink).unwrap();
    assert_eq!(streamed.metadata.batch_size, Some(1));
    // mlx-radiomics v1.1.0 groups seven first-order rows at this budget.
    assert_eq!(streamed.metadata.scratch_estimate_bytes, Some(1_945_728));
    assert!(batches.len() > 1);
    assert!(batches.iter().all(|(c, v)| c.nrows() <= 1
        && v.len() == 3
        && v.iter().all(|col| col.len() == c.nrows())));
    let delivered = ndarray::concatenate(
        ndarray::Axis(0),
        &batches.iter().map(|(c, _)| c.view()).collect::<Vec<_>>(),
    )
    .unwrap();
    let expected: Vec<i32> = mask
        .indexed_iter()
        .filter(|(_, m)| **m)
        .flat_map(|(at, _)| at.slice().iter().map(|&c| c as i32).collect::<Vec<_>>())
        .collect();
    assert_eq!(delivered.iter().copied().collect::<Vec<_>>(), expected);
    let retained = maps(&img, &mask, &settings, &[1.0; 3], MapDtype::Float32).unwrap();
    // A 256 MiB delivery batch covers the whole ROI and needs no first-order cache.
    let large = Settings {
        scratch_mib: 256,
        ..settings.clone()
    };
    let ungrouped = maps(&img, &mask, &large, &[1.0; 3], MapDtype::Float32).unwrap();
    for ((key, small), (_, large)) in retained.features.iter().zip(&ungrouped.features) {
        assert!(
            map32(small)
                .iter()
                .zip(map32(large))
                .all(|(a, b)| a.to_bits() == b.to_bits()),
            "{key}"
        );
    }
    let mut row = 0;
    for (centers, columns) in &batches {
        for r in 0..centers.nrows() {
            let at = [
                centers[[r, 0]] as usize,
                centers[[r, 1]] as usize,
                centers[[r, 2]] as usize,
            ];
            for (f, (_, map)) in retained.features.iter().enumerate() {
                assert_eq!(map32(map)[at].to_bits(), columns[f][r].to_bits());
            }
            row += 1;
        }
    }
    assert_eq!(row, mask.iter().filter(|&&m| m).count());
}

#[test]
fn refusals_use_mlx_messages() {
    let shape = [4usize, 5];
    let img = image(&shape, 11, true);
    let mask = roi(&shape);
    let settings = Settings {
        bin_width: 25.0,
        features: vec!["glcm_Contrast".into()],
        ..Settings::default()
    };
    let run = |img: &ArrayD<f64>, mask: &ArrayD<bool>, settings: &Settings, spacing: &[f64]| {
        maps(img, mask, settings, spacing, MapDtype::Float32)
    };
    assert!(run(&img, &mask, &settings, &[1.0; 2]).is_ok());

    // Singleton ROI.
    let single = ArrayD::from_shape_fn(IxDyn(&shape), |at| at[0] == 1 && at[1] == 1);
    let error = run(&img, &single, &settings, &[1.0; 2]).unwrap_err();
    assert_eq!(
        error.to_string(),
        "voxel extraction requires at least two selected voxels"
    );

    // Non-finite sampled intensity.
    let mut nan = img.clone();
    let at = mask.indexed_iter().find(|(_, m)| **m).unwrap().0;
    nan[&at] = f64::NAN;
    assert!(run(&nan, &mask, &settings, &[1.0; 2]).is_err());

    // One centre's working set exceeds the scratch budget.
    let large = Settings {
        kernel_radius: 400,
        scratch_mib: 1,
        ..settings.clone()
    };
    let error = run(&img, &mask, &large, &[1.0; 2]).unwrap_err();
    assert_eq!(
        error.to_string(),
        "scratch_mib cannot hold one local histogram; reduce bins/kernel radius"
    );

    // Families without voxel maps and unknown names.
    for key in [
        "shape_Sphericity",
        "localintensity_LocalIntensityPeak",
        "glcm_Nope",
    ] {
        let bad = Settings {
            features: vec![key.into()],
            ..settings.clone()
        };
        assert!(run(&img, &mask, &bad, &[1.0; 2]).is_err(), "{key}");
    }
    // Spacing is checked.
    assert!(run(&img, &mask, &settings, &[1.0; 3]).is_err());
}

#[test]
fn constant_images_give_one_value_in_every_full_window() {
    // mlx's "flat" case: a constant background has one gray level everywhere, so every
    // unclipped window of the default radius gives the same value.
    let shape = [7usize, 7, 7];
    let img = ArrayD::from_elem(IxDyn(&shape), 42.0);
    let mask = ArrayD::from_elem(IxDyn(&shape), true);
    let settings = Settings {
        bin_width: 25.0,
        features: names(&TEXTURE_FAMILIES, 3),
        ..Settings::pyradiomics_conventions()
    };
    let result = maps(&img, &mask, &settings, &[1.0; 3], MapDtype::Float32).unwrap();
    assert_eq!(result.features.len(), settings.features.len());
    for (key, map) in &result.features {
        let map = map32(map);
        let reference = map[[3, 3, 3]];
        for z in 2..5 {
            for y in 2..5 {
                for x in 2..5 {
                    let value = map[[z, y, x]];
                    assert!(
                        value.to_bits() == reference.to_bits()
                            || value.is_nan() && reference.is_nan(),
                        "{key} [{z}, {y}, {x}]"
                    );
                }
            }
        }
    }
}

/// The three scratch-budget refusals of mlx-radiomics v1.1.0 `tests/test_sparse_glcm_voxels.py`,
/// through the public extraction entry point on its fractional-intensity lattice.
#[test]
fn upstream_sparse_lattice_refusals_hold_through_public_extraction() {
    let bins = [
        16_001., 16_002., 16_003., 16_004., 16_005., 16_006., 24_001., 25_001.,
    ];
    let image = ArrayD::from_shape_vec(
        IxDyn(&[2, 2, 2]),
        bins.iter().map(|b| (b - 0.5) * 0.05).collect(),
    )
    .unwrap();
    let mask = ArrayD::from_elem(IxDyn(&[2, 2, 2]), true);
    let run = |features: &[&str], mode: DirectionalAggregation, scratch_mib: u32| {
        let settings = Settings {
            features: features.iter().map(|f| f.to_string()).collect(),
            bin_width: 0.05,
            bin_anchor: Some(0.0),
            glcm_aggregation: mode,
            slice_axis: 0,
            scratch_mib,
            ..Settings::default()
        };
        extract(
            &image.clone().into(),
            &mask.clone().into(),
            &settings,
            &TransformSettings::default(),
            None,
            Output::Maps {
                dtype: MapDtype::Float64,
                max_bytes: None,
            },
        )
    };
    let refusal = "scratch_mib cannot hold one local histogram";
    let shade = ["glcm_Contrast", "glcm_ClusterShade", "glcm_Idmn"];
    let mixed = [
        "glcm_Contrast",
        "glcm_ClusterShade",
        "glcm_Idmn",
        "gldm_SmallDependenceEmphasis",
    ];
    let native = DirectionalAggregation::Native;
    assert!(
        run(&shade, native, 1)
            .unwrap_err()
            .to_string()
            .contains(refusal)
    );
    assert!(run(&shade, native, 128).is_ok());
    for mode in [
        DirectionalAggregation::Merge3D,
        DirectionalAggregation::SliceMerge2D,
    ] {
        assert!(run(&["glcm_Contrast"], mode, 1).is_ok());
        assert!(
            run(&mixed, mode, 1)
                .unwrap_err()
                .to_string()
                .contains(refusal)
        );
        assert!(
            run(&["glcm_Contrast", "gldm_SmallDependenceEmphasis"], mode, 1)
                .unwrap_err()
                .to_string()
                .contains(refusal)
        );
    }
}
