//! Map NRRD storage axes onto the scalar image axes exposed by SimpleITK.
use crate::{Error, Result};

pub(super) struct Axes {
    /// ITK axis order, fastest first, including extra range axes after the spatial axes.
    pub(super) image: Vec<usize>,
    pub(super) spatial_count: usize,
    pub(super) component: Option<usize>,
    pub(super) complex: bool,
}

impl Axes {
    pub(super) fn new(
        sizes: &[usize],
        kinds: Option<&str>,
        space_dimension: usize,
    ) -> Result<Self> {
        let kinds: Vec<_> = kinds
            .into_iter()
            .flat_map(str::split_whitespace)
            .map(str::to_ascii_lowercase)
            .collect();
        if !kinds.is_empty() && kinds.len() != sizes.len() {
            return Err(Error::Execution("NRRD kinds do not match dimension".into()));
        }
        let mut image = Vec::new();
        let mut ranges = Vec::new();
        let mut component = None;
        for (axis, &size) in sizes.iter().enumerate() {
            let kind = kinds.get(axis).map_or("domain", String::as_str);
            let required_size = match kind {
                "domain" | "space" | "time" | "none" | "???" => {
                    image.push(axis);
                    continue;
                }
                "list"
                | "point"
                | "vector"
                | "contravariant-vector"
                | "covariant-vector"
                | "normal" => None,
                "stub" | "scalar" => Some(1),
                "complex" | "2-vector" => Some(2),
                "3-color" | "rgb-color" | "rgbcolor" | "rgb" | "hsv-color" | "hsvcolor" | "hsv"
                | "xyz-color" | "3-vector" | "3-gradient" | "3-normal" => Some(3),
                "4-color" | "rgba-color" | "rgbacolor" | "rgba" | "4-vector" | "quaternion" => {
                    Some(4)
                }
                _ => {
                    let compact = kind.replace("symmetric", "sym").replace("tensor", "matrix");
                    Some(match compact.as_str() {
                        "2d-sym-matrix" => 3,
                        "2d-masked-sym-matrix" | "2d-matrix" => 4,
                        "2d-masked-matrix" => 5,
                        "3d-sym-matrix" => 6,
                        "3d-masked-sym-matrix" => 7,
                        "3d-matrix" => 9,
                        "3d-masked-matrix" => 10,
                        _ => {
                            return Err(Error::Execution(format!(
                                "unknown NRRD axis kind {kind:?}"
                            )));
                        }
                    })
                }
            };
            if required_size.is_some_and(|n| n != size) {
                return Err(Error::Execution(format!(
                    "invalid NRRD axis length for {kind}"
                )));
            }
            ranges.push(axis);
            if component.is_none() && kind != "list" {
                component = Some(axis);
            }
        }
        let spatial_count = image.len();
        if space_dimension != 0 && spatial_count != space_dimension {
            return Err(Error::Execution(
                "NRRD spatial axes do not match the space dimension".into(),
            ));
        }
        let component = component.or_else(|| ranges.first().copied());
        image.extend(ranges.into_iter().filter(|&axis| Some(axis) != component));
        Ok(Self {
            image,
            spatial_count,
            component,
            complex: component.is_some_and(|axis| kinds[axis] == "complex"),
        })
    }
}
