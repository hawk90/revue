//! TimeSeries data type tests: TimePoint, TimeSeriesData, line styles, formats, ranges and markers

use revue::style::Color;
use revue::widget::data::chart::timeseries::{
    MarkerStyle, TimeFormat, TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeriesData,
};

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

#[test]
fn test_time_range_default() {
    assert!(matches!(TimeRange::default(), TimeRange::All));
}

#[test]
fn test_time_range_variants() {
    let range = TimeRange::LastSeconds(60);
    assert!(matches!(range, TimeRange::LastSeconds(60)));

    let range = TimeRange::Range {
        start: 1000,
        end: 2000,
    };
    assert!(matches!(
        range,
        TimeRange::Range {
            start: 1000,
            end: 2000
        }
    ));
}

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

#[test]
fn test_marker_style_default() {
    assert_eq!(MarkerStyle::default(), MarkerStyle::Line);
}
