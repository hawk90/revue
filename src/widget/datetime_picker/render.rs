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
