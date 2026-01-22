//! Code editor rendering

use crate::render::Cell;
use crate::style::Color;
use crate::widget::code_editor::CodeEditor;
use crate::widget::traits::{RenderContext, View};

impl CodeEditor {
    // =========================================================================
    // Rendering Helpers
    // =========================================================================

    /// Get line number width
    pub(super) fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let digits = format!("{}", self.lines.len()).len();
            (digits + 2) as u16
        } else {
            0
        }
    }

    /// Get syntax highlights for a line
    pub(super) fn get_highlights(&self, line: &str) -> Vec<crate::widget::syntax::HighlightSpan> {
        self.highlighter
            .as_ref()
            .map(|h| h.highlight_line(line))
            .unwrap_or_default()
    }

    /// Check if position is in selection
    pub(super) fn is_selected(&self, line: usize, col: usize) -> bool {
        let anchor = match self.anchor {
            Some(a) => a,
            None => return false,
        };

        let (start, end) = if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        };

        if line < start.0 || line > end.0 {
            return false;
        }
        if line == start.0 && line == end.0 {
            col >= start.1 && col < end.1
        } else if line == start.0 {
            col >= start.1
        } else if line == end.0 {
            col < end.1
        } else {
            true
        }
    }

    /// Check if position is in a find match
    pub(super) fn get_find_match_at(&self, line: usize, col: usize) -> Option<(bool, usize)> {
        for (idx, &(m_line, m_start, m_end)) in self.find_matches.iter().enumerate() {
            if m_line == line && col >= m_start && col < m_end {
                return Some((idx == self.find_index, idx));
            }
        }
        None
    }
}

impl View for CodeEditor {
    crate::impl_view_meta!("CodeEditor");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let line_num_width = self.line_number_width();
        let minimap_width = if self.config.show_minimap {
            self.config.minimap_width
        } else {
            0
        };
        let text_width = area.width.saturating_sub(line_num_width + minimap_width);
        let visible_lines = area.height as usize;

        // Find matching bracket
        let bracket_match = self.find_matching_bracket();

