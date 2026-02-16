//! Chart statistics tests extracted from src/widget/data/chart/chart_stats.rs
//!
//! This file contains tests for statistical functions used by charts:
//! - filter_finite() - Filter out NaN and infinite values
//! - percentile() - Calculate percentile from sorted data
//! - mean() - Calculate arithmetic mean
//! - median() - Calculate median of data
//! - quartiles() - Calculate Q1, median, Q3
//! - min_max() - Get minimum and maximum values
//! - iqr() - Calculate interquartile range
//! - outliers_iqr() - Detect outliers using IQR method
//! - compute_bins() - Compute histogram bins from data
//! - HistogramBin struct and BinConfig enum

use revue::widget::data::chart::chart_stats::{
    BinConfig, compute_bins, filter_finite, iqr, mean, median, min_max, outliers_iqr,
    percentile, HistogramBin,
};

#[test]
fn test_filter_finite() {
    let data = vec![1.0, f64::NAN, 2.0, f64::INFINITY, 3.0];
    let filtered = filter_finite(&data);
    assert_eq!(filtered, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_percentile() {
    let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(percentile(&sorted, 0.0), 1.0);
    assert_eq!(percentile(&sorted, 50.0), 3.0);
    assert_eq!(percentile(&sorted, 100.0), 5.0);
}

#[test]
fn test_percentile_empty() {
    let sorted: Vec<f64> = vec![];
    assert_eq!(percentile(&sorted, 50.0), 0.0);
}

#[test]
fn test_mean() {
    assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
    assert_eq!(mean(&[]), None);
    assert_eq!(mean(&[f64::NAN]), None);
}

#[test]
fn test_median_odd() {
    assert_eq!(median(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
}

#[test]
fn test_median_even() {
    assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
}

#[test]
fn test_median_empty() {
    assert_eq!(median(&[]), None);
}

#[test]
fn test_quartiles() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let (q1, med, q3) = quartiles(&data).unwrap();
    // Linear interpolation: k = p/100 * (n-1), where n=9
    // Q1: k = 0.25 * 8 = 2.0 -> sorted[2] = 3.0
    // Median: k = 0.5 * 8 = 4.0 -> sorted[4] = 5.0
    // Q3: k = 0.75 * 8 = 6.0 -> sorted[6] = 7.0
    assert!((q1 - 3.0).abs() < 0.1);
    assert!((med - 5.0).abs() < 0.1);
    assert!((q3 - 7.0).abs() < 0.1);
}

#[test]
fn test_min_max() {
    assert_eq!(min_max(&[3.0, 1.0, 4.0, 1.0, 5.0]), Some((1.0, 5.0)));
    assert_eq!(min_max(&[]), None);
}

#[test]
fn test_iqr() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let iqr_val = iqr(&data).unwrap();
    assert!(iqr_val > 0.0);
}

#[test]
fn test_outliers_iqr() {
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0); // Outlier
    data.push(-50.0); // Outlier

    let outliers = outliers_iqr(&data);
    assert!(outliers.contains(&100.0));
    assert!(outliers.contains(&-50.0));
}

#[test]
fn test_compute_bins_auto() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Auto);
    assert!(!bins.is_empty());
    assert!(bins.len() <= 100);
}

#[test]
fn test_compute_bins_count() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Count(10));
    assert_eq!(bins.len(), 10);
}

#[test]
fn test_compute_bins_width() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Width(10.0));
    assert_eq!(bins.len(), 10);
}

#[test]
fn test_compute_bins_edges() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Edges(vec![0.0, 25.0, 50.0, 75.0, 100.0]));
    assert_eq!(bins.len(), 4);
}

#[test]
fn test_compute_bins_empty() {
    let bins = compute_bins(&[], &BinConfig::Auto);
    assert!(bins.is_empty());
}

#[test]
fn test_histogram_bin_frequency() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let bins = compute_bins(&data, &BinConfig::Count(10));

    // Sum of frequencies should be approximately 1.0
    let total_freq: f64 = bins.iter().map(|b| b.frequency).sum();
    assert!((total_freq - 1.0).abs() < 0.01);
}
