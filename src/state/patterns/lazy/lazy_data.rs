//! Lazy-loaded value (single-threaded)

use std::cell::RefCell;

use crate::patterns::lazy::types::LoadState;

/// A lazily loaded value (single-threaded)
pub struct LazyData<T, F>
where
    F: FnOnce() -> T,
{
    /// The loaded value
    value: RefCell<Option<T>>,
    /// The loader function
    loader: RefCell<Option<F>>,
    /// Current state
    state: RefCell<LoadState>,
}

impl<T, F> LazyData<T, F>
where
    F: FnOnce() -> T,
{
    /// Create new lazy data with a loader function
    pub fn new(loader: F) -> Self {
        Self {
            value: RefCell::new(None),
            loader: RefCell::new(Some(loader)),
            state: RefCell::new(LoadState::Idle),
        }
    }

    /// Get the value, loading if necessary
    pub fn get(&self) -> Option<std::cell::Ref<'_, T>> {
        self.ensure_loaded();
        let borrow = self.value.borrow();
        if borrow.is_some() {
            Some(std::cell::Ref::map(borrow, |opt| opt.as_ref().unwrap()))
        } else {
            None
        }
    }

    /// Check if data is loaded
    pub fn is_loaded(&self) -> bool {
        *self.state.borrow() == LoadState::Loaded
    }

    /// Get current state
    pub fn state(&self) -> LoadState {
        *self.state.borrow()
    }

    /// Force reload
    pub fn reload(&self)
    where
        F: Clone,
    {
        // Can't reload with FnOnce without Clone
        // This is a limitation - consider using Fn instead
    }

    /// Ensure the data is loaded
    fn ensure_loaded(&self) {
        if *self.state.borrow() != LoadState::Idle {
            return;
        }

        *self.state.borrow_mut() = LoadState::Loading;

        if let Some(loader) = self.loader.borrow_mut().take() {
            let value = loader();
            *self.value.borrow_mut() = Some(value);
            *self.state.borrow_mut() = LoadState::Loaded;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::lazy::types::LoadState;

    #[test]
    fn test_lazy_data_new() {
        let lazy = LazyData::new(|| 42);
        assert!(!lazy.is_loaded());
        assert_eq!(lazy.state(), LoadState::Idle);
    }

    #[test]
    fn test_lazy_data_get_loads() {
        let lazy = LazyData::new(|| 42);
        let value = lazy.get();
        assert!(value.is_some());
        assert_eq!(*value.unwrap(), 42);
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_data_get_returns_value() {
        let lazy = LazyData::new(|| "hello".to_string());
        let value = lazy.get();
        assert!(value.is_some());
        assert_eq!(*value.unwrap(), "hello");
    }

    #[test]
    fn test_lazy_data_is_loaded() {
        let lazy = LazyData::new(|| 42);
        assert!(!lazy.is_loaded());
        lazy.get();
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_data_state() {
        let lazy = LazyData::new(|| 42);
        assert_eq!(lazy.state(), LoadState::Idle);
        lazy.get();
        assert_eq!(lazy.state(), LoadState::Loaded);
    }

    #[test]
    fn test_lazy_data_get_caches() {
        let lazy = LazyData::new(|| 42);
        let value1 = lazy.get();
        let value2 = lazy.get();
        assert_eq!(*value1.unwrap(), *value2.unwrap());
    }

    #[test]
    fn test_lazy_data_with_vec() {
        let lazy = LazyData::new(|| vec![1, 2, 3, 4, 5]);
        let value = lazy.get();
        assert!(value.is_some());
        assert_eq!(*value.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_lazy_data_with_closure() {
        let x = 10;
        let lazy = LazyData::new(|| x * 2);
        let value = lazy.get();
        assert!(value.is_some());
        assert_eq!(*value.unwrap(), 20);
    }

    #[test]
    fn test_lazy_data_reload_compiles() {
        let lazy = LazyData::new(|| 42);
        lazy.get();
        assert!(lazy.is_loaded());
        // reload() does nothing for FnOnce, but should compile
        lazy.reload();
    }
}
