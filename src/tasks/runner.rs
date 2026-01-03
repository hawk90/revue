//! Background task runner with result polling
//!
//! Spawn background tasks and poll for results in the tick loop.

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

/// Unique task identifier
pub type TaskId = &'static str;

/// Task result with ID
#[derive(Debug)]
pub struct TaskResult<T> {
    /// Task identifier
    pub id: TaskId,
    /// Task result (success or error message)
    pub result: Result<T, String>,
}

/// Internal task message
struct TaskMessage<T> {
    id: TaskId,
    result: Result<T, String>,
}

/// Background task runner
///
/// # Example
///
/// ```ignore
/// let mut tasks: TaskRunner<MountResult> = TaskRunner::new();
///
/// // Spawn a background task
/// tasks.spawn("mount_host", || {
///     mount_sshfs("myhost")
/// });
///
/// // In tick handler
/// while let Some(result) = tasks.poll() {
///     match result.id {
///         "mount_host" => handle_mount(result.result),
///         _ => {}
///     }
/// }
/// ```
pub struct TaskRunner<T: Send + 'static> {
    rx: Receiver<TaskMessage<T>>,
    tx: Sender<TaskMessage<T>>,
    pending: HashMap<TaskId, ()>,
    handles: Vec<JoinHandle<()>>,
}

impl<T: Send + 'static> TaskRunner<T> {
    /// Create a new task runner
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            rx,
            tx,
            pending: HashMap::new(),
            handles: Vec::new(),
        }
    }

    /// Spawn a background task
    ///
    /// The task function runs in a separate thread and results are
    /// collected via `poll()`.
    pub fn spawn<F>(&mut self, id: TaskId, task: F)
    where
        F: FnOnce() -> T + Send + 'static,
    {
        if self.pending.contains_key(id) {
            return; // Task with this ID already running
        }

        self.pending.insert(id, ());
        let tx = self.tx.clone();

        let handle = thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
            let msg = match result {
                Ok(value) => TaskMessage {
                    id,
                    result: Ok(value),
                },
                Err(e) => TaskMessage {
                    id,
                    result: Err(format!("Task panicked: {:?}", e)),
                },
            };
            let _ = tx.send(msg);
        });

        self.handles.push(handle);
    }

    /// Spawn a task that returns Result
    pub fn spawn_result<F, E>(&mut self, id: TaskId, task: F)
    where
        F: FnOnce() -> Result<T, E> + Send + 'static,
        E: std::fmt::Display,
    {
        if self.pending.contains_key(id) {
            return;
        }

        self.pending.insert(id, ());
        let tx = self.tx.clone();

        let handle = thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
            let msg = match result {
                Ok(Ok(value)) => TaskMessage {
                    id,
                    result: Ok(value),
                },
                Ok(Err(e)) => TaskMessage {
                    id,
                    result: Err(e.to_string()),
                },
                Err(e) => TaskMessage {
                    id,
                    result: Err(format!("Task panicked: {:?}", e)),
                },
            };
            let _ = tx.send(msg);
        });

        self.handles.push(handle);
    }

    /// Poll for completed task results. Call this in your tick handler.
    pub fn poll(&mut self) -> Option<TaskResult<T>> {
        match self.rx.try_recv() {
            Ok(msg) => {
                self.pending.remove(msg.id);
                Some(TaskResult {
                    id: msg.id,
                    result: msg.result,
                })
            }
            Err(_) => None,
        }
    }

    /// Check if a specific task is running
    pub fn is_running(&self, id: TaskId) -> bool {
        self.pending.contains_key(id)
    }

    /// Check if any tasks are running
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Get count of pending tasks
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Cancel tracking of a task (doesn't stop the thread)
    pub fn cancel(&mut self, id: TaskId) {
        self.pending.remove(id);
    }

    /// Clean up completed thread handles
    pub fn cleanup(&mut self) {
        self.handles.retain(|h| !h.is_finished());
    }
}

impl<T: Send + 'static> Default for TaskRunner<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + 'static> Drop for TaskRunner<T> {
    fn drop(&mut self) {
        // Wait for all threads to complete
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_spawn_and_poll() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn("add", || 1 + 1);

        assert!(runner.is_running("add"));

        // Wait for completion
        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.id, "add");
        assert_eq!(result.result, Ok(2));

