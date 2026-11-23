//! W1 I/O parity: `read_image` and `read_mask` against mlx-radiomics v1.1.0 / SimpleITK 2.5.6
//! goldens (`validation/w1_io_goldens.py`), plus the IBSI 1 digital phantom.
#![allow(missing_docs)]

mod w1_common;

use crate::w1_common::*;
use ndarray::ArrayD;
use radiomics::Error;
use radiomics::io::{HostArray, Volume, read_image, read_mask};
use serde_json::Value;

/// Geometry (spacing, origin, direction) must agree with SimpleITK to this absolute
/// tolerance; ITK evaluates the NIfTI quaternion and normalises in its own order, so the
/// two differ only by float64 round-off (measured maximum 1.1e-16, printed by the test).
const GEOMETRY_TOLERANCE: f64 = 1e-15;

static OBSERVED: Observed = Observed::new();

fn same_class(error: &Error, class: &str) -> bool {
    match class {
        "ValueError" => matches!(error, Error::InvalidInput(_)),
        "RuntimeError" => matches!(error, Error::Execution(_) | Error::NotImplemented(_)),
        "OSError" | "EOFError" => matches!(error, Error::Io(_)),
        _ => false,
    }
}

fn check_error(error: &Error, record: &Value, path: &std::path::Path, context: &str) {
    let class = record["class"].as_str().unwrap();
    assert!(
        same_class(error, class),
        "{context}: expected {class}, got {error:?}"
    );
    if class == "ValueError" {
        let expected = record["message"]
            .as_str()
            .unwrap()
            .replace("{path}", &path.display().to_string());
        if !expected.starts_with("Whoops") {
            assert_eq!(error.to_string(), expected, "{context}: message");
        }
    }
}

fn check_volume(volume: &Volume, record: &Value, context: &str) {
    let spacing = f64_list(&record["spacing"]);
    let origin = f64_list(&record["origin"]);
    let direction = f64_list(&record["direction"]);
    for (name, actual, expected) in [
        ("spacing", &volume.spacing, &spacing),
        ("origin", &volume.origin, &origin),
        ("direction", &volume.direction, &direction),
    ] {
        assert_eq!(actual.len(), expected.len(), "{context}: {name} length");
        for (a, e) in actual.iter().zip(expected) {
            let error = (a - e).abs();
            OBSERVED.note(name, error);
            assert!(error <= GEOMETRY_TOLERANCE, "{context}: {name} {a} vs {e}");
        }
    }
    let error = compare_record(&volume.array, &record["array"], context);
    OBSERVED.note("voxel value", error);
    assert_eq!(error, 0.0, "{context}: voxel values must be identical");
}

#[test]
fn readers_match_mlx_and_simpleitk() {
    let goldens = load_json("io_goldens.json");
    let cases = goldens["cases"].as_array().unwrap();
    assert!(cases.len() > 50);
    for case in cases {
        let file = case["file"].as_str().unwrap();
        let path = data_dir().join(file);
        let context = file.to_string();
        for (key, result) in [("image", read_image(&path)), ("mask", read_mask(&path))] {
            let golden = &case[key];
            let label = format!("{context} {key}");
            match result {
                Ok(volume) => {
                    let ok = golden
                        .get("ok")
                        .unwrap_or_else(|| panic!("{label}: mlx failed: {golden}"));
                    check_volume(&volume, ok, &label);
                }
                Err(error) => {
                    let error_record = golden.get("error").unwrap_or_else(|| {
                        panic!("{label}: Rust failed with {error:?} but mlx read it")
                    });
                    check_error(&error, error_record, &path, &label);
                }
            }
        }
    }
    OBSERVED.report("io");
}

