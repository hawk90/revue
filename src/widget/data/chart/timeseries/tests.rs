//! Time series unit tests that need private state
//!
//! The live tests in tests/widget/timeseries_tests.rs cover these builders only
//! through Debug output or a render that asserts nothing, which cannot tell
//! whether the value was stored. These read the fields directly.

use super::types::*;
use super::TimeSeries;
use crate::style::Color;

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
