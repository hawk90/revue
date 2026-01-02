//! List selection with viewport scrolling
//!
//! Provides wrap-around navigation for lists with automatic viewport management.
//!
//! # Example
//! ```ignore
//! let mut sel = Selection::new(100); // 100 items
//! sel.set_visible(10); // 10 visible rows
//!
//! sel.next(); // Move to next item (wraps around)
//! sel.prev(); // Move to previous item (wraps around)
//!
//! // Render only visible items
//! for i in sel.visible_range() {
//!     render_item(i, i == sel.index);
//! }
//! ```

use std::cell::Cell;

/// Wrap-around index navigation with viewport scrolling
///
/// Uses `Cell` for offset/visible to allow updates from immutable context (e.g., render).
#[derive(Clone, Debug)]
pub struct Selection {
    /// Currently selected index
    pub index: usize,
    /// Total number of items
    pub len: usize,
    /// First visible item (scroll offset)
    offset: Cell<usize>,
    /// Number of visible items
    visible: Cell<usize>,
}

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
            offset: Cell::new(0),
            visible: Cell::new(usize::MAX),
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
            self.index = if self.index == 0 { self.len - 1 } else { self.index - 1 };
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

/// Wrap index forward (standalone function)
pub fn wrap_next(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else { (index + 1) % len }
}

/// Wrap index backward (standalone function)
pub fn wrap_prev(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else if index == 0 { len - 1 } else { index - 1 }
}

// ============================================================================
// Sectioned Selection (for collapsible sections)
// ============================================================================

use std::collections::HashMap;

/// Selection state for list view with collapsible sections
///
/// Commonly used in views where items are grouped by status, category, etc.
/// and each group can be collapsed/expanded.
///
/// # Example
///
/// ```ignore
/// use revue::utils::SectionedSelection;
///
/// let mut sel = SectionedSelection::new();
///
/// // Navigate
/// sel.next(&[5, 3, 2]); // section_sizes = [5 items, 3 items, 2 items]
/// sel.prev(&[5, 3, 2]);
///
/// // Toggle section
/// sel.toggle_section();
///
/// // Check state
/// if !sel.is_section_collapsed(0) {
///     // Render section 0 items...
/// }
/// ```
#[derive(Clone, Debug)]
pub struct SectionedSelection {
    /// Currently selected section index
    pub section: usize,
    /// Currently selected item index within the section
    pub item: usize,
    /// Map of section index -> collapsed state
    pub collapsed: HashMap<usize, bool>,
}

impl Default for SectionedSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl SectionedSelection {
    /// Create a new sectioned selection
    pub fn new() -> Self {
        Self {
            section: 0,
            item: 0,
            collapsed: HashMap::new(),
        }
    }

    /// Move to next item (wraps to next section if at end)
    ///
    /// # Arguments
    ///
    /// * `section_sizes` - Number of items in each section
    ///
    /// # Example
    ///
    /// ```ignore
    /// sel.next(&[5, 3, 2]); // 3 sections with 5, 3, 2 items
    /// ```
    pub fn next(&mut self, section_sizes: &[usize]) {
        if section_sizes.is_empty() {
            return;
        }

        let current_section_size = section_sizes.get(self.section).copied().unwrap_or(0);

        if current_section_size == 0 || self.item >= current_section_size - 1 {
            // Move to next section
            self.section = (self.section + 1) % section_sizes.len();
            self.item = 0;
        } else {
            // Move to next item in current section
            self.item += 1;
        }
    }

    /// Move to previous item (wraps to previous section if at start)
    pub fn prev(&mut self, section_sizes: &[usize]) {
        if section_sizes.is_empty() {
            return;
        }

        if self.item == 0 {
            // Move to previous section
            if self.section == 0 {
                self.section = section_sizes.len() - 1;
            } else {
                self.section -= 1;
            }
            let prev_section_size = section_sizes.get(self.section).copied().unwrap_or(0);
            self.item = prev_section_size.saturating_sub(1);
        } else {
            // Move to previous item in current section
            self.item -= 1;
        }
    }

    /// Jump to next section
    pub fn next_section(&mut self, section_count: usize) {
        if section_count > 0 {
            self.section = (self.section + 1) % section_count;
            self.item = 0;
        }
    }

