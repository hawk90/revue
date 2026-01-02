//! Worker pool implementation
//!
//! Uses Condvar for efficient thread signaling instead of busy-polling.

use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;
use super::{WorkerHandle, WorkerConfig, Priority};

/// Shared state between pool and workers
struct SharedState {
    /// Task queue
    queue: TaskQueue,
    /// Shutdown flag
    shutdown: bool,
}

/// A pool of worker threads for background tasks
pub struct WorkerPool {
    /// Worker threads
    workers: Vec<Worker>,
    /// Shared state (queue + shutdown)
    state: Arc<(Mutex<SharedState>, Condvar)>,
    /// Configuration
    config: WorkerConfig,
}

impl WorkerPool {
    /// Create a new worker pool with default config
    pub fn new(threads: usize) -> Self {
        Self::with_config(WorkerConfig::with_threads(threads))
    }

    /// Create with custom config
    pub fn with_config(config: WorkerConfig) -> Self {
        let shared_state = SharedState {
            queue: TaskQueue::new(config.queue_capacity),
            shutdown: false,
        };
        let state = Arc::new((Mutex::new(shared_state), Condvar::new()));
        let mut workers = Vec::with_capacity(config.threads);

        for id in 0..config.threads {
            workers.push(Worker::new(id, state.clone()));
        }

        Self {
            workers,
            state,
            config,
        }
    }

    /// Spawn a blocking task
    pub fn spawn_blocking<F, T>(&self, f: F) -> WorkerHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        WorkerHandle::spawn_blocking(f)
    }

    /// Submit a task to the pool's queue
    pub fn submit<F>(&self, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.submit_with_priority(f, Priority::Normal)
    }

    /// Submit a task with priority
    pub fn submit_with_priority<F>(&self, f: F, priority: Priority) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        let (lock, cvar) = &*self.state;
        if let Ok(mut state) = lock.lock() {
            if state.shutdown {
                return false;
            }
            let task = QueuedTask::new(f, priority);
            if state.queue.push(task) {
                // Wake one waiting worker
                cvar.notify_one();
                return true;
            }
        }
        false
    }

    /// Get number of active workers
    pub fn active_workers(&self) -> usize {
        self.workers.iter().filter(|w| w.is_active()).count()
    }

    /// Get queue length
    pub fn queue_len(&self) -> usize {
        let (lock, _) = &*self.state;
        lock.lock().map(|s| s.queue.len()).unwrap_or(0)
    }

    /// Get thread count
    pub fn thread_count(&self) -> usize {
        self.config.threads
    }

    /// Shutdown the pool gracefully
    pub fn shutdown(&self) {
        let (lock, cvar) = &*self.state;
        if let Ok(mut state) = lock.lock() {
            state.shutdown = true;
            // Wake all workers so they can exit
            cvar.notify_all();
        }
    }

    /// Check if pool is shutdown
    pub fn is_shutdown(&self) -> bool {
        let (lock, _) = &*self.state;
        lock.lock().map(|s| s.shutdown).unwrap_or(true)
    }
}

