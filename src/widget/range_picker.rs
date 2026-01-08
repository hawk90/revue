//! Date/time range picker widget
//!
//! Provides a widget for selecting date ranges with:
//! - Start and end date selection
//! - Common preset ranges (Today, Last 7 Days, etc.)
//! - Optional time selection
//! - Validation to ensure end >= start

use super::calendar::{days_in_month, Date, FirstDayOfWeek};
use super::datetime_picker::{DateTime, Time};
use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_state_builders, impl_styled_view};
use unicode_width::UnicodeWidthChar;

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

/// A date/time range picker widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{range_picker, RangePicker, Date};
///
/// // Basic date range picker
/// let picker = range_picker()
///     .start_date(Date::new(2025, 1, 1))
///     .end_date(Date::new(2025, 1, 31));
///
/// // With presets
/// let picker = range_picker()
///     .with_presets(true);
///
/// // Analytics-style range picker
/// let picker = analytics_range_picker();
/// ```
pub struct RangePicker {
    /// Start datetime
    start: DateTime,
    /// End datetime
    end: DateTime,
    /// Currently active preset
    active_preset: Option<PresetRange>,
    /// Available presets
    presets: Vec<PresetRange>,
    /// Cursor position in presets
    preset_cursor: usize,
    /// Current focus area
    focus: RangeFocus,
    /// First day of week
    first_day: FirstDayOfWeek,
    /// Show time selection
    show_time: bool,
    /// Calendar cursor day (for start)
    start_cursor_day: u32,
    /// Calendar cursor day (for end)
    end_cursor_day: u32,
    /// Minimum allowed date
    min_date: Option<Date>,
    /// Maximum allowed date
    max_date: Option<Date>,
    /// Show presets panel
    show_presets: bool,
    /// Colors
    header_fg: Color,
    selected_fg: Color,
    selected_bg: Color,
    range_bg: Color,
    preset_fg: Color,
    preset_selected_fg: Color,
    preset_selected_bg: Color,
    /// Widget state
    state: WidgetState,
    /// Widget props
    props: WidgetProps,
}

