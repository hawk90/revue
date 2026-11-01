//! Rendering implementation for the Input widget

use super::Input;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
use crate::widget::traits::{RenderContext, View};

impl View for Input {
    /// One row, as wide as it is offered: the field scrolls its text, so
    /// its width is the layout's to choose.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, 1.min(max_height)))
    }

    /// It stretches across the width it is offered.
    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // An empty input shows its placeholder whether or not it is focused,
        // as browsers and most toolkits do. `Input::new()` starts focused, so
        // hiding the placeholder while focused meant it never showed at all.
        // When focused, the cursor is drawn over the first placeholder cell,
        // so it stays visible.
        let (css_fg, css_bg) = self.text_colors(ctx);

        // Scrolled by columns (a wide glyph takes two) so the cursor, and
        // the whole glyph under it, stays in view; the scroll follows the
        // cursor focused or not
        let scroll = EditField::of(&self.state, &self.placeholder, true)
            .scroll(self.scroll_x.get(), area.width);
        self.scroll_x.set(scroll);

        let field = EditField::of(&self.state, &self.placeholder, self.focused);
        let selected = crate::widget::traits::render_context::edit_line::selection_colors();
        ctx.put_edit_field(0, 0, area.width, &field, scroll, |kind, ch| {
            let mut cell = Cell::new(ch);
            (cell.fg, cell.bg) = match kind {
                FieldCell::Cursor => (self.cursor_fg, self.cursor_bg),
                FieldCell::Selected => (Some(selected.0), self.selection_bg.or(Some(selected.1))),
                FieldCell::Placeholder => (Some(PLACEHOLDER_FG), None),
                FieldCell::Text => (css_fg, css_bg),
            };
            cell
        });
    }

    crate::impl_view_meta!("Input", focusable);
}

impl Input {
    /// The text's fg and bg: the builder's, else the stylesheet's
    /// `color` and `background`, else none
    fn text_colors(&self, ctx: &RenderContext) -> (Option<Color>, Option<Color>) {
        let css_fg = self.fg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.color;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        let css_bg = self.bg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.background;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        (css_fg, css_bg)
    }
}
