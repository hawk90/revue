//! Calendar widget for date display and selection
//!
//! This module provides calendar widgets for displaying and selecting dates.
//! Supports month/year navigation, date selection, range selection, and custom styling.
//!
//! # Features
//!
//! - **Date selection** - Single date or range selection
//! - **Month/year navigation** - Navigate between months and years
//! - **Custom styling** - Style weekends, holidays, selected dates
//! - **First day of week** - Configure Sunday or Monday as first day
//! - **Date markers** - Mark special dates (holidays, events)
//! - **Week numbers** - Show ISO week numbers
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! calendar()
//!     .year(2025)
//!     .month(1)
//!     .on_select(|date| println!("Selected: {:?}", date))
//!     .width(30)
//!     .height(12);
//! ```
//!
//! # Date Range Selection
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Highlight the days from the 10th to the 20th
//! calendar(2025, 1).range(Date::new(2025, 1, 10), Date::new(2025, 1, 20));
//! ```
//!
//! # Custom Date Styling
//!
//! ```rust,ignore
//! use revue::style::Color;
//!
//! calendar()
//!     .weekend_color(Color::BLUE)
//!     .selected_color(Color::YELLOW)
//!     .today_marker("Today")
//!     .date_marker(|date| {
//!         if is_holiday(date) {
//!             Some(DateMarker::holiday("Holiday"))
//!         } else {
//!             None
//!         }
//!     });
//! ```

use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

mod date;
mod render;
mod types;
mod utils;

pub use date::Date;
#[allow(deprecated)]
pub use types::CalendarMode;
pub use types::{DateMarker, FirstDayOfWeek};
pub use utils::{days_in_month, first_day_of_month, is_leap_year};

/// Calendar widget
#[derive(Clone)]
pub struct Calendar {
    /// Current displayed year
    year: i32,
    /// Current displayed month (1-12)
    month: u32,
    /// Selected date
    selected: Option<Date>,
    /// Selection range end (for range selection)
    range_end: Option<Date>,
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
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: Vec::new(),
            today: None,
            header_fg: Color::CYAN,
            header_bg: None,
            day_fg: Color::WHITE,
            weekend_fg: LIGHT_GRAY,
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            today_fg: Color::YELLOW,
            outside_fg: DARK_GRAY,
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

    /// Set display mode (has no effect: the calendar always shows one month)
    #[deprecated(
        since = "3.5.0",
        note = "never applied: the calendar always shows one month; lay out several `Calendar`s for a year view"
    )]
    #[allow(deprecated)]
    pub fn mode(self, _mode: CalendarMode) -> Self {
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
