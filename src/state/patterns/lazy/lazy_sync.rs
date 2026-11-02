//! Thread-safe lazy data

use crate::patterns::lazy::types::{LoadState, OnPanic};
use crate::utils::lock::{lock_or_recover, read_or_recover, write_or_recover};
use std::sync::{Arc, Mutex, RwLock};

/// Thread-safe lazy data
pub struct LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    /// The loaded value
    value: Arc<RwLock<Option<T>>>,
    /// The loader function
    loader: Arc<Mutex<Option<F>>>,
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
            loader: Arc::new(Mutex::new(Some(loader))),
            state: Arc::new(RwLock::new(LoadState::Idle)),
        }
    }

    /// Get the value, loading if necessary
    /// Returns a clone of the value
    ///
    /// While another thread runs the loader this waits for it, as
    /// [`OnceLock::get_or_init`](std::sync::OnceLock::get_or_init) does, so a
    /// loader must not call back into its own `LazySync`. A panic in the loader
    /// carries on to the thread that ran it and leaves the state at
    /// [`LoadState::Failed`]; every other call then returns `None`.
    ///
    /// To look without waiting, use [`state`](Self::state) or
    /// [`try_read`](Self::try_read).
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.ensure_loaded();
        read_or_recover(&self.value).clone()
    }

    /// Get a read guard to the value without cloning (zero-copy access)
    ///
    /// Loads the value first if it is not loaded yet, waiting as
    /// [`get`](Self::get) does. The guard holds `None` if the loader panicked.
    ///
    /// # Example
    /// ```
    /// use revue::patterns::LazySync;
    ///
    /// let data = LazySync::new(|| vec![1, 2, 3]);
    /// let guard = data.read();
    /// assert_eq!(guard.as_ref().map(|v| v.len()), Some(3));
    /// ```
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Option<T>> {
        self.ensure_loaded();
        read_or_recover(&self.value)
    }

    /// Try to get a read guard without blocking
    ///
    /// Returns `None` if the lock is currently held by a writer. Does not load,
    /// and the guard holds `None` until the value is there.
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

    /// Ensure the data is loaded, or that loading it failed
    ///
    /// The loader's lock is held for the whole load, which is what makes a
    /// second thread wait for the first.
    fn ensure_loaded(&self) {
        if self.is_settled() {
            return;
        }

        let mut loader = lock_or_recover(&self.loader);
        if self.is_settled() {
            return;
        }

        if let Some(loader) = loader.take() {
            *write_or_recover(&self.state) = LoadState::Loading;
            let failed = OnPanic(|| *write_or_recover(&self.state) = LoadState::Failed);
            let value = loader();
            std::mem::forget(failed);
            *write_or_recover(&self.value) = Some(value);
            *write_or_recover(&self.state) = LoadState::Loaded;
        }
    }

    fn is_settled(&self) -> bool {
        matches!(
            *read_or_recover(&self.state),
            LoadState::Loaded | LoadState::Failed
        )
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

    /// A `LazySync` whose loader waits for `release`, and a way to wait until
    /// the first thread is inside it.
    fn started_load(
        outcome: impl FnOnce() -> i32 + Send + 'static,
    ) -> (
        LazySync<i32, impl FnOnce() -> i32 + Send>,
        std::sync::mpsc::Sender<()>,
        std::thread::JoinHandle<()>,
    ) {
        let (release, wait) = std::sync::mpsc::channel::<()>();
        let lazy = LazySync::new(move || {
            wait.recv().unwrap();
            outcome()
        });
        let first = lazy.clone();
        let loading = std::thread::spawn(move || {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| first.get()));
        });
        while lazy.state() != LoadState::Loading {
            std::thread::yield_now();
        }
        (lazy, release, loading)
    }

    /// Long enough for a thread that does not wait to have finished.
    fn still_waiting<R>(thread: &std::thread::JoinHandle<R>) -> bool {
        std::thread::sleep(std::time::Duration::from_millis(50));
        !thread.is_finished()
    }

    #[test]
    fn a_second_thread_waits_for_the_value() {
        let (lazy, release, loading) = started_load(|| 42);

        let second = lazy.clone();
        let waiter = std::thread::spawn(move || second.get());
        assert!(still_waiting(&waiter));

        release.send(()).unwrap();
        assert_eq!(waiter.join().unwrap(), Some(42));
        loading.join().unwrap();
    }

    #[test]
    fn read_waits_for_the_value_too() {
        let (lazy, release, loading) = started_load(|| 42);

        let second = lazy.clone();
        let waiter = std::thread::spawn(move || *second.read());
        assert!(still_waiting(&waiter));

        release.send(()).unwrap();
        assert_eq!(waiter.join().unwrap(), Some(42));
        loading.join().unwrap();
    }

    #[test]
    fn looking_without_waiting_does_not_block_during_a_load() {
        let (lazy, release, loading) = started_load(|| 42);

        assert_eq!(lazy.state(), LoadState::Loading);
        assert!(!lazy.is_loaded());
        assert!(lazy.try_read().unwrap().is_none());

        release.send(()).unwrap();
        loading.join().unwrap();
        assert!(lazy.is_loaded());
    }

    #[test]
    fn a_panicking_loader_fails_and_wakes_the_thread_waiting_for_it() {
        let (lazy, release, loading) = started_load(|| std::panic::resume_unwind(Box::new("no")));

        let second = lazy.clone();
        let waiter = std::thread::spawn(move || second.get());
        assert!(still_waiting(&waiter));

        release.send(()).unwrap();
        assert_eq!(waiter.join().unwrap(), None);
        loading.join().unwrap();
        assert_eq!(lazy.state(), LoadState::Failed);
        assert_eq!(lazy.get(), None);
    }

    #[test]
    fn the_loader_runs_once_however_many_threads_ask() {
        let runs = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counted = Arc::clone(&runs);
        let lazy = LazySync::new(move || {
            counted.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            7
        });

        let threads: Vec<_> = (0..8)
            .map(|_| {
                let lazy = lazy.clone();
                std::thread::spawn(move || lazy.get())
            })
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), Some(7));
        }
        assert_eq!(runs.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
