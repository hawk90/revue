//! Drawing: the prefix, the value or the edit buffer with its cursor,
//! the suffix and the buttons

use super::NumberInput;
use crate::impl_view_meta;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, LIGHT_GRAY, SEPARATOR_COLOR};
use crate::widget::traits::{RenderContext, View};

impl View for NumberInput {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }
        // The field spans its configured width, never more than the area.
        let field_width = self.width.map_or(area.width, |w| w.min(area.width));

        let (fg, bg) =
            self.state
                .resolve_colors_interactive(ctx.style, Color::WHITE, SEPARATOR_COLOR);

        let mut x: u16 = 0;

        // Draw prefix
        if let Some(ref prefix) = self.prefix {
            ctx.draw_text_clipped(x, 0, prefix, LIGHT_GRAY, field_width.saturating_sub(x));
            x = x.saturating_add(prefix.chars().count() as u16);
        }

        // Draw value (or input buffer when editing)
        let value_str = if self.editing {
            &self.input_buffer
        } else {
            &self.format_value()
        };

        for (i, ch) in value_str.chars().enumerate() {
            if x >= field_width {
                break;
            }

            let is_cursor_at_pos = self.editing && self.state.focused && i == self.cursor;

            let mut cell = Cell::new(ch);
            if is_cursor_at_pos {
                cell.fg = Some(Color::BLACK);
                cell.bg = Some(Color::WHITE);
            } else {
                cell.fg = Some(fg);
                cell.bg = Some(bg);
            }

            ctx.set(x, 0, cell);
            x += 1;
        }

        // Draw cursor at end if editing and cursor is at end
        if self.editing
            && self.state.focused
            && self.cursor >= value_str.chars().count()
            && x < field_width
        {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.fg = Some(Color::BLACK);
            cursor_cell.bg = Some(Color::WHITE);
            ctx.set(x, 0, cursor_cell);
            x += 1;
        }

        // Draw suffix
        if let Some(ref suffix) = self.suffix {
            ctx.draw_text_clipped(x, 0, suffix, LIGHT_GRAY, field_width.saturating_sub(x));
            x = x.saturating_add(suffix.chars().count() as u16);
        }

        // Draw buttons if enabled and space available
        if self.show_buttons && x.saturating_add(4) <= field_width {
            let button_x = field_width - 4;
            let button_fg = if self.state.disabled {
                DARK_GRAY
            } else {
                LIGHT_GRAY
            };
            ctx.draw_text(button_x, 0, " -+", button_fg);
        }
    }

    impl_view_meta!("NumberInput", focusable, disabled: state);
}
