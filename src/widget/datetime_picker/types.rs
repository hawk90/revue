//! DateTime picker types

use super::super::data::calendar::Date;

/// Time of day
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
}

impl Time {
    /// Create a new time
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
            second: second.min(59),
        }
    }

    /// Create time from hours and minutes
    pub fn hm(hour: u8, minute: u8) -> Self {
        Self::new(hour, minute, 0)
    }

    /// Get current time (placeholder - returns 12:00:00)
    pub fn now() -> Self {
        // In a real implementation, use std::time or chrono
        Self::new(12, 0, 0)
    }

    /// Check if time is valid
    pub fn is_valid(&self) -> bool {
        self.hour < 24 && self.minute < 60 && self.second < 60
    }

    /// Format as HH:MM
    pub fn format_hm(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Format as HH:MM:SS
    pub fn format_hms(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    /// Format as 12-hour with AM/PM
    pub fn format_12h(&self) -> String {
        let (hour, ampm) = if self.hour == 0 {
            (12, "AM")
        } else if self.hour < 12 {
            (self.hour, "AM")
        } else if self.hour == 12 {
            (12, "PM")
        } else {
            (self.hour - 12, "PM")
        };
        format!("{:2}:{:02} {}", hour, self.minute, ampm)
    }
}

/// Combined date and time
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateTime {
    /// Date component
    pub date: Date,
    /// Time component
    pub time: Time,
}

impl DateTime {
    /// Create a new datetime
    pub fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Create from components
    pub fn from_parts(year: i32, month: u32, day: u32, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            date: Date::new(year, month, day),
            time: Time::new(hour, minute, second),
        }
    }
}

/// DateTime display format
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateTimeFormat {
    /// Date only (YYYY-MM-DD)
    DateOnly,
    /// Time only (HH:MM)
    TimeOnly,
    /// Time with seconds (HH:MM:SS)
    TimeWithSeconds,
    /// Date and time (YYYY-MM-DD HH:MM)
    #[default]
    DateTime,
    /// Date and time 24h (YYYY-MM-DD HH:MM:SS)
    DateTime24,
    /// Date and time 12h (YYYY-MM-DD hh:mm AM/PM)
    DateTime12,
}

/// DateTime picker mode (which component is being edited)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateTimeMode {
    /// Selecting date in calendar
    #[default]
    Date,
    /// Selecting time
    Time,
}

