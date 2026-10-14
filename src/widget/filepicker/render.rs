//! Rendering implementation for the FilePicker widget

use super::types::{PickerEntry, PickerMode};
use super::validation::truncate_string_safe;
use super::FilePicker;
use crate::style::Color;
use crate::widget::stack::Stack;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
use crate::widget::traits::{RenderContext, View};
use crate::widget::Text;

impl View for FilePicker {
    crate::impl_view_meta!("FilePicker");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;

        let mut content = vstack();
        content = self.header(content);
        content = self.rows(content, ctx);
        content = self.footer(content);
        content.render(ctx);
    }
}

impl FilePicker {
    /// The title, the current path, a separator and the parent row.
    fn header(&self, mut content: Stack) -> Stack {
        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        content = content.child(Text::new(format!(" {}", self.shown_path())).fg(Color::CYAN));

        // Separator
        content = content.child(Text::new("─".repeat(self.width as usize)).fg(DARK_GRAY));

        // Parent directory option
        content.child(Text::new("  📁 ..").fg(LIGHT_GRAY))
    }

    /// Current path - truncate from the left to show the end of the path
    fn shown_path(&self) -> String {
        let path_str = self.current_dir.display().to_string();
        let max_path_width = (self.width as usize).saturating_sub(4);
        if crate::utils::display_width(&path_str) > max_path_width {
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
        }
    }

    /// The entries in view, with a "more" marker above and below when the
    /// list goes on.
    fn rows(&self, mut content: Stack, ctx: &RenderContext) -> Stack {
        // File list
        let start = self.scroll_offset;
        let end = (start + self.max_visible).min(self.entries.len());

        if start > 0 {
            content = content.child(Text::new("  ↑ more...").fg(DISABLED_FG));
        }

        for i in start..end {
            content = content.child(self.entry_row(i, ctx));
        }

        if end < self.entries.len() {
            content = content.child(Text::new("  ↓ more...").fg(DISABLED_FG));
        }

        content
    }

    /// One entry: its selection mark, icon, name and size.
    fn entry_row(&self, i: usize, ctx: &RenderContext) -> Text {
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

        let mut text = Text::new(&line).fg(self.entry_fg(entry, is_highlighted, ctx));
        if is_highlighted {
            text = text.bold();
        }

        text
    }

    /// The color of an entry's row.
    fn entry_fg(&self, entry: &PickerEntry, is_highlighted: bool, ctx: &RenderContext) -> Color {
        if is_highlighted {
            Color::CYAN
        } else if entry.is_dir {
            self.dir_fg.unwrap_or(Color::BLUE)
        } else if entry.is_hidden {
            self.hidden_fg.unwrap_or(PLACEHOLDER_FG)
        } else {
            // The highlighted row, directories and hidden files each say
            // something one `color` cannot; the plain file is the base.
            self.fg
                .or_else(|| ctx.css_color_if_set())
                .unwrap_or(Color::WHITE)
        }
    }

    /// A separator, the typed name (save mode), the selection count
    /// (multi-select) and the help line.
    fn footer(&self, mut content: Stack) -> Stack {
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

        content.child(Text::new(self.help()).fg(DARK_GRAY))
    }

    /// The help line for the mode.
    fn help(&self) -> &'static str {
        match self.mode {
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
        }
    }
}
