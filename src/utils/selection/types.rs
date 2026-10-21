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
    /// An index set past the end (a builder before the items): kept until
    /// [`set_len`](Selection::set_len) gives the items, so the same calls
    /// in either order select the same item (#1011)
    requested: Option<usize>,
}

impl Selection {
    /// Create a new selection with given item count
    pub fn new(len: usize) -> Self {
        Self {
            index: 0,
            len,
            offset: Cell::new(0),
            visible: Cell::new(usize::MAX),
            requested: None,
        }
    }

    /// Remember (or forget) an index past the end
    pub(super) fn set_request(&mut self, index: Option<usize>) {
        self.requested = index;
    }

    /// The index remembered past the end, if any
    pub(super) fn request(&self) -> Option<usize> {
        self.requested
    }
}

use std::collections::HashMap;

/// Selection state for list view with collapsible sections
///
/// Commonly used in views where items are grouped by status, category, etc.
/// and each group can be collapsed/expanded.
///
/// # Example
///
/// ```
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
