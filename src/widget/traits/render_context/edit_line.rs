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
use crate::utils::EditState;

/// Columns from the start of `text` to char index `idx` (the end of the text
/// when `idx` is past it).
pub(crate) fn col_of(text: &str, idx: usize) -> usize {
    text.chars().take(idx).map(char_width).sum()
}

/// Char index of the glyph covering column `col` of `text` (the char count
/// when `col` is past the end): the inverse of [`col_of`], landing on a wide
/// glyph when `col` is its right half.
pub(crate) fn char_at_col(text: &str, col: usize) -> usize {
    let mut start = 0;
    for (idx, ch) in text.chars().enumerate() {
        let end = start + char_width(ch);
        if col < end {
            return idx;
        }
        start = end;
    }
    text.chars().count()
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

/// What a cell of a one-line edit field shows, for styling it
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FieldCell {
    /// The text
    Text,
    /// Selected text
    Selected,
    /// The placeholder, shown while the text is empty
    Placeholder,
    /// The cursor: over a glyph, over the placeholder's first one, or a
    /// blank past the end of the text
    Cursor,
}

/// A one-line edit field to draw with
/// [`put_edit_field`](super::RenderContext::put_edit_field)
pub(crate) struct EditField<'a> {
    /// The text
    pub text: &'a str,
    /// Shown while the text is empty
    pub placeholder: &'a str,
    /// The cursor as a char index into `text`, or `None` when it is not
    /// drawn (the field is not focused)
    pub cursor: Option<usize>,
    /// The selected chars, `start..end`
    pub selection: Option<(usize, usize)>,
}

impl<'a> EditField<'a> {
    /// The field for the first line of `state`, its primary cursor drawn
    /// when `focused`
    pub(crate) fn of(state: &'a EditState, placeholder: &'a str, focused: bool) -> Self {
        let cursor = state.primary();
        Self {
            text: state.line(0).unwrap_or(""),
            placeholder,
            cursor: focused.then(|| state.char_index(cursor.head())),
            selection: cursor.has_selection().then(|| {
                let (from, to) = cursor.range();
                (state.char_index(from), state.char_index(to))
            }),
        }
    }

    /// The scroll (in columns) for a view `width` columns wide that was at
    /// `scroll`: the least move that keeps the cursor in view, and the
    /// start of the text when the cursor is not drawn or the placeholder is
    /// shown
    pub(crate) fn scroll(&self, scroll: usize, width: u16) -> usize {
        match self.cursor {
            Some(cursor) if !self.text.is_empty() => scroll_to_cursor(
                scroll,
                col_of(self.text, cursor),
                cursor_width(self.text, cursor),
                col_of(self.text, usize::MAX) + 1,
                width as usize,
            ),
            _ => 0,
        }
    }
}

impl super::RenderContext<'_> {
    /// Draw a one-line edit field at relative `(x, y)`, `width` columns
    /// wide and scrolled `scroll` columns (see [`EditField::scroll`]): the
    /// text, or the placeholder while it is empty, and the cursor over the
    /// glyph at its index, over the placeholder's first glyph, or as a
    /// blank past the end. `style(kind, ch)` makes each cell.
    pub(crate) fn put_edit_field<F>(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        field: &EditField,
        scroll: usize,
        style: F,
    ) where
        F: Fn(FieldCell, char) -> Cell,
    {
        let placeholder = field.text.is_empty() && !field.placeholder.is_empty();
        let (text, scroll) = if placeholder {
            (field.placeholder, 0)
        } else {
            (field.text, scroll)
        };
        let selected = |i: usize| field.selection.is_some_and(|(a, b)| a <= i && i < b);
        self.put_edit_line(x, y, text, scroll, width, |i, ch| {
            let kind = if field.cursor == Some(i) {
                FieldCell::Cursor
            } else if placeholder {
                FieldCell::Placeholder
            } else if selected(i) {
                FieldCell::Selected
            } else {
                FieldCell::Text
            };
            style(kind, ch)
        });
        let len = field.text.chars().count();
        if !placeholder && field.cursor.is_some_and(|c| c >= len) {
            let col = col_of(field.text, usize::MAX).saturating_sub(scroll);
            if col < width as usize {
                self.set(
                    x.saturating_add(col as u16),
                    y,
                    style(FieldCell::Cursor, ' '),
                );
            }
        }
    }

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
        assert_eq!(char_at_col("a한b", 1), 1);
        assert_eq!(char_at_col("a한b", 2), 1);
        assert_eq!(char_at_col("a한b", 3), 2);
        assert_eq!(char_at_col("a한b", 9), 3);
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
