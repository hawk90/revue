//! Calendar widget for date display and selection
//!
//! Supports month/year navigation, date selection, range selection,
//! and custom styling for different date types.

use super::traits::{RenderContext, View, WidgetProps};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

mod date;
mod render;
mod types;
mod utils;

#[cfg(test)]
mod tests {
    //! Tests for calendar module

    use super::types::{CalendarMode, DateMarker, FirstDayOfWeek};
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_calendar_new() {
        let cal = Calendar::new(2025, 1);
        assert_eq!(cal.year, 2025);
        assert_eq!(cal.month, 1);
    }

    #[test]
    fn test_calendar_month_clamp() {
        let cal = Calendar::new(2025, 13);
        assert_eq!(cal.month, 12);

        let cal = Calendar::new(2025, 0);
        assert_eq!(cal.month, 1);
    }

    #[test]
    fn test_date_new() {
        let date = Date::new(2025, 6, 15);
        assert_eq!(date.year, 2025);
        assert_eq!(date.month, 6);
        assert_eq!(date.day, 15);
    }

    #[test]
    fn test_date_valid() {
        assert!(Date::new(2025, 1, 1).is_valid());
        assert!(Date::new(2025, 2, 28).is_valid());
        assert!(Date::new(2024, 2, 29).is_valid()); // Leap year
        assert!(!Date::new(2025, 2, 29).is_valid()); // Not leap year
        assert!(!Date::new(2025, 13, 1).is_valid());
        assert!(!Date::new(2025, 1, 32).is_valid());
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2025, 1), 31);
        assert_eq!(days_in_month(2025, 2), 28);
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 4), 30);
    }

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2025));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_calendar_navigation() {
        let mut cal = Calendar::new(2025, 1);

        cal.next_month();
        assert_eq!(cal.month, 2);

        cal.prev_month();
        assert_eq!(cal.month, 1);

        cal.prev_month();
        assert_eq!(cal.month, 12);
        assert_eq!(cal.year, 2024);

        cal.next_month();
        assert_eq!(cal.month, 1);
        assert_eq!(cal.year, 2025);
    }

    #[test]
    fn test_calendar_year_navigation() {
        let mut cal = Calendar::new(2025, 6);

        cal.next_year();
        assert_eq!(cal.year, 2026);

        cal.prev_year();
        assert_eq!(cal.year, 2025);
    }

    #[test]
    fn test_calendar_selection() {
        let mut cal = Calendar::new(2025, 1);

        cal.select(Date::new(2025, 1, 15));
        assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 15)));

        cal.clear_selection();
        assert_eq!(cal.get_selected(), None);
    }

    #[test]
    fn test_calendar_select_next_day() {
        let mut cal = Calendar::new(2025, 1).selected(Date::new(2025, 1, 31));

        cal.select_next_day();
        assert_eq!(cal.get_selected(), Some(Date::new(2025, 2, 1)));
        assert_eq!(cal.month, 2);
    }

    #[test]
    fn test_calendar_select_prev_day() {
        let mut cal = Calendar::new(2025, 2).selected(Date::new(2025, 2, 1));

        cal.select_prev_day();
        assert_eq!(cal.get_selected(), Some(Date::new(2025, 1, 31)));
        assert_eq!(cal.month, 1);
    }

    #[test]
    fn test_date_marker() {
        let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED).symbol('★');

        assert_eq!(marker.date, Date::new(2025, 1, 1));
        assert_eq!(marker.color, Color::RED);
        assert_eq!(marker.symbol, Some('★'));
    }

    #[test]
    fn test_calendar_range() {
        let _cal = Calendar::new(2025, 1).range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));

        let render_state = render::CalendarRender {
            year: 2025,
            month: 1,
            selected: Some(Date::new(2025, 1, 10)),
            range_end: Some(Date::new(2025, 1, 20)),
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::CYAN,
            header_bg: None,
            day_fg: Color::WHITE,
            weekend_fg: Color::rgb(150, 150, 150),
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            today_fg: Color::YELLOW,
            outside_fg: Color::rgb(80, 80, 80),
            border_color: None,
            focused: false,
        };

        assert!(render_state.is_in_range(&Date::new(2025, 1, 15)));
        assert!(!render_state.is_in_range(&Date::new(2025, 1, 5)));
    }

    #[test]
    fn test_calendar_render() {
        let mut buffer = Buffer::new(30, 12);
        let area = Rect::new(0, 0, 30, 12);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cal = Calendar::new(2025, 1)
            .selected(Date::new(2025, 1, 15))
            .today(Date::new(2025, 1, 10));

        cal.render(&mut ctx);
        // Smoke test - renders without panic
    }

    #[test]
    fn test_calendar_with_border() {
        let mut buffer = Buffer::new(30, 12);
        let area = Rect::new(0, 0, 30, 12);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cal = Calendar::new(2025, 1).border(Color::WHITE);
        cal.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    #[test]
    fn test_calendar_first_day() {
        let _cal_sun = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Sunday);
        let _cal_mon = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Monday);

        let render_state_sun = render::CalendarRender {
            year: 2025,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::CYAN,
            header_bg: None,
            day_fg: Color::WHITE,
            weekend_fg: Color::rgb(150, 150, 150),
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            today_fg: Color::YELLOW,
            outside_fg: Color::rgb(80, 80, 80),
            border_color: None,
            focused: false,
        };

        let render_state_mon = render::CalendarRender {
            first_day: FirstDayOfWeek::Monday,
            ..render_state_sun
        };

        assert_eq!(render_state_sun.day_names()[0], "Su");
        assert_eq!(render_state_mon.day_names()[0], "Mo");
    }

    #[test]
    fn test_first_day_of_month() {
        // January 1, 2025 is Wednesday
        assert_eq!(first_day_of_month(2025, 1), 3);
    }

    #[test]
    fn test_calendar_helper() {
        let cal = calendar(2025, 6);
        assert_eq!(cal.year, 2025);
        assert_eq!(cal.month, 6);
    }

    #[test]
    fn test_iso_week_number() {
        let _cal = Calendar::new(2025, 1);

        let render_state = render::CalendarRender {
            year: 2025,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::CYAN,
            header_bg: None,
            day_fg: Color::WHITE,
            weekend_fg: Color::rgb(150, 150, 150),
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            today_fg: Color::YELLOW,
            outside_fg: Color::rgb(80, 80, 80),
            border_color: None,
            focused: false,
        };

        // 2025-01-01 is Wednesday, ISO week 1
        assert_eq!(render_state.get_week_number(2025, 1, 1), 1);

        // 2025-01-06 is Monday, still week 1
        assert_eq!(render_state.get_week_number(2025, 1, 6), 2);

        // 2024-12-30 is Monday, week 1 of 2025
        assert_eq!(render_state.get_week_number(2024, 12, 30), 1);

        // 2024-12-28 is Saturday, week 52 of 2024
        assert_eq!(render_state.get_week_number(2024, 12, 28), 52);
    }

    #[test]
    fn test_iso_week_number_edge_cases() {
        let _cal = Calendar::new(2020, 1);

        let render_state = render::CalendarRender {
            year: 2020,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::CYAN,
            header_bg: None,
            day_fg: Color::WHITE,
            weekend_fg: Color::rgb(150, 150, 150),
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            today_fg: Color::YELLOW,
            outside_fg: Color::rgb(80, 80, 80),
            border_color: None,
            focused: false,
        };

        // 2020-01-01 is Wednesday, ISO week 1
        assert_eq!(render_state.get_week_number(2020, 1, 1), 1);

        // 2019-12-30 is Monday, week 1 of 2020
        assert_eq!(render_state.get_week_number(2019, 12, 30), 1);

        // 2020-12-31 is Thursday, week 53 of 2020
        assert_eq!(render_state.get_week_number(2020, 12, 31), 53);
    }
}

