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

use super::traits::WidgetProps;
use crate::style::Color;
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
mod tests;
