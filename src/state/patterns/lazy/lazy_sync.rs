//! Thread-safe lazy data

use crate::patterns::lazy::types::LoadState;
use crate::utils::lock::{read_or_recover, write_or_recover};
use std::sync::{Arc, RwLock};

/// Thread-safe lazy data
pub struct LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    /// The loaded value
    value: Arc<RwLock<Option<T>>>,
    /// The loader function
    loader: Arc<RwLock<Option<F>>>,
    /// Current state
    state: Arc<RwLock<LoadState>>,
}

impl<T, F> LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    /// Create new thread-safe lazy data
    pub fn new(loader: F) -> Self {
        Self {
            value: Arc::new(RwLock::new(None)),
            loader: Arc::new(RwLock::new(Some(loader))),
            state: Arc::new(RwLock::new(LoadState::Idle)),
        }
    }

    /// Get the value, loading if necessary
    /// Returns a clone of the value
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.ensure_loaded();
        read_or_recover(&self.value).clone()
    }

    /// Get a read guard to the value without cloning (zero-copy access)
    ///
    /// Returns `None` if the value hasn't been loaded yet.
    /// Use `ensure_loaded()` or `get()` first if you need guaranteed access.
    ///
    /// # Example
    /// ```ignore
    /// let data = LazySync::new(|| vec![1, 2, 3]);
    /// data.get(); // trigger loading
    /// if let Some(guard) = data.read() {
    ///     println!("Length: {}", guard.as_ref().map(|v| v.len()).unwrap_or(0));
    /// }
    /// ```
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Option<T>> {
        self.ensure_loaded();
        read_or_recover(&self.value)
    }

    /// Try to get a read guard without blocking
    ///
    /// Returns `None` if the lock is currently held by a writer.
    pub fn try_read(&self) -> Option<std::sync::RwLockReadGuard<'_, Option<T>>> {
        self.value.try_read().ok()
    }

    /// Check if data is loaded
    pub fn is_loaded(&self) -> bool {
        *read_or_recover(&self.state) == LoadState::Loaded
    }

    /// Get current state
    pub fn state(&self) -> LoadState {
        *read_or_recover(&self.state)
    }

    /// Ensure the data is loaded
    fn ensure_loaded(&self) {
        {
            let state = read_or_recover(&self.state);
            if *state != LoadState::Idle {
                return;
            }
        }

        {
            let mut state = write_or_recover(&self.state);
            if *state != LoadState::Idle {
                return; // Double-check after acquiring write lock
            }
            *state = LoadState::Loading;
        }

        if let Some(loader) = write_or_recover(&self.loader).take() {
            let value = loader();
            *write_or_recover(&self.value) = Some(value);
            *write_or_recover(&self.state) = LoadState::Loaded;
        }
    }
}

impl<T, F> Clone for LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            loader: Arc::clone(&self.loader),
            state: Arc::clone(&self.state),
        }
    }
}
