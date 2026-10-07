//! Drawing the title, count, items and scroll indicators

use super::{SelectionList, SelectionStyle};
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::{RenderContext, View};

impl SelectionList {
    /// Get item prefix based on style
    fn item_prefix(&self, index: usize) -> String {
        let is_selected = self.is_selected(index);
        let is_disabled = self.items[index].disabled;

        match self.style {
            SelectionStyle::Checkbox => {
                if is_disabled {
                    "[-] ".to_string()
                } else if is_selected {
                    "[x] ".to_string()
                } else {
                    "[ ] ".to_string()
                }
            }
            SelectionStyle::Bullet => {
                if is_disabled {
                    "◌ ".to_string()
                } else if is_selected {
                    "● ".to_string()
                } else {
                    "○ ".to_string()
                }
            }
            SelectionStyle::Highlight => if is_selected { "▸ " } else { "  " }.to_string(),
            SelectionStyle::Bracket => {
                if is_selected {
                    "[".to_string()
                } else {
                    " ".to_string()
                }
            }
        }
    }

    /// Get item suffix based on style
    fn item_suffix(&self, index: usize) -> String {
        if self.style == SelectionStyle::Bracket && self.is_selected(index) {
            "]".to_string()
        } else {
            String::new()
        }
    }
}

impl View for SelectionList {
    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Selection count
        if self.show_count {
            let count_text = if self.max_selections > 0 {
                format!("Selected: {}/{}", self.selected.len(), self.max_selections)
            } else {
                format!("Selected: {}", self.selected.len())
            };
            content = content.child(Text::new(count_text).fg(PLACEHOLDER_FG));
        }

        // Calculate visible range
        let max_visible = if self.max_visible > 0 {
            self.max_visible
        } else {
            self.items.len()
        };

        let start = self.scroll_offset;
        let end = (start + max_visible).min(self.items.len());

        // Show scroll indicator at top
        if start > 0 {
            content = content.child(Text::new("  ↑ more...").fg(DISABLED_FG));
        }

        // Render items
        for i in start..end {
            let item = &self.items[i];
            let prefix = self.item_prefix(i);
            let suffix = self.item_suffix(i);

            let icon = item.icon.as_deref().unwrap_or("");
            let text = format!("{}{}{}{}", prefix, icon, item.text, suffix);

            let is_highlighted = i == self.highlighted && self.focused;
            let is_selected = self.is_selected(i);

            let fg = if item.disabled {
                DISABLED_FG
            } else if is_highlighted {
                self.highlighted_fg.unwrap_or(Color::CYAN)
            } else if is_selected {
                self.selected_fg.unwrap_or(Color::GREEN)
            } else {
                self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE))
            };

            let mut text_widget = Text::new(&text).fg(fg);
            if let Some(bg) = self.bg {
                text_widget = text_widget.bg(bg);
            }

            if is_highlighted {
                text_widget = text_widget.bold();
            }

            content = content.child(text_widget);

            // Show description
            if self.show_descriptions {
                if let Some(desc) = &item.description {
                    let desc_text = format!("    {}", desc);
                    let mut desc_widget = Text::new(desc_text).fg(PLACEHOLDER_FG);
                    if let Some(bg) = self.bg {
                        desc_widget = desc_widget.bg(bg);
                    }
                    content = content.child(desc_widget);
                }
            }
        }

        // Show scroll indicator at bottom
        if end < self.items.len() {
            content = content.child(Text::new("  ↓ more...").fg(DISABLED_FG));
        }

        // Help text
        if self.focused {
            content = content
                .child(Text::new("↑↓: Navigate | Space: Toggle | a: All | n: None").fg(DARK_GRAY));
        }

        content.render(ctx);
    }

    crate::impl_view_meta!("SelectionList", focusable);
}
