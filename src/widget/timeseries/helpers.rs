//! Convenience constructors for Time Series widget

use super::types::{TimePoint, TimeRange, TimeSeriesData};
use super::TimeSeries;
use crate::style::Color;

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
