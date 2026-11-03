//! A line of editable text, drawn in terminal columns
//!
//! Text widgets keep their cursor as a char index, which is what editing
//! needs, but the screen is laid out in columns: a wide glyph (Hangul, CJK,
//! emoji) takes two, a zero-width char (a combining mark, VS16) none. These
//! helpers turn char positions into columns, keep a horizontally scrolled
//! view around the cursor, and draw the visible part of the line with each
//! wide glyph's continuation cell.
//!
//! A tab reaches the next tab stop: a multiple of the caller's tab width,
//! counted from the start of the text drawn (a line, or a wrapped row of
//! it). Text that holds no tabs, or whose tabs take no column, passes
//! [`NO_TABS`].

use crate::render::Cell;
pub(crate) use crate::utils::unicode::{glyph_width, NO_TABS};
use crate::utils::EditState;

/// Columns from the start of `text` to char index `idx` (the end of the text
/// when `idx` is past it), with tab stops every `tab` columns.
pub(crate) fn col_of(text: &str, idx: usize, tab: usize) -> usize {
    text.chars()
        .take(idx)
        .fold(0, |col, ch| col + glyph_width(ch, col, tab))
}

/// Char index of the glyph covering column `col` of `text` (the char count
/// when `col` is past the end): the inverse of [`col_of`], landing on a wide
/// glyph or a tab when `col` is inside it.
pub(crate) fn char_at_col(text: &str, col: usize, tab: usize) -> usize {
    let mut start = 0;
    for (idx, ch) in text.chars().enumerate() {
        let end = start + glyph_width(ch, start, tab);
        if col < end {
            return idx;
        }
        start = end;
    }
    text.chars().count()
}