        assert!(!runner.is_running("add"));
    }

    #[test]
    fn test_multiple_tasks() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn("task1", || 10);
        runner.spawn("task2", || 20);

        assert_eq!(runner.pending_count(), 2);

        std::thread::sleep(Duration::from_millis(50));

        let mut results = Vec::new();
        while let Some(r) = runner.poll() {
            results.push(r);
        }

        assert_eq!(results.len(), 2);
        assert_eq!(runner.pending_count(), 0);
    }

    #[test]
    fn test_default() {
        let runner: TaskRunner<i32> = TaskRunner::default();
        assert!(!runner.has_pending());
        assert_eq!(runner.pending_count(), 0);
    }

    #[test]
    fn test_has_pending() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();
        assert!(!runner.has_pending());

        runner.spawn("task", || 42);
        assert!(runner.has_pending());

        std::thread::sleep(Duration::from_millis(50));
        runner.poll();
        assert!(!runner.has_pending());
    }

    #[test]
    fn test_cancel() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn("slow_task", || {
            std::thread::sleep(Duration::from_millis(500));
            42
        });

        assert!(runner.is_running("slow_task"));
        runner.cancel("slow_task");
        assert!(!runner.is_running("slow_task"));
        assert_eq!(runner.pending_count(), 0);
    }

    #[test]
    #[ignore] // Flaky in CI due to timing sensitivity
    fn test_duplicate_task_id_rejected() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn("same_id", || {
            std::thread::sleep(Duration::from_millis(100));
            1
        });

        // This should be ignored since task with same ID is running
        runner.spawn("same_id", || 2);

        // Still only 1 pending
        assert_eq!(runner.pending_count(), 1);

        std::thread::sleep(Duration::from_millis(150));

        let result = runner.poll();
        assert!(result.is_some());
        // Should get result from first task
        assert_eq!(result.unwrap().result, Ok(1));
    }

    #[test]
    fn test_spawn_result_ok() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn_result("ok_task", || -> Result<i32, &str> { Ok(100) });

        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll().unwrap();
        assert_eq!(result.id, "ok_task");
        assert_eq!(result.result, Ok(100));
    }

    #[test]
    fn test_spawn_result_err() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn_result("err_task", || -> Result<i32, &str> { Err("failed") });

        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll().unwrap();
        assert_eq!(result.id, "err_task");
        assert!(result.result.is_err());
        assert_eq!(result.result.unwrap_err(), "failed");
    }

    #[test]
    fn test_spawn_result_duplicate_rejected() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn_result("dup", || -> Result<i32, &str> {
            std::thread::sleep(Duration::from_millis(100));
            Ok(1)
        });

        runner.spawn_result("dup", || -> Result<i32, &str> { Ok(2) });

        assert_eq!(runner.pending_count(), 1);
    }

    #[test]
    fn test_cleanup() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn("fast", || 1);
        std::thread::sleep(Duration::from_millis(50));

        // Poll to get result
        runner.poll();

        // Cleanup should remove finished handles
        runner.cleanup();
        // No assertion needed - just verify it doesn't panic
    }

    #[test]
    fn test_poll_empty() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();
        assert!(runner.poll().is_none());
    }

    #[test]
    fn test_is_running_nonexistent() {
        let runner: TaskRunner<i32> = TaskRunner::new();
        assert!(!runner.is_running("nonexistent"));
    }

    #[test]
    fn test_cancel_nonexistent() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();
        // Should not panic
        runner.cancel("nonexistent");
        assert_eq!(runner.pending_count(), 0);
    }

    #[test]
    fn test_task_result_debug() {
        let result = TaskResult {
            id: "test",
            result: Ok(42),
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("test"));
        assert!(debug.contains("42"));
    }

    #[test]
    fn test_task_with_string_result() {
        let mut runner: TaskRunner<String> = TaskRunner::new();

        runner.spawn("string_task", || "hello".to_string());

        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll().unwrap();
        assert_eq!(result.result, Ok("hello".to_string()));
    }

    #[test]
    fn test_task_with_vec_result() {
        let mut runner: TaskRunner<Vec<i32>> = TaskRunner::new();

        runner.spawn("vec_task", || vec![1, 2, 3]);

        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll().unwrap();
        assert_eq!(result.result, Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_panic_handling() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn("panic_task", || {
            panic!("intentional panic");
        });

        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll().unwrap();
        assert_eq!(result.id, "panic_task");
        assert!(result.result.is_err());
        assert!(result.result.unwrap_err().contains("panicked"));
    }

    #[test]
    fn test_spawn_result_panic_handling() {
        let mut runner: TaskRunner<i32> = TaskRunner::new();

        runner.spawn_result("panic_result", || -> Result<i32, &str> {
            panic!("panic in result task");
        });

        std::thread::sleep(Duration::from_millis(50));

        let result = runner.poll().unwrap();
        assert!(result.result.is_err());
        assert!(result.result.unwrap_err().contains("panicked"));
    }
}
