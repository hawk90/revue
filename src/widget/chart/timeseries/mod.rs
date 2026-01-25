//! Time Series Chart Widget
//!
//! A specialized chart for visualizing time-based data with proper time axis formatting,
//! auto-scaling, and support for multiple series.

mod helpers;
mod types;
mod view;

pub use helpers::{cpu_chart, memory_chart, network_chart, time_series, time_series_with_data};
pub use types::{
    MarkerStyle, TimeFormat, TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeriesData,
};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

use types::{TimeFormat as Format, TimeRange as Range};

/// Time Series Chart widget
#[derive(Debug, Clone)]
pub struct TimeSeries {
    /// Chart title
    title: Option<String>,
    /// Data series
    series: Vec<TimeSeriesData>,
    /// Time format
    time_format: TimeFormat,
    /// Time range
    time_range: TimeRange,
    /// Y-axis label
    y_label: Option<String>,
    /// Show grid
    show_grid: bool,
    /// Show legend
    show_legend: bool,
    /// Y-axis min value
    y_min: Option<f64>,
    /// Y-axis max value
    y_max: Option<f64>,
    /// Markers
    markers: Vec<TimeMarker>,
    /// Background color
    bg_color: Option<Color>,
    /// Grid color
    grid_color: Color,
    /// Height
    height: Option<u16>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Default for TimeSeries {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeSeries {
    /// Create a new time series chart
    pub fn new() -> Self {
        Self {
            title: None,
            series: Vec::new(),
            time_format: Format::Auto,
            time_range: Range::All,
            y_label: None,
            show_grid: true,
            show_legend: true,
            y_min: None,
            y_max: None,
            markers: Vec::new(),
            bg_color: None,
            grid_color: Color::rgb(60, 60, 60),
            height: None,
            props: WidgetProps::new(),
        }
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a data series
    pub fn series(mut self, data: TimeSeriesData) -> Self {
        self.series.push(data);
        self
    }

    /// Set time format
    pub fn time_format(mut self, format: TimeFormat) -> Self {
        self.time_format = format;
        self
    }

    /// Set time range
    pub fn time_range(mut self, range: TimeRange) -> Self {
        self.time_range = range;
        self
    }

    /// Set Y-axis label
    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    /// Show or hide grid
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Show or hide legend
    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Set Y-axis minimum
    pub fn y_min(mut self, min: f64) -> Self {
        self.y_min = Some(min);
        self
    }

    /// Set Y-axis maximum
    pub fn y_max(mut self, max: f64) -> Self {
        self.y_max = Some(max);
        self
    }

    /// Set Y-axis range
    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_min = Some(min);
        self.y_max = Some(max);
        self
    }

    /// Add a marker
    pub fn marker(mut self, marker: TimeMarker) -> Self {
        self.markers.push(marker);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set grid color
    pub fn grid_color(mut self, color: Color) -> Self {
        self.grid_color = color;
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Get time bounds for the current time range
    pub fn get_time_bounds(&self) -> (u64, u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        match self.time_range {
            Range::All => {
                let mut min_ts = u64::MAX;
                let mut max_ts = 0u64;
                for series in &self.series {
                    for point in &series.points {
                        min_ts = min_ts.min(point.timestamp);
                        max_ts = max_ts.max(point.timestamp);
                    }
                }
                if min_ts == u64::MAX {
                    (now - 3600, now)
                } else {
                    (min_ts, max_ts)
                }
            }
            Range::LastSeconds(s) => (now.saturating_sub(s), now),
            Range::LastMinutes(m) => (now.saturating_sub(m * 60), now),
            Range::LastHours(h) => (now.saturating_sub(h * 3600), now),
            Range::LastDays(d) => (now.saturating_sub(d * 86400), now),
            Range::Range { start, end } => (start, end),
        }
    }

    /// Get value bounds for the current data
    pub fn get_value_bounds(&self) -> (f64, f64) {
        let (time_min, time_max) = self.get_time_bounds();

        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        for series in &self.series {
            for point in &series.points {
                if point.timestamp >= time_min && point.timestamp <= time_max {
                    min_val = min_val.min(point.value);
                    max_val = max_val.max(point.value);
                }
            }
        }

        if min_val == f64::MAX {
            min_val = 0.0;
            max_val = 100.0;
        }

        let min = self.y_min.unwrap_or(min_val);
        let max = self.y_max.unwrap_or(max_val);

        let range = max - min;
        let padding = if range == 0.0 { 1.0 } else { range * 0.1 };

        (
            self.y_min.unwrap_or(min - padding),
            self.y_max.unwrap_or(max + padding),
        )
    }

    /// Format a timestamp for display
    pub fn format_time(&self, timestamp: u64, range: u64) -> String {
        let format = match self.time_format {
            Format::Auto => {
                if range < 60 {
                    Format::Seconds
                } else if range < 3600 {
                    Format::Minutes
                } else if range < 86400 {
                    Format::Hours
                } else if range < 86400 * 30 {
                    Format::Days
                } else {
                    Format::Months
                }
            }
            f => f,
        };

        let secs = timestamp % 60;
        let mins = (timestamp / 60) % 60;
        let hours = (timestamp / 3600) % 24;
        let days = (timestamp / 86400) % 31;
        let months = ((timestamp / 86400) / 30) % 12;

        match format {
            Format::Seconds => format!("{:02}:{:02}:{:02}", hours, mins, secs),
            Format::Minutes => format!("{:02}:{:02}", hours, mins),
            Format::Hours => format!("{:02}:00", hours),
            Format::Days => format!("{:02}/{:02}", months + 1, days + 1),
            Format::Months => format!("{}/{:02}", 1970 + timestamp / 31536000, months + 1),
            Format::Unix => format!("{}", timestamp),
            Format::Relative => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let diff = now.saturating_sub(timestamp);
                if diff < 60 {
                    format!("{}s", diff)
                } else if diff < 3600 {
                    format!("{}m", diff / 60)
                } else if diff < 86400 {
                    format!("{}h", diff / 3600)
                } else {
                    format!("{}d", diff / 86400)
                }
            }
            Format::Auto => {
                if timestamp < 3600 {
                    format!("{:02}:{:02}:{:02}", hours, mins, secs)
                } else if timestamp < 86400 {
                    format!("{:02}:{:02}", hours, mins)
                } else if timestamp < 31536000 {
                    format!("{:02}/{:02}", months + 1, days + 1)
                } else {
                    format!("{}/{:02}", 1970 + timestamp / 31536000, months + 1)
                }
            }
        }
    }
}

impl_styled_view!(TimeSeries);
impl_props_builders!(TimeSeries);

#[cfg(test)]
mod tests {
    //! Tests for Time Series widget

    use super::helpers::{
        cpu_chart, memory_chart, network_chart, time_series, time_series_with_data,
    };
    use super::types::*;
    use super::TimeSeries;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::traits::{RenderContext, View};

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
        let chart = TimeSeries::new();
        assert!(chart.title.is_none());
        assert!(chart.series.is_empty());
        assert!(chart.show_grid);
        assert!(chart.show_legend);
    }

    #[test]
    fn test_time_series_default() {
        let chart = TimeSeries::default();
        assert!(chart.title.is_none());
    }

    #[test]
    fn test_time_series_title() {
        let chart = TimeSeries::new().title("CPU Usage");
        assert_eq!(chart.title, Some("CPU Usage".to_string()));
    }

    #[test]
    fn test_time_series_series() {
        let chart = TimeSeries::new()
            .series(TimeSeriesData::new("Data1"))
            .series(TimeSeriesData::new("Data2"));
        assert_eq!(chart.series.len(), 2);
    }

    #[test]
    fn test_time_series_time_format() {
        let chart = TimeSeries::new().time_format(TimeFormat::Seconds);
        assert_eq!(chart.time_format, TimeFormat::Seconds);
    }

    #[test]
    fn test_time_series_time_range() {
        let chart = TimeSeries::new().time_range(TimeRange::LastMinutes(30));
        matches!(chart.time_range, TimeRange::LastMinutes(30));
    }

    #[test]
    fn test_time_series_y_label() {
        let chart = TimeSeries::new().y_label("%");
        assert_eq!(chart.y_label, Some("%".to_string()));
    }

    #[test]
    fn test_time_series_show_grid() {
        let chart = TimeSeries::new().show_grid(false);
        assert!(!chart.show_grid);
    }

    #[test]
    fn test_time_series_show_legend() {
        let chart = TimeSeries::new().show_legend(false);
        assert!(!chart.show_legend);
    }

    #[test]
    fn test_time_series_y_min_max() {
        let chart = TimeSeries::new().y_min(0.0).y_max(100.0);
        assert_eq!(chart.y_min, Some(0.0));
        assert_eq!(chart.y_max, Some(100.0));
    }

    #[test]
    fn test_time_series_y_range() {
        let chart = TimeSeries::new().y_range(-50.0, 50.0);
        assert_eq!(chart.y_min, Some(-50.0));
        assert_eq!(chart.y_max, Some(50.0));
    }

    #[test]
    fn test_time_series_marker() {
        let chart = TimeSeries::new()
            .marker(TimeMarker::new(1000, "Event1"))
            .marker(TimeMarker::new(2000, "Event2"));
        assert_eq!(chart.markers.len(), 2);
    }

    #[test]
    fn test_time_series_bg() {
        let chart = TimeSeries::new().bg(Color::BLACK);
        assert_eq!(chart.bg_color, Some(Color::BLACK));
    }

    #[test]
    fn test_time_series_grid_color() {
        let chart = TimeSeries::new().grid_color(Color::WHITE);
        assert_eq!(chart.grid_color, Color::WHITE);
    }

    #[test]
    fn test_time_series_height() {
        let chart = TimeSeries::new().height(20);
        assert_eq!(chart.height, Some(20));
    }

    // ========================================================================
    // Helper function tests
    // ========================================================================

    #[test]
    fn test_time_series_helper() {
        let chart = time_series();
        assert!(chart.title.is_none());
    }

    #[test]
    fn test_time_series_with_data_helper() {
        let chart = time_series_with_data(TimeSeriesData::new("Data").point(100, 50.0));
        assert_eq!(chart.series.len(), 1);
    }

    #[test]
    fn test_cpu_chart_helper() {
        let chart = cpu_chart(vec![(1000, 50.0), (2000, 60.0)]);
        assert_eq!(chart.title, Some("CPU Usage".to_string()));
        assert_eq!(chart.series.len(), 1);
    }

    #[test]
    fn test_memory_chart_helper() {
        let chart = memory_chart(vec![(1000, 4.0), (2000, 5.0)]);
        assert_eq!(chart.title, Some("Memory Usage".to_string()));
    }

    #[test]
    fn test_network_chart_helper() {
        let chart = network_chart(vec![(1000, 100.0)], vec![(1000, 50.0)]);
        assert_eq!(chart.title, Some("Network Traffic".to_string()));
        assert_eq!(chart.series.len(), 2);
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
}
