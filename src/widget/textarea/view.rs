//! View implementation for TextArea

use super::super::traits::{RenderContext, View};
use crate::render::{Cell, Modifier};
use crate::style::Color;

impl TextArea {
    /// Get line number width
    fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let max_line = self.lines.len();
            let digits = format!("{}", max_line).len();
            (digits + 2) as u16 // digits + space + separator
        } else {
            0
        }
    }
}

impl View for TextArea {
    crate::impl_view_meta!("TextArea");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let line_num_width = self.line_number_width();
        let text_start_x = line_num_width;
        let text_width = area.width.saturating_sub(line_num_width);
        let visible_lines = area.height as usize;

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

        // Show placeholder if empty
        if self.lines.len() == 1 && self.lines[0].is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                for (i, ch) in placeholder.chars().enumerate() {
                    if (i as u16) < text_width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(128, 128, 128));
                        cell.modifier = Modifier::ITALIC;
                        ctx.buffer
                            .set(area.x + text_start_x + i as u16, area.y, cell);
                    }
                }
            }
        }

        // Render visible lines
        for (view_row, line_idx) in (self.scroll.0..self.scroll.0 + visible_lines).enumerate() {
            if line_idx >= self.lines.len() {
                break;
            }

            let y = area.y + view_row as u16;

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
                        cell.fg = self.line_number_fg;
                        ctx.buffer.set(area.x + i as u16, y, cell);
                    }
                }
            }

            // Draw text
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();
            let scroll_col = if self.wrap { 0 } else { self.scroll.1 };

            // Get syntax highlighting spans for this line
            let highlights = self.highlighter.as_ref().map(|h| h.highlight_line(line));

            for (view_col, char_idx) in (scroll_col..scroll_col + text_width as usize).enumerate() {
                let x = area.x + text_start_x + view_col as u16;
                if x >= area.x + area.width {
                    break;
                }

                let ch = chars.get(char_idx).copied().unwrap_or(' ');
                let mut cell = Cell::new(ch);

                // Check if this position is selected (from any cursor)
                let is_selected = self.cursors.iter().any(|c| {
                    c.selection()
                        .map(|s| s.contains(line_idx, char_idx))
                        .unwrap_or(false)
                });

                // Check if this is any cursor position
                let is_cursor = self.focused
                    && self
                        .cursors
                        .iter()
                        .any(|c| c.pos.line == line_idx && c.pos.col == char_idx);

                // Check if this position is in a find match
                let (is_match, is_current_match) = if let Some(ref state) = self.find_replace {
                    let mut in_match = false;
                    let mut in_current = false;
                    for (idx, m) in state.matches.iter().enumerate() {
                        if m.start.line == line_idx
                            && char_idx >= m.start.col
                            && char_idx < m.end.col
                        {
                            in_match = true;
                            if state.current_match == Some(idx) {
                                in_current = true;
                            }
                            break;
                        }
                    }
                    (in_match, in_current)
                } else {
                    (false, false)
                };

                if is_cursor {
                    cell.fg = self.cursor_fg;
                    cell.bg = Some(Color::WHITE);
                    cell.modifier = Modifier::BOLD;
                } else if is_selected {
                    cell.fg = Some(Color::WHITE);
                    cell.bg = self.selection_bg;
                } else if is_current_match {
                    cell.fg = Some(Color::BLACK);
                    cell.bg = self.current_match_bg;
                } else if is_match {
                    cell.fg = Some(Color::BLACK);
                    cell.bg = self.match_highlight_bg;
                } else {
                    // Apply syntax highlighting if available
                    let mut highlight_applied = false;
                    if let Some(ref spans) = highlights {
                        for span in spans {
                            if char_idx >= span.start && char_idx < span.end {
                                cell.fg = Some(span.fg);
                                if span.bold {
                                    cell.modifier |= Modifier::BOLD;
                                }
                                if span.italic {
                                    cell.modifier |= Modifier::ITALIC;
                                }
                                highlight_applied = true;
                                break;
                            }
                        }
                    }
                    if !highlight_applied {
                        cell.fg = self.fg;
                    }
                    cell.bg = self.bg;
                }

                ctx.buffer.set(x, y, cell);
            }

            // Draw cursors at end of line if needed
            if self.focused {
                for cursor in self.cursors.iter() {
                    if cursor.pos.line == line_idx && cursor.pos.col >= chars.len() {
                        let cursor_x = area.x + text_start_x + (cursor.pos.col - scroll_col) as u16;
                        if cursor_x < area.x + area.width {
                            let mut cell = Cell::new(' ');
                            cell.bg = Some(Color::WHITE);
                            ctx.buffer.set(cursor_x, y, cell);
                        }
                    }
                }
            }
        }
    }
}

use crate::widget::textarea::TextArea;
