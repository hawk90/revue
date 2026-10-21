//! Scroll state for a view, in rows
//!
//! [`ScrollState`] is where a view is scrolled to, kept apart from what is
//! selected in it (#973). It works in rows, so items of different heights
//! scroll the same way as one-row items: the widget says which rows an item
//! takes.
//!
//! - The offset counts rows from the top and stays in
//!   `0..=content - viewport`.
//! - [`ScrollState::reveal`] scrolls just enough to show a range of rows, in
//!   either direction, as `scrollIntoView({ block: "nearest" })` does. Call it
//!   when the selection changes, or give the selected rows to
//!   [`track`](ScrollState::track) where the view is drawn.
//! - A view with a tail mode ([`follow_end`](ScrollState::follow_end)) follows
//!   the end while it is at the end: content added below keeps the last page
//!   in view. Scrolling up stops that; reaching the end again starts it. A
//!   view without one never follows.
//! - Rows inserted or removed above the view move the offset with them, so
//!   the rows on screen stay put.
//!
//! The viewport is only known when the view is drawn, so
//! [`set_viewport`](ScrollState::set_viewport) and
//! [`set_content`](ScrollState::set_content) take `&self` and can be called
//! from `render`. A [`reveal`](ScrollState::reveal) made before the first
//! draw waits for the viewport.
//!
//! # Example
//!
//! ```
//! use revue::utils::ScrollState;
//!
//! let mut scroll = ScrollState::new();
//! scroll.set_content(100);
//! scroll.set_viewport(10); // in render
//!
//! scroll.reveal(40..41); // the selected item moved to row 40
//! assert_eq!(scroll.visible(), 31..41);
//!
//! scroll.scroll_to_end();
//! scroll.set_content(120); // new rows below: the view follows
//! assert_eq!(scroll.visible(), 110..120);
//! ```

use std::cell::Cell;
use std::ops::Range;

/// Where a view is scrolled to, in rows. See the [module docs](self).
#[derive(Clone, Debug, Default)]
pub struct ScrollState {
    offset: Cell<usize>,
    viewport: Cell<usize>,
    content: Cell<usize>,
    follow_end: Cell<bool>,
    /// Whether the view has a tail mode at all: set once following is
    /// turned on. A list without one never starts following by reaching its
    /// end.
    tail: Cell<bool>,
    /// A reveal made while the viewport was unknown
    pending: Cell<Option<(usize, usize)>>,
    /// The rows last given to [`track`](ScrollState::track)
    tracked: Cell<Option<(usize, usize)>>,
}

impl ScrollState {
    /// A view at the top, not following the end
    pub fn new() -> Self {
        Self::default()
    }

    /// Start following the end (a log or a stream), or not
    pub fn follow_end(self, follow: bool) -> Self {
        self.tail.set(true);
        self.follow_end.set(follow);
        self.settle();
        self
    }

    /// Follow the end from now on, or stop following and keep the view
    /// where it is (a paused log)
    pub fn set_follow_end(&mut self, follow: bool) {
        self.tail.set(self.tail.get() || follow);
        self.follow_end.set(follow);
        self.settle();
    }

    /// The first row shown
    pub fn offset(&self) -> usize {
        self.offset.get()
    }

    /// How many rows the view shows (0 until it is drawn)
    pub fn viewport(&self) -> usize {
        self.viewport.get()
    }

    /// How many rows the content has
    pub fn content(&self) -> usize {
        self.content.get()
    }

    /// Whether the view follows the end
    pub fn is_following(&self) -> bool {
        self.follow_end.get()
    }

    /// The largest offset: the one that shows the last page full
    pub fn max_offset(&self) -> usize {
        self.content.get().saturating_sub(self.viewport.get())
    }

    /// The rows on screen
    pub fn visible(&self) -> Range<usize> {
        let end = (self.offset.get() + self.viewport.get()).min(self.content.get());
        self.offset.get().min(end)..end
    }

    /// Whether there are rows above the view
    pub fn has_above(&self) -> bool {
        self.offset.get() > 0
    }

    /// Whether there are rows below the view
    pub fn has_below(&self) -> bool {
        self.offset.get() + self.viewport.get() < self.content.get()
    }

    /// Set the number of rows the view shows. Called where the view is drawn.
    pub fn set_viewport(&self, rows: usize) {
        self.viewport.set(rows);
        if rows > 0 {
            if let Some((start, end)) = self.pending.take() {
                self.apply_reveal(start, end);
            }
        }
        self.settle();
    }

