//! Where text goes: wrapping lines into rows, the line-number gutter, and
//! the scroll that keeps the primary cursor in view

use crate::widget::traits::render_context::edit_line::{col_of, cursor_width, scroll_to_cursor};

use super::{CursorPos, TextArea};

/// Split a logical line into visual segments `[start, end)` (character indices) that each
/// fit within `width` display columns. Prefers breaking at word boundaries (after a space)
/// and lets one space hang past the end of a full row, rather than start the next row with
/// it; further spaces go to the next row. The caller wraps one column short of the view,
/// so the hanging space sits in the last column, visible (#1008). Falls back to a hard
/// break when a single word is longer than the width. Always makes forward progress, so it
/// terminates even for zero-width characters or `width == 0`.
fn wrap_segments(chars: &[char], width: u16) -> Vec<(usize, usize)> {
    if chars.is_empty() {
        return vec![(0, 0)];
    }

    let width = width.max(1);
    let mut segments = Vec::new();
    let mut start = 0usize;
    let mut col: u16 = 0;
    let mut last_space_end: Option<usize> = None;
    let mut hung = false;
    let mut i = 0usize;

    while i < chars.len() {
        let cw = (crate::utils::char_width(chars[i]) as u16).max(1);

        if col + cw > width && i > start {
            if chars[i] == ' ' && !hung {
                // One space past a full row hangs on it rather than starting
                // the next row
                hung = true;
                i += 1;
                last_space_end = Some(i);
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
            hung = false;
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
    ///
    /// When wrapping, text takes one column less than the view: the last
    /// column holds a space hanging past a full row and the cursor at the
    /// end of a full row, so every position has a cell of its own (as nano
    /// does when it wraps at blanks, #1008). A line whose last row reaches
    /// the last column anyway (it ends in a hanging space) gets one more,
    /// empty row for the cursor past its end.
    pub(super) fn line_rows(&self, idx: usize, text_width: u16) -> Vec<(usize, usize)> {
        let line = self.lines.get(idx).map_or("", String::as_str);
        if self.wrap && text_width > 0 {
            let chars: Vec<char> = line.chars().collect();
            let mut rows = wrap_segments(&chars, text_width.saturating_sub(1).max(1));
            let &(start, end) = rows.last().expect("one row at least");
            let last: String = chars[start..end].iter().collect();
            if end > start && col_of(&last, usize::MAX) >= text_width as usize {
                rows.push((end, end));
            }
            rows
        } else {
            vec![(0, line.chars().count())]
        }
    }

    /// Where `pos` is drawn on its line: the row index and the column within
    /// that row. A position at a wrap point is drawn at the start of the
    /// next row.
    pub(super) fn place(&self, pos: CursorPos, text_width: u16) -> (usize, usize) {
        let line = pos.line.min(self.lines.len().saturating_sub(1));
        let rows = self.line_rows(line, text_width);
        let text = self.lines.get(line).map_or("", String::as_str);
        place_in(&rows, text, pos.col, text_width, self.wrap)
    }

    /// The first visible line and row within it that keep the primary
    /// cursor's visual row in a view `visible_lines` rows high, moving the
    /// current scroll as little as possible: the vertical counterpart of
    /// `scroll_to_cursor`. Rows, not lines: the view can start partway down a
    /// wrapped line taller than it (#1008).
    pub(super) fn scroll_to_cursor_row(
        &self,
        text_width: u16,
        visible_lines: usize,
    ) -> (usize, usize) {
        let pos = self.cursors.primary().pos;
        let mut rows = RowCounts::new(self, text_width);
        let line = pos.line.min(self.lines.len().saturating_sub(1));
        let cursor = (line, self.place(pos, text_width).0);
        let visible = visible_lines.max(1);
        let top_line = self.scroll.get().min(self.lines.len().saturating_sub(1));
        let top = (
            top_line,
            self.scroll_row.get().min(rows.count(top_line) - 1),
        );
        if cursor < top {
            return cursor;
        }
        if rows.between(top, cursor, visible) < visible {
            return top;
        }
        // Too far down: the cursor's row goes on the bottom row
        rows.back(cursor, visible - 1)
    }

    /// Build the list of visual rows to render, starting from the vertical scroll offset.
    ///
    /// Each entry is `(line_idx, seg_start, seg_end, row)`, `row` being the
    /// row's index within its line. When `wrap` is on a single logical line
    /// may span several visual rows; otherwise every logical line maps to
    /// exactly one visual row, scrolled horizontally when drawn.
    pub(super) fn visual_rows(
        &self,
        text_width: u16,
        visible_lines: usize,
    ) -> Vec<(usize, usize, usize, usize)> {
        let mut rows = Vec::with_capacity(visible_lines);
        let mut line_idx = self.scroll.get();
        let mut skip = self.scroll_row.get();

        while rows.len() < visible_lines && line_idx < self.lines.len() {
            for (k, (s, e)) in self
                .line_rows(line_idx, text_width)
                .into_iter()
                .enumerate()
                .skip(skip)
            {
                if rows.len() >= visible_lines {
                    break;
                }
                rows.push((line_idx, s, e, k));
            }
            skip = 0;
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

/// Where char `col` is drawn among a line's `rows`: the row index and the
/// column within that row. A position at a wrap point is drawn at the start
/// of the next row; one at or past the view's width (the end of a row that
/// reaches the last column) at the start of the row after it.
pub(super) fn place_in(
    rows: &[(usize, usize)],
    text: &str,
    col: usize,
    text_width: u16,
    wrap: bool,
) -> (usize, usize) {
    let mut k = rows
        .iter()
        .position(|&(_, end)| col < end)
        .unwrap_or(rows.len() - 1);
    let x = |k: usize| {
        let (start, end) = rows[k];
        let seg: String = text.chars().skip(start).take(end - start).collect();
        col_of(&seg, col.saturating_sub(start))
    };
    if wrap && k + 1 < rows.len() && x(k) >= text_width as usize {
        k += 1;
        return (k, 0);
    }
    (k, x(k))
}

/// Visual rows counted per line once, for walking the view row by row
pub(super) struct RowCounts<'a> {
    area: &'a TextArea,
    width: u16,
    counts: std::collections::HashMap<usize, usize>,
}

impl<'a> RowCounts<'a> {
    pub(super) fn new(area: &'a TextArea, width: u16) -> Self {
        Self {
            area,
            width,
            counts: std::collections::HashMap::new(),
        }
    }

    /// The rows line `line` takes
    pub(super) fn count(&mut self, line: usize) -> usize {
        let (area, width) = (self.area, self.width);
        *self
            .counts
            .entry(line)
            .or_insert_with(|| area.line_rows(line, width).len())
    }

    /// The last row of the text
    pub(super) fn end(&mut self) -> (usize, usize) {
        let last = self.area.lines.len().saturating_sub(1);
        (last, self.count(last) - 1)
    }

    /// `n` rows after `at`, stopping at the last row
    pub(super) fn forward(
        &mut self,
        (mut line, mut row): (usize, usize),
        n: usize,
    ) -> (usize, usize) {
        let end = self.end();
        for _ in 0..n {
            if (line, row) >= end {
                break;
            }
            if row + 1 < self.count(line) {
                row += 1;
            } else {
                line += 1;
                row = 0;
            }
        }
        (line, row)
    }

    /// `n` rows before `at`, stopping at the first row
    pub(super) fn back(&mut self, (mut line, mut row): (usize, usize), n: usize) -> (usize, usize) {
        for _ in 0..n {
            if row > 0 {
                row -= 1;
            } else if line > 0 {
                line -= 1;
                row = self.count(line) - 1;
            } else {
                break;
            }
        }
        (line, row)
    }

    /// Rows from `a` down to `b`, counted up to `limit`
    pub(super) fn between(&mut self, a: (usize, usize), b: (usize, usize), limit: usize) -> usize {
        let mut at = a;
        let mut n = 0;
        while at < b && n < limit {
            at = self.forward(at, 1);
            n += 1;
        }
        n
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
