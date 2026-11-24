//! The public contract shared with mlx-radiomics: catalog, selection and defaults.

use radiomics::settings::{BinOrigin, DirectionalAggregation, Kurtosis};
use radiomics::{
    Family, ImageType, Mode, Settings, TransformSettings, default_features, features, list_features,
};

#[test]
fn catalog_matches_mlx_radiomics() {
    let all = features();
    assert_eq!(all.len(), 200);
    assert_eq!(all.first().unwrap().key(), "firstorder_Mean");
    assert_eq!(all.last().unwrap().key(), "momentinvariant_ShapeF3");
    let counts: Vec<usize> = Family::ALL
        .iter()
        .map(|family| all.iter().filter(|spec| spec.family == *family).count())
        .collect();
    assert_eq!(counts, [21, 26, 17, 5, 16, 16, 32, 9, 16, 23, 2, 7, 10]);
    let mut keys: Vec<String> = all.iter().map(|spec| spec.key()).collect();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), 200, "feature keys must be unique");
    assert_eq!(
        all.iter().filter(|spec| spec.ibsi_id.is_some()).count(),
        174
    );
}

#[test]
fn default_selection_is_the_93_pyradiomics_texture_features() {
    let defaults = default_features();
    assert_eq!(defaults.len(), 93);
    let families: std::collections::BTreeSet<&str> = defaults
        .iter()
        .map(|name| name.split_once('_').unwrap().0)
        .collect();
    assert_eq!(
        families.into_iter().collect::<Vec<_>>(),
        ["firstorder", "glcm", "gldm", "glrlm", "glszm", "ngtdm"]
    );
    for excluded in [
        "firstorder_MedianAbsoluteDeviation",
        "glcm_SumVariance",
        "gldm_DependencePercentage",
    ] {
        assert!(!defaults.iter().any(|name| name == excluded), "{excluded}");
    }
    assert_eq!(Settings::default().features, defaults);
}

#[test]
fn list_features_filters_by_mode_and_dimension() {
    assert_eq!(list_features(&[], Mode::Roi, None).unwrap().len(), 200);
    assert_eq!(list_features(&[], Mode::Voxel, Some(3)).unwrap().len(), 147);
    assert_eq!(list_features(&[], Mode::Voxel, Some(2)).unwrap().len(), 147);
    assert_eq!(list_features(&[], Mode::Roi, Some(3)).unwrap().len(), 191);
    assert_eq!(list_features(&[], Mode::Roi, Some(2)).unwrap().len(), 156);
    let picked = list_features(&["glcm", "firstorder", "glcm"], Mode::Roi, None).unwrap();
    assert_eq!(picked.len(), 47);
    assert_eq!(
        picked[0], "firstorder_Mean",
        "catalog order, not argument order"
    );
    assert!(list_features(&["nope"], Mode::Roi, None).is_err());
    assert!(list_features(&[], Mode::Roi, Some(4)).is_err());
}

#[test]
fn defaults_follow_ibsi_and_round_trip_their_spelling() {
    let settings = Settings::default();
    assert_eq!(
        (settings.bin_width, settings.bin_count, settings.label),
        (25.0, None, 1)
    );
    assert_eq!(settings.kurtosis, Kurtosis::Excess);
    assert_eq!(settings.bin_origin.as_str(), "resegment-or-minimum");
    assert_eq!(
        Settings::pyradiomics_conventions().bin_origin,
        BinOrigin::ZeroLattice
    );
    for choice in DirectionalAggregation::ALL {
        assert_eq!(
            choice.as_str().parse::<DirectionalAggregation>().unwrap(),
            *choice
        );
    }
    assert!("2d".parse::<DirectionalAggregation>().is_err());
    assert_eq!(ImageType::ALL.len(), 16);
    assert_eq!("lbp-3D".parse::<ImageType>().unwrap(), ImageType::Lbp3D);
    assert_eq!(
        TransformSettings::default().image_types,
        [ImageType::Original]
    );
}

#[test]
fn laws_kernel_names_validate() {
    let laws = |kernel: &str| TransformSettings {
        image_types: vec![ImageType::Laws],
        laws: radiomics::settings::LawsSettings {
            kernel: kernel.into(),
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(laws("L5S5").validate().is_ok());
    assert!(laws("E5L5S5").validate().is_ok());
    for bad in ["L5", "L5S5E5W5", "L5X5", "l5s5"] {
        assert!(laws(bad).validate().is_err(), "{bad}");
    }
}
