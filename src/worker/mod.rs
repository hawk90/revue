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
            // Use multi_thread runtime to allow block_on() from within runtime context
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(4),
                )
                .build()
                .map(|runtime| {
                    // Gracefully handle races where another thread sets the runtime first.
                    if RUNTIME.set(runtime).is_err() {
                        // Another thread initialized the runtime; fall back to the existing one.
                    }
                    // SAFETY: RUNTIME is guaranteed to be initialized here because:
                    // 1. Either this thread just successfully set it (line 74 above)
                    // 2. Or another thread set it (causing the Err case at line 74)
                    // In both cases, RUNTIME.get() returns Some.
                    RUNTIME
                        .get()
                        .expect("Shared runtime must be initialized after line 74")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_error_display() {
        assert_eq!(
            format!("{}", WorkerError::Cancelled),
            "Worker task was cancelled"
        );
        assert_eq!(
            format!("{}", WorkerError::Panicked("test".to_string())),
            "Worker task panicked: test"
        );
        assert_eq!(
            format!("{}", WorkerError::ChannelClosed),
            "Worker channel closed"
        );
        assert_eq!(format!("{}", WorkerError::Timeout), "Worker task timed out");
        assert_eq!(
            format!("{}", WorkerError::Custom("error".to_string())),
            "Worker error: error"
        );
        assert_eq!(
            format!(
                "{}",
                WorkerError::RuntimeCreationFailed("failed".to_string())
            ),
            "Failed to create tokio runtime: failed"
        );
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Low < Priority::Normal);
        assert!(Priority::Normal < Priority::High);
        assert!(Priority::Low < Priority::High);
        assert_eq!(Priority::Normal, Priority::default());
    }

    #[test]
    fn test_worker_config_default() {
        let config = WorkerConfig::default();
        assert!(config.threads >= 1);
        assert_eq!(config.queue_capacity, 1000);
        assert!(config.default_timeout_ms.is_none());
    }

    #[test]
    fn test_worker_config_with_threads() {
        let config = WorkerConfig::with_threads(4);
        assert_eq!(config.threads, 4);
        assert_eq!(config.queue_capacity, 1000);

        // Minimum 1 thread
        let config = WorkerConfig::with_threads(0);
        assert_eq!(config.threads, 1);
    }

    #[test]
    fn test_run_blocking() {
        let handle = run_blocking(|| 42);
        // Handle is created successfully
        drop(handle);
    }

    #[cfg(feature = "async")]
    #[test]
    fn test_shared_runtime_handle() {
        // Multiple calls should return handles
        let result1 = shared_runtime::handle();
        assert!(result1.is_ok());

        let result2 = shared_runtime::handle();
        assert!(result2.is_ok());
    }
}
