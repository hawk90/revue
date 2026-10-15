//! Drawing the field, its strength bar and its validation message

use super::{MaskedInput, ValidationState};
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::stack::{hstack, vstack, Stack};
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::Text;
use crate::widget::{RenderContext, View};

impl MaskedInput {
    /// The text inside the brackets: the masked value, or the placeholder
    /// while the value is empty, padded or cut to the field width
    fn field_text(&self) -> String {
        // Input field
        let display = if self.value.is_empty() {
            self.placeholder.clone().unwrap_or_default()
        } else {
            self.masked_display()
        };

        // Build input display with pre-allocated padding
        let width = self.width.unwrap_or(20) as usize;
        let display_w = display_width(&display);
        if display_w < width {
            let mut result = String::with_capacity(width);
            result.push_str(&display);
            result.extend(std::iter::repeat_n(' ', width - display_w));
            result
        } else {
            crate::utils::truncate_to_width(&display, width).to_owned()
        }
    }

    /// The field between the brackets: its text with the cursor when
    /// focused, and the reveal indicator when revealing is allowed
    fn field(&self, ctx: &RenderContext) -> Stack {
        let is_placeholder = self.value.is_empty() && self.placeholder.is_some();
        let padded = self.field_text();

        // Insert cursor if focused
        let display_with_cursor = if self.focused && !self.disabled {
            let cursor_pos = self.cursor.min(padded.chars().count());
            // Use iterators for O(n) instead of O(n²) with .chars().nth()
            let before: String = padded.chars().take(cursor_pos).collect();
            let cursor_char = padded.chars().skip(cursor_pos).next().unwrap_or(' ');
            let after: String = padded.chars().skip(cursor_pos + 1).collect();
            (before, cursor_char, after)
        } else {
            (padded.clone(), ' ', String::new())
        };

        // Render input box
        let mut input_text = if self.focused && !self.disabled {
            // A focused empty field still shows its placeholder in grey
            let around_cursor = |s: String| {
                let mut text = Text::new(s);
                if is_placeholder {
                    text = text.fg(PLACEHOLDER_FG);
                }
                if let Some(bg) = self.bg {
                    text = text.bg(bg);
                }
                text
            };
            hstack()
                .child(around_cursor(display_with_cursor.0))
                .child(
                    Text::new(display_with_cursor.1.to_string())
                        .bg(Color::WHITE)
                        .fg(Color::BLACK),
                )
                .child(around_cursor(display_with_cursor.2))
        } else {
            let mut text = Text::new(&padded);
            if is_placeholder {
                text = text.fg(PLACEHOLDER_FG);
            } else if self.disabled {
                text = text.fg(DISABLED_FG);
            } else if let Some(fg) = self.fg.or_else(|| ctx.css_color_if_set()) {
                // The typed value takes `color`. The placeholder, the disabled
                // grey and the strength scale keep theirs - the scale in
                // particular runs red to green, and one `color` cannot say that.
                text = text.fg(fg);
            }
            if let Some(bg) = self.bg {
                text = text.bg(bg);
            }
            hstack().child(text)
        };

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
        if self.show_strength && !self.value.is_empty() {
            content = content.child(self.strength_bar());
        }

        // Validation message
        if let ValidationState::Invalid(msg) = &self.validation {
            content = content.child(Text::new(msg).fg(Color::RED));
        }

        content.render(ctx);
    }
}
