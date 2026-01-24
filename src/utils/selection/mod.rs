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

mod core;
mod helper;
mod sectioned;
mod types;

pub use types::{SectionedSelection, Selection};

// Re-export helper functions
pub use helper::{wrap_next, wrap_prev};

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::selection::{wrap_next, wrap_prev, SectionedSelection, Selection};

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
