//! Helper functions for the range picker widget

use super::types::PresetRange;
use super::RangePicker;

/// Create a basic range picker
pub fn range_picker() -> RangePicker {
    RangePicker::new()
}

/// Create a date-only range picker (no time)
pub fn date_range_picker() -> RangePicker {
    RangePicker::new().show_time(false)
}

/// Create an analytics-style range picker with common presets
pub fn analytics_range_picker() -> RangePicker {
    RangePicker::new().with_presets(true).presets(vec![
        PresetRange::Today,
        PresetRange::Yesterday,
        PresetRange::Last7Days,
        PresetRange::Last30Days,
        PresetRange::ThisMonth,
        PresetRange::LastMonth,
        PresetRange::ThisYear,
    ])
}

/// Helper function to get month name
pub(crate) fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

// Tests moved to tests/widget/range_picker/helpers.rs
// Tests here access private fields and should remain in source

#[cfg(test)]
mod tests {
    // Tests that access private fields should remain here
    // KEEP HERE - accesses private fields
}
