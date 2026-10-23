//! Autocomplete `View`: the input row with the value or placeholder and the
//! cursor, then the matching suggestions below it

use crate::render::Cell;
use crate::style::Color;
use crate::utils::fuzzy_match;
use crate::widget::traits::render_context::edit_line::{col_of, EditField, FieldCell};
use crate::widget::traits::{RenderContext, View};

use super::Autocomplete;

impl Autocomplete {
    /// The input row: the value (or placeholder), scrolled to keep the
    /// cursor in view; returns the input text color the dropdown reuses
    fn render_input(&self, ctx: &mut RenderContext) -> Color {
        let area = ctx.area;

        // Render input box
        let input_width = area.width;
        for x in 0..input_width {
            ctx.set(x, 0, Cell::new(' ').bg(self.input_bg));
        }

        // Resolved once: the builder's color if it named one, else the
        // stylesheet's, else white.
        let input_fg = self.input_fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));

        // The value (or the placeholder), scrolled to keep the cursor in
        // view while focused. Unfocused, the value shows from its start -
        // the cursor is not drawn, and the start is what identifies the
        // value. The typed value takes `color`; the placeholder keeps its
        // own.
        let field = EditField::of(&self.state, &self.placeholder, self.focused);
        let scroll = field.scroll(self.scroll_x.get(), input_width);
        self.scroll_x.set(scroll);
        ctx.put_edit_field(0, 0, input_width, &field, scroll, |kind, ch| match kind {
            FieldCell::Cursor => Cell::new(ch).fg(self.input_bg).bg(input_fg),
            FieldCell::Selected => Cell::new(ch).fg(self.selected_fg).bg(self.selected_bg),
            FieldCell::Placeholder => Cell::new(ch).fg(self.placeholder_fg).bg(self.input_bg),
            FieldCell::Text => Cell::new(ch).fg(input_fg).bg(self.input_bg),
        });
        input_fg
    }

    /// The matching suggestions, one per row below the input
    fn render_dropdown(&self, ctx: &mut RenderContext, input_fg: Color) {
        let area = ctx.area;
        let input_width = area.width;

        // Render dropdown if visible and there's room
        if self.dropdown_visible && area.height > 1 && !self.filtered.is_empty() {
            let dropdown_height = (self.filtered.len() as u16).min(area.height - 1);
            let dropdown_y: u16 = 1;

            // The rows in view: the dropdown follows the highlight (#882)
            let scroll = &self.dropdown_scroll;
            scroll.set_content(self.filtered.len());
            scroll.set_viewport(dropdown_height as usize);
            let highlighted = self.selection.index;
            scroll.track(highlighted..highlighted + 1);
            let shown = scroll.visible();

            for (row, (i, &suggestion_idx)) in self
                .filtered
                .iter()
                .enumerate()
                .skip(shown.start)
                .take(shown.len())
                .enumerate()
            {
                let suggestion = &self.suggestions[suggestion_idx];
                let y = dropdown_y + row as u16;
                let is_selected = i == highlighted;

                let (fg, bg) = if is_selected {
                    (self.selected_fg, self.selected_bg)
                } else {
                    (input_fg, self.dropdown_bg)
                };

                // Fill background
                for x in 0..input_width {
                    ctx.set(x, y, Cell::new(' ').bg(bg));
                }

                let mut x: u16 = 0;

                // Icon
                if let Some(icon) = suggestion.icon {
                    ctx.set(x, y, Cell::new(icon).fg(fg).bg(bg));
                    x += 2;
                }

                // Label with highlight (fuzzy indices are char indices), in
                // columns: a wide glyph takes two cells.
                let matched = fuzzy_match(self.get_value(), &suggestion.label)
                    .map(|fm| fm.indices)
                    .unwrap_or_default();
                let room = input_width.saturating_sub(x);
                ctx.put_edit_line(x, y, &suggestion.label, 0, room, |j, ch| {
                    let char_fg = if matched.contains(&j) {
                        self.highlight_fg
                    } else {
                        fg
                    };
                    Cell::new(ch).fg(char_fg).bg(bg)
                });
                x = x.saturating_add(col_of(&suggestion.label, usize::MAX) as u16);

                // Description (if fits)
                if let Some(ref desc) = suggestion.description {
                    x = x.saturating_add(1);
                    let room = input_width.saturating_sub(x);
                    ctx.put_edit_line(x, y, desc, 0, room, |_, ch| {
                        Cell::new(ch).fg(self.description_fg).bg(bg)
                    });
                }
            }
        }
    }
}

impl View for Autocomplete {
    crate::impl_view_meta!("Autocomplete", focusable);

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let input_fg = self.render_input(ctx);
        self.render_dropdown(ctx, input_fg);
    }
}
