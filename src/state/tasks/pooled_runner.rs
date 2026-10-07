//! Thread pool-based task runner with bounded concurrency
//!
//! Prevents thread explosion by using a fixed number of worker threads
//! to process a queue of tasks.

use crate::constants::MAX_TASK_QUEUE_SIZE;
use crate::utils::lock::lock_or_recover;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Unique task identifier (must be unique per task)
pub type TaskId = String;

/// Task result with ID
#[derive(Debug)]
pub struct TaskResult<T> {
    /// Task identifier
    pub id: TaskId,
    /// Task result (success or error message)
    pub result: Result<T, String>,
}

/// A queued task; an error it returns is its failure (not a panic).
type Task<T> = Box<dyn FnOnce() -> Result<T, String> + Send + 'static>;

/// Work item submitted to the pool
struct WorkItem<T> {
    id: TaskId,
    task: Task<T>,
}

/// Message from worker to main thread
struct ResultMessage<T> {
    id: TaskId,
    result: Result<T, String>,
}

/// Worker thread that processes tasks from the queue
struct Worker {
    _handle: JoinHandle<()>,
}

impl Worker {
    fn new<T: Send + 'static>(
        work_rx: Arc<Mutex<Receiver<WorkItem<T>>>>,
        result_tx: Sender<ResultMessage<T>>,
    ) -> Self {
        let handle = thread::spawn(move || {
            loop {
                // Try to get work from the queue
                let work_item = {
                    let rx = lock_or_recover(&work_rx);
                    rx.recv()
                };

                match work_item {
                    Ok(item) => {
                        // Execute the task
                        let result =
                            crate::render::catch_panic(std::panic::AssertUnwindSafe(|| {
                                (item.task)()
                            }));

                        let msg = ResultMessage {
                            id: item.id,
                            result: result.unwrap_or_else(|e| {
                                Err(format!("Task panicked: {}", super::panic_message(&*e)))
                            }),
                        };

                        // Send result back
                        let _ = result_tx.send(msg);
                    }
                    Err(_) => {
                        // Channel closed, worker exits
                        break;
                    }
                }
            }
        });

        Worker { _handle: handle }
    }
}

/// Thread pool-based task runner with bounded concurrency
///
/// Unlike the basic TaskRunner that spawns a new thread for each task,
/// this uses a fixed pool of worker threads to process tasks from a queue.
///
/// # Example
///
/// ```ignore
/// use revue::tasks::PooledTaskRunner;
///
/// // Create a pool with 4 worker threads
/// let mut tasks = PooledTaskRunner::new(4);
///
/// // Spawn multiple tasks (won't create 100 threads!)
/// for i in 0..100 {
///     tasks.spawn(format!("task_{}", i), move || {
///         // Expensive network/IO operation
///         fetch_data(i)
///     });
/// }
///
/// // In your tick loop, poll for results
/// while let Some(result) = tasks.poll() {
///     println!("Task {} completed: {:?}", result.id, result.result);
/// }
/// ```
pub struct PooledTaskRunner<T: Send + 'static> {
    /// Channel for submitting work to the pool
    work_tx: mpsc::SyncSender<WorkItem<T>>,
    /// Shared receiver for workers to get work (kept alive for workers)
    _work_rx: Arc<Mutex<Receiver<WorkItem<T>>>>,
    /// Channel for receiving results
    result_rx: Receiver<ResultMessage<T>>,
    /// Sender for workers to send results (kept alive for workers)
    _result_tx: Sender<ResultMessage<T>>,
    /// Worker threads
    _workers: Vec<Worker>,
    /// Pending tasks (to prevent duplicate IDs)
    pending: HashMap<TaskId, ()>,
    /// Queue of tasks waiting to be submitted (for future backpressure)
    _queue: VecDeque<(TaskId, Task<T>)>,
}

