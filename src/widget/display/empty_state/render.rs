//! Drawing the empty state: full, compact and minimal layouts

use super::{EmptyState, EmptyStateVariant};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

impl View for EmptyState {
    crate::impl_view_meta!("EmptyState");

    /// [`height`](EmptyState::height) rows, as wide as offered (the content
    /// is centered across it).
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, self.height().min(max_height)))
    }

    /// It stretches across the width it is offered.
    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        match self.variant {
            EmptyStateVariant::Full => self.render_full(ctx),
            EmptyStateVariant::Compact => self.render_compact(ctx),
            EmptyStateVariant::Minimal => self.render_minimal(ctx),
        }
    }
}

/// Draw `text` from column `x` of row `y`, stopping at the first character
/// that would cross the right edge
fn draw_clipped(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    text: &str,
    fg: Color,
    modifier: Modifier,
) {
    let area = ctx.area;
    let mut dx: u16 = 0;
    for ch in text.chars() {
        let cw = char_width(ch) as u16;
        if x + dx + cw > area.width {
            break;
        }
        let mut cell = Cell::new(ch);
        cell.fg = Some(fg);
        cell.modifier |= modifier;
        ctx.set(x + dx, y, cell);
        dx += cw;
    }
}

/// The column that centers `text` in `width`
fn centered_x(width: u16, text: &str) -> u16 {
    let len = display_width(text) as u16;
    width.saturating_sub(len) / 2
}

impl EmptyState {
    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.state_type.icon())
    }

    /// The color of the icon and the action
    fn accent(&self, ctx: &RenderContext) -> Color {
        // The state palette is this widget's own default; a rule naming
        // this node outranks it.
        ctx.css_color(self.state_type.color())
    }

    /// Draw the icon at (`x`, `y`)
    fn draw_icon(&self, ctx: &mut RenderContext, x: u16, y: u16, accent: Color) {
        let icon = self.get_icon();
        let mut cell = Cell::new(icon);
        cell.fg = Some(accent);
        ctx.set(x, y, cell);
    }

    fn render_full(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let accent = self.accent(ctx);

        // Calculate vertical centering
        let content_height = self.height();
        let start_y = if area.height > content_height {
            (area.height - content_height) / 2
        } else {
            0u16
        };

        let mut y = start_y;

        // Icon (centered, large)
        if self.show_icon && y < area.height {
            self.draw_icon(ctx, area.width / 2, y, accent);
            y += 2;
        }

        // Title (centered, bold)
        if y < area.height {
            let title_x = centered_x(area.width, &self.title);
            draw_clipped(ctx, title_x, y, &self.title, Color::WHITE, Modifier::BOLD);
            y += 1;
        }

        // Description (centered, dimmed)
        if let Some(ref desc) = self.description {
            if y < area.height {
                let desc_x = centered_x(area.width, desc);
                draw_clipped(ctx, desc_x, y, desc, LIGHT_GRAY, Modifier::empty());
                y += 2;
            }
        }

        // Action button (centered)
        if let Some(ref action_text) = self.action {
            if y < area.height {
                let btn_text = format!("[ {} ]", action_text);
                let btn_x = centered_x(area.width, &btn_text);
                draw_clipped(ctx, btn_x, y, &btn_text, accent, Modifier::empty());
            }
        }
    }

    fn render_compact(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let accent = self.accent(ctx);
        let mut y: u16 = 0;

        // Icon + Title on same line
        let mut x: u16 = 0;
        if self.show_icon {
            self.draw_icon(ctx, x, y, accent);
            x += 2;
        }

        draw_clipped(ctx, x, y, &self.title, Color::WHITE, Modifier::BOLD);
        y += 1;

        // Description and action sit under the title, past the icon
        let indent: u16 = if self.show_icon { 2 } else { 0 };

        // Description
        if let Some(ref desc) = self.description {
            if y < area.height {
                draw_clipped(ctx, indent, y, desc, LIGHT_GRAY, Modifier::empty());
                y += 1;
            }
        }

        // Action
        if let Some(ref action_text) = self.action {
            if y < area.height {
                let btn_text = format!("[{}]", action_text);
                draw_clipped(ctx, indent, y, &btn_text, accent, Modifier::empty());
            }
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext) {
        let accent = self.accent(ctx);
        let mut x: u16 = 0;

        // Icon
        if self.show_icon {
            self.draw_icon(ctx, x, 0, accent);
            x += 2;
        }

        // Title
        draw_clipped(ctx, x, 0, &self.title, LIGHT_GRAY, Modifier::empty());
    }
}
