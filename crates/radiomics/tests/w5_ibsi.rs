//! W5 parity: IBSI 1 digital phantom morphology reference rows (CC BY 4.0 phantom).
//!
//! The fixture `tests/data/w5/ibsi_phantom.json` is exported by `validation/w5_goldens.py` from
//! the IBSI digital phantom NIfTI files (4 x 4 x 5 voxels, 2 mm spacing, 74 ROI voxels) and
//! the morphology rows of `digital phantom.csv`. Each row carries the IBSI consensus value
//! and tolerance; a feature passes when `|value - reference| <= tolerance`. Features without
//! a standardised reference (OMBB and MVEE densities) are compared with mlx-radiomics only.

use ndarray::{ArrayD, IxDyn};
use radiomics::{FeatureValue, HostArray, Output, Settings, TransformSettings, extract};
use serde_json::Value;

/// Image, mask and spacing of the IBSI digital phantom.
struct Phantom {
    image: HostArray,
    mask: HostArray,
    spacing: Vec<f64>,
}

/// Shape and morphology values by feature name, through the public `extract`.
fn compute(phantom: &Phantom, settings: &Settings, names: &[&str]) -> Vec<(String, f64)> {
    let settings = Settings {
        features: names.iter().map(|name| (*name).to_owned()).collect(),
        ..settings.clone()
    };
    extract(
        &phantom.image,
        &phantom.mask,
        &settings,
        &TransformSettings::default(),
        Some(&phantom.spacing),
        Output::Roi,
    )
    .unwrap()
    .features
    .into_iter()
    .map(|(key, value)| match value {
        FeatureValue::Scalar(x) => (key.strip_prefix("original_").unwrap().to_owned(), x),
        _ => panic!("scalar expected"),
    })
    .collect()
}

/// IBSI rows are checked at the tolerance published with the reference value.
/// Against mlx-radiomics float32 (rtol 3e-6, atol 2e-6 as in mlx's own shape tests).
const MLX_RTOL: f64 = 3e-6;
const MLX_ATOL: f64 = 2e-6;
/// MVEE area density: the 0.001 tolerance is sensitive to the stop step of the ellipsoid fit.
const MLX_MVEE_AREA_RTOL: f64 = 5e-4;

fn tag_to_feature(tag: &str) -> Option<&'static str> {
    Some(match tag {
        "morph_volume" => "shape_MeshVolume",
        "morph_vol_approx" => "shape_VoxelVolume",
        "morph_area_mesh" => "shape_SurfaceArea",
        "morph_av" => "shape_SurfaceVolumeRatio",
        "morph_comp_1" => "shape_Compactness1",
        "morph_comp_2" => "shape_Compactness2",
        "morph_sph_dispr" => "shape_SphericalDisproportion",
        "morph_sphericity" => "shape_Sphericity",
        "morph_asphericity" => "shape_Asphericity",
        "morph_com" => "shape_CentreOfMassShift",
        "morph_diam" => "shape_Maximum3DDiameter",
        "morph_pca_maj_axis" => "shape_MajorAxisLength",
        "morph_pca_min_axis" => "shape_MinorAxisLength",
        "morph_pca_least_axis" => "shape_LeastAxisLength",
        "morph_pca_elongation" => "shape_Elongation",
        "morph_pca_flatness" => "shape_Flatness",
        "morph_vol_dens_aabb" => "shape_VolumeDensityAxisAlignedBoundingBox",
        "morph_area_dens_aabb" => "shape_AreaDensityAxisAlignedBoundingBox",
        "morph_vol_dens_aee" => "shape_VolumeDensityApproximateEnclosingEllipsoid",
        "morph_area_dens_aee" => "shape_AreaDensityApproximateEnclosingEllipsoid",
        "morph_vol_dens_conv_hull" => "shape_VolumeDensityConvexHull",
        "morph_area_dens_conv_hull" => "shape_AreaDensityConvexHull",
        "morph_integ_int" => "shape_IntegratedIntensity",
        "morph_moran_i" => "shape_MoransI",
        "morph_geary_c" => "shape_GearysC",
        _ => return None,
    })
}

fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

