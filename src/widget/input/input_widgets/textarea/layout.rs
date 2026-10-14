//! Where text goes: wrapping lines into rows, the line-number gutter, and
//! the scroll that keeps the primary cursor in view

use crate::widget::traits::render_context::edit_line::{col_of, cursor_width, scroll_to_cursor};

use super::TextArea;

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
    pub(super) fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let max_line = self.lines.len();
            let digits = format!("{}", max_line).len();
            (digits + 2) as u16 // digits + space + separator
        } else {
            0
        }
    }

    /// Visual rows logical line `idx` takes: one, or one per wrapped segment.
    pub(super) fn line_rows(&self, idx: usize, text_width: u16) -> Vec<(usize, usize)> {
        let line = self.lines.get(idx).map_or("", String::as_str);
        if self.wrap && text_width > 0 {
            let chars: Vec<char> = line.chars().collect();
            wrap_segments(&chars, text_width)
        } else {
            vec![(0, line.chars().count())]
        }
    }

    /// The first visible line that keeps the primary cursor's visual row in a
    /// view `visible_lines` rows high, moving the current scroll as little as
    /// possible: the vertical counterpart of `scroll_to_cursor`.
    pub(super) fn scroll_to_cursor_row(&self, text_width: u16, visible_lines: usize) -> usize {
        let pos = self.cursors.primary().pos;
        let cursor_line = pos.line.min(self.lines.len().saturating_sub(1));
        let scroll = self.scroll.get().min(cursor_line);

        // Rows from the top of the cursor's line down to the cursor's row.
        let mut rows = self.cursor_segment(text_width).0 + 1;

        // Walk up from the cursor's line while the lines above still fit,
        // stopping at the current scroll so a cursor in view does not move it.
        let mut top = cursor_line;
        while top > scroll {
            let above = self.line_rows(top - 1, text_width).len();
            if rows + above > visible_lines {
                break;
            }
            rows += above;
            top -= 1;
        }
        top
    }

    /// The primary cursor's wrapped segment: its index within the cursor's
    /// line and its `[start, end)` chars. A cursor at a wrap point is drawn
    /// at the start of the next segment.
    pub(super) fn cursor_segment(&self, text_width: u16) -> (usize, (usize, usize)) {
        let pos = self.cursors.primary().pos;
        let cursor_line = pos.line.min(self.lines.len().saturating_sub(1));
        let segments = self.line_rows(cursor_line, text_width);
        let idx = segments
            .iter()
            .position(|&(_, end)| pos.col < end)
            .unwrap_or(segments.len() - 1);
        (idx, segments[idx])
    }

    /// Build the list of visual rows to render, starting from the vertical scroll offset.
    ///
    /// Each entry is `(line_idx, seg_start, seg_end, is_first_segment)`. When `wrap` is on a
    /// single logical line may span several visual rows; otherwise every logical line maps to
    /// exactly one visual row, scrolled horizontally when drawn.
    pub(super) fn visual_rows(
        &self,
        text_width: u16,
        visible_lines: usize,
    ) -> Vec<(usize, usize, usize, bool)> {
        let mut rows = Vec::with_capacity(visible_lines);
        let mut line_idx = self.scroll.get();

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

    /// Unwrapped lines scroll horizontally, in columns, to keep the primary
    /// cursor (and the whole glyph under it) in view; wrapped lines do not.
    pub(super) fn horizontal_scroll(&self, text_width: u16) -> usize {
        if self.wrap {
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
        }
    }
}

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
