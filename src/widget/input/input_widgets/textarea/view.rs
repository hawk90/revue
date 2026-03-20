//! View implementation for TextArea

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{RenderContext, View};

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
        let mut area = ctx.area;

        // Enforce minimum height: expand the render area if the layout gave us
        // less than min_height rows. This ensures TextArea is visible even when
        // used as an auto-sized child in a vstack.
        if self.min_height > 0 && area.height < self.min_height {
            area.height = self.min_height;
        }

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
                    ctx.set(x, y, cell);
                }
            }
        }

        // Show placeholder if empty
        if self.lines.len() == 1 && self.lines[0].is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                ctx.draw_text_clipped(text_start_x, 0, placeholder, PLACEHOLDER_FG, text_width);
            }
        }

        // Render visible lines
        for (view_row, line_idx) in (self.scroll.0..self.scroll.0 + visible_lines).enumerate() {
            if line_idx >= self.lines.len() {
                break;
            }

            let y = view_row as u16;

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
                        ctx.set(i as u16, y, cell);
                    }
                }
            }

            // Draw text
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();
            let scroll_col = if self.wrap { 0 } else { self.scroll.1 };

            // Get syntax highlighting spans for this line
            let highlights = self.highlighter.as_ref().map(|h| h.highlight_line(line));

            // Render characters with display-width awareness
            let mut display_x: u16 = 0;
            for (char_idx, &ch) in chars.iter().enumerate().skip(scroll_col) {
                let cw = crate::utils::char_width(ch) as u16;
                if display_x + cw > text_width {
                    break;
                }

                let x = text_start_x + display_x;

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

                ctx.set(x, y, cell);
                display_x += cw;
            }

            // Draw cursors at end of line if needed
            if self.focused {
                for cursor in self.cursors.iter() {
                    if cursor.pos.line == line_idx && cursor.pos.col >= chars.len() {
                        // Calculate display position from character widths
                        let cursor_display_x: u16 = chars
                            .iter()
                            .skip(scroll_col)
                            .map(|&ch| crate::utils::char_width(ch) as u16)
                            .sum();
                        let cursor_x = text_start_x + cursor_display_x;
                        if cursor_x < area.width {
                            let mut cell = Cell::new(' ');
                            cell.bg = Some(Color::WHITE);
                            ctx.set(cursor_x, y, cell);
                        }
                    }
                }
            }
        }
    }
}

use super::TextArea;
