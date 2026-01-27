//! Worker pool implementation
//!
//! Uses Condvar for efficient thread signaling instead of busy-polling.
//! Uses BinaryHeap for O(log n) priority queue operations.

use super::{Priority, WorkerConfig, WorkerHandle};
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

// Use lock utilities for consistent poison handling
use crate::utils::lock as lock_util;

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
        let mut state = lock_util::lock_or_recover(lock);
        if state.shutdown {
            return false;
        }

        // Get and increment sequence number for FIFO ordering
        let seq = state.queue.next_seq;
        state.queue.next_seq = state.queue.next_seq.wrapping_add(1);

        let task = QueuedTask::new(f, priority, seq);
        if state.queue.push(task) {
            // Wake one waiting worker
            cvar.notify_one();
            return true;
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
        lock_util::lock_or_recover(lock).queue.len()
    }

    /// Get thread count
    pub fn thread_count(&self) -> usize {
        self.config.threads
    }

    /// Shutdown the pool gracefully
    pub fn shutdown(&self) {
        let (lock, cvar) = &*self.state;
        let mut state = lock_util::lock_or_recover(lock);
        state.shutdown = true;
        // Wake all workers so they can exit
        cvar.notify_all();
    }

    /// Check if pool is shutdown
    pub fn is_shutdown(&self) -> bool {
        let (lock, _) = &*self.state;
        lock_util::lock_or_recover(lock).shutdown
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

        // Join all worker threads for clean shutdown
        for worker in &mut self.workers {
            worker.join();
        }
    }
}

/// A single worker thread
pub struct Worker {
    /// Worker ID
    id: usize,
    /// Thread handle for joining on shutdown
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
                        let mut state = lock_util::lock_or_recover(lock);

                        // Wait while queue is empty and not shutting down
                        while state.queue.is_empty() && !state.shutdown {
                            // Condvar::wait can also return poisoned, handle it
                            state = cvar.wait(state).unwrap_or_else(|poisoned| {
                                log_warn!("Condvar wait was poisoned, recovering");
                                poisoned.into_inner()
                            });
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
        *lock_util::lock_or_recover(&self.active)
    }

    /// Join the worker thread, waiting for it to finish
    pub fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// Task queue for worker pool
struct TaskQueue {
    /// Tasks waiting to be executed (using BinaryHeap for O(log n) operations)
    tasks: BinaryHeap<QueuedTask>,
    /// Maximum capacity
    capacity: usize,
    /// Next sequence number (for FIFO ordering within same priority)
    next_seq: u64,
}

impl TaskQueue {
    fn new(capacity: usize) -> Self {
        Self {
            tasks: BinaryHeap::with_capacity(capacity),
            capacity,
            next_seq: 0,
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

        self.tasks.push(task);
        true
    }

    fn pop(&mut self) -> Option<QueuedTask> {
        self.tasks.pop()
    }
}

/// A queued task with priority and sequence number for FIFO ordering
struct QueuedTask {
    /// Task to execute
    task: Box<dyn FnOnce() + Send + 'static>,
    /// Priority (higher values = higher priority)
    priority: Priority,
    /// Sequence number for FIFO ordering within same priority
    seq: u64,
}

impl QueuedTask {
    fn new<F>(task: F, priority: Priority, seq: u64) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            task: Box::new(task),
            priority,
            seq,
        }
    }
}

// Implement Ord for BinaryHeap
// BinaryHeap is a max-heap, so we want higher priority tasks to compare as "greater"
// For same priority, lower sequence number (earlier) should come first (FIFO)
impl PartialEq for QueuedTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.seq == other.seq
    }
}

impl Eq for QueuedTask {}

impl PartialOrd for QueuedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by priority (higher priority = greater)
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => {
                // For same priority, lower sequence number (earlier) should come first
                // Reverse the seq comparison so lower seq = "greater" in max-heap
                other.seq.cmp(&self.seq)
            }
            other => other,
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
        use std::sync::atomic::{AtomicBool, Ordering};

        let pool = WorkerPool::new(1);
        let order = Arc::new(Mutex::new(Vec::new()));

        // Use a barrier to hold the worker while we queue both tasks
        let barrier = Arc::new(AtomicBool::new(false));
        let barrier_clone = barrier.clone();

        // Submit a blocking task first to hold the worker
        pool.submit(move || {
            while !barrier_clone.load(Ordering::SeqCst) {
                thread::sleep(std::time::Duration::from_millis(1));
            }
        });

        // Give the worker time to pick up the blocking task
        thread::sleep(std::time::Duration::from_millis(10));

        // Now submit low and high priority tasks - they'll queue up
        let order1 = order.clone();
        pool.submit_with_priority(
            move || {
                order1.lock().unwrap().push("low");
            },
            Priority::Low,
        );

        let order2 = order.clone();
        pool.submit_with_priority(
            move || {
                order2.lock().unwrap().push("high");
            },
            Priority::High,
        );

        // Release the barrier - worker will process queued tasks by priority
        barrier.store(true, Ordering::SeqCst);

        // Wait for completion
        thread::sleep(std::time::Duration::from_millis(100));
        pool.shutdown();

        let result = order.lock().unwrap();
        // High priority should be processed first
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "high");
        assert_eq!(result[1], "low");
    }
}