#[test]
fn attached_and_detached_hex_match_simpleitk_pixels() {
    use std::fs;
    let directory = std::env::temp_dir().join(format!("w1_hex_{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let header = "NRRD0005\ntype: short\ndimension: 2\nsizes: 2 2\nencoding: hex\nendian: big\nline skip: 1\nbyte skip: 2\n";
    let payload = "ignore this line\nxx fF\x0bff 7fff  80\n00 0012!";
    let attached = directory.join("attached.nrrd");
    let detached = directory.join("detached.nhdr");
    fs::write(&attached, format!("{header}\n{payload}")).unwrap();
    fs::write(&detached, format!("{header}data file: pixels.hex\n\n")).unwrap();
    fs::write(directory.join("pixels.hex"), payload).unwrap();
    for path in [&attached, &detached] {
        for volume in [read_image(path).unwrap(), read_mask(path).unwrap()] {
            let HostArray::I16(values) = volume.array else {
                panic!("expected int16 pixels")
            };
            assert_eq!(values.shape(), &[2, 2]);
            assert_eq!(values.as_slice().unwrap(), &[-1, 32767, -32768, 18]);
        }
    }
    for (text, message) in [("ffff7", "truncated"), ("ffffz", "invalid character")] {
        fs::write(&attached, format!("{header}\nskip\nxx{text}")).unwrap();
        assert!(matches!(read_image(&attached), Err(Error::Execution(m)) if m.contains(message)));
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn nifti2_is_reported_as_not_implemented() {
    let path = data_dir().join("io/nifti2.nii");
    assert!(matches!(read_image(&path), Err(Error::NotImplemented(_))));
}

#[test]
fn ibsi_digital_phantom_geometry_and_values_match_simpleitk() {
    let image = read_image(&data_dir().join("io/ibsi_digital_phantom_image.nii.gz")).unwrap();
    let mask = read_mask(&data_dir().join("io/ibsi_digital_phantom_mask.nii.gz")).unwrap();
    // The phantom is 5 x 4 x 4 voxels (x, y, z), 2 mm isotropic.
    assert_eq!(image.array.shape(), &[4, 4, 5]);
    assert_eq!(mask.array.shape(), &[4, 4, 5]);
    assert_eq!(image.spacing, vec![2.0, 2.0, 2.0]);
    assert_eq!(image.array.dtype_name(), "int16");
    let goldens = load_json("io_goldens.json");
    for case in goldens["cases"].as_array().unwrap() {
        if case["file"]
            .as_str()
            .unwrap()
            .contains("ibsi_digital_phantom_image")
        {
            check_volume(&image, &case["image"]["ok"], "phantom image");
        }
        if case["file"]
            .as_str()
            .unwrap()
            .contains("ibsi_digital_phantom_mask")
        {
            check_volume(&mask, &case["mask"]["ok"], "phantom mask");
        }
    }
    // IBSI 1: the digital phantom has 74 ROI voxels with label 1 and gray levels 1 to 6.
    let (HostArray::I16(values), HostArray::I16(labels)) = (&image.array, &mask.array) else {
        panic!("phantom pixel type");
    };
    let roi: Vec<i16> = values
        .iter()
        .zip(labels.iter())
        .filter_map(|(&v, &l)| (l == 1).then_some(v))
        .collect();
    assert_eq!(roi.len(), 74);
    assert_eq!((roi.iter().min(), roi.iter().max()), (Some(&1), Some(&6)));
}

#[test]
fn volume_contract_is_enforced_on_construction() {
    let ok =
        |a: HostArray| Volume::new(a, vec![1.0, 1.0], vec![0.0, 0.0], vec![1.0, 0.0, 0.0, 1.0]);
    let two_d = || HostArray::U8(ArrayD::zeros(ndarray::IxDyn(&[2, 3])));
    assert!(ok(two_d()).is_ok());
    let message = |r: radiomics::Result<Volume>| match r {
        Err(Error::InvalidInput(m)) => m,
        other => panic!("expected InvalidInput, got {other:?}"),
    };
    assert_eq!(
        message(ok(HostArray::U8(ArrayD::zeros(ndarray::IxDyn(&[2, 0]))))),
        "volume must have a nonempty 2D or 3D shape"
    );
    assert_eq!(
        message(ok(HostArray::U8(ArrayD::zeros(ndarray::IxDyn(&[2]))))),
        "volume must have a nonempty 2D or 3D shape"
    );
    assert_eq!(
        message(Volume::new(
            two_d(),
            vec![1.0, 0.0],
            vec![0.0; 2],
            vec![1.0, 0.0, 0.0, 1.0]
        )),
        "volume requires finite positive spacing per axis"
    );
    assert_eq!(
        message(Volume::new(
            two_d(),
            vec![1.0, f64::NAN],
            vec![0.0; 2],
            vec![1.0, 0.0, 0.0, 1.0]
        )),
        "volume requires finite positive spacing per axis"
    );
    assert_eq!(
        message(Volume::new(
            two_d(),
            vec![1.0; 2],
            vec![0.0; 3],
            vec![1.0, 0.0, 0.0, 1.0]
        )),
        "volume requires a finite origin per physical axis"
    );
    assert_eq!(
        message(Volume::new(
            two_d(),
            vec![1.0; 2],
            vec![0.0; 2],
            vec![1.0, 0.0, 0.0]
        )),
        "volume requires a finite square direction matrix"
    );
    assert_eq!(
        message(Volume::new(
            two_d(),
            vec![1.0; 2],
            vec![0.0; 2],
            vec![1.0, 0.1, 0.0, 1.0]
        )),
        "volume direction must be orthonormal"
    );
    // The direction tolerance is 1e-5: a 5e-6 deviation passes, 2e-5 does not.
    assert!(
        Volume::new(
            two_d(),
            vec![1.0; 2],
            vec![0.0; 2],
            vec![1.0, 0.0, 0.0, 1.0 + 2.5e-6]
        )
        .is_ok()
    );
    assert!(
        Volume::new(
            two_d(),
            vec![1.0; 2],
            vec![0.0; 2],
            vec![1.0, 0.0, 0.0, 1.0 + 2e-5]
        )
        .is_err()
    );
}

#[test]
fn missing_and_misnamed_files_raise_value_errors() {
    let missing = data_dir().join("io/does_not_exist.nii");
    assert!(
        matches!(read_image(&missing), Err(Error::InvalidInput(m)) if m.starts_with("missing input volume: "))
    );
    let wrong = data_dir().join("io/not_a_volume.txt");
    assert!(
        matches!(read_mask(&wrong), Err(Error::InvalidInput(m)) if m.starts_with("expected a .nii/.nii.gz/.nrrd/.nhdr file: "))
    );
}

#[test]
fn detached_nrrd_slabs_preserve_pixel_order_and_source_geometry() {
    use flate2::{Compression, write::GzEncoder};
    use std::fs;
    use std::io::Write;

    let directory = std::env::temp_dir().join(format!("w1_nrrd_pieces_{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("scan.nhdr");
    let base = "NRRD0005\ntype: short\ndimension: 3\nsizes: 2 3 4\nendian: big\n";
    let geometry = "space directions: (0,2,0) (-3,0,0) (0,0,4)\nspace origin: (3,4,5)\n";
    let expected: Vec<i16> = (-12..12).collect();
    let raw: Vec<u8> = expected.iter().flat_map(|v| v.to_be_bytes()).collect();

    fs::write(directory.join("all scan pixels"), &raw).unwrap();
    for (number, slice) in [-2, 0, 2, 4].iter().zip(raw.chunks_exact(12)) {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(b"xx").unwrap();
        encoder.write_all(slice).unwrap();
        let mut file = b"slice header\n".to_vec();
        file.extend(encoder.finish().unwrap());
        fs::write(directory.join(format!("scan%{number:03}.gz")), file).unwrap();
    }
    for (i, slab) in raw.chunks_exact(24).enumerate() {
        let mut file = vec![0x5a; 3 * i];
        file.extend_from_slice(slab);
        fs::write(directory.join(format!("scan part {i}")), file).unwrap();
    }
    for (i, row) in expected.chunks_exact(2).enumerate() {
        let file = format!("skip\nxx{} {} trailing bytes", row[0], row[1]);
        fs::write(directory.join(format!("row {i}")), file).unwrap();
    }

    let row_list = (0..12).map(|i| format!("row {i}\n")).collect::<String>();
    let row_field =
        format!("encoding: ascii\nline skip: 1\nbyte skip: 2\ndata file: LIST 1\n{row_list}");
    for (space, fields, y_sign) in [
        (
            "left-anterior-superior",
            "encoding: raw\ndata file: all scan pixels\n",
            -1.,
        ),
        (
            "scanner-xyz",
            "encoding: gzip\nline skip: 1\nbyte skip: 2\ndatafile: scan%%%03d.gz -2 5 2\n",
            1.,
        ),
        (
            "3D-right-handed",
            "encoding: raw\nbyte skip: -1\ndata file: LIST 3\nscan part 0\nscan part 1\n",
            1.,
        ),
        ("3D-left-handed", row_field.as_str(), 1.),
    ] {
        fs::write(&path, format!("{base}space: {space}\n{geometry}{fields}")).unwrap();
        for volume in [read_image(&path).unwrap(), read_mask(&path).unwrap()] {
            assert_eq!(volume.spacing, [4., 3., 2.]);
            assert_eq!(volume.origin, [3., 4. * y_sign, 5.]);
            assert_eq!(volume.direction, [0., -1., 0., y_sign, 0., 0., 0., 0., 1.]);
            let HostArray::I16(values) = volume.array else {
                panic!("expected int16")
            };
            assert_eq!(values.shape(), [4, 3, 2]);
            assert_eq!(values.as_slice().unwrap(), expected);
        }
    }
    fs::write(
        &path,
        format!(
            "{base}encoding: gzip\nline skip: 1\nbyte skip: 2\ndata file: scan%%%03d.gz 4 -2 -2\n"
        ),
    )
    .unwrap();
    let HostArray::I16(reversed) = read_image(&path).unwrap().array else {
        panic!("expected int16")
    };
    let reversed_slices: Vec<i16> = expected.chunks_exact(6).rev().flatten().copied().collect();
    assert_eq!(reversed.as_slice().unwrap(), reversed_slices);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn detached_nrrd_rejects_missing_slices_and_short_individual_payloads() {
    use std::fs;
    let directory = std::env::temp_dir().join(format!("w1_nrrd_layout_{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("volume.nhdr");
    let base = "NRRD0005\ntype: uchar\ndimension: 3\nsizes: 2 3 2\nencoding: raw\n";
    for field in [
        "LIST\nfirst\n",
        "LIST 0\nfirst\nsecond\n",
        "LIST 3\na\nb\nc\n",
        "%d 0 2 1\n",
        "%d 0 1 0\n",
        "%d 0 1 -1\n",
    ] {
        fs::write(&path, format!("{base}data file: {field}")).unwrap();
        assert!(read_image(&path).is_err(), "{field}");
    }
    // The second slab is long enough to compensate for the first in a simple concatenation.
    fs::write(directory.join("first"), [1u8; 5]).unwrap();
    fs::write(directory.join("second"), [2u8; 7]).unwrap();
    // A header with comments can put the list beyond one MiB.
    let comments = "# metadata\n".repeat(100_000);
    fs::write(
        &path,
        format!("{base}{comments}data file: LIST\nfirst\nsecond\n"),
    )
    .unwrap();
    assert!(matches!(read_image(&path), Err(Error::Execution(m)) if m.contains("truncated")));
    fs::write(directory.join("first"), [1u8; 6]).unwrap();
    fs::remove_file(directory.join("second")).unwrap();
    assert!(read_image(&path).is_err());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn nrrd_axes_distinguish_channels_from_spatial_dimensions_in_any_position() {
    use std::fs;
    let directory = std::env::temp_dir().join(format!("w1_nrrd_axes_{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("scan.nrrd");
    let write = |sizes: &str, fields: &str, pixels: &[u8]| {
        let header = format!(
            "NRRD0005\ntype: uchar\ndimension: 3\nsizes: {sizes}\nencoding: raw\n{fields}\n\n"
        );
        fs::write(&path, [header.as_bytes(), pixels].concat()).unwrap();
    };
    let pixels: Vec<u8> = (0..12).collect();
    for (sizes, kinds, directions) in [
        ("1 4 3", "list domain domain", "none (0,2) (-3,0)"),
        ("4 1 3", "domain list domain", "(0,2) none (-3,0)"),
        ("4 3 1", "domain domain list", "(0,2) (-3,0) none"),
    ] {
        let fields = format!(
            "kinds: {kinds}\nspace dimension: 2\nspace directions: {directions}\nspace origin: (11,7)"
        );
        write(sizes, &fields, &pixels);
        for volume in [read_image(&path).unwrap(), read_mask(&path).unwrap()] {
            assert_eq!(volume.spacing, [3., 2.]);
            assert_eq!(volume.origin, [11., 7.]);
            assert_eq!(volume.direction, [0., -1., 1., 0.]);
            let HostArray::U8(a) = volume.array else {
                panic!("expected uint8")
            };
            assert_eq!(a.shape(), [3, 4]);
            assert_eq!(a.as_slice().unwrap(), pixels);
        }
    }
    for kinds in ["vector space space", "space rgb space", "space space list"] {
        write("3 3 3", &format!("kinds: {kinds}"), &[1; 27]);
        for result in [read_image(&path), read_mask(&path)] {
            assert!(
                matches!(result,Err(Error::InvalidInput(m)) if m.contains("single-channel volume"))
            );
        }
    }
    // The additional list axis becomes the outer image axis, including its samples.
    write("1 3 4", "kinds: list list domain", &pixels);
    let volume = read_image(&path).unwrap();
    let HostArray::U8(a) = volume.array else {
        panic!("expected uint8")
    };
    assert_eq!(a.shape(), [3, 4]);
    assert_eq!(
        a.as_slice().unwrap(),
        [0, 3, 6, 9, 1, 4, 7, 10, 2, 5, 8, 11]
    );

    write("4 3 2", "space dimension: 2", &[1; 24]);
    assert!(read_image(&path).is_err());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn nrrd_origin_uses_axis_limits_and_centering_when_space_origin_is_absent() {
    use std::fs;
    let directory = std::env::temp_dir().join(format!("w1_nrrd_centers_{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("scan.nrrd");
    for (fields, spacing, origin) in [
        ("axis mins: 10 20\naxis maxs: 14 32", [1., 1.], [11., 22.]),
        (
            "axis mins: 10 20\naxis maxs: 14 32\ncenters: node cell",
            [1., 1.],
            [10., 22.],
        ),
        ("axis mins: 10 20\nspacings: 10 20", [20., 10.], [15., 30.]),
        ("axis mins: 10 20", [1., 1.], [0., 0.]),
        (
            "space dimension: 2\nspace directions: none (0,2)\nspace origin: (0,0)",
            [2., 1.],
            [0., 0.],
        ),
    ] {
        let header = format!(
            "NRRD0005\ntype: short\ndimension: 2\nsizes: 2 3\nencoding: raw\nendian: big\n{fields}\n\n"
        );
        fs::write(&path, [header.as_bytes(), &[0u8; 12]].concat()).unwrap();
        let volume = read_image(&path).unwrap();
        assert_eq!(volume.origin, origin);
        assert_eq!(volume.spacing, spacing);
    }
    fs::remove_dir_all(directory).unwrap();
}
