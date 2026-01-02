//! Calendar widget for date display and selection
//!
//! Supports month/year navigation, date selection, range selection,
//! and custom styling for different date types.

use super::traits::{View, RenderContext, WidgetProps};
use crate::{impl_styled_view, impl_props_builders};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::render_border;

/// Days in a month (accounting for leap years)
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Check if year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Day of week for the first day of a month (0 = Sunday, 6 = Saturday)
/// Using Zeller's congruence
fn first_day_of_month(year: i32, month: u32) -> u32 {
    let m = if month < 3 { month as i32 + 12 } else { month as i32 };
    let y = if month < 3 { year - 1 } else { year };
    let q = 1i32; // First day of month
    let k = y % 100;
    let j = y / 100;

    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    // Convert from Zeller (0 = Saturday) to standard (0 = Sunday)
    // Handle negative modulo
    let h = ((h + 6) % 7 + 7) % 7;
    h as u32
}

/// Calendar date
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
}

impl Date {
    /// Create a new date
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Get today's date (placeholder - returns 2025-01-01)
    pub fn today() -> Self {
        // In a real implementation, use chrono or time crate
        Self::new(2025, 1, 1)
    }

    /// Check if date is valid
    pub fn is_valid(&self) -> bool {
        self.month >= 1 && self.month <= 12 && self.day >= 1 && self.day <= days_in_month(self.year, self.month)
    }

    /// Day of week (0 = Sunday, 6 = Saturday)
    pub fn weekday(&self) -> u32 {
        let first = first_day_of_month(self.year, self.month);
        (first + self.day - 1) % 7
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::new(2025, 1, 1)
    }
}

/// Date marker for highlighting specific dates
#[derive(Clone, Debug)]
pub struct DateMarker {
    /// Date to mark
    pub date: Date,
    /// Marker color
    pub color: Color,
    /// Optional symbol
    pub symbol: Option<char>,
}

impl DateMarker {
    /// Create a new marker
    pub fn new(date: Date, color: Color) -> Self {
        Self {
            date,
            color,
            symbol: None,
        }
    }

    /// Set symbol
    pub fn symbol(mut self, symbol: char) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

/// Calendar display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    /// Single month view
    #[default]
    Month,
    /// Year overview (12 months)
    Year,
    /// Week view
    Week,
}

