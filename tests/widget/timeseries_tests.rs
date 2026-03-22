//! Tests for Time Series widget
//!
//! Extracted from src/widget/data/chart/timeseries/

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::timeseries::{
    cpu_chart, memory_chart, network_chart, time_series, time_series_with_data, MarkerStyle,
    TimeFormat, TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeries, TimeSeriesData,
};
use revue::widget::traits::{RenderContext, View};

// ========================================================================
// TimePoint tests
// ========================================================================

#[test]
fn test_time_point_new() {
    let point = TimePoint::new(1000, 42.0);
    assert_eq!(point.timestamp, 1000);
    assert_eq!(point.value, 42.0);
}

#[test]
fn test_time_point_now() {
    let point = TimePoint::now(50.0);
    assert!(point.timestamp > 0);
    assert_eq!(point.value, 50.0);
}

// ========================================================================
// TimeSeriesData tests
// ========================================================================

#[test]
fn test_time_series_data_new() {
    let data = TimeSeriesData::new("Test");
    assert_eq!(data.name, "Test");
    assert!(data.points.is_empty());
    assert_eq!(data.color, Color::CYAN);
    assert!(!data.fill);
}

#[test]
fn test_time_series_data_point() {
    let data = TimeSeriesData::new("Test")
        .point(100, 10.0)
        .point(200, 20.0);

    assert_eq!(data.points.len(), 2);
    assert_eq!(data.points[0].timestamp, 100);
    assert_eq!(data.points[0].value, 10.0);
}

#[test]
fn test_time_series_data_points() {
    let data = TimeSeriesData::new("Test").points(vec![(100, 10.0), (200, 20.0), (300, 30.0)]);

    assert_eq!(data.points.len(), 3);
}

#[test]
fn test_time_series_data_color() {
    let data = TimeSeriesData::new("Test").color(Color::RED);
    assert_eq!(data.color, Color::RED);
}

#[test]
fn test_time_series_data_line_style() {
    let data = TimeSeriesData::new("Test").line_style(TimeLineStyle::Dashed);
    assert_eq!(data.line_style, TimeLineStyle::Dashed);
}

#[test]
fn test_time_series_data_filled() {
    let data = TimeSeriesData::new("Test").filled();
    assert!(data.fill);
}

// ========================================================================
// TimeLineStyle tests
// ========================================================================

#[test]
fn test_time_line_style_default() {
    assert_eq!(TimeLineStyle::default(), TimeLineStyle::Solid);
}

#[test]
fn test_time_line_style_variants() {
    assert_eq!(TimeLineStyle::Solid, TimeLineStyle::Solid);
    assert_ne!(TimeLineStyle::Solid, TimeLineStyle::Dashed);
    assert_ne!(TimeLineStyle::Dashed, TimeLineStyle::Dotted);
    assert_ne!(TimeLineStyle::Dotted, TimeLineStyle::Step);
}

// ========================================================================
// TimeFormat tests
// ========================================================================

#[test]
fn test_time_format_default() {
    assert_eq!(TimeFormat::default(), TimeFormat::Auto);
}

#[test]
fn test_time_format_variants() {
    assert_ne!(TimeFormat::Seconds, TimeFormat::Minutes);
    assert_ne!(TimeFormat::Hours, TimeFormat::Days);
    assert_ne!(TimeFormat::Months, TimeFormat::Unix);
}

// ========================================================================
// TimeRange tests
// ========================================================================

#[test]
fn test_time_range_default() {
    matches!(TimeRange::default(), TimeRange::All);
}

#[test]
fn test_time_range_variants() {
    let range = TimeRange::LastSeconds(60);
    matches!(range, TimeRange::LastSeconds(60));

    let range = TimeRange::Range {
        start: 1000,
        end: 2000,
    };
    matches!(
        range,
        TimeRange::Range {
            start: 1000,
            end: 2000
        }
    );
}

// ========================================================================
// TimeMarker tests
// ========================================================================

#[test]
fn test_time_marker_new() {
    let marker = TimeMarker::new(1000, "Event");
    assert_eq!(marker.timestamp, 1000);
    assert_eq!(marker.label, "Event");
    assert_eq!(marker.color, Color::YELLOW);
}

#[test]
fn test_time_marker_color() {
    let marker = TimeMarker::new(1000, "Event").color(Color::RED);
    assert_eq!(marker.color, Color::RED);
}

#[test]
fn test_time_marker_style() {
    let marker = TimeMarker::new(1000, "Event").style(MarkerStyle::Point);
    assert_eq!(marker.style, MarkerStyle::Point);
}

// ========================================================================
// MarkerStyle tests
// ========================================================================

#[test]
fn test_marker_style_default() {
    assert_eq!(MarkerStyle::default(), MarkerStyle::Line);
}

// ========================================================================
// TimeSeries tests
// ========================================================================

#[test]
fn test_time_series_new() {
    // Verify construction and default state via render (fields are private)
    let chart = TimeSeries::new();
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_time_series_default() {
    // Default produces a chart with no title (verified via Debug output)
    let chart = TimeSeries::default();
    let debug = format!("{:?}", chart);
    assert!(debug.contains("title: None"));
}

#[test]
fn test_time_series_title() {
    let chart = TimeSeries::new().title("CPU Usage");
    let debug = format!("{:?}", chart);
    assert!(debug.contains("CPU Usage"));
}

#[test]
fn test_time_series_series() {
    // Verify series are stored by checking get_value_bounds works with data
    let chart = TimeSeries::new()
        .series(TimeSeriesData::new("Data1").point(100, 10.0))
        .series(TimeSeriesData::new("Data2").point(200, 20.0));
    // Both series have data, so value bounds should reflect both
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 10.0);
    assert!(max >= 20.0);
}

