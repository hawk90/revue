//! A line of editable text, drawn in terminal columns
//!
//! Text widgets keep their cursor as a char index, which is what editing
//! needs, but the screen is laid out in columns: a wide glyph (Hangul, CJK,
//! emoji) takes two, a zero-width char (a combining mark, VS16) none. These
//! helpers turn char positions into columns, keep a horizontally scrolled
//! view around the cursor, and draw the visible part of the line with each
//! wide glyph's continuation cell.

use crate::render::Cell;
use crate::utils::unicode::char_width;

/// Columns from the start of `text` to char index `idx` (the end of the text
/// when `idx` is past it).
pub(crate) fn col_of(text: &str, idx: usize) -> usize {
    text.chars().take(idx).map(char_width).sum()
}

/// Columns the cursor at char index `idx` covers: the glyph under it, or one
/// past the end of the text (also on a zero-width char, so it stays visible).
pub(crate) fn cursor_width(text: &str, idx: usize) -> usize {
    text.chars().nth(idx).map_or(1, |c| char_width(c).max(1))
}

/// The horizontal scroll (in columns) that keeps the cursor inside a view
/// `width` columns wide, moving the current `scroll` as little as possible.
///
/// The cursor starts at column `cursor_col` and covers `cursor_w` columns, so
/// a cursor on a wide glyph shows the whole glyph. `line_w` is the width of
/// the line including the cell after its end; once that fits, the scroll
/// is pulled back so a shortened line does not leave the view half empty.
pub(crate) fn scroll_to_cursor(
    scroll: usize,
    cursor_col: usize,
    cursor_w: usize,
    line_w: usize,
    width: usize,
) -> usize {
    let scroll = scroll.min(line_w.saturating_sub(width));
    if cursor_col < scroll {
        cursor_col
    } else if cursor_col + cursor_w > scroll + width {
        // Never past the cursor's own column, even in a view narrower than
        // the glyph under it.
        (cursor_col + cursor_w)
            .saturating_sub(width)
            .min(cursor_col)
    } else {
        scroll
    }
}

impl super::RenderContext<'_> {
    /// Draw `text` at relative `(x, y)`, scrolled `scroll` columns to the
    /// left and at most `width` columns wide.
    ///
    /// `make_cell(char_idx, ch)` styles each glyph, so a caller can paint
    /// the cursor, a selection or a match by char index. A wide glyph gets a
    /// continuation cell with the same colors and modifiers; a zero-width
    /// char is skipped. A wide glyph cut by either edge of the view is drawn
    /// as blanks in its style, so the visible half never shows stale cells.
    pub(crate) fn put_edit_line<F>(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        scroll: usize,
        width: u16,
        mut make_cell: F,
    ) where
        F: FnMut(usize, char) -> Cell,
    {
        let end = scroll + width as usize;
        let screen_x = |col: usize| x.saturating_add((col - scroll) as u16);
        let mut col = 0usize;
        for (idx, ch) in text.chars().enumerate() {
            if col >= end {
                break;
            }
            let w = char_width(ch);
            if w == 0 {
                continue;
            }
            let next = col + w;
            if col >= scroll && next <= end {
                let cell = make_cell(idx, ch);
                let sx = screen_x(col);
                self.set(sx, y, cell);
                for dx in 1..w {
                    let mut cont = Cell::continuation();
                    cont.fg = cell.fg;
                    cont.bg = cell.bg;
                    cont.modifier = cell.modifier;
                    self.set(sx.saturating_add(dx as u16), y, cont);
                }
            } else if next > scroll {
                let mut blank = make_cell(idx, ch);
                blank.symbol = ' ';
                for c in col.max(scroll)..next.min(end) {
                    self.set(screen_x(c), y, blank);
                }
            }
            col = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn columns_count_wide_glyphs_twice() {
        assert_eq!(col_of("a한b", 0), 0);
        assert_eq!(col_of("a한b", 2), 3);
        assert_eq!(col_of("a한b", 9), 4);
        assert_eq!(cursor_width("a한b", 1), 2);
        assert_eq!(cursor_width("a한b", 3), 1);
    }

    #[test]
    fn scroll_moves_only_as_far_as_the_cursor_needs() {
        // Already visible: unchanged.
        assert_eq!(scroll_to_cursor(2, 4, 1, 30, 8), 2);
        // Past the right edge: the cursor lands in the last column(s).
        assert_eq!(scroll_to_cursor(0, 20, 1, 21, 8), 13);
        assert_eq!(scroll_to_cursor(0, 18, 2, 21, 8), 12);
        // Left of the view: the cursor lands in the first column.
        assert_eq!(scroll_to_cursor(10, 4, 2, 30, 8), 4);
        // A line that now fits pulls the view back.
        assert_eq!(scroll_to_cursor(10, 3, 1, 5, 8), 0);
        // A view narrower than the glyph keeps the glyph's start.
        assert_eq!(scroll_to_cursor(0, 6, 2, 9, 1), 6);
    }
}
