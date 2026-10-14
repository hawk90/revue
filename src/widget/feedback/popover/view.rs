use super::types::{PopoverArrow, PopoverPosition, PopoverStyle};
use super::Popover;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::widget::traits::{RenderContext, View};

/// Where the popover's box lands in the area, and the side it took
#[derive(Clone, Copy)]
struct Frame {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    side: PopoverPosition,
}

/// The resolved colors of one render
#[derive(Clone, Copy)]
struct Colors {
    fg: Color,
    bg: Color,
    border: Color,
}

impl View for Popover {
    crate::impl_view_meta!("Popover");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.open {
            return;
        }

        let area = ctx.area;
        let (popup_w, popup_h) = self.calculate_dimensions();
        let (popup_x, popup_y, actual_position) = self.calculate_position(area.width, area.height);
        let frame = Frame {
            x: popup_x,
            y: popup_y,
            w: popup_w,
            h: popup_h,
            side: actual_position,
        };
        let colors = self.colors(ctx);

        // Draw shadow for elevated style (offset by 1 pixel right and down)
        if matches!(self.popover_style, PopoverStyle::Elevated) {
            Self::draw_shadow(ctx, frame);
        }
        Self::draw_background(ctx, frame, colors);

        // Draw border
        let content_start_x;
        let content_start_y;

        if let Some(border) = self.popover_style.border_chars() {
            content_start_x = popup_x + 2;
            content_start_y = popup_y + 1;
            self.draw_border(ctx, frame, colors, &border);
        } else {
            content_start_x = popup_x + 1;
            content_start_y = popup_y;
        }

        self.draw_content(ctx, frame, colors, (content_start_x, content_start_y));
        self.draw_arrow(ctx, frame, colors);
    }
}

impl Popover {
    /// Builder, then the stylesheet, then the popover style's own palette.
    fn colors(&self, ctx: &RenderContext) -> Colors {
        let (default_fg, default_bg, default_border) = self.popover_style.colors();
        let fg = self.state.fg.unwrap_or_else(|| ctx.css_color(default_fg));
        let bg = self
            .state
            .bg
            .unwrap_or_else(|| ctx.css_background(default_bg));
        let border_fg = self
            .border_color
            .or_else(|| ctx.css_border_or_text_color())
            .unwrap_or(default_border);
        Colors {
            fg,
            bg,
            border: border_fg,
        }
    }

    fn draw_shadow(ctx: &mut RenderContext, frame: Frame) {
        let area = ctx.area;
        let Frame {
            x: popup_x,
            y: popup_y,
            w: popup_w,
            h: popup_h,
            ..
        } = frame;
        // Shadow only renders in the area below and to the right of the popover,
        // not overlapping the popover itself. Clamp to area bounds.
        let shadow_x_start = popup_x + 1;
        let shadow_y_start = popup_y + 1;
        let shadow_x_end = (popup_x + popup_w + 1).min(area.width);
        let shadow_y_end = (popup_y + popup_h + 1).min(area.height);

        for y in shadow_y_start..shadow_y_end {
            for x in shadow_x_start..shadow_x_end {
                // Only draw shadow outside the popover body
                let inside_popup =
                    x >= popup_x && x < popup_x + popup_w && y >= popup_y && y < popup_y + popup_h;
                if !inside_popup {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(Color::rgb(15, 15, 15));
                    ctx.set(x, y, cell);
                }
            }
        }
    }

    fn draw_background(ctx: &mut RenderContext, frame: Frame, colors: Colors) {
        let area = ctx.area;
        let Frame {
            x: popup_x,
            y: popup_y,
            w: popup_w,
            h: popup_h,
            ..
        } = frame;
        for dy in 0..popup_h {
            for dx in 0..popup_w {
                let x = popup_x + dx;
                let y = popup_y + dy;
                if x < area.width && y < area.height {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(colors.bg);
                    ctx.set(x, y, cell);
                }
            }
        }
    }

    /// The frame's border, and the title on the row below its top edge
    fn draw_border(
        &self,
        ctx: &mut RenderContext,
        frame: Frame,
        colors: Colors,
        border: &BorderChars,
    ) {
        let area = ctx.area;
        let Frame {
            x: popup_x,
            y: popup_y,
            w: popup_w,
            h: popup_h,
            ..
        } = frame;
        let edge = |ch: char| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(colors.border);
            cell.bg = Some(colors.bg);
            cell
        };

