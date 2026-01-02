//! Worker handle for task management

use super::{WorkerError, WorkerResult};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// State of a worker task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    /// Task is pending/queued
    Pending,
    /// Task is running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was cancelled
    Cancelled,
}

/// Handle to a background task
pub struct WorkerHandle<T> {
    /// Shared state
    state: Arc<Mutex<WorkerState>>,
    /// Result storage
    result: Arc<Mutex<Option<WorkerResult<T>>>>,
    /// Thread handle (for blocking tasks)
    thread: Option<JoinHandle<()>>,
    /// Cancel flag
    cancelled: Arc<Mutex<bool>>,
}

impl<T: Send + 'static> WorkerHandle<T> {
    /// Spawn a blocking task on a new thread
    pub fn spawn_blocking<F>(f: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let state = Arc::new(Mutex::new(WorkerState::Pending));
        let result: Arc<Mutex<Option<WorkerResult<T>>>> = Arc::new(Mutex::new(None));
        let cancelled = Arc::new(Mutex::new(false));

        let state_clone = state.clone();
        let result_clone = result.clone();
        let cancelled_clone = cancelled.clone();

        let thread = thread::spawn(move || {
            // Update state to running
            if let Ok(mut s) = state_clone.lock() {
                *s = WorkerState::Running;
            }

            // Check if cancelled before starting
            if cancelled_clone.lock().map(|c| *c).unwrap_or(false) {
                if let Ok(mut s) = state_clone.lock() {
                    *s = WorkerState::Cancelled;
                }
                if let Ok(mut r) = result_clone.lock() {
                    *r = Some(Err(WorkerError::Cancelled));
                }
                return;
            }

            // Execute task with panic handling
            let task_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

            // Store result
            match task_result {
                Ok(value) => {
                    if let Ok(mut r) = result_clone.lock() {
                        *r = Some(Ok(value));
                    }
                    if let Ok(mut s) = state_clone.lock() {
                        *s = WorkerState::Completed;
                    }
                }
                Err(panic_info) => {
                    let msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = panic_info.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "Unknown panic".to_string()
                    };

                    if let Ok(mut r) = result_clone.lock() {
                        *r = Some(Err(WorkerError::Panicked(msg)));
                    }
                    if let Ok(mut s) = state_clone.lock() {
                        *s = WorkerState::Failed;
                    }
                }
            }
        });

        Self {
            state,
            result,
            thread: Some(thread),
            cancelled,
        }
    }

    /// Spawn an async task
    ///
    /// Uses tokio runtime if available, otherwise uses a simple polling executor.
    #[cfg(feature = "async")]
    pub fn spawn<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        let state = Arc::new(Mutex::new(WorkerState::Pending));
        let result: Arc<Mutex<Option<WorkerResult<T>>>> = Arc::new(Mutex::new(None));
        let cancelled = Arc::new(Mutex::new(false));

        let state_clone = state.clone();
        let result_clone = result.clone();
        let _cancelled_clone = cancelled.clone();

        let thread = thread::spawn(move || {
            // Update state to running
            if let Ok(mut s) = state_clone.lock() {
                *s = WorkerState::Running;
            }

            // Use tokio runtime for async task
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");
            let value = rt.block_on(future);

            // Store result
            if let Ok(mut r) = result_clone.lock() {
                *r = Some(Ok(value));
            }
            if let Ok(mut s) = state_clone.lock() {
                *s = WorkerState::Completed;
            }
        });

        Self {
            state,
            result,
            thread: Some(thread),
            cancelled,
        }
    }

    /// Spawn an async task using a simple polling executor (no tokio required)
    #[cfg(not(feature = "async"))]
    pub fn spawn<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        use std::pin::Pin;
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

        // Simple waker that does nothing (busy-polling)
        fn dummy_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(_: *const ()) -> RawWaker {
                dummy_raw_waker()
            }
            let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), vtable)
        }

        let state = Arc::new(Mutex::new(WorkerState::Pending));
        let result: Arc<Mutex<Option<WorkerResult<T>>>> = Arc::new(Mutex::new(None));
        let cancelled = Arc::new(Mutex::new(false));

        let state_clone = state.clone();
        let result_clone = result.clone();
        let cancelled_clone = cancelled.clone();

        let thread = thread::spawn(move || {
            // Update state to running
            if let Ok(mut s) = state_clone.lock() {
                *s = WorkerState::Running;
            }

            // Simple polling executor
            let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
            let mut cx = Context::from_waker(&waker);
            let mut future = Box::pin(future);

            loop {
                // Check cancellation
                if cancelled_clone.lock().map(|c| *c).unwrap_or(false) {
                    if let Ok(mut s) = state_clone.lock() {
                        *s = WorkerState::Cancelled;
                    }
                    if let Ok(mut r) = result_clone.lock() {
                        *r = Some(Err(WorkerError::Cancelled));
                    }
                    return;
                }

                match Pin::as_mut(&mut future).poll(&mut cx) {
                    Poll::Ready(value) => {
                        if let Ok(mut r) = result_clone.lock() {
                            *r = Some(Ok(value));
                        }
                        if let Ok(mut s) = state_clone.lock() {
                            *s = WorkerState::Completed;
                        }
                        return;
                    }
                    Poll::Pending => {
                        // Yield to other threads
                        thread::yield_now();
                    }
                }
            }
        });

        Self {
            state,
            result,
            thread: Some(thread),
            cancelled,
        }
    }

    /// Get current state
    pub fn state(&self) -> WorkerState {
        self.state.lock().map(|s| *s).unwrap_or(WorkerState::Failed)
    }

    /// Check if task is finished (completed, failed, or cancelled)
    pub fn is_finished(&self) -> bool {
        matches!(
            self.state(),
            WorkerState::Completed | WorkerState::Failed | WorkerState::Cancelled
        )
    }

    /// Check if task completed successfully
    pub fn is_success(&self) -> bool {
        self.state() == WorkerState::Completed
    }

    /// Check if task is still running
    pub fn is_running(&self) -> bool {
        self.state() == WorkerState::Running
    }

    /// Cancel the task
    pub fn cancel(&self) {
        if let Ok(mut c) = self.cancelled.lock() {
            *c = true;
        }
        if let Ok(mut s) = self.state.lock() {
            if *s == WorkerState::Pending {
                *s = WorkerState::Cancelled;
            }
        }
    }

    /// Wait for the task to complete and get result
    pub fn join(mut self) -> WorkerResult<T> {
        // Wait for thread to finish
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }

        // Get result
        self.result
            .lock()
            .ok()
            .and_then(|mut r| r.take())
            .unwrap_or(Err(WorkerError::ChannelClosed))
    }

    /// Try to get result without blocking
    pub fn try_join(&mut self) -> Option<WorkerResult<T>> {
        if !self.is_finished() {
            return None;
        }

        self.result.lock().ok().and_then(|mut r| r.take())
    }

    /// Wait for completion with timeout
    pub fn join_timeout(self, timeout: Duration) -> WorkerResult<T> {
        let start = std::time::Instant::now();

        while !self.is_finished() {
            if start.elapsed() >= timeout {
                self.cancel();
                return Err(WorkerError::Timeout);
            }
            thread::sleep(Duration::from_millis(1));
        }

        self.join()
    }

    /// Poll for result (non-blocking)
    pub fn poll(&self) -> Option<WorkerState> {
        Some(self.state())
    }
}

impl<T> Drop for WorkerHandle<T> {
    fn drop(&mut self) {
        // Cancel task if still running - just set the flag
        if let Ok(mut c) = self.cancelled.lock() {
            *c = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_blocking() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_worker_state() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(50));
            "done"
        });

        // Should be running or pending
        assert!(!handle.is_finished());

        let result = handle.join();
        assert_eq!(result.unwrap(), "done");
    }

    #[test]
    fn test_cancel() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_secs(10));
            "never"
        });

        handle.cancel();
        // Note: cancel only affects pending tasks, running tasks continue
    }

    #[test]
    fn test_join_timeout() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_secs(10));
            42
        });

        let result = handle.join_timeout(Duration::from_millis(50));
        assert!(matches!(result, Err(WorkerError::Timeout)));
    }

    #[test]
    fn test_panic_handling() {
        let handle = WorkerHandle::spawn_blocking(|| {
            panic!("intentional panic");
            #[allow(unreachable_code)]
            42
        });

        let result = handle.join();
        assert!(matches!(result, Err(WorkerError::Panicked(_))));
    }
}
