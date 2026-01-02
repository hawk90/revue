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
}
