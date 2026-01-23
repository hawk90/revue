//! List selection with viewport scrolling - type definitions

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
    pub(crate) offset: Cell<usize>,
    /// Number of visible items
    pub(crate) visible: Cell<usize>,
}

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