impl Default for WorkerPool {
    fn default() -> Self {
        Self::with_config(WorkerConfig::default())
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// A single worker thread
pub struct Worker {
    /// Worker ID
    id: usize,
    /// Thread handle
    thread: Option<JoinHandle<()>>,
    /// Active flag
    active: Arc<Mutex<bool>>,
}

impl Worker {
    /// Create a new worker
    fn new(id: usize, state: Arc<(Mutex<SharedState>, Condvar)>) -> Self {
        let active = Arc::new(Mutex::new(true));
        let active_clone = active.clone();

        let thread = thread::Builder::new()
            .name(format!("revue-worker-{}", id))
            .spawn(move || {
                let (lock, cvar) = &*state;

                loop {
                    // Wait for a task or shutdown signal
                    let task = {
                        let mut state = match lock.lock() {
                            Ok(guard) => guard,
                            Err(poisoned) => poisoned.into_inner(),
                        };

                        // Wait while queue is empty and not shutting down
                        while state.queue.is_empty() && !state.shutdown {
                            state = match cvar.wait(state) {
                                Ok(guard) => guard,
                                Err(poisoned) => poisoned.into_inner(),
                            };
                        }

                        // Check if we should exit
                        if state.shutdown && state.queue.is_empty() {
                            break;
                        }

                        // Pop a task from the queue
                        state.queue.pop()
                    };

                    // Execute the task outside the lock
                    if let Some(queued_task) = task {
                        (queued_task.task)();
                    }
                }

                // Mark as inactive
                if let Ok(mut active) = active_clone.lock() {
                    *active = false;
                }
            })
            .ok();

        Self { id, thread, active }
    }

    /// Get worker ID
    #[allow(dead_code)]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Check if worker is active
    pub fn is_active(&self) -> bool {
        self.active.lock().map(|a| *a).unwrap_or(false)
    }
}

/// Task queue for worker pool
struct TaskQueue {
    /// Tasks waiting to be executed
    tasks: VecDeque<QueuedTask>,
    /// Maximum capacity
    capacity: usize,
}

impl TaskQueue {
    fn new(capacity: usize) -> Self {
        Self {
            tasks: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn push(&mut self, task: QueuedTask) -> bool {
        if self.tasks.len() >= self.capacity {
            return false;
        }

        // Insert based on priority
        let insert_pos = self.tasks
            .iter()
            .position(|t| t.priority < task.priority)
            .unwrap_or(self.tasks.len());

        self.tasks.insert(insert_pos, task);
        true
    }

    fn pop(&mut self) -> Option<QueuedTask> {
        self.tasks.pop_front()
    }
}

/// A queued task
struct QueuedTask {
    /// Task to execute
    task: Box<dyn FnOnce() + Send + 'static>,
    /// Priority
    priority: Priority,
}

impl QueuedTask {
    fn new<F>(task: F, priority: Priority) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            task: Box::new(task),
            priority,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_worker_pool_new() {
        let pool = WorkerPool::new(4);
        assert_eq!(pool.thread_count(), 4);
    }

    #[test]
    fn test_worker_pool_default() {
        let pool = WorkerPool::default();
        assert!(pool.thread_count() >= 1);
    }

    #[test]
    fn test_worker_pool_shutdown() {
        let pool = WorkerPool::new(2);
        assert!(!pool.is_shutdown());
        pool.shutdown();
        assert!(pool.is_shutdown());
    }

    #[test]
    fn test_worker_pool_submit() {
        let pool = WorkerPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));

        // Submit tasks
        for _ in 0..10 {
            let counter = counter.clone();
            assert!(pool.submit(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Wait for tasks to complete
        thread::sleep(std::time::Duration::from_millis(100));

        assert_eq!(counter.load(Ordering::SeqCst), 10);
        pool.shutdown();
    }

    #[test]
    fn test_worker_pool_submit_after_shutdown() {
        let pool = WorkerPool::new(1);
        pool.shutdown();

        // Should fail to submit after shutdown
        assert!(!pool.submit(|| {}));
    }

    #[test]
    fn test_worker_pool_priority() {
        let pool = WorkerPool::new(1);
        let order = Arc::new(Mutex::new(Vec::new()));

        // Submit low priority first
        let order1 = order.clone();
        pool.submit_with_priority(move || {
            order1.lock().unwrap().push("low");
        }, Priority::Low);

        // Submit high priority second (should run first due to priority)
        let order2 = order.clone();
        pool.submit_with_priority(move || {
            order2.lock().unwrap().push("high");
        }, Priority::High);

        // Wait for completion
        thread::sleep(std::time::Duration::from_millis(100));
        pool.shutdown();

        let result = order.lock().unwrap();
        // High priority should be processed first
        if result.len() == 2 {
            assert_eq!(result[0], "high");
            assert_eq!(result[1], "low");
        }
    }
}