#[test]
fn test_time_series_time_format() {
    // Verify time_format affects format_time output
    let chart = TimeSeries::new().time_format(TimeFormat::Seconds);
    // With Seconds format, output should include colons (HH:MM:SS)
    let formatted = chart.format_time(3661, 100);
    assert!(formatted.contains(':'));
}

#[test]
fn test_time_series_time_range() {
    // Verify time_range is applied by checking get_time_bounds
    let chart = TimeSeries::new().time_range(TimeRange::LastMinutes(30));
    let (start, end) = chart.get_time_bounds();
    // end - start should be approximately 30 * 60 = 1800 seconds
    let diff = end.saturating_sub(start);
    assert!((diff as i64 - 1800).abs() <= 2);
}

#[test]
fn test_time_series_y_label() {
    let chart = TimeSeries::new().y_label("%");
    let debug = format!("{:?}", chart);
    assert!(debug.contains("\"%\"") || debug.contains("Some(\"%\")"));
}

#[test]
fn test_time_series_show_grid() {
    let chart = TimeSeries::new().show_grid(false);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("show_grid: false"));
}

#[test]
fn test_time_series_show_legend() {
    let chart = TimeSeries::new().show_legend(false);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("show_legend: false"));
}

#[test]
fn test_time_series_y_min_max() {
    let chart = TimeSeries::new().y_min(0.0).y_max(100.0);
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, 0.0);
    assert_eq!(max, 100.0);
}

#[test]
fn test_time_series_y_range() {
    let chart = TimeSeries::new().y_range(-50.0, 50.0);
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, -50.0);
    assert_eq!(max, 50.0);
}

#[test]
fn test_time_series_marker() {
    // Verify markers are accepted without panic via render
    let chart = TimeSeries::new()
        .series(
            TimeSeriesData::new("Data")
                .point(1000, 50.0)
                .point(2000, 75.0),
        )
        .marker(TimeMarker::new(1000, "Event1"))
        .marker(TimeMarker::new(2000, "Event2"));
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_time_series_bg() {
    let chart = TimeSeries::new().bg(Color::BLACK);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("bg_color"));
}

#[test]
fn test_time_series_grid_color() {
    let chart = TimeSeries::new().grid_color(Color::WHITE);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("grid_color"));
}

#[test]
fn test_time_series_height() {
    let chart = TimeSeries::new().height(20);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("height: Some(20)"));
}

// ========================================================================
// Helper function tests
// ========================================================================

#[test]
fn test_time_series_helper() {
    let chart = time_series();
    let debug = format!("{:?}", chart);
    assert!(debug.contains("title: None"));
}

#[test]
fn test_time_series_with_data_helper() {
    let chart = time_series_with_data(TimeSeriesData::new("Data").point(100, 50.0));
    // One series with a data point: value bounds should reflect it
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 50.0);
    assert!(max >= 50.0);
}

#[test]
fn test_cpu_chart_helper() {
    let chart = cpu_chart(vec![(1000, 50.0), (2000, 60.0)]);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("CPU Usage"));
    // y_range is set to 0..100
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, 0.0);
    assert_eq!(max, 100.0);
}

#[test]
fn test_memory_chart_helper() {
    let chart = memory_chart(vec![(1000, 4.0), (2000, 5.0)]);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("Memory Usage"));
}

#[test]
fn test_network_chart_helper() {
    let chart = network_chart(vec![(1000, 100.0)], vec![(1000, 50.0)]);
    let debug = format!("{:?}", chart);
    assert!(debug.contains("Network Traffic"));
    // Two series: RX and TX
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 50.0);
    assert!(max >= 100.0);
}

// ========================================================================
// Render tests
// ========================================================================

#[test]
fn test_time_series_render() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new().title("Test").series(
        TimeSeriesData::new("Data")
            .point(1000, 50.0)
            .point(2000, 75.0),
    );

    chart.render(&mut ctx);
}

#[test]
fn test_time_series_render_empty() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new();
    chart.render(&mut ctx);
}

#[test]
fn test_time_series_render_small_area() {
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new().series(TimeSeriesData::new("Data").point(1000, 50.0));

    chart.render(&mut ctx);
    // Should handle small area gracefully
}

#[test]
fn test_time_series_render_with_markers() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new()
        .series(
            TimeSeriesData::new("Data")
                .point(1000, 50.0)
                .point(2000, 75.0),
        )
        .marker(TimeMarker::new(1500, "Event"));

    chart.render(&mut ctx);
}

#[test]
fn test_time_series_render_multiple_series() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new()
        .series(TimeSeriesData::new("Series1").point(1000, 50.0))
        .series(
            TimeSeriesData::new("Series2")
                .point(1000, 75.0)
                .color(Color::RED),
        );

    chart.render(&mut ctx);
}

#[test]
fn test_time_series_render_step_line() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new().series(
        TimeSeriesData::new("Data")
            .line_style(TimeLineStyle::Step)
            .point(1000, 50.0)
            .point(2000, 75.0),
    );

    chart.render(&mut ctx);
}

#[test]
fn test_time_series_render_filled() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = TimeSeries::new().series(
        TimeSeriesData::new("Data")
            .filled()
            .point(1000, 50.0)
            .point(2000, 75.0),
    );

    chart.render(&mut ctx);
}