        // Top border
        if popup_y < area.height {
            ctx.set(popup_x, popup_y, edge(border.top_left));
            for dx in 1..popup_w - 1 {
                ctx.set(popup_x + dx, popup_y, edge(border.horizontal));
            }
            ctx.set(popup_x + popup_w - 1, popup_y, edge(border.top_right));
        }

        // Title if present
        if let Some(ref title) = self.title {
            let title_x = popup_x + 2;
            let title_y = popup_y + 1;
            let (fg, bg) = (colors.fg, colors.bg);
            ctx.put_str_with(title_x, title_y, title, popup_x + popup_w - 2, |ch| {
                Cell::new(ch).fg(fg).bg(bg).bold()
            });
        }

        // Side borders
        for dy in 1..popup_h - 1 {
            let y = popup_y + dy;
            if y < area.height {
                ctx.set(popup_x, y, edge(border.vertical));
                ctx.set(popup_x + popup_w - 1, y, edge(border.vertical));
            }
        }

        // Bottom border
        let bottom_y = popup_y + popup_h - 1;
        if bottom_y < area.height {
            ctx.set(popup_x, bottom_y, edge(border.bottom_left));
            for dx in 1..popup_w - 1 {
                ctx.set(popup_x + dx, bottom_y, edge(border.horizontal));
            }
            ctx.set(popup_x + popup_w - 1, bottom_y, edge(border.bottom_right));
        }
    }

    /// The wrapped content lines, from `(content_start_x, content_start_y)`
    fn draw_content(
        &self,
        ctx: &mut RenderContext,
        frame: Frame,
        colors: Colors,
        (content_start_x, content_start_y): (u16, u16),
    ) {
        let area = ctx.area;
        let Frame {
            x: popup_x,
            y: popup_y,
            w: popup_w,
            h: popup_h,
            ..
        } = frame;
        let (fg, bg) = (colors.fg, colors.bg);
        let lines = self.wrap_content();
        let text_y_offset = if self.title.is_some() && self.popover_style.border_chars().is_some() {
            1
        } else {
            0
        };

        // The last row is the bottom border, if there is one.
        let content_end_y = if self.popover_style.border_chars().is_some() {
            popup_y + popup_h - 1
        } else {
            popup_y + popup_h
        };
        for (i, line) in lines.iter().enumerate() {
            let y = content_start_y + text_y_offset + i as u16;
            if y >= area.height || y >= content_end_y {
                break;
            }

            ctx.put_str_with(content_start_x, y, line, popup_x + popup_w - 1, |ch| {
                Cell::new(ch).fg(fg).bg(bg)
            });
        }
    }

    /// The arrow next to the anchor, on the side the frame took
    fn draw_arrow(&self, ctx: &mut RenderContext, frame: Frame, colors: Colors) {
        if matches!(self.arrow, PopoverArrow::None) {
            return;
        }
        let area = ctx.area;
        let Frame {
            x: popup_x,
            y: popup_y,
            w: popup_w,
            h: popup_h,
            side: actual_position,
        } = frame;
        let arrow_char = self.arrow.chars(actual_position);
        let (arrow_x, arrow_y) = match actual_position {
            PopoverPosition::Top => (self.anchor.0, popup_y + popup_h),
            PopoverPosition::Bottom => (self.anchor.0, popup_y.saturating_sub(1)),
            PopoverPosition::Left => (popup_x + popup_w, self.anchor.1),
            PopoverPosition::Right => (popup_x.saturating_sub(1), self.anchor.1),
            // Auto is already resolved to a concrete position in calculate_position
            // This case should never be reached, but use Bottom as fallback
            PopoverPosition::Auto => (self.anchor.0, popup_y.saturating_sub(1)),
        };

        if arrow_x < area.width && arrow_y < area.height {
            let mut cell = Cell::new(arrow_char);
            cell.fg = Some(colors.border);
            ctx.set(arrow_x, arrow_y, cell);
        }
    }
}
