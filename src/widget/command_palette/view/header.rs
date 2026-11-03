//! The title line and the search line.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::char_width;
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
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

        // The query, or the placeholder while it is empty, with the cursor
        // at the end of the query: over the placeholder's first letter,
        // which it keeps, as in Input (#1091)
        let input_x = x + 4;
        let input_width = text_end.saturating_sub(input_x);
        let field = EditField {
            text: &self.query,
            placeholder: &self.placeholder,
            cursor: Some(self.query.chars().count()),
            selection: None,
        };
        let scroll = field.scroll(0, input_width);
        let text_color = ctx.css_color(Color::WHITE);
        ctx.put_edit_field(
            input_x,
            current_y,
            input_width,
            &field,
            scroll,
            |kind, ch| {
                let mut cell = Cell::new(ch);
                (cell.fg, cell.bg) = match kind {
                    FieldCell::Cursor => (Some(self.bg_color), Some(Color::WHITE)),
                    // The empty-state grey keeps its own color
                    FieldCell::Placeholder => (Some(DISABLED_FG), Some(self.bg_color)),
                    FieldCell::Text | FieldCell::Selected => {
                        (Some(text_color), Some(self.bg_color))
                    }
                };
                cell
            },
        );

        self.draw_right(ctx, layout, current_y);
    }
}
