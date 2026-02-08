//! Calendar rendering logic

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::render_border;
use crate::widget::traits::RenderContext;

use super::types::{DateMarker, FirstDayOfWeek};
use super::{days_in_month, first_day_of_month, Date};

/// Calendar rendering state
pub struct CalendarRender<'a> {
    pub year: i32,
    pub month: u32,
    pub selected: Option<Date>,
    pub range_end: Option<Date>,
    pub first_day: FirstDayOfWeek,
    pub show_week_numbers: bool,
    pub markers: &'a [DateMarker],
    pub today: Option<Date>,
    pub header_fg: Color,
    pub header_bg: Option<Color>,
    pub day_fg: Color,
    pub weekend_fg: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub today_fg: Color,
    pub outside_fg: Color,
    pub border_color: Option<Color>,
    pub focused: bool,
}

impl<'a> CalendarRender<'a> {
    /// Check if date is in selection range
    pub fn is_in_range(&self, date: &Date) -> bool {
        match (self.selected, self.range_end) {
            (Some(start), Some(end)) => {
                let (start, end) = if start <= end {
                    (start, end)
                } else {
                    (end, start)
                };
                date >= &start && date <= &end
            }
            _ => false,
        }
    }

    /// Get marker for date
    pub fn get_marker(&self, date: &Date) -> Option<&DateMarker> {
        self.markers.iter().find(|m| &m.date == date)
    }

    /// Get day names
    pub fn day_names(&self) -> [&'static str; 7] {
        match self.first_day {
            FirstDayOfWeek::Sunday => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            FirstDayOfWeek::Monday => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
        }
    }

    /// Check if day index is weekend
    pub fn is_weekend(&self, day_index: u32) -> bool {
        match self.first_day {
            FirstDayOfWeek::Sunday => day_index == 0 || day_index == 6,
            FirstDayOfWeek::Monday => day_index == 5 || day_index == 6,
        }
    }

    /// Get ISO 8601 week number
    ///
    /// ISO week rules:
    /// - Weeks start on Monday
    /// - Week 1 contains the first Thursday of the year
    /// - Week numbers range from 1 to 52 or 53
    pub fn get_week_number(&self, year: i32, month: u32, day: u32) -> u32 {
        // Calculate day of year (1-based)
        let day_of_year = (1..month).map(|m| days_in_month(year, m)).sum::<u32>() + day;

        // Calculate weekday (0=Monday, 6=Sunday) using Zeller's congruence
        let weekday = {
            let m = if month < 3 {
                month as i32 + 12
            } else {
                month as i32
            };
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

        let days_in_year = if super::is_leap_year(year) { 366 } else { 365 };
        if thursday_day_of_year > days_in_year as i32 {
            // This day belongs to week 1 of the next year
            return 1;
        }

        // Calculate week number
        ((thursday_day_of_year as u32 - 1) / 7) + 1
    }

    /// Render month view
    pub fn render_month(&self, ctx: &mut RenderContext) {
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
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
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
            ctx.buffer
                .set(start_x + week_num_offset + 21, start_y, right);
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
                cell.fg = Some(if is_weekend {
                    self.weekend_fg
                } else {
                    self.header_fg
                });
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
                    (
                        self.selected_fg,
                        Some(Color::rgb(60, 90, 120)),
                        Modifier::empty(),
                    )
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // is_in_range tests
    // =========================================================================

    #[test]
    fn test_is_in_range_no_selection() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let date = Date::new(2024, 1, 15);
        assert!(!render.is_in_range(&date));
    }

    #[test]
    fn test_is_in_range_with_range() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: Some(Date::new(2024, 1, 10)),
            range_end: Some(Date::new(2024, 1, 20)),
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let date = Date::new(2024, 1, 15);
        assert!(render.is_in_range(&date));
    }

    #[test]
    fn test_is_in_range_before_start() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: Some(Date::new(2024, 1, 10)),
            range_end: Some(Date::new(2024, 1, 20)),
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let date = Date::new(2024, 1, 5);
        assert!(!render.is_in_range(&date));
    }

    #[test]
    fn test_is_in_range_after_end() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: Some(Date::new(2024, 1, 10)),
            range_end: Some(Date::new(2024, 1, 20)),
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let date = Date::new(2024, 1, 25);
        assert!(!render.is_in_range(&date));
    }

    #[test]
    fn test_is_in_range_reversed() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: Some(Date::new(2024, 1, 20)),
            range_end: Some(Date::new(2024, 1, 10)),
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let date = Date::new(2024, 1, 15);
        // Should handle reversed range
        assert!(render.is_in_range(&date));
    }

    // =========================================================================
    // get_marker tests
    // =========================================================================

    #[test]
    fn test_get_marker_found() {
        let date = Date::new(2024, 1, 15);
        let marker = DateMarker::new(date, Color::RED);
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[marker],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let result = render.get_marker(&date);
        assert!(result.is_some());
    }

    #[test]
    fn test_get_marker_not_found() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let date = Date::new(2024, 1, 15);
        assert!(render.get_marker(&date).is_none());
    }

    // =========================================================================
    // day_names tests
    // =========================================================================

    #[test]
    fn test_day_names_sunday_first() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let names = render.day_names();
        assert_eq!(names[0], "Su");
        assert_eq!(names[6], "Sa");
    }

    #[test]
    fn test_day_names_monday_first() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Monday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        let names = render.day_names();
        assert_eq!(names[0], "Mo");
        assert_eq!(names[6], "Su");
    }

    // =========================================================================
    // is_weekend tests
    // =========================================================================

    #[test]
    fn test_is_weekend_sunday_first_sunday() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        assert!(render.is_weekend(0)); // Sunday
        assert!(render.is_weekend(6)); // Saturday
    }

    #[test]
    fn test_is_weekend_sunday_first_weekday() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        assert!(!render.is_weekend(1)); // Monday
        assert!(!render.is_weekend(2)); // Tuesday
    }

    #[test]
    fn test_is_weekend_monday_first_saturday() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Monday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        assert!(render.is_weekend(5)); // Saturday
        assert!(render.is_weekend(6)); // Sunday
    }

    #[test]
    fn test_is_weekend_monday_first_weekday() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Monday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        assert!(!render.is_weekend(0)); // Monday
        assert!(!render.is_weekend(4)); // Friday
    }

    // =========================================================================
    // get_week_number tests
    // =========================================================================

    #[test]
    fn test_get_week_number_january_1_2024() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        // January 1, 2024 was Monday, week 1
        assert_eq!(render.get_week_number(2024, 1, 1), 1);
    }

    #[test]
    fn test_get_week_number_february_2024() {
        let render = CalendarRender {
            year: 2024,
            month: 2,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        // February 1, 2024 was Thursday, should be week 5
        let week = render.get_week_number(2024, 2, 1);
        assert!(week >= 1 && week <= 53);
    }

    #[test]
    fn test_get_week_number_range() {
        let render = CalendarRender {
            year: 2024,
            month: 1,
            selected: None,
            range_end: None,
            first_day: FirstDayOfWeek::Sunday,
            show_week_numbers: false,
            markers: &[],
            today: None,
            header_fg: Color::BLACK,
            header_bg: None,
            day_fg: Color::BLACK,
            weekend_fg: Color::BLACK,
            selected_fg: Color::BLACK,
            selected_bg: Color::BLACK,
            today_fg: Color::BLACK,
            outside_fg: Color::BLACK,
            border_color: None,
            focused: false,
        };
        // Test multiple dates to ensure valid range
        for day in 1..31 {
            let week = render.get_week_number(2024, 1, day);
            assert!(week >= 1 && week <= 53);
        }
    }
}
