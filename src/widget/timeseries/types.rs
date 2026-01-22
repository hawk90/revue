//! Time Series widget types

use crate::style::Color;

/// A single data point in a time series
#[derive(Debug, Clone)]
pub struct TimePoint {
    /// Unix timestamp in seconds
    pub timestamp: u64,
    /// Value at this time
    pub value: f64,
}

impl TimePoint {
    /// Create a new time point
    pub fn new(timestamp: u64, value: f64) -> Self {
        Self { timestamp, value }
    }

    /// Create from current time
    pub fn now(value: f64) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self { timestamp, value }
    }
}

/// A series of time-based data points
#[derive(Debug, Clone)]
pub struct TimeSeriesData {
    /// Name of this series
    pub name: String,
    /// Data points
    pub points: Vec<TimePoint>,
    /// Line color
    pub color: Color,
    /// Line style
    pub line_style: TimeLineStyle,
    /// Whether to fill area under the line
    pub fill: bool,
}

impl TimeSeriesData {
    /// Create a new time series
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            points: Vec::new(),
            color: Color::CYAN,
            line_style: TimeLineStyle::Solid,
            fill: false,
        }
    }

    /// Add a data point
    pub fn point(mut self, timestamp: u64, value: f64) -> Self {
        self.points.push(TimePoint::new(timestamp, value));
        self
    }

    /// Add multiple points
    pub fn points(mut self, points: Vec<(u64, f64)>) -> Self {
        for (ts, val) in points {
            self.points.push(TimePoint::new(ts, val));
        }
        self
    }

    /// Set line color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set line style
    pub fn line_style(mut self, style: TimeLineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Enable area fill
    pub fn filled(mut self) -> Self {
        self.fill = true;
        self
    }
}

/// Line style for time series
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeLineStyle {
    /// Solid line
    #[default]
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Step function line
    Step,
}

/// Time format for the X axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeFormat {
    /// Auto-detect based on time range
    #[default]
    Auto,
    /// Seconds (HH:MM:SS)
    Seconds,
    /// Minutes (HH:MM)
    Minutes,
    /// Hours (HH:00)
    Hours,
    /// Days (MM/DD)
    Days,
    /// Months (YYYY/MM)
    Months,
    /// Unix timestamp
    Unix,
    /// Relative (e.g., "5m ago")
    Relative,
}

/// Time range for the chart
#[derive(Debug, Clone, Copy, Default)]
pub enum TimeRange {
    /// Show all data
    #[default]
    All,
    /// Last N seconds
    LastSeconds(u64),
    /// Last N minutes
    LastMinutes(u64),
    /// Last N hours
    LastHours(u64),
    /// Last N days
    LastDays(u64),
    /// Custom range
    Range {
        /// Start timestamp
        start: u64,
        /// End timestamp
        end: u64,
    },
}

/// Marker on the time series chart
#[derive(Debug, Clone)]
pub struct TimeMarker {
    /// Timestamp for the marker
    pub timestamp: u64,
    /// Label for the marker
    pub label: String,
    /// Marker color
    pub color: Color,
    /// Marker style
    pub style: MarkerStyle,
}

impl TimeMarker {
    /// Create a new marker
    pub fn new(timestamp: u64, label: impl Into<String>) -> Self {
        Self {
            timestamp,
            label: label.into(),
            color: Color::YELLOW,
            style: MarkerStyle::Line,
        }
    }

    /// Set marker color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set marker style
    pub fn style(mut self, style: MarkerStyle) -> Self {
        self.style = style;
        self
    }
}

/// Style for time markers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerStyle {
    /// Vertical line marker
    #[default]
    Line,
    /// Point marker
    Point,
    /// Region marker
    Region,
}
