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

#![allow(missing_docs)]
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

mod helpers;
mod input;
mod navigation;
pub mod render;
pub mod types;

use crate::event::Key;
use crate::style::Color;
use crate::widget::data::calendar::{Date, FirstDayOfWeek};
use crate::widget::datetime_picker::render::Rendering;
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

pub use types::{DateTime, DateTimeFormat, DateTimeMode, Time, TimeField};

// Re-export helpers
pub use helpers::{date_picker, datetime_picker, time_picker};

/// DateTime picker widget
pub struct DateTimePicker {
    /// Selected date
    pub date: Date,
    /// Selected time
    pub time: Time,
    /// Display format
    pub format: DateTimeFormat,
    /// Current editing mode
    pub mode: DateTimeMode,
    /// Current time field (when in Time mode)
    pub time_field: TimeField,
    /// First day of week
    pub first_day: FirstDayOfWeek,
    /// Show seconds in time picker
    pub show_seconds: bool,
    /// Use 24-hour format
    pub use_24h: bool,
    /// Minimum date constraint
    pub min_date: Option<Date>,
    /// Maximum date constraint
    pub max_date: Option<Date>,
    /// Minimum time constraint (for time-only picker)
    pub min_time: Option<Time>,
    /// Maximum time constraint (for time-only picker)
    pub max_time: Option<Time>,
    /// Calendar cursor position (day of month)
    pub cursor_day: u32,
    /// Colors
    pub header_fg: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub weekend_fg: Color,
    pub cursor_fg: Color,
    pub cursor_bg: Color,
    pub field_fg: Color,
    pub field_active_fg: Color,
    pub field_active_bg: Color,
    /// Widget state
    pub state: WidgetState,
    /// Widget properties
    pub props: WidgetProps,
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
    pub fn is_date_valid(&self, date: &Date) -> bool {
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

    /// Handle keyboard input (delegates to input module)
    pub fn handle_key(&mut self, key: &Key) -> bool {
        <Self as input::Input>::handle_key(self, key)
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

    crate::impl_view_meta!("DateTimePicker");
}

impl_styled_view!(DateTimePicker);
impl_widget_builders!(DateTimePicker);

// Implement navigation trait
impl navigation::Navigation for DateTimePicker {
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

    fn is_date_valid(&self, date: &Date) -> bool {
        self.is_date_valid(date)
    }
}

// Implement input trait
impl input::Input for DateTimePicker {
    fn mode(&self) -> DateTimeMode {
        self.mode
    }

    fn format(&self) -> DateTimeFormat {
        self.format
    }

    fn is_disabled(&self) -> bool {
        self.state.disabled
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            DateTimeMode::Date => DateTimeMode::Time,
            DateTimeMode::Time => DateTimeMode::Date,
        };
    }

    fn nav_move_day_left(&mut self) {
        <Self as navigation::Navigation>::move_day_left(self);
    }

    fn nav_move_day_right(&mut self) {
        <Self as navigation::Navigation>::move_day_right(self);
    }

    fn nav_move_week_up(&mut self) {
        <Self as navigation::Navigation>::move_week_up(self);
    }

    fn nav_move_week_down(&mut self) {
        <Self as navigation::Navigation>::move_week_down(self);
    }

    fn nav_prev_month(&mut self) {
        <Self as navigation::Navigation>::prev_month(self);
    }

    fn nav_next_month(&mut self) {
        <Self as navigation::Navigation>::next_month(self);
    }

    fn nav_prev_year(&mut self) {
        <Self as navigation::Navigation>::prev_year(self);
    }

    fn nav_next_year(&mut self) {
        <Self as navigation::Navigation>::next_year(self);
    }

    fn nav_select_date(&mut self) {
        <Self as navigation::Navigation>::select_date(self);
    }

    fn nav_prev_time_field(&mut self) {
        <Self as navigation::Navigation>::prev_time_field(self);
    }

    fn nav_next_time_field(&mut self) {
        <Self as navigation::Navigation>::next_time_field(self);
    }

    fn nav_increment_time(&mut self) {
        <Self as navigation::Navigation>::increment_time(self);
    }

    fn nav_decrement_time(&mut self) {
        <Self as navigation::Navigation>::decrement_time(self);
    }
}

// Implement rendering trait
impl render::Rendering for DateTimePicker {
    fn date(&self) -> Date {
        self.date
    }

    fn time(&self) -> Time {
        self.time
    }

    fn mode(&self) -> DateTimeMode {
        self.mode
    }

    fn first_day(&self) -> FirstDayOfWeek {
        self.first_day
    }

    fn cursor_day(&self) -> u32 {
        self.cursor_day
    }

    fn time_field(&self) -> TimeField {
        self.time_field
    }

    fn show_seconds(&self) -> bool {
        self.show_seconds
    }

    fn header_fg(&self) -> Color {
        self.header_fg
    }

    fn selected_fg(&self) -> Color {
        self.selected_fg
    }

    fn selected_bg(&self) -> Color {
        self.selected_bg
    }

    fn weekend_fg(&self) -> Color {
        self.weekend_fg
    }

    fn cursor_fg(&self) -> Color {
        self.cursor_fg
    }

    fn cursor_bg(&self) -> Color {
        self.cursor_bg
    }

    fn field_fg(&self) -> Color {
        self.field_fg
    }

    fn field_active_fg(&self) -> Color {
        self.field_active_fg
    }

    fn field_active_bg(&self) -> Color {
        self.field_active_bg
    }
}

#[cfg(test)]
mod tests {
    //! DateTime picker tests

    use super::helpers::month_name;
    use super::render::Rendering;
    use super::types::{DateTime, DateTimeFormat, DateTimeMode, Time, TimeField};
    use super::{date_picker, datetime_picker, time_picker, DateTimePicker};
    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::data::calendar::Date;
    use crate::widget::data::calendar::FirstDayOfWeek;
    use crate::widget::traits::{RenderContext, View};

    // =========================================================================
    // Time Tests
    // =========================================================================

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

    // =========================================================================
    // DateTime Tests
    // =========================================================================

    #[test]
    fn test_datetime_new() {
        let dt = DateTime::new(Date::new(2025, 6, 15), Time::new(10, 30, 0));
        assert_eq!(dt.date.year, 2025);
        assert_eq!(dt.date.month, 6);
        assert_eq!(dt.date.day, 15);
        assert_eq!(dt.time.hour, 10);
        assert_eq!(dt.time.minute, 30);
    }

    // =========================================================================
    // Picker Creation Tests
    // =========================================================================

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
        let p = date_picker().selected_date(Date::new(2025, 6, 15));

        let mut buffer = Buffer::new(25, 10);
        let area = Rect::new(0, 0, 25, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_render_time_only() {
        let p = time_picker().selected_time(Time::new(14, 30, 0));

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_render_small_area() {
        let p = datetime_picker();

        // Too small area should return early
        let mut buffer = Buffer::new(10, 3);
        let area = Rect::new(0, 0, 10, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        p.render(&mut ctx);
    }

    #[test]
    fn test_picker_render_with_seconds() {
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
