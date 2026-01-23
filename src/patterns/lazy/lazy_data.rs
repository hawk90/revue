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
