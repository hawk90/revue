//! Async state management for reactive systems
//!
//! Provides hooks for integrating async operations with the reactive system.
//! Uses background threads for non-blocking execution while maintaining
//! reactive updates. Since Signals are now thread-safe (`Arc<RwLock>`),
//! async operations can directly update signals from background threads.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Create async state that fetches data
//! let (user_data, fetch) = use_async(|| {
//!     // This runs in background thread
//!     fetch_user(1)
//! });
//!
//! // Trigger the fetch
//! fetch();
//!
//! // Check the state
//! match user_data.get() {
//!     AsyncState::Idle => println!("Not started"),
//!     AsyncState::Loading => println!("Loading..."),
//!     AsyncState::Ready(user) => println!("User: {:?}", user),
//!     AsyncState::Error(e) => println!("Error: {}", e),
//! }
//! ```

use std::fmt;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::{Arc, RwLock};
use std::thread;

use super::{signal, Signal};

/// State of an async operation
///
/// Represents the lifecycle of an async task from idle to completion.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AsyncState<T> {
    /// Task has not started yet
    #[default]
    Idle,
    /// Task is currently running
    Loading,
    /// Task completed successfully with a result
    Ready(T),
    /// Task failed with an error message
    Error(String),
}

impl<T> AsyncState<T> {
    /// Returns true if the state is Idle
    pub fn is_idle(&self) -> bool {
        matches!(self, AsyncState::Idle)
    }

    /// Returns true if the state is Loading
    pub fn is_loading(&self) -> bool {
        matches!(self, AsyncState::Loading)
    }

    /// Returns true if the state is Ready
    pub fn is_ready(&self) -> bool {
        matches!(self, AsyncState::Ready(_))
    }

    /// Returns true if the state is Error
    pub fn is_error(&self) -> bool {
        matches!(self, AsyncState::Error(_))
    }

    /// Get the value if Ready, otherwise None
    pub fn value(&self) -> Option<&T> {
        match self {
            AsyncState::Ready(v) => Some(v),
            _ => None,
        }
    }

    /// Get the error message if Error, otherwise None
    pub fn error(&self) -> Option<&str> {
        match self {
            AsyncState::Error(e) => Some(e),
            _ => None,
        }
    }

    /// Map the value if Ready
    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> AsyncState<U>
    where
        U: Clone,
    {
        match self {
            AsyncState::Idle => AsyncState::Idle,
            AsyncState::Loading => AsyncState::Loading,
            AsyncState::Ready(v) => AsyncState::Ready(f(v)),
            AsyncState::Error(e) => AsyncState::Error(e.clone()),
        }
    }

    /// Unwrap the value or return a default
    pub fn unwrap_or(&self, default: T) -> T
    where
        T: Clone,
    {
        match self {
            AsyncState::Ready(v) => v.clone(),
            _ => default,
        }
    }

    /// Unwrap the value or compute a default
    pub fn unwrap_or_else<F: FnOnce() -> T>(&self, f: F) -> T
    where
        T: Clone,
    {
        match self {
            AsyncState::Ready(v) => v.clone(),
            _ => f(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for AsyncState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncState::Idle => write!(f, "Idle"),
            AsyncState::Loading => write!(f, "Loading"),
            AsyncState::Ready(v) => write!(f, "Ready({})", v),
            AsyncState::Error(e) => write!(f, "Error({})", e),
        }
    }
}

/// Result type for async tasks
pub type AsyncResult<T> = Result<T, String>;

/// Create an async state with manual trigger control
///
/// Returns a tuple of (state signal, trigger function).
/// Call the trigger function to start the async operation.
/// Since Signal is now thread-safe, the background thread can
/// directly update the signal.
///
/// # Example
///
/// ```rust,ignore
/// let (state, trigger) = use_async(|| {
///     // Runs in background thread
///     fetch_data()
/// });
///
/// // Start the async operation
/// trigger();
///
/// // Check state
/// if state.get().is_ready() {
///     // Handle result
/// }
/// ```
pub fn use_async<T, F>(f: F) -> (Signal<AsyncState<T>>, impl Fn() + Clone)
where
    T: Clone + Send + Sync + 'static,
    F: Fn() -> AsyncResult<T> + Send + Sync + Clone + 'static,
{
    let state: Signal<AsyncState<T>> = signal(AsyncState::Idle);
    let state_clone = state.clone();

    let trigger = move || {
        state_clone.set(AsyncState::Loading);

        let f_clone = f.clone();
        let state_for_thread = state_clone.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f_clone()));
            let result = match result {
                Ok(r) => r,
                Err(_) => Err("Task panicked".to_string()),
            };