impl<T: Send + 'static> PooledTaskRunner<T> {
    /// Create a new pooled task runner with the specified number of workers
    ///
    /// # Arguments
    ///
    /// * `num_workers` - Number of worker threads (typically 2-8)
    ///
    /// # Panics
    ///
    /// Panics if `num_workers` is 0
    pub fn new(num_workers: usize) -> Self {
        assert!(num_workers > 0, "Must have at least 1 worker");

        // Use bounded channel to prevent unbounded queue growth
        let (work_tx, work_rx) = mpsc::sync_channel(MAX_TASK_QUEUE_SIZE);
        let (result_tx, result_rx) = mpsc::channel();
        let work_rx = Arc::new(Mutex::new(work_rx));

        let mut workers = Vec::with_capacity(num_workers);
        for _ in 0..num_workers {
            workers.push(Worker::new(work_rx.clone(), result_tx.clone()));
        }

        Self {
            work_tx,
            _work_rx: work_rx,
            result_rx,
            _result_tx: result_tx,
            _workers: workers,
            pending: HashMap::new(),
            _queue: VecDeque::new(),
        }
    }

    /// Spawn a task in the thread pool
    ///
    /// The task will be queued and executed by an available worker thread.
    /// If a task with the same ID is already pending, this is a no-op.
    ///
    /// If the task queue is full, the task will be rejected (not spawned):
    /// [`is_pending`](Self::is_pending) is then false for its id.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique task identifier
    /// * `task` - Function to execute in the background
    pub fn spawn<F>(&mut self, id: impl Into<TaskId>, task: F)
    where
        F: FnOnce() -> T + Send + 'static,
    {
        self.submit(id.into(), Box::new(move || Ok(task())));
    }

    /// Queue a work item unless a task with its id is already pending.
    fn submit(&mut self, id: TaskId, task: Task<T>) {
        if self.pending.contains_key(&id) {
            return; // Task already running
        }

        // Submit to pool (non-blocking). If the queue is full the task is
        // rejected to prevent blocking - and is not pending, or it would stay
        // pending forever and block its id.
        let work_item = WorkItem {
            id: id.clone(),
            task,
        };
        if self.work_tx.try_send(work_item).is_ok() {
            self.pending.insert(id, ());
        }
    }

    /// Spawn a task that returns Result
    pub fn spawn_result<F, E>(&mut self, id: impl Into<TaskId>, task: F)
    where
        F: FnOnce() -> Result<T, E> + Send + 'static,
        E: std::fmt::Display,
    {
        // The error travels as the result, not as a panic: a panic would
        // reach the panic hook, which prints over the app's screen.
        self.submit(
            id.into(),
            Box::new(move || task().map_err(|e| e.to_string())),
        );
    }

    /// Poll for completed task results (non-blocking)
    ///
    /// Returns `Some(result)` if a task has completed, `None` if no results are ready.
    /// Call this in your tick/update loop to process results.
    pub fn poll(&mut self) -> Option<TaskResult<T>> {
        match self.result_rx.try_recv() {
            Ok(msg) => {
                self.pending.remove(&msg.id);
                Some(TaskResult {
                    id: msg.id,
                    result: msg.result,
                })
            }
            Err(_) => None,
        }
    }

    /// Check if there are any pending tasks
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Get number of pending tasks
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Check if a specific task is pending
    pub fn is_pending(&self, id: &str) -> bool {
        self.pending.contains_key(id)
    }
}

