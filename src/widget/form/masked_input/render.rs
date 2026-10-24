//! Drawing the field, its strength bar and its validation message

use super::{MaskedInput, ValidationState};
use crate::style::Color;
use crate::utils::char_width;
use crate::widget::stack::{hstack, vstack, Stack};
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::traits::render_context::edit_line::{visible_from, EditField, FieldCell};
use crate::widget::Text;
use crate::widget::{RenderContext, View};

impl MaskedInput {
    /// The field's cells, `width` columns: the masked value (or the
    /// placeholder while it is empty) scrolled to keep the cursor in view,
    /// then blanks. Each glyph comes with what it shows; a wide glyph cut
    /// by an edge of the view becomes blanks.
    fn field_cells(&self) -> Vec<(char, FieldCell)> {
        let width = self.width.unwrap_or(20);
        let masked = self.masked_display();
        let placeholder = self.placeholder.as_deref().unwrap_or("");
        let editing = self.focused && !self.disabled;
        // The cursor and selection, turned into char indices of the masked
        // text, where a masked character is one char. Hidden masks to
        // nothing, so its cursor is drawn at the start whatever its index,
        // and does not give the length away.
        let mut field = EditField::of(&self.state, placeholder, editing);
        field.text = &masked;
        field.cursor = editing.then(|| self.masked_index(self.state.cursor().col));
        field.selection = field.selection.map(|_| {
            let (from, to) = self.state.primary().range();
            (self.masked_index(from.col), self.masked_index(to.col))
        });
        // A zero-width char takes no cell: the cursor on one is drawn on
        // the next glyph
        field.cursor = field.cursor.map(|c| visible_from(&masked, c));
        let scroll = field.scroll(self.scroll_x.get(), width);
        self.scroll_x.set(scroll);

        let shows_placeholder = self.get_value().is_empty() && !placeholder.is_empty();
        let (text, scroll) = if shows_placeholder {
            (placeholder, 0)
        } else {
            (field.text, scroll)
        };
        let end = scroll + width as usize;
        let mut cells = Vec::with_capacity(width as usize);
        let mut col = 0;
        for (i, ch) in text.chars().enumerate() {
            let w = char_width(ch);
            if w == 0 {
                continue;
            }
            let next = col + w;
            let kind = if field.cursor == Some(i) {
                FieldCell::Cursor
            } else if shows_placeholder {
                FieldCell::Placeholder
            } else if field.selection.is_some_and(|(a, b)| a <= i && i < b) {
                FieldCell::Selected
            } else {
                FieldCell::Text
            };
            if col >= scroll && next <= end {
                cells.push((ch, kind));
            } else if next > scroll {
                // Cut by an edge: the part in view, as blanks
                let from = col.max(scroll);
                cells.extend(std::iter::repeat_n((' ', kind), next.min(end) - from));
            }
            if next >= end {
                break;
            }
            col = next;
        }
        // The rest of the field: blanks, the first one the cursor when it
        // is past the end of the text
        let used: usize = cells.iter().map(|&(c, _)| char_width(c)).sum();
        let past_end =
            !shows_placeholder && field.cursor.is_some_and(|c| c >= text.chars().count());
        for i in used..width as usize {
            let kind = if past_end && i == used {
                FieldCell::Cursor
            } else {
                FieldCell::Text
            };
            cells.push((' ', kind));
        }
        cells
    }

    /// The field between the brackets: its text with the cursor when
    /// focused, and the reveal indicator when revealing is allowed
    fn field(&self, ctx: &RenderContext) -> Stack {
        let text_fg = if self.disabled {
            Some(DISABLED_FG)
        } else {
            // The typed value takes `color`. The placeholder, the disabled
            // grey and the strength scale keep theirs - the scale in
            // particular runs red to green, and one `color` cannot say that.
            self.fg.or_else(|| ctx.css_color_if_set())
        };
        let style = |kind: FieldCell| match kind {
            FieldCell::Cursor => (Some(Color::BLACK), Some(Color::WHITE)),
            FieldCell::Selected => (Some(Color::WHITE), Some(Color::BLUE)),
            FieldCell::Placeholder => (Some(PLACEHOLDER_FG), self.bg),
            FieldCell::Text => (text_fg, self.bg),
        };

        // One Text per run of cells drawn alike
        let mut input_text = hstack();
        let mut run = String::new();
        let mut run_kind = None;
        let flush = |stack: Stack, run: &mut String, kind: Option<FieldCell>| match kind {
            Some(kind) if !run.is_empty() => {
                let (fg, bg) = style(kind);
                let mut text = Text::new(std::mem::take(run));
                if let Some(fg) = fg {
                    text = text.fg(fg);
                }
                if let Some(bg) = bg {
                    text = text.bg(bg);
                }
                stack.child(text)
            }
            _ => stack,
        };
        for (ch, kind) in self.field_cells() {
            if run_kind != Some(kind) {
                input_text = flush(input_text, &mut run, run_kind);
                run_kind = Some(kind);
            }
            run.push(ch);
        }
        input_text = flush(input_text, &mut run, run_kind);

        // Add reveal indicator
        if self.allow_reveal {
            let eye = if self.revealing {
                "👁"
            } else {
                "👁‍🗨"
            };
            input_text = input_text.child(Text::new(format!(" {}", eye)));
        }

        input_text
    }

    /// The bracket color: gray when disabled, then the validation state's
    /// red or green, cyan when focused
    fn border_color(&self) -> Color {
        if self.disabled {
            DARK_GRAY
        } else if matches!(self.validation, ValidationState::Invalid(_)) {
            Color::RED
        } else if matches!(self.validation, ValidationState::Valid) {
            Color::GREEN
        } else if self.focused {
            Color::CYAN
        } else {
            PLACEHOLDER_FG
        }
    }

    /// The password strength bar: five cells filled up to the strength,
    /// then its label, in the strength color
    fn strength_bar(&self) -> Stack {
        let strength = self.password_strength();
        let color = self.strength_color();
        // Pre-allocate strength bar (max 5 chars = strength + 1)
        let bar: String = std::iter::repeat_n('█', strength + 1).collect();
        let empty: String = std::iter::repeat_n('░', 4 - strength).collect();

        hstack()
            .child(Text::new(&bar).fg(color))
            .child(Text::new(&empty).fg(DARK_GRAY))
            .child(Text::new(format!(" {}", self.strength_label())).fg(color))
    }
}

impl View for MaskedInput {
    crate::impl_view_meta!("MaskedInput");

    fn render(&self, ctx: &mut RenderContext) {
        let mut content = vstack();

        // Label
        if let Some(label) = &self.label {
            content = content.child(Text::new(label).bold());
        }

        // Wrap in border
        let border_color = self.border_color();

        let bordered = hstack()
            .child(Text::new("[").fg(border_color))
            .child(self.field(ctx))
            .child(Text::new("]").fg(border_color));

        content = content.child(bordered);

        // Password strength indicator
        if self.show_strength && !self.get_value().is_empty() {
            content = content.child(self.strength_bar());
        }

        // Validation message
        if let ValidationState::Invalid(msg) = &self.validation {
            content = content.child(Text::new(msg).fg(Color::RED));
        }

        content.render(ctx);
    }
}
