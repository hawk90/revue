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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::lazy::types::LoadState;

    #[test]
    fn test_lazy_sync_new() {
        let lazy = LazySync::new(|| 42);
        assert!(!lazy.is_loaded());
        assert_eq!(lazy.state(), LoadState::Idle);
    }

    #[test]
    fn test_lazy_sync_get_loads() {
        let lazy = LazySync::new(|| 42);
        let value = lazy.get();
        assert!(value.is_some());
        assert_eq!(value.unwrap(), 42);
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_sync_is_loaded() {
        let lazy = LazySync::new(|| 42);
        assert!(!lazy.is_loaded());
        lazy.get();
        assert!(lazy.is_loaded());
    }

    #[test]
    fn test_lazy_sync_state() {
        let lazy = LazySync::new(|| 42);
        assert_eq!(lazy.state(), LoadState::Idle);
        lazy.get();
        assert_eq!(lazy.state(), LoadState::Loaded);
    }

    #[test]
    fn test_lazy_sync_read() {
        let lazy = LazySync::new(|| vec![1, 2, 3, 4, 5]);
        lazy.get(); // trigger loading
        let guard = lazy.read();
        assert!(guard.is_some());
        assert_eq!(guard.as_ref().unwrap().len(), 5);
    }

    #[test]
    fn test_lazy_sync_try_read() {
        let lazy = LazySync::new(|| 42);
        lazy.get(); // trigger loading
        let guard = lazy.try_read();
        assert!(guard.is_some());
        assert_eq!(guard.unwrap().as_ref().unwrap(), &42);
    }

    #[test]
    fn test_lazy_sync_try_read_before_load() {
        let lazy = LazySync::new(|| 42);
        let guard = lazy.try_read();
        assert!(guard.is_some());
        // Should be None since not loaded yet
        assert!(guard.unwrap().is_none());
    }

    #[test]
    fn test_lazy_sync_clone() {
        let lazy = LazySync::new(|| 42);
        lazy.get();
        let cloned = lazy.clone();
        assert!(cloned.is_loaded());
        assert_eq!(cloned.get().unwrap(), 42);
    }

    #[test]
    fn test_lazy_sync_get_caches() {
        let lazy = LazySync::new(|| 42);
        let value1 = lazy.get();
        let value2 = lazy.get();
        assert_eq!(value1.unwrap(), value2.unwrap());
    }

    #[test]
    fn test_lazy_sync_with_string() {
        let lazy = LazySync::new(|| "hello".to_string());
        let value = lazy.get();
        assert!(value.is_some());
        assert_eq!(value.unwrap(), "hello");
    }
}
