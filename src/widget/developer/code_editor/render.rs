//! Code editor rendering

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::{RenderContext, View};

    // =========================================================================
    // line_number_width tests
    // =========================================================================

    #[test]
    fn test_line_number_width_with_single_digit() {
        let editor = CodeEditor::new().content("line1");
        assert_eq!(editor.line_number_width(), 3); // "1 " = 2 digits + 1 space = 3
    }

    #[test]
    fn test_line_number_width_with_double_digit() {
        let mut editor = CodeEditor::new();
        for i in 0..15 {
            editor.lines.push(format!("line{}", i));
        }
        assert_eq!(editor.line_number_width(), 4); // "15 " = 2 digits + 2 spaces = 4
    }

    #[test]
    fn test_line_number_width_hidden() {
        let editor = CodeEditor::new().content("test").line_numbers(false);
        assert_eq!(editor.line_number_width(), 0);
    }

    // =========================================================================
    // get_highlights tests
    // =========================================================================

    #[test]
    fn test_get_highlights_default() {
        let editor = CodeEditor::new();
        let highlights = editor.get_highlights("fn main() {}");
        // No highlighter by default
        assert!(highlights.is_empty());
    }

    #[test]
    fn test_get_highlights_empty_line() {
        let editor = CodeEditor::new();
        let highlights = editor.get_highlights("");
        assert!(highlights.is_empty());
    }

    // =========================================================================
    // is_selected tests
    // =========================================================================

    #[test]
    fn test_is_selected_no_selection() {
        let editor = CodeEditor::new().content("hello");
        assert!(!editor.is_selected(0, 0));
        assert!(!editor.is_selected(0, 2));
    }

    #[test]
    fn test_is_selected_single_line() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.cursor = (0, 5);
        editor.start_selection();
        editor.move_right();
        editor.move_right();
        editor.move_right();

        assert!(!editor.is_selected(0, 0));
        assert!(editor.is_selected(0, 5));
        assert!(editor.is_selected(0, 6));
        assert!(editor.is_selected(0, 7));
        assert!(!editor.is_selected(0, 8));
    }

    #[test]
    fn test_is_selected_multi_line() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.start_selection();
        editor.move_down();
        editor.move_down();
        // Move right to include some of line3
        editor.move_right();
        editor.move_right();

        // All of line1, line2, line3 should be selected
        assert!(editor.is_selected(0, 0));
        assert!(editor.is_selected(0, 4));
        assert!(editor.is_selected(1, 0));
        assert!(editor.is_selected(1, 4));
        assert!(editor.is_selected(2, 0));
        assert!(editor.is_selected(2, 1));
    }

    #[test]
    fn test_is_selected_reverse() {
        let mut editor = CodeEditor::new().content("hello");
        editor.cursor = (0, 5);
        editor.start_selection();
        editor.move_left();
        editor.move_left();

        assert!(!editor.is_selected(0, 0));
        assert!(!editor.is_selected(0, 1));
        assert!(!editor.is_selected(0, 2));
        assert!(editor.is_selected(0, 3));
        assert!(editor.is_selected(0, 4));
    }

    #[test]
    fn test_is_selected_partial_lines() {
        let mut editor = CodeEditor::new().content("line1\nline2\nline3");
        editor.cursor = (0, 2);
        editor.start_selection();
        editor.move_down();
        editor.move_right();

        // line1: from col 2 to end
        assert!(!editor.is_selected(0, 0));
        assert!(!editor.is_selected(0, 1));
        assert!(editor.is_selected(0, 2));
        assert!(editor.is_selected(0, 4));

        // line2: columns 0-2 selected (cursor at column 3, exclusive end)
        assert!(editor.is_selected(1, 0));
        assert!(editor.is_selected(1, 2));
        assert!(!editor.is_selected(1, 3)); // Column 3 is the cursor position (exclusive)

        // line3: not selected
        assert!(!editor.is_selected(2, 0));
    }

    // =========================================================================
    // get_find_match_at tests
    // =========================================================================

    #[test]
    fn test_get_find_match_at_no_matches() {
        let editor = CodeEditor::new().content("hello world");
        assert!(editor.get_find_match_at(0, 0).is_none());
        assert!(editor.get_find_match_at(0, 5).is_none());
    }

    #[test]
    fn test_get_find_match_at_with_matches() {
        let mut editor = CodeEditor::new().content("hello hello hello");
        editor.open_find();
        editor.set_find_query("hello");

        // Should have 3 matches
        assert_eq!(editor.find_match_count(), 3);

        // Check first match
        let result = editor.get_find_match_at(0, 0);
        assert!(result.is_some());
        let (is_current, idx) = result.unwrap();
        assert!(is_current); // First match is current
        assert_eq!(idx, 0);

        // Check second match
        let result = editor.get_find_match_at(0, 6);
        assert!(result.is_some());
        let (is_current, idx) = result.unwrap();
        assert!(!is_current);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_get_find_match_at_not_in_match() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.open_find();
        editor.set_find_query("hello");

        assert!(editor.get_find_match_at(0, 5).is_none()); // Space after hello
        assert!(editor.get_find_match_at(0, 6).is_none()); // 'w' in world
    }

    #[test]
    fn test_get_find_match_at_after_next() {
        let mut editor = CodeEditor::new().content("hello hello");
        editor.open_find();
        editor.set_find_query("hello");
        editor.find_next(); // Move to second match

        // First match is no longer current
        let result = editor.get_find_match_at(0, 0);
        assert!(result.is_some());
        let (is_current, idx) = result.unwrap();
        assert!(!is_current);
        assert_eq!(idx, 0);

        // Second match is current
        let result = editor.get_find_match_at(0, 6);
        assert!(result.is_some());
        let (is_current, idx) = result.unwrap();
        assert!(is_current);
        assert_eq!(idx, 1);
    }

    // =========================================================================
    // render tests (integration)
    // =========================================================================

    #[test]
    fn test_render_basic() {
        let editor = CodeEditor::new().content("hello");
        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        // Should not panic
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_empty_area() {
        let editor = CodeEditor::new().content("test");
        let mut buffer = Buffer::new(0, 0);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        // Should not panic with zero area
        editor.render(&mut ctx);
    }

    #[test]
    fn test_render_with_selection() {
        let mut editor = CodeEditor::new().content("hello world");
        editor.start_selection();
        editor.move_right();
        editor.move_right();

        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Selection should be rendered
    }

    #[test]
    fn test_render_with_cursor() {
        let editor = CodeEditor::new().content("test");
        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Cursor should be rendered at position (0, 0)
    }

    #[test]
    fn test_render_multiline() {
        let editor = CodeEditor::new().content("line1\nline2\nline3");
        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // All three lines should be rendered
    }

    #[test]
    fn test_render_with_line_numbers() {
        let editor = CodeEditor::new().content("line1\nline2\nline3");
        let mut buffer = Buffer::new(30, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Line numbers should be rendered
    }

    #[test]
    fn test_render_without_line_numbers() {
        let editor = CodeEditor::new()
            .content("line1\nline2")
            .line_numbers(false);
        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // No line numbers should be rendered
    }

    #[test]
    fn test_render_with_goto_dialog() {
        let mut editor = CodeEditor::new().content("test");
        editor.open_goto_line();
        editor.handle_key(&Key::Char('5'));
        editor.handle_key(&Key::Char('0'));

        let mut buffer = Buffer::new(30, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Go-to-line dialog should be rendered
    }

    #[test]
    fn test_render_with_find_dialog() {
        let mut editor = CodeEditor::new().content("test hello world");
        editor.open_find();
        editor.handle_key(&Key::Char('h'));
        editor.handle_key(&Key::Char('e'));
        editor.handle_key(&Key::Char('l'));
        editor.handle_key(&Key::Char('l'));
        editor.handle_key(&Key::Char('o'));

        let mut buffer = Buffer::new(40, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Find dialog should be rendered with "hello" query
    }

    #[test]
    fn test_render_with_minimap() {
        let editor = CodeEditor::new()
            .content("line1\nline2\nline3\nline4\nline5")
            .minimap(true);

        let mut buffer = Buffer::new(40, 10);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Minimap should be rendered
    }

    #[test]
    fn test_render_unfocused() {
        let editor = CodeEditor::new().content("test").focused(false);

        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Cursor should not be highlighted when unfocused
    }

    #[test]
    fn test_render_with_bracket_match() {
        let mut editor = CodeEditor::new()
            .content("function()")
            .bracket_matching(true);
        editor.set_cursor(0, 9);

        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Matching bracket should be highlighted
    }

    #[test]
    fn test_render_scrolled() {
        let content: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
        let mut editor = CodeEditor::new().content(&content.join("\n"));
        editor.cursor = (50, 0);

        let mut buffer = Buffer::new(20, 10);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Should show lines around line 50, not from line 0
    }

    #[test]
    fn test_render_long_line_truncated() {
        let long_line = "a".repeat(100);
        let editor = CodeEditor::new().content(&long_line);

        let mut buffer = Buffer::new(20, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));

        editor.render(&mut ctx);
        // Long line should be truncated to fit width
    }
}

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
            .map(|h: &crate::widget::syntax::SyntaxHighlighter| h.highlight_line(line))
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