pub use date::Date;
pub use types::{CalendarMode, DateMarker, FirstDayOfWeek};
pub use utils::{days_in_month, first_day_of_month, is_leap_year};

/// Calendar widget
pub struct Calendar {
    /// Current displayed year
    year: i32,
    /// Current displayed month (1-12)
    month: u32,
    /// Selected date
    selected: Option<Date>,
    /// Selection range end (for range selection)
    range_end: Option<Date>,
    /// Display mode
    mode: CalendarMode,
    /// First day of week
    first_day: FirstDayOfWeek,
    /// Show week numbers
    show_week_numbers: bool,
    /// Date markers
    markers: Vec<DateMarker>,
    /// Today indicator
    today: Option<Date>,
    /// Colors
    header_fg: Color,
    header_bg: Option<Color>,
    day_fg: Color,
    weekend_fg: Color,
    selected_fg: Color,
    selected_bg: Color,
    today_fg: Color,
    outside_fg: Color,
    border_color: Option<Color>,
    /// Focused (interactive)
    focused: bool,
    /// Widget properties
    props: WidgetProps,
}

impl Calendar {
    /// Create a new calendar
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            selected: None,
            range_end: None,
            mode: CalendarMode::Month,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: Vec::new(),
            today: None,
            header_fg: Color::CYAN,
            header_bg: None,
            day_fg: Color::WHITE,
            weekend_fg: Color::rgb(150, 150, 150),
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            today_fg: Color::YELLOW,
            outside_fg: Color::rgb(80, 80, 80),
            border_color: None,
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set selected date
    pub fn selected(mut self, date: Date) -> Self {
        self.selected = Some(date);
        self.year = date.year;
        self.month = date.month;
        self
    }

    /// Set selection range
    pub fn range(mut self, start: Date, end: Date) -> Self {
        self.selected = Some(start);
        self.range_end = Some(end);
        self
    }

    /// Set display mode
    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set first day of week
    pub fn first_day(mut self, first: FirstDayOfWeek) -> Self {
        self.first_day = first;
        self
    }

    /// Show week numbers
    pub fn week_numbers(mut self, show: bool) -> Self {
        self.show_week_numbers = show;
        self
    }

