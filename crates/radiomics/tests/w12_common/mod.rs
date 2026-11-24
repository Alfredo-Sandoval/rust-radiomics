//! Shared IBSI 1 comparison protocol for the W12 tests, following mlx-radiomics'
//! `scripts/qualify_ibsi_1.py` (branch `ibsi-conventions`, commit 2531c04).
//!
//! Rows come from the IBSI 1 submission workbook CSV export. A nonzero published
//! tolerance is applied literally. Integer-valued digital-phantom rows must match
//! exactly (mlx allowed 4 float32 ulps). Every other zero-tolerance row must match
//! after both values are rounded to three significant digits, ties to even, applied to
//! the shortest decimal representation of each value.
#![allow(dead_code, missing_docs)]

use radiomics::settings::{DirectionalAggregation, MatrixAggregation};
use radiomics::{FeatureValue, Output};
use radiomics::{ImageSettings, Settings, TransformSettings, extract_files};
use std::collections::BTreeMap;
use std::path::Path;

/// Workbook tag (without aggregation suffix) to public feature name.
pub const FEATURES: &[(&str, &str)] = &[
    ("morph_volume", "shape_MeshVolume"),
    ("morph_vol_approx", "shape_VoxelVolume"),
    ("morph_area_mesh", "shape_SurfaceArea"),
    ("morph_av", "shape_SurfaceVolumeRatio"),
    ("morph_comp_1", "shape_Compactness1"),
    ("morph_comp_2", "shape_Compactness2"),
    ("morph_sph_dispr", "shape_SphericalDisproportion"),
    ("morph_sphericity", "shape_Sphericity"),
    ("morph_asphericity", "shape_Asphericity"),
    ("morph_com", "shape_CentreOfMassShift"),
    ("morph_diam", "shape_Maximum3DDiameter"),
    ("morph_pca_maj_axis", "shape_MajorAxisLength"),
    ("morph_pca_min_axis", "shape_MinorAxisLength"),
    ("morph_pca_least_axis", "shape_LeastAxisLength"),
    ("morph_pca_elongation", "shape_Elongation"),
    ("morph_pca_flatness", "shape_Flatness"),
    (
        "morph_vol_dens_aabb",
        "shape_VolumeDensityAxisAlignedBoundingBox",
    ),
    (
        "morph_area_dens_aabb",
        "shape_AreaDensityAxisAlignedBoundingBox",
    ),
    (
        "morph_vol_dens_ombb",
        "shape_VolumeDensityOrientedMinimumBoundingBox",
    ),
    (
        "morph_area_dens_ombb",
        "shape_AreaDensityOrientedMinimumBoundingBox",
    ),
    (
        "morph_vol_dens_aee",
        "shape_VolumeDensityApproximateEnclosingEllipsoid",
    ),
    (
        "morph_area_dens_aee",
        "shape_AreaDensityApproximateEnclosingEllipsoid",
    ),
    (
        "morph_vol_dens_mvee",
        "shape_VolumeDensityMinimumVolumeEnclosingEllipsoid",
    ),
    (
        "morph_area_dens_mvee",
        "shape_AreaDensityMinimumVolumeEnclosingEllipsoid",
    ),
    ("morph_vol_dens_conv_hull", "shape_VolumeDensityConvexHull"),
    ("morph_area_dens_conv_hull", "shape_AreaDensityConvexHull"),
    ("morph_integ_int", "shape_IntegratedIntensity"),
    ("morph_moran_i", "shape_MoransI"),
    ("morph_geary_c", "shape_GearysC"),
    ("loc_peak_loc", "localintensity_LocalIntensityPeak"),
    ("loc_peak_glob", "localintensity_GlobalIntensityPeak"),
    ("stat_mean", "firstorder_Mean"),
    ("stat_var", "firstorder_Variance"),
    ("stat_skew", "firstorder_Skewness"),
    ("stat_kurt", "firstorder_Kurtosis"),
    ("stat_median", "firstorder_Median"),
    ("stat_min", "firstorder_Minimum"),
    ("stat_p10", "firstorder_10Percentile"),
    ("stat_p90", "firstorder_90Percentile"),
    ("stat_max", "firstorder_Maximum"),
    ("stat_iqr", "firstorder_InterquartileRange"),
    ("stat_range", "firstorder_Range"),
    ("stat_mad", "firstorder_MeanAbsoluteDeviation"),
    ("stat_rmad", "firstorder_RobustMeanAbsoluteDeviation"),
    ("stat_medad", "firstorder_MedianAbsoluteDeviation"),
    ("stat_cov", "firstorder_CoefficientOfVariation"),
    ("stat_qcod", "firstorder_QuartileCoefficientOfDispersion"),
    ("stat_energy", "firstorder_Energy"),
    ("stat_rms", "firstorder_RootMeanSquared"),
    ("ih_mean", "intensityhistogram_Mean"),
    ("ih_var", "intensityhistogram_Variance"),
    ("ih_skew", "intensityhistogram_Skewness"),
    ("ih_kurt", "intensityhistogram_Kurtosis"),
    ("ih_median", "intensityhistogram_Median"),
    ("ih_min", "intensityhistogram_Minimum"),
    ("ih_p10", "intensityhistogram_10Percentile"),
    ("ih_p90", "intensityhistogram_90Percentile"),
    ("ih_max", "intensityhistogram_Maximum"),
    ("ih_mode", "intensityhistogram_Mode"),
    ("ih_iqr", "intensityhistogram_InterquartileRange"),
    ("ih_range", "intensityhistogram_Range"),
    ("ih_mad", "intensityhistogram_MeanAbsoluteDeviation"),
    ("ih_rmad", "intensityhistogram_RobustMeanAbsoluteDeviation"),
    ("ih_medad", "intensityhistogram_MedianAbsoluteDeviation"),
    ("ih_cov", "intensityhistogram_CoefficientOfVariation"),
    (
        "ih_qcod",
        "intensityhistogram_QuartileCoefficientOfDispersion",
    ),
    ("ih_entropy", "intensityhistogram_Entropy"),
    ("ih_uniformity", "intensityhistogram_Uniformity"),
    ("ih_max_grad", "intensityhistogram_MaximumGradient"),
    (
        "ih_max_grad_g",
        "intensityhistogram_MaximumGradientGrayLevel",
    ),
    ("ih_min_grad", "intensityhistogram_MinimumGradient"),
    (
        "ih_min_grad_g",
        "intensityhistogram_MinimumGradientGrayLevel",
    ),
    ("ivh_v10", "ivh_VolumeAtIntensityFraction10"),
    ("ivh_v90", "ivh_VolumeAtIntensityFraction90"),
    ("ivh_i10", "ivh_IntensityAtVolumeFraction10"),
    ("ivh_i90", "ivh_IntensityAtVolumeFraction90"),
    ("ivh_diff_v10_v90", "ivh_VolumeFractionDifference"),
    ("ivh_diff_i10_i90", "ivh_IntensityFractionDifference"),
    ("ivh_auc", "ivh_AreaUnderIVHCurve"),
    ("cm_joint_max", "glcm_MaximumProbability"),
    ("cm_joint_avg", "glcm_JointAverage"),
    ("cm_joint_var", "glcm_SumSquares"),
    ("cm_joint_entr", "glcm_JointEntropy"),
    ("cm_diff_avg", "glcm_DifferenceAverage"),
    ("cm_diff_var", "glcm_DifferenceVariance"),
    ("cm_diff_entr", "glcm_DifferenceEntropy"),
    ("cm_sum_avg", "glcm_SumAverage"),
    ("cm_sum_var", "glcm_SumVariance"),
    ("cm_sum_entr", "glcm_SumEntropy"),
    ("cm_energy", "glcm_JointEnergy"),
    ("cm_contrast", "glcm_Contrast"),
    ("cm_dissimilarity", "glcm_Dissimilarity"),
    ("cm_inv_diff", "glcm_Id"),
    ("cm_inv_diff_norm", "glcm_Idn"),
    ("cm_inv_diff_mom", "glcm_Idm"),
    ("cm_inv_diff_mom_norm", "glcm_Idmn"),
    ("cm_inv_var", "glcm_InverseVariance"),
    ("cm_corr", "glcm_Correlation"),
    ("cm_auto_corr", "glcm_Autocorrelation"),
    ("cm_clust_tend", "glcm_ClusterTendency"),
    ("cm_clust_shade", "glcm_ClusterShade"),
    ("cm_clust_prom", "glcm_ClusterProminence"),
    ("cm_info_corr1", "glcm_Imc1"),
    ("cm_info_corr2", "glcm_Imc2"),
    ("rlm_sre", "glrlm_ShortRunEmphasis"),
    ("rlm_lre", "glrlm_LongRunEmphasis"),
    ("rlm_lgre", "glrlm_LowGrayLevelRunEmphasis"),
    ("rlm_hgre", "glrlm_HighGrayLevelRunEmphasis"),
    ("rlm_srlge", "glrlm_ShortRunLowGrayLevelEmphasis"),
    ("rlm_srhge", "glrlm_ShortRunHighGrayLevelEmphasis"),
    ("rlm_lrlge", "glrlm_LongRunLowGrayLevelEmphasis"),
    ("rlm_lrhge", "glrlm_LongRunHighGrayLevelEmphasis"),
    ("rlm_glnu", "glrlm_GrayLevelNonUniformity"),
    ("rlm_glnu_norm", "glrlm_GrayLevelNonUniformityNormalized"),
    ("rlm_rlnu", "glrlm_RunLengthNonUniformity"),
    ("rlm_rlnu_norm", "glrlm_RunLengthNonUniformityNormalized"),
    ("rlm_r_perc", "glrlm_RunPercentage"),
    ("rlm_gl_var", "glrlm_GrayLevelVariance"),
    ("rlm_rl_var", "glrlm_RunVariance"),
    ("rlm_rl_entr", "glrlm_RunEntropy"),
    ("szm_sze", "glszm_SmallAreaEmphasis"),
    ("szm_lze", "glszm_LargeAreaEmphasis"),
    ("szm_lgze", "glszm_LowGrayLevelZoneEmphasis"),
    ("szm_hgze", "glszm_HighGrayLevelZoneEmphasis"),
    ("szm_szlge", "glszm_SmallAreaLowGrayLevelEmphasis"),
    ("szm_szhge", "glszm_SmallAreaHighGrayLevelEmphasis"),
    ("szm_lzlge", "glszm_LargeAreaLowGrayLevelEmphasis"),
    ("szm_lzhge", "glszm_LargeAreaHighGrayLevelEmphasis"),
    ("szm_glnu", "glszm_GrayLevelNonUniformity"),
    ("szm_glnu_norm", "glszm_GrayLevelNonUniformityNormalized"),
    ("szm_zsnu", "glszm_SizeZoneNonUniformity"),
    ("szm_zsnu_norm", "glszm_SizeZoneNonUniformityNormalized"),
    ("szm_z_perc", "glszm_ZonePercentage"),
    ("szm_gl_var", "glszm_GrayLevelVariance"),
    ("szm_zs_var", "glszm_ZoneVariance"),
    ("szm_zs_entr", "glszm_ZoneEntropy"),
    ("dzm_sde", "gldzm_SmallDistanceEmphasis"),
    ("dzm_lde", "gldzm_LargeDistanceEmphasis"),
    ("dzm_lgze", "gldzm_LowGrayLevelZoneEmphasis"),
    ("dzm_hgze", "gldzm_HighGrayLevelZoneEmphasis"),
    ("dzm_sdlge", "gldzm_SmallDistanceLowGrayLevelEmphasis"),
    ("dzm_sdhge", "gldzm_SmallDistanceHighGrayLevelEmphasis"),
    ("dzm_ldlge", "gldzm_LargeDistanceLowGrayLevelEmphasis"),
    ("dzm_ldhge", "gldzm_LargeDistanceHighGrayLevelEmphasis"),
    ("dzm_glnu", "gldzm_GrayLevelNonUniformity"),
    ("dzm_glnu_norm", "gldzm_GrayLevelNonUniformityNormalized"),
    ("dzm_zdnu", "gldzm_ZoneDistanceNonUniformity"),
    ("dzm_zdnu_norm", "gldzm_ZoneDistanceNonUniformityNormalized"),
    ("dzm_z_perc", "gldzm_ZonePercentage"),
    ("dzm_gl_var", "gldzm_GrayLevelVariance"),
    ("dzm_zd_var", "gldzm_ZoneDistanceVariance"),
    ("dzm_zd_entr", "gldzm_ZoneDistanceEntropy"),
    ("ngt_coarseness", "ngtdm_Coarseness"),
    ("ngt_contrast", "ngtdm_Contrast"),
    ("ngt_busyness", "ngtdm_Busyness"),
    ("ngt_complexity", "ngtdm_Complexity"),
    ("ngt_strength", "ngtdm_Strength"),
    ("ngl_lde", "gldm_SmallDependenceEmphasis"),
    ("ngl_hde", "gldm_LargeDependenceEmphasis"),
    ("ngl_lgce", "gldm_LowGrayLevelEmphasis"),
    ("ngl_hgce", "gldm_HighGrayLevelEmphasis"),
    ("ngl_ldlge", "gldm_SmallDependenceLowGrayLevelEmphasis"),
    ("ngl_ldhge", "gldm_SmallDependenceHighGrayLevelEmphasis"),
    ("ngl_hdlge", "gldm_LargeDependenceLowGrayLevelEmphasis"),
    ("ngl_hdhge", "gldm_LargeDependenceHighGrayLevelEmphasis"),
    ("ngl_glnu", "gldm_GrayLevelNonUniformity"),
    ("ngl_glnu_norm", "gldm_GrayLevelNonUniformityNormalized"),
    ("ngl_dcnu", "gldm_DependenceNonUniformity"),
    ("ngl_dcnu_norm", "gldm_DependenceNonUniformityNormalized"),
    ("ngl_dc_perc", "gldm_DependencePercentage"),
    ("ngl_gl_var", "gldm_GrayLevelVariance"),
    ("ngl_dc_var", "gldm_DependenceVariance"),
    ("ngl_dc_entr", "gldm_DependenceEntropy"),
    ("ngl_dc_energy", "gldm_DependenceCountEnergy"),
];

