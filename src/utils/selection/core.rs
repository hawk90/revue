//! List selection with viewport scrolling - Selection implementation

use super::types::Selection;

impl Default for Selection {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Selection {
    /// Create a new selection with given item count
    pub fn new(len: usize) -> Self {
        Self {
            index: 0,
            len,
            offset: std::cell::Cell::new(0),
            visible: std::cell::Cell::new(usize::MAX),
        }
    }

    /// Move to next item (wraps to 0 at end)
    pub fn next(&mut self) {
        if self.len > 0 {
            self.index = (self.index + 1) % self.len;
            self.ensure_visible();
        }
    }

    /// Move to previous item (wraps to end at 0)
    pub fn prev(&mut self) {
        if self.len > 0 {
            self.index = if self.index == 0 {
                self.len - 1
            } else {
                self.index - 1
            };
            self.ensure_visible();
        }
    }

    /// Move down without wrapping
    pub fn down(&mut self) {
        if self.len > 0 && self.index < self.len - 1 {
            self.index += 1;
            self.ensure_visible();
        }
    }

    /// Move up without wrapping
    pub fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.ensure_visible();
        }
    }

    /// Jump to first item
    pub fn first(&mut self) {
        self.index = 0;
        self.offset.set(0);
    }

    /// Jump to last item
    pub fn last(&mut self) {
        if self.len > 0 {
            self.index = self.len - 1;
            self.ensure_visible();
        }
    }

    /// Set index with bounds check
    pub fn set(&mut self, index: usize) {
        self.index = index.min(self.len.saturating_sub(1));
        self.ensure_visible();
    }

    /// Select specific index (alias for set)
    pub fn select(&mut self, index: usize) {
        self.set(index);
    }

    /// Update length (adjusts index if needed)
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
        if self.index >= len && len > 0 {
            self.index = len - 1;
        }
        self.ensure_visible();
    }

    /// Set visible count (can be called from &self context due to Cell)
    pub fn set_visible(&self, visible: usize) {
        self.visible.set(visible);
        self.ensure_visible();
    }

    /// Ensure selected item is visible by adjusting offset
    fn ensure_visible(&self) {
        let visible = self.visible.get();
        if visible == 0 {
            return;
        }

        // If everything fits, reset offset
        if visible >= self.len {
            self.offset.set(0);
            return;
        }

        let mut offset = self.offset.get();

        // Scroll down if selection is below viewport
        if self.index >= offset + visible {
            offset = self.index.saturating_sub(visible - 1);
        }

        // Scroll up if selection is above viewport
        if self.index < offset {
            offset = self.index;
        }

        // Clamp offset
        let max_offset = self.len.saturating_sub(visible);
        self.offset.set(offset.min(max_offset));
    }

    /// Get current scroll offset
    pub fn offset(&self) -> usize {
        self.offset.get()
    }

    /// Get visible range (start..end indices)
    pub fn visible_range(&self) -> std::ops::Range<usize> {
        let offset = self.offset.get();
        let end = (offset + self.visible.get()).min(self.len);
        offset..end
    }

    /// Reset offset to 0 (scroll to top)
    pub fn reset_offset(&mut self) {
        self.offset.set(0);
    }

    /// Check if index is currently selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.index == index
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Page down (move by visible count)
    pub fn page_down(&mut self) {
        let visible = self.visible.get();
        if visible > 0 && self.len > 0 {
            self.index = (self.index + visible).min(self.len - 1);
            self.ensure_visible();
        }
    }

    /// Page up (move by visible count)
    pub fn page_up(&mut self) {
        let visible = self.visible.get();
        if visible > 0 {
            self.index = self.index.saturating_sub(visible);
            self.ensure_visible();
        }
    }
}
