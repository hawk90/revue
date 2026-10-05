//! View implementation for TextArea

use crate::event::KeyEvent;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::text::char_to_byte_index;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::render_context::edit_line::{col_of, cursor_width, scroll_to_cursor};
use crate::widget::traits::{EventResult, Interactive, RenderContext, View};

/// Split a logical line into visual segments `[start, end)` (character indices) that each
/// fit within `width` display columns. Prefers breaking at word boundaries (after a space)
/// and lets trailing spaces overflow onto the current row; falls back to a hard break when
/// a single word is longer than the width. Always makes forward progress, so it terminates
/// even for zero-width characters or `width == 0`.
fn wrap_segments(chars: &[char], width: u16) -> Vec<(usize, usize)> {
    if chars.is_empty() {
        return vec![(0, 0)];
    }

    let width = width.max(1);
    let mut segments = Vec::new();
    let mut start = 0usize;
    let mut col: u16 = 0;
    let mut last_space_end: Option<usize> = None;
    let mut i = 0usize;

    while i < chars.len() {
        let cw = (crate::utils::char_width(chars[i]) as u16).max(1);

        if col + cw > width && i > start {
            if chars[i] == ' ' {
                // Trailing space overflowing the row: keep it on the current visual
                // row rather than pushing a lone space to the next one.
                i += 1;
                continue;
            }
            let brk = match last_space_end {
                Some(b) if b > start => b,
                _ => i,
            };
            segments.push((start, brk));
            start = brk;
            col = 0;
            last_space_end = None;
            i = brk;
            continue;
        }

        col += cw;
        if chars[i] == ' ' {
            last_space_end = Some(i + 1);
        }
        i += 1;
    }

    segments.push((start, chars.len()));
    segments
}

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

    /// Build the list of visual rows to render, starting from the vertical scroll offset.
    ///
    /// Each entry is `(line_idx, seg_start, seg_end, is_first_segment)`. When `wrap` is on a
    /// single logical line may span several visual rows; otherwise every logical line maps to
    /// exactly one visual row, scrolled horizontally when drawn.
    fn visual_rows(
        &self,
        text_width: u16,
        visible_lines: usize,
    ) -> Vec<(usize, usize, usize, bool)> {
        let mut rows = Vec::with_capacity(visible_lines);
        let mut line_idx = self.scroll;

        while rows.len() < visible_lines && line_idx < self.lines.len() {
            let char_count = self.lines[line_idx].chars().count();

            if self.wrap && text_width > 0 {
                let chars: Vec<char> = self.lines[line_idx].chars().collect();
                for (k, (s, e)) in wrap_segments(&chars, text_width).into_iter().enumerate() {
                    if rows.len() >= visible_lines {
                        break;
                    }
                    rows.push((line_idx, s, e, k == 0));
                }
            } else {
                // Unwrapped rows are scrolled by columns when drawn.
                rows.push((line_idx, 0, char_count, true));
            }

            line_idx += 1;
        }

        rows
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
        let scroll_x = if self.wrap {
            0
        } else {
            let pos = self.cursors.primary().pos;
            let line = self.lines.get(pos.line).map_or("", String::as_str);
            scroll_to_cursor(
                self.scroll_x.get(),
                col_of(line, pos.col),
                cursor_width(line, pos.col),
                col_of(line, usize::MAX) + 1,
                text_width as usize,
            )
        };
        self.scroll_x.set(scroll_x);
        let css_fg = ctx.css_color_if_set();

        // Render visible visual rows (a logical line may span multiple rows when wrapping).
        let rows = self.visual_rows(text_width, visible_lines);
        for (view_row, &(line_idx, seg_start, seg_end, is_first)) in rows.iter().enumerate() {
            let y = view_row as u16;

            // Draw line numbers (only on the first visual row of a logical line)
            if self.show_line_numbers && is_first {
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

            // Draw text. Columns, not chars: a wide glyph takes two cells.
            let line = &self.lines[line_idx];
            let char_count = line.chars().count();
            let is_last_segment = seg_end >= char_count;
            let seg = &line[char_to_byte_index(line, seg_start)..char_to_byte_index(line, seg_end)];

            // Get syntax highlighting spans for this line
            let highlights = self.highlighter.as_ref().map(|h| h.highlight_line(line));

            ctx.put_edit_line(text_start_x, y, seg, scroll_x, text_width, |i, ch| {
                let char_idx = seg_start + i;
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
                        // The body text takes the stylesheet; the cursor,
                        // line numbers and selection keep their own.
                        cell.fg = self.fg.or(css_fg);
                    }
                    cell.bg = self.bg;
                }
                cell
            });

            // Draw cursors at end of line if needed (only on the last visual row of the line)
            if self.focused && is_last_segment {
                let end_x = col_of(seg, usize::MAX);
                for cursor in self.cursors.iter() {
                    if cursor.pos.line == line_idx && cursor.pos.col >= char_count {
                        if let Some(x) = end_x.checked_sub(scroll_x) {
                            if x < text_width as usize {
                                let mut cell = Cell::new(' ');
                                cell.bg = Some(Color::WHITE);
                                ctx.set(text_start_x + x as u16, y, cell);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Interactive for TextArea {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        EventResult::from(self.handle_key_event(event))
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_blur(&mut self) {
        self.focused = false;
    }
}

use super::TextArea;

#[cfg(test)]
mod wrap_segment_tests {
    use super::wrap_segments;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    /// Every segment must reconstruct the original line exactly, in order.
    fn assert_lossless(s: &str, width: u16) -> Vec<(usize, usize)> {
        let cs = chars(s);
        let segs = wrap_segments(&cs, width);
        // Contiguous, covering, in order.
        assert_eq!(
            segs.first().map(|s| s.0),
            Some(0),
            "first segment starts at 0"
        );
        assert_eq!(
            segs.last().map(|s| s.1),
            Some(cs.len()),
            "last segment ends at len"
        );
        for w in segs.windows(2) {
            assert_eq!(w[0].1, w[1].0, "segments are contiguous: {:?}", segs);
            assert!(w[0].0 < w[0].1, "no empty interior segment: {:?}", segs);
        }
        segs
    }

    #[test]
    fn empty_line_yields_single_empty_segment() {
        assert_eq!(wrap_segments(&[], 5), vec![(0, 0)]);
    }

    #[test]
    fn short_line_is_not_split() {
        assert_eq!(wrap_segments(&chars("hello"), 10), vec![(0, 5)]);
        // exact fit
        assert_eq!(wrap_segments(&chars("hello"), 5), vec![(0, 5)]);
    }

    #[test]
    fn long_word_hard_wraps_at_width() {
        assert_eq!(
            wrap_segments(&chars("abcdefgh"), 3),
            vec![(0, 3), (3, 6), (6, 8)]
        );
    }

    #[test]
    fn wraps_at_word_boundary_and_keeps_trailing_space() {
        // "aa " stays together, "bb" moves to the next row.
        assert_eq!(wrap_segments(&chars("aa bb"), 3), vec![(0, 3), (3, 5)]);
    }

    #[test]
    fn trailing_space_overflow_stays_on_row() {
        // The space that overflows width should not become a lone next row.
        let segs = assert_lossless("hello world", 5);
        assert!(segs.len() >= 2);
        // No segment is just a single space.
        let cs = chars("hello world");
        for (s, e) in &segs {
            let seg: String = cs[*s..*e].iter().collect();
            assert_ne!(seg, " ", "no lone-space segment: {:?}", segs);
        }
    }

    #[test]
    fn always_makes_progress_on_zero_width() {
        // Must terminate and cover the whole line even with width 0.
        let segs = assert_lossless("abc", 0);
        assert!(!segs.is_empty());
    }
}