    /// Set the number of rows the content has. For content that grows or
    /// shrinks above the view, use [`rows_inserted`](Self::rows_inserted) and
    /// [`rows_removed`](Self::rows_removed) instead, so the view stays put.
    pub fn set_content(&self, rows: usize) {
        self.content.set(rows);
        self.settle();
    }

    /// `count` rows were inserted at row `at`. Rows inserted above the view
    /// move it down with the rows it shows.
    pub fn rows_inserted(&self, at: usize, count: usize) {
        self.content.set(self.content.get() + count);
        let offset = self.offset.get();
        if at < offset {
            self.offset.set(offset + count);
        }
        self.settle();
    }

    /// `count` rows were removed from row `at`. Rows removed above the view
    /// move it up with the rows it shows.
    pub fn rows_removed(&self, at: usize, count: usize) {
        let count = count.min(self.content.get().saturating_sub(at));
        self.content.set(self.content.get() - count);
        let offset = self.offset.get();
        if at + count <= offset {
            self.offset.set(offset - count);
        } else if at < offset {
            self.offset.set(at);
        }
        self.settle();
    }

    /// Scroll by `rows` (negative is up), without moving any selection
    pub fn scroll_by(&mut self, rows: isize) {
        let target = if rows < 0 {
            self.offset.get().saturating_sub(rows.unsigned_abs())
        } else {
            self.offset.get().saturating_add(rows as usize)
        };
        self.scroll_to(target);
    }

    /// Scroll so that `row` is the first row shown, or as near as the
    /// content allows
    pub fn scroll_to(&mut self, row: usize) {
        self.pending.set(None);
        self.offset.set(row.min(self.max_offset()));
        self.follow_end.set(self.tail.get() && self.at_end());
    }

    /// Scroll to the first row
    pub fn scroll_to_top(&mut self) {
        self.scroll_to(0);
    }

    /// Scroll to the last page, and follow the end in a view with a tail
    /// mode
    pub fn scroll_to_end(&mut self) {
        self.pending.set(None);
        if self.tail.get() {
            self.follow_end.set(true);
            self.settle();
        } else {
            self.offset.set(self.max_offset());
        }
    }

    /// Scroll up by one page
    pub fn page_up(&mut self) {
        self.scroll_by(-(self.viewport.get().max(1) as isize));
    }

    /// Scroll down by one page
    pub fn page_down(&mut self) {
        self.scroll_by(self.viewport.get().max(1) as isize);
    }

    /// Scroll just enough to show `rows`, in either direction. A range taller
    /// than the view shows its first rows. Before the view is drawn, the
    /// reveal waits for [`set_viewport`](Self::set_viewport).
    ///
    /// Following the end changes only when the view moves: a reveal that
    /// scrolls up stops it, one that scrolls to the end starts it, and one
    /// already on screen leaves it as it is.
    pub fn reveal(&mut self, rows: Range<usize>) {
        let (start, end) = (rows.start, rows.end.max(rows.start + 1));
        if self.viewport.get() == 0 {
            self.pending.set(Some((start, end)));
            return;
        }
        self.apply_reveal(start, end);
        self.settle();
    }

    /// Reveal `rows` when they differ from the rows given last time. Call it
    /// where the view is drawn with the selected item's rows: the view then
    /// follows the selection whichever code moved it, and scrolling away from
    /// an unchanged selection is kept.
    pub fn track(&self, rows: Range<usize>) {
        let key = (rows.start, rows.end.max(rows.start + 1));
        if self.tracked.replace(Some(key)) == Some(key) {
            return;
        }
        if self.viewport.get() == 0 {
            self.pending.set(Some(key));
            return;
        }
        self.apply_reveal(key.0, key.1);
        self.settle();
    }

    fn apply_reveal(&self, start: usize, end: usize) {
        let viewport = self.viewport.get();
        let offset = self.offset.get();
        let target = if start < offset || end - start > viewport {
            start
        } else if end > offset + viewport {
            end - viewport
        } else {
            offset
        };
        let target = target.min(self.max_offset());
        if target != offset {
            self.offset.set(target);
            self.follow_end.set(self.tail.get() && self.at_end());
        }
    }

    fn at_end(&self) -> bool {
        self.offset.get() >= self.max_offset()
    }

    /// Keep the offset in range, and on the last page while following.
    /// Before the view is drawn (viewport 0) the range is the whole content.
    fn settle(&self) {
        let max = self.max_offset();
        if self.follow_end.get() {
            self.offset.set(max);
        } else {
            self.offset.set(self.offset.get().min(max));
        }
    }
}
