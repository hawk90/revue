//! Progressive loader for chunked loading

use std::cell::RefCell;

/// Progressive loader for chunked loading
pub struct ProgressiveLoader<T> {
    /// All items to load
    items: Vec<T>,
    /// Number of items loaded so far
    loaded: RefCell<usize>,
    /// Chunk size
    chunk_size: usize,
}

impl<T: Clone> ProgressiveLoader<T> {
    /// Create a new progressive loader
    pub fn new(items: Vec<T>, chunk_size: usize) -> Self {
        Self {
            items,
            loaded: RefCell::new(0),
            chunk_size: chunk_size.max(1),
        }
    }

    /// Get total item count
    pub fn total(&self) -> usize {
        self.items.len()
    }

    /// Get number of items loaded
    pub fn loaded_count(&self) -> usize {
        *self.loaded.borrow()
    }

    /// Check if all items are loaded
    pub fn is_complete(&self) -> bool {
        *self.loaded.borrow() >= self.items.len()
    }

    /// Get loading progress (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        if self.items.is_empty() {
            1.0
        } else {
            *self.loaded.borrow() as f32 / self.items.len() as f32
        }
    }

    /// Load next chunk, returns the newly loaded items
    pub fn load_next(&self) -> Vec<T> {
        let start = *self.loaded.borrow();
        let end = (start + self.chunk_size).min(self.items.len());

        if start >= self.items.len() {
            return Vec::new();
        }

        let chunk: Vec<T> = self.items[start..end].to_vec();
        *self.loaded.borrow_mut() = end;
        chunk
    }

    /// Get all loaded items
    pub fn loaded_items(&self) -> Vec<T> {
        let count = *self.loaded.borrow();
        self.items[..count].to_vec()
    }

    /// Reset loading progress
    pub fn reset(&self) {
        *self.loaded.borrow_mut() = 0;
    }
}
