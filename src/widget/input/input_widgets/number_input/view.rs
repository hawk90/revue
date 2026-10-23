//! Drawing: the prefix, the value or the edit buffer with its cursor,
//! the suffix and the buttons

use super::NumberInput;
use crate::impl_view_meta;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::theme::{DARK_GRAY, LIGHT_GRAY, SEPARATOR_COLOR};
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
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

        // Draw the value, or the text being typed with its cursor and
        // selection, as wide as it is (and a cell for the cursor past its
        // end), scrolled to the cursor when the field is narrower
        let formatted = self.format_value();
        let field = if self.editing {
            EditField::of(&self.edit, "", self.state.focused)
        } else {
            EditField {
                text: &formatted,
                placeholder: "",
                cursor: None,
                selection: None,
            }
        };
        let past_end = field
            .cursor
            .is_some_and(|c| c >= field.text.chars().count());
        let text_w = display_width(field.text) + usize::from(past_end);
        let w = (text_w.min(u16::MAX as usize) as u16).min(field_width.saturating_sub(x));
        let scroll = field.scroll(self.scroll_x.get(), w);
        self.scroll_x.set(scroll);
        ctx.put_edit_field(x, 0, w, &field, scroll, |kind, ch| {
            let mut cell = Cell::new(ch);
            (cell.fg, cell.bg) = match kind {
                FieldCell::Cursor => (Some(Color::BLACK), Some(Color::WHITE)),
                FieldCell::Selected => (Some(Color::WHITE), Some(Color::BLUE)),
                FieldCell::Text | FieldCell::Placeholder => (Some(fg), Some(bg)),
            };
            cell
        });
        x += w;

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
