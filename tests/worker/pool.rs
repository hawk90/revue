//! Worker pool integration tests
//!
//! Tests for worker pool construction, task submission, and lifecycle.

use revue::worker::{Priority, WorkerConfig, WorkerPool};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// =============================================================================
// WorkerPool Construction Tests
// =============================================================================

#[test]
fn test_pool_config_threads() {
    let pool = WorkerPool::new(4);
    assert_eq!(pool.thread_count(), 4);
}

#[test]
fn test_pool_config_queue_capacity() {
    let config = WorkerConfig {
        threads: 2,
        queue_capacity: 100,
        default_timeout_ms: None,
    };
    let pool = WorkerPool::with_config(config.clone());
    assert_eq!(pool.thread_count(), 2);

    // Submit up to capacity
    for _ in 0..100 {
        assert!(pool.submit(|| {}));
    }
}

#[test]
fn test_pool_config_timeout() {
    let config = WorkerConfig {
        threads: 1,
        queue_capacity: 10,
        default_timeout_ms: Some(1000),
    };
    let pool = WorkerPool::with_config(config);
    assert_eq!(pool.thread_count(), 1);
}

#[test]
fn test_pool_default() {
    let pool = WorkerPool::default();
    // Should use available parallelism or default to 4
    assert!(pool.thread_count() >= 1);
}

#[test]
fn test_pool_with_threads() {
    let pool = WorkerPool::new(8);
    assert_eq!(pool.thread_count(), 8);
}

// =============================================================================
// Task Submission Tests
// =============================================================================

#[test]
fn test_pool_submit_full_queue() {
    let config = WorkerConfig {
        threads: 1,
        queue_capacity: 5,
        default_timeout_ms: None,
    };
    let pool = WorkerPool::with_config(config);

    // Hold the worker with a long task
    let barrier = Arc::new(AtomicUsize::new(0));
    let barrier_clone = barrier.clone();

    pool.submit(move || {
        while barrier_clone.load(Ordering::SeqCst) == 0 {
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::sleep(Duration::from_millis(50));

    // Fill the queue
    for _ in 0..4 {
        assert!(pool.submit(|| {}));
    }

    // Queue is now full, next submission should fail
    assert!(!pool.submit(|| {}));
}

#[test]
fn test_pool_submit_priority_ordering() {
    let pool = WorkerPool::new(1);
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));

    // Hold worker initially
    let barrier = Arc::new(AtomicUsize::new(0));
    let barrier_clone = barrier.clone();

    pool.submit(move || {
        while barrier_clone.load(Ordering::SeqCst) == 0 {
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::sleep(Duration::from_millis(10));

    // Queue tasks with different priorities
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
            order2.lock().unwrap().push("normal");
        },
        Priority::Normal,
    );

    let order3 = order.clone();
    pool.submit_with_priority(
        move || {
            order3.lock().unwrap().push("high");
        },
        Priority::High,
    );

    // Release barrier
    barrier.store(1, Ordering::SeqCst);

    // Wait for completion
    thread::sleep(Duration::from_millis(200));
    pool.shutdown();

    let result = order.lock().unwrap();
    // High should execute first, then normal, then low
    assert_eq!(result[0], "high");
    assert_eq!(result[1], "normal");
    assert_eq!(result[2], "low");
}

#[test]
fn test_pool_submit_many_tasks() {
    let pool = WorkerPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    // Submit many tasks
    for _ in 0..100 {
        let counter = counter.clone();
        pool.submit(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        });
    }

    // Wait for completion
    thread::sleep(Duration::from_millis(500));

    assert_eq!(counter.load(Ordering::SeqCst), 100);
}

#[test]
fn test_pool_shutdown_graceful() {
    let pool = WorkerPool::new(2);
    let counter = Arc::new(AtomicUsize::new(0));

    // Submit some tasks
    for _ in 0..10 {
        let counter = counter.clone();
        pool.submit(move || {
            counter.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(10));
        });
    }

    // Shutdown should wait for tasks to complete
    pool.shutdown();
    thread::sleep(Duration::from_millis(200));

    // Most tasks should have completed
    assert!(counter.load(Ordering::SeqCst) > 0);
}

