//! Lazy-loaded value with reload support (single-threaded)

use std::cell::RefCell;

use crate::patterns::lazy::types::LoadState;

/// A lazily loaded value with reload support (single-threaded)
pub struct LazyReloadable<T, F>
where
    F: Fn() -> T,
{
    /// The loaded value
    value: RefCell<Option<T>>,
    /// The loader function
    loader: F,
    /// Current state
    state: RefCell<LoadState>,
}

impl<T, F> LazyReloadable<T, F>
where
    F: Fn() -> T,
{
    /// Create new lazy reloadable data
    pub fn new(loader: F) -> Self {
        Self {
            value: RefCell::new(None),
            loader,
            state: RefCell::new(LoadState::Idle),
        }
    }

    /// Get the value, loading if necessary
    pub fn get(&self) -> std::cell::Ref<'_, T> {
        self.ensure_loaded();
        // ensure_loaded() guarantees value is Some() after completion
        std::cell::Ref::map(self.value.borrow(), |opt| {
            opt.as_ref()
                .unwrap_or_else(|| panic!("value should be loaded after ensure_loaded()"))
        })
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
    pub fn reload(&self) {
        *self.state.borrow_mut() = LoadState::Loading;
        let value = (self.loader)();
        *self.value.borrow_mut() = Some(value);
        *self.state.borrow_mut() = LoadState::Loaded;
    }

    /// Invalidate cached value (will reload on next access)
    pub fn invalidate(&self) {
        *self.value.borrow_mut() = None;
        *self.state.borrow_mut() = LoadState::Idle;
    }

    /// Ensure the data is loaded
    fn ensure_loaded(&self) {
        if *self.state.borrow() == LoadState::Loaded {
            return;
        }

        *self.state.borrow_mut() = LoadState::Loading;
        let value = (self.loader)();
        *self.value.borrow_mut() = Some(value);
        *self.state.borrow_mut() = LoadState::Loaded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::lazy::types::LoadState;

    #[test]
    fn test_lazy_reloadable_new() {
        let lazy = LazyReloadable::new(|| 42);
        assert!(!lazy.is_loaded());
        assert_eq!(lazy.state(), LoadState::Idle);
    }

    #[test]
    fn test_lazy_reloadable_get_loads() {
        let lazy = LazyReloadable::new(|| 42);
        let value = lazy.get();
        assert_eq!(*value, 42);
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_reloadable_is_loaded() {
        let lazy = LazyReloadable::new(|| 42);
        assert!(!lazy.is_loaded());
        lazy.get();
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_reloadable_state() {
        let lazy = LazyReloadable::new(|| 42);
        assert_eq!(lazy.state(), LoadState::Idle);
        lazy.get();
        assert_eq!(lazy.state(), LoadState::Loaded);
    }

    #[test]
    fn test_lazy_reloadable_reload() {
        let lazy = LazyReloadable::new(|| 42);
        lazy.get();
        assert!(lazy.is_loaded());
        lazy.reload();
        assert!(lazy.is_loaded());
        assert_eq!(*lazy.get(), 42);
    }

    #[test]
    fn test_lazy_reloadable_invalidate() {
        let lazy = LazyReloadable::new(|| 42);
        lazy.get();
        assert!(lazy.is_loaded());
        lazy.invalidate();
        assert!(!lazy.is_loaded());
        // Getting after invalidate should reload
        assert_eq!(*lazy.get(), 42);
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_reloadable_get_returns_value() {
        let lazy = LazyReloadable::new(|| "hello".to_string());
        let value = lazy.get();
        assert_eq!(*value, "hello");
    }

    #[test]
    fn test_lazy_reloadable_with_vec() {
        let lazy = LazyReloadable::new(|| vec![1, 2, 3, 4, 5]);
        let value = lazy.get();
        assert_eq!(*value, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_lazy_reloadable_with_closure() {
        let x = 10;
        let lazy = LazyReloadable::new(|| x * 2);
        let value = lazy.get();
        assert_eq!(*value, 20);
    }

    #[test]
    fn test_lazy_reloadable_get_caches() {
        let lazy = LazyReloadable::new(|| 42);
        let value1 = lazy.get();
        let value2 = lazy.get();
        assert_eq!(*value1, *value2);
    }
}
