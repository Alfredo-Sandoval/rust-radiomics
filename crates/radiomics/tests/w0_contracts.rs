//! Schema-2 metadata keys, non-finite JSON and borrowed voxel batches.

use indexmap::IndexMap;
use ndarray::{ArrayView1, ArrayView2, array};
use radiomics::metadata::{
    Device, ExecutionMetadata, ImageGeometryPayload, Precision, RuntimeMetadata,
};
use radiomics::{
    Error, ExtractionResult, ImageSettings, MapDtype, MapExport, Settings, TransformSettings,
    VoxelSink,
};
use serde_json::{Value, json};
use std::path::PathBuf;

fn oracle() -> Value {
    serde_json::from_str(include_str!("data/w0/contracts.json")).unwrap()
}

fn keys(value: &Value) -> Vec<String> {
    let mut names: Vec<_> = value.as_object().unwrap().keys().cloned().collect();
    names.sort();
    names
}

fn metadata() -> ExecutionMetadata {
    ExecutionMetadata {
        settings: Settings {
            bin_width: f64::NAN,
            bin_count: Some(8),
            ..Default::default()
        },
        device: Device::Cpu,
        precision: Precision::Float64,
        runtime: RuntimeMetadata::default(),
        voxel: false,
        spacing: vec![2., 3.],
        array_shape: vec![2, 2],
        array_dtype: "int16".into(),
        gray_levels: 8,
        roi_voxels: 3,
        excluded_voxels: 1,
        feature_validity: IndexMap::new(),
        histogram_method: "fixed bin count".into(),
        preprocessing: "CPU float64 binning".into(),
        map_dtype: None,
        aggregation: None,
        ivh_lattice: None,
        gray_levels_by_image: None,
        batch_size: None,
        scratch_estimate_bytes: None,
        first_order_batch_size: None,
        first_order_scratch_estimate_bytes: None,
        first_order_cache_bytes: None,
        image_geometry: None,
        source_geometry: None,
        image_settings: None,
        transforms: None,
    }
}

#[test]
fn execution_metadata_matches_required_and_optional_schema_keys() {
    let reference = oracle();
    let mut md = metadata();
    md.runtime.versions = [("upstream".into(), "1.2".into())].into();
    let minimal = serde_json::to_value(&md).unwrap();
    assert_eq!(json!(keys(&minimal)), reference["required"]);
    assert_eq!(json!(keys(&minimal["runtime"])), reference["runtime"]);
    assert_eq!(minimal["device"], "cpu");
    assert_eq!(minimal["precision"], "float64");
    assert_eq!(minimal["runtime"]["python"], Value::Null);
    assert_eq!(minimal["settings"]["bin_width"], "NaN");
    let source = ImageGeometryPayload {
        shape: vec![2, 2],
        spacing: vec![2., 3.],
        origin: vec![7., -1.],
        direction: vec![0., 1., -1., 0.],
    };
    let processed = ImageGeometryPayload {
        shape: vec![3, 3],
        spacing: vec![1., 1.],
        ..source.clone()
    };
    let mut full = md.clone();
    full.image_geometry = Some(processed);
    full.source_geometry = Some(source);
    full.image_settings = Some(ImageSettings {
        resampled_spacing: Some(vec![1., 1.]),
        ..Default::default()
    });
    full.voxel = true;
    full.map_dtype = Some(MapDtype::Float32);
    full.transforms = Some(TransformSettings::default());
    full.aggregation = Some([("glcm".into(), "2d_average".into())].into());
    full.ivh_lattice = Some([("levels".into(), serde_json::Number::from(8))].into());
    full.gray_levels_by_image = Some([("original".into(), 8)].into());
    full.batch_size = Some(3);
    full.scratch_estimate_bytes = Some(64);
    full.first_order_batch_size = Some(5);
    full.first_order_scratch_estimate_bytes = Some(128);
    full.first_order_cache_bytes = Some(32);
    let complete = serde_json::to_value(&full).unwrap();
    let optional: Vec<_> = keys(&complete)
        .into_iter()
        .filter(|k| minimal.get(k).is_none())
        .collect();
    assert_eq!(json!(optional), reference["optional"]);
    assert_eq!(
        complete["image_settings"]["resampled_spacing"],
        json!([1.0, 1.0])
    );
    assert_eq!(complete["transforms"]["image_types"], json!(["original"]));
    assert_eq!(complete["map_dtype"], "float32");
}

#[test]
fn result_values_and_export_paths_keep_feature_names_and_order() {
    let result = ExtractionResult {
        features: [
            ("square_glcm_Contrast".into(), f64::NAN),
            ("original_firstorder_Mean".into(), 2.5),
        ]
        .into(),
        metadata: metadata(),
    };
    let safe = serde_json::to_value(&result).unwrap();
    assert_eq!(
        safe["features"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["square_glcm_Contrast", "original_firstorder_Mean"]
    );
    assert_eq!(safe["features"]["square_glcm_Contrast"], "NaN");
    assert_eq!(safe["features"]["original_firstorder_Mean"], 2.5);
    let export = MapExport {
        metadata: result.metadata,
        report: PathBuf::from("/study/report.json"),
        maps: [(
            "original_firstorder_Mean".into(),
            PathBuf::from("/study/mean.nii.gz"),
        )]
        .into(),
    };
    let safe = serde_json::to_value(&export).unwrap();
    assert_eq!(safe["report"], "/study/report.json");
    assert_eq!(
        safe["maps"]["original_firstorder_Mean"],
        "/study/mean.nii.gz"
    );
}

#[test]
fn voxel_sink_borrows_numpy_order_coordinates_and_feature_columns() {
    let centers = array![[0i32, 1, 2], [1, 0, 1]];
    let values = array![3.0f32, -5.0];
    let batch = IndexMap::from([("original_firstorder_Mean".into(), values.view())]);
    let mut sink = |coords: ArrayView2<'_, i32>,
                    columns: &IndexMap<String, ArrayView1<'_, f32>>| {
        assert_eq!(coords, centers.view());
        assert_eq!(columns["original_firstorder_Mean"], values.view());
        Err(Error::Io("output closed".into()))
    };
    let consumer: &mut dyn VoxelSink = &mut sink;
    assert_eq!(
        consumer.consume(centers.view(), &batch),
        Err(Error::Io("output closed".into()))
    );
}
