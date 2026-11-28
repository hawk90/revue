//! RangePicker widget tests
//!
//! The in-source tests in src/widget/range_picker/ cover the builders,
//! getters, setters, navigation and key handling.

mod core;
mod integration;
mod types;

use revue::testing::MockTime;
use revue::utils::clock::MockClockGuard;
use std::time::{Duration, UNIX_EPOCH};

/// A mock clock on which today is 2025-06-15 (a Sunday) at noon, in UTC
pub(super) fn on_june_15() -> (MockTime, MockClockGuard) {
    let time = MockTime::new();
    time.set_wall_clock(UNIX_EPOCH + Duration::from_secs(1_749_988_800), 0);
    let guard = time.install();
    (time, guard)
}