            match result {
                Ok(value) => state_for_thread.set(AsyncState::Ready(value)),
                Err(e) => state_for_thread.set(AsyncState::Error(e)),
            }
        });
    };

    (state, trigger)
}

/// Create an async state that polls in the tick loop
///
/// This version is designed to work with the app's tick loop for polling.
/// Returns a tuple of (state, start_fn, poll_fn) where:
/// - state: The reactive state signal
/// - start_fn: Call to begin the async operation
/// - poll_fn: Call each tick to check for completion (returns true when done)
///
/// # Example
///
/// ```rust,ignore
/// let (state, start, poll) = use_async_poll(|| fetch_data());
///
/// // In your app:
/// fn on_button_click(&mut self) {
///     start();
/// }
///
/// fn tick(&mut self) {
///     poll(); // Call each tick to check for completion
/// }
/// ```
pub fn use_async_poll<T, F>(
    f: F,
) -> (
    Signal<AsyncState<T>>,
    impl Fn() + Clone,
    impl Fn() -> bool + Clone,
)
where
    T: Clone + Send + Sync + 'static,
    F: Fn() -> AsyncResult<T> + Send + Sync + Clone + 'static,
{
    let state: Signal<AsyncState<T>> = signal(AsyncState::Idle);
    let receiver: Arc<RwLock<Option<Receiver<AsyncResult<T>>>>> = Arc::new(RwLock::new(None));

    let receiver_start = receiver.clone();
    let state_start = state.clone();
    let start = move || {
        state_start.set(AsyncState::Loading);

        let (tx, rx) = mpsc::channel();
        let f_clone = f.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f_clone()));
            let result = match result {
                Ok(r) => r,
                Err(_) => Err("Task panicked".to_string()),
            };
            let _ = tx.send(result);
        });

        *receiver_start.write().unwrap() = Some(rx);
    };

    let receiver_poll = receiver.clone();
    let state_poll = state.clone();
    let poll = move || -> bool {
        let mut rx_ref = receiver_poll.write().unwrap();
        if let Some(rx) = rx_ref.as_ref() {
            match rx.try_recv() {
                Ok(result) => {
                    match result {
                        Ok(value) => state_poll.set(AsyncState::Ready(value)),
                        Err(e) => state_poll.set(AsyncState::Error(e)),
                    }
                    *rx_ref = None;
                    true // Task completed
                }
                Err(TryRecvError::Empty) => false, // Still running
                Err(TryRecvError::Disconnected) => {
                    state_poll.set(AsyncState::Error("Task disconnected".to_string()));
                    *rx_ref = None;
                    true // Task ended (with error)
                }
            }
        } else {
            false // No task running
        }
    };

    (state, start, poll)
}

/// Create an async state that immediately starts execution
///
/// Unlike `use_async`, this starts the async operation immediately.
///
/// # Example
///
/// ```rust,ignore
/// let state = use_async_immediate(|| fetch_data());
/// // Operation has already started
/// ```
pub fn use_async_immediate<T, F>(f: F) -> Signal<AsyncState<T>>
where
    T: Clone + Send + Sync + 'static,
    F: FnOnce() -> AsyncResult<T> + Send + 'static,
{
    let state: Signal<AsyncState<T>> = signal(AsyncState::Loading);
    let state_for_thread = state.clone();

    thread::spawn(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        let result = match result {
            Ok(r) => r,
            Err(_) => Err("Task panicked".to_string()),
        };

        match result {
            Ok(value) => state_for_thread.set(AsyncState::Ready(value)),
            Err(e) => state_for_thread.set(AsyncState::Error(e)),
        }
    });

    state
}

