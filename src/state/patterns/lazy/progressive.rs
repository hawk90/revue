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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progressive_loader_new() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(loader.total(), 5);
        assert_eq!(loader.loaded_count(), 0);
        assert!(!loader.is_complete());
    }

    #[test]
    fn test_progressive_loader_total() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3], 1);
        assert_eq!(loader.total(), 3);

        let empty: ProgressiveLoader<i32> = ProgressiveLoader::new(vec![], 1);
        assert_eq!(empty.total(), 0);
    }

    #[test]
    fn test_progressive_loader_loaded_count() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(loader.loaded_count(), 0);
        loader.load_next();
        assert_eq!(loader.loaded_count(), 2);
        loader.load_next();
        assert_eq!(loader.loaded_count(), 4);
        loader.load_next();
        assert_eq!(loader.loaded_count(), 5);
    }

    #[test]
    fn test_progressive_loader_is_complete() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3], 3);
        assert!(!loader.is_complete());
        loader.load_next();
        assert!(loader.is_complete());
    }

    #[test]
    fn test_progressive_loader_progress() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(loader.progress(), 0.0);
        loader.load_next();
        assert_eq!(loader.progress(), 2.0 / 5.0);
        loader.load_next();
        assert_eq!(loader.progress(), 4.0 / 5.0);
        loader.load_next();
        assert_eq!(loader.progress(), 1.0);
    }

    #[test]
    fn test_progressive_loader_progress_empty() {
        let loader: ProgressiveLoader<i32> = ProgressiveLoader::new(vec![], 1);
        assert_eq!(loader.progress(), 1.0);
    }

    #[test]
    fn test_progressive_loader_load_next() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3, 4, 5], 2);
        let chunk1 = loader.load_next();
        assert_eq!(chunk1, vec![1, 2]);
        let chunk2 = loader.load_next();
        assert_eq!(chunk2, vec![3, 4]);
        let chunk3 = loader.load_next();
        assert_eq!(chunk3, vec![5]);
        let chunk4 = loader.load_next();
        assert!(chunk4.is_empty());
    }

    #[test]
    fn test_progressive_loader_loaded_items() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(loader.loaded_items(), vec![] as Vec<i32>);
        loader.load_next();
        assert_eq!(loader.loaded_items(), vec![1, 2]);
        loader.load_next();
        assert_eq!(loader.loaded_items(), vec![1, 2, 3, 4]);
        loader.load_next();
        assert_eq!(loader.loaded_items(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_progressive_loader_reset() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3, 4, 5], 2);
        loader.load_next();
        loader.load_next();
        assert_eq!(loader.loaded_count(), 4);
        loader.reset();
        assert_eq!(loader.loaded_count(), 0);
        assert!(!loader.is_complete());
    }

    #[test]
    fn test_progressive_loader_with_strings() {
        let loader = ProgressiveLoader::new(vec!["a", "b", "c"], 2);
        let chunk1 = loader.load_next();
        assert_eq!(chunk1, vec!["a", "b"]);
        let chunk2 = loader.load_next();
        assert_eq!(chunk2, vec!["c"]);
    }

    #[test]
    fn test_progressive_loader_chunk_size_minimum() {
        let loader = ProgressiveLoader::new(vec![1, 2, 3], 0);
        // chunk_size should be max(1, 0) = 1
        let chunk1 = loader.load_next();
        assert_eq!(chunk1, vec![1]);
        assert_eq!(loader.loaded_count(), 1);
    }

    #[test]
    fn test_progressive_loader_empty_vec() {
        let loader: ProgressiveLoader<i32> = ProgressiveLoader::new(vec![], 1);
        assert!(loader.is_complete());
        assert_eq!(loader.load_next(), vec![]);
        assert_eq!(loader.loaded_items(), vec![] as Vec<i32>);
    }
}