/// Columns the cursor at char index `idx` covers: the glyph or tab under it,
/// or one past the end of the text (also on a zero-width char, so it stays
/// visible).
pub(crate) fn cursor_width(text: &str, idx: usize, tab: usize) -> usize {
    text.chars()
        .nth(idx)
        .map_or(1, |c| glyph_width(c, col_of(text, idx, tab), tab).max(1))
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

/// The first char at or after `idx` that takes a cell (the char count when
/// none does): where a cursor at `idx` is drawn. A tab takes a cell unless
/// `tab` is [`NO_TABS`].
pub(crate) fn visible_from(text: &str, idx: usize, tab: usize) -> usize {
    text.chars()
        .enumerate()
        .skip(idx)
        .find(|&(_, c)| glyph_width(c, 0, tab) > 0)
        .map_or(text.chars().count(), |(i, _)| i)
}

/// The cells [`put_edit_line`](super::RenderContext::put_edit_line) draws
/// for glyph `ch` styled as `cell`, `w` columns wide: a tab's first cell
/// and blanks to its stop, or the glyph and its continuation. For the
/// cells under a caret, see [`set_caret`](super::RenderContext::set_caret).
pub(crate) fn glyph_cells(mut cell: Cell, ch: char, w: usize) -> Vec<Cell> {
    if ch != '\t' {
        return super::caret_cells(cell);
    }
    if cell.symbol == '\t' {
        cell.symbol = ' ';
    }
    let mut blank = cell;
    blank.symbol = ' ';
    let mut cells = vec![blank; w.max(1)];
    cells[0] = cell;
    cells
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

/// The foreground and background of selected text, from the theme: every field
/// draws a selection alike.
pub(crate) fn selection_colors() -> (crate::style::Color, crate::style::Color) {
    let theme = crate::style::use_theme().get();
    (theme.colors.selection_text, theme.colors.selection)
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
                col_of(self.text, cursor, NO_TABS),
                cursor_width(self.text, cursor, NO_TABS),
                col_of(self.text, usize::MAX, NO_TABS) + 1,
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
        // A zero-width char takes no cell, so a cursor on one is drawn on
        // the next glyph that has one
        let cursor = field.cursor.map(|c| visible_from(text, c, NO_TABS));
        // The cursor's glyph as it looks without the cursor, for the
        // terminal to show its own cursor over
        let mut under = None;
        self.put_edit_line(x, y, text, scroll, width, NO_TABS, |i, ch| {
            let kind = if placeholder {
                FieldCell::Placeholder
            } else if selected(i) {
                FieldCell::Selected
            } else {
                FieldCell::Text
            };
            if cursor == Some(i) {
                under = Some(style(kind, ch));
                style(FieldCell::Cursor, ch)
            } else {
                style(kind, ch)
            }
        });
        let Some(cursor) = cursor else {
            return;
        };
        let col = col_of(text, cursor, NO_TABS).saturating_sub(scroll);
        if col >= width as usize {
            return;
        }
        let cx = x.saturating_add(col as u16);
        match under {
            Some(under) => self.set_caret(cx, y, &super::relative::caret_cells(under)),
            // Past the end of the text: a blank, over what was there
            None if !placeholder && cursor >= text.chars().count() => {
                let under = self.get(cx, y).copied().unwrap_or_default();
                self.set(cx, y, style(FieldCell::Cursor, ' '));
                self.set_caret(cx, y, &[under]);
            }
            None => {}
        }
    }

    /// Draw `text` at relative `(x, y)`, scrolled `scroll` columns to the
    /// left and at most `width` columns wide, with tab stops every `tab`
    /// columns.
    ///
    /// `make_cell(char_idx, ch)` styles each glyph, so a caller can paint
    /// the cursor, a selection or a match by char index. A wide glyph gets a
    /// continuation cell with the same colors and modifiers; a zero-width
    /// char is skipped. A tab is blank cells in its style up to the next
    /// stop, the first showing the symbol `make_cell` gives it (a whitespace
    /// marker; a blank when that is the tab itself). A wide glyph cut by
    /// either edge of the view is drawn as blanks in its style, so the
    /// visible half never shows stale cells.
    #[allow(clippy::too_many_arguments)] // position, text, view, tabs, styler
    pub(crate) fn put_edit_line<F>(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        scroll: usize,
        width: u16,
        tab: usize,
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
            let w = glyph_width(ch, col, tab);
            if w == 0 {
                continue;
            }
            let next = col + w;
            if ch == '\t' {
                if next > scroll {
                    let mut first = make_cell(idx, ch);
                    if first.symbol == '\t' {
                        first.symbol = ' ';
                    }
                    let mut blank = first;
                    blank.symbol = ' ';
                    for c in col.max(scroll)..next.min(end) {
                        self.set(screen_x(c), y, if c == col { first } else { blank });
                    }
                }
            } else if col >= scroll && next <= end {
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
        assert_eq!(col_of("a한b", 0, NO_TABS), 0);
        assert_eq!(col_of("a한b", 2, NO_TABS), 3);
        assert_eq!(col_of("a한b", 9, NO_TABS), 4);
        assert_eq!(cursor_width("a한b", 1, NO_TABS), 2);
        assert_eq!(cursor_width("a한b", 3, NO_TABS), 1);
        assert_eq!(char_at_col("a한b", 1, NO_TABS), 1);
        assert_eq!(char_at_col("a한b", 2, NO_TABS), 1);
        assert_eq!(char_at_col("a한b", 3, NO_TABS), 2);
        assert_eq!(char_at_col("a한b", 9, NO_TABS), 3);
    }

    #[test]
    fn a_tab_reaches_the_next_stop() {
        // From a stop, partway, and from the column before a stop
        assert_eq!(glyph_width('\t', 0, 4), 4);
        assert_eq!(glyph_width('\t', 2, 4), 2);
        assert_eq!(glyph_width('\t', 3, 4), 1);
        assert_eq!(glyph_width('\t', 4, 4), 4);
        assert_eq!(glyph_width('\t', 5, 1), 1);
        // NO_TABS: no column, as before
        assert_eq!(glyph_width('\t', 2, NO_TABS), 0);
        assert_eq!(glyph_width('한', 3, 4), 2);

        let text = "a\t한\tb";
        // a 0, tab 1..4, 한 4..6, tab 6..8, b 8
        let cols: Vec<_> = (0..=5).map(|i| col_of(text, i, 4)).collect();
        assert_eq!(cols, vec![0, 1, 4, 6, 8, 9]);
        assert_eq!(col_of(text, 9, NO_TABS), 4);
        // Every column inside a tab lands on it
        let at: Vec<_> = (0..10).map(|c| char_at_col(text, c, 4)).collect();
        assert_eq!(at, vec![0, 1, 1, 1, 2, 2, 3, 3, 4, 5]);
        // The cursor on a tab covers the columns it takes
        assert_eq!(cursor_width(text, 1, 4), 3);
        assert_eq!(cursor_width(text, 3, 4), 2);
        assert_eq!(cursor_width(text, 1, NO_TABS), 1);
        // A tab takes a cell for the cursor unless NO_TABS
        assert_eq!(visible_from("\u{200B}\tx", 0, 4), 1);
        assert_eq!(visible_from("\u{200B}\tx", 0, NO_TABS), 2);
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
