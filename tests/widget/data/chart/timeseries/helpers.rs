//! TimeSeries constructor function tests: time_series, time_series_with_data and the cpu, memory and network charts

use super::super::render_rows;
use super::fixed_range;
use revue::widget::data::chart::timeseries::{
    cpu_chart, memory_chart, network_chart, time_series, time_series_with_data, TimeSeries,
    TimeSeriesData,
};

#[test]
fn test_time_series_with_data_helper() {
    let chart = time_series_with_data(TimeSeriesData::new("Data").point(100, 50.0));
    // One series with a data point: value bounds should reflect it
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 50.0);
    assert!(max >= 50.0);
}

#[test]
fn test_time_series_helper() {
    assert_eq!(
        render_rows(&time_series().time_range(fixed_range()), 60, 20),
        render_rows(&TimeSeries::new().time_range(fixed_range()), 60, 20)
    );
}

#[test]
fn test_cpu_chart_helper() {
    let chart = cpu_chart(vec![(1000, 50.0), (2000, 60.0)]);
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "CPU Usage");
    assert_eq!(rows[1].trim(), "─ CPU");
    assert_eq!(rows[2].trim(), "%");
    // y_range is set to 0..100
    let (min, max) = chart.get_value_bounds();
    assert_eq!(min, 0.0);
    assert_eq!(max, 100.0);
}

#[test]
fn test_memory_chart_helper() {
    let chart = memory_chart(vec![(1000, 4.0), (2000, 5.0)]);
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "Memory Usage");
    assert_eq!(rows[1].trim(), "─ Memory");
    assert_eq!(rows[2].trim(), "GB");
}

#[test]
fn test_network_chart_helper() {
    let chart = network_chart(vec![(1000, 100.0)], vec![(1000, 50.0)]);
    let rows = render_rows(&chart, 40, 12);
    assert_eq!(rows[0].trim(), "Network Traffic");
    assert_eq!(rows[1].trim(), "─ RX   ─ TX");
    assert_eq!(rows[2].trim(), "Mbps");
    // Two series: RX and TX
    let (min, max) = chart.get_value_bounds();
    assert!(min <= 50.0);
    assert!(max >= 100.0);
}
