//! Worker handle integration tests
//!
//! Tests for task handles, state management, and result retrieval.

use revue::worker::{WorkerError, WorkerHandle, WorkerState};
use std::thread;
use std::time::{Duration, Instant};

/// Poll for a condition with a timeout, returning when the condition becomes true
/// or the timeout elapses. Returns true if condition was met, false on timeout.
fn poll_until<F>(mut condition: F, timeout_ms: u64) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms);
    let poll_interval = Duration::from_millis(2);

    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        thread::sleep(poll_interval);
    }
    false
}

// =============================================================================
// Blocking Tasks Tests
// =============================================================================

#[test]
fn test_spawn_blocking_return_types() {
    // Test various return types
    let handle1 = WorkerHandle::spawn_blocking(|| 42i32);
    assert_eq!(handle1.join().unwrap(), 42);

    let handle2 = WorkerHandle::spawn_blocking(|| "Hello".to_string());
    assert_eq!(handle2.join().unwrap(), "Hello");

    let handle3 = WorkerHandle::spawn_blocking(|| vec![1, 2, 3]);
    assert_eq!(handle3.join().unwrap(), vec![1, 2, 3]);

    let handle4 = WorkerHandle::spawn_blocking(|| ());
    assert_eq!(handle4.join().unwrap(), ());

    let handle5 = WorkerHandle::spawn_blocking(|| (1, "two", 3.0));
    assert_eq!(handle5.join().unwrap(), (1, "two", 3.0));
}

#[test]
fn test_spawn_blocking_panic_recovery() {
    let handle = WorkerHandle::spawn_blocking(|| {
        panic!("intentional panic");
    });

    let result = handle.join();
    assert!(matches!(result, Err(WorkerError::Panicked(_))));
}

#[test]
fn test_spawn_blocking_large_result() {
    let large_data: Vec<u8> = (0..10000).map(|i| i as u8).collect();

    let handle = WorkerHandle::spawn_blocking(move || large_data.len());

    assert_eq!(handle.join().unwrap(), 10000);
}

#[test]
fn test_spawn_blocking_immediate() {
    let handle = WorkerHandle::spawn_blocking(|| 42);
    assert!(!handle.is_finished());

    // Wait for task to finish
    poll_until(|| handle.is_finished(), 500);

    assert!(handle.is_finished());
}

// =============================================================================
// State Management Tests
// =============================================================================

#[test]
fn test_state_transitions() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(50));
        42
    });

    // Should transition Pending -> Running -> Completed
    let state = handle.state();
    assert!(matches!(
        state,
        WorkerState::Pending | WorkerState::Running | WorkerState::Completed
    ));

    // Wait for completion
    poll_until(|| matches!(handle.state(), WorkerState::Completed), 500);

    assert!(matches!(handle.state(), WorkerState::Completed));
}

#[test]
fn test_is_finished_success() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(10));
        "done"
    });

    assert!(!handle.is_finished());

    poll_until(|| handle.is_finished(), 500);

    assert!(handle.is_finished());
    assert!(handle.is_success());
}

#[test]
fn test_is_finished_failure() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(10));
        panic!("failed");
    });

    assert!(!handle.is_finished());

    poll_until(|| handle.is_finished(), 500);

    assert!(handle.is_finished());
    assert!(!handle.is_success());
}

#[test]
fn test_is_running() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(100));
        42
    });

    // May be running or pending initially
    let _initial_state = handle.state();

    // Wait a bit then check running state
    poll_until(|| handle.is_running(), 200);
    assert!(handle.is_running());

    // Wait for completion
    poll_until(|| !handle.is_running(), 500);
    assert!(!handle.is_running());
    assert!(handle.is_finished());
}

#[test]
fn test_cancel_running_task() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(100));
        "never"
    });

    // Cancel the task
    handle.cancel();

    // Note: cancel mainly affects pending tasks
    // Running tasks continue to completion
    let result = handle.join();
    // Result depends on timing - may complete or be cancelled
    let _ = result;
}

#[test]
fn test_cancel_before_start() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(100));
        42
    });

    handle.cancel();

    // Task should be cancelled if still pending
    // But once running, cancel just sets the flag
    let state = handle.state();
    let _ = state;
}

// =============================================================================
// Result Retrieval Tests
// =============================================================================

#[test]
fn test_try_join_not_ready() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(100));
        42
    });

    let mut handle = handle;
    let result = handle.try_join();
    assert!(result.is_none());
}

#[test]
fn test_try_join_ready() {
    let mut handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(50)); // Give task time to complete
        42
    });

    // Wait for task to complete - use is_finished() instead of try_join()
    // because try_join() consumes the result, leaving nothing for the assertion
    poll_until(|| handle.is_finished(), 500);

    let result = handle.try_join();
    assert!(result.is_some(), "Task should be ready by now");
    assert!(result.as_ref().unwrap().is_ok());
    assert_eq!(result.unwrap().unwrap(), 42);
}

