//! Horizontal timeline: a line across, each event's icon on it, the title
//! above and the timestamp below, 15 columns per event

use super::Timeline;
use crate::render::{Cell, Modifier};
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::traits::RenderContext;

impl Timeline {
    pub(super) fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if self.events.is_empty() || area.width < 10 {
            return;
        }

        let event_width = 15u16;
        let line_y = 1u16;

        // Draw horizontal line
        for x in 0..area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.line_color);
            ctx.set(x, line_y, cell);
        }

        // Draw events
        let mut x = 0u16;
        for (i, event) in self.events.iter().enumerate() {
            if x >= area.width {
                break;
            }

            let is_selected = self.selected == Some(i);
            let color = event.display_color();

            // Draw icon
            let icon = event.event_type.icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(color);
            if is_selected {
                icon_cell.modifier |= Modifier::BOLD;
            }
            ctx.set(x + event_width / 2, line_y, icon_cell);

            // Draw title above
            let title = truncate_to_width(&event.title, event_width as usize - 1);
            let title_x = x + (event_width.saturating_sub(display_width(title) as u16)) / 2;
            let mut dx: u16 = 0;
            for ch in title.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > event_width - 1 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if is_selected { color } else { self.title_color });
                ctx.set(title_x + dx, 0, cell);
                dx += cw;
            }

            // Draw timestamp below
            if self.show_timestamps {
                if let Some(ref ts) = event.timestamp {
                    let ts_str = truncate_to_width(ts, event_width as usize - 1);
                    let ts_x = x + (event_width.saturating_sub(display_width(ts_str) as u16)) / 2;
                    let mut dx: u16 = 0;
                    for ch in ts_str.chars() {
                        let cw = char_width(ch) as u16;
                        if dx + cw > event_width - 1 {
                            break;
                        }
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.timestamp_color);
                        ctx.set(ts_x + dx, line_y + 1, cell);
                        dx += cw;
                    }
                }
            }

            x += event_width;
        }
    }
}
