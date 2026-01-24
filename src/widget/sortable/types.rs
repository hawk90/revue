//! Type definitions for the sortable list widget

use std::sync::atomic::{AtomicU64, Ordering};

use crate::event::drag::DragId;

/// Atomic counter for generating unique sortable list IDs
static SORTABLE_ID_COUNTER: AtomicU64 = AtomicU64::new(1000);

/// Item in a sortable list
#[derive(Debug, Clone)]
pub struct SortableItem {
    /// Item label
    pub label: String,
    /// Is item selected
    pub selected: bool,
    /// Is item being dragged
    pub dragging: bool,
    /// Original index (before any reordering)
    pub original_index: usize,
}

impl SortableItem {
    /// Create a new sortable item
    pub fn new(label: impl Into<String>, index: usize) -> Self {
        Self {
            label: label.into(),
            selected: false,
            dragging: false,
            original_index: index,
        }
    }
}

/// Reorder callback type
pub type ReorderCallback = Box<dyn FnMut(usize, usize)>;

/// Generate a unique sortable list ID
pub fn generate_id() -> DragId {
    SORTABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
