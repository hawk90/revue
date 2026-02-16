//! Tests for scatter chart public API
//!
//! Extracted from src/widget/data/chart/scatterchart.rs

use revue::widget::data::chart::{
    ScatterChart, ScatterSeries, Axis, ChartGrid, Legend, LegendPosition, Marker,
    scatter_chart, bubble_chart,
};
use revue::style::Color;

#[test]
fn test_scatter_chart_new() {
    let chart = ScatterChart::new();
    assert!(chart.series.is_empty());
}

#[test]
fn test_scatter_series() {
    let series = ScatterSeries::new("Test")
        .points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
        .color(Color::RED)
        .marker(Marker::Star);

    assert_eq!(series.name, "Test");
    assert_eq!(series.data.len(), 3);
    assert_eq!(series.color, Some(Color::RED));
    assert_eq!(series.marker, Marker::Star);
}

#[test]
fn test_scatter_chart_series() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("A").points(&[(1.0, 1.0)]))
        .series(ScatterSeries::new("B").points(&[(2.0, 2.0)]));

    assert_eq!(chart.series.len(), 2);
}

#[test]
fn test_scatter_chart_bounds() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("Test").points(&[(0.0, 0.0), (10.0, 20.0)]));

    let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
    assert!(x_min < 0.0);
    assert!(x_max > 10.0);
    assert!(y_min < 0.0);
    assert!(y_max > 20.0);
}

#[test]
fn test_scatter_chart_axis_override() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("Test").points(&[(5.0, 5.0)]))
        .x_axis(Axis::new().bounds(0.0, 100.0))
        .y_axis(Axis::new().bounds(0.0, 50.0));

    let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
    assert!(x_min <= 0.0);
    assert!(x_max >= 100.0);
    assert!(y_min <= 0.0);
    assert!(y_max >= 50.0);
}

#[test]
fn test_bubble_chart() {
    let series = ScatterSeries::new("Bubbles")
        .points(&[(1.0, 1.0), (2.0, 2.0)])
        .sizes(vec![10.0, 50.0]);

    assert!(series.sizes.is_some());
    assert_eq!(series.sizes.as_ref().unwrap().len(), 2);
}

#[test]
fn test_scatter_chart_legend() {
    let chart = ScatterChart::new().legend(Legend::bottom_left());
    assert_eq!(chart.legend.position, LegendPosition::BottomLeft);

    let chart = ScatterChart::new().no_legend();
    assert!(!chart.legend.is_visible());
}

#[test]
fn test_scatter_chart_grid() {
    let chart = ScatterChart::new().grid(ChartGrid::y_only());
    assert!(!chart.grid.x);
    assert!(chart.grid.y);
}

#[test]
fn test_scatter_chart_builder() {
    let chart = ScatterChart::new()
        .title("Scatter Plot")
        .series(ScatterSeries::new("Data").points(&[(1.0, 1.0)]))
        .x_axis(Axis::new().title("X"))
        .y_axis(Axis::new().title("Y"))
        .legend(Legend::top_right())
        .grid(ChartGrid::both());

    assert_eq!(chart.title, Some("Scatter Plot".to_string()));
    assert_eq!(chart.series.len(), 1);
    assert!(chart.grid.x);
    assert!(chart.grid.y);
}

#[test]
fn test_scatter_helpers() {
    let chart = scatter_chart();
    assert!(chart.series.is_empty());

    let chart = bubble_chart();
    assert!(chart.series.is_empty());
}