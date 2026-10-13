//! Vertical timeline: per event, the timestamp, the icon on the line, the
//! title and the description, one under the other

#![allow(clippy::too_many_arguments)]

use super::{Timeline, TimelineEvent, TimelineStyle};
use crate::render::{Cell, Modifier};
use crate::utils::{char_width, truncate_to_width};
use crate::widget::traits::RenderContext;

impl Timeline {
    pub(super) fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if self.events.is_empty() || area.height < 2 {
            return;
        }

        let timestamp_width = if self.show_timestamps { 12 } else { 0 };
        let icon_x = timestamp_width;
        let content_x = icon_x + 3;
        let content_width = area.width.saturating_sub(timestamp_width + 3);

        let mut y = 0u16;

        for (i, event) in self.events.iter().enumerate().skip(self.scroll) {
            if y >= area.height {
                break;
            }

            let is_selected = self.selected == Some(i);

            self.draw_timestamp(ctx, event, y, timestamp_width);
            self.draw_marker(ctx, i, event, y, icon_x, is_selected);
            self.draw_title(ctx, event, y, content_x, content_width, is_selected);

            y += 1;

            if self.draw_description(ctx, i, event, y, icon_x, content_x, content_width) {
                y += 1;
            }

            // Add spacing between events
            if y < area.height && i < self.events.len() - 1 {
                if self.style != TimelineStyle::Minimal {
                    let mut line_cell = Cell::new('│');
                    line_cell.fg = Some(self.line_color);
                    ctx.set(icon_x, y, line_cell);
                }
                y += 1;
            }
        }
    }

    /// The timestamp (when shown), cut to its gutter, on row `y`
    fn draw_timestamp(
        &self,
        ctx: &mut RenderContext,
        event: &TimelineEvent,
        y: u16,
        timestamp_width: u16,
    ) {
        // Draw timestamp
        if self.show_timestamps {
            if let Some(ref ts) = event.timestamp {
                let truncated = truncate_to_width(ts, timestamp_width as usize - 1);
                let mut dx: u16 = 0;
                for ch in truncated.chars() {
                    let cw = char_width(ch) as u16;
                    if dx + cw > timestamp_width - 1 {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.timestamp_color);
                    ctx.set(dx, y, cell);
                    dx += cw;
                }
            }
        }
    }

    /// The icon, the line down to the next event and the connector, on row `y`
    fn draw_marker(
        &self,
        ctx: &mut RenderContext,
        i: usize,
        event: &TimelineEvent,
        y: u16,
        icon_x: u16,
        is_selected: bool,
    ) {
        let area = ctx.area;
        let color = event.display_color();

        // Draw icon
        let icon = event.event_type.icon();
        let mut icon_cell = Cell::new(icon);
        icon_cell.fg = Some(color);
        if is_selected {
            icon_cell.modifier |= Modifier::BOLD;
        }
        ctx.set(icon_x, y, icon_cell);

        // Draw line (except for last item)
        if i < self.events.len() - 1 && self.style != TimelineStyle::Minimal {
            let line_char = match self.style {
                TimelineStyle::Line => '│',
                TimelineStyle::Boxed => '│',
                TimelineStyle::Alternating => '│',
                TimelineStyle::Minimal => ' ',
            };
            let line_y = y + 1;
            if line_y < area.height {
                let mut line_cell = Cell::new(line_char);
                line_cell.fg = Some(self.line_color);
                ctx.set(icon_x, line_y, line_cell);
            }
        }

        // Draw connector
        let connector = match self.style {
            TimelineStyle::Line | TimelineStyle::Alternating => '─',
            TimelineStyle::Boxed => '─',
            TimelineStyle::Minimal => ' ',
        };
        if self.style != TimelineStyle::Minimal {
            let mut conn_cell = Cell::new(connector);
            conn_cell.fg = Some(self.line_color);
            ctx.set(icon_x + 1, y, conn_cell);
        }
    }

    /// The title, cut to the content width, on row `y`
    fn draw_title(
        &self,
        ctx: &mut RenderContext,
        event: &TimelineEvent,
        y: u16,
        content_x: u16,
        content_width: u16,
        is_selected: bool,
    ) {
        let color = event.display_color();

        // Draw title
        let title_fg = if is_selected { color } else { self.title_color };
        let title_truncated = truncate_to_width(&event.title, content_width as usize);
        let mut dx: u16 = 0;
        for ch in title_truncated.chars() {
            let cw = char_width(ch) as u16;
            if dx + cw > content_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_fg);
            if is_selected {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(content_x + dx, y, cell);
            dx += cw;
        }
    }

    /// The description (when shown) on row `y`, with the line beside it; whether it was drawn
    fn draw_description(
        &self,
        ctx: &mut RenderContext,
        i: usize,
        event: &TimelineEvent,
        y: u16,
        icon_x: u16,
        content_x: u16,
        content_width: u16,
    ) -> bool {
        let area = ctx.area;

        // Draw description
        if self.show_descriptions {
            if let Some(ref desc) = event.description {
                if y < area.height {
                    // Draw line continuation
                    if i < self.events.len() - 1 && self.style != TimelineStyle::Minimal {
                        let mut line_cell = Cell::new('│');
                        line_cell.fg = Some(self.line_color);
                        ctx.set(icon_x, y, line_cell);
                    }

                    // Draw description text
                    let desc_truncated = truncate_to_width(desc, content_width as usize);
                    let mut dx: u16 = 0;
                    for ch in desc_truncated.chars() {
                        let cw = char_width(ch) as u16;
                        if dx + cw > content_width {
                            break;
                        }
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.desc_color);
                        ctx.set(content_x + dx, y, cell);
                        dx += cw;
                    }

                    return true;
                }
            }
        }
        false
    }
}