    /// Add date marker
    pub fn marker(mut self, marker: DateMarker) -> Self {
        self.markers.push(marker);
        self
    }

    /// Add multiple markers
    pub fn markers(mut self, markers: Vec<DateMarker>) -> Self {
        self.markers.extend(markers);
        self
    }

    /// Set today's date
    pub fn today(mut self, date: Date) -> Self {
        self.today = Some(date);
        self
    }

    /// Set header color
    pub fn header_color(mut self, fg: Color) -> Self {
        self.header_fg = fg;
        self
    }

    /// Set header background
    pub fn header_bg(mut self, bg: Color) -> Self {
        self.header_bg = Some(bg);
        self
    }

    /// Set day color
    pub fn day_color(mut self, fg: Color) -> Self {
        self.day_fg = fg;
        self
    }

    /// Set weekend color
    pub fn weekend_color(mut self, fg: Color) -> Self {
        self.weekend_fg = fg;
        self
    }

    /// Set selected colors
    pub fn selected_color(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = fg;
        self.selected_bg = bg;
        self
    }

    /// Set today color
    pub fn today_color(mut self, fg: Color) -> Self {
        self.today_fg = fg;
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Navigate to previous month
    pub fn prev_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
    }

    /// Navigate to next month
    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }

    /// Navigate to previous year
    pub fn prev_year(&mut self) {
        self.year -= 1;
    }

    /// Navigate to next year
    pub fn next_year(&mut self) {
        self.year += 1;
    }

    /// Select a date
    pub fn select(&mut self, date: Date) {
        self.selected = Some(date);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
        self.range_end = None;
    }

    /// Get selected date
    pub fn get_selected(&self) -> Option<Date> {
        self.selected
    }

    /// Select next day
    pub fn select_next_day(&mut self) {
        if let Some(date) = self.selected {
            let days = days_in_month(date.year, date.month);
            if date.day < days {
                self.selected = Some(Date::new(date.year, date.month, date.day + 1));
            } else if date.month < 12 {
                self.selected = Some(Date::new(date.year, date.month + 1, 1));
                self.month = date.month + 1;
            } else {
                self.selected = Some(Date::new(date.year + 1, 1, 1));
                self.year = date.year + 1;
                self.month = 1;
            }
        } else {
            self.selected = Some(Date::new(self.year, self.month, 1));
        }
    }

    /// Select previous day
    pub fn select_prev_day(&mut self) {
        if let Some(date) = self.selected {
            if date.day > 1 {
                self.selected = Some(Date::new(date.year, date.month, date.day - 1));
            } else if date.month > 1 {
                let prev_month = date.month - 1;
                let days = days_in_month(date.year, prev_month);
                self.selected = Some(Date::new(date.year, prev_month, days));
                self.month = prev_month;
            } else {
                let days = days_in_month(date.year - 1, 12);
                self.selected = Some(Date::new(date.year - 1, 12, days));
                self.year = date.year - 1;
                self.month = 12;
            }
        } else {
            self.selected = Some(Date::new(self.year, self.month, 1));
        }
    }

    /// Select next week
    pub fn select_next_week(&mut self) {
        for _ in 0..7 {
            self.select_next_day();
        }
    }

    /// Select previous week
    pub fn select_prev_week(&mut self) {
        for _ in 0..7 {
            self.select_prev_day();
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.focused {
            return false;
        }

        match key {
            Key::Left | Key::Char('h') => {
                self.select_prev_day();
                true
            }
            Key::Right | Key::Char('l') => {
                self.select_next_day();
                true
            }
            Key::Up | Key::Char('k') => {
                self.select_prev_week();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next_week();
                true
            }
            Key::Char('[') => {
                self.prev_month();
                true
            }
            Key::Char(']') => {
                self.next_month();
                true
            }
            Key::Char('{') => {
                self.prev_year();
                true
            }
            Key::Char('}') => {
                self.next_year();
                true
            }
            _ => false,
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new(2025, 1)
    }
}

impl View for Calendar {
    crate::impl_view_meta!("Calendar");

    fn render(&self, ctx: &mut RenderContext) {
        let render_state = render::CalendarRender {
            year: self.year,
            month: self.month,
            selected: self.selected,
            range_end: self.range_end,
            first_day: self.first_day,
            show_week_numbers: self.show_week_numbers,
            markers: &self.markers,
            today: self.today,
            header_fg: self.header_fg,
            header_bg: self.header_bg,
            day_fg: self.day_fg,
            weekend_fg: self.weekend_fg,
            selected_fg: self.selected_fg,
            selected_bg: self.selected_bg,
            today_fg: self.today_fg,
            outside_fg: self.outside_fg,
            border_color: self.border_color,
            focused: self.focused,
        };

        render_state.render_month(ctx);
    }
}

impl_styled_view!(Calendar);
impl_props_builders!(Calendar);

/// Helper to create a calendar
pub fn calendar(year: i32, month: u32) -> Calendar {
    Calendar::new(year, month)
}