/// Builder for creating async resources with more control
///
/// # Example
///
/// ```rust,ignore
/// let (user, trigger) = AsyncResource::new(|| fetch_user())
///     .build();
/// ```
pub struct AsyncResource<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: Fn() -> AsyncResult<T> + Send + Sync + Clone + 'static,
{
    fetch: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> AsyncResource<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: Fn() -> AsyncResult<T> + Send + Sync + Clone + 'static,
{
    /// Create a new async resource builder
    pub fn new(fetch: F) -> Self {
        Self {
            fetch,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Build and return the async state signal with manual trigger
    pub fn build(self) -> (Signal<AsyncState<T>>, impl Fn() + Clone) {
        use_async(self.fetch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_async_state_variants() {
        let idle: AsyncState<i32> = AsyncState::Idle;
        assert!(idle.is_idle());
        assert!(!idle.is_loading());
        assert!(!idle.is_ready());
        assert!(!idle.is_error());

        let loading: AsyncState<i32> = AsyncState::Loading;
        assert!(loading.is_loading());

        let ready: AsyncState<i32> = AsyncState::Ready(42);
        assert!(ready.is_ready());
        assert_eq!(ready.value(), Some(&42));

        let error: AsyncState<i32> = AsyncState::Error("failed".to_string());
        assert!(error.is_error());
        assert_eq!(error.error(), Some("failed"));
    }

    #[test]
    fn test_async_state_map() {
        let ready: AsyncState<i32> = AsyncState::Ready(10);
        let mapped = ready.map(|v| v * 2);
        assert_eq!(mapped, AsyncState::Ready(20));

        let loading: AsyncState<i32> = AsyncState::Loading;
        let mapped_loading = loading.map(|v| v * 2);
        assert!(mapped_loading.is_loading());
    }

    #[test]
    fn test_async_state_unwrap_or() {
        let ready: AsyncState<i32> = AsyncState::Ready(42);
        assert_eq!(ready.unwrap_or(0), 42);

        let loading: AsyncState<i32> = AsyncState::Loading;
        assert_eq!(loading.unwrap_or(0), 0);
    }

    #[test]
    fn test_use_async() {
        let (state, trigger) = use_async(|| {
            thread::sleep(Duration::from_millis(10));
            Ok(42)
        });

        assert!(state.get().is_idle());

        trigger();

        // Wait for completion
        thread::sleep(Duration::from_millis(50));

        assert_eq!(state.get(), AsyncState::Ready(42));
    }

    #[test]
    fn test_use_async_error() {
        let (state, trigger) = use_async::<i32, _>(|| Err("Something went wrong".to_string()));

        trigger();

        thread::sleep(Duration::from_millis(50));

        assert!(state.get().is_error());
        assert_eq!(state.get().error(), Some("Something went wrong"));
    }

    #[test]
    fn test_use_async_poll() {
        let (state, start, poll) = use_async_poll(|| {
            thread::sleep(Duration::from_millis(10));
            Ok("done".to_string())
        });

        assert!(state.get().is_idle());

        start();
        assert!(state.get().is_loading());

        // Poll until complete
        for _ in 0..20 {
            if poll() {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }

        assert_eq!(state.get(), AsyncState::Ready("done".to_string()));
    }

    #[test]
    fn test_use_async_immediate() {
        let state = use_async_immediate(|| {
            thread::sleep(Duration::from_millis(10));
            Ok(100)
        });

        // Should start as Loading (already running)
        assert!(state.get().is_loading());

        // Wait for completion
        thread::sleep(Duration::from_millis(50));

        assert_eq!(state.get(), AsyncState::Ready(100));
    }

    #[test]
    fn test_async_state_display() {
        let idle: AsyncState<i32> = AsyncState::Idle;
        assert_eq!(format!("{}", idle), "Idle");

        let loading: AsyncState<i32> = AsyncState::Loading;
        assert_eq!(format!("{}", loading), "Loading");

        let ready: AsyncState<i32> = AsyncState::Ready(42);
        assert_eq!(format!("{}", ready), "Ready(42)");

        let error: AsyncState<i32> = AsyncState::Error("fail".to_string());
        assert_eq!(format!("{}", error), "Error(fail)");
    }

    #[test]
    fn test_use_async_multiple_triggers() {
        let (state, trigger) = use_async(|| {
            thread::sleep(Duration::from_millis(20));
            Ok(42)
        });

        // First trigger
        trigger();
        assert!(state.get().is_loading());

        // Second trigger while first is running (replaces)
        trigger();
        assert!(state.get().is_loading());

        // Wait for completion
        thread::sleep(Duration::from_millis(50));

        assert_eq!(state.get(), AsyncState::Ready(42));
    }
}
