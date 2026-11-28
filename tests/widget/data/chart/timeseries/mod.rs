//! TimeSeries widget tests

mod axes;
mod bounds;
mod helpers;
mod lines;
mod markers;
mod types;
mod view;

use revue::widget::data::chart::timeseries::{TimeRange, TimeSeriesData};

/// A series from (1000, 50) to (2000, 75)
pub(super) fn two_points() -> TimeSeriesData {
    TimeSeriesData::new("Data")
        .point(1000, 50.0)
        .point(2000, 75.0)
}

/// 2023-11-14 22:13:20 UTC
pub(super) const NOW: u64 = 1_700_000_000;

/// A mock clock whose time of day is `secs` after 1970, in UTC
pub(super) fn wall_at(
    secs: u64,
) -> (
    revue::testing::MockTime,
    revue::utils::clock::MockClockGuard,
) {
    let time = revue::testing::MockTime::new();
    time.set_wall_clock(
        std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs),
        0,
    );
    let guard = time.install();
    (time, guard)
}

/// A fixed hour, so charts without data do not read the clock
pub(super) fn fixed_range() -> TimeRange {
    TimeRange::Range {
        start: 0,
        end: 3600,
    }
}

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_timeseries_basic() {
        use revue::style::Color;
        use revue::widget::{TimeSeries, TimeSeriesData};

        let data = TimeSeriesData::new("CPU")
            .point(0, 25.0)
            .point(1, 45.0)
            .point(2, 30.0)
            .point(3, 60.0)
            .point(4, 55.0)
            .color(Color::CYAN);

        let view = TimeSeries::new()
            .title("CPU Usage")
            .series(data)
            .show_legend(true);

        let config = TestConfig::with_size(60, 15);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timeseries_basic");
    }

    #[test]
    fn test_timeseries_multiple_series() {
        use revue::style::Color;
        use revue::widget::{TimeSeries, TimeSeriesData};

        let cpu = TimeSeriesData::new("CPU")
            .points(vec![(0, 20.0), (1, 40.0), (2, 35.0), (3, 50.0)])
            .color(Color::CYAN);

        let memory = TimeSeriesData::new("Memory")
            .points(vec![(0, 60.0), (1, 65.0), (2, 70.0), (3, 68.0)])
            .color(Color::MAGENTA);

        let view = TimeSeries::new()
            .title("System Metrics")
            .series(cpu)
            .series(memory)
            .y_label("Usage %")
            .show_grid(true);

        let config = TestConfig::with_size(70, 18);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timeseries_multi");
    }
}