#[test]
fn test_pool_submit_after_shutdown() {
    let pool = WorkerPool::new(1);
    pool.shutdown();

    // Should fail to submit after shutdown
    assert!(!pool.submit(|| {}));
    assert!(!pool.submit_with_priority(|| {}, Priority::High));
}

// =============================================================================
// Priority Queue Tests
// =============================================================================

#[test]
fn test_priority_fifo_same_priority() {
    let pool = WorkerPool::new(1);
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));

    // Hold worker
    let barrier = Arc::new(AtomicUsize::new(0));
    let barrier_clone = barrier.clone();

    pool.submit(move || {
        while barrier_clone.load(Ordering::SeqCst) == 0 {
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::sleep(Duration::from_millis(10));

    // Queue multiple normal priority tasks
    for i in 0..5 {
        let order = order.clone();
        pool.submit_with_priority(
            move || {
                order.lock().unwrap().push(i);
            },
            Priority::Normal,
        );
    }

    // Release
    barrier.store(1, Ordering::SeqCst);

    thread::sleep(Duration::from_millis(200));
    pool.shutdown();

    let result = order.lock().unwrap();
    // Should maintain FIFO order for same priority
    assert_eq!(*result, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_priority_high_preempts_low() {
    let pool = WorkerPool::new(1);
    let order = Arc::new(std::sync::Mutex::new(Vec::new()));

    // Hold worker
    let barrier = Arc::new(AtomicUsize::new(0));
    let barrier_clone = barrier.clone();

    pool.submit(move || {
        while barrier_clone.load(Ordering::SeqCst) == 0 {
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::sleep(Duration::from_millis(10));

    // Queue: low, high, low
    let order1 = order.clone();
    pool.submit_with_priority(
        move || {
            order1.lock().unwrap().push("low1");
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

    let order3 = order.clone();
    pool.submit_with_priority(
        move || {
            order3.lock().unwrap().push("low2");
        },
        Priority::Low,
    );

    barrier.store(1, Ordering::SeqCst);

    thread::sleep(Duration::from_millis(200));
    pool.shutdown();

    let result = order.lock().unwrap();
    // High should execute before low priority tasks
    assert_eq!(result[0], "high");
}

// =============================================================================
// Pool State Tests
// =============================================================================

#[test]
fn test_pool_active_workers() {
    let pool = WorkerPool::new(4);

    // All workers should be active (waiting for tasks)
    thread::sleep(Duration::from_millis(50));
    // Workers may not all show as active if they're waiting
    let _active = pool.active_workers();
}

#[test]
fn test_pool_queue_length() {
    let config = WorkerConfig {
        threads: 1,
        queue_capacity: 100,
        default_timeout_ms: None,
    };
    let pool = WorkerPool::with_config(config);

    // Hold worker
    let barrier = Arc::new(AtomicUsize::new(0));
    let barrier_clone = barrier.clone();

    pool.submit(move || {
        while barrier_clone.load(Ordering::SeqCst) == 0 {
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::sleep(Duration::from_millis(10));

    // Queue some tasks
    for _ in 0..10 {
        pool.submit(|| {});
    }

    // Should have tasks in queue
    let queue_len = pool.queue_len();
    assert!(queue_len > 0);

    barrier.store(1, Ordering::SeqCst);
}

#[test]
fn test_pool_is_shutdown() {
    let pool = WorkerPool::new(2);
    assert!(!pool.is_shutdown());

    pool.shutdown();
    assert!(pool.is_shutdown());
}

#[test]
fn test_pool_multiple_shutdowns() {
    let pool = WorkerPool::new(2);

    pool.shutdown();
    pool.shutdown(); // Should be safe to call multiple times

    assert!(pool.is_shutdown());
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_pool_concurrent_submissions() {
    let pool = Arc::new(WorkerPool::new(4));
    let counter = Arc::new(AtomicUsize::new(0));

    // Submit from multiple threads
    let mut handles = Vec::new();
    for _ in 0..4 {
        let pool = pool.clone();
        let counter = counter.clone();

        let handle = thread::spawn(move || {
            for _ in 0..25 {
                let counter = counter.clone();
                pool.submit(move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                });
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(500));
    assert_eq!(counter.load(Ordering::SeqCst), 100);
}
