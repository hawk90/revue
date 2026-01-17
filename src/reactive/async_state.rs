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

use crate::utils::lock::{read_or_recover, write_or_recover};
use std::fmt;
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
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f_clone));
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

/// Internal state for poll-based async operations
#[derive(Clone)]
enum PollState<T> {
    /// No task running
    Idle,
    /// Task is running (no result yet)
    Running,
    /// Task completed with result
    Done(AsyncResult<T>),
}

/// Create an async state that polls in the tick loop
///
/// This version is designed to work with the app's tick loop for polling.
/// Returns a tuple of (state, start_fn, poll_fn) where:
/// - state: The reactive state signal
/// - start_fn: Call to begin the async operation
/// - poll_fn: Call each tick to check for completion (returns true when done)
///
/// # Thread Safety
///
/// Both `start` and `poll` can be called from any thread. The result is
/// communicated through thread-safe shared state (`Arc<RwLock>`).
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
    // Use thread-safe shared state instead of channel (Receiver is !Sync)
    let poll_state: Arc<RwLock<PollState<T>>> = Arc::new(RwLock::new(PollState::Idle));

    let poll_state_start = poll_state.clone();
    let state_start = state.clone();
    let start = move || {
        state_start.set(AsyncState::Loading);
        *write_or_recover(&poll_state_start) = PollState::Running;

        let f_clone = f.clone();
        let poll_state_thread = poll_state_start.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f_clone));
            let result = match result {
                Ok(r) => r,
                Err(_) => Err("Task panicked".to_string()),
            };
            *write_or_recover(&poll_state_thread) = PollState::Done(result);
        });
    };

    let poll_state_poll = poll_state.clone();
    let state_poll = state.clone();
    let poll = move || -> bool {
        // First, read to check if done (avoids unnecessary write lock)
        let is_done = matches!(*read_or_recover(&poll_state_poll), PollState::Done(_));

        if is_done {
            // Take the result with a write lock
            let mut guard = write_or_recover(&poll_state_poll);
            if let PollState::Done(result) = std::mem::replace(&mut *guard, PollState::Idle) {
                match result {
                    Ok(value) => state_poll.set(AsyncState::Ready(value)),
                    Err(e) => state_poll.set(AsyncState::Error(e)),
                }
                return true; // Task completed
            }
        }
        false // Still running or idle
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
