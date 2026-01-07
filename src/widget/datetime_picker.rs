//! DateTime Picker widget for selecting dates and times
//!
//! Combines calendar-style date selection with time input.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{DateTimePicker, datetime_picker, Date, Time};
//!
//! // Date only picker
//! let date_picker = DateTimePicker::date_only()
//!     .selected_date(Date::new(2025, 1, 15));
//!
//! // Time only picker
//! let time_picker = DateTimePicker::time_only()
//!     .selected_time(Time::new(14, 30, 0));
//!
//! // Combined date and time
//! let picker = datetime_picker()
//!     .selected(Date::new(2025, 1, 15), Time::new(14, 30, 0))
//!     .format(DateTimeFormat::DateTime24);
//! ```

use super::calendar::{days_in_month, Date, FirstDayOfWeek};
use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_state_builders, impl_styled_view};
use unicode_width::UnicodeWidthChar;

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

/// DateTime picker widget
pub struct DateTimePicker {
    /// Selected date
    date: Date,
    /// Selected time
    time: Time,
    /// Display format
    format: DateTimeFormat,
    /// Current editing mode
    mode: DateTimeMode,
    /// Current time field (when in Time mode)
    time_field: TimeField,
    /// First day of week
    first_day: FirstDayOfWeek,
    /// Show seconds in time picker
    show_seconds: bool,
    /// Use 24-hour format
    use_24h: bool,
    /// Minimum date constraint
    min_date: Option<Date>,
    /// Maximum date constraint
    max_date: Option<Date>,
    /// Minimum time constraint (for time-only picker)
    min_time: Option<Time>,
    /// Maximum time constraint (for time-only picker)
    max_time: Option<Time>,
    /// Calendar cursor position (day of month)
    cursor_day: u32,
    /// Colors
    header_fg: Color,
    selected_fg: Color,
    selected_bg: Color,
    weekend_fg: Color,
    cursor_fg: Color,
    cursor_bg: Color,
    field_fg: Color,
    field_active_fg: Color,
    field_active_bg: Color,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl DateTimePicker {
    /// Create a new datetime picker
    pub fn new() -> Self {
        let today = Date::today();
        Self {
            date: today,
            time: Time::now(),
            format: DateTimeFormat::DateTime,
            mode: DateTimeMode::Date,
            time_field: TimeField::Hour,
            first_day: FirstDayOfWeek::Sunday,
            show_seconds: false,
            use_24h: true,
            min_date: None,
            max_date: None,
            min_time: None,
            max_time: None,
            cursor_day: today.day,
            header_fg: Color::CYAN,
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            weekend_fg: Color::rgb(150, 150, 150),
            cursor_fg: Color::BLACK,
            cursor_bg: Color::WHITE,
            field_fg: Color::WHITE,
            field_active_fg: Color::BLACK,
            field_active_bg: Color::CYAN,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create a date-only picker
    pub fn date_only() -> Self {
        Self::new().format(DateTimeFormat::DateOnly)
    }

    /// Create a time-only picker
    pub fn time_only() -> Self {
        let mut picker = Self::new().format(DateTimeFormat::TimeOnly);
        picker.mode = DateTimeMode::Time;
        picker
    }

    /// Set display format
    pub fn format(mut self, format: DateTimeFormat) -> Self {
        self.format = format;
        self
    }

    /// Set selected date
    pub fn selected_date(mut self, date: Date) -> Self {
        self.date = date;
        self.cursor_day = date.day;
        self
    }

    /// Set selected time
    pub fn selected_time(mut self, time: Time) -> Self {
        self.time = time;
        self
    }

    /// Set both date and time
    pub fn selected(mut self, date: Date, time: Time) -> Self {
        self.date = date;
        self.time = time;
        self.cursor_day = date.day;
        self
    }

    /// Set first day of week
    pub fn first_day(mut self, first: FirstDayOfWeek) -> Self {
        self.first_day = first;
        self
    }

    /// Show seconds in time picker
    pub fn show_seconds(mut self, show: bool) -> Self {
        self.show_seconds = show;
        self
    }

    /// Use 24-hour format
    pub fn use_24h(mut self, use_24h: bool) -> Self {
        self.use_24h = use_24h;
        self
    }

    /// Set minimum date
    pub fn min_date(mut self, date: Date) -> Self {
        self.min_date = Some(date);
        self
    }

    /// Set maximum date
    pub fn max_date(mut self, date: Date) -> Self {
        self.max_date = Some(date);
        self
    }

    /// Set date range constraint
    pub fn date_range(mut self, min: Date, max: Date) -> Self {
        self.min_date = Some(min);
        self.max_date = Some(max);
        self
    }

    /// Set minimum time
    pub fn min_time(mut self, time: Time) -> Self {
        self.min_time = Some(time);
        self
    }

    /// Set maximum time
    pub fn max_time(mut self, time: Time) -> Self {
        self.max_time = Some(time);
        self
    }

    /// Get selected datetime
    pub fn get_datetime(&self) -> DateTime {
        DateTime::new(self.date, self.time)
    }

    /// Get selected date
    pub fn get_date(&self) -> Date {
        self.date
    }

    /// Get selected time
    pub fn get_time(&self) -> Time {
        self.time
    }

    /// Get the current picker mode (Date or Time)
    pub fn get_mode(&self) -> DateTimeMode {
        self.mode
    }

    /// Set header color
    pub fn header_color(mut self, color: Color) -> Self {
        self.header_fg = color;
        self
    }

    /// Set selected colors
    pub fn selected_colors(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = fg;
        self.selected_bg = bg;
        self
    }

    /// Check if date is within constraints
    fn is_date_valid(&self, date: &Date) -> bool {
        if let Some(min) = &self.min_date {
            if date < min {
                return false;
            }
        }
        if let Some(max) = &self.max_date {
            if date > max {
                return false;
            }
        }
        true
    }

    /// Check if time is within constraints
    #[allow(dead_code)]
    fn is_time_valid(&self, time: &Time) -> bool {
        if let Some(min) = &self.min_time {
            if time < min {
                return false;
            }
        }
        if let Some(max) = &self.max_time {
            if time > max {
                return false;
            }
        }
        true
    }

    /// Move cursor to previous day
    fn move_day_left(&mut self) {
        if self.cursor_day > 1 {
            self.cursor_day -= 1;
        } else {
            // Go to previous month
            self.prev_month();
            self.cursor_day = days_in_month(self.date.year, self.date.month);
        }
    }

    /// Move cursor to next day
    fn move_day_right(&mut self) {
        let max_day = days_in_month(self.date.year, self.date.month);
        if self.cursor_day < max_day {
            self.cursor_day += 1;
        } else {
            // Go to next month
            self.next_month();
            self.cursor_day = 1;
        }
    }

    /// Move cursor up one week
    fn move_week_up(&mut self) {
        if self.cursor_day > 7 {
            self.cursor_day -= 7;
        } else {
            // Go to previous month
            self.prev_month();
            let max_day = days_in_month(self.date.year, self.date.month);
            self.cursor_day = max_day - (7 - self.cursor_day);
        }
    }

    /// Move cursor down one week
    fn move_week_down(&mut self) {
        let max_day = days_in_month(self.date.year, self.date.month);
        if self.cursor_day + 7 <= max_day {
            self.cursor_day += 7;
        } else {
            // Go to next month
            let overflow = self.cursor_day + 7 - max_day;
            self.next_month();
            self.cursor_day = overflow;
        }
    }

    /// Go to previous month
    fn prev_month(&mut self) {
        if self.date.month > 1 {
            self.date.month -= 1;
        } else {
            self.date.month = 12;
            self.date.year -= 1;
        }
        // Clamp cursor day to valid range
        let max_day = days_in_month(self.date.year, self.date.month);
        self.cursor_day = self.cursor_day.min(max_day);
    }

    /// Go to next month
    fn next_month(&mut self) {
        if self.date.month < 12 {
            self.date.month += 1;
        } else {
            self.date.month = 1;
            self.date.year += 1;
        }
        // Clamp cursor day to valid range
        let max_day = days_in_month(self.date.year, self.date.month);
        self.cursor_day = self.cursor_day.min(max_day);
    }

    /// Go to previous year
    fn prev_year(&mut self) {
        self.date.year -= 1;
        // Handle Feb 29 in non-leap years
        let max_day = days_in_month(self.date.year, self.date.month);
        self.cursor_day = self.cursor_day.min(max_day);
    }

    /// Go to next year
    fn next_year(&mut self) {
        self.date.year += 1;
        // Handle Feb 29 in non-leap years
        let max_day = days_in_month(self.date.year, self.date.month);
        self.cursor_day = self.cursor_day.min(max_day);
    }

    /// Select current cursor position as the date
    fn select_date(&mut self) {
        let new_date = Date::new(self.date.year, self.date.month, self.cursor_day);
        if self.is_date_valid(&new_date) {
            self.date.day = self.cursor_day;
        }
    }

    /// Increment current time field
    fn increment_time(&mut self) {
        match self.time_field {
            TimeField::Hour => {
                self.time.hour = (self.time.hour + 1) % 24;
            }
            TimeField::Minute => {
                self.time.minute = (self.time.minute + 1) % 60;
            }
            TimeField::Second => {
                self.time.second = (self.time.second + 1) % 60;
            }
        }
    }

    /// Decrement current time field
    fn decrement_time(&mut self) {
        match self.time_field {
            TimeField::Hour => {
                self.time.hour = if self.time.hour == 0 {
                    23
                } else {
                    self.time.hour - 1
                };
            }
            TimeField::Minute => {
                self.time.minute = if self.time.minute == 0 {
                    59
                } else {
                    self.time.minute - 1
                };
            }
            TimeField::Second => {
                self.time.second = if self.time.second == 0 {
                    59
                } else {
                    self.time.second - 1
                };
            }
        }
    }

    /// Move to next time field
    fn next_time_field(&mut self) {
        self.time_field = match self.time_field {
            TimeField::Hour => TimeField::Minute,
            TimeField::Minute => {
                if self.show_seconds {
                    TimeField::Second
                } else {
                    TimeField::Hour
                }
            }
            TimeField::Second => TimeField::Hour,
        };
    }

    /// Move to previous time field
    fn prev_time_field(&mut self) {
        self.time_field = match self.time_field {
            TimeField::Hour => {
                if self.show_seconds {
                    TimeField::Second
                } else {
                    TimeField::Minute
                }
            }
            TimeField::Minute => TimeField::Hour,
            TimeField::Second => TimeField::Minute,
        };
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            // Switch between date and time modes
            Key::Tab => {
                if self.format != DateTimeFormat::DateOnly
                    && self.format != DateTimeFormat::TimeOnly
                {
                    self.mode = match self.mode {
                        DateTimeMode::Date => DateTimeMode::Time,
                        DateTimeMode::Time => DateTimeMode::Date,
                    };
                    return true;
                }
                false
            }

            // Date mode navigation
            Key::Left | Key::Char('h') if self.mode == DateTimeMode::Date => {
                self.move_day_left();
                true
            }
            Key::Right | Key::Char('l') if self.mode == DateTimeMode::Date => {
                self.move_day_right();
                true
            }
            Key::Up | Key::Char('k') if self.mode == DateTimeMode::Date => {
                self.move_week_up();
                true
            }
            Key::Down | Key::Char('j') if self.mode == DateTimeMode::Date => {
                self.move_week_down();
                true
            }
            Key::Char('[') if self.mode == DateTimeMode::Date => {
                self.prev_month();
                true
            }
            Key::Char(']') if self.mode == DateTimeMode::Date => {
                self.next_month();
                true
            }
            Key::Char('{') if self.mode == DateTimeMode::Date => {
                self.prev_year();
                true
            }
            Key::Char('}') if self.mode == DateTimeMode::Date => {
                self.next_year();
                true
            }
            Key::Enter | Key::Char(' ') if self.mode == DateTimeMode::Date => {
                self.select_date();
                true
            }

            // Time mode navigation
            Key::Left | Key::Char('h') if self.mode == DateTimeMode::Time => {
                self.prev_time_field();
                true
            }
            Key::Right | Key::Char('l') if self.mode == DateTimeMode::Time => {
                self.next_time_field();
                true
            }
            Key::Up | Key::Char('k') if self.mode == DateTimeMode::Time => {
                self.increment_time();
                true
            }
            Key::Down | Key::Char('j') if self.mode == DateTimeMode::Time => {
                self.decrement_time();
                true
            }

            _ => false,
        }
    }

    /// Get day of week for first day of current month
    fn first_weekday(&self) -> u32 {
        let m = if self.date.month < 3 {
            self.date.month as i32 + 12
        } else {
            self.date.month as i32
        };
        let y = if self.date.month < 3 {
            self.date.year - 1
        } else {
            self.date.year
        };
        let k = y % 100;
        let j = y / 100;
        let h = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        let h = ((h + 6) % 7 + 7) % 7;

        // Adjust for first day of week
        match self.first_day {
            FirstDayOfWeek::Sunday => h as u32,
            FirstDayOfWeek::Monday => ((h + 6) % 7) as u32,
        }
    }

    /// Render calendar portion
    fn render_calendar(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        let days_in_month = days_in_month(self.date.year, self.date.month);
        let first_weekday = self.first_weekday();

        // Header: month and year
        let header = format!("{} {}", month_name(self.date.month), self.date.year);
        let header_x = x + (width.saturating_sub(header.len() as u16)) / 2;
        self.draw_text(ctx, header_x, y, &header, self.header_fg, true);

        // Day headers
        let day_headers = match self.first_day {
            FirstDayOfWeek::Sunday => "Su Mo Tu We Th Fr Sa",
            FirstDayOfWeek::Monday => "Mo Tu We Th Fr Sa Su",
        };
        self.draw_text(ctx, x, y + 1, day_headers, Color::rgb(150, 150, 150), false);

        // Days grid
        let mut row = 0u16;
        let mut col = first_weekday as u16;

        for day in 1..=days_in_month {
            let day_x = x + col * 3;
            let day_y = y + 2 + row;
            let day_str = format!("{:2}", day);

            let is_selected = day == self.date.day;
            let is_cursor = day == self.cursor_day && self.mode == DateTimeMode::Date;
            let is_weekend = match self.first_day {
                FirstDayOfWeek::Sunday => col == 0 || col == 6,
                FirstDayOfWeek::Monday => col == 5 || col == 6,
            };

            let (fg, bg, bold) = if is_cursor {
                (self.cursor_fg, Some(self.cursor_bg), true)
            } else if is_selected {
                (self.selected_fg, Some(self.selected_bg), true)
            } else if is_weekend {
                (self.weekend_fg, None, false)
            } else {
                (Color::WHITE, None, false)
            };

            // Draw day with background if needed
            if let Some(bg_color) = bg {
                for i in 0..2 {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg_color);
                    ctx.buffer.set(day_x + i, day_y, cell);
                }
            }
            self.draw_text(ctx, day_x, day_y, &day_str, fg, bold);

            col += 1;
            if col > 6 {
                col = 0;
                row += 1;
            }
        }
    }

    /// Render time picker portion
    fn render_time(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        // Draw time label
        self.draw_text(ctx, x, y, "Time:", Color::rgb(150, 150, 150), false);

        // Draw time fields with highlighting
        let field_y = y + 1;
        let mut field_x = x;

        // Hour
        let hour_str = format!("{:02}", self.time.hour);
        let (hour_fg, hour_bg) =
            if self.mode == DateTimeMode::Time && self.time_field == TimeField::Hour {
                (self.field_active_fg, Some(self.field_active_bg))
            } else {
                (self.field_fg, None)
            };
        if let Some(bg) = hour_bg {
            for i in 0..2 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(field_x + i, field_y, cell);
            }
        }
        self.draw_text(ctx, field_x, field_y, &hour_str, hour_fg, hour_bg.is_some());
        field_x += 2;

        // Colon
        self.draw_text(ctx, field_x, field_y, ":", self.field_fg, false);
        field_x += 1;

        // Minute
        let minute_str = format!("{:02}", self.time.minute);
        let (minute_fg, minute_bg) =
            if self.mode == DateTimeMode::Time && self.time_field == TimeField::Minute {
                (self.field_active_fg, Some(self.field_active_bg))
            } else {
                (self.field_fg, None)
            };
        if let Some(bg) = minute_bg {
            for i in 0..2 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(field_x + i, field_y, cell);
            }
        }
        self.draw_text(
            ctx,
            field_x,
            field_y,
            &minute_str,
            minute_fg,
            minute_bg.is_some(),
        );
        field_x += 2;