/// Workbook aggregation suffix to directional (GLCM, GLRLM) mode.
pub const DIRECTIONAL: &[(&str, DirectionalAggregation)] = &[
    ("2D_avg", DirectionalAggregation::Average2D),
    ("2D_comb", DirectionalAggregation::SliceMerge2D),
    ("2_5D_avg", DirectionalAggregation::DirectionMerge25D),
    ("2_5D_comb", DirectionalAggregation::Merge25D),
    ("3D_avg", DirectionalAggregation::Average3D),
    ("3D_comb", DirectionalAggregation::Merge3D),
];

/// Workbook aggregation suffix to matrix mode (GLSZM, GLDZM, NGTDM, GLDM).
pub const MATRIX: &[(&str, MatrixAggregation)] = &[
    ("2D", MatrixAggregation::Average2D),
    ("2_5D", MatrixAggregation::Merge25D),
    ("3D", MatrixAggregation::ThreeD),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    PublishedTolerance,
    ExactDefinition,
    ThreeSignificant,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub tag: String,
    pub consensus: String,
    pub reference: String,
    pub tolerance: String,
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub tag: String,
    pub feature: String,
    pub value: f64,
    pub reference: String,
    pub tolerance: String,
    pub rule: Rule,
    /// Passes the declared rule.
    pub declared: bool,
    /// Within the literal published tolerance.
    pub literal: bool,
}

