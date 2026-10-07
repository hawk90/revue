//! Timeline rendering: vertical and horizontal layouts and the `View` impl

use super::{Timeline, TimelineOrientation, TimelineStyle};
use crate::render::{Cell, Modifier};
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::traits::{RenderContext, View};

impl View for Timeline {
    crate::impl_view_meta!("Timeline");

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            TimelineOrientation::Vertical => self.render_vertical(ctx),
            TimelineOrientation::Horizontal => self.render_horizontal(ctx),
        }
    }
}

impl Timeline {
    fn render_vertical(&self, ctx: &mut RenderContext) {
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
            let color = event.display_color();

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

            y += 1;

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

                        y += 1;
                    }
                }
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

    fn render_horizontal(&self, ctx: &mut RenderContext) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::data::timeline::TimelineEvent;

    // KEEP HERE: uses internal RenderContext and Buffer
    #[test]
    fn test_timeline_render() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new()
            .event(TimelineEvent::new("Event 1").timestamp("10:00"))
            .event(TimelineEvent::new("Event 2").timestamp("11:00"));

        tl.render(&mut ctx);
        // Smoke test
    }

    // KEEP HERE: uses internal RenderContext and Buffer
    #[test]
    fn test_render_horizontal() {
        let mut buffer = Buffer::new(60, 10);
        let area = Rect::new(0, 0, 60, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new()
            .horizontal()
            .event(TimelineEvent::new("Event 1").timestamp("10:00"))
            .event(TimelineEvent::new("Event 2").timestamp("11:00"));

        tl.render(&mut ctx); // Should not panic
    }

    // KEEP HERE: uses internal RenderContext and Buffer
    #[test]
    fn test_render_empty() {
        let mut buffer = Buffer::new(60, 10);
        let area = Rect::new(0, 0, 60, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new();
        tl.render(&mut ctx); // Should return early without panicking
    }

    // KEEP HERE: uses internal RenderContext and Buffer
    #[test]
    fn test_render_with_descriptions() {
        let mut buffer = Buffer::new(60, 10);
        let area = Rect::new(0, 0, 60, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let tl = Timeline::new()
            .descriptions(true)
            .event(TimelineEvent::new("Event").description("Details here"));

        tl.render(&mut ctx);
    }
}