/// First day of week
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FirstDayOfWeek {
    /// Sunday first (US style)
    #[default]
    Sunday,
    /// Monday first (ISO style)
    Monday,
}

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

    /// Check if date is in selection range
    fn is_in_range(&self, date: &Date) -> bool {
        match (self.selected, self.range_end) {
            (Some(start), Some(end)) => {
                let (start, end) = if start <= end { (start, end) } else { (end, start) };
                date >= &start && date <= &end
            }
            _ => false,
        }
    }

    /// Get marker for date
    fn get_marker(&self, date: &Date) -> Option<&DateMarker> {
        self.markers.iter().find(|m| &m.date == date)
    }

    /// Get day names
    fn day_names(&self) -> [&'static str; 7] {
        match self.first_day {
            FirstDayOfWeek::Sunday => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            FirstDayOfWeek::Monday => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
        }
    }

    /// Check if day index is weekend
    fn is_weekend(&self, day_index: u32) -> bool {
        match self.first_day {
            FirstDayOfWeek::Sunday => day_index == 0 || day_index == 6,
            FirstDayOfWeek::Monday => day_index == 5 || day_index == 6,
        }
    }

    /// Render month view
    fn render_month(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 20 || area.height < 8 {
            return;
        }

        let has_border = self.border_color.is_some();
        let start_x = area.x + if has_border { 1 } else { 0 };
        let start_y = area.y + if has_border { 1 } else { 0 };
        let week_num_offset: u16 = if self.show_week_numbers { 4 } else { 0 };

        // Draw border if specified
        if let Some(border_color) = self.border_color {
            render_border(ctx, area, border_color);
        }

        // Month name and year header
        let month_names = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        let header = format!("{} {}", month_names[(self.month - 1) as usize], self.year);
        let header_x = start_x + week_num_offset + (20 - header.len() as u16) / 2;

        for (i, ch) in header.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(self.header_fg);
            cell.bg = self.header_bg;
            cell.modifier |= Modifier::BOLD;
            ctx.buffer.set(header_x + i as u16, start_y, cell);
        }

        // Navigation arrows
        if self.focused {
            let mut left = Cell::new('◀');
            left.fg = Some(self.header_fg);
            ctx.buffer.set(start_x + week_num_offset, start_y, left);

            let mut right = Cell::new('▶');
            right.fg = Some(self.header_fg);
            ctx.buffer.set(start_x + week_num_offset + 21, start_y, right);
        }

        // Week header
        let y = start_y + 2;
        let day_names = self.day_names();

        if self.show_week_numbers {
            let mut wk = Cell::new('W');
            wk.fg = Some(self.header_fg);
            ctx.buffer.set(start_x, y, wk);
        }

        for (i, name) in day_names.iter().enumerate() {
            let x = start_x + week_num_offset + (i as u16) * 3;
            let is_weekend = self.is_weekend(i as u32);

            for (j, ch) in name.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(if is_weekend { self.weekend_fg } else { self.header_fg });
                ctx.buffer.set(x + j as u16, y, cell);
            }
        }

        // Days
        let first_day = first_day_of_month(self.year, self.month);
        let first_day_adjusted = match self.first_day {
            FirstDayOfWeek::Sunday => first_day,
            FirstDayOfWeek::Monday => (first_day + 6) % 7,
        };
        let days = days_in_month(self.year, self.month);

        let mut day = 1u32;
        let mut row = 0u32;

        while day <= days {
            let y = start_y + 3 + row as u16;

            // Week number
            if self.show_week_numbers {
                let week_num = self.get_week_number(self.year, self.month, day);
                let week_str = format!("{:2}", week_num);
                for (i, ch) in week_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.outside_fg);
                    ctx.buffer.set(start_x + i as u16, y, cell);
                }
            }

            for col in 0..7u32 {
                let cell_day = if row == 0 {
                    if col < first_day_adjusted {
                        continue;
                    }
                    col - first_day_adjusted + 1
                } else {
                    row * 7 + col - first_day_adjusted + 1
                };

                if cell_day < 1 || cell_day > days {
                    continue;
                }

                let x = start_x + week_num_offset + col as u16 * 3;
                let date = Date::new(self.year, self.month, cell_day);

                // Determine styling
                let is_selected = self.selected == Some(date);
                let is_in_range = self.is_in_range(&date);
                let is_today = self.today == Some(date);
                let is_weekend = self.is_weekend(col);
                let marker = self.get_marker(&date);

                let (fg, bg, modifier) = if is_selected {
                    (self.selected_fg, Some(self.selected_bg), Modifier::BOLD)
                } else if is_in_range {
                    (self.selected_fg, Some(Color::rgb(60, 90, 120)), Modifier::empty())
                } else if is_today {
                    (self.today_fg, None, Modifier::BOLD)
                } else if let Some(m) = marker {
                    (m.color, None, Modifier::empty())
                } else if is_weekend {
                    (self.weekend_fg, None, Modifier::empty())
                } else {
                    (self.day_fg, None, Modifier::empty())
                };

                // Draw day number
                let day_str = format!("{:2}", cell_day);
                for (i, ch) in day_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = bg;
                    cell.modifier = modifier;
                    ctx.buffer.set(x + i as u16, y, cell);
                }

                // Draw marker symbol
                if let Some(m) = marker {
                    if let Some(sym) = m.symbol {
                        let mut cell = Cell::new(sym);
                        cell.fg = Some(m.color);
                        ctx.buffer.set(x + 2, y, cell);
                    }
                }
            }

            if row == 0 {
                day = 8 - first_day_adjusted;
            } else {
                day += 7;
            }
            row += 1;
        }
    }

    /// Get ISO 8601 week number
    ///
    /// ISO week rules:
    /// - Weeks start on Monday
    /// - Week 1 contains the first Thursday of the year
    /// - Week numbers range from 1 to 52 or 53
    fn get_week_number(&self, year: i32, month: u32, day: u32) -> u32 {
        // Calculate day of year (1-based)
        let day_of_year = (1..month).map(|m| days_in_month(year, m)).sum::<u32>() + day;

        // Calculate weekday (0=Monday, 6=Sunday) using Zeller's congruence
        let weekday = {
            let m = if month < 3 { month as i32 + 12 } else { month as i32 };
            let y = if month < 3 { year - 1 } else { year };
            let k = y % 100;
            let j = y / 100;
            let h = (day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
            // Convert from Zeller (0=Sat) to ISO (0=Mon)
            ((h + 5) % 7) as u32
        };

        // Calculate ISO week number
        // Thursday of the same week determines the year for ISO week
        let thursday_day_of_year = day_of_year as i32 + 3 - weekday as i32;

        if thursday_day_of_year < 1 {
            // This day belongs to the last week of the previous year
            return self.get_week_number(year - 1, 12, 31);
        }

        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if thursday_day_of_year > days_in_year as i32 {
            // This day belongs to week 1 of the next year
            return 1;
        }

        // Calculate week number
        ((thursday_day_of_year as u32 - 1) / 7) + 1
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
        match self.mode {
            CalendarMode::Month => self.render_month(ctx),
            CalendarMode::Year | CalendarMode::Week => {
                // Simplified: just render month view for now
                self.render_month(ctx);
            }
        }
    }
}