impl RangePicker {
    /// Create a new range picker
    pub fn new() -> Self {
        let today = Date::today();
        Self {
            start: DateTime::new(today, Time::new(0, 0, 0)),
            end: DateTime::new(today, Time::new(23, 59, 59)),
            active_preset: Some(PresetRange::Today),
            presets: PresetRange::common().to_vec(),
            preset_cursor: 0,
            focus: RangeFocus::Start,
            first_day: FirstDayOfWeek::Sunday,
            show_time: false,
            start_cursor_day: today.day,
            end_cursor_day: today.day,
            min_date: None,
            max_date: None,
            show_presets: true,
            header_fg: Color::CYAN,
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            range_bg: Color::rgb(60, 100, 140),
            preset_fg: Color::WHITE,
            preset_selected_fg: Color::BLACK,
            preset_selected_bg: Color::CYAN,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

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
    fn swap_if_needed(&mut self) {
        if self.end.date < self.start.date {
            std::mem::swap(&mut self.start.date, &mut self.end.date);
            std::mem::swap(&mut self.start_cursor_day, &mut self.end_cursor_day);
        }
    }

    // =========================================================================
    // Navigation
    // =========================================================================

    /// Move to next focus area
    fn next_focus(&mut self) {
        self.focus = match self.focus {
            RangeFocus::Start => RangeFocus::End,
            RangeFocus::End => {
                if self.show_presets {
                    RangeFocus::Presets
                } else {
                    RangeFocus::Start
                }
            }
            RangeFocus::Presets => RangeFocus::Start,
        };
    }

    /// Move to previous focus area
    fn prev_focus(&mut self) {
        self.focus = match self.focus {
            RangeFocus::Start => {
                if self.show_presets {
                    RangeFocus::Presets
                } else {
                    RangeFocus::End
                }
            }
            RangeFocus::End => RangeFocus::Start,
            RangeFocus::Presets => RangeFocus::End,
        };
    }

    /// Get mutable reference to current calendar date/cursor
    fn current_date_mut(&mut self) -> (&mut Date, &mut u32) {
        match self.focus {
            RangeFocus::Start => (&mut self.start.date, &mut self.start_cursor_day),
            RangeFocus::End | RangeFocus::Presets => (&mut self.end.date, &mut self.end_cursor_day),
        }
    }

    /// Move cursor left (previous day)
    fn move_day_left(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if *cursor > 1 {
            *cursor -= 1;
        } else {
            // Previous month
            if date.month > 1 {
                date.month -= 1;
            } else {
                date.month = 12;
                date.year -= 1;
            }
            *cursor = days_in_month(date.year, date.month);
        }
    }

    /// Move cursor right (next day)
    fn move_day_right(&mut self) {
        let (date, cursor) = self.current_date_mut();
        let max_day = days_in_month(date.year, date.month);
        if *cursor < max_day {
            *cursor += 1;
        } else {
            // Next month
            if date.month < 12 {
                date.month += 1;
            } else {
                date.month = 1;
                date.year += 1;
            }
            *cursor = 1;
        }
    }

    /// Move cursor up (previous week)
    fn move_week_up(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if *cursor > 7 {
            *cursor -= 7;
        } else {
            // Previous month
            if date.month > 1 {
                date.month -= 1;
            } else {
                date.month = 12;
                date.year -= 1;
            }
            let max_day = days_in_month(date.year, date.month);
            *cursor = max_day - (7 - *cursor);
        }
    }

    /// Move cursor down (next week)
    fn move_week_down(&mut self) {
        let (date, cursor) = self.current_date_mut();
        let max_day = days_in_month(date.year, date.month);
        if *cursor + 7 <= max_day {
            *cursor += 7;
        } else {
            let overflow = *cursor + 7 - max_day;
            if date.month < 12 {
                date.month += 1;
            } else {
                date.month = 1;
                date.year += 1;
            }
            *cursor = overflow;
        }
    }

    /// Go to previous month
    fn prev_month(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if date.month > 1 {
            date.month -= 1;
        } else {
            date.month = 12;
            date.year -= 1;
        }
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Go to next month
    fn next_month(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if date.month < 12 {
            date.month += 1;
        } else {
            date.month = 1;
            date.year += 1;
        }
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Go to previous year
    fn prev_year(&mut self) {
        let (date, cursor) = self.current_date_mut();
        date.year -= 1;
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Go to next year
    fn next_year(&mut self) {
        let (date, cursor) = self.current_date_mut();
        date.year += 1;
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Select current cursor position
    fn select_date(&mut self) {
        match self.focus {
            RangeFocus::Start => {
                self.start.date.day = self.start_cursor_day;
                self.active_preset = Some(PresetRange::Custom);
            }
            RangeFocus::End => {
                self.end.date.day = self.end_cursor_day;
                self.active_preset = Some(PresetRange::Custom);
            }
            RangeFocus::Presets => {
                if let Some(preset) = self.presets.get(self.preset_cursor) {
                    self.apply_preset(*preset);
                }
            }
        }
        self.swap_if_needed();
    }

    /// Move preset cursor up
    fn preset_up(&mut self) {
        if !self.presets.is_empty() {
            self.preset_cursor = self
                .preset_cursor
                .checked_sub(1)
                .unwrap_or(self.presets.len() - 1);
        }
    }

    /// Move preset cursor down
    fn preset_down(&mut self) {
        if !self.presets.is_empty() {
            self.preset_cursor = (self.preset_cursor + 1) % self.presets.len();
        }
    }

    // =========================================================================
    // Key handling
    // =========================================================================

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            // Switch focus areas
            Key::Tab => {
                self.next_focus();
                true
            }
            Key::BackTab => {
                self.prev_focus();
                true
            }

            // Navigation
            Key::Left | Key::Char('h') if self.focus != RangeFocus::Presets => {
                self.move_day_left();
                true
            }
            Key::Right | Key::Char('l') if self.focus != RangeFocus::Presets => {
                self.move_day_right();
                true
            }
            Key::Up | Key::Char('k') if self.focus != RangeFocus::Presets => {
                self.move_week_up();
                true
            }
            Key::Down | Key::Char('j') if self.focus != RangeFocus::Presets => {
                self.move_week_down();
                true
            }

            // Preset navigation
            Key::Up | Key::Char('k') if self.focus == RangeFocus::Presets => {
                self.preset_up();
                true
            }
            Key::Down | Key::Char('j') if self.focus == RangeFocus::Presets => {
                self.preset_down();
                true
            }

            // Month/Year navigation
            Key::Char('[') if self.focus != RangeFocus::Presets => {
                self.prev_month();
                true
            }
            Key::Char(']') if self.focus != RangeFocus::Presets => {
                self.next_month();
                true
            }
            Key::Char('{') if self.focus != RangeFocus::Presets => {
                self.prev_year();
                true
            }
            Key::Char('}') if self.focus != RangeFocus::Presets => {
                self.next_year();
                true
            }

            // Selection
            Key::Enter | Key::Char(' ') => {
                self.select_date();
                true
            }

            _ => false,
        }
    }

    // =========================================================================
    // Rendering helpers
    // =========================================================================

    /// Get day of week for first day of month
    fn first_weekday(&self, date: &Date) -> u32 {
        let m = if date.month < 3 {
            date.month as i32 + 12
        } else {
            date.month as i32
        };
        let y = if date.month < 3 {
            date.year - 1
        } else {
            date.year
        };
        let k = y % 100;
        let j = y / 100;
        let h = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        let h = ((h + 6) % 7 + 7) % 7;

        match self.first_day {
            FirstDayOfWeek::Sunday => h as u32,
            FirstDayOfWeek::Monday => ((h + 6) % 7) as u32,
        }
    }

    /// Render a calendar
    #[allow(clippy::too_many_arguments)]
    fn render_calendar(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        date: &Date,
        cursor_day: u32,
        is_start: bool,
        is_focused: bool,
    ) {
        let days = days_in_month(date.year, date.month);
        let first_weekday = self.first_weekday(date);

        // Header
        let title = if is_start { "Start" } else { "End" };
        let header = format!("{}: {} {}", title, month_name(date.month), date.year);
        let header_color = if is_focused {
            self.header_fg
        } else {
            Color::rgb(100, 100, 100)
        };
        self.draw_text(ctx, x, y, &header, header_color, true);

        // Day headers
        let day_headers = match self.first_day {
            FirstDayOfWeek::Sunday => "Su Mo Tu We Th Fr Sa",
            FirstDayOfWeek::Monday => "Mo Tu We Th Fr Sa Su",
        };
        self.draw_text(ctx, x, y + 1, day_headers, Color::rgb(150, 150, 150), false);

        // Days
        let mut row = 0u16;
        let mut col = first_weekday as u16;

        let selected_day = if is_start {
            self.start.date.day
        } else {
            self.end.date.day
        };

        for day in 1..=days {
            let day_x = x + col * 3;
            let day_y = y + 2 + row;
            let day_str = format!("{:2}", day);

            let check_date = Date::new(date.year, date.month, day);
            let in_range = self.is_in_range(&check_date);
            let is_selected = day == selected_day;
            let is_cursor = day == cursor_day && is_focused;

            let (fg, bg, bold) = if is_cursor {
                (Color::BLACK, Some(Color::WHITE), true)
            } else if is_selected {
                (self.selected_fg, Some(self.selected_bg), true)
            } else if in_range {
                (Color::WHITE, Some(self.range_bg), false)
            } else {
                (Color::WHITE, None, false)
            };

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

    /// Render presets list
    fn render_presets(&self, ctx: &mut RenderContext, x: u16, y: u16, is_focused: bool) {
        let title_color = if is_focused {
            self.header_fg
        } else {
            Color::rgb(100, 100, 100)
        };
        self.draw_text(ctx, x, y, "Presets", title_color, true);

        for (i, preset) in self.presets.iter().enumerate() {
            let preset_y = y + 1 + i as u16;
            let is_cursor = i == self.preset_cursor && is_focused;
            let is_active = self.active_preset == Some(*preset);

            let (fg, bg) = if is_cursor {
                (self.preset_selected_fg, Some(self.preset_selected_bg))
            } else if is_active {
                (Color::CYAN, None)
            } else {
                (self.preset_fg, None)
            };

            let marker = if is_active { "● " } else { "  " };
            let text = format!("{}{}", marker, preset.name());

            if let Some(bg_color) = bg {
                for dx in 0..16 {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg_color);
                    ctx.buffer.set(x + dx, preset_y, cell);
                }
            }
            self.draw_text(ctx, x, preset_y, &text, fg, is_cursor);
        }
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

impl Default for RangePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl View for RangePicker {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 50 || area.height < 10 {
            return;
        }

        let x = area.x;
        let y = area.y;

        // Render start calendar
        let start_focused = self.focus == RangeFocus::Start;
        self.render_calendar(
            ctx,
            x,
            y,
            &self.start.date,
            self.start_cursor_day,
            true,
            start_focused,
        );

        // Render end calendar (next to start)
        let end_x = x + 24;
        let end_focused = self.focus == RangeFocus::End;
        self.render_calendar(
            ctx,
            end_x,
            y,
            &self.end.date,
            self.end_cursor_day,
            false,
            end_focused,
        );

        // Render presets if enabled
        if self.show_presets {
            let presets_x = end_x + 24;
            let presets_focused = self.focus == RangeFocus::Presets;
            self.render_presets(ctx, presets_x, y, presets_focused);
        }

        // Render selected range summary
        let summary_y = y + 9;
        let range_str = format!(
            "Range: {}-{:02}-{:02} to {}-{:02}-{:02}",
            self.start.date.year,
            self.start.date.month,
            self.start.date.day,
            self.end.date.year,
            self.end.date.month,
            self.end.date.day,
        );
        self.draw_text(
            ctx,
            x,
            summary_y,
            &range_str,
            Color::rgb(200, 200, 200),
            false,
        );

        // Help text
        let help = "Tab: switch | ←→↑↓: navigate | [/]: month | Enter: select";
        self.draw_text(
            ctx,
            x,
            summary_y + 1,
            help,
            Color::rgb(100, 100, 100),
            false,
        );
    }
}

impl_styled_view!(RangePicker);
impl_state_builders!(RangePicker);
impl_props_builders!(RangePicker);

/// Helper function to get month name
fn month_name(month: u32) -> &'static str {
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

// =============================================================================
// Helper functions
// =============================================================================

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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_range_picker_new() {
        let picker = RangePicker::new();
        let (start, end) = picker.get_range();
        assert_eq!(start, end); // Default is today-today
    }

    #[test]
    fn test_range_picker_set_range() {
        let picker = range_picker()
            .start_date(Date::new(2025, 1, 1))
            .end_date(Date::new(2025, 1, 31));

        let (start, end) = picker.get_range();
        assert_eq!(start, Date::new(2025, 1, 1));
        assert_eq!(end, Date::new(2025, 1, 31));
    }

    #[test]
    fn test_range_picker_swap() {
        let picker = range_picker()
            .start_date(Date::new(2025, 12, 31))
            .end_date(Date::new(2025, 1, 1));

        let (start, end) = picker.get_range();
        assert!(start <= end);
    }

    #[test]
    fn test_range_picker_is_in_range() {
        let picker = range_picker()
            .start_date(Date::new(2025, 1, 10))
            .end_date(Date::new(2025, 1, 20));

        assert!(picker.is_in_range(&Date::new(2025, 1, 15)));
        assert!(picker.is_in_range(&Date::new(2025, 1, 10)));
        assert!(picker.is_in_range(&Date::new(2025, 1, 20)));
        assert!(!picker.is_in_range(&Date::new(2025, 1, 5)));
        assert!(!picker.is_in_range(&Date::new(2025, 1, 25)));
    }

    #[test]
    fn test_range_picker_focus_navigation() {
        let mut picker = range_picker();

        assert_eq!(picker.focus, RangeFocus::Start);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.focus, RangeFocus::End);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.focus, RangeFocus::Presets);

        picker.handle_key(&Key::Tab);
        assert_eq!(picker.focus, RangeFocus::Start);

        picker.handle_key(&Key::BackTab);
        assert_eq!(picker.focus, RangeFocus::Presets);
    }

    #[test]
    fn test_range_picker_preset_apply() {
        let mut picker = range_picker();
        let today = Date::today();

        picker.apply_preset(PresetRange::Today);
        let (start, end) = picker.get_range();
        assert_eq!(start, today);
        assert_eq!(end, today);
    }

    #[test]
    fn test_range_picker_preset_last7days() {
        let today = Date::today();
        let (start, end) = PresetRange::Last7Days.calculate(today);

        assert_eq!(end, today);
        // Start should be 6 days before today
        let expected_start = today.subtract_days(6);
        assert_eq!(start, expected_start);
    }

    #[test]
    fn test_range_picker_preset_this_month() {
        let today = Date::today();
        let (start, end) = PresetRange::ThisMonth.calculate(today);

        assert_eq!(start.day, 1);
        assert_eq!(start.month, today.month);
        assert_eq!(end, today);
    }

    #[test]
    fn test_range_picker_key_navigation() {
        let mut picker = range_picker().start_date(Date::new(2025, 6, 15));

        picker.handle_key(&Key::Right);
        assert_eq!(picker.start_cursor_day, 16);

        picker.handle_key(&Key::Left);
        assert_eq!(picker.start_cursor_day, 15);

        picker.handle_key(&Key::Down);
        assert_eq!(picker.start_cursor_day, 22);

        picker.handle_key(&Key::Up);
        assert_eq!(picker.start_cursor_day, 15);
    }

    #[test]
    fn test_range_picker_month_navigation() {
        let mut picker = range_picker().start_date(Date::new(2025, 6, 15));

        picker.handle_key(&Key::Char(']'));
        assert_eq!(picker.start.date.month, 7);

        picker.handle_key(&Key::Char('['));
        assert_eq!(picker.start.date.month, 6);
    }

    #[test]
    fn test_range_picker_year_navigation() {
        let mut picker = range_picker().start_date(Date::new(2025, 6, 15));

        picker.handle_key(&Key::Char('}'));
        assert_eq!(picker.start.date.year, 2026);

        picker.handle_key(&Key::Char('{'));
        assert_eq!(picker.start.date.year, 2025);
    }

    #[test]
    fn test_range_picker_select() {
        // Create a picker with a range that gives us room to select
        let mut picker = range_picker()
            .start_date(Date::new(2025, 1, 10))
            .end_date(Date::new(2025, 1, 20));

        assert_eq!(picker.start.date.day, 10);
        assert_eq!(picker.start_cursor_day, 10);

        // Navigate cursor to day 15
        for _ in 0..5 {
            picker.handle_key(&Key::Right);
        }
        assert_eq!(picker.start_cursor_day, 15);

        // Select day 15
        picker.handle_key(&Key::Enter);
        assert_eq!(picker.start.date.day, 15);
        assert_eq!(picker.end.date.day, 20); // End unchanged
    }

    #[test]
    fn test_range_picker_preset_navigation() {
        let mut picker = range_picker();
        picker.focus = RangeFocus::Presets;

        assert_eq!(picker.preset_cursor, 0);

        picker.handle_key(&Key::Down);
        assert_eq!(picker.preset_cursor, 1);

        picker.handle_key(&Key::Up);
        assert_eq!(picker.preset_cursor, 0);
    }

    #[test]
    fn test_range_picker_preset_select() {
        let mut picker = range_picker();
        picker.focus = RangeFocus::Presets;
        picker.preset_cursor = 2; // Last7Days

        picker.handle_key(&Key::Enter);
        assert_eq!(picker.active_preset, Some(PresetRange::Last7Days));
    }

    #[test]
    fn test_range_picker_helper_functions() {
        let _ = range_picker();
        let _ = date_range_picker();
        let _ = analytics_range_picker();
    }

    #[test]
    fn test_range_picker_render() {
        let mut buffer = Buffer::new(80, 15);
        let area = Rect::new(0, 0, 80, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let picker = range_picker()
            .start_date(Date::new(2025, 1, 1))
            .end_date(Date::new(2025, 1, 31))
            .focused(true);

        picker.render(&mut ctx);
        // Verify rendering completed without panic
    }

    #[test]
    fn test_preset_names() {
        assert_eq!(PresetRange::Today.name(), "Today");
        assert_eq!(PresetRange::Last7Days.name(), "Last 7 Days");
        assert_eq!(PresetRange::Custom.name(), "Custom");
    }

    #[test]
    fn test_preset_common() {
        let common = PresetRange::common();
        assert!(common.contains(&PresetRange::Today));
        assert!(common.contains(&PresetRange::Last7Days));
        assert!(!common.contains(&PresetRange::Custom));
    }

    #[test]
    fn test_range_picker_disabled() {
        let mut picker = range_picker().disabled(true);

        let handled = picker.handle_key(&Key::Right);
        assert!(!handled);
    }
}
