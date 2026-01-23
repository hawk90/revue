//! Mock state for testing without reactive system

use std::cell::RefCell;
use std::rc::Rc;

/// Mock state for testing without reactive system
pub struct MockState<T> {
    value: Rc<RefCell<T>>,
    change_count: Rc<RefCell<usize>>,
}

impl<T> MockState<T> {
    /// Create a new mock state
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            change_count: Rc::new(RefCell::new(0)),
        }
    }

    /// Get the current value
    pub fn get(&self) -> std::cell::Ref<'_, T> {
        self.value.borrow()
    }

    /// Get mutable access to the value
    pub fn get_mut(&self) -> std::cell::RefMut<'_, T> {
        self.value.borrow_mut()
    }

    /// Set a new value
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        *self.change_count.borrow_mut() += 1;
    }

    /// Get the number of times the value has changed
    pub fn change_count(&self) -> usize {
        *self.change_count.borrow()
    }

    /// Reset change count
    pub fn reset_count(&self) {
        *self.change_count.borrow_mut() = 0;
    }
}

impl<T: Clone> MockState<T> {
    /// Get a cloned value
    pub fn value(&self) -> T {
        self.value.borrow().clone()
    }

    /// Update value with a function
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.value.borrow_mut());
        *self.change_count.borrow_mut() += 1;
    }
}

impl<T: Clone> Clone for MockState<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            change_count: Rc::clone(&self.change_count),
        }
    }
}