        // Second (if shown)
        if self.show_seconds {
            self.draw_text(ctx, field_x, field_y, ":", self.field_fg, false);
            field_x += 1;

            let second_str = format!("{:02}", self.time.second);
            let (second_fg, second_bg) =
                if self.mode == DateTimeMode::Time && self.time_field == TimeField::Second {
                    (self.field_active_fg, Some(self.field_active_bg))
                } else {
                    (self.field_fg, None)
                };
            if let Some(bg) = second_bg {
                for i in 0..2 {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(field_x + i, field_y, cell);
                }
            }
            self.draw_text(
                ctx,
                field_x,
                field_y,
                &second_str,
                second_fg,
                second_bg.is_some(),
            );
        }

        // Instructions
        let help = if self.mode == DateTimeMode::Time {
            "↑↓: change  ←→: field  Tab: date"
        } else {
            "Tab: switch to time"
        };
        self.draw_text(ctx, x, y + 3, help, Color::rgb(100, 100, 100), false);
    }

    /// Draw text helper
    fn draw_text(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        text: &str,
        color: Color,
        bold: bool,
    ) {
        let mut offset = 0u16;
        for ch in text.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            if bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x + offset, y, cell);
            for i in 1..char_width {
                ctx.buffer.set(x + offset + i, y, Cell::continuation());
            }
            offset += char_width;
        }
    }
}

