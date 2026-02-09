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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_list_new() {
        let list = LazyList::<i32>::new(10);
        assert_eq!(list.capacity(), 10);
        assert_eq!(list.loaded_count(), 0);
    }

    #[test]
    fn test_lazy_list_capacity() {
        let list = LazyList::<String>::new(5);
        assert_eq!(list.capacity(), 5);
    }

    #[test]
    fn test_lazy_list_get_none() {
        let list = LazyList::<i32>::new(10);
        assert!(list.get(0).is_none());
        assert!(list.get(5).is_none());
    }

    #[test]
    fn test_lazy_list_set_and_get() {
        let list = LazyList::new(10);
        list.set(0, 42);
        assert_eq!(list.get(0), Some(42));
        assert!(list.get(1).is_none());
    }

    #[test]
    fn test_lazy_list_is_loaded() {
        let list = LazyList::new(10);
        assert!(!list.is_loaded(0));
        list.set(0, 42);
        assert!(list.is_loaded(0));
        assert!(!list.is_loaded(1));
    }

    #[test]
    fn test_lazy_list_loaded_count() {
        let list = LazyList::new(10);
        assert_eq!(list.loaded_count(), 0);
        list.set(0, 1);
        list.set(2, 3);
        list.set(5, 6);
        assert_eq!(list.loaded_count(), 3);
    }

    #[test]
    fn test_lazy_list_clear() {
        let list = LazyList::new(10);
        list.set(0, 42);
        assert!(list.is_loaded(0));
        list.clear(0);
        assert!(!list.is_loaded(0));
    }

    #[test]
    fn test_lazy_list_clear_all() {
        let list = LazyList::new(10);
        list.set(0, 1);
        list.set(2, 3);
        list.set(5, 6);
        assert_eq!(list.loaded_count(), 3);
        list.clear_all();
        assert_eq!(list.loaded_count(), 0);
    }

    #[test]
    fn test_lazy_list_with_strings() {
        let list = LazyList::new(5);
        list.set(0, "hello".to_string());
        assert_eq!(list.get(0), Some("hello".to_string()));
    }

    #[test]
    fn test_lazy_list_out_of_bounds() {
        let list = LazyList::new(10);
        assert!(list.get(100).is_none());
        // set should do nothing for out of bounds
        list.set(100, 42);
        assert!(list.get(100).is_none());
    }

    #[test]
    fn test_lazy_list_clear_out_of_bounds() {
        let list: LazyList<i32> = LazyList::new(10);
        // clear should do nothing for out of bounds
        list.clear(100);
        assert_eq!(list.loaded_count(), 0);
    }
}
