//! Drawing the title, count, items and scroll indicators

use super::{SelectionList, SelectionStyle};
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::{RenderContext, Text, View};

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

    /// Item `i`'s row, and its description's row when shown
    fn item_rows(&self, i: usize, ctx: &RenderContext) -> Vec<Text> {
        let mut rows = Vec::with_capacity(2);
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

        rows.push(text_widget);

        // Show description
        if self.show_descriptions {
            if let Some(desc) = &item.description {
                let desc_text = format!("    {}", desc);
                let mut desc_widget = Text::new(desc_text).fg(PLACEHOLDER_FG);
                if let Some(bg) = self.bg {
                    desc_widget = desc_widget.bg(bg);
                }
                rows.push(desc_widget);
            }
        }
        rows
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

        // The item rows (an item with a description takes two), scrolled to
        // keep the highlighted item in view (#973). The window is the rows
        // the area has left, or `max_visible` items' worth; when the items
        // overflow it, a row each is kept for the "more" markers.
        let mut rows = Vec::new();
        let mut highlight = None;
        let mut tallest = 1;
        for i in 0..self.items.len() {
            let item_rows = self.item_rows(i, ctx);
            tallest = tallest.max(item_rows.len());
            if i == self.highlighted {
                highlight = Some(rows.len()..rows.len() + item_rows.len());
            }
            rows.extend(item_rows);
        }
        let fixed = usize::from(self.title.is_some())
            + usize::from(self.show_count)
            + usize::from(self.focused);
        let room = (ctx.area.height as usize).saturating_sub(fixed);
        let cap = if self.max_visible > 0 {
            self.max_visible.saturating_mul(tallest)
        } else {
            usize::MAX
        };
        let window = if rows.len() <= room.min(cap) {
            rows.len()
        } else {
            cap.min(room.saturating_sub(2)).max(1)
        };
        self.scroll.set_content(rows.len());
        self.scroll.set_viewport(window);
        if let Some(highlight) = highlight {
            self.scroll.track(highlight);
        }

        if self.scroll.has_above() {
            content = content.child(Text::new("  ↑ more...").fg(DISABLED_FG));
        }
        let shown = self.scroll.visible();
        let (above, below) = (shown.start, rows.len() - shown.end);
        for row in rows.into_iter().skip(above).take(shown.len()) {
            content = content.child(row);
        }
        if below > 0 {
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