    /// Jump to previous section
    pub fn prev_section(&mut self, section_count: usize) {
        if section_count > 0 {
            if self.section == 0 {
                self.section = section_count - 1;
            } else {
                self.section -= 1;
            }
            self.item = 0;
        }
    }

    /// Toggle current section's collapsed state
    pub fn toggle_section(&mut self) {
        let collapsed = self.collapsed.entry(self.section).or_insert(false);
        *collapsed = !*collapsed;
    }

    /// Check if a section is collapsed
    pub fn is_section_collapsed(&self, section: usize) -> bool {
        self.collapsed.get(&section).copied().unwrap_or(false)
    }

    /// Collapse a specific section
    pub fn collapse_section(&mut self, section: usize) {
        self.collapsed.insert(section, true);
    }

    /// Expand a specific section
    pub fn expand_section(&mut self, section: usize) {
        self.collapsed.insert(section, false);
    }

    /// Expand all sections
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Collapse all sections
    pub fn collapse_all(&mut self, section_count: usize) {
        for i in 0..section_count {
            self.collapsed.insert(i, true);
        }
    }

    /// Get current selection as (section, item)
    pub fn get(&self) -> (usize, usize) {
        (self.section, self.item)
    }

    /// Set selection to specific section and item
    pub fn set(&mut self, section: usize, item: usize) {
        self.section = section;
        self.item = item;
    }

