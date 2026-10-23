//! TextArea `View`: the background, placeholder, line numbers, and each
//! row's text with cursors, selection, find matches and syntax colors

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::text::char_to_byte_index;
use crate::widget::syntax::HighlightSpan;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{RenderContext, View};

use super::TextArea;

impl TextArea {
    /// The line number, right-aligned in the gutter
    fn draw_line_number(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        line_idx: usize,
        line_num_width: u16,
    ) {
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

    /// The cell for char `char_idx` of line `line_idx`: a cursor, then the
    /// selection, then find matches, then syntax colors or the text color
    fn text_cell(
        &self,
        line_idx: usize,
        char_idx: usize,
        ch: char,
        highlights: &Option<Vec<HighlightSpan>>,
        css_fg: Option<Color>,
    ) -> Cell {
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
                if m.start.line == line_idx && char_idx >= m.start.col && char_idx < m.end.col {
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
                // The body text takes the stylesheet; the cursor,
                // line numbers and selection keep their own.
                cell.fg = self.fg.or(css_fg);
            }
            cell.bg = self.bg;
        }
        cell
    }
}

impl View for TextArea {
    crate::impl_view_meta!("TextArea", focusable);

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
        self.last_viewport_height.set(visible_lines);
        self.last_text_width.set(text_width);

        // Draw background
        if let Some(bg) = self.bg {
            ctx.fill_box_background(bg);
        }

        // Show placeholder if empty
        if self.lines.len() == 1 && self.lines[0].is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                ctx.draw_text_clipped(text_start_x, 0, placeholder, PLACEHOLDER_FG, text_width);
            }
        }

        // Unwrapped lines scroll horizontally, in columns, to keep the
        // primary cursor (and the whole glyph under it) in view.
        let scroll_x = self.horizontal_scroll(text_width);
        self.scroll_x.set(scroll_x);

        // Scroll vertically, in visual rows, to keep the primary cursor in view.
        let (top, top_row) = self.scroll_to_cursor_row(text_width, visible_lines);
        self.scroll.set(top);
        self.scroll_row.set(top_row);
        let css_fg = ctx.css_color_if_set();

        // Render visible visual rows (a logical line may span multiple rows when wrapping).
        let rows = self.visual_rows(text_width, visible_lines);
        for (view_row, &(line_idx, seg_start, seg_end, row)) in rows.iter().enumerate() {
            let is_first = row == 0;
            let y = view_row as u16;

            // Draw line numbers (only on the first visual row of a logical line)
            if self.show_line_numbers && is_first {
                self.draw_line_number(ctx, y, line_idx, line_num_width);
            }

            // Draw text. Columns, not chars: a wide glyph takes two cells.
            let line = &self.lines[line_idx];
            let seg = &line[char_to_byte_index(line, seg_start)..char_to_byte_index(line, seg_end)];

            // Get syntax highlighting spans for this line
            let highlights = self.highlighter.as_ref().map(|h| h.highlight_line(line));

            ctx.put_edit_line(text_start_x, y, seg, scroll_x, text_width, |i, ch| {
                self.text_cell(line_idx, seg_start + i, ch, &highlights, css_fg)
            });

            // Cursors the text cells do not show: past the line's end (on
            // its last row, or the extra row of a full one), and one moved
            // here from a space hanging past the row above (drawn over this
            // row's first char)
            if self.focused {
                for cursor in self.cursors.iter().filter(|c| c.pos.line == line_idx) {
                    let (k, x) = self.place(cursor.pos, text_width);
                    if k != row || (seg_start..seg_end).contains(&cursor.pos.col) {
                        continue;
                    }
                    let Some(x) = x.checked_sub(scroll_x) else {
                        continue;
                    };
                    if x < text_width as usize {
                        let under = if cursor.pos.col < seg_start {
                            seg.chars().next()
                        } else {
                            None
                        };
                        let mut cell = Cell::new(under.unwrap_or(' '));
                        cell.fg = self.cursor_fg;
                        cell.bg = Some(Color::WHITE);
                        ctx.set(text_start_x + x as u16, y, cell);
                    }
                }
            }
        }
    }
}
