//! Worker system for background tasks
//!
//! Run CPU-intensive or I/O operations without blocking the UI.
//!
//! # Example
//!
//! ```ignore
//! use revue::worker::{Worker, WorkerPool};
//!
//! let pool = WorkerPool::new(4);
//!
//! // Spawn a background task
//! let handle = pool.spawn(async {
//!     // Long-running operation
//!     fetch_data().await
//! });
//!
//! // Check if done
//! if handle.is_finished() {
//!     let result = handle.join().unwrap();
//! }
//! ```

mod channel;
mod handle;
mod pool;

pub use channel::{WorkerChannel, WorkerCommand, WorkerMessage, WorkerReceiver, WorkerSender};
pub use handle::{WorkerHandle, WorkerState};
pub use pool::{Worker, WorkerPool};

use std::future::Future;
use std::pin::Pin;

/// Shared tokio runtime for async worker tasks
#[cfg(feature = "async")]
mod shared_runtime {
    use std::sync::OnceLock;
    use tokio::runtime::{Handle, Runtime};

    static RUNTIME: OnceLock<Runtime> = OnceLock::new();

    /// Get or create the shared runtime handle
    ///
    /// Returns an error string if runtime creation fails instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if:
    /// - The tokio runtime cannot be created (e.g., insufficient resources, system limits)
    /// - The runtime thread pool cannot be initialized
    pub fn handle() -> Result<Handle, String> {
        // First, try to get the current runtime if we're already in one
        if let Ok(handle) = Handle::try_current() {
            return Ok(handle);
        }

        // Try to get or create the shared runtime
        // If initialization failed, return an error instead of panicking
        if let Some(runtime) = RUNTIME.get() {
            Ok(runtime.handle().clone())
        } else {
            // Attempt to create the runtime
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map(|runtime| {
                    // Gracefully handle races where another thread sets the runtime first.
                    if RUNTIME.set(runtime).is_err() {
                        // Another thread initialized the runtime; fall back to the existing one.
                    }
                    RUNTIME
                        .get()
                        .expect("Shared runtime should be initialized")
                        .handle()
                        .clone()
                })
                .map_err(|e| format!("Failed to create tokio runtime: {}", e))
        }
    }
}

#[cfg(feature = "async")]
pub(crate) use shared_runtime::handle as get_runtime_handle;

/// A boxed future for worker tasks
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// Worker task result
pub type WorkerResult<T> = Result<T, WorkerError>;

/// Worker error types
#[derive(Debug, Clone)]
pub enum WorkerError {
    /// Task was cancelled
    Cancelled,
    /// Task panicked
    Panicked(String),
    /// Channel closed
    ChannelClosed,
    /// Timeout
    Timeout,
    /// Custom error
    Custom(String),
    /// Runtime creation failed
    RuntimeCreationFailed(String),
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerError::Cancelled => write!(f, "Worker task was cancelled"),
            WorkerError::Panicked(msg) => write!(f, "Worker task panicked: {}", msg),
            WorkerError::ChannelClosed => write!(f, "Worker channel closed"),
            WorkerError::Timeout => write!(f, "Worker task timed out"),
            WorkerError::Custom(msg) => write!(f, "Worker error: {}", msg),
            WorkerError::RuntimeCreationFailed(msg) => {
                write!(f, "Failed to create tokio runtime: {}", msg)
            }
        }
    }
}

impl std::error::Error for WorkerError {}

/// Worker task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Priority {
    /// Low priority (background)
    Low,
    /// Normal priority
    #[default]
    Normal,
    /// High priority
    High,
}

/// Worker configuration
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Number of worker threads
    pub threads: usize,
    /// Task queue capacity
    pub queue_capacity: usize,
    /// Default timeout in milliseconds
    pub default_timeout_ms: Option<u64>,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            queue_capacity: 1000,
            default_timeout_ms: None,
        }
    }
}

impl WorkerConfig {
    /// Create with specific thread count
    pub fn with_threads(threads: usize) -> Self {
        Self {
            threads: threads.max(1),
            ..Default::default()
        }
    }
}

/// Convenience function to run a sync task in background
pub fn run_blocking<F, T>(f: F) -> WorkerHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    WorkerHandle::spawn_blocking(f)
}

/// Convenience function to run an async task
pub fn spawn<F, T>(future: F) -> WorkerHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    WorkerHandle::spawn(future)
}
