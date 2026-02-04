//! Lazy list that loads items on demand

use std::cell::RefCell;

/// Lazy list that loads items on demand
pub struct LazyList<T> {
    /// Loaded items (sparse)
    items: RefCell<Vec<Option<T>>>,
    /// Total capacity
    capacity: usize,
}

impl<T: Clone> LazyList<T> {
    /// Create a new lazy list with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            items: RefCell::new(vec![None; capacity]),
            capacity,
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<T> {
        self.items.borrow().get(index).and_then(|opt| opt.clone())
    }

    /// Set item at index
    pub fn set(&self, index: usize, value: T) {
        if let Some(slot) = self.items.borrow_mut().get_mut(index) {
            *slot = Some(value);
        }
    }

    /// Check if item is loaded
    pub fn is_loaded(&self, index: usize) -> bool {
        self.items
            .borrow()
            .get(index)
            .map(|opt| opt.is_some())
            .unwrap_or(false)
    }

    /// Get number of loaded items
    pub fn loaded_count(&self) -> usize {
        self.items
            .borrow()
            .iter()
            .filter(|opt| opt.is_some())
            .count()
    }

    /// Clear item at index
    pub fn clear(&self, index: usize) {
        if let Some(slot) = self.items.borrow_mut().get_mut(index) {
            *slot = None;
        }
    }

    /// Clear all items
    pub fn clear_all(&self) {
        for slot in self.items.borrow_mut().iter_mut() {
            *slot = None;
        }
    }
}
