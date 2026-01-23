//! Range picker builder methods, getters, setters, and helpers

use super::core::RangePicker;
use super::types::{PresetRange, RangeFocus};
use crate::style::Color;
use crate::widget::calendar::{Date, FirstDayOfWeek};
use crate::widget::datetime_picker::{DateTime, Time};

impl RangePicker {
    // =========================================================================
    // Builder Methods
    // =========================================================================

    /// Set start date
    pub fn start_date(mut self, date: Date) -> Self {
        self.start.date = date;
        self.start_cursor_day = date.day;
        self.active_preset = Some(PresetRange::Custom);
        self
    }

    /// Set end date
    pub fn end_date(mut self, date: Date) -> Self {
        self.end.date = date;
        self.end_cursor_day = date.day;
        self.active_preset = Some(PresetRange::Custom);
        self.swap_if_needed();
        self
    }

    /// Set start time
    pub fn start_time(mut self, time: Time) -> Self {
        self.start.time = time;
        self
    }

    /// Set end time
    pub fn end_time(mut self, time: Time) -> Self {
        self.end.time = time;
        self
    }

    /// Set date range
    pub fn range(mut self, start: Date, end: Date) -> Self {
        self.start.date = start;
        self.end.date = end;
        self.start_cursor_day = start.day;
        self.end_cursor_day = end.day;
        self.active_preset = Some(PresetRange::Custom);
        self.swap_if_needed();
        self
    }

    /// Set first day of week
    pub fn first_day(mut self, first: FirstDayOfWeek) -> Self {
        self.first_day = first;
        self
    }

    /// Show or hide time selection
    pub fn show_time(mut self, show: bool) -> Self {
        self.show_time = show;
        self
    }

    /// Show or hide presets panel
    pub fn with_presets(mut self, show: bool) -> Self {
        self.show_presets = show;
        self
    }

    /// Set available presets
    pub fn presets(mut self, presets: Vec<PresetRange>) -> Self {
        self.presets = presets;
        self
    }

    /// Set minimum date constraint
    pub fn min_date(mut self, date: Date) -> Self {
        self.min_date = Some(date);
        self
    }

    /// Set maximum date constraint
    pub fn max_date(mut self, date: Date) -> Self {
        self.max_date = Some(date);
        self
    }

    /// Set range colors
    pub fn range_color(mut self, color: Color) -> Self {
        self.range_bg = color;
        self
    }

    // =========================================================================
    // Getters
    // =========================================================================

    /// Get the selected date range
    pub fn get_range(&self) -> (Date, Date) {
        (self.start.date, self.end.date)
    }

    /// Get the selected datetime range
    pub fn get_datetime_range(&self) -> (DateTime, DateTime) {
        (self.start, self.end)
    }

    /// Get start date
    pub fn get_start(&self) -> Date {
        self.start.date
    }

    /// Get end date
    pub fn get_end(&self) -> Date {
        self.end.date
    }

    /// Get active preset (if any)
    pub fn get_active_preset(&self) -> Option<PresetRange> {
        self.active_preset
    }

    /// Check if a date is within the selected range
    pub fn is_in_range(&self, date: &Date) -> bool {
        date >= &self.start.date && date <= &self.end.date
    }

    /// Get the current focus area
    pub fn get_focus(&self) -> RangeFocus {
        self.focus
    }

    // =========================================================================
    // Setters
    // =========================================================================

    /// Set the start date
    pub fn set_start(&mut self, date: Date) {
        self.start.date = date;
        self.start_cursor_day = date.day;
        self.active_preset = Some(PresetRange::Custom);
        self.swap_if_needed();
    }

    /// Set the end date
    pub fn set_end(&mut self, date: Date) {
        self.end.date = date;
        self.end_cursor_day = date.day;
        self.active_preset = Some(PresetRange::Custom);
        self.swap_if_needed();
    }

    /// Apply a preset range
    pub fn apply_preset(&mut self, preset: PresetRange) {
        let today = Date::today();
        let (start, end) = preset.calculate(today);
        self.start.date = start;
        self.end.date = end;
        self.start_cursor_day = start.day;
        self.end_cursor_day = end.day;
        self.active_preset = Some(preset);
    }

    /// Ensure end >= start, swap if needed
    pub(crate) fn swap_if_needed(&mut self) {
        if self.end.date < self.start.date {
            std::mem::swap(&mut self.start.date, &mut self.end.date);
            std::mem::swap(&mut self.start_cursor_day, &mut self.end_cursor_day);
        }
    }
}

// =============================================================================
// Helper functions
// =============================================================================

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