/// Time field being edited
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimeField {
    /// Hour field (0-23)
    #[default]
    Hour,
    /// Minute field (0-59)
    Minute,
    /// Second field (0-59)
    Second,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Time tests
    // =========================================================================

    #[test]
    fn test_time_new() {
        let time = Time::new(10, 30, 45);
        assert_eq!(time.hour, 10);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 45);
    }

    #[test]
    fn test_time_new_clamps_hour() {
        let time = Time::new(25, 30, 45);
        assert_eq!(time.hour, 23); // Clamped to 23
    }

    #[test]
    fn test_time_new_clamps_minute() {
        let time = Time::new(10, 70, 45);
        assert_eq!(time.minute, 59); // Clamped to 59
    }

    #[test]
    fn test_time_new_clamps_second() {
        let time = Time::new(10, 30, 70);
        assert_eq!(time.second, 59); // Clamped to 59
    }

    #[test]
    fn test_time_hm() {
        let time = Time::hm(10, 30);
        assert_eq!(time.hour, 10);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_time_now() {
        let time = Time::now();
        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_time_is_valid_true() {
        let time = Time::new(10, 30, 45);
        assert!(time.is_valid());
    }

    #[test]
    fn test_time_is_valid_boundary() {
        assert!(Time::new(23, 59, 59).is_valid());
    }

    #[test]
    fn test_time_is_valid_false_hour() {
        let time = Time {
            hour: 24,
            minute: 0,
            second: 0,
        };
        assert!(!time.is_valid());
    }

    #[test]
    fn test_time_is_valid_false_minute() {
        let time = Time {
            hour: 0,
            minute: 60,
            second: 0,
        };
        assert!(!time.is_valid());
    }

    #[test]
    fn test_time_is_valid_false_second() {
        let time = Time {
            hour: 0,
            minute: 0,
            second: 60,
        };
        assert!(!time.is_valid());
    }

    #[test]
    fn test_time_format_hm() {
        let time = Time::new(10, 30, 45);
        assert_eq!(time.format_hm(), "10:30");
    }

    #[test]
    fn test_time_format_hm_single_digit() {
        let time = Time::new(5, 5, 5);
        assert_eq!(time.format_hm(), "05:05");
    }

    #[test]
    fn test_time_format_hms() {
        let time = Time::new(10, 30, 45);
        assert_eq!(time.format_hms(), "10:30:45");
    }

    #[test]
    fn test_time_format_hms_single_digit() {
        let time = Time::new(5, 5, 5);
        assert_eq!(time.format_hms(), "05:05:05");
    }

    #[test]
    fn test_time_format_12h_am() {
        let time = Time::new(10, 30, 0);
        assert_eq!(time.format_12h(), "10:30 AM");
    }

    #[test]
    fn test_time_format_12h_pm() {
        let time = Time::new(14, 30, 0);
        assert_eq!(time.format_12h(), " 2:30 PM");
    }

    #[test]
    fn test_time_format_12h_midnight() {
        let time = Time::new(0, 30, 0);
        assert_eq!(time.format_12h(), "12:30 AM");
    }

    #[test]
    fn test_time_format_12h_noon() {
        let time = Time::new(12, 30, 0);
        assert_eq!(time.format_12h(), "12:30 PM");
    }

    #[test]
    fn test_time_format_12h_single_digit_minute() {
        let time = Time::new(10, 5, 0);
        assert_eq!(time.format_12h(), "10:05 AM");
    }

    #[test]
    fn test_time_default() {
        let time = Time::default();
        assert_eq!(time.hour, 0);
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_time_partial_eq() {
        let time1 = Time::new(10, 30, 45);
        let time2 = Time::new(10, 30, 45);
        assert_eq!(time1, time2);
    }

    #[test]
    fn test_time_partial_ord() {
        let time1 = Time::new(10, 30, 45);
        let time2 = Time::new(11, 30, 45);
        assert!(time1 < time2);
    }

    #[test]
    fn test_time_copy() {
        let time = Time::new(10, 30, 45);
        let copied = time;
        assert_eq!(time, copied);
    }

    #[test]
    fn test_time_clone() {
        let time = Time::new(10, 30, 45);
        let cloned = time.clone();
        assert_eq!(time, cloned);
    }

    #[test]
    fn test_time_debug() {
        let time = Time::new(10, 30, 45);
        let debug_str = format!("{:?}", time);
        assert!(debug_str.contains("Time"));
    }

    // =========================================================================
    // DateTime tests
    // =========================================================================

    #[test]
    fn test_datetime_new() {
        let date = Date::new(2024, 1, 15);
        let time = Time::new(10, 30, 45);
        let dt = DateTime::new(date, time);
        assert_eq!(dt.date, date);
        assert_eq!(dt.time, time);
    }

    #[test]
    fn test_datetime_from_parts() {
        let dt = DateTime::from_parts(2024, 1, 15, 10, 30, 45);
        assert_eq!(dt.date.year, 2024);
        assert_eq!(dt.date.month, 1);
        assert_eq!(dt.date.day, 15);
        assert_eq!(dt.time.hour, 10);
        assert_eq!(dt.time.minute, 30);
        assert_eq!(dt.time.second, 45);
    }

    #[test]
    fn test_datetime_default() {
        let dt = DateTime::default();
        // Both date and time have Default
        assert_eq!(dt.time, Time::default());
    }

    #[test]
    fn test_datetime_clone() {
        let date = Date::new(2024, 1, 15);
        let time = Time::new(10, 30, 45);
        let dt = DateTime::new(date, time);
        let cloned = dt.clone();
        assert_eq!(dt, cloned);
    }

    #[test]
    fn test_datetime_copy() {
        let date = Date::new(2024, 1, 15);
        let time = Time::new(10, 30, 45);
        let dt = DateTime::new(date, time);
        let copied = dt;
        assert_eq!(dt, copied);
    }

    #[test]
    fn test_datetime_partial_eq() {
        let date = Date::new(2024, 1, 15);
        let time = Time::new(10, 30, 45);
        let dt1 = DateTime::new(date, time);
        let dt2 = DateTime::new(date, time);
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_datetime_debug() {
        let date = Date::new(2024, 1, 15);
        let time = Time::new(10, 30, 45);
        let dt = DateTime::new(date, time);
        let debug_str = format!("{:?}", dt);
        assert!(debug_str.contains("DateTime"));
    }

    // =========================================================================
    // DateTimeFormat tests
    // =========================================================================

    #[test]
    fn test_datetime_format_default() {
        assert_eq!(DateTimeFormat::default(), DateTimeFormat::DateTime);
    }

    #[test]
    fn test_datetime_format_partial_eq() {
        assert_eq!(DateTimeFormat::DateOnly, DateTimeFormat::DateOnly);
        assert_eq!(DateTimeFormat::TimeOnly, DateTimeFormat::TimeOnly);
        assert_eq!(DateTimeFormat::DateTime, DateTimeFormat::DateTime);
        assert_eq!(DateTimeFormat::DateTime24, DateTimeFormat::DateTime24);
        assert_eq!(DateTimeFormat::DateTime12, DateTimeFormat::DateTime12);
    }

    #[test]
    fn test_datetime_format_ne() {
        assert_ne!(DateTimeFormat::DateOnly, DateTimeFormat::TimeOnly);
        assert_ne!(DateTimeFormat::DateTime, DateTimeFormat::DateTime24);
    }

    #[test]
    fn test_datetime_format_copy() {
        let format = DateTimeFormat::DateTime12;
        let copied = format;
        assert_eq!(format, copied);
    }

    #[test]
    fn test_datetime_format_clone() {
        let format = DateTimeFormat::DateTime24;
        let cloned = format.clone();
        assert_eq!(format, cloned);
    }

    #[test]
    fn test_datetime_format_debug() {
        let format = DateTimeFormat::DateTime;
        let debug_str = format!("{:?}", format);
        assert!(debug_str.contains("DateTime"));
    }

    // =========================================================================
    // DateTimeMode tests
    // =========================================================================

    #[test]
    fn test_datetime_mode_default() {
        assert_eq!(DateTimeMode::default(), DateTimeMode::Date);
    }

    #[test]
    fn test_datetime_mode_partial_eq() {
        assert_eq!(DateTimeMode::Date, DateTimeMode::Date);
        assert_eq!(DateTimeMode::Time, DateTimeMode::Time);
    }

    #[test]
    fn test_datetime_mode_ne() {
        assert_ne!(DateTimeMode::Date, DateTimeMode::Time);
    }

    #[test]
    fn test_datetime_mode_copy() {
        let mode = DateTimeMode::Time;
        let copied = mode;
        assert_eq!(mode, copied);
    }

    #[test]
    fn test_datetime_mode_clone() {
        let mode = DateTimeMode::Date;
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_datetime_mode_debug() {
        let mode = DateTimeMode::Date;
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("Date"));
    }

    // =========================================================================
    // TimeField tests
    // =========================================================================

    #[test]
    fn test_time_field_default() {
        assert_eq!(TimeField::default(), TimeField::Hour);
    }

    #[test]
    fn test_time_field_partial_eq() {
        assert_eq!(TimeField::Hour, TimeField::Hour);
        assert_eq!(TimeField::Minute, TimeField::Minute);
        assert_eq!(TimeField::Second, TimeField::Second);
    }

    #[test]
    fn test_time_field_ne() {
        assert_ne!(TimeField::Hour, TimeField::Minute);
        assert_ne!(TimeField::Minute, TimeField::Second);
        assert_ne!(TimeField::Hour, TimeField::Second);
    }

    #[test]
    fn test_time_field_copy() {
        let field = TimeField::Minute;
        let copied = field;
        assert_eq!(field, copied);
    }

    #[test]
    fn test_time_field_clone() {
        let field = TimeField::Second;
        let cloned = field.clone();
        assert_eq!(field, cloned);
    }

    #[test]
    fn test_time_field_debug() {
        let field = TimeField::Hour;
        let debug_str = format!("{:?}", field);
        assert!(debug_str.contains("Hour"));
    }

    #[test]
    fn test_time_field_all_variants_unique() {
        assert_ne!(TimeField::Hour, TimeField::Minute);
        assert_ne!(TimeField::Hour, TimeField::Second);
        assert_ne!(TimeField::Minute, TimeField::Second);
    }
}