impl_styled_view!(Calendar);
impl_props_builders!(Calendar);

/// Helper to create a calendar
pub fn calendar(year: i32, month: u32) -> Calendar {
    Calendar::new(year, month)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;
    

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
        let marker = DateMarker::new(Date::new(2025, 1, 1), Color::RED)
            .symbol('★');

        assert_eq!(marker.date, Date::new(2025, 1, 1));
        assert_eq!(marker.color, Color::RED);
        assert_eq!(marker.symbol, Some('★'));
    }

    #[test]
    fn test_calendar_range() {
        let cal = Calendar::new(2025, 1)
            .range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));

        assert!(cal.is_in_range(&Date::new(2025, 1, 15)));
        assert!(!cal.is_in_range(&Date::new(2025, 1, 5)));
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
        let cal_sun = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Sunday);
        let cal_mon = Calendar::new(2025, 1).first_day(FirstDayOfWeek::Monday);

        assert_eq!(cal_sun.day_names()[0], "Su");
        assert_eq!(cal_mon.day_names()[0], "Mo");
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
        let cal = Calendar::new(2025, 1);

        // 2025-01-01 is Wednesday, ISO week 1
        assert_eq!(cal.get_week_number(2025, 1, 1), 1);

        // 2025-01-06 is Monday, still week 1
        assert_eq!(cal.get_week_number(2025, 1, 6), 2);

        // 2024-12-30 is Monday, week 1 of 2025
        assert_eq!(cal.get_week_number(2024, 12, 30), 1);

        // 2024-12-28 is Saturday, week 52 of 2024
        assert_eq!(cal.get_week_number(2024, 12, 28), 52);
    }

    #[test]
    fn test_iso_week_number_edge_cases() {
        let cal = Calendar::new(2020, 1);

        // 2020-01-01 is Wednesday, ISO week 1
        assert_eq!(cal.get_week_number(2020, 1, 1), 1);

        // 2019-12-30 is Monday, week 1 of 2020
        assert_eq!(cal.get_week_number(2019, 12, 30), 1);

        // 2020-12-31 is Thursday, week 53 of 2020
        assert_eq!(cal.get_week_number(2020, 12, 31), 53);
    }
}
