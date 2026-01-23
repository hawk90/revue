//! DateTime picker navigation

use super::super::calendar::{days_in_month, Date};
use super::types::{TimeField, TimeField::Hour};

/// Navigation trait for DateTimePicker
pub trait Navigation {
    /// Get date
    fn date(&self) -> Date;
    /// Set date
    fn set_date(&mut self, date: Date);
    /// Get time
    fn time(&self) -> crate::widget::datetime_picker::types::Time;
    /// Set time
    fn set_time(&mut self, time: crate::widget::datetime_picker::types::Time);
    /// Get cursor day
    fn cursor_day(&self) -> u32;
    /// Set cursor day
    fn set_cursor_day(&mut self, day: u32);
    /// Get time field
    fn time_field(&self) -> TimeField;
    /// Set time field
    fn set_time_field(&mut self, field: TimeField);
    /// Get show_seconds
    fn show_seconds(&self) -> bool;
    /// Check if date is valid
    fn is_date_valid(&self, date: &Date) -> bool;

    /// Move cursor to previous day
    fn move_day_left(&mut self) {
        if self.cursor_day() > 1 {
            self.set_cursor_day(self.cursor_day() - 1);
        } else {
            self.prev_month();
            let max_day = days_in_month(self.date().year, self.date().month);
            self.set_cursor_day(max_day);
        }
    }

    /// Move cursor to next day
    fn move_day_right(&mut self) {
        let max_day = days_in_month(self.date().year, self.date().month);
        if self.cursor_day() < max_day {
            self.set_cursor_day(self.cursor_day() + 1);
        } else {
            self.next_month();
            self.set_cursor_day(1);
        }
    }

    /// Move cursor up one week
    fn move_week_up(&mut self) {
        if self.cursor_day() > 7 {
            self.set_cursor_day(self.cursor_day() - 7);
        } else {
            self.prev_month();
            let max_day = days_in_month(self.date().year, self.date().month);
            self.set_cursor_day(max_day - (7 - self.cursor_day()));
        }
    }

    /// Move cursor down one week
    fn move_week_down(&mut self) {
        let max_day = days_in_month(self.date().year, self.date().month);
        if self.cursor_day() + 7 <= max_day {
            self.set_cursor_day(self.cursor_day() + 7);
        } else {
            let overflow = self.cursor_day() + 7 - max_day;
            self.next_month();
            self.set_cursor_day(overflow);
        }
    }

    /// Go to previous month
    fn prev_month(&mut self) {
        let mut date = self.date();
        if date.month > 1 {
            date.month -= 1;
        } else {
            date.month = 12;
            date.year -= 1;
        }
        // Clamp cursor day to valid range
        let max_day = days_in_month(date.year, date.month);
        self.set_cursor_day(self.cursor_day().min(max_day));
        self.set_date(date);
    }

    /// Go to next month
    fn next_month(&mut self) {
        let mut date = self.date();
        if date.month < 12 {
            date.month += 1;
        } else {
            date.month = 1;
            date.year += 1;
        }
        // Clamp cursor day to valid range
        let max_day = days_in_month(date.year, date.month);
        self.set_cursor_day(self.cursor_day().min(max_day));
        self.set_date(date);
    }

    /// Go to previous year
    fn prev_year(&mut self) {
        let mut date = self.date();
        date.year -= 1;
        // Handle Feb 29 in non-leap years
        let max_day = days_in_month(date.year, date.month);
        self.set_cursor_day(self.cursor_day().min(max_day));
        self.set_date(date);
    }

    /// Go to next year
    fn next_year(&mut self) {
        let mut date = self.date();
        date.year += 1;
        // Handle Feb 29 in non-leap years
        let max_day = days_in_month(date.year, date.month);
        self.set_cursor_day(self.cursor_day().min(max_day));
        self.set_date(date);
    }

    /// Select current cursor position as the date
    fn select_date(&mut self) {
        let new_date = Date::new(self.date().year, self.date().month, self.cursor_day());
        if self.is_date_valid(&new_date) {
            let mut date = self.date();
            date.day = self.cursor_day();
            self.set_date(date);
        }
    }

    /// Increment current time field
    fn increment_time(&mut self) {
        let mut time = self.time();
        match self.time_field() {
            TimeField::Hour => {
                time.hour = (time.hour + 1) % 24;
            }
            TimeField::Minute => {
                time.minute = (time.minute + 1) % 60;
            }
            TimeField::Second => {
                time.second = (time.second + 1) % 60;
            }
        }
        self.set_time(time);
    }

    /// Decrement current time field
    fn decrement_time(&mut self) {
        let mut time = self.time();
        match self.time_field() {
            TimeField::Hour => {
                time.hour = if time.hour == 0 { 23 } else { time.hour - 1 };
            }
            TimeField::Minute => {
                time.minute = if time.minute == 0 {
                    59
                } else {
                    time.minute - 1
                };
            }
            TimeField::Second => {
                time.second = if time.second == 0 {
                    59
                } else {
                    time.second - 1
                };
            }
        }
        self.set_time(time);
    }

    /// Move to next time field
    fn next_time_field(&mut self) {
        self.set_time_field(match self.time_field() {
            TimeField::Hour => TimeField::Minute,
            TimeField::Minute => {
                if self.show_seconds() {
                    TimeField::Second
                } else {
                    Hour
                }
            }
            TimeField::Second => Hour,
        });
    }

    /// Move to previous time field
    fn prev_time_field(&mut self) {
        self.set_time_field(match self.time_field() {
            TimeField::Hour => {
                if self.show_seconds() {
                    TimeField::Second
                } else {
                    TimeField::Minute
                }
            }
            TimeField::Minute => Hour,
            TimeField::Second => TimeField::Minute,
        });
    }
}
