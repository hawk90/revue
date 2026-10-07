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

impl EmptyState {
    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.state_type.icon())
    }

    fn render_full(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        // The state palette is this widget's own default; a rule naming
        // this node outranks it.
        let accent = ctx.css_color(self.state_type.color());

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
            let icon = self.get_icon();
            let icon_x = area.width / 2;
            let mut cell = Cell::new(icon);
            cell.fg = Some(accent);
            ctx.set(icon_x, y, cell);
            y += 2;
        }

        // Title (centered, bold)
        if y < area.height {
            let title_len = display_width(&self.title) as u16;
            let title_x = area.width.saturating_sub(title_len) / 2;
            let mut dx: u16 = 0;
            for ch in self.title.chars() {
                let cw = char_width(ch) as u16;
                if title_x + dx + cw > area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.modifier |= Modifier::BOLD;
                ctx.set(title_x + dx, y, cell);
                dx += cw;
            }
            y += 1;
        }

        // Description (centered, dimmed)
        if let Some(ref desc) = self.description {
            if y < area.height {
                let desc_len = display_width(desc) as u16;
                let desc_x = area.width.saturating_sub(desc_len) / 2;
                let mut dx: u16 = 0;
                for ch in desc.chars() {
                    let cw = char_width(ch) as u16;
                    if desc_x + dx + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(LIGHT_GRAY);
                    ctx.set(desc_x + dx, y, cell);
                    dx += cw;
                }
                y += 2;
            }
        }

        // Action button (centered)
        if let Some(ref action_text) = self.action {
            if y < area.height {
                let btn_text = format!("[ {} ]", action_text);
                let btn_len = display_width(&btn_text) as u16;
                let btn_x = area.width.saturating_sub(btn_len) / 2;
                let mut dx: u16 = 0;
                for ch in btn_text.chars() {
                    let cw = char_width(ch) as u16;
                    if btn_x + dx + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(accent);
                    ctx.set(btn_x + dx, y, cell);
                    dx += cw;
                }
            }
        }
    }

    fn render_compact(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        // The state palette is this widget's own default; a rule naming
        // this node outranks it.
        let accent = ctx.css_color(self.state_type.color());
        let mut y: u16 = 0;

        // Icon + Title on same line
        let mut x: u16 = 0;
        if self.show_icon {
            let icon = self.get_icon();
            let mut cell = Cell::new(icon);
            cell.fg = Some(accent);
            ctx.set(x, y, cell);
            x += 2;
        }

        let mut dx: u16 = 0;
        for ch in self.title.chars() {
            let cw = char_width(ch) as u16;
            if x + dx + cw > area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.modifier |= Modifier::BOLD;
            ctx.set(x + dx, y, cell);
            dx += cw;
        }
        y += 1;

        // Description
        if let Some(ref desc) = self.description {
            if y < area.height {
                let desc_x: u16 = if self.show_icon { 2 } else { 0 };
                let mut dx: u16 = 0;
                for ch in desc.chars() {
                    let cw = char_width(ch) as u16;
                    if desc_x + dx + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(LIGHT_GRAY);
                    ctx.set(desc_x + dx, y, cell);
                    dx += cw;
                }
                y += 1;
            }
        }

        // Action
        if let Some(ref action_text) = self.action {
            if y < area.height {
                let action_x: u16 = if self.show_icon { 2 } else { 0 };
                let btn_text = format!("[{}]", action_text);
                let mut dx: u16 = 0;
                for ch in btn_text.chars() {
                    let cw = char_width(ch) as u16;
                    if action_x + dx + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(accent);
                    ctx.set(action_x + dx, y, cell);
                    dx += cw;
                }
            }
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        // The state palette is this widget's own default; a rule naming
        // this node outranks it.
        let accent = ctx.css_color(self.state_type.color());
        let mut x: u16 = 0;

        // Icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut cell = Cell::new(icon);
            cell.fg = Some(accent);
            ctx.set(x, 0, cell);
            x += 2;
        }

        // Title
        let mut dx: u16 = 0;
        for ch in self.title.chars() {
            let cw = char_width(ch) as u16;
            if x + dx + cw > area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(LIGHT_GRAY);
            ctx.set(x + dx, 0, cell);
            dx += cw;
        }
    }
}
