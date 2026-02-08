//! DateTime picker navigation

use super::super::data::calendar::{days_in_month, Date};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::datetime_picker::types::Time;

    // Mock struct for testing Navigation trait
    struct MockDateTimePicker {
        date: Date,
        time: Time,
        cursor_day: u32,
        time_field: TimeField,
        show_seconds: bool,
        valid_date: bool,
    }

    impl MockDateTimePicker {
        fn new() -> Self {
            Self {
                date: Date::new(2024, 1, 15),
                time: Time::new(10, 30, 45),
                cursor_day: 15,
                time_field: TimeField::Hour,
                show_seconds: true,
                valid_date: true,
            }
        }

        fn with_date(year: i32, month: u32, day: u32) -> Self {
            let mut picker = Self::new();
            picker.date = Date::new(year, month, day);
            picker.cursor_day = day;
            picker
        }

        fn with_cursor_day(day: u32) -> Self {
            let mut picker = Self::new();
            picker.cursor_day = day;
            picker
        }

        fn with_time_field(field: TimeField) -> Self {
            let mut picker = Self::new();
            picker.time_field = field;
            picker
        }

        fn with_time(hour: u8, minute: u8, second: u8) -> Self {
            let mut picker = Self::new();
            picker.time = Time::new(hour, minute, second);
            picker
        }

        fn without_seconds() -> Self {
            let mut picker = Self::new();
            picker.show_seconds = false;
            picker
        }

        fn with_valid_date(valid: bool) -> Self {
            let mut picker = Self::new();
            picker.valid_date = valid;
            picker
        }
    }

    impl Navigation for MockDateTimePicker {
        fn date(&self) -> Date {
            self.date
        }

        fn set_date(&mut self, date: Date) {
            self.date = date;
        }

        fn time(&self) -> Time {
            self.time
        }

        fn set_time(&mut self, time: Time) {
            self.time = time;
        }

        fn cursor_day(&self) -> u32 {
            self.cursor_day
        }

        fn set_cursor_day(&mut self, day: u32) {
            self.cursor_day = day;
        }

        fn time_field(&self) -> TimeField {
            self.time_field
        }

        fn set_time_field(&mut self, field: TimeField) {
            self.time_field = field;
        }

        fn show_seconds(&self) -> bool {
            self.show_seconds
        }

        fn is_date_valid(&self, _date: &Date) -> bool {
            self.valid_date
        }
    }

    // =========================================================================
    // Day movement tests
    // =========================================================================

    #[test]
    fn test_move_day_left() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.move_day_left();
        assert_eq!(picker.cursor_day(), 14);
        assert_eq!(picker.date().month, 1);
    }

    #[test]
    fn test_move_day_left_wrap_to_previous_month() {
        let mut picker = MockDateTimePicker::with_cursor_day(1);
        picker.move_day_left();
        // Should go to last day of December 2023 (31 days)
        assert_eq!(picker.cursor_day(), 31);
        assert_eq!(picker.date().month, 12);
        assert_eq!(picker.date().year, 2023);
    }

    #[test]
    fn test_move_day_right() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.move_day_right();
        assert_eq!(picker.cursor_day(), 16);
        assert_eq!(picker.date().month, 1);
    }

    #[test]
    fn test_move_day_right_wrap_to_next_month() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 31);
        picker.move_day_right();
        assert_eq!(picker.cursor_day(), 1);
        assert_eq!(picker.date().month, 2);
    }

    #[test]
    fn test_move_week_up() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.move_week_up();
        assert_eq!(picker.cursor_day(), 8);
    }

    #[test]
    fn test_move_week_up_wrap_to_previous_month() {
        let mut picker = MockDateTimePicker::with_cursor_day(5);
        picker.move_week_up();
        // 5 - 7 = -2, so should wrap to previous month
        // January has 31 days, 31 - (7-5) = 29
        assert_eq!(picker.cursor_day(), 29);
        assert_eq!(picker.date().month, 12);
        assert_eq!(picker.date().year, 2023);
    }

    #[test]
    fn test_move_week_down() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.move_week_down();
        assert_eq!(picker.cursor_day(), 22);
    }

    #[test]
    fn test_move_week_down_wrap_to_next_month() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 28);
        picker.move_week_down();
        // 28 + 7 = 35, Jan has 31 days, overflow = 4
        assert_eq!(picker.cursor_day(), 4);
        assert_eq!(picker.date().month, 2);
    }

    // =========================================================================
    // Month navigation tests
    // =========================================================================

    #[test]
    fn test_prev_month() {
        let mut picker = MockDateTimePicker::with_date(2024, 3, 15);
        picker.prev_month();
        assert_eq!(picker.date().month, 2);
        assert_eq!(picker.date().year, 2024);
        assert_eq!(picker.cursor_day(), 15);
    }

    #[test]
    fn test_prev_month_wrap_year() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.prev_month();
        assert_eq!(picker.date().month, 12);
        assert_eq!(picker.date().year, 2023);
    }

    #[test]
    fn test_prev_month_clamps_cursor_day() {
        let mut picker = MockDateTimePicker::with_date(2024, 3, 31);
        picker.prev_month();
        // February 2024 has 29 days (leap year)
        assert_eq!(picker.date().month, 2);
        assert_eq!(picker.cursor_day(), 29);
    }

    #[test]
    fn test_next_month() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.next_month();
        assert_eq!(picker.date().month, 2);
        assert_eq!(picker.date().year, 2024);
    }

    #[test]
    fn test_next_month_wrap_year() {
        let mut picker = MockDateTimePicker::with_date(2024, 12, 15);
        picker.next_month();
        assert_eq!(picker.date().month, 1);
        assert_eq!(picker.date().year, 2025);
    }

    #[test]
    fn test_next_month_clamps_cursor_day() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 31);
        picker.next_month();
        // February 2024 has 29 days
        assert_eq!(picker.date().month, 2);
        assert_eq!(picker.cursor_day(), 29);
    }

    // =========================================================================
    // Year navigation tests
    // =========================================================================

    #[test]
    fn test_prev_year() {
        let mut picker = MockDateTimePicker::with_date(2024, 6, 15);
        picker.prev_year();
        assert_eq!(picker.date().year, 2023);
        assert_eq!(picker.date().month, 6);
    }

    #[test]
    fn test_prev_year_handles_feb_29() {
        let mut picker = MockDateTimePicker::with_date(2024, 2, 29);
        picker.prev_year();
        // 2023 is not a leap year, Feb has 28 days
        assert_eq!(picker.date().year, 2023);
        assert_eq!(picker.cursor_day(), 28);
    }

    #[test]
    fn test_next_year() {
        let mut picker = MockDateTimePicker::with_date(2024, 6, 15);
        picker.next_year();
        assert_eq!(picker.date().year, 2025);
        assert_eq!(picker.date().month, 6);
    }

    #[test]
    fn test_next_year_handles_feb_29() {
        let mut picker = MockDateTimePicker::with_date(2024, 2, 29);
        picker.next_year();
        // 2025 is not a leap year, Feb has 28 days
        assert_eq!(picker.date().year, 2025);
        assert_eq!(picker.cursor_day(), 28);
    }

    // =========================================================================
    // Date selection tests
    // =========================================================================

    #[test]
    fn test_select_date() {
        let mut picker = MockDateTimePicker::with_date(2024, 1, 15);
        picker.set_cursor_day(20);
        picker.select_date();
        assert_eq!(picker.date().day, 20);
    }

    #[test]
    fn test_select_date_invalid() {
        let mut picker = MockDateTimePicker::with_valid_date(false);
        picker.set_cursor_day(20);
        picker.select_date();
        // Should not change date if invalid
        assert_eq!(picker.date().day, 15);
    }

    // =========================================================================
    // Time increment/decrement tests
    // =========================================================================

    #[test]
    fn test_increment_time_hour() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Hour);
        picker.increment_time();
        assert_eq!(picker.time().hour, 11);
    }

    #[test]
    fn test_increment_time_hour_wrap() {
        let mut picker = MockDateTimePicker::with_time(23, 30, 45);
        picker.set_time_field(TimeField::Hour);
        picker.increment_time();
        assert_eq!(picker.time().hour, 0);
    }

    #[test]
    fn test_increment_time_minute() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Minute);
        picker.increment_time();
        assert_eq!(picker.time().minute, 31);
    }

    #[test]
    fn test_increment_time_minute_wrap() {
        let mut picker = MockDateTimePicker::with_time(10, 59, 45);
        picker.set_time_field(TimeField::Minute);
        picker.increment_time();
        assert_eq!(picker.time().minute, 0);
    }

    #[test]
    fn test_increment_time_second() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Second);
        picker.increment_time();
        assert_eq!(picker.time().second, 46);
    }

    #[test]
    fn test_increment_time_second_wrap() {
        let mut picker = MockDateTimePicker::with_time(10, 30, 59);
        picker.set_time_field(TimeField::Second);
        picker.increment_time();
        assert_eq!(picker.time().second, 0);
    }

    #[test]
    fn test_decrement_time_hour() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Hour);
        picker.decrement_time();
        assert_eq!(picker.time().hour, 9);
    }

    #[test]
    fn test_decrement_time_hour_wrap() {
        let mut picker = MockDateTimePicker::with_time(0, 30, 45);
        picker.set_time_field(TimeField::Hour);
        picker.decrement_time();
        assert_eq!(picker.time().hour, 23);
    }

    #[test]
    fn test_decrement_time_minute() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Minute);
        picker.decrement_time();
        assert_eq!(picker.time().minute, 29);
    }

    #[test]
    fn test_decrement_time_minute_wrap() {
        let mut picker = MockDateTimePicker::with_time(10, 0, 45);
        picker.set_time_field(TimeField::Minute);
        picker.decrement_time();
        assert_eq!(picker.time().minute, 59);
    }

    #[test]
    fn test_decrement_time_second() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Second);
        picker.decrement_time();
        assert_eq!(picker.time().second, 44);
    }

    #[test]
    fn test_decrement_time_second_wrap() {
        let mut picker = MockDateTimePicker::with_time(10, 30, 0);
        picker.set_time_field(TimeField::Second);
        picker.decrement_time();
        assert_eq!(picker.time().second, 59);
    }

    // =========================================================================
    // Time field navigation tests
    // =========================================================================

    #[test]
    fn test_next_time_field_hour_to_minute() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Hour);
        picker.next_time_field();
        assert_eq!(picker.time_field(), TimeField::Minute);
    }

    #[test]
    fn test_next_time_field_minute_to_second() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Minute);
        picker.next_time_field();
        assert_eq!(picker.time_field(), TimeField::Second);
    }

    #[test]
    fn test_next_time_field_second_to_hour() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Second);
        picker.next_time_field();
        assert_eq!(picker.time_field(), TimeField::Hour);
    }

    #[test]
    fn test_next_time_field_minute_to_hour_no_seconds() {
        let mut picker = MockDateTimePicker::without_seconds();
        picker.set_time_field(TimeField::Minute);
        picker.next_time_field();
        assert_eq!(picker.time_field(), TimeField::Hour);
    }

    #[test]
    fn test_prev_time_field_hour_to_second() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Hour);
        picker.prev_time_field();
        assert_eq!(picker.time_field(), TimeField::Second);
    }

    #[test]
    fn test_prev_time_field_second_to_minute() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Second);
        picker.prev_time_field();
        assert_eq!(picker.time_field(), TimeField::Minute);
    }

    #[test]
    fn test_prev_time_field_minute_to_hour() {
        let mut picker = MockDateTimePicker::with_time_field(TimeField::Minute);
        picker.prev_time_field();
        assert_eq!(picker.time_field(), TimeField::Hour);
    }

    #[test]
    fn test_prev_time_field_hour_to_minute_no_seconds() {
        let mut picker = MockDateTimePicker::without_seconds();
        picker.set_time_field(TimeField::Hour);
        picker.prev_time_field();
        assert_eq!(picker.time_field(), TimeField::Minute);
    }
}
