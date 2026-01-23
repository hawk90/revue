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
        std::cell::Ref::map(self.value.borrow(), |opt| opt.as_ref().unwrap())
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
