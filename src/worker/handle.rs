//! Worker handle for task management

use super::{WorkerError, WorkerResult};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// Use lock utilities for consistent poison handling
use crate::utils::lock as lock_util;

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
            {
                let mut s = lock_util::lock_or_recover(&state_clone);
                *s = WorkerState::Running;
            }

            // Check if cancelled before starting
            if *lock_util::lock_or_recover(&cancelled_clone) {
                let mut s = lock_util::lock_or_recover(&state_clone);
                *s = WorkerState::Cancelled;
                let mut r = lock_util::lock_or_recover(&result_clone);
                *r = Some(Err(WorkerError::Cancelled));
                return;
            }

            // Execute task with panic handling
            let task_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

            // Store result
            match task_result {
                Ok(value) => {
                    let mut r = lock_util::lock_or_recover(&result_clone);
                    *r = Some(Ok(value));
                    let mut s = lock_util::lock_or_recover(&state_clone);
                    *s = WorkerState::Completed;
                }
                Err(panic_info) => {
                    let msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = panic_info.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "Unknown panic".to_string()
                    };

                    let mut r = lock_util::lock_or_recover(&result_clone);
                    *r = Some(Err(WorkerError::Panicked(msg)));
                    let mut s = lock_util::lock_or_recover(&state_clone);
                    *s = WorkerState::Failed;
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
            {
                let mut s = lock_util::lock_or_recover(&state_clone);
                *s = WorkerState::Running;
            }

            // Use shared runtime for async task (avoids ~100KB allocation per task)
            let result_value = match super::get_runtime_handle() {
                Ok(handle) => Ok(handle.block_on(future)),
                Err(e) => Err(crate::worker::WorkerError::RuntimeCreationFailed(e)),
            };

            // Store result
            {
                let mut r = lock_util::lock_or_recover(&result_clone);
                *r = Some(result_value);
            }
            let mut s = lock_util::lock_or_recover(&state_clone);
            *s = WorkerState::Completed;
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

        let state = Arc::new(Mutex::new(WorkerState::Pending));
        let result: Arc<Mutex<Option<WorkerResult<T>>>> = Arc::new(Mutex::new(None));
        let cancelled = Arc::new(Mutex::new(false));

        let state_clone = state.clone();
        let result_clone = result.clone();
        let cancelled_clone = cancelled.clone();

        let thread = thread::spawn(move || {
            // Update state to running
            {
                let mut s = lock_util::lock_or_recover(&state_clone);
                *s = WorkerState::Running;
            }

            // Simple polling executor with exponential backoff
            // SAFETY:
            // - The vtable functions are all no-ops that don't access any data
            // - RawWaker is created with a dummy pointer since no data is needed
            // - The waker is only used as a placeholder for Context
            // - We never call wake() or wake_by_ref() on this waker
            // - Only clone() and drop() are called, which are safe no-ops
            // - This is a polling executor that never actually needs to wake anything
            //
            // Note: This fallback is only used when the "async" feature is disabled.
            // When async is enabled, the proper tokio runtime is used instead.
            static DUMMY: () = ();
            let dummy_raw_waker = {
                fn no_op(_: *const ()) {}
                fn clone(_: *const ()) -> RawWaker {
                    // Use a valid pointer to DUMMY static instead of null
                    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
                    RawWaker::new(&DUMMY as *const () as *const (), vtable)
                }
                dummy_raw_waker()
            };
            let waker = unsafe { Waker::from_raw(dummy_raw_waker) };
            let mut cx = Context::from_waker(&waker);
            let mut future = Box::pin(future);
            let mut sleep_duration = Duration::from_millis(1);
            const MAX_SLEEP: Duration = Duration::from_millis(10);

            loop {
                // Check cancellation
                if *lock_util::lock_or_recover(&cancelled_clone) {
                    let mut s = lock_util::lock_or_recover(&state_clone);
                    *s = WorkerState::Cancelled;
                    let mut r = lock_util::lock_or_recover(&result_clone);
                    *r = Some(Err(WorkerError::Cancelled));
                    return;
                }

                match Pin::as_mut(&mut future).poll(&mut cx) {
                    Poll::Ready(value) => {
                        let mut r = lock_util::lock_or_recover(&result_clone);
                        *r = Some(Ok(value));
                        let mut s = lock_util::lock_or_recover(&state_clone);
                        *s = WorkerState::Completed;
                        return;
                    }
                    Poll::Pending => {
                        // Exponential backoff: gradually increase sleep up to MAX_SLEEP
                        thread::sleep(sleep_duration);
                        sleep_duration = (sleep_duration * 2).min(MAX_SLEEP);
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
        *lock_util::lock_or_recover(&self.state)
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
        {
            let mut c = lock_util::lock_or_recover(&self.cancelled);
            *c = true;
        }
        let mut s = lock_util::lock_or_recover(&self.state);
        if *s == WorkerState::Pending {
            *s = WorkerState::Cancelled;
        }
    }

    /// Wait for the task to complete and get result
    ///
    /// This blocks the current thread until the task completes.
    ///
    /// # Errors
    ///
    /// Returns `Err(WorkerError::ChannelClosed)` if the result channel is closed.
    /// Returns `Err(WorkerError::Cancelled)` if the task was cancelled.
    /// Returns `Err(WorkerError::Panicked)` if the task panicked.
    pub fn join(mut self) -> WorkerResult<T> {
        // Wait for thread to finish
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }

        // Get result
        lock_util::lock_or_recover(&self.result)
            .take()
            .unwrap_or(Err(WorkerError::ChannelClosed))
    }

    /// Try to get result without blocking
    ///
    /// Returns `None` if the task is still running.
    ///
    /// # Errors
    ///
    /// If the task is finished, returns `Some(Ok(value))` or `Some(Err(...))`:
    /// - `Err(WorkerError::ChannelClosed)` if the result channel is closed
    /// - `Err(WorkerError::Cancelled)` if the task was cancelled
    /// - `Err(WorkerError::Panicked)` if the task panicked
    pub fn try_join(&mut self) -> Option<WorkerResult<T>> {
        if !self.is_finished() {
            return None;
        }

        lock_util::lock_or_recover(&self.result).take()
    }

    /// Wait for completion with timeout
    ///
    /// This blocks the current thread until the task completes or the timeout elapses.
    ///
    /// # Errors
    ///
    /// Returns `Err(WorkerError::Timeout)` if the timeout elapses before completion.
    /// May also return other errors as documented in [`join`](Self::join).
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
        let mut c = lock_util::lock_or_recover(&self.cancelled);
        *c = true;
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

    #[test]
    fn test_is_success() {
        let handle = WorkerHandle::spawn_blocking(|| {
            // Spin loop instead of sleep
            for _ in 0..1000 {
                std::hint::spin_loop();
            }
            42
        });

        // Spin until finished
        let timeout = std::time::Instant::now() + Duration::from_millis(100);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            std::hint::spin_loop();
        }

        // After completion, should be success
        assert!(handle.is_success());

        let _result = handle.join();
    }

    #[test]
    fn test_is_success_after_panic() {
        let handle = WorkerHandle::spawn_blocking(|| {
            for _ in 0..100 {
                std::hint::spin_loop();
            }
            panic!("test panic");
        });

        // Spin until finished
        let timeout = std::time::Instant::now() + Duration::from_millis(100);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            std::hint::spin_loop();
        }

        // After panic, should not be success
        assert!(!handle.is_success());

        let _result = handle.join();
    }

    #[test]
    fn test_is_running() {
        let handle = WorkerHandle::spawn_blocking(|| {
            // Longer spin to keep it running
            for _ in 0..10000 {
                std::hint::spin_loop();
            }
            42
        });

        // Should be running or pending immediately after spawn
        let state = handle.state();
        assert!(matches!(state, WorkerState::Pending | WorkerState::Running));

        // Spin until finished
        let timeout = std::time::Instant::now() + Duration::from_millis(100);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            std::hint::spin_loop();
        }

        // Should no longer be running
        assert!(!handle.is_running());

        let _result = handle.join();
    }

    #[test]
    fn test_try_join_not_finished() {
        let mut handle = WorkerHandle::spawn_blocking(|| {
            // Spin to simulate work
            for _ in 0..10000 {
                std::hint::spin_loop();
            }
            42
        });

        // Immediately should return None (not finished yet)
        assert!(handle.try_join().is_none());

        // Spin until finished
        let timeout = std::time::Instant::now() + Duration::from_millis(100);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            std::hint::spin_loop();
        }

        // After joining, result is taken
        let _result = handle.join();
    }

    #[test]
    fn test_try_join_finished() {
        let mut handle = WorkerHandle::spawn_blocking(|| {
            for _ in 0..100 {
                std::hint::spin_loop();
            }
            42
        });

        // Spin until finished
        let timeout = std::time::Instant::now() + Duration::from_millis(100);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            std::hint::spin_loop();
        }

        let result = handle.try_join();
        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), 42);
    }

    #[test]
    fn test_poll() {
        let handle = WorkerHandle::spawn_blocking(|| {
            for _ in 0..100 {
                std::hint::spin_loop();
            }
            42
        });

        // poll should always return Some(state)
        assert!(handle.poll().is_some());

        // After completion - join consumes the handle
        let result = handle.join();
        assert!(result.is_ok());
    }

    #[test]
    fn test_cancel_pending_task() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(100));
            42
        });

        // Cancel immediately (might be pending or just started)
        handle.cancel();

        // The task should be cancelled
        let result = handle.join();
        // Either cancelled or completed (race condition)
        assert!(result.is_ok() || matches!(result, Err(WorkerError::Cancelled)));
    }

    #[test]
    fn test_join_timeout_success() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        let result = handle.join_timeout(Duration::from_millis(100));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_join_timeout_zero() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        let result = handle.join_timeout(Duration::ZERO);
        // Should timeout or succeed depending on timing
        assert!(result.is_ok() || matches!(result, Err(WorkerError::Timeout)));
    }

    #[test]
    fn test_state_transitions() {
        let handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        // Initial state should be Pending or Running
        let initial_state = handle.state();
        assert!(matches!(
            initial_state,
            WorkerState::Pending | WorkerState::Running
        ));

        // After completion, we can't check state because join consumes the handle
        // But we can verify the result is correct
        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_multiple_try_join_calls() {
        let mut handle = WorkerHandle::spawn_blocking(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        // Wait for completion
        thread::sleep(Duration::from_millis(50));

        // First call returns the result
        let result1 = handle.try_join();
        assert!(result1.is_some());

        // Second call returns None (result was taken)
        let result2 = handle.try_join();
        assert!(result2.is_none());
    }
}