    /// Reset to first section, first item
    pub fn reset(&mut self) {
        self.section = 0;
        self.item = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_prev_wrap() {
        let mut sel = Selection::new(3);
        assert_eq!(sel.index, 0);

        sel.next();
        assert_eq!(sel.index, 1);

        sel.next();
        assert_eq!(sel.index, 2);

        sel.next(); // wrap
        assert_eq!(sel.index, 0);

        sel.prev(); // wrap back
        assert_eq!(sel.index, 2);
    }

    #[test]
    fn test_viewport_scrolling() {
        let mut sel = Selection::new(10);
        sel.set_visible(3);

        assert_eq!(sel.offset(), 0);
        assert_eq!(sel.visible_range(), 0..3);

        sel.set(5);
        assert_eq!(sel.offset(), 3); // scrolled to show index 5
        assert_eq!(sel.visible_range(), 3..6);
    }

    #[test]
    fn test_set_len() {
        let mut sel = Selection::new(10);
        sel.set(9);
        assert_eq!(sel.index, 9);

        sel.set_len(5);
        assert_eq!(sel.index, 4); // clamped
    }

    #[test]
    fn test_page_navigation() {
        let mut sel = Selection::new(100);
        sel.set_visible(10);

        sel.page_down();
        assert_eq!(sel.index, 10);

        sel.page_down();
        assert_eq!(sel.index, 20);

        sel.page_up();
        assert_eq!(sel.index, 10);
    }

    #[test]
    fn test_wrap_functions() {
        assert_eq!(wrap_next(2, 3), 0);
        assert_eq!(wrap_prev(0, 3), 2);
        assert_eq!(wrap_next(0, 0), 0);
        assert_eq!(wrap_prev(0, 0), 0);
    }

    // =============================================================================
    // Edge Case Tests - Selection
    // =============================================================================

    #[test]
    fn test_empty_selection() {
        let mut sel = Selection::new(0);
        assert!(sel.is_empty());
        assert_eq!(sel.index, 0);

        // Operations on empty selection should be no-ops
        sel.next();
        assert_eq!(sel.index, 0);

        sel.prev();
        assert_eq!(sel.index, 0);

        sel.first();
        assert_eq!(sel.index, 0);

        sel.last();
        assert_eq!(sel.index, 0);
    }

    #[test]
    fn test_single_item_selection() {
        let mut sel = Selection::new(1);
        assert!(!sel.is_empty());
        assert_eq!(sel.index, 0);

        // next/prev should wrap to same item
        sel.next();
        assert_eq!(sel.index, 0);

        sel.prev();
        assert_eq!(sel.index, 0);
    }

    #[test]
    fn test_first_and_last() {
        let mut sel = Selection::new(10);
        sel.set(5);
        assert_eq!(sel.index, 5);

        sel.first();
        assert_eq!(sel.index, 0);
        assert_eq!(sel.offset(), 0);

        sel.last();
        assert_eq!(sel.index, 9);
    }

    #[test]
    fn test_up_down_no_wrap() {
        let mut sel = Selection::new(5);

        // down should not wrap
        sel.down();
        assert_eq!(sel.index, 1);
        sel.set(4);
        sel.down();
        assert_eq!(sel.index, 4); // stays at end

        // up should not wrap
        sel.set(1);
        sel.up();
        assert_eq!(sel.index, 0);
        sel.up();
        assert_eq!(sel.index, 0); // stays at start
    }

    #[test]
    fn test_is_selected() {
        let mut sel = Selection::new(5);
        sel.set(2);

        assert!(!sel.is_selected(0));
        assert!(!sel.is_selected(1));
        assert!(sel.is_selected(2));
        assert!(!sel.is_selected(3));
    }

    #[test]
    fn test_set_out_of_bounds() {
        let mut sel = Selection::new(5);

        // set beyond length should clamp
        sel.set(100);
        assert_eq!(sel.index, 4);

        // set at exact boundary
        sel.set(4);
        assert_eq!(sel.index, 4);
    }

    #[test]
    fn test_select_alias() {
        let mut sel = Selection::new(10);
        sel.select(5);
        assert_eq!(sel.index, 5);
    }

    #[test]
    fn test_reset_offset() {
        let mut sel = Selection::new(10);
        sel.set_visible(3);
        sel.set(8); // scroll down
        assert!(sel.offset() > 0);

        sel.reset_offset();
        assert_eq!(sel.offset(), 0);
    }

    #[test]
    fn test_visible_greater_than_len() {
        let mut sel = Selection::new(5);
        sel.set_visible(100); // more visible than items

        sel.set(4);
        assert_eq!(sel.offset(), 0); // no scrolling needed
        assert_eq!(sel.visible_range(), 0..5);
    }

    #[test]
    fn test_page_down_at_end() {
        let mut sel = Selection::new(20);
        sel.set_visible(10);

        sel.set(15);
        sel.page_down();
        assert_eq!(sel.index, 19); // clamped to last item
    }

    #[test]
    fn test_page_up_at_start() {
        let mut sel = Selection::new(20);
        sel.set_visible(10);

        sel.set(5);
        sel.page_up();
        assert_eq!(sel.index, 0); // clamped to first item
    }

    #[test]
    fn test_default() {
        let sel = Selection::default();
        assert!(sel.is_empty());
        assert_eq!(sel.index, 0);
        assert_eq!(sel.len, 0);
    }

    #[test]
    fn test_set_len_to_zero() {
        let mut sel = Selection::new(10);
        sel.set(5);
        sel.set_len(0);
        assert!(sel.is_empty());
        // Index should remain but operations become no-ops
        sel.next();
        sel.prev();
    }

    #[test]
    fn test_visible_range_empty() {
        let sel = Selection::new(0);
        assert_eq!(sel.visible_range(), 0..0);
    }

    #[test]
    fn test_scroll_up_when_selection_above_viewport() {
        let mut sel = Selection::new(10);
        sel.set_visible(3);
        sel.set(8); // scroll to bottom
        assert_eq!(sel.offset(), 6);

        sel.set(2); // select above viewport
        assert_eq!(sel.offset(), 2); // scrolled up to show item 2
    }

    // =============================================================================
    // Edge Case Tests - SectionedSelection
    // =============================================================================

    #[test]
    fn test_sectioned_new() {
        let sel = SectionedSelection::new();
        assert_eq!(sel.section, 0);
        assert_eq!(sel.item, 0);
        assert!(sel.collapsed.is_empty());
    }

    #[test]
    fn test_sectioned_default() {
        let sel = SectionedSelection::default();
        assert_eq!(sel.get(), (0, 0));
    }

    #[test]
    fn test_sectioned_next_within_section() {
        let mut sel = SectionedSelection::new();
        sel.next(&[5, 3, 2]);
        assert_eq!(sel.get(), (0, 1));

        sel.next(&[5, 3, 2]);
        assert_eq!(sel.get(), (0, 2));
    }

    #[test]
    fn test_sectioned_next_across_sections() {
        let mut sel = SectionedSelection::new();
        sel.set(0, 4); // last item of section 0

        sel.next(&[5, 3, 2]);
        assert_eq!(sel.get(), (1, 0)); // moved to section 1, item 0
    }

    #[test]
    fn test_sectioned_next_wrap() {
        let mut sel = SectionedSelection::new();
        sel.set(2, 1); // last section, last item

        sel.next(&[5, 3, 2]);
        assert_eq!(sel.get(), (0, 0)); // wrapped to first
    }

    #[test]
    fn test_sectioned_prev_within_section() {
        let mut sel = SectionedSelection::new();
        sel.set(0, 2);

        sel.prev(&[5, 3, 2]);
        assert_eq!(sel.get(), (0, 1));
    }

    #[test]
    fn test_sectioned_prev_across_sections() {
        let mut sel = SectionedSelection::new();
        sel.set(1, 0); // first item of section 1

        sel.prev(&[5, 3, 2]);
        assert_eq!(sel.get(), (0, 4)); // last item of section 0
    }

    #[test]
    fn test_sectioned_prev_wrap() {
        let mut sel = SectionedSelection::new();
        // Already at (0, 0)
        sel.prev(&[5, 3, 2]);
        assert_eq!(sel.get(), (2, 1)); // wrapped to last section, last item
    }

    #[test]
    fn test_sectioned_empty_sections() {
        let mut sel = SectionedSelection::new();
        // Empty section_sizes
        sel.next(&[]);
        assert_eq!(sel.get(), (0, 0)); // no change

        sel.prev(&[]);
        assert_eq!(sel.get(), (0, 0)); // no change
    }

    #[test]
    fn test_sectioned_next_section() {
        let mut sel = SectionedSelection::new();
        sel.set(0, 3);

        sel.next_section(3);
        assert_eq!(sel.get(), (1, 0)); // moved to section 1, item reset to 0
    }

    #[test]
    fn test_sectioned_prev_section() {
        let mut sel = SectionedSelection::new();
        sel.set(1, 2);

        sel.prev_section(3);
        assert_eq!(sel.get(), (0, 0)); // moved to section 0, item reset to 0
    }

    #[test]
    fn test_sectioned_prev_section_wrap() {
        let mut sel = SectionedSelection::new();
        sel.prev_section(3);
        assert_eq!(sel.get(), (2, 0)); // wrapped to last section
    }

    #[test]
    fn test_sectioned_toggle_collapse() {
        let mut sel = SectionedSelection::new();
        assert!(!sel.is_section_collapsed(0));

        sel.toggle_section();
        assert!(sel.is_section_collapsed(0));

        sel.toggle_section();
        assert!(!sel.is_section_collapsed(0));
    }

    #[test]
    fn test_sectioned_collapse_expand() {
        let mut sel = SectionedSelection::new();

        sel.collapse_section(1);
        assert!(sel.is_section_collapsed(1));

        sel.expand_section(1);
        assert!(!sel.is_section_collapsed(1));
    }

    #[test]
    fn test_sectioned_collapse_expand_all() {
        let mut sel = SectionedSelection::new();

        sel.collapse_all(3);
        assert!(sel.is_section_collapsed(0));
        assert!(sel.is_section_collapsed(1));
        assert!(sel.is_section_collapsed(2));

        sel.expand_all();
        assert!(!sel.is_section_collapsed(0));
        assert!(!sel.is_section_collapsed(1));
        assert!(!sel.is_section_collapsed(2));
    }

    #[test]
    fn test_sectioned_reset() {
        let mut sel = SectionedSelection::new();
        sel.set(2, 5);
        assert_eq!(sel.get(), (2, 5));

        sel.reset();
        assert_eq!(sel.get(), (0, 0));
    }

    #[test]
    fn test_sectioned_empty_section_navigation() {
        let mut sel = SectionedSelection::new();
        // Section with 0 items should move to next section
        sel.next(&[0, 3, 2]);
        assert_eq!(sel.get(), (1, 0));
    }

    #[test]
    fn test_sectioned_next_section_zero_count() {
        let mut sel = SectionedSelection::new();
        sel.next_section(0); // edge case: no sections
        assert_eq!(sel.get(), (0, 0)); // unchanged
    }
}
