//! Rendering implementation for the FilePicker widget

use super::types::PickerMode;
use super::validation::truncate_string_safe;
use super::FilePicker;
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
use crate::widget::traits::{RenderContext, View};
use crate::widget::Text;

impl View for FilePicker {
    crate::impl_view_meta!("FilePicker");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Current path - truncate from the left to show the end of the path
        let path_str = self.current_dir.display().to_string();
        let max_path_width = self.width as usize - 4;
        let truncated_path = if crate::utils::display_width(&path_str) > max_path_width {
            let suffix_width = max_path_width.saturating_sub(3); // "..." prefix
                                                                 // Find suffix that fits by iterating from the end
            let chars: Vec<char> = path_str.chars().collect();
            let mut w = 0;
            let mut start = chars.len();
            for i in (0..chars.len()).rev() {
                let cw = crate::utils::char_width(chars[i]);
                if w + cw > suffix_width {
                    break;
                }
                w += cw;
                start = i;
            }
            let suffix: String = chars[start..].iter().collect();
            format!("...{}", suffix)
        } else {
            path_str
        };
        content = content.child(Text::new(format!(" {}", truncated_path)).fg(Color::CYAN));

        // Separator
        content = content.child(Text::new("─".repeat(self.width as usize)).fg(DARK_GRAY));

        // Parent directory option
        content = content.child(Text::new("  📁 ..").fg(LIGHT_GRAY));

        // File list
        let start = self.scroll_offset;
        let end = (start + self.max_visible).min(self.entries.len());

        if start > 0 {
            content = content.child(Text::new("  ↑ more...").fg(DISABLED_FG));
        }

        for i in start..end {
            let entry = &self.entries[i];
            let is_highlighted = i == self.highlighted;

            let icon = if entry.is_dir { "📁" } else { "📄" };
            let selected_mark = if entry.selected { "✓ " } else { "  " };
            // Truncate name safely at UTF-8 character boundaries
            let name = if entry.name.len() > 30 {
                truncate_string_safe(&entry.name, 27)
            } else {
                entry.name.clone()
            };

            let size = if entry.is_dir {
                String::new()
            } else {
                entry.format_size()
            };

            let line = format!("{}{} {:<32} {:>10}", selected_mark, icon, name, size);

            let fg = if is_highlighted {
                Color::CYAN
            } else if entry.is_dir {
                self.dir_fg.unwrap_or(Color::BLUE)
            } else if entry.is_hidden {
                self.hidden_fg.unwrap_or(PLACEHOLDER_FG)
            } else {
                self.fg.unwrap_or(Color::WHITE)
            };

            let mut text = Text::new(&line).fg(fg);
            if is_highlighted {
                text = text.bold();
            }

            content = content.child(text);
        }

        if end < self.entries.len() {
            content = content.child(Text::new("  ↓ more...").fg(DISABLED_FG));
        }

        // Separator
        content = content.child(Text::new("─".repeat(self.width as usize)).fg(DARK_GRAY));

        // Filename input (for save mode)
        if self.mode == PickerMode::Save {
            let input_display = format!("Filename: {}_", self.input_name);
            content = content.child(Text::new(input_display).fg(Color::YELLOW));
        }

        // Selection count (for multi-select)
        if self.mode == PickerMode::MultiSelect && !self.selected.is_empty() {
            content = content.child(
                Text::new(format!("Selected: {} files", self.selected.len())).fg(Color::GREEN),
            );
        }

        // Help
        let help = match self.mode {
            PickerMode::Open => {
                "↑↓: Navigate | Enter: Select/Open | Backspace: Parent | h: Hidden | q: Cancel"
            }
            PickerMode::Save => {
                "↑↓: Navigate | Enter: Save | Type: Filename | Backspace: Delete | q: Cancel"
            }
            PickerMode::Directory => {
                "↑↓: Navigate | Enter: Open | Space: Select | Backspace: Parent | q: Cancel"
            }
            PickerMode::MultiSelect => {
                "↑↓: Navigate | Space: Toggle | Enter: Confirm | a: All | n: None | q: Cancel"
            }
        };
        content = content.child(Text::new(help).fg(DARK_GRAY));

        content.render(ctx);
    }
}
