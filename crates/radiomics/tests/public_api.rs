//! Names every item of the public facade through root and facade-module paths only.
//! It compiles only while each type that appears in a facade signature stays nameable.
#![allow(unused_imports)]

use radiomics::io::{ArrayKind, HostArray, VOLUME_SUFFIXES, Volume, read_image, read_mask};
use radiomics::metadata::{
    Device, ExecutionMetadata, FeatureValidity, ImageGeometryPayload, Precision, RuntimeMetadata,
};
use radiomics::params::{
    ExtractionParameters, Value, load_params, parse_params, settings_from_params,
};
use radiomics::settings::*;
use radiomics::{
    Error, ExportFilesOptions, ExportOptions, ExtractionResult, Family, FeatureSpec, FeatureValue,
    IbsiInterpolation, ImageSettings, ImageType, Interpolator, MapDtype, MapExport, Mode, Output,
    Result, Settings, SinkFactory, TransformSettings, VoxelSink, default_features, export,
    export_files, extract, extract_files, features, list_features,
};

fn send_sync<T: Send + Sync + 'static>() {}

#[test]
fn facade_types_are_thread_safe() {
    send_sync::<Error>();
    send_sync::<Settings>();
    send_sync::<TransformSettings>();
    send_sync::<ImageSettings>();
    send_sync::<ExtractionResult<FeatureValue>>();
    send_sync::<MapExport>();
    send_sync::<HostArray>();
    send_sync::<Volume>();
}

#[test]
fn error_is_a_std_error() {
    fn assert_error<E: std::error::Error>() {}
    assert_error::<Error>();
}
