//! Tests for box plot public API
//!
//! Extracted from src/widget/data/chart/boxplot/mod.rs

use revue::widget::data::chart::{
    BoxPlot, BoxGroup, BoxStats, WhiskerStyle, boxplot,
};
use revue::widget::data::chart::chart_common::{Axis, ChartOrientation};
use revue::style::Color;

#[test]
fn test_boxplot_new() {
    let bp = BoxPlot::new();
    assert!(bp.groups.is_empty());
}

#[test]
fn test_boxplot_group() {
    let bp = BoxPlot::new()
        .group("A", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .group("B", &[2.0, 3.0, 4.0, 5.0, 6.0]);

    assert_eq!(bp.groups.len(), 2);
}

#[test]
fn test_boxstats_from_data() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();

    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 10.0);
    assert_eq!(stats.median, 5.5);
}

#[test]
fn test_boxstats_quartiles() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();

    assert!(stats.q1 >= 2.0 && stats.q1 <= 3.0);
    assert!(stats.q3 >= 6.0 && stats.q3 <= 7.0);
}

#[test]
fn test_boxstats_outliers() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier
    let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();

    assert!(!stats.outliers.is_empty());
    assert!(stats.outliers.contains(&100.0));
}

#[test]
fn test_boxplot_orientation() {
    let bp = BoxPlot::new().horizontal();
    assert_eq!(bp.orientation, ChartOrientation::Horizontal);

    let bp = BoxPlot::new().vertical();
    assert_eq!(bp.orientation, ChartOrientation::Vertical);
}

#[test]
fn test_boxplot_whisker_style() {
    let bp = BoxPlot::new().whisker_style(WhiskerStyle::MinMax);
    assert_eq!(bp.whisker_style, WhiskerStyle::MinMax);

    let bp = BoxPlot::new().whisker_style(WhiskerStyle::Percentile);
    assert_eq!(bp.whisker_style, WhiskerStyle::Percentile);
}

#[test]
fn test_boxplot_notched() {
    let bp = BoxPlot::new().notched(true);
    assert!(bp.notched);
}

#[test]
fn test_boxplot_show_outliers() {
    let bp = BoxPlot::new().show_outliers(false);
    assert!(!bp.show_outliers);
}

#[test]
fn test_boxplot_box_width() {
    let bp = BoxPlot::new().box_width(0.8);
    assert_eq!(bp.box_width, 0.8);
}

#[test]
fn test_boxplot_builder() {
    let bp = BoxPlot::new()
        .title("Distribution")
        .group("A", &[1.0, 2.0, 3.0])
        .group("B", &[4.0, 5.0, 6.0])
        .value_axis(Axis::new().title("Value"))
        .whisker_style(WhiskerStyle::IQR)
        .show_outliers(true)
        .notched(false);

    assert_eq!(bp.title, Some("Distribution".to_string()));
    assert_eq!(bp.groups.len(), 2);
}

#[test]
fn test_boxplot_helper() {
    let bp = boxplot();
    assert!(bp.groups.is_empty());
}

#[test]
fn test_box_group() {
    let group = BoxGroup::new("Test", &[1.0, 2.0, 3.0]);
    assert_eq!(group.label, "Test");
    assert_eq!(group.data.len(), 3);

    let group = group.color(Color::RED);
    assert_eq!(group.color, Some(Color::RED));
}
