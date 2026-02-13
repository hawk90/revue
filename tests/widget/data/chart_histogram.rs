//! Tests for histogram public API
//!
//! Extracted from src/widget/data/chart/histogram/mod.rs

use revue::widget::data::chart::{Histogram, BinConfig, histogram};
use revue::widget::data::chart::chart_common::{Axis, ChartGrid, ChartOrientation};
use revue::style::Color;

#[test]
fn test_histogram_new() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.data.len(), 5);
    assert!(!hist.bins.is_empty());
}

#[test]
fn test_histogram_empty() {
    let hist = Histogram::new(&[]);
    assert!(hist.data.is_empty());
    assert!(hist.bins.is_empty());
}

#[test]
fn test_histogram_bins_auto() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data);
    assert!(!hist.bins.is_empty());
}

#[test]
fn test_histogram_bins_count() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).bin_count(10);
    assert_eq!(hist.bins.len(), 10);
}

#[test]
fn test_histogram_bins_width() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).bin_width(10.0);
    assert_eq!(hist.bins.len(), 10);
}

#[test]
fn test_histogram_mean() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.mean(), Some(3.0));
}

#[test]
fn test_histogram_median() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.median(), Some(3.0));

    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(hist.median(), Some(2.5));
}

#[test]
fn test_histogram_density() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).density(true);
    assert!(hist.density);
}

#[test]
fn test_histogram_cumulative() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).cumulative(true);
    assert!(hist.cumulative);
}

#[test]
fn test_histogram_show_stats() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).show_stats(true);
    assert!(hist.show_stats);
}

#[test]
fn test_histogram_builder() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0])
        .title("Distribution")
        .bin_count(5)
        .color(Color::GREEN)
        .density(true)
        .show_stats(true)
        .x_axis(Axis::new().title("Value"))
        .y_axis(Axis::new().title("Density"));

    assert_eq!(hist.title, Some("Distribution".to_string()));
    assert!(hist.density);
    assert!(hist.show_stats);
}

#[test]
fn test_histogram_helper() {
    let hist = histogram(&[1.0, 2.0, 3.0]);
    assert_eq!(hist.data.len(), 3);
}

#[test]
fn test_histogram_orientation() {
    let hist = Histogram::new(&[1.0]).horizontal();
    assert_eq!(hist.orientation, ChartOrientation::Horizontal);

    let hist = Histogram::new(&[1.0]).vertical();
    assert_eq!(hist.orientation, ChartOrientation::Vertical);
}

#[test]
fn test_histogram_bins_config() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).bins(BinConfig::Count(15));
    assert_eq!(hist.bins.len(), 15);
}

#[test]
fn test_histogram_fill_color() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).fill_color(Color::YELLOW);
    assert_eq!(hist.fill_color, Color::YELLOW);
}

#[test]
fn test_histogram_color_alias() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).color(Color::CYAN);
    assert_eq!(hist.fill_color, Color::CYAN);
}

#[test]
fn test_histogram_bar_border() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).bar_border(Color::WHITE);
    assert_eq!(hist.bar_border, Some(Color::WHITE));
}

#[test]
fn test_histogram_data_update() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).data(&[4.0, 5.0, 6.0, 7.0]);
    assert_eq!(hist.data.len(), 4);
}