impl<T: Send + 'static> Drop for PooledTaskRunner<T> {
    fn drop(&mut self) {
        // Dropping work_tx will cause all workers to exit gracefully
        // when they finish their current task
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    /// Poll until a result arrives, failing after `timeout`. A fixed sleep
    /// before a single poll races the worker on a slow machine.
    fn poll_within<T: Send + 'static>(
        runner: &mut PooledTaskRunner<T>,
        timeout: Duration,
    ) -> TaskResult<T> {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            if let Some(result) = runner.poll() {
                return result;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "no task result within {timeout:?}"
            );
            thread::sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn test_pooled_runner_basic() {
        let mut runner = PooledTaskRunner::new(2);

        runner.spawn("task1", || 42);
        runner.spawn("task2", || 100);

        assert!(runner.has_pending());

        // Wait for results
        let mut results = Vec::new();
        for _ in 0..2 {
            while results.len() < 2 {
                if let Some(result) = runner.poll() {
                    results.push(result);
                }
                thread::sleep(Duration::from_millis(10));
            }
        }

        assert_eq!(results.len(), 2);
        assert!(!runner.has_pending());
    }

    #[test]
    fn test_pooled_runner_many_tasks() {
        let mut runner = PooledTaskRunner::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        // Spawn 100 tasks (but only 4 threads will be used)
        for i in 0..100 {
            let counter = counter.clone();
            runner.spawn(format!("task_{}", i), move || {
                counter.fetch_add(1, Ordering::SeqCst);
                i
            });
        }

        // Collect all results
        let mut results = Vec::new();
        while results.len() < 100 {
            if let Some(result) = runner.poll() {
                assert!(result.result.is_ok());
                results.push(result);
            }
            thread::sleep(Duration::from_millis(1));
        }

        assert_eq!(results.len(), 100);
        assert_eq!(counter.load(Ordering::SeqCst), 100);
        assert!(!runner.has_pending());
    }

    #[test]
    fn test_pooled_runner_duplicate_id() {
        let mut runner = PooledTaskRunner::new(2);

        runner.spawn("duplicate", || 1);
        runner.spawn("duplicate", || 2); // Should be ignored

        let result = poll_within(&mut runner, Duration::from_secs(5));
        assert_eq!(result.result, Ok(1));
        // The second spawn was never queued, so nothing is left to arrive
        assert!(!runner.has_pending());
        assert!(runner.poll().is_none());
    }

    #[test]
    fn test_pooled_runner_panic_handling() {
        let mut runner = PooledTaskRunner::<i32>::new(2);

        runner.spawn("panic_task", || {
            panic!("Test panic");
        });

        let result = poll_within(&mut runner, Duration::from_secs(5));
        assert!(result.result.unwrap_err().contains("panicked"));
    }

    #[test]
    fn test_pooled_runner_bounded_concurrency() {
        let runner = PooledTaskRunner::<()>::new(2);

        // With 2 workers, we should never have more than 2 threads running tasks concurrently
        // This is a property of the thread pool design
        assert_eq!(runner._workers.len(), 2);
    }

    #[test]
    fn test_pooled_runner_pending_count() {
        let mut runner = PooledTaskRunner::new(2);

        assert_eq!(runner.pending_count(), 0);

        runner.spawn("task1", || 42);
        runner.spawn("task2", || 100);
        runner.spawn("task3", || 200);

        assert_eq!(runner.pending_count(), 3);

        // Wait for one result
        while runner.poll().is_none() {
            thread::sleep(Duration::from_millis(10));
        }

        assert!(runner.pending_count() < 3);
    }

    #[test]
    fn test_pooled_runner_is_pending() {
        let mut runner = PooledTaskRunner::new(2);

        assert!(!runner.is_pending("task1"));

        runner.spawn("task1", || 42);
        assert!(runner.is_pending("task1"));
        assert!(!runner.is_pending("task2"));

        // Wait for completion
        while runner.poll().is_none() {
            thread::sleep(Duration::from_millis(10));
        }

        assert!(!runner.is_pending("task1"));
    }

    #[test]
    fn test_pooled_runner_spawn_result_ok() {
        let mut runner = PooledTaskRunner::new(2);

        runner.spawn_result("task1", || Ok::<i32, &str>(42));

        while let Some(result) = runner.poll() {
            assert_eq!(result.id, "task1");
            assert!(result.result.is_ok());
            assert_eq!(result.result.unwrap(), 42);
        }
    }

    #[test]
    fn test_pooled_runner_spawn_result_err() {
        let mut runner = PooledTaskRunner::new(2);

        runner.spawn_result("task1", || Err::<i32, &str>("error"));

        while let Some(result) = runner.poll() {
            assert_eq!(result.id, "task1");
            assert!(result.result.is_err());
        }
    }

    #[test]
    fn test_pooled_runner_no_workers_panics() {
        // Creating with 0 workers should panic
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _runner = PooledTaskRunner::<i32>::new(0);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_pooled_runner_poll_empty() {
        let mut runner = PooledTaskRunner::<i32>::new(2);
        assert!(runner.poll().is_none());
    }

    #[test]
    fn test_task_result_fields() {
        let result = TaskResult {
            id: "test".to_string(),
            result: Ok(42),
        };
        assert_eq!(result.id, "test");
        assert_eq!(result.result.unwrap(), 42);
    }

    #[test]
    fn a_failure_keeps_its_message() {
        let mut runner: PooledTaskRunner<u32> = PooledTaskRunner::new(1);
        runner.spawn_result("err", || Err::<u32, _>("permission denied"));
        let r = poll_within(&mut runner, Duration::from_secs(5));
        assert_eq!(r.result, Err("permission denied".to_string()));

        runner.spawn("boom", || {
            std::panic::resume_unwind(Box::new("the disk is gone"))
        });
        let r = poll_within(&mut runner, Duration::from_secs(5));
        assert_eq!(r.result, Err("Task panicked: the disk is gone".to_string()));
    }

    #[test]
    fn a_task_the_full_queue_refuses_is_not_pending() {
        let (release_tx, release_rx) = std::sync::mpsc::channel::<()>();
        let release_rx = Arc::new(std::sync::Mutex::new(release_rx));
        let mut runner: PooledTaskRunner<usize> = PooledTaskRunner::new(1);
        let total = MAX_TASK_QUEUE_SIZE + 5;
        for i in 0..total {
            let rx = release_rx.clone();
            runner.spawn(format!("t{i}"), move || {
                let _ = rx.lock().unwrap().recv_timeout(Duration::from_secs(5));
                i
            });
        }
        let accepted = runner.pending_count();
        assert!(accepted < total, "a full queue accepted every task");
        assert!(!runner.is_pending(&format!("t{}", total - 1)));
        drop(release_tx);

        let mut got = 0;
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while runner.has_pending() && std::time::Instant::now() < deadline {
            while runner.poll().is_some() {
                got += 1;
            }
            thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(got, accepted);
        assert_eq!(runner.pending_count(), 0);
    }
}
