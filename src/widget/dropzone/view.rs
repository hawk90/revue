//! Drawing: the background, the frame or indicator of each style, the
//! placeholder or drag verdict, and the bold focus frame

use crate::event::drag::DragData;
use crate::render::Modifier;
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View};

use super::{DropZone, DropZoneStyle};

/// Set one cell's symbol and foreground, if it is inside the area.
fn put(ctx: &mut RenderContext, x: u16, y: u16, symbol: char, color: Color) {
    if let Some(cell) = ctx.get_mut(x, y) {
        cell.symbol = symbol;
        cell.fg = Some(color);
    }
}

/// Set the background of the first `height` rows of the area.
fn fill_bg(ctx: &mut RenderContext, height: u16, bg: Color) {
    for y in 0..height {
        for x in 0..ctx.area.width {
            if let Some(cell) = ctx.get_mut(x, y) {
                cell.bg = Some(bg);
            }
        }
    }
}

impl<F> DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    /// Get current border color based on state
    ///
    /// While a drag is over the zone the border says whether the drag would be
    /// accepted, which no single `border-color` can express - the same reading
    /// an invalid form field gets. At rest the stylesheet is in charge, with
    /// the builder on top of it.
    fn current_border_color(&self, ctx: &RenderContext) -> Color {
        if self.hovered {
            if self.can_accept_current {
                self.hover_color
            } else {
                self.reject_color
            }
        } else {
            self.border_color
                .or_else(|| ctx.css_border_or_text_color())
                .unwrap_or(DISABLED_FG)
        }
    }

    /// Get border characters based on style
    fn border_chars(&self) -> (char, char, char, char, char, char) {
        match self.style {
            DropZoneStyle::Solid => ('┌', '┐', '└', '┘', '─', '│'),
            DropZoneStyle::Dashed => ('┌', '┐', '└', '┘', '╌', '╎'),
            DropZoneStyle::Highlight | DropZoneStyle::Minimal => (' ', ' ', ' ', ' ', ' ', ' '),
        }
    }

    /// The Solid or Dashed frame around `height` rows.
    fn draw_frame(&self, ctx: &mut RenderContext, height: u16, color: Color) {
        let width = ctx.area.width;
        let (tl, tr, bl, br, h, v) = self.border_chars();

        // One row cannot hold a top and a bottom: the sides, around the text.
        if height == 1 {
            put(ctx, 0, 0, v, color);
            if width > 1 {
                put(ctx, width - 1, 0, v, color);
            }
            return;
        }

        // Top border
        put(ctx, 0, 0, tl, color);
        for x in 1..width.saturating_sub(1) {
            put(ctx, x, 0, h, color);
        }
        if width > 1 {
            put(ctx, width - 1, 0, tr, color);
        }

        // Bottom border
        let bottom_y = height.saturating_sub(1);
        put(ctx, 0, bottom_y, bl, color);
        for x in 1..width.saturating_sub(1) {
            put(ctx, x, bottom_y, h, color);
        }
        if width > 1 {
            put(ctx, width - 1, bottom_y, br, color);
        }

        // Side borders
        for y in 1..bottom_y {
            put(ctx, 0, y, v, color);
            if width > 1 {
                put(ctx, width - 1, y, v, color);
            }
        }
    }

    /// The Highlight style: the zone filled while a drag is over it.
    fn draw_highlight(&self, ctx: &mut RenderContext, height: u16) {
        if self.hovered {
            let bg = if self.can_accept_current {
                Color::rgb(30, 60, 30)
            } else {
                Color::rgb(60, 30, 30)
            };
            fill_bg(ctx, height, bg);
        }
    }

    /// The Minimal style: an indicator down the left edge.
    fn draw_minimal(&self, ctx: &mut RenderContext, height: u16, color: Color) {
        let indicator = if self.hovered {
            if self.can_accept_current {
                '▶'
            } else {
                '✗'
            }
        } else {
            '│'
        };
        for y in 0..height {
            put(ctx, 0, y, indicator, color);
        }
    }

    /// The placeholder at rest, or the drag's verdict while one hovers.
    fn draw_placeholder(&self, ctx: &mut RenderContext, height: u16, color: Color, resting: Color) {
        // Inside a frame the text keeps off its two border rows; two rows are
        // nothing but border.
        let framed = matches!(self.style, DropZoneStyle::Solid | DropZoneStyle::Dashed);
        if framed && height == 2 {
            return;
        }
        let text_y = height / 2;
        let text_x: u16 = 2;
        let max_len = ctx.area.width.saturating_sub(4);

        let display_text = if self.hovered {
            if self.can_accept_current {
                "Drop here!"
            } else {
                "Cannot drop here"
            }
        } else {
            &self.placeholder
        };

        let text_color = if self.hovered { color } else { resting };

        let highlighted = self.hovered && self.style == DropZoneStyle::Highlight;
        match self.state.bg {
            Some(bg) if !highlighted => {
                ctx.draw_text_clipped_bg(text_x, text_y, display_text, text_color, bg, max_len)
            }
            _ => ctx.draw_text_clipped(text_x, text_y, display_text, text_color, max_len),
        }
    }

    /// Focused, the frame (or the minimal indicator) is drawn bold.
    fn bold_frame(&self, ctx: &mut RenderContext, height: u16) {
        let width = ctx.area.width;
        let bottom_y = height.saturating_sub(1);
        let right_x = width.saturating_sub(1);
        let framed = matches!(self.style, DropZoneStyle::Solid | DropZoneStyle::Dashed);
        for y in 0..height {
            for x in 0..width {
                let on_frame = if framed {
                    y == 0 || y == bottom_y || x == 0 || x == right_x
                } else {
                    self.style == DropZoneStyle::Minimal && x == 0
                };
                if on_frame {
                    if let Some(cell) = ctx.get_mut(x, y) {
                        cell.modifier |= Modifier::BOLD;
                    }
                }
            }
        }
    }
}

impl<F> View for DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    /// At least `min_height` rows, however little the others leave.
    fn min_size(&self) -> (u16, u16) {
        (0, self.min_height)
    }

    fn render(&self, ctx: &mut RenderContext) {
        // Draws into the area it is given; `min_height` asks the layout for
        // rows through `min_size`.
        let height = ctx.area.height;
        let disabled = self.state.disabled;
        // Disabled, the zone is greyed out at rest; a drag over it still gets
        // the reject color, which says why nothing happens.
        let color = if disabled && !self.hovered {
            DISABLED_FG
        } else {
            self.current_border_color(ctx)
        };
        let resting_text = if disabled {
            DISABLED_FG
        } else {
            self.state.fg.unwrap_or_else(|| ctx.css_color(LIGHT_GRAY))
        };

        if let Some(bg) = self.state.bg {
            fill_bg(ctx, height, bg);
        }

        match self.style {
            DropZoneStyle::Solid | DropZoneStyle::Dashed => self.draw_frame(ctx, height, color),
            DropZoneStyle::Highlight => self.draw_highlight(ctx, height),
            DropZoneStyle::Minimal => self.draw_minimal(ctx, height, color),
        }

        self.draw_placeholder(ctx, height, color, resting_text);

        if self.state.focused && !disabled {
            self.bold_frame(ctx, height);
        }
    }

    crate::impl_view_meta!(@common);
    fn meta(&self) -> crate::dom::WidgetMeta {
        self.props
            .build_meta("DropZone", false, self.state.disabled)
    }
}
