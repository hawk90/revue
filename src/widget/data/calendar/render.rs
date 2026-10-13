//! Calendar rendering: the drawing state, the header and day-name rows, and
//! `render_month`, which lays them out above the day grid

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::render_border;
use crate::utils::{char_width, display_width};
use crate::widget::traits::RenderContext;

use super::types::{DateMarker, FirstDayOfWeek};
use super::Date;

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

    /// Render month view
    pub fn render_month(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 20 || area.height < 8 {
            return;
        }

        let has_border = self.border_color.is_some();
        let start_x = if has_border { 1u16 } else { 0u16 };
        let start_y = if has_border { 1u16 } else { 0u16 };
        let week_num_offset: u16 = if self.show_week_numbers { 4 } else { 0 };

        // Draw border if specified
        if let Some(border_color) = self.border_color {
            render_border(ctx, area, border_color);
        }

        self.render_header(ctx, start_x, start_y, week_num_offset);
        self.render_day_names(ctx, start_x, start_y, week_num_offset);
        self.render_days(ctx, start_x, start_y, week_num_offset);
    }

    /// Month name and year, centered over the grid, with the arrows when focused
    fn render_header(
        &self,
        ctx: &mut RenderContext,
        start_x: u16,
        start_y: u16,
        week_num_offset: u16,
    ) {
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
        let header_x =
            start_x + week_num_offset + (20u16.saturating_sub(display_width(&header) as u16)) / 2;

        let mut dx: u16 = 0;
        for ch in header.chars() {
            let cw = char_width(ch) as u16;
            let mut cell = Cell::new(ch);
            cell.fg = Some(self.header_fg);
            cell.bg = self.header_bg;
            cell.modifier |= Modifier::BOLD;
            ctx.set(header_x + dx, start_y, cell);
            dx += cw;
        }

        // Navigation arrows
        if self.focused {
            let mut left = Cell::new('◀');
            left.fg = Some(self.header_fg);
            ctx.set(start_x + week_num_offset, start_y, left);

            let mut right = Cell::new('▶');
            right.fg = Some(self.header_fg);
            ctx.set(start_x + week_num_offset + 21, start_y, right);
        }
    }

    /// The day names (and `W` over the week numbers), two rows below the header
    fn render_day_names(
        &self,
        ctx: &mut RenderContext,
        start_x: u16,
        start_y: u16,
        week_num_offset: u16,
    ) {
        // Week header
        let y = start_y + 2;
        let day_names = self.day_names();

        if self.show_week_numbers {
            let mut wk = Cell::new('W');
            wk.fg = Some(self.header_fg);
            ctx.set(start_x, y, wk);
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
                ctx.set(x + j as u16, y, cell);
            }
        }
    }
}
