//! Masking the value for display and drawing the field, strength bar and message

use super::{MaskStyle, MaskedInput, ValidationState};
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::{RenderContext, View};

impl MaskedInput {
    /// Get masked display string
    ///
    /// This method is optimized to minimize string allocations by:
    /// - Pre-allocating strings with known capacity
    /// - Avoiding repeated `.to_string().repeat()` calls
    /// - Using `extend` with char iterators instead of format!
    pub fn masked_display(&self) -> String {
        if self.revealing {
            return self.value.clone();
        }

        let len = self.char_count();
        if len == 0 {
            return String::new();
        }

        match self.mask_style {
            MaskStyle::Full => {
                // Pre-allocate with exact capacity
                std::iter::repeat_n(self.mask_char, len).collect()
            }
            MaskStyle::ShowLast(n) => {
                if len <= n {
                    self.value.clone()
                } else {
                    let mask_count = len - n;
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, mask_count));
                    result.push_str(&self.value[self.byte_at(len - n)..]);
                    result
                }
            }
            MaskStyle::ShowFirst(n) => {
                if len <= n {
                    self.value.clone()
                } else {
                    let mut result = String::with_capacity(len);
                    result.push_str(&self.value[..self.byte_at(n)]);
                    result.extend(std::iter::repeat_n(self.mask_char, len - n));
                    result
                }
            }
            MaskStyle::Peek => {
                if self.peek_countdown > 0 && self.cursor > 0 && self.cursor <= len {
                    // Show the last typed character
                    // Use char_indices for O(n) instead of O(n²) with .chars().nth()
                    let last_char = self
                        .value
                        .char_indices()
                        .nth(self.cursor - 1)
                        .map(|(_, c)| c)
                        .unwrap_or(' ');
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, self.cursor - 1));
                    result.push(last_char);
                    result.extend(std::iter::repeat_n(self.mask_char, len - self.cursor));
                    result
                } else {
                    std::iter::repeat_n(self.mask_char, len).collect()
                }
            }
            MaskStyle::Hidden => String::new(),
        }
    }
}

impl View for MaskedInput {
    crate::impl_view_meta!("MaskedInput");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::{hstack, vstack};
        use crate::widget::Text;

        let mut content = vstack();

        // Label
        if let Some(label) = &self.label {
            content = content.child(Text::new(label).bold());
        }

        // Input field
        let display = if self.value.is_empty() {
            self.placeholder.clone().unwrap_or_default()
        } else {
            self.masked_display()
        };

        let is_placeholder = self.value.is_empty() && self.placeholder.is_some();

        // Build input display with pre-allocated padding
        let width = self.width.unwrap_or(20) as usize;
        let display_w = display_width(&display);
        let padded = if display_w < width {
            let mut result = String::with_capacity(width);
            result.push_str(&display);
            result.extend(std::iter::repeat_n(' ', width - display_w));
            result
        } else {
            crate::utils::truncate_to_width(&display, width).to_owned()
        };

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
                let text = Text::new(s);
                if is_placeholder {
                    text.fg(PLACEHOLDER_FG)
                } else {
                    text
                }
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

        // Wrap in border
        let border_color = if self.disabled {
            DARK_GRAY
        } else if matches!(self.validation, ValidationState::Invalid(_)) {
            Color::RED
        } else if matches!(self.validation, ValidationState::Valid) {
            Color::GREEN
        } else if self.focused {
            Color::CYAN
        } else {
            PLACEHOLDER_FG
        };

        let bordered = hstack()
            .child(Text::new("[").fg(border_color))
            .child(input_text)
            .child(Text::new("]").fg(border_color));

        content = content.child(bordered);

        // Password strength indicator
        if self.show_strength && !self.value.is_empty() {
            let strength = self.password_strength();
            let color = self.strength_color();
            // Pre-allocate strength bar (max 5 chars = strength + 1)
            let bar: String = std::iter::repeat_n('█', strength + 1).collect();
            let empty: String = std::iter::repeat_n('░', 4 - strength).collect();

            let strength_display = hstack()
                .child(Text::new(&bar).fg(color))
                .child(Text::new(&empty).fg(DARK_GRAY))
                .child(Text::new(format!(" {}", self.strength_label())).fg(color));

            content = content.child(strength_display);
        }

        // Validation message
        if let ValidationState::Invalid(msg) = &self.validation {
            content = content.child(Text::new(msg).fg(Color::RED));
        }

        content.render(ctx);
    }
}