/// Minimal RFC 4180 line splitter for the workbook export.
fn split(line: &str) -> Vec<String> {
    let (mut fields, mut field, mut quoted) = (Vec::new(), String::new(), false);
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => fields.push(std::mem::take(&mut field)),
            _ => field.push(c),
        }
    }
    fields.push(field);
    fields
}

/// Feature rows with a published value and tolerance; diagnostics are excluded.
pub fn read_rows(path: &Path) -> Vec<Row> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut lines = text.lines();
    let header = split(lines.next().expect("header"));
    let col = |name: &str| header.iter().position(|h| h == name).unwrap();
    let (family, consensus, value, tolerance, tag) = (
        col("family"),
        col("consensus"),
        col("reference value"),
        col("tolerance"),
        col("tag"),
    );
    lines
        .filter(|l| !l.is_empty())
        .map(split)
        .filter(|f| {
            !f[family].starts_with("Diagnostics")
                && !f[value].is_empty()
                && !f[tolerance].is_empty()
        })
        .map(|f| Row {
            tag: f[tag].clone(),
            consensus: f[consensus].clone(),
            reference: f[value].clone(),
            tolerance: f[tolerance].clone(),
        })
        .collect()
}

/// Split a tag into its feature tag and aggregation suffix (empty when none).
pub fn split_tag(tag: &str) -> (&str, &str) {
    let mut suffixes: Vec<&str> = DIRECTIONAL
        .iter()
        .map(|s| s.0)
        .chain(MATRIX.iter().map(|s| s.0))
        .collect();
    suffixes.sort_by_key(|s| std::cmp::Reverse(s.len()));
    for suffix in suffixes {
        if let Some(head) = tag.strip_suffix(suffix)
            && let Some(head) = head.strip_suffix('_')
        {
            return (head, suffix);
        }
    }
    (tag, "")
}

