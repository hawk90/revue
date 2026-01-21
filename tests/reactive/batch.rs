//! Batch tests

#![allow(unused_imports)]

use revue::reactive::*;
use serial_test::serial;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[serial]
#[test]
fn test_batch_basic() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    batch(|| {
        queue_update(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[serial]
#[test]
fn test_batch_nested() {
    let counter = Arc::new(AtomicUsize::new(0));

    batch(|| {
        let c1 = counter.clone();
        queue_update(move || {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        batch(|| {
            let c2 = counter.clone();
            queue_update(move || {
                c2.fetch_add(1, Ordering::SeqCst);
            });
        });

        assert_eq!(counter.load(Ordering::SeqCst), 0);
    });

    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[serial]
#[test]
fn test_is_batching() {
    assert!(!is_batching());

    batch(|| {
        assert!(is_batching());
    });

    assert!(!is_batching());
}

#[serial]
#[test]
fn test_batch_guard() {
    assert!(!is_batching());

    {
        let _guard = BatchGuard::new();
        assert!(is_batching());
    }

    assert!(!is_batching());
}

#[serial]
#[test]
fn test_transaction_commit() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut tx = Transaction::new();
    let c1 = counter.clone();
    tx.update(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });
    let c2 = counter.clone();
    tx.update(move || {
        c2.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(tx.len(), 2);
    assert_eq!(counter.load(Ordering::SeqCst), 0);

    tx.commit();
    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[serial]
#[test]
fn test_transaction_rollback() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut tx = Transaction::new();
    let c1 = counter.clone();
    tx.update(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    tx.rollback();
    assert_eq!(counter.load(Ordering::SeqCst), 0);
}

#[serial]
#[test]
fn test_flush() {
    let counter = Arc::new(AtomicUsize::new(0));

    start_batch();

    let c1 = counter.clone();
    queue_update(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    flush();
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    end_batch();
}

#[serial]
#[test]
fn test_queue_update_no_batch() {
    let counter = Arc::new(AtomicUsize::new(0));
    let c1 = counter.clone();

    queue_update(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[serial]
#[test]
fn test_batch_depth() {
    assert_eq!(batch_depth(), 0);

    start_batch();
    assert_eq!(batch_depth(), 1);

    start_batch();
    assert_eq!(batch_depth(), 2);

    end_batch();
    assert_eq!(batch_depth(), 1);

    end_batch();
    assert_eq!(batch_depth(), 0);
}

#[serial]
#[test]
fn test_batch_count() {
    let initial = batch_count();

    batch(|| {});
    assert!(batch_count() > initial);

    batch(|| {
        batch(|| {});
    });
    assert!(batch_count() > initial + 1);
}

#[serial]
#[test]
fn test_pending_count() {
    assert_eq!(pending_count(), 0);

    start_batch();

    queue_update(|| {});
    assert_eq!(pending_count(), 1);

    queue_update(|| {});
    assert_eq!(pending_count(), 2);

    end_batch();
    assert_eq!(pending_count(), 0);
}

#[serial]
#[test]
fn test_transaction_is_empty() {
    let tx = Transaction::new();
    assert!(tx.is_empty());
    assert_eq!(tx.len(), 0);
}

#[serial]
#[test]
fn test_transaction_len() {
    let mut tx = Transaction::new();
    assert_eq!(tx.len(), 0);

    tx.update(|| {});
    assert_eq!(tx.len(), 1);

    tx.update(|| {});
    assert_eq!(tx.len(), 2);
}

#[serial]
#[test]
fn test_batch_return_value() {
    let result = batch(|| 42);
    assert_eq!(result, 42);

    let result = batch(|| "hello".to_string());
    assert_eq!(result, "hello");
}

#[serial]
#[test]
fn test_multiple_flushes() {
    let counter = Arc::new(AtomicUsize::new(0));

    start_batch();

    let c1 = counter.clone();
    queue_update(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    flush();
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    let c2 = counter.clone();
    queue_update(move || {
        c2.fetch_add(1, Ordering::SeqCst);
    });

    flush();
    assert_eq!(counter.load(Ordering::SeqCst), 2);

    end_batch();
}

#[serial]
#[test]
fn test_batch_guard_drop() {
    let counter = Arc::new(AtomicUsize::new(0));

    {
        let _guard = BatchGuard::new();
        let c1 = counter.clone();
        queue_update(move || {
            c1.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[serial]
#[test]
fn test_nested_batch_guard() {
    let counter = Arc::new(AtomicUsize::new(0));

    {
        let _outer = BatchGuard::new();
        assert!(is_batching());

        {
            let _inner = BatchGuard::new();
            assert!(is_batching());

            let c1 = counter.clone();
            queue_update(move || {
                c1.fetch_add(1, Ordering::SeqCst);
            });
        }

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        assert!(is_batching());
    }

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert!(!is_batching());
}

#[serial]
#[test]
fn test_transaction_default() {
    let tx = Transaction::default();
    assert!(tx.is_empty());
}

#[serial]
#[test]
fn test_end_batch_without_start() {
    let depth_before = batch_depth();
    end_batch();
    let depth_after = batch_depth();

    assert!(depth_after <= depth_before);
}

#[serial]
#[test]
fn test_independent_thread_batches() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let c = counter.clone();
        handles.push(std::thread::spawn(move || {
            batch(|| {
                assert!(is_batching());
                queue_update(move || {
                    c.fetch_add(1, Ordering::SeqCst);
                });
                assert!(is_batching());
            });
            assert!(!is_batching());
        }));
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    assert_eq!(counter.load(Ordering::SeqCst), 4);
}
