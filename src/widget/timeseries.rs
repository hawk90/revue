//! Time Series Chart Widget
//!
//! A specialized chart for visualizing time-based data with proper time axis formatting,
//! auto-scaling, and support for multiple series.
//!
//! # Features
//!
//! - Automatic time axis formatting (seconds, minutes, hours, days)
//! - Multiple series support with different line styles
//! - Real-time data streaming support
//! - Markers for important events
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{time_series, TimeSeriesData};
//!
//! let data = TimeSeriesData::new("CPU")
//!     .point(now - 3600, 10.0)
//!     .point(now - 2400, 15.0)
//!     .point(now, 18.0);
//!
//! let chart = time_series()
//!     .title("CPU Usage")
//!     .series(data)
//!     .time_format(TimeFormat::Auto);
//! ```

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

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
            time_format: TimeFormat::Auto,
            time_range: TimeRange::All,
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

    fn get_time_bounds(&self) -> (u64, u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        match self.time_range {
            TimeRange::All => {
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
            TimeRange::LastSeconds(s) => (now.saturating_sub(s), now),
            TimeRange::LastMinutes(m) => (now.saturating_sub(m * 60), now),
            TimeRange::LastHours(h) => (now.saturating_sub(h * 3600), now),
            TimeRange::LastDays(d) => (now.saturating_sub(d * 86400), now),
            TimeRange::Range { start, end } => (start, end),
        }
    }

    fn get_value_bounds(&self) -> (f64, f64) {
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

    fn format_time(&self, timestamp: u64, range: u64) -> String {
        let format = match self.time_format {
            TimeFormat::Auto => {
                if range < 60 {
                    TimeFormat::Seconds
                } else if range < 3600 {
                    TimeFormat::Minutes
                } else if range < 86400 {
                    TimeFormat::Hours
                } else if range < 86400 * 30 {
                    TimeFormat::Days
                } else {
                    TimeFormat::Months
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
            TimeFormat::Seconds => format!("{:02}:{:02}:{:02}", hours, mins, secs),
            TimeFormat::Minutes => format!("{:02}:{:02}", hours, mins),
            TimeFormat::Hours => format!("{:02}:00", hours),
            TimeFormat::Days => format!("{:02}/{:02}", months + 1, days + 1),
            TimeFormat::Months => format!("{}/{:02}", 1970 + timestamp / 31536000, months + 1),
            TimeFormat::Unix => format!("{}", timestamp),
            TimeFormat::Relative => {
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
            TimeFormat::Auto => {
                // Auto-detect best format based on timestamp magnitude
                if timestamp < 3600 {
                    // Less than an hour - show seconds
                    format!("{:02}:{:02}:{:02}", hours, mins, secs)
                } else if timestamp < 86400 {
                    // Less than a day - show hours:minutes
                    format!("{:02}:{:02}", hours, mins)
                } else if timestamp < 31536000 {
                    // Less than a year - show month/day
                    format!("{:02}/{:02}", months + 1, days + 1)
                } else {
                    // Year or more - show year/month
                    format!("{}/{:02}", 1970 + timestamp / 31536000, months + 1)
                }
            }
        }
    }
}

impl View for TimeSeries {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 10 || height < 5 {
            return;
        }

        let mut current_y = area.y;

        // Background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + height.min(area.height) {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Title
        if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            ctx.buffer
                .put_str_styled(title_x, current_y, title, Some(Color::WHITE), self.bg_color);
            current_y += 1;
        }

        // Legend
        if self.show_legend && !self.series.is_empty() {
            let mut x = area.x + 2;
            for series in &self.series {
                let marker = match series.line_style {
                    TimeLineStyle::Solid => "─",
                    TimeLineStyle::Dashed => "╌",
                    TimeLineStyle::Dotted => "·",
                    TimeLineStyle::Step => "┐",
                };
                ctx.buffer
                    .put_str_styled(x, current_y, marker, Some(series.color), self.bg_color);
                x += 2;
                ctx.buffer.put_str_styled(
                    x,
                    current_y,
                    &series.name,
                    Some(Color::WHITE),
                    self.bg_color,
                );
                x += series.name.len() as u16 + 3;
            }
            current_y += 1;
        }

        let y_label_width = 8u16;
        let plot_x = area.x + y_label_width;
        let plot_width = area.width.saturating_sub(y_label_width + 1);
        let plot_y = current_y;
        let plot_height = height.saturating_sub(current_y - area.y + 2);

        if plot_width < 5 || plot_height < 3 {
            return;
        }

        let (time_min, time_max) = self.get_time_bounds();
        let (val_min, val_max) = self.get_value_bounds();
        let time_range = time_max.saturating_sub(time_min);
        let val_range = val_max - val_min;

        // Draw grid
        if self.show_grid {
            let grid_rows = 4.min(plot_height as usize);
            for i in 0..=grid_rows {
                let y = plot_y + (i * plot_height as usize / grid_rows) as u16;
                for x in plot_x..plot_x + plot_width {
                    let ch = if i == grid_rows { '─' } else { '┄' };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.grid_color);
                    ctx.buffer.set(x, y, cell);
                }

                // Y-axis labels
                let val = val_max - (i as f64 * val_range / grid_rows as f64);
                let label = if val.abs() >= 1000.0 {
                    format!("{:.1}k", val / 1000.0)
                } else if val.abs() >= 1.0 {
                    format!("{:.1}", val)
                } else {
                    format!("{:.2}", val)
                };
                let label_x = area.x + y_label_width.saturating_sub(label.len() as u16 + 1);
                ctx.buffer
                    .put_str_styled(label_x, y, &label, Some(Color::WHITE), self.bg_color);
            }
        }

        // Draw markers
        for marker in &self.markers {
            if marker.timestamp >= time_min && marker.timestamp <= time_max && time_range > 0 {
                let x_pos = ((marker.timestamp - time_min) as f64 / time_range as f64
                    * (plot_width - 1) as f64) as u16;
                let x = plot_x + x_pos;

                match marker.style {
                    MarkerStyle::Line => {
                        for y in plot_y..plot_y + plot_height {
                            let mut cell = Cell::new('│');
                            cell.fg = Some(marker.color);
                            ctx.buffer.set(x, y, cell);
                        }
                    }
                    MarkerStyle::Point | MarkerStyle::Region => {
                        let mut cell = Cell::new('▼');
                        cell.fg = Some(marker.color);
                        ctx.buffer.set(x, plot_y, cell);
                    }
                }

                if !marker.label.is_empty() {
                    let label_x = x.saturating_sub(marker.label.len() as u16 / 2);
                    ctx.buffer.put_str_styled(
                        label_x,
                        plot_y + plot_height + 1,
                        &marker.label,
                        Some(marker.color),
                        self.bg_color,
                    );
                }
            }
        }

        // Draw series
        for series in &self.series {
            let filtered_points: Vec<_> = series
                .points
                .iter()
                .filter(|p| p.timestamp >= time_min && p.timestamp <= time_max)
                .collect();

            if filtered_points.is_empty() {
                continue;
            }

            // Map points to screen coordinates
            let screen_points: Vec<(u16, u16)> = filtered_points
                .iter()
                .map(|p| {
                    let x_ratio = if time_range > 0 {
                        (p.timestamp - time_min) as f64 / time_range as f64
                    } else {
                        0.5
                    };
                    let y_ratio = if val_range > 0.0 {
                        (p.value - val_min) / val_range
                    } else {
                        0.5
                    };

                    let x = plot_x + (x_ratio * (plot_width - 1) as f64) as u16;
                    let y = plot_y + plot_height - 1 - (y_ratio * (plot_height - 1) as f64) as u16;
                    (x, y)
                })
                .collect();

            // Draw lines between points
            for i in 0..screen_points.len().saturating_sub(1) {
                let (x1, y1) = screen_points[i];
                let (x2, y2) = screen_points[i + 1];

                match series.line_style {
                    TimeLineStyle::Step => {
                        for x in x1..=x2 {
                            let mut cell = Cell::new('─');
                            cell.fg = Some(series.color);
                            ctx.buffer.set(x, y1, cell);
                        }
                        let (start_y, end_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                        for y in start_y..=end_y {
                            let mut cell = Cell::new('│');
                            cell.fg = Some(series.color);
                            ctx.buffer.set(x2, y, cell);
                        }
                    }
                    _ => {
                        // Simple line drawing
                        let dx = (x2 as i32 - x1 as i32).abs();
                        let dy = (y2 as i32 - y1 as i32).abs();
                        let sx = if x1 < x2 { 1i32 } else { -1i32 };
                        let sy = if y1 < y2 { 1i32 } else { -1i32 };
                        let mut err = dx - dy;
                        let mut x = x1 as i32;
                        let mut y = y1 as i32;
                        let mut step = 0;

                        loop {
                            let ch = match series.line_style {
                                TimeLineStyle::Solid => {
                                    if dx > dy {
                                        '─'
                                    } else {
                                        '│'
                                    }
                                }
                                TimeLineStyle::Dashed => {
                                    if step % 2 == 0 {
                                        if dx > dy {
                                            '╌'
                                        } else {
                                            '╎'
                                        }
                                    } else {
                                        ' '
                                    }
                                }
                                TimeLineStyle::Dotted => {
                                    if step % 2 == 0 {
                                        '·'
                                    } else {
                                        ' '
                                    }
                                }
                                // Step is handled by the outer match (line 673), use fallback here
                                TimeLineStyle::Step => '─',
                            };

                            if ch != ' ' {
                                let mut cell = Cell::new(ch);
                                cell.fg = Some(series.color);
                                ctx.buffer.set(x as u16, y as u16, cell);
                            }

                            if x == x2 as i32 && y == y2 as i32 {
                                break;
                            }

                            let e2 = 2 * err;
                            if e2 > -dy {
                                err -= dy;
                                x += sx;
                            }
                            if e2 < dx {
                                err += dx;
                                y += sy;
                            }
                            step += 1;
                        }
                    }
                }

                // Fill area if enabled
                if series.fill {
                    let bottom_y = plot_y + plot_height - 1;
                    for x in x1..=x2 {
                        let y_at_x = if x2 != x1 {
                            let t = (x - x1) as f64 / (x2 - x1) as f64;
                            (y1 as f64 + t * (y2 as f64 - y1 as f64)) as u16
                        } else {
                            y1
                        };
                        for y in y_at_x..=bottom_y {
                            let fill_color = Color::rgb(
                                (series.color.r as u16 * 3 / 10) as u8,
                                (series.color.g as u16 * 3 / 10) as u8,
                                (series.color.b as u16 * 3 / 10) as u8,
                            );
                            let mut cell = Cell::new(' ');
                            cell.bg = Some(fill_color);
                            ctx.buffer.set(x, y, cell);
                        }
                    }
                }
            }

            // Draw points
            for &(x, y) in &screen_points {
                let mut cell = Cell::new('●');
                cell.fg = Some(series.color);
                ctx.buffer.set(x, y, cell);
            }
        }

        // X-axis time labels
        let x_label_y = plot_y + plot_height + 1;
        if x_label_y < area.y + height {
            let num_labels = (plot_width / 12).max(2) as usize;
            for i in 0..num_labels {
                let ratio = i as f64 / (num_labels - 1) as f64;
                let ts = time_min + (ratio * time_range as f64) as u64;
                let label = self.format_time(ts, time_range);
                let x = plot_x + (ratio * (plot_width - 1) as f64) as u16;
                let label_x = x.saturating_sub(label.len() as u16 / 2);
                ctx.buffer.put_str_styled(
                    label_x,
                    x_label_y,
                    &label,
                    Some(Color::WHITE),
                    self.bg_color,
                );
            }
        }
    }

    crate::impl_view_meta!("TimeSeries");
}

impl_styled_view!(TimeSeries);
impl_props_builders!(TimeSeries);

// Convenience constructors

/// Create a new time series chart
pub fn time_series() -> TimeSeries {
    TimeSeries::new()
}

/// Create a time series chart with data
pub fn time_series_with_data(data: TimeSeriesData) -> TimeSeries {
    TimeSeries::new().series(data)
}

/// Create a CPU usage chart
pub fn cpu_chart(values: Vec<(u64, f64)>) -> TimeSeries {
    let mut series = TimeSeriesData::new("CPU").color(Color::CYAN);
    for (ts, val) in values {
        series.points.push(TimePoint::new(ts, val));
    }

    TimeSeries::new()
        .title("CPU Usage")
        .series(series)
        .time_range(TimeRange::LastHours(1))
        .y_range(0.0, 100.0)
        .y_label("%")
}

/// Create a memory usage chart
pub fn memory_chart(values: Vec<(u64, f64)>) -> TimeSeries {
    let mut series = TimeSeriesData::new("Memory").color(Color::MAGENTA).filled();
    for (ts, val) in values {
        series.points.push(TimePoint::new(ts, val));
    }

    TimeSeries::new()
        .title("Memory Usage")
        .series(series)
        .time_range(TimeRange::LastHours(1))
        .y_range(0.0, 100.0)
        .y_label("GB")
}

/// Create a network traffic chart
pub fn network_chart(rx_values: Vec<(u64, f64)>, tx_values: Vec<(u64, f64)>) -> TimeSeries {
    let mut rx_series = TimeSeriesData::new("RX").color(Color::GREEN);
    for (ts, val) in rx_values {
        rx_series.points.push(TimePoint::new(ts, val));
    }

    let mut tx_series = TimeSeriesData::new("TX").color(Color::BLUE);
    for (ts, val) in tx_values {
        tx_series.points.push(TimePoint::new(ts, val));
    }

    TimeSeries::new()
        .title("Network Traffic")
        .series(rx_series)
        .series(tx_series)
        .time_range(TimeRange::LastMinutes(5))
        .y_label("Mbps")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

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