/// Select one workbook aggregation for every texture family.
pub fn with_aggregation(base: &Settings, suffix: &str) -> Settings {
    let mut s = base.clone();
    if let Some((_, mode)) = DIRECTIONAL.iter().find(|d| d.0 == suffix) {
        s.glcm_aggregation = *mode;
        s.glrlm_aggregation = *mode;
    } else if let Some((_, mode)) = MATRIX.iter().find(|m| m.0 == suffix) {
        s.glszm_aggregation = *mode;
        s.gldzm_aggregation = *mode;
        s.ngtdm_aggregation = *mode;
        s.gldm_aggregation = *mode;
    }
    s
}

/// A finite decimal as (sign, 3 digits, power of ten of the first digit), rounded half-even.
type Sig3 = Option<(bool, [u8; 3], i32)>;

fn sig3_from_parts(negative: bool, digits: &str, exponent10: i32) -> Sig3 {
    let digits: Vec<u8> = digits.bytes().map(|b| b - b'0').collect();
    let first = digits.iter().position(|d| *d != 0)?;
    let exponent = exponent10 - first as i32;
    let mut digits = digits[first..].to_vec();
    let rest = digits.split_off(digits.len().min(3));
    digits.resize(3, 0);
    let tail = rest.first().copied().unwrap_or(0);
    let nonzero_after = rest.iter().skip(1).any(|d| *d != 0);
    let up = tail > 5 || (tail == 5 && (nonzero_after || digits[2] % 2 == 1));
    let mut exponent = exponent;
    if up {
        let mut i = 3;
        loop {
            if i == 0 {
                digits = vec![1, 0, 0];
                exponent += 1;
                break;
            }
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    Some((negative, [digits[0], digits[1], digits[2]], exponent))
}

/// Round a decimal string (plain or scientific) to three significant digits.
fn sig3_text(text: &str) -> Sig3 {
    let text = text.trim();
    let (negative, text) = match text.strip_prefix('-') {
        Some(t) => (true, t),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };
    let lower = text.to_ascii_lowercase();
    let (mantissa, exp) = match lower.split_once('e') {
        Some((m, e)) => (m.to_string(), e.parse::<i32>().expect("exponent")),
        None => (lower.clone(), 0),
    };
    let (int, frac) = mantissa.split_once('.').unwrap_or((&mantissa, ""));
    let digits = format!("{int}{frac}");
    sig3_from_parts(negative, &digits, int.len() as i32 - 1 + exp)
}

/// Three significant digits of the shortest round-trip decimal of `value`.
fn sig3_value(value: f64) -> Sig3 {
    sig3_text(&format!("{value:e}"))
}

pub fn comparison_rule(digital: bool, tag: &str, tolerance: &str) -> Rule {
    if tolerance.parse::<f64>().unwrap() != 0.0 {
        return Rule::PublishedTolerance;
    }
    let (feature, suffix) = split_tag(tag);
    if (feature == "ngl_dc_perc" && !suffix.is_empty())
        || (digital
            && matches!(
                tag,
                "stat_min"
                    | "stat_max"
                    | "stat_range"
                    | "ih_min"
                    | "ih_max"
                    | "ih_range"
                    | "ih_mode"
            ))
    {
        return Rule::ExactDefinition;
    }
    Rule::ThreeSignificant
}

pub fn matches(value: f64, reference: &str, tolerance: &str, rule: Rule) -> bool {
    if !value.is_finite() {
        return false;
    }
    let expected: f64 = reference.parse().unwrap();
    match rule {
        Rule::PublishedTolerance => (value - expected).abs() <= tolerance.parse::<f64>().unwrap(),
        Rule::ExactDefinition => value == expected,
        Rule::ThreeSignificant => sig3_value(value) == sig3_text(reference),
    }
}

/// Run every aggregation group of `rows` through the public file API and score the rows.
pub fn qualify(
    digital: bool,
    image: &Path,
    mask: &Path,
    base: &Settings,
    image_settings: &ImageSettings,
    rows: &[Row],
) -> Vec<Outcome> {
    let mut groups: BTreeMap<&str, Vec<(&Row, &str)>> = BTreeMap::new();
    for row in rows {
        let (feature, suffix) = split_tag(&row.tag);
        let (_, name) = FEATURES
            .iter()
            .find(|(t, _)| *t == feature)
            .unwrap_or_else(|| panic!("unmapped tag {}", row.tag));
        groups.entry(suffix).or_default().push((row, name));
    }
    let mut outcomes = Vec::new();
    for (suffix, entries) in groups {
        let mut settings = with_aggregation(base, suffix);
        let mut names: Vec<String> = Vec::new();
        for (_, name) in &entries {
            if !names.iter().any(|n| n == name) {
                names.push((*name).to_string());
            }
        }
        settings.features = names;
        let result = extract_files(
            image,
            mask,
            &settings,
            image_settings,
            &TransformSettings::default(),
            Output::Roi,
        )
        .unwrap_or_else(|e| panic!("extract_files failed for suffix {suffix:?}: {e}"));
        for (row, name) in entries {
            let key = format!("original_{name}");
            let value = match result.features.get(&key) {
                Some(FeatureValue::Scalar(v)) => *v,
                other => panic!("{key}: unexpected {other:?}"),
            };
            let rule = comparison_rule(digital, &row.tag, &row.tolerance);
            let declared = matches(value, &row.reference, &row.tolerance, rule);
            let literal = (value - row.reference.parse::<f64>().unwrap()).abs()
                <= row.tolerance.parse::<f64>().unwrap();
            outcomes.push(Outcome {
                tag: row.tag.clone(),
                feature: (*name).to_string(),
                value,
                reference: row.reference.clone(),
                tolerance: row.tolerance.clone(),
                rule,
                declared,
                literal,
            });
        }
    }
    outcomes
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Counts {
    pub compared: usize,
    pub literal: usize,
    pub declared: usize,
    pub rounding: usize,
    pub exact: usize,
    pub failures: usize,
}

pub fn count(outcomes: &[Outcome]) -> Counts {
    let mut c = Counts::default();
    for o in outcomes {
        c.compared += 1;
        c.literal += usize::from(o.literal);
        c.declared += usize::from(o.declared);
        if !o.declared {
            c.failures += 1;
        } else {
            // Rounding passes are declared-rule passes that are not literally equal.
            match o.rule {
                Rule::ThreeSignificant if !o.literal => c.rounding += 1,
                Rule::ThreeSignificant => {}
                Rule::ExactDefinition => c.exact += 1,
                Rule::PublishedTolerance => {}
            }
        }
    }
    c
}

pub fn print_table(rows: &[(String, Counts)]) {
    println!(
        "{:<14} {:>8} {:>10} {:>9} {:>9} {:>10} {:>8}",
        "dataset", "compared", "declared", "literal", "rounding", "exact-def", "failures"
    );
    for (name, c) in rows {
        println!(
            "{name:<14} {:>8} {:>10} {:>9} {:>9} {:>10} {:>8}",
            c.compared, c.declared, c.literal, c.rounding, c.exact, c.failures
        );
    }
}

/// Optional per-row record for figures: with `IBSI_ROWS_DIR` set, writes
/// `ibsi1-<name>.csv` there (one line per scored row, value printed round-trip).
pub fn record_rows(name: &str, outcomes: &[Outcome]) {
    let Some(dir) = std::env::var_os("IBSI_ROWS_DIR") else {
        return;
    };
    let mut text =
        String::from("dataset,tag,feature,value,reference,tolerance,rule,declared,literal\n");
    for o in outcomes {
        text.push_str(&format!(
            "{name},{},{},{:e},{},{},{:?},{},{}\n",
            o.tag, o.feature, o.value, o.reference, o.tolerance, o.rule, o.declared, o.literal
        ));
    }
    let path = Path::new(&dir).join(format!("ibsi1-{name}.csv"));
    std::fs::write(&path, text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
}

pub fn print_failures(name: &str, outcomes: &[Outcome]) {
    for o in outcomes {
        if !o.declared {
            println!(
                "  FAIL {name} {} [{}]: {:.12e} vs {} +/- {} ({:?})",
                o.tag, o.feature, o.value, o.reference, o.tolerance, o.rule
            );
        }
    }
}
