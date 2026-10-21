//! Scroll position: jumping, scrolling by rows and keeping the selection in
//! view. The view is a ScrollState in rows, so items of different heights
//! and an offset inside an item are drawn as they scroll (#971).

use super::core::VirtualList;
use super::types::ScrollAlignment;
use crate::utils::ScrollAlign;
use std::ops::Range;

impl<T: ToString + Clone> VirtualList<T> {
    /// Jump to a specific index with alignment
    pub fn jump_to(&mut self, index: usize) {
        self.jump_to_with_alignment(index, ScrollAlignment::Start);
    }

    /// Select `index` and scroll it to `alignment`: the top, the middle or
    /// the bottom of the view, or just into view. The view stops at either
    /// end, so a jump near the end shows a full last page. Before the first
    /// draw the jump waits for the viewport.
    pub fn jump_to_with_alignment(&mut self, index: usize, alignment: ScrollAlignment) {
        if index >= self.items.len() {
            return;
        }
        self.selected = Some(index);
        let align = match alignment {
            ScrollAlignment::Start => ScrollAlign::Start,
            ScrollAlignment::Center => ScrollAlign::Center,
            ScrollAlignment::End => ScrollAlign::End,
            ScrollAlignment::Nearest => ScrollAlign::Nearest,
        };
        self.adopt_offsets();
        let rows = self.rows_of(index);
        self.view_mut().reveal_aligned(rows, align);
        self.sync_offsets();
    }

    /// Scroll by a number of rows, within an item as well as across items
    pub fn scroll_by(&mut self, rows: i32) {
        self.adopt_offsets();
        // The selection as it is now is the one the view follows: the next
        // draw keeps this scroll until the selection moves
        self.track_selection();
        self.view_mut().scroll_by(rows as isize);
        self.sync_offsets();
    }

    /// Get scroll position as percentage (0.0 - 1.0)
    pub fn scroll_position(&self) -> f32 {
        self.adopt_offsets();
        let max = self.view().max_offset();
        if max == 0 {
            0.0
        } else {
            self.view().offset() as f32 / max as f32
        }
    }

    /// Set scroll position by percentage (0.0 - 1.0)
    pub fn set_scroll_position(&mut self, position: f32) {
        let position = position.clamp(0.0, 1.0);
        self.adopt_offsets();
        self.track_selection();
        let row = (position * self.view().max_offset() as f32) as usize;
        self.view_mut().scroll_to(row);
        self.sync_offsets();
    }

    /// Items that fit in `viewport_height` rows at the uniform height; at
    /// least one. Used for page moves.
    pub(super) fn visible_count(&self, viewport_height: u16) -> usize {
        usize::from(viewport_height / self.item_height.max(1)).max(1)
    }

    /// The rows item `index` takes
    pub(super) fn rows_of(&self, index: usize) -> Range<usize> {
        let start = self.row_of_index(index) as usize;
        start..start + usize::from(self.get_item_height(index).max(1))
    }

    /// Give the selection's rows to the view to follow: revealed now if they
    /// changed, kept in the middle in Center mode
    pub(super) fn track_selection(&self) {
        if let Some(index) = self.selected.filter(|&i| i < self.items.len()) {
            let rows = self.rows_of(index);
            if self.scroll_mode == super::types::ScrollMode::Center {
                self.view().track_aligned(rows, ScrollAlign::Center);
            } else {
                self.view().track(rows);
            }
        }
    }

    /// Give the view the rows, and take up offsets the app wrote to the
    /// public fields since they were last mirrored
    pub(super) fn adopt_offsets(&self) {
        self.view().set_content(self.total_height() as usize);
        let fields = (self.scroll_offset, self.scroll_sub_offset);
        if fields != self.mirrored().get() {
            let row = self.row_of_index(self.scroll_offset) as usize
                + usize::from(self.scroll_sub_offset);
            self.view().set_offset(row);
            self.mirrored().set(fields);
        }
    }

    /// Mirror the view's offset in the public item and row offsets
    pub(super) fn sync_offsets(&mut self) {
        let row = self.view().offset() as u32;
        self.scroll_offset = if self.items.is_empty() {
            0
        } else {
            self.index_at_row(row)
        };
        let start = self.row_of_index(self.scroll_offset);
        self.scroll_sub_offset = row.saturating_sub(start).min(u32::from(u16::MAX)) as u16;
        self.mirrored()
            .set((self.scroll_offset, self.scroll_sub_offset));
    }

    /// Scroll to make selected item visible
    pub fn ensure_visible(&mut self, viewport_height: u16) {
        self.adopt_offsets();
        self.view().set_viewport(usize::from(viewport_height));
        if let Some(index) = self.selected {
            let rows = self.rows_of(index);
            self.view_mut().reveal(rows);
        }
        self.sync_offsets();
    }

    /// The items with a row in view, and `overscan` more on either side
    pub fn visible_range(&self, viewport_height: u16) -> Range<usize> {
        if self.items.is_empty() {
            return 0..0;
        }
        self.adopt_offsets();
        let top = self.view().offset() as u32;
        let bottom = top + u32::from(viewport_height.max(1)) - 1;
        let first = self.index_at_row(top);
        let last = self.index_at_row(bottom);
        first.saturating_sub(self.overscan)..(last + 1 + self.overscan).min(self.items.len())
    }
}