        // Draw background
        if let Some(bg) = self.bg {
            for y in 0..area.height {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(area.x + x, area.y + y, cell);
                }
            }
        }

        // Render visible lines
        let start_line = self.scroll.0;
        let end_line = (start_line + visible_lines).min(self.lines.len());

        for (view_row, line_idx) in (start_line..end_line).enumerate() {
            let y = area.y + view_row as u16;
            let line = &self.lines[line_idx];
            let is_current_line = line_idx == self.cursor.0;

            // Current line highlight
            if self.config.highlight_current_line && is_current_line && self.focused {
                for x in line_num_width..line_num_width + text_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.current_line_bg);
                    ctx.buffer.set(area.x + x, y, cell);
                }
            }

            // Draw line numbers
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>width$} ",
                    line_idx + 1,
                    width = (line_num_width - 2) as usize
                );
                for (i, ch) in num_str.chars().enumerate() {
                    if (i as u16) < line_num_width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(if is_current_line && self.focused {
                            Color::WHITE
                        } else {
                            self.line_number_fg
                        });
                        cell.bg = self.bg;
                        ctx.buffer.set(area.x + i as u16, y, cell);
                    }
                }
            }

            // Get syntax highlights
            let highlights = self.get_highlights(line);

            // Draw text
            let chars: Vec<char> = line.chars().collect();
            let scroll_col = self.scroll.1;

            for (view_col, char_idx) in (scroll_col..scroll_col + text_width as usize).enumerate() {
                let x = area.x + line_num_width + view_col as u16;
                if x >= area.x + area.width - minimap_width {
                    break;
                }

                let ch = chars.get(char_idx).copied().unwrap_or(' ');
                let mut cell = Cell::new(ch);

                // Check cursor position
                let is_cursor =
                    self.focused && line_idx == self.cursor.0 && char_idx == self.cursor.1;

                // Check selection
                let is_selected = self.is_selected(line_idx, char_idx);

                // Check bracket match
                let is_bracket_match = bracket_match
                    .as_ref()
                    .map(|m| m.position == (line_idx, char_idx))
                    .unwrap_or(false);

                // Check find match
                let find_match = self.get_find_match_at(line_idx, char_idx);

                if is_cursor {
                    cell.bg = Some(self.cursor_bg);
                    cell.fg = Some(Color::BLACK);
                } else if is_selected {
                    cell.bg = Some(self.selection_bg);
                    cell.fg = self.fg;
                } else if is_bracket_match {
                    cell.bg = Some(self.bracket_match_bg);
                    cell.fg = Some(Color::BLACK);
                    cell.modifier |= crate::render::Modifier::BOLD;
                } else if let Some((is_current, _)) = find_match {
                    cell.bg = Some(if is_current {
                        self.current_find_bg
                    } else {
                        self.find_match_bg
                    });
                    cell.fg = Some(Color::BLACK);
                } else {
                    // Apply syntax highlighting
                    let mut fg_set = false;
                    for span in &highlights {
                        if char_idx >= span.start && char_idx < span.end {
                            cell.fg = Some(span.fg);
                            if span.bold {
                                cell.modifier |= crate::render::Modifier::BOLD;
                            }
                            if span.italic {
                                cell.modifier |= crate::render::Modifier::ITALIC;
                            }
                            fg_set = true;
                            break;
                        }
                    }
                    if !fg_set {
                        cell.fg = self.fg;
                    }
                    if self.config.highlight_current_line && is_current_line && self.focused {
                        cell.bg = Some(self.current_line_bg);
                    } else {
                        cell.bg = self.bg;
                    }
                }

                ctx.buffer.set(x, y, cell);
            }

            // Draw cursor at end of line if needed
            if self.focused && line_idx == self.cursor.0 && self.cursor.1 >= chars.len() {
                let cursor_x = area.x + line_num_width + (self.cursor.1 - scroll_col) as u16;
                if cursor_x < area.x + area.width - minimap_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.cursor_bg);
                    ctx.buffer.set(cursor_x, y, cell);
                }
            }
        }

        // Draw minimap if enabled
        if self.config.show_minimap && minimap_width > 0 {
            let minimap_x = area.x + area.width - minimap_width;

            // Minimap background
            for y in 0..area.height {
                for x in 0..minimap_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.minimap_bg);
                    ctx.buffer.set(minimap_x + x, area.y + y, cell);
                }
            }

            // Calculate minimap scale
            let total_lines = self.lines.len();
            let minimap_height = area.height as usize;
            let lines_per_row = (total_lines as f32 / minimap_height as f32).max(1.0);

            // Draw minimap content
            for row in 0..minimap_height {
                let start_line = (row as f32 * lines_per_row) as usize;
                let y = area.y + row as u16;

                // Highlight visible area
                if start_line >= self.scroll.0 && start_line < self.scroll.0 + visible_lines {
                    for x in 0..minimap_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(self.minimap_visible_bg);
                        ctx.buffer.set(minimap_x + x, y, cell);
                    }
                }

                // Draw condensed line representation
                if let Some(line) = self.lines.get(start_line) {
                    let chars: Vec<char> = line.chars().collect();
                    for (i, &ch) in chars.iter().take(minimap_width as usize).enumerate() {
                        if !ch.is_whitespace() {
                            let mut cell = Cell::new('▪');
                            cell.fg = Some(Color::rgb(128, 128, 128));
                            ctx.buffer.set(minimap_x + i as u16, y, cell);
                        }
                    }
                }
            }
        }

        // Draw go-to-line dialog
        if self.goto_line_mode {
            let dialog_width = 20u16;
            let dialog_x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
            let dialog_y = area.y;

            // Background
            for x in 0..dialog_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + x, dialog_y, cell);
            }

            // Label
            let label = "Go to line: ";
            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + i as u16, dialog_y, cell);
            }

            // Input
            for (i, ch) in self.goto_line_input.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(166, 227, 161));
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer
                    .set(dialog_x + label.len() as u16 + i as u16, dialog_y, cell);
            }
        }

        // Draw find dialog
        if self.find_mode {
            let dialog_width = 30u16;
            let dialog_x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
            let dialog_y = area.y;

            // Background
            for x in 0..dialog_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + x, dialog_y, cell);
            }

            // Label
            let label = "Find: ";
            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(dialog_x + i as u16, dialog_y, cell);
            }

            // Query
            for (i, ch) in self.find_query.chars().enumerate() {
                if (label.len() + i) < dialog_width as usize - 8 {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(166, 227, 161));
                    cell.bg = Some(Color::rgb(49, 50, 68));
                    ctx.buffer
                        .set(dialog_x + label.len() as u16 + i as u16, dialog_y, cell);
                }
            }

            // Match count
            let count = format!(" {}/{}", self.current_find_index(), self.find_match_count());
            let count_x = dialog_x + dialog_width - count.len() as u16;
            for (i, ch) in count.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(180, 190, 254));
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.buffer.set(count_x + i as u16, dialog_y, cell);
            }
        }
    }
}
