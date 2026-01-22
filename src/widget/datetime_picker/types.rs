//! DateTime picker types

use super::super::calendar::Date;

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
