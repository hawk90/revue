//! BoxStats and WhiskerStyle tests: quartiles, whiskers and outliers

use revue::widget::data::chart::{BoxStats, WhiskerStyle};

#[test]
fn test_whisker_style_default() {
    assert_eq!(WhiskerStyle::default(), WhiskerStyle::IQR);
}

#[test]
fn test_whisker_style_variants_distinct() {
    assert_ne!(WhiskerStyle::IQR, WhiskerStyle::MinMax);
    assert_ne!(WhiskerStyle::MinMax, WhiskerStyle::Percentile);
    assert_ne!(WhiskerStyle::IQR, WhiskerStyle::Percentile);
}

#[test]
fn test_box_stats_from_data_empty() {
    assert!(BoxStats::from_data(&[], WhiskerStyle::IQR).is_none());
    assert!(BoxStats::from_data(&[f64::NAN], WhiskerStyle::IQR).is_none());
}

#[test]
fn test_box_stats_from_data_single_value() {
    let stats = BoxStats::from_data(&[5.0], WhiskerStyle::IQR).unwrap();
    assert_eq!(stats.min, 5.0);
    assert_eq!(stats.q1, 5.0);
    assert_eq!(stats.median, 5.0);
    assert_eq!(stats.q3, 5.0);
    assert_eq!(stats.max, 5.0);
    assert!(stats.outliers.is_empty());
}

#[test]
fn test_box_stats_from_data_even_count() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 10.0);
    assert_eq!(stats.median, 5.5);
}

#[test]
fn test_box_stats_quartiles() {
    // Linear interpolation over 8 points: k = 1.75 and 5.25
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.q1, 2.75);
    assert_eq!(stats.q3, 6.25);
}

#[test]
fn test_box_stats_from_data_ignores_non_finite() {
    let data = vec![1.0, f64::NAN, 3.0, f64::INFINITY, 5.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 5.0);
    assert_eq!(stats.median, 3.0);
}

#[test]
fn test_box_stats_iqr_whiskers_and_outliers() {
    // Q1 = 2, Q3 = 4, IQR = 2, fences at -1 and 7
    let data = [1.0, 2.0, 3.0, 4.0, 100.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 100.0);
    assert_eq!(stats.median, 3.0);
    assert_eq!(stats.outliers, vec![100.0]);
    assert_eq!(stats.whisker_low, 1.0);
    assert_eq!(stats.whisker_high, 4.0);
}

#[test]
fn test_box_stats_iqr_no_outliers() {
    let stats = BoxStats::from_data(&[1.0, 2.0, 3.0, 4.0, 5.0], WhiskerStyle::IQR).unwrap();
    assert!(stats.outliers.is_empty());
    assert_eq!(stats.whisker_low, 1.0);
    assert_eq!(stats.whisker_high, 5.0);
}

#[test]
fn test_box_stats_minmax_whiskers() {
    let stats = BoxStats::from_data(&[1.0, 2.0, 3.0, 4.0, 100.0], WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.whisker_low, 1.0);
    assert_eq!(stats.whisker_high, 100.0);
    assert!(stats.outliers.is_empty());
}

#[test]
fn test_box_stats_percentile_whiskers() {
    // 5th / 95th percentile of [1..5]: k = 0.2 and 3.8
    let stats = BoxStats::from_data(&[1.0, 2.0, 3.0, 4.0, 5.0], WhiskerStyle::Percentile).unwrap();
    assert!((stats.whisker_low - 1.2).abs() < 1e-9);
    assert!((stats.whisker_high - 4.8).abs() < 1e-9);
    assert_eq!(stats.outliers, vec![1.0, 5.0]);
}
