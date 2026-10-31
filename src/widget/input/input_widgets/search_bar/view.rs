//! Drawing: the frame and icon, the text field with the cursor and the
//! error mark, and the hint line

use super::SearchBar;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
use crate::widget::traits::{RenderContext, View};

impl View for SearchBar {
    /// Its width, one row, and a row for the hint while one shows.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let rows = 1 + u16::from(self.hint().is_some());
        Some((self.width.min(max_width), rows.min(max_height)))
    }
    fn render(&self, ctx: &mut RenderContext) {
        // The query text takes `color`; the placeholder and border keep theirs.
        let text_color = self
            .text_color
            .unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let area = ctx.area;
        let width = self.width.min(area.width);

        if width < 5 || area.height < 1 {
            return;
        }

        self.draw_frame(ctx, width);
        self.draw_field(ctx, width, text_color);
        self.draw_hints(ctx, width, area.height);
    }

    crate::impl_view_meta!("SearchBar", focusable);
}

impl SearchBar {
    /// The background, the borders and the icon on row 0
    fn draw_frame(&self, ctx: &mut RenderContext, width: u16) {
        // Draw background
        ctx.fill_row(0, width, None, Some(self.bg_color));

        // Draw border (left and right)
        let mut left_border = Cell::new('│');
        left_border.fg = Some(if self.focused {
            Color::CYAN
        } else {
            self.border_color
        });
        ctx.set(0, 0, left_border);

        let mut right_border = Cell::new('│');
        right_border.fg = Some(if self.focused {
            Color::CYAN
        } else {
            self.border_color
        });
        ctx.set(width - 1, 0, right_border);

        // Draw search icon (an emoji by default: two columns)
        ctx.put_edit_line(2, 0, &self.icon.to_string(), 0, 2, |_, ch| {
            let mut cell = Cell::new(ch);
            cell.bg = Some(self.bg_color);
            cell
        });
    }

    /// The placeholder or the query, the cursor, and the error mark
    fn draw_field(&self, ctx: &mut RenderContext, width: u16, text_color: Color) {
        // The field runs from x 4 to the column before the error mark, so
        // the mark never covers text or the cursor, and the text does not
        // shift when the query turns invalid (#1014). Columns: a wide glyph
        // takes two cells; scroll so the cursor stays in view.
        let input_x: u16 = 4;
        let input_width = width.saturating_sub(7);
        let input_fg = if self.parse_error.is_some() {
            self.error_color
        } else {
            text_color
        };
        // The cursor goes over the placeholder's first letter, as in Input,
        // and keeps it (#1014)
        let field = EditField::of(&self.state, &self.placeholder, self.focused);
        let scroll = field.scroll(self.scroll_x.get(), input_width);
        self.scroll_x.set(scroll);
        ctx.put_edit_field(input_x, 0, input_width, &field, scroll, |kind, ch| {
            let mut cell = Cell::new(ch);
            (cell.fg, cell.bg) = match kind {
                FieldCell::Cursor => (Some(self.bg_color), Some(text_color)),
                FieldCell::Selected => (Some(self.bg_color), Some(self.placeholder_color)),
                FieldCell::Placeholder => (Some(self.placeholder_color), Some(self.bg_color)),
                FieldCell::Text => (Some(input_fg), Some(self.bg_color)),
            };
            cell
        });

        // Draw error indicator, past the field
        if self.parse_error.is_some() {
            let mut error_cell = Cell::new('!');
            error_cell.fg = Some(self.error_color);
            error_cell.bg = Some(self.bg_color);
            error_cell.modifier |= Modifier::BOLD;
            ctx.set(width - 3, 0, error_cell);
        }
    }

    /// The hint to show below the field: only while focused, and only when
    /// there is something to say.
    fn hint(&self) -> Option<&'static str> {
        if !self.show_hints || !self.focused {
            None
        } else if self.parse_error.is_some() {
            Some("Invalid query syntax")
        } else if self.get_input().is_empty() {
            Some("Try: field:value, text~contains, age:>18")
        } else {
            None
        }
    }

    /// The hint on row 1, while focused
    fn draw_hints(&self, ctx: &mut RenderContext, width: u16, height: u16) {
        // Draw hints (on second line if available)
        if height > 1 {
            if let Some(hint) = self.hint() {
                for (i, ch) in hint.chars().enumerate() {
                    if i as u16 >= width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.parse_error.is_some() {
                        self.error_color
                    } else {
                        self.placeholder_color
                    });
                    ctx.set(i as u16, 1, cell);
                }
            }
        }
    }
}