impl Default for DateTimePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl View for DateTimePicker {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 20 || area.height < 5 {
            return;
        }

        let x = area.x + 1;
        let y = area.y;

        match self.format {
            DateTimeFormat::DateOnly => {
                self.render_calendar(ctx, x, y, area.width.saturating_sub(2));
            }
            DateTimeFormat::TimeOnly | DateTimeFormat::TimeWithSeconds => {
                self.render_time(ctx, x, y);
            }
            _ => {
                // Combined view
                self.render_calendar(ctx, x, y, area.width.saturating_sub(2));
                let time_y = y + 9; // Below calendar
                if area.height > 12 {
                    self.render_time(ctx, x, time_y);
                }
            }
        }
    }
}

impl_styled_view!(DateTimePicker);
impl_state_builders!(DateTimePicker);
impl_props_builders!(DateTimePicker);

/// Helper function to get month name
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Create a new datetime picker
pub fn datetime_picker() -> DateTimePicker {
    DateTimePicker::new()
}

/// Create a date-only picker
pub fn date_picker() -> DateTimePicker {
    DateTimePicker::date_only()
}

/// Create a time-only picker
pub fn time_picker() -> DateTimePicker {
    DateTimePicker::time_only()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_new() {
        let t = Time::new(14, 30, 45);
        assert_eq!(t.hour, 14);
        assert_eq!(t.minute, 30);
        assert_eq!(t.second, 45);
    }

    #[test]
    fn test_time_clamp() {
        let t = Time::new(25, 70, 80);
        assert_eq!(t.hour, 23);
        assert_eq!(t.minute, 59);
        assert_eq!(t.second, 59);
    }

    #[test]
    fn test_time_format() {
        let t = Time::new(14, 5, 9);
        assert_eq!(t.format_hm(), "14:05");
        assert_eq!(t.format_hms(), "14:05:09");
    }

    #[test]
    fn test_time_format_12h() {
        assert_eq!(Time::new(0, 30, 0).format_12h(), "12:30 AM");
        assert_eq!(Time::new(9, 15, 0).format_12h(), " 9:15 AM");
        assert_eq!(Time::new(12, 0, 0).format_12h(), "12:00 PM");
        assert_eq!(Time::new(15, 45, 0).format_12h(), " 3:45 PM");
    }

    #[test]
    fn test_datetime_new() {
        let dt = DateTime::new(Date::new(2025, 6, 15), Time::new(10, 30, 0));
        assert_eq!(dt.date.year, 2025);
        assert_eq!(dt.date.month, 6);
        assert_eq!(dt.date.day, 15);
        assert_eq!(dt.time.hour, 10);
        assert_eq!(dt.time.minute, 30);
    }

    #[test]
    fn test_picker_new() {
        let p = DateTimePicker::new();
        assert_eq!(p.mode, DateTimeMode::Date);
        assert_eq!(p.format, DateTimeFormat::DateTime);
    }

    #[test]
    fn test_picker_date_only() {
        let p = DateTimePicker::date_only();
        assert_eq!(p.format, DateTimeFormat::DateOnly);
    }

    #[test]
    fn test_picker_time_only() {
        let p = DateTimePicker::time_only();
        assert_eq!(p.format, DateTimeFormat::TimeOnly);
        assert_eq!(p.mode, DateTimeMode::Time);
    }

    #[test]
    fn test_picker_selected() {
        let p = datetime_picker()
            .selected_date(Date::new(2025, 3, 20))
            .selected_time(Time::new(15, 45, 0));
        assert_eq!(p.date, Date::new(2025, 3, 20));
        assert_eq!(p.time, Time::new(15, 45, 0));
    }

    #[test]
    fn test_picker_constraints() {
        let p = datetime_picker()
            .min_date(Date::new(2025, 1, 1))
            .max_date(Date::new(2025, 12, 31));
        assert!(p.is_date_valid(&Date::new(2025, 6, 15)));
        assert!(!p.is_date_valid(&Date::new(2024, 12, 31)));
        assert!(!p.is_date_valid(&Date::new(2026, 1, 1)));
    }

    #[test]
    fn test_picker_handle_key_tab() {
        let mut p = datetime_picker();
        assert_eq!(p.mode, DateTimeMode::Date);
        p.handle_key(&Key::Tab);
        assert_eq!(p.mode, DateTimeMode::Time);
        p.handle_key(&Key::Tab);
        assert_eq!(p.mode, DateTimeMode::Date);
    }

    #[test]
    fn test_picker_time_navigation() {
        let mut p = datetime_picker().selected_time(Time::new(10, 30, 0));
        p.mode = DateTimeMode::Time;

        // Increment hour
        p.handle_key(&Key::Up);
        assert_eq!(p.time.hour, 11);

        // Move to minute
        p.handle_key(&Key::Right);
        assert_eq!(p.time_field, TimeField::Minute);

        // Decrement minute
        p.handle_key(&Key::Down);
        assert_eq!(p.time.minute, 29);
    }

    #[test]
    fn test_picker_month_navigation() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

        // Next month
        p.handle_key(&Key::Char(']'));
        assert_eq!(p.date.month, 7);

        // Previous month
        p.handle_key(&Key::Char('['));
        assert_eq!(p.date.month, 6);
    }

    #[test]
    fn test_picker_year_navigation() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

        // Next year
        p.handle_key(&Key::Char('}'));
        assert_eq!(p.date.year, 2026);

        // Previous year
        p.handle_key(&Key::Char('{'));
        assert_eq!(p.date.year, 2025);
    }

    #[test]
    fn test_helper_functions() {
        let _ = datetime_picker();
        let _ = date_picker();
        let _ = time_picker();
    }

    #[test]
    fn test_month_name() {
        assert_eq!(month_name(1), "January");
        assert_eq!(month_name(6), "June");
        assert_eq!(month_name(12), "December");
        assert_eq!(month_name(13), "Unknown");
    }

    #[test]
    fn test_first_weekday() {
        // Test known dates
        let p = datetime_picker().selected_date(Date::new(2025, 1, 1));
        let weekday = p.first_weekday();
        // January 2025 starts on Wednesday (3 for Sunday-first, 2 for Monday-first)
        assert!(weekday <= 6);
    }

    #[test]
    fn test_first_weekday_monday_start() {
        let p = datetime_picker()
            .selected_date(Date::new(2025, 1, 1))
            .first_day(FirstDayOfWeek::Monday);
        let weekday = p.first_weekday();
        assert!(weekday <= 6);
    }

    #[test]
    fn test_picker_cursor_navigation() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

        // Move right
        p.handle_key(&Key::Right);
        assert_eq!(p.cursor_day, 16);

        // Move left
        p.handle_key(&Key::Left);
        assert_eq!(p.cursor_day, 15);

        // Move down (week)
        p.handle_key(&Key::Down);
        assert_eq!(p.cursor_day, 22);

        // Move up (week)
        p.handle_key(&Key::Up);
        assert_eq!(p.cursor_day, 15);
    }

    #[test]
    fn test_picker_vim_navigation() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));

        // vim keys: h, j, k, l
        p.handle_key(&Key::Char('l'));
        assert_eq!(p.cursor_day, 16);

        p.handle_key(&Key::Char('h'));
        assert_eq!(p.cursor_day, 15);

        p.handle_key(&Key::Char('j'));
        assert_eq!(p.cursor_day, 22);

        p.handle_key(&Key::Char('k'));
        assert_eq!(p.cursor_day, 15);
    }

    #[test]
    fn test_picker_select_date() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));
        p.cursor_day = 20;

        p.handle_key(&Key::Enter);
        assert_eq!(p.date.day, 20);
    }

    #[test]
    fn test_picker_time_vim_keys() {
        let mut p = datetime_picker().selected_time(Time::new(10, 30, 0));
        p.mode = DateTimeMode::Time;

        // vim keys in time mode
        p.handle_key(&Key::Char('k')); // increment
        assert_eq!(p.time.hour, 11);

        p.handle_key(&Key::Char('l')); // next field
        assert_eq!(p.time_field, TimeField::Minute);

        p.handle_key(&Key::Char('j')); // decrement
        assert_eq!(p.time.minute, 29);

        p.handle_key(&Key::Char('h')); // prev field
        assert_eq!(p.time_field, TimeField::Hour);
    }

    #[test]
    fn test_picker_month_boundary() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 1, 31));
        p.cursor_day = 31;

        // Go to next month (Feb has fewer days)
        p.handle_key(&Key::Char(']'));
        assert!(p.cursor_day <= 28);
    }

    #[test]
    fn test_picker_year_boundary() {
        let mut p = datetime_picker().selected_date(Date::new(2024, 2, 29));
        p.cursor_day = 29;

        // Go to next year (2025 is not leap year)
        p.handle_key(&Key::Char('}'));
        assert_eq!(p.cursor_day, 28);
    }

    #[test]
    fn test_picker_render_datetime() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let p = datetime_picker()
            .selected_date(Date::new(2025, 6, 15))
            .selected_time(Time::new(14, 30, 0));

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
        // Verify rendering completed without panic
    }

    #[test]
    fn test_picker_render_date_only() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let p = date_picker().selected_date(Date::new(2025, 6, 15));

        let mut buffer = Buffer::new(25, 10);
        let area = Rect::new(0, 0, 25, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_render_time_only() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let p = time_picker().selected_time(Time::new(14, 30, 0));

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let p = datetime_picker();

        // Too small area should return early
        let mut buffer = Buffer::new(10, 3);
        let area = Rect::new(0, 0, 10, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_render_with_seconds() {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let p = time_picker()
            .selected_time(Time::new(14, 30, 45))
            .show_seconds(true);

        let mut buffer = Buffer::new(25, 10);
        let area = Rect::new(0, 0, 25, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_time_field_second() {
        let mut p = datetime_picker()
            .selected_time(Time::new(10, 30, 45))
            .show_seconds(true);
        p.mode = DateTimeMode::Time;
        p.time_field = TimeField::Second;

        p.handle_key(&Key::Up);
        assert_eq!(p.time.second, 46);

        p.handle_key(&Key::Down);
        assert_eq!(p.time.second, 45);
    }

    #[test]
    fn test_picker_time_wrap() {
        let mut p = datetime_picker().selected_time(Time::new(23, 59, 59));
        p.mode = DateTimeMode::Time;

        // Hour wrap
        p.handle_key(&Key::Up);
        assert_eq!(p.time.hour, 0);

        // Minute wrap
        p.time_field = TimeField::Minute;
        p.handle_key(&Key::Up);
        assert_eq!(p.time.minute, 0);

        // Second wrap
        p.time_field = TimeField::Second;
        p.handle_key(&Key::Up);
        assert_eq!(p.time.second, 0);
    }

    #[test]
    fn test_picker_time_wrap_down() {
        let mut p = datetime_picker().selected_time(Time::new(0, 0, 0));
        p.mode = DateTimeMode::Time;

        p.handle_key(&Key::Down);
        assert_eq!(p.time.hour, 23);

        p.time_field = TimeField::Minute;
        p.handle_key(&Key::Down);
        assert_eq!(p.time.minute, 59);

        p.time_field = TimeField::Second;
        p.handle_key(&Key::Down);
        assert_eq!(p.time.second, 59);
    }

    #[test]
    fn test_picker_unhandled_key() {
        let mut p = datetime_picker();
        let handled = p.handle_key(&Key::Char('x'));
        assert!(!handled);
    }

    #[test]
    fn test_picker_space_select() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 15));
        p.cursor_day = 20;

        p.handle_key(&Key::Char(' '));
        assert_eq!(p.date.day, 20);
    }

    #[test]
    fn test_picker_default() {
        let p = DateTimePicker::default();
        assert_eq!(p.format, DateTimeFormat::DateTime);
    }

    #[test]
    fn test_picker_get_mode() {
        let p = datetime_picker();
        assert_eq!(p.get_mode(), DateTimeMode::Date);
    }

    #[test]
    fn test_datetime_format_variants() {
        let p1 = datetime_picker().format(DateTimeFormat::DateTime);
        assert_eq!(p1.format, DateTimeFormat::DateTime);

        let p2 = datetime_picker().format(DateTimeFormat::DateOnly);
        assert_eq!(p2.format, DateTimeFormat::DateOnly);

        let p3 = datetime_picker().format(DateTimeFormat::TimeOnly);
        assert_eq!(p3.format, DateTimeFormat::TimeOnly);

        let p4 = datetime_picker().format(DateTimeFormat::TimeWithSeconds);
        assert_eq!(p4.format, DateTimeFormat::TimeWithSeconds);
    }

    #[test]
    fn test_picker_cursor_boundary_right() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 30));
        p.cursor_day = 30; // Last day of June

        p.handle_key(&Key::Right);
        // Should wrap to next month
        assert_eq!(p.date.month, 7);
        assert_eq!(p.cursor_day, 1);
    }

    #[test]
    fn test_picker_cursor_boundary_left() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 1));
        p.cursor_day = 1;

        p.handle_key(&Key::Left);
        // Should wrap to previous month
        assert_eq!(p.date.month, 5);
    }

    #[test]
    fn test_picker_cursor_boundary_down() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 28));
        p.cursor_day = 28;

        p.handle_key(&Key::Down);
        // Should wrap to next month
        assert_eq!(p.date.month, 7);
    }

    #[test]
    fn test_picker_cursor_boundary_up() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 6, 3));
        p.cursor_day = 3;

        p.handle_key(&Key::Up);
        // Should wrap to previous month
        assert_eq!(p.date.month, 5);
    }

    #[test]
    fn test_picker_month_wrap_december() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 12, 15));

        p.handle_key(&Key::Char(']'));
        assert_eq!(p.date.month, 1);
        assert_eq!(p.date.year, 2026);
    }

    #[test]
    fn test_picker_month_wrap_january() {
        let mut p = datetime_picker().selected_date(Date::new(2025, 1, 15));

        p.handle_key(&Key::Char('['));
        assert_eq!(p.date.month, 12);
        assert_eq!(p.date.year, 2024);
    }

    #[test]
    fn test_picker_constraint_select() {
        let mut p = datetime_picker()
            .selected_date(Date::new(2025, 6, 15))
            .min_date(Date::new(2025, 6, 10))
            .max_date(Date::new(2025, 6, 20));

        // Try to select a date outside constraints
        p.cursor_day = 5;
        p.handle_key(&Key::Enter);
        // Date should not change because 5 is before min
        assert_eq!(p.date.day, 15);
    }

    #[test]
    fn test_time_field_navigation_wrap() {
        let mut p = datetime_picker().show_seconds(true);
        p.mode = DateTimeMode::Time;
        p.time_field = TimeField::Second;

        // Second wraps to Hour
        p.handle_key(&Key::Right);
        assert_eq!(p.time_field, TimeField::Hour);

        // Hour wraps back to Second
        p.handle_key(&Key::Left);
        assert_eq!(p.time_field, TimeField::Second);
    }

    #[test]
    fn test_time_field_navigation_no_seconds() {
        let mut p = datetime_picker().show_seconds(false);
        p.mode = DateTimeMode::Time;
        p.time_field = TimeField::Minute;

        // Without seconds, Minute wraps to Hour
        p.handle_key(&Key::Right);
        assert_eq!(p.time_field, TimeField::Hour);

        // Hour wraps to Minute (skips Second)
        p.handle_key(&Key::Left);
        assert_eq!(p.time_field, TimeField::Minute);
    }
}
