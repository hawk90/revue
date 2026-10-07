//! Code editor rendering

use crate::render::Cell;
use crate::style::Color;
use crate::widget::code_editor::CodeEditor;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::render_context::edit_line::{col_of, cursor_width, scroll_to_cursor};
use crate::widget::traits::{RenderContext, View};

/// Draw a dialog's typed text on row 0 in `width` columns, scrolled so its
/// end (where typing happens) stays in view.
fn put_query(ctx: &mut RenderContext, x: u16, width: u16, text: &str) {
    let text_w = col_of(text, usize::MAX);
    let scroll = scroll_to_cursor(0, text_w, 0, text_w, width as usize);
    ctx.put_edit_line(x, 0, text, scroll, width, |_, ch| {
        let mut cell = Cell::new(ch);
        cell.fg = Some(Color::rgb(166, 227, 161));
        cell.bg = Some(Color::rgb(49, 50, 68));
        cell
    });
}

impl CodeEditor {
    // =========================================================================
    // Rendering Helpers
    // =========================================================================

    /// Get line number width
    #[doc(hidden)]
    pub fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let digits = format!("{}", self.lines.len()).len();
            (digits + 2) as u16
        } else {
            0
        }
    }

    /// Get syntax highlights for a line
    #[doc(hidden)]
    pub fn get_highlights(&self, line: &str) -> Vec<crate::widget::syntax::HighlightSpan> {
        self.highlighter
            .as_ref()
            .map(|h: &crate::widget::syntax::SyntaxHighlighter| h.highlight_line(line))
            .unwrap_or_default()
    }

    /// Check if position is in selection
    #[doc(hidden)]
    pub fn is_selected(&self, line: usize, col: usize) -> bool {
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
    #[doc(hidden)]
    pub fn get_find_match_at(&self, line: usize, col: usize) -> Option<(bool, usize)> {
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
        // The find and go-to-line boxes take row 0 while open; lines start
        // below them so the box never covers the cursor's line.
        let top: u16 = if (self.find_mode || self.goto_line_mode) && area.height > 1 {
            1
        } else {
            0
        };
        let visible_lines = (area.height - top) as usize;
        self.page_height.set(visible_lines);

        // Find matching bracket
        let bracket_match = self.find_matching_bracket();

        // Draw background
        if let Some(bg) = self.bg {
            ctx.fill_box_background(bg);
        }

        // Scroll horizontally, in columns, to keep the cursor (and the whole
        // glyph under it) in view.
        let scroll_x = {
            let line = self.lines.get(self.cursor.0).map_or("", String::as_str);
            scroll_to_cursor(
                self.scroll_x.get(),
                col_of(line, self.cursor.1),
                cursor_width(line, self.cursor.1),
                col_of(line, usize::MAX) + 1,
                text_width as usize,
            )
        };
        self.scroll_x.set(scroll_x);

        // Scroll vertically to keep the cursor's line in view, moving the
        // first visible line as little as possible.
        let scroll = self.scroll_for_cursor(visible_lines);
        self.scroll.set(scroll);

        // Render visible lines
        let start_line = scroll;
        let end_line = (start_line + visible_lines).min(self.lines.len());

        for (view_row, line_idx) in (start_line..end_line).enumerate() {
            let y = top + view_row as u16;
            let line = &self.lines[line_idx];
            let is_current_line = line_idx == self.cursor.0;

            // Current line highlight
            if self.config.highlight_current_line && is_current_line && self.focused {
                for x in line_num_width..line_num_width + text_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.current_line_bg);
                    ctx.set(x, y, cell);
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
                        ctx.set(i as u16, y, cell);
                    }
                }
            }

            // Get syntax highlights
            let highlights = self.get_highlights(line);

            // Draw text, in columns: a wide glyph takes two cells.
            ctx.put_edit_line(
                line_num_width,
                y,
                line,
                scroll_x,
                text_width,
                |char_idx, ch| {
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
                    cell
                },
            );

            // Draw cursor at end of line if needed
            if self.focused && line_idx == self.cursor.0 && self.cursor.1 >= line.chars().count() {
                if let Some(x) = col_of(line, usize::MAX).checked_sub(scroll_x) {
                    if x < text_width as usize {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(self.cursor_bg);
                        ctx.set(line_num_width + x as u16, y, cell);
                    }
                }
            }
        }

        // Draw minimap if enabled
        if self.config.show_minimap && minimap_width > 0 {
            let minimap_x = area.width - minimap_width;

            // Minimap background
            for y in 0..area.height {
                for x in 0..minimap_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.minimap_bg);
                    ctx.set(minimap_x + x, y, cell);
                }
            }

            // Calculate minimap scale
            let total_lines = self.lines.len();
            let minimap_height = area.height as usize;
            let lines_per_row = (total_lines as f32 / minimap_height as f32).max(1.0);

            // Draw minimap content
            for row in 0..minimap_height {
                let start_line = (row as f32 * lines_per_row) as usize;
                let y = row as u16;

                // Highlight visible area
                if start_line >= scroll && start_line < scroll + visible_lines {
                    for x in 0..minimap_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(self.minimap_visible_bg);
                        ctx.set(minimap_x + x, y, cell);
                    }
                }

                // Draw condensed line representation
                if let Some(line) = self.lines.get(start_line) {
                    let chars: Vec<char> = line.chars().collect();
                    for (i, &ch) in chars.iter().take(minimap_width as usize).enumerate() {
                        if !ch.is_whitespace() {
                            let mut cell = Cell::new('▪');
                            cell.fg = Some(PLACEHOLDER_FG);
                            ctx.set(minimap_x + i as u16, y, cell);
                        }
                    }
                }
            }
        }

        // Draw go-to-line dialog
        if self.goto_line_mode {
            let dialog_width = 20u16;
            let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;

            // Background
            for x in 0..dialog_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.set(dialog_x + x, 0, cell);
            }

            // Label
            let label = "Go to line: ";
            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.set(dialog_x + i as u16, 0, cell);
            }

            // Input
            let input_x = dialog_x + label.len() as u16;
            let input_w = dialog_width.saturating_sub(label.len() as u16);
            put_query(ctx, input_x, input_w, &self.goto_line_input);
        }

        // Draw find dialog
        if self.find_mode {
            let dialog_width = 30u16;
            let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;

            // Background
            for x in 0..dialog_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.set(dialog_x + x, 0, cell);
            }

            // Label
            let label = "Find: ";
            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.set(dialog_x + i as u16, 0, cell);
            }

            // Query, leaving 8 columns for the match count
            let query_x = dialog_x + label.len() as u16;
            let query_w = dialog_width.saturating_sub(label.len() as u16 + 8);
            put_query(ctx, query_x, query_w, &self.find_query);

            // Match count
            let count = format!(" {}/{}", self.current_find_index(), self.find_match_count());
            let count_x = dialog_x + dialog_width - count.len() as u16;
            for (i, ch) in count.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(180, 190, 254));
                cell.bg = Some(Color::rgb(49, 50, 68));
                ctx.set(count_x + i as u16, 0, cell);
            }
        }
    }
}
