//! Range picker view/render implementation

use super::core::RangePicker;
use super::types::RangeFocus;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::widget::calendar::{days_in_month, Date};
use crate::widget::traits::{RenderContext, View};
use crate::{impl_styled_view, impl_widget_builders};

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
impl_widget_builders!(RangePicker);

impl RangePicker {
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
            crate::widget::calendar::FirstDayOfWeek::Sunday => h as u32,
            crate::widget::calendar::FirstDayOfWeek::Monday => ((h + 6) % 7) as u32,
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
        let header = format!(
            "{}: {} {}",
            title,
            super::impls::month_name(date.month),
            date.year
        );
        let header_color = if is_focused {
            self.header_fg
        } else {
            Color::rgb(100, 100, 100)
        };
        self.draw_text(ctx, x, y, &header, header_color, true);

        // Day headers
        let day_headers = match self.first_day {
            crate::widget::calendar::FirstDayOfWeek::Sunday => "Su Mo Tu We Th Fr Sa",
            crate::widget::calendar::FirstDayOfWeek::Monday => "Mo Tu We Th Fr Sa Su",
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
            let ch_width = char_width(ch) as u16;
            if ch_width == 0 {
                continue;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            if bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x + offset, y, cell);
            for i in 1..ch_width {
                ctx.buffer.set(x + offset + i, y, Cell::continuation());
            }
            offset += ch_width;
        }
    }
}
