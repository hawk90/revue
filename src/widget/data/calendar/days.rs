//! The day grid: one row per week, each day styled by selection, range,
//! today, marker and weekend, with the week numbers on the left

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::RenderContext;

use super::render::CalendarRender;
use super::types::{DateMarker, FirstDayOfWeek};
use super::{days_in_month, first_day_of_month, Date};

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

    /// Colors and modifier of a day: the selection, then the range, today,
    /// a marker and the weekend, else the day color
    fn day_style(
        &self,
        ctx: &RenderContext,
        is_selected: bool,
        is_in_range: bool,
        is_today: bool,
        is_weekend: bool,
        marker: Option<&DateMarker>,
    ) -> (Color, Option<Color>, Modifier) {
        if is_selected {
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
            // An ordinary day takes `color`; today, weekends, markers
            // and the selection keep theirs - a rule cannot address
            // those separately.
            (
                ctx.color_or(self.day_fg, Color::WHITE),
                None,
                Modifier::empty(),
            )
        }
    }

    /// Every day of the month, one row per week below the day names
    pub(super) fn render_days(
        &self,
        ctx: &mut RenderContext,
        start_x: u16,
        start_y: u16,
        week_num_offset: u16,
    ) {
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
                    ctx.set(start_x + i as u16, y, cell);
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

                let (fg, bg, modifier) =
                    self.day_style(ctx, is_selected, is_in_range, is_today, is_weekend, marker);

                // Draw day number
                let day_str = format!("{:2}", cell_day);
                for (i, ch) in day_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = bg;
                    cell.modifier = modifier;
                    ctx.set(x + i as u16, y, cell);
                }

                // Draw marker symbol
                if let Some(m) = marker {
                    if let Some(sym) = m.symbol {
                        let mut cell = Cell::new(sym);
                        cell.fg = Some(m.color);
                        ctx.set(x + 2, y, cell);
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
