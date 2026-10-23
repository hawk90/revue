//! The title line and the search line.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::RenderContext;

use super::{CommandPalette, Layout};

impl CommandPalette {
    /// The title row: bold cyan text between the side borders.
    pub(super) fn draw_title(
        &self,
        ctx: &mut RenderContext,
        layout: &Layout,
        current_y: u16,
        title: &str,
    ) {
        self.draw_left(ctx, layout, current_y);

        let title_x = layout.x + 2;
        let mut dx: u16 = 0;
        for ch in title.chars() {
            let cw = char_width(ch) as u16;
            // A glyph that would cross the border is not drawn
            if title_x + dx + cw > layout.text_end {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::CYAN);
            cell.bg = Some(self.bg_color);
            cell.modifier |= Modifier::BOLD;
            ctx.set(title_x + dx, current_y, cell);
            dx += cw;
        }

        self.draw_right(ctx, layout, current_y);
    }

    /// The search row: icon, query (or placeholder) and cursor.
    pub(super) fn draw_search(&self, ctx: &mut RenderContext, layout: &Layout, current_y: u16) {
        let x = layout.x;
        let text_end = layout.text_end;

        self.draw_left(ctx, layout, current_y);

        // Search icon
        let search_icon = Cell::new('🔍');
        ctx.set(x + 2, current_y, search_icon);

        // Query or placeholder
        let input_x = x + 4;
        let display_text = if self.query.is_empty() {
            &self.placeholder
        } else {
            &self.query
        };
        // The typed query takes `color`; the empty-state grey keeps its own.
        let text_color = if self.query.is_empty() {
            DISABLED_FG
        } else {
            ctx.css_color(Color::WHITE)
        };

        let mut dx: u16 = 0;
        for ch in display_text.chars() {
            let cw = char_width(ch) as u16;
            if input_x + dx + cw > text_end {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(text_color);
            cell.bg = Some(self.bg_color);
            ctx.set(input_x + dx, current_y, cell);
            dx += cw;
        }

        // Cursor
        if !self.query.is_empty() || display_text == &self.placeholder {
            let cursor_x = input_x + display_width(&self.query) as u16;
            if cursor_x < text_end {
                let mut cursor = Cell::new('▏');
                cursor.fg = Some(Color::WHITE);
                cursor.bg = Some(self.bg_color);
                ctx.set(cursor_x, current_y, cursor);
            }
        }

        self.draw_right(ctx, layout, current_y);
    }
}
