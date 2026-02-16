//! Date/time range picker types

use crate::widget::data::calendar::{days_in_month, Date};

/// Preset date ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetRange {
    /// Today only
    Today,
    /// Yesterday only
    Yesterday,
    /// Last 7 days including today
    Last7Days,
    /// Last 30 days including today
    Last30Days,
    /// Current week (Sunday/Monday to today)
    ThisWeek,
    /// Previous full week
    LastWeek,
    /// Current month
    ThisMonth,
    /// Previous full month
    LastMonth,
    /// This year
    ThisYear,
    /// Custom range (user selected)
    Custom,
}

impl PresetRange {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            PresetRange::Today => "Today",
            PresetRange::Yesterday => "Yesterday",
            PresetRange::Last7Days => "Last 7 Days",
            PresetRange::Last30Days => "Last 30 Days",
            PresetRange::ThisWeek => "This Week",
            PresetRange::LastWeek => "Last Week",
            PresetRange::ThisMonth => "This Month",
            PresetRange::LastMonth => "Last Month",
            PresetRange::ThisYear => "This Year",
            PresetRange::Custom => "Custom",
        }
    }

    /// Get all common presets
    pub fn common() -> &'static [PresetRange] {
        &[
            PresetRange::Today,
            PresetRange::Yesterday,
            PresetRange::Last7Days,
            PresetRange::Last30Days,
            PresetRange::ThisWeek,
            PresetRange::LastWeek,
            PresetRange::ThisMonth,
            PresetRange::LastMonth,
        ]
    }

    /// Calculate the date range for this preset
    pub fn calculate(&self, today: Date) -> (Date, Date) {
        match self {
            PresetRange::Today => (today, today),
            PresetRange::Yesterday => {
                let yesterday = today.prev_day();
                (yesterday, yesterday)
            }
            PresetRange::Last7Days => {
                let start = today.subtract_days(6);
                (start, today)
            }
            PresetRange::Last30Days => {
                let start = today.subtract_days(29);
                (start, today)
            }
            PresetRange::ThisWeek => {
                let weekday = today.weekday(); // 0 = Sunday
                let start = today.subtract_days(weekday);
                (start, today)
            }
            PresetRange::LastWeek => {
                let weekday = today.weekday();
                let this_week_start = today.subtract_days(weekday);
                let last_week_end = this_week_start.prev_day();
                let last_week_start = last_week_end.subtract_days(6);
                (last_week_start, last_week_end)
            }
            PresetRange::ThisMonth => {
                let start = Date::new(today.year, today.month, 1);
                (start, today)
            }
            PresetRange::LastMonth => {
                let (year, month) = if today.month == 1 {
                    (today.year - 1, 12)
                } else {
                    (today.year, today.month - 1)
                };
                let start = Date::new(year, month, 1);
                let end = Date::new(year, month, days_in_month(year, month));
                (start, end)
            }
            PresetRange::ThisYear => {
                let start = Date::new(today.year, 1, 1);
                (start, today)
            }
            PresetRange::Custom => (today, today),
        }
    }
}

/// Which part of the range picker has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RangeFocus {
    /// Start date calendar
    #[default]
    Start,
    /// End date calendar
    End,
    /// Preset list
    Presets,
}

// KEEP HERE: Private tests - all public API tests moved to tests/widget/range_picker/types.rs
