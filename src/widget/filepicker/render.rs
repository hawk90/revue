//! Rendering implementation for the FilePicker widget

use super::types::{PickerEntry, PickerMode};
use super::validation::truncate_string_safe;
use super::FilePicker;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::EditState;
use crate::widget::stack::Stack;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
use crate::widget::traits::{RenderContext, View};
use crate::widget::Text;

impl View for FilePicker {
    crate::impl_view_meta!("FilePicker", focusable);

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;

        let mut content = vstack();
        content = self.header(content);
        content = self.rows(content, ctx);
        content = self.footer(content);
        content.render(ctx);
        ctx.bold_when_focused();
    }
}

/// `Filename: ` and the name being typed, with its cursor.
struct NameField {
    name: EditState,
    width: u16,
}

impl View for NameField {
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((self.width.min(max_width), 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        const LABEL: &str = "Filename: ";
        let from = LABEL.len() as u16;
        let width = ctx.area.width.saturating_sub(from);
        ctx.draw_text(0, 0, LABEL, Color::YELLOW);
        let field = EditField::of(&self.name, "", true);
        let scroll = field.scroll(0, width);
        let selected = crate::widget::traits::render_context::edit_line::selection_colors();
        ctx.put_edit_field(from, 0, width, &field, scroll, |kind, ch| match kind {
            FieldCell::Cursor => Cell::new(ch).fg(Color::BLACK).bg(Color::WHITE),
            FieldCell::Selected => Cell::new(ch).fg(selected.0).bg(selected.1),
            FieldCell::Text | FieldCell::Placeholder => Cell::new(ch).fg(Color::YELLOW),
        });
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
    /// list goes on. The window is the rows the area has left after the
    /// header and footer, at most `max_visible`; when the entries overflow
    /// it, a row each is kept for the markers. It follows the highlight
    /// (#973).
    fn rows(&self, mut content: Stack, ctx: &RenderContext) -> Stack {
        let len = self.entries.len();
        let room = (ctx.area.height as usize).saturating_sub(self.fixed_rows());
        let window = if len <= room.min(self.max_visible) {
            len
        } else {
            self.max_visible.min(room.saturating_sub(2))
        };
        self.scroll.set_content(len);
        self.scroll.set_viewport(window);
        if len > 0 {
            self.scroll.track(self.highlighted..self.highlighted + 1);
        }
        let shown = self.scroll.visible();

        if self.scroll.has_above() {
            content = content.child(Text::new("  ↑ more...").fg(DISABLED_FG));
        }

        for i in shown.clone() {
            content = content.child(self.entry_row(i, ctx));
        }

        if shown.end < len {
            content = content.child(Text::new("  ↓ more...").fg(DISABLED_FG));
        }

        content
    }

    /// The rows the header and footer take
    fn fixed_rows(&self) -> usize {
        let header = usize::from(self.title.is_some()) + 3;
        let footer = 2
            + usize::from(self.mode == PickerMode::Save)
            + usize::from(self.mode == PickerMode::MultiSelect && !self.selected.is_empty());
        header + footer
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
            content = content.child(NameField {
                name: self.input_name.clone(),
                width: self.width,
            });
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
                "↑↓: Navigate | Enter: Select/Open | Backspace: Parent | h: Hidden | q/Esc: Cancel"
            }
            PickerMode::Save => {
                "↑↓: Navigate | Enter: Save | Type: Filename | Backspace: Delete | Esc: Cancel"
            }
            PickerMode::Directory => {
                "↑↓: Navigate | Enter: Open | Space: Select | Backspace: Parent | q/Esc: Cancel"
            }
            PickerMode::MultiSelect => {
                "↑↓: Navigate | Space: Toggle | Enter: Open/Confirm | a: All | n: None | q/Esc: Cancel"
            }
        }
    }
}
