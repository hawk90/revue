//! DateTime picker rendering

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::data::calendar::{days_in_month, FirstDayOfWeek};
use crate::widget::traits::RenderContext;
use unicode_width::UnicodeWidthChar;

/// Rendering methods for DateTimePicker
pub trait Rendering {
    // Required getters
    fn date(&self) -> crate::widget::data::calendar::Date;
    fn time(&self) -> super::types::Time;
    fn mode(&self) -> super::types::DateTimeMode;
    fn first_day(&self) -> crate::widget::data::calendar::FirstDayOfWeek;
    fn cursor_day(&self) -> u32;
    fn time_field(&self) -> super::types::TimeField;
    fn show_seconds(&self) -> bool;

    // Colors
    fn header_fg(&self) -> Color;
    fn selected_fg(&self) -> Color;
    fn selected_bg(&self) -> Color;
    fn weekend_fg(&self) -> Color;
    fn cursor_fg(&self) -> Color;
    fn cursor_bg(&self) -> Color;
    fn field_fg(&self) -> Color;
    fn field_active_fg(&self) -> Color;
    fn field_active_bg(&self) -> Color;

    /// Get day of week for first day of current month
    fn first_weekday(&self) -> u32 {
        let date = self.date();
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

        // Adjust for first day of week
        match self.first_day() {
            FirstDayOfWeek::Sunday => h as u32,
            FirstDayOfWeek::Monday => ((h + 6) % 7) as u32,
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

    /// Render calendar portion
    fn render_calendar(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        let days_in_month = days_in_month(self.date().year, self.date().month);
        let first_weekday = self.first_weekday();

        // Header: month and year
        let header = format!(
            "{} {}",
            super::helpers::month_name(self.date().month),
            self.date().year
        );
        let header_x = x + (width.saturating_sub(header.len() as u16)) / 2;
        self.draw_text(ctx, header_x, y, &header, self.header_fg(), true);

        // Day headers
        let day_headers = match self.first_day() {
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

            let is_selected = day == self.date().day;
            let is_cursor =
                day == self.cursor_day() && self.mode() == super::types::DateTimeMode::Date;
            let is_weekend = match self.first_day() {
                FirstDayOfWeek::Sunday => col == 0 || col == 6,
                FirstDayOfWeek::Monday => col == 5 || col == 6,
            };

            let (fg, bg, bold) = if is_cursor {
                (self.cursor_fg(), Some(self.cursor_bg()), true)
            } else if is_selected {
                (self.selected_fg(), Some(self.selected_bg()), true)
            } else if is_weekend {
                (self.weekend_fg(), None, false)
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
        let hour_str = format!("{:02}", self.time().hour);
        let (hour_fg, hour_bg) = if self.mode() == super::types::DateTimeMode::Time
            && self.time_field() == super::types::TimeField::Hour
        {
            (self.field_active_fg(), Some(self.field_active_bg()))
        } else {
            (self.field_fg(), None)
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
        self.draw_text(ctx, field_x, field_y, ":", self.field_fg(), false);
        field_x += 1;

        // Minute
        let minute_str = format!("{:02}", self.time().minute);
        let (minute_fg, minute_bg) = if self.mode() == super::types::DateTimeMode::Time
            && self.time_field() == super::types::TimeField::Minute
        {
            (self.field_active_fg(), Some(self.field_active_bg()))
        } else {
            (self.field_fg(), None)
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
        if self.show_seconds() {
            self.draw_text(ctx, field_x, field_y, ":", self.field_fg(), false);
            field_x += 1;

            let second_str = format!("{:02}", self.time().second);
            let (second_fg, second_bg) = if self.mode() == super::types::DateTimeMode::Time
                && self.time_field() == super::types::TimeField::Second
            {
                (self.field_active_fg(), Some(self.field_active_bg()))
            } else {
                (self.field_fg(), None)
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
        let help = if self.mode() == super::types::DateTimeMode::Time {
            "↑↓: change  ←→: field  Tab: date"
        } else {
            "Tab: switch to time"
        };
        self.draw_text(ctx, x, y + 3, help, Color::rgb(100, 100, 100), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::calendar::Date;
    use crate::widget::datetime_picker::types::{DateTimeMode, Time, TimeField};

    // Mock struct for testing Rendering trait
    struct MockRenderingPicker {
        date: Date,
        time: Time,
        mode: DateTimeMode,
        first_day: FirstDayOfWeek,
        cursor_day: u32,
        time_field: TimeField,
        show_seconds: bool,
        header_fg: Color,
        selected_fg: Color,
        selected_bg: Color,
        weekend_fg: Color,
        cursor_fg: Color,
        cursor_bg: Color,
        field_fg: Color,
        field_active_fg: Color,
        field_active_bg: Color,
    }

    impl MockRenderingPicker {
        fn new() -> Self {
            Self {
                date: Date::new(2024, 1, 15),
                time: Time::new(10, 30, 45),
                mode: DateTimeMode::Date,
                first_day: FirstDayOfWeek::Sunday,
                cursor_day: 15,
                time_field: TimeField::Hour,
                show_seconds: true,
                header_fg: Color::CYAN,
                selected_fg: Color::BLACK,
                selected_bg: Color::WHITE,
                weekend_fg: Color::rgb(200, 100, 100),
                cursor_fg: Color::BLACK,
                cursor_bg: Color::rgb(100, 200, 100),
                field_fg: Color::WHITE,
                field_active_fg: Color::BLACK,
                field_active_bg: Color::rgb(100, 150, 255),
            }
        }

        fn with_date(year: i32, month: u32, day: u32) -> Self {
            let mut picker = Self::new();
            picker.date = Date::new(year, month, day);
            picker.cursor_day = day;
            picker
        }

        fn with_first_day(first_day: FirstDayOfWeek) -> Self {
            let mut picker = Self::new();
            picker.first_day = first_day;
            picker
        }

        fn with_date_and_first_day(
            year: i32,
            month: u32,
            day: u32,
            first_day: FirstDayOfWeek,
        ) -> Self {
            let mut picker = Self::new();
            picker.date = Date::new(year, month, day);
            picker.cursor_day = day;
            picker.first_day = first_day;
            picker
        }
    }

    impl Rendering for MockRenderingPicker {
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

    // =========================================================================
    // first_weekday tests
    // =========================================================================

    #[test]
    fn test_first_weekday_jan_2024_sunday() {
        // Jan 1, 2024 is a Monday (day 1)
        // With Sunday first, Monday is day 1
        let picker = MockRenderingPicker::with_date(2024, 1, 1);
        // Jan 1 2024 is Monday, so first_weekday returns 1 for Sunday-first week
        assert_eq!(picker.first_weekday(), 1);
    }

    #[test]
    fn test_first_weekday_jan_2024_monday() {
        // Jan 1, 2024 is a Monday
        // With Monday first, Monday is day 0
        let picker = MockRenderingPicker::with_first_day(FirstDayOfWeek::Monday);
        assert_eq!(picker.first_weekday(), 0);
    }

    #[test]
    fn test_first_weekday_feb_2024_sunday() {
        // Feb 1, 2024 is a Thursday
        // With Sunday first, Thursday is day 4
        let picker = MockRenderingPicker::with_date(2024, 2, 1);
        assert_eq!(picker.first_weekday(), 4);
    }

    #[test]
    fn test_first_weekday_feb_2024_monday() {
        // Feb 1, 2024 is a Thursday
        // With Monday first, Thursday is day 3
        let picker =
            MockRenderingPicker::with_date_and_first_day(2024, 2, 1, FirstDayOfWeek::Monday);
        assert_eq!(picker.first_weekday(), 3);
    }

    #[test]
    fn test_first_weekday_mar_2024_sunday() {
        // Mar 1, 2024 is a Friday
        // With Sunday first, Friday is day 5
        let picker = MockRenderingPicker::with_date(2024, 3, 1);
        assert_eq!(picker.first_weekday(), 5);
    }

    #[test]
    fn test_first_weekday_mar_2024_monday() {
        // Mar 1, 2024 is a Friday
        // With Monday first, Friday is day 4
        let picker =
            MockRenderingPicker::with_date_and_first_day(2024, 3, 1, FirstDayOfWeek::Monday);
        assert_eq!(picker.first_weekday(), 4);
    }

    #[test]
    fn test_first_weekday_apr_2024_sunday() {
        // Apr 1, 2024 is a Monday
        // With Sunday first, Monday is day 1
        let picker = MockRenderingPicker::with_date(2024, 4, 1);
        assert_eq!(picker.first_weekday(), 1);
    }

    #[test]
    fn test_first_weekday_may_2024_sunday() {
        // May 1, 2024 is a Wednesday
        // With Sunday first, Wednesday is day 3
        let picker = MockRenderingPicker::with_date(2024, 5, 1);
        assert_eq!(picker.first_weekday(), 3);
    }

    #[test]
    fn test_first_weekday_jun_2024_sunday() {
        // Jun 1, 2024 is a Saturday
        // With Sunday first, Saturday is day 6
        let picker = MockRenderingPicker::with_date(2024, 6, 1);
        assert_eq!(picker.first_weekday(), 6);
    }

    #[test]
    fn test_first_weekday_jul_2024_sunday() {
        // Jul 1, 2024 is a Monday
        // With Sunday first, Monday is day 1
        let picker = MockRenderingPicker::with_date(2024, 7, 1);
        assert_eq!(picker.first_weekday(), 1);
    }

    #[test]
    fn test_first_weekday_aug_2024_sunday() {
        // Aug 1, 2024 is a Thursday
        // With Sunday first, Thursday is day 4
        let picker = MockRenderingPicker::with_date(2024, 8, 1);
        assert_eq!(picker.first_weekday(), 4);
    }

    #[test]
    fn test_first_weekday_sep_2024_sunday() {
        // Sep 1, 2024 is a Sunday
        // With Sunday first, Sunday is day 0
        let picker = MockRenderingPicker::with_date(2024, 9, 1);
        assert_eq!(picker.first_weekday(), 0);
    }

    #[test]
    fn test_first_weekday_sep_2024_monday() {
        // Sep 1, 2024 is a Sunday
        // With Monday first, Sunday is day 6
        let picker =
            MockRenderingPicker::with_date_and_first_day(2024, 9, 1, FirstDayOfWeek::Monday);
        assert_eq!(picker.first_weekday(), 6);
    }

    #[test]
    fn test_first_weekday_oct_2024_sunday() {
        // Oct 1, 2024 is a Tuesday
        // With Sunday first, Tuesday is day 2
        let picker = MockRenderingPicker::with_date(2024, 10, 1);
        assert_eq!(picker.first_weekday(), 2);
    }

    #[test]
    fn test_first_weekday_nov_2024_sunday() {
        // Nov 1, 2024 is a Friday
        // With Sunday first, Friday is day 5
        let picker = MockRenderingPicker::with_date(2024, 11, 1);
        assert_eq!(picker.first_weekday(), 5);
    }

    #[test]
    fn test_first_weekday_dec_2024_sunday() {
        // Dec 1, 2024 is a Sunday
        // With Sunday first, Sunday is day 0
        let picker = MockRenderingPicker::with_date(2024, 12, 1);
        assert_eq!(picker.first_weekday(), 0);
    }

    #[test]
    fn test_first_weekday_leap_year_feb() {
        // Feb 1, 2024 is a Thursday (same as 2023)
        let picker = MockRenderingPicker::with_date(2024, 2, 1);
        assert_eq!(picker.first_weekday(), 4);
    }

    #[test]
    fn test_first_weekday_non_leap_year_feb() {
        // Feb 1, 2023 is a Wednesday
        let picker = MockRenderingPicker::with_date(2023, 2, 1);
        assert_eq!(picker.first_weekday(), 3);
    }

    // =========================================================================
    // Trait method access tests
    // =========================================================================

    #[test]
    fn test_rendering_trait_methods_accessible() {
        let picker = MockRenderingPicker::new();
        // Test that all trait methods are accessible
        let _ = picker.date();
        let _ = picker.time();
        let _ = picker.mode();
        let _ = picker.first_day();
        let _ = picker.cursor_day();
        let _ = picker.time_field();
        let _ = picker.show_seconds();
        let _ = picker.header_fg();
        let _ = picker.selected_fg();
        let _ = picker.selected_bg();
        let _ = picker.weekend_fg();
        let _ = picker.cursor_fg();
        let _ = picker.cursor_bg();
        let _ = picker.field_fg();
        let _ = picker.field_active_fg();
        let _ = picker.field_active_bg();
    }

    #[test]
    fn test_first_weekday_returns_u32() {
        let picker = MockRenderingPicker::new();
        let result = picker.first_weekday();
        // Should be a valid day (0-6)
        assert!(result < 7);
    }
}