#[test]
fn test_join_timeout_success() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });

    let result = handle.join_timeout(Duration::from_millis(100));
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_join_timeout_failure() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_secs(10));
        42
    });

    let result = handle.join_timeout(Duration::from_millis(50));
    assert!(matches!(result, Err(WorkerError::Timeout)));
}

#[test]
fn test_join_consume() {
    let handle = WorkerHandle::spawn_blocking(|| 42);

    let result = handle.join();
    assert_eq!(result.unwrap(), 42);

    // handle is consumed after join
}

#[test]
fn test_join_panic_result() {
    let handle = WorkerHandle::spawn_blocking(|| {
        panic!("test panic");
        #[allow(unreachable_code)]
        42
    });

    let result = handle.join();
    assert!(matches!(result, Err(WorkerError::Panicked(_))));

    if let Err(WorkerError::Panicked(msg)) = result {
        assert!(msg.contains("test panic"));
    }
}

// =============================================================================
// Async Task Tests (if feature is enabled)
// =============================================================================

#[cfg(feature = "async")]
#[test]
fn test_spawn_async_basic() {
    let handle = WorkerHandle::spawn(async {
        thread::sleep(Duration::from_millis(10));
        42
    });

    let result = handle.join();
    assert_eq!(result.unwrap(), 42);
}

// TODO: Fix this test - worker pool doesn't support async tasks properly
// The worker thread already has a tokio runtime initialized, causing
// "Another thread initialized runtime first" panic when spawning async tasks.
// This test needs the worker pool to be async-aware or use a different approach.
#[cfg(feature = "async")]
#[test]
fn test_spawn_async_multiple_await() {
    let handle = WorkerHandle::spawn(async {
        let a = async { 1 }.await;
        let b = async { 2 }.await;
        a + b
    });

    let result = handle.join();
    assert_eq!(result.unwrap(), 3);
}

// =============================================================================
// Poll Tests
// =============================================================================

#[test]
fn test_poll_returns_state() {
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });

    let state = handle.poll();
    assert!(state.is_some());
    assert!(matches!(
        state.unwrap(),
        WorkerState::Pending | WorkerState::Running
    ));

    // Wait for completion with timeout
    let completed = poll_until(
        || matches!(handle.poll(), Some(WorkerState::Completed)),
        500,
    );
    assert!(completed, "Worker did not complete in time");
}

// =============================================================================
// Error Tests
// =============================================================================

#[test]
fn test_worker_error_display() {
    let err = WorkerError::Cancelled;
    assert_eq!(format!("{}", err), "Worker task was cancelled");

    let err = WorkerError::Panicked("test".to_string());
    assert!(format!("{}", err).contains("test"));

    let err = WorkerError::ChannelClosed;
    assert_eq!(format!("{}", err), "Worker channel closed");

    let err = WorkerError::Timeout;
    assert_eq!(format!("{}", err), "Worker task timed out");

    let err = WorkerError::Custom("custom error".to_string());
    assert!(format!("{}", err).contains("custom error"));
}

#[test]
fn test_worker_error_clone() {
    let err1 = WorkerError::Custom("test".to_string());
    let err2 = err1.clone();
    assert_eq!(format!("{}", err1), format!("{}", err2));
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_multiple_handles_concurrent() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            WorkerHandle::spawn_blocking(move || {
                thread::sleep(Duration::from_millis(10));
                i * 2
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    assert_eq!(results.len(), 10);
    assert_eq!(results[0], 0);
    assert_eq!(results[9], 18);
}

#[test]
fn test_handle_drop_cancels() {
    // Handle drop sets cancel flag
    let handle = WorkerHandle::spawn_blocking(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });

    // Explicitly drop
    drop(handle);

    // Task may have already completed, that's ok
}

#[test]
fn test_state_after_completion() {
    let handle = WorkerHandle::spawn_blocking(|| 42);

    thread::sleep(Duration::from_millis(50));

    // State should be Completed
    assert_eq!(handle.state(), WorkerState::Completed);
    assert!(handle.is_success());
}

#[test]
fn test_state_after_panic() {
    let handle = WorkerHandle::spawn_blocking(|| {
        panic!("test");
    });

    thread::sleep(Duration::from_millis(50));

    // State should be Failed
    assert_eq!(handle.state(), WorkerState::Failed);
    assert!(!handle.is_success());
}

#[test]
fn test_run_blocking_convenience() {
    let handle = revue::worker::run_blocking(|| {
        thread::sleep(Duration::from_millis(10));
        "result"
    });

    assert_eq!(handle.join().unwrap(), "result");
}

#[cfg(feature = "async")]
#[test]
fn test_spawn_convenience() {
    let handle = revue::worker::spawn(async {
        thread::sleep(Duration::from_millis(10));
        "async result"
    });

    assert_eq!(handle.join().unwrap(), "async result");
}

#[test]
fn test_try_join_consumes() {
    let handle = WorkerHandle::spawn_blocking(|| 42);

    thread::sleep(Duration::from_millis(50));

    let mut handle_ref = handle;
    let result1 = handle_ref.try_join();
    assert!(result1.is_some());

    // Result should be consumed
    let result2 = handle_ref.try_join();
    assert!(result2.is_none());
}
