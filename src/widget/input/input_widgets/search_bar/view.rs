//! Drawing: the frame and icon, the text field with the cursor and the
//! error mark, and the hint line

use super::SearchBar;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::render_context::edit_line::{col_of, cursor_width, scroll_to_cursor};
use crate::widget::traits::{RenderContext, View};

impl View for SearchBar {
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
        let input = self.get_input();
        let cursor = self.cursor_char();

        if input.is_empty() {
            // The cursor goes over the placeholder's first letter, as in
            // Input, and keeps it (#1014)
            ctx.put_edit_line(input_x, 0, &self.placeholder, 0, input_width, |i, ch| {
                let mut cell = Cell::new(ch);
                if self.focused && i == 0 {
                    cell.fg = Some(self.bg_color);
                    cell.bg = Some(text_color);
                } else {
                    cell.fg = Some(self.placeholder_color);
                    cell.bg = Some(self.bg_color);
                }
                cell
            });
        }

        let scroll = scroll_to_cursor(
            0,
            col_of(input, cursor),
            cursor_width(input, cursor),
            col_of(input, usize::MAX) + 1,
            input_width as usize,
        );
        let input_fg = if self.parse_error.is_some() {
            self.error_color
        } else {
            text_color
        };
        ctx.put_edit_line(input_x, 0, input, scroll, input_width, |i, ch| {
            let mut cell = Cell::new(ch);
            if self.focused && i == cursor {
                cell.fg = Some(self.bg_color);
                cell.bg = Some(text_color);
            } else {
                cell.fg = Some(input_fg);
                cell.bg = Some(self.bg_color);
            }
            cell
        });

        // Draw the cursor past the end of the text (over the placeholder's
        // first letter when there is no text)
        let at_end = cursor >= input.chars().count();
        if self.focused && at_end && !(input.is_empty() && !self.placeholder.is_empty()) {
            let x = col_of(input, usize::MAX).saturating_sub(scroll);
            if x < input_width as usize {
                let mut cursor_cell = Cell::new(' ');
                cursor_cell.fg = Some(self.bg_color);
                cursor_cell.bg = Some(text_color);
                ctx.set(input_x + x as u16, 0, cursor_cell);
            }
        }

        // Draw error indicator, past the field
        if self.parse_error.is_some() {
            let mut error_cell = Cell::new('!');
            error_cell.fg = Some(self.error_color);
            error_cell.bg = Some(self.bg_color);
            error_cell.modifier |= Modifier::BOLD;
            ctx.set(width - 3, 0, error_cell);
        }
    }

    /// The hint on row 1, while focused
    fn draw_hints(&self, ctx: &mut RenderContext, width: u16, height: u16) {
        // Draw hints (on second line if available)
        if self.show_hints && height > 1 && self.focused {
            let hint = if self.parse_error.is_some() {
                "Invalid query syntax"
            } else if self.get_input().is_empty() {
                "Try: field:value, text~contains, age:>18"
            } else {
                ""
            };

            if !hint.is_empty() {
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