fn phantom() -> (Phantom, Value) {
    let path = format!(
        "{}/tests/data/w5/ibsi_phantom.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let document: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let shape: Vec<usize> = document["shape"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect();
    let mask = ArrayD::from_shape_vec(
        IxDyn(&shape),
        document["mask"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_i64().unwrap() != 0)
            .collect(),
    )
    .unwrap();
    let image = ArrayD::from_shape_vec(IxDyn(&shape), numbers(&document["image"])).unwrap();
    let spacing = numbers(&document["spacing"]);
    let phantom = Phantom {
        image: HostArray::from(image),
        mask: HostArray::from(mask.mapv(u8::from)),
        spacing,
    };
    (phantom, document)
}

fn geometric_names() -> Vec<String> {
    radiomics::features()
        .iter()
        .filter(|spec| spec.family == radiomics::Family::Shape)
        .map(|spec| spec.key())
        .filter(|key| {
            ![
                "CentreOfMassShift",
                "IntegratedIntensity",
                "MoransI",
                "GearysC",
            ]
            .iter()
            .any(|name| key.ends_with(name))
        })
        .collect()
}

#[test]
fn digital_phantom_matches_ibsi_morphology_rows() {
    let (phantom, document) = phantom();
    let settings = Settings::default();
    let geometric = geometric_names();
    let names: Vec<&str> = geometric.iter().map(String::as_str).collect();
    let mut values = compute(&phantom, &settings, &names);
    values.extend(compute(
        &phantom,
        &settings,
        &[
            "shape_CentreOfMassShift",
            "shape_IntegratedIntensity",
            "shape_MoransI",
            "shape_GearysC",
        ],
    ));
    let value_of = |key: &str| values.iter().find(|(k, _)| k == key).unwrap().1;
    let mut checked = 0;
    let mut worst_fraction = 0.0f64;
    for row in document["rows"].as_array().unwrap() {
        let tag = row["tag"].as_str().unwrap();
        let reference = row["reference"].as_f64().unwrap();
        let tolerance = row["tolerance"].as_f64().unwrap();
        let Some(key) = tag_to_feature(tag) else {
            panic!("unmapped IBSI tag {tag}")
        };
        let value = value_of(key);
        let fraction = (value - reference).abs() / tolerance;
        worst_fraction = worst_fraction.max(fraction);
        assert!(
            fraction <= 1.0,
            "{tag} ({key}): rust {value} vs IBSI {reference} +/- {tolerance}"
        );
        checked += 1;
    }
    assert_eq!(checked, 25, "every standardised morphology row is checked");
    eprintln!("IBSI phantom: {checked} rows, worst |error| / tolerance = {worst_fraction:.3}");
}

#[test]
fn digital_phantom_matches_mlx_including_unstandardised_features() {
    let (phantom, document) = phantom();
    let mut worst = 0.0f64;
    for (label, settings) in [
        ("ibsi", Settings::default()),
        ("pyradiomics", Settings::pyradiomics_conventions()),
    ] {
        let golden = document["mlx"][label].as_object().unwrap();
        let keys: Vec<String> = golden.keys().map(|name| format!("shape_{name}")).collect();
        let names: Vec<&str> = keys.iter().map(String::as_str).collect();
        let values = compute(&phantom, &settings, &names);
        for ((key, value), (name, expected)) in values.iter().zip(golden) {
            let expected = expected.as_f64().unwrap();
            let rtol = if name == "AreaDensityMinimumVolumeEnclosingEllipsoid" {
                MLX_MVEE_AREA_RTOL
            } else {
                MLX_RTOL
            };
            worst = worst.max(((value - expected) / expected).abs());
            assert!(
                (value - expected).abs() <= MLX_ATOL + rtol * expected.abs(),
                "{label}/{key}: rust {value} vs mlx {expected}"
            );
        }
    }
    let settings = Settings::default();
    for (name, expected) in document["mlx_morphology"].as_object().unwrap() {
        let key = format!("shape_{name}");
        let value = compute(&phantom, &settings, &[key.as_str()])[0].1;
        let expected = expected.as_f64().unwrap();
        worst = worst.max(((value - expected) / expected).abs());
        assert!(
            (value - expected).abs() <= MLX_ATOL + MLX_RTOL * expected.abs(),
            "{name}: {value} vs {expected}"
        );
    }
    eprintln!("IBSI phantom vs mlx: max relative error {worst:.3e}");
}
