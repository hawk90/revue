//! The terminal's own cursor, shown at the focused field's caret (#1457)

use crossterm::queue;
use std::io::Write;

use super::Terminal;
use crate::render::{diff, Buffer};
use crate::Result;

impl<W: Write> Terminal<W> {
    /// Show `buffer`'s caret with the terminal's cursor rather than as the
    /// cells a field drew for it (#1457): an input method puts its preedit
    /// and candidate window at the cursor, so the cursor has to be at the
    /// field.
    ///
    /// `changes` then draw the caret's cells as they look without it, and
    /// redraw the cells last drawn that way if no caret covers them now,
    /// since the buffers the changes come from never held them. Returns
    /// where the cursor goes.
    pub(crate) fn lift_caret(
        &mut self,
        changes: &mut Vec<diff::Change>,
        buffer: &Buffer,
    ) -> Option<(u16, u16)> {
        let caret = buffer.caret();
        let span = caret.map(|c| (c.x, c.y, c.under.len() as u16));
        let covers = |span: Option<(u16, u16, u16)>, x: u16, y: u16| {
            span.is_some_and(|(cx, cy, w)| y == cy && x >= cx && x - cx < w)
        };
        if let Some((lx, ly, lw)) = self.lifted.take() {
            for x in lx..lx.saturating_add(lw) {
                let stale = !covers(span, x, ly) && !changes.iter().any(|c| c.x == x && c.y == ly);
                if let Some(cell) = buffer.get(x, ly).filter(|_| stale) {
                    if self.current.get(x, ly) != Some(cell) {
                        changes.push(diff::Change {
                            x,
                            y: ly,
                            cell: *cell,
                        });
                    }
                }
            }
        }
        let caret = caret?;
        changes.retain(|c| !covers(span, c.x, c.y));
        for (x, cell) in (caret.x..).zip(&caret.under) {
            if self.current.get(x, caret.y) != Some(cell) {
                changes.push(diff::Change {
                    x,
                    y: caret.y,
                    cell: *cell,
                });
            }
        }
        self.lifted = span;
        Some((caret.x, caret.y))
    }

    /// Show the cursor at `at`, or hide it. It moves when it has to: to a
    /// new place, or back after `wrote` cells moved it.
    pub(crate) fn place_cursor(&mut self, at: Option<(u16, u16)>, wrote: bool) -> Result<()> {
        use crossterm::cursor::{Hide, MoveTo, Show};
        match at {
            Some((x, y)) => {
                if wrote || self.cursor != at {
                    queue!(self.writer, MoveTo(x, y))?;
                }
                if self.cursor.is_none() {
                    queue!(self.writer, Show)?;
                }
            }
            None if self.cursor.is_some() => queue!(self.writer, Hide)?,
            None => {}
        }
        self.cursor = at;
        Ok(())
    }

    /// The screen was cleared or resized: no cell is drawn in place of a
    /// caret any more
    pub(crate) fn forget_lifted(&mut self) {
        self.lifted = None;
    }

    /// Where the cursor is shown, `None` while it is hidden
    pub(crate) fn cursor_at(&self) -> Option<(u16, u16)> {
        self.cursor
    }

    /// The cursor was hidden outside [`place_cursor`](Self::place_cursor)
    pub(crate) fn cursor_hidden(&mut self) {
        self.cursor = None;
    }
}
