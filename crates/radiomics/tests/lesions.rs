//! Connected lesions of one mask label are split, ordered and extracted as separate ROIs.

use radiomics::ndarray::ArrayD;
use radiomics::{
    Error, HostArray, LesionOptions, Output, Settings, TransformSettings, extract, extract_lesions,
    label_lesions,
};

fn keep(min_voxels: usize, max_lesions: Option<usize>) -> LesionOptions {
    LesionOptions {
        min_voxels,
        max_lesions,
    }
}

#[test]
fn lesions_join_at_corners_and_are_numbered_by_size_then_position() {
    let mut mask = ArrayD::from_elem(vec![4, 6, 6], 0_u8);
    let mut expected = ArrayD::from_elem(vec![4, 6, 6], 0_i32);
    // Three voxels in a row.
    for x in 3..6 {
        mask[[0, 0, x]] = 1;
        expected[[0, 0, x]] = 1;
    }
    // Two voxels that share only a corner.
    for at in [[0, 0, 0], [1, 1, 1]] {
        mask[at] = 1;
        expected[at] = 2;
    }
    // Two voxels, later in C order.
    for y in 4..6 {
        mask[[3, y, 0]] = 1;
        expected[[3, y, 0]] = 3;
    }
    // One voxel, and another label touching the row.
    mask[[3, 0, 5]] = 1;
    expected[[3, 0, 5]] = 4;
    mask[[0, 1, 4]] = 2;
    let mask = HostArray::from(mask);

    assert_eq!(label_lesions(&mask, 1, &keep(1, None)).unwrap(), expected);
    assert_eq!(
        label_lesions(&mask, 1, &keep(2, None)).unwrap(),
        expected.mapv(|v| if v == 4 { 0 } else { v })
    );
    assert_eq!(
        label_lesions(&mask, 1, &keep(1, Some(2))).unwrap(),
        expected.mapv(|v| if v > 2 { 0 } else { v })
    );
    let other = label_lesions(&mask, 2, &keep(1, None)).unwrap();
    assert_eq!((other.sum(), other[[0, 1, 4]]), (1, 1));
    assert_eq!(label_lesions(&mask, 1, &keep(4, None)).unwrap().sum(), 0);
    assert!(matches!(
        label_lesions(&mask, 3, &keep(1, None)),
        Err(Error::InvalidInput(message)) if message.contains("absent")
    ));
    assert!(matches!(
        label_lesions(&mask, 1, &keep(0, None)),
        Err(Error::InvalidInput(_))
    ));
    // Label 0 would number the background; a float mask is refused as in `extract`.
    assert!(matches!(
        label_lesions(&mask, 0, &keep(1, None)),
        Err(Error::InvalidInput(message)) if message.contains("positive integer")
    ));
    let float = HostArray::from(ArrayD::from_elem(vec![4, 6, 6], 1_f64));
    assert!(matches!(
        label_lesions(&float, 1, &keep(1, None)),
        Err(Error::InvalidInput(message)) if message.contains("integer labels")
    ));
}

#[test]
fn each_lesion_is_extracted_as_its_own_roi() {
    let shape = vec![6, 10, 12];
    let image = HostArray::from(ArrayD::from_shape_fn(shape.clone(), |ix| {
        ((ix[0] * 31 + ix[1] * 17 + ix[2] * 7) % 53) as i16
    }));
    // Label 3 holds a 4x5x5 block and a 3x3x3 block; label 1 fills the gap between them.
    let block = |ix: &[usize], lo: [usize; 3], hi: [usize; 3]| {
        (0..3).all(|axis| ix[axis] >= lo[axis] && ix[axis] < hi[axis])
    };
    let (large, small) = (([1, 1, 1], [5, 6, 6]), ([2, 6, 8], [5, 9, 11]));
    let mask = ArrayD::from_shape_fn(shape.clone(), |ix| {
        let at = [ix[0], ix[1], ix[2]];
        if block(&at, large.0, large.1) || block(&at, small.0, small.1) {
            3_u8
        } else {
            u8::from(at[2] == 7)
        }
    });
    let settings = Settings {
        label: 3,
        features: [
            "firstorder_Mean",
            "glcm_Contrast",
            "shape_VoxelVolume",
            "momentinvariant_ShapeI1",
        ]
        .map(String::from)
        .to_vec(),
        bin_width: 5.0,
        ..Settings::default()
    };
    let transforms = TransformSettings::default();
    let spacing = [2.5, 0.8, 0.8];
    let lesions = extract_lesions(
        &image,
        &HostArray::from(mask.clone()),
        &settings,
        &transforms,
        Some(&spacing),
        &LesionOptions::default(),
    )
    .unwrap();
    assert_eq!(lesions.len(), 2);
    // Invalid settings fail even when the size floor keeps no lesion.
    let invalid = Settings {
        bin_width: 0.0,
        ..settings.clone()
    };
    assert!(matches!(
        extract_lesions(
            &image,
            &HostArray::from(mask.clone()),
            &invalid,
            &transforms,
            Some(&spacing),
            &keep(1000, None),
        ),
        Err(Error::InvalidInput(message)) if message.contains("bin_width")
    ));
    for (lesion, (index, (lower, upper))) in lesions.iter().zip([(1, large), (2, small)]) {
        let alone = ArrayD::from_shape_fn(shape.clone(), |ix| {
            u8::from(block(&[ix[0], ix[1], ix[2]], lower, upper)) * 3
        });
        let expected = extract(
            &image,
            &HostArray::from(alone),
            &settings,
            &transforms,
            Some(&spacing),
            Output::Roi,
        )
        .unwrap();
        assert_eq!(lesion.result.features, expected.features);
        assert_eq!(lesion.result.metadata.settings.label, 3);
        assert_eq!(lesion.index, index);
        assert_eq!(
            lesion.voxels,
            (0..3).map(|a| upper[a] - lower[a]).product::<usize>()
        );
        assert_eq!(
            (&lesion.lower[..], &lesion.upper[..]),
            (&lower[..], &upper[..])
        );
    }
}
