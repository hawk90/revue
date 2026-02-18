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
            //
            // SAFETY & CONTRACT VIOLATION WORKAROUND:
            //
            // This code constructs a Waker from a null pointer, which appears to violate
            // the Rust contract for Waker creation. However, this is safe in this specific
            // context because:
            //
            // 1. **No Data Requirement**: The RawWakerVTable functions don't actually use
            //    the pointer value - they're all no-ops that ignore the parameter.
            //
            // 2. **No Wake Operations**: We never call wake() or wake_by_ref() on this waker.
            //    The polling executor continuously polls in a loop rather than waiting for
            //    wake notifications.
            //
            // 3. **Only Safe Operations Called**: The only vtable methods ever invoked are:
            //    - clone(): Creates a new RawWaker with the same null pointer (safe)
            //    - drop(): No-op, no resources to clean up (safe)
            //
            // 4. **Context Usage Only**: The waker is only used to create a Context, which
            //    is passed to Future::poll(). The futures we poll never attempt to wake
            //    themselves (they're designed for polling executors).
            //
            // 5. **Fallback Code Path**: This is only executed when the "async" feature is
            //    disabled. When async is enabled, the proper tokio runtime with valid wakers
            //    is used instead.
            //
            // **WHY NULL POINTER?**
            // Using null() is intentional and safe because:
            // - No memory is being pointed to
            // - No dereferencing occurs
            // - The pointer value is irrelevant to the vtable functions
            // - Alternative approaches (e.g., pointing to dummy data) would be misleading
            //
            // **POTENTIAL ISSUES:**
            // - If a future attempts to call wake() on this waker, it will do nothing
            // - This is acceptable for polling executors where continuous polling is expected
            // - Futures designed for wake-based notification may not work correctly
            //
            // **TESTING:**
            // This code path is tested by:
            // - test_polling_executor_no_async: Basic future polling
            // - test_polling_executor_with_async_fn: Async function execution
            // - test_polling_executor_panic_handling: Panic recovery
            static VTABLE: RawWakerVTable = RawWakerVTable::new(
                // clone: Create a new RawWaker from the same pointer
                // SAFETY: Ignores ptr, returns new null-pointer RawWaker
                |_ptr| RawWaker::new(std::ptr::null(), &VTABLE),
                // wake: No-op since we never wake
                // SAFETY: Does nothing (polling executor doesn't use wake)
                |_ptr| {},
                // wake_by_ref: No-op since we never wake
                // SAFETY: Does nothing (polling executor doesn't use wake)
                |_ptr| {},
                // drop: No-op since we have no resources to clean up
                // SAFETY: Does nothing (no resources allocated)
                |_ptr| {},
            );
            let dummy_raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
            // SAFETY: The dummy_raw_waker is constructed correctly according to the
            // safety contract documented above. While it uses a null pointer, this
            // is safe because:
            // 1. The vtable functions never dereference the pointer
            // 2. Only clone() and drop() are ever called on this waker
            // 3. wake() and wake_by_ref() are never called
            // 4. The waker is only used to construct a Context for polling
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
            // Sleep long enough that try_join() below runs before this completes
            std::thread::sleep(Duration::from_millis(200));
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

        // Wait for completion using is_finished() instead of fixed sleep
        let timeout = std::time::Instant::now() + Duration::from_millis(200);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            thread::sleep(Duration::from_millis(1));
        }

        // First call returns the result
        let result1 = handle.try_join();
        assert!(result1.is_some());

        // Second call returns None (result was taken)
        let result2 = handle.try_join();
        assert!(result2.is_none());
    }

    // =============================================================================
    // Security: Polling Executor Tests (Raw Waker Code Path)
    // =============================================================================

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_polling_executor_no_async() {
        use std::future;

        // Test basic future polling with the raw waker
        let handle = WorkerHandle::spawn(async {
            // Simple async block that returns a value
            42
        });

        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_polling_executor_with_async_fn() {
        use std::future;

        // Test async function execution
        async fn async_add(a: i32, b: i32) -> i32 {
            a + b
        }

        let handle = WorkerHandle::spawn(async_add(10, 20));
        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 30);
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_polling_executor_panic_handling() {
        use std::future;

        // Test that panics in async code cause thread termination
        // Note: The polling executor runs in a thread, so panics will
        // terminate that thread. The handle should detect this as a failure.
        let handle = WorkerHandle::spawn(async {
            panic!("intentional panic in async");
        });

        let result = handle.join();
        // The panic will cause the thread to terminate, resulting in an error
        assert!(result.is_err());
        // It might be Panicked or another error depending on how the thread exits
        match result {
            Err(WorkerError::Panicked(_)) => {
                // Expected: panic was caught
            }
            Err(_) => {
                // Also acceptable: thread terminated due to panic
            }
            Ok(_) => {
                panic!("Expected error from panicked async block");
            }
        }
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_polling_executor_cancellation() {
        use std::future;

        // Test cancellation of polling executor tasks
        let handle = WorkerHandle::spawn(async {
            // This would run forever if not cancelled
            std::future::pending::<()>()
        });

        // Cancel immediately
        handle.cancel();

        let result = handle.join_timeout(Duration::from_millis(100));
        // Should either be cancelled or timeout
        assert!(result.is_err());
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_polling_executor_multiple_awaits() {
        use std::future;

        // Test future with multiple await points
        async fn multi_await() -> i32 {
            let mut sum = 0;
            for i in 1..=5 {
                sum += i;
                // Simulate some async work
                let _ = std::future::ready(i).await;
            }
            sum
        }

        let handle = WorkerHandle::spawn(multi_await());
        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 15); // 1+2+3+4+5 = 15
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_polling_executor_state_transitions() {
        use std::future;

        let handle = WorkerHandle::spawn(async { 42 });

        // Should start in Pending or Running
        let initial_state = handle.state();
        assert!(matches!(
            initial_state,
            WorkerState::Pending | WorkerState::Running
        ));

        // Wait for completion
        let timeout = std::time::Instant::now() + Duration::from_millis(100);
        while !handle.is_finished() && std::time::Instant::now() < timeout {
            thread::sleep(Duration::from_millis(1));
        }

        // Should be completed now
        assert!(handle.is_finished());

        let result = handle.join();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
}
