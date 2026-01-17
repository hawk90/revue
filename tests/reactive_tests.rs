//! Integration tests for the reactive module
//!
//! Tests migrated from src/reactive/*.rs inline test modules.
//! Tests that access private items remain inline in source files.

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Signal Tests (from src/reactive/signal.rs)
// ============================================================================

#[test]
fn test_signal_get_set() {
    let count = Signal::new(0);
    assert_eq!(count.get(), 0);

    count.set(5);
    assert_eq!(count.get(), 5);
}

#[test]
fn test_signal_update() {
    let count = Signal::new(10);
    count.update(|n| *n += 5);
    assert_eq!(count.get(), 15);

    count.update(|n| *n *= 2);
    assert_eq!(count.get(), 30);
}

#[test]
fn test_signal_subscribe() {
    let count = Signal::new(0);
    let called = Arc::new(AtomicUsize::new(0));

    let called_clone = called.clone();
    let _sub = count.subscribe(move || {
        called_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(called.load(Ordering::SeqCst), 0);

    count.set(1);
    assert_eq!(called.load(Ordering::SeqCst), 1);

    count.set(2);
    assert_eq!(called.load(Ordering::SeqCst), 2);
}

#[test]
fn test_signal_unsubscribe_on_drop() {
    let count = Signal::new(0);
    let called = Arc::new(AtomicUsize::new(0));

    let called_clone = called.clone();
    let sub = count.subscribe(move || {
        called_clone.fetch_add(1, Ordering::SeqCst);
    });

    count.set(1);
    assert_eq!(called.load(Ordering::SeqCst), 1);

    drop(sub);

    count.set(2);
    assert_eq!(called.load(Ordering::SeqCst), 1);
}

#[test]
fn test_signal_multiple_subscriptions() {
    let count = Signal::new(0);
    let called1 = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::new(AtomicUsize::new(0));

    let c1 = called1.clone();
    let sub1 = count.subscribe(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    let c2 = called2.clone();
    let _sub2 = count.subscribe(move || {
        c2.fetch_add(1, Ordering::SeqCst);
    });

    count.set(1);
    assert_eq!(called1.load(Ordering::SeqCst), 1);
    assert_eq!(called2.load(Ordering::SeqCst), 1);

    drop(sub1);

    count.set(2);
    assert_eq!(called1.load(Ordering::SeqCst), 1);
    assert_eq!(called2.load(Ordering::SeqCst), 2);
}

#[test]
fn test_signal_clone_shares_value() {
    let count = Signal::new(0);
    let count2 = count.clone();

    count.set(42);
    assert_eq!(count2.get(), 42);

    count2.set(100);
    assert_eq!(count.get(), 100);
}

#[test]
fn test_signal_with_string() {
    let name = Signal::new(String::from("hello"));
    assert_eq!(name.get(), "hello");

    name.set(String::from("world"));
    assert_eq!(name.get(), "world");

    name.update(|s| s.push_str("!"));
    assert_eq!(name.get(), "world!");
}

#[test]
fn test_signal_with_vec() {
    let items = Signal::new(vec![1, 2, 3]);
    assert_eq!(items.get(), vec![1, 2, 3]);

    items.update(|v| v.push(4));
    assert_eq!(items.get(), vec![1, 2, 3, 4]);
}

#[test]
fn test_signal_unique_ids() {
    let s1 = Signal::new(1);
    let s2 = Signal::new(2);
    assert_ne!(s1.id(), s2.id());
}

#[test]
fn test_signal_read_zero_copy() {
    let items = Signal::new(vec![1, 2, 3]);
    assert_eq!(items.read().len(), 3);
    assert_eq!(items.read()[0], 1);
}

#[test]
fn test_signal_borrow_zero_copy() {
    let items = Signal::new(vec![1, 2, 3]);
    assert_eq!(items.borrow().len(), 3);
    assert_eq!(items.borrow()[0], 1);
}

#[test]
fn test_signal_with_zero_copy() {
    let items = Signal::new(vec![1, 2, 3]);
    let len = items.with(|v| v.len());
    assert_eq!(len, 3);

    let sum: i32 = items.with(|v| v.iter().sum());
    assert_eq!(sum, 6);
}

#[test]
fn test_signal_thread_safety() {
    let count = Signal::new(0);
    let count_clone = count.clone();

    let handle = std::thread::spawn(move || {
        for _ in 0..100 {
            count_clone.update(|n| *n += 1);
        }
    });

    for _ in 0..100 {
        count.update(|n| *n += 1);
    }

    handle.join().unwrap();
    assert_eq!(count.get(), 200);
}

#[test]
fn test_signal_cross_thread_subscribe() {
    let count = Signal::new(0);
    let notified = Arc::new(AtomicUsize::new(0));

    let notified_clone = notified.clone();
    let _sub = count.subscribe(move || {
        notified_clone.fetch_add(1, Ordering::SeqCst);
    });

    let count_clone = count.clone();
    let handle = std::thread::spawn(move || {
        count_clone.set(42);
    });

    handle.join().unwrap();

    assert_eq!(count.get(), 42);
    assert_eq!(notified.load(Ordering::SeqCst), 1);
}

#[test]
fn test_signal_debug() {
    let sig = Signal::new(42);
    let debug_str = format!("{:?}", sig);
    assert!(debug_str.contains("Signal"));
    assert!(debug_str.contains("42"));
}

// ============================================================================
// Computed Tests (from src/reactive/computed.rs)
// ============================================================================

#[test]
fn test_computed_basic() {
    let computed = Computed::new(|| 42);
    assert_eq!(computed.get(), 42);
}

#[test]
fn test_computed_with_closure() {
    let multiplier = 3;
    let computed = Computed::new(move || 10 * multiplier);
    assert_eq!(computed.get(), 30);
}

#[test]
fn test_computed_caching() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let computed = Computed::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        42
    });

    assert_eq!(computed.get(), 42);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    assert_eq!(computed.get(), 42);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    computed.invalidate();
    assert_eq!(computed.get(), 42);
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_computed_dirty_flag() {
    let computed = Computed::new(|| 1);

    assert!(computed.is_dirty());

    computed.get();
    assert!(!computed.is_dirty());

    computed.invalidate();
    assert!(computed.is_dirty());
}

#[test]
fn test_computed_auto_invalidation() {
    let source = signal(10);
    let compute_count = Arc::new(AtomicUsize::new(0));

    let cc = compute_count.clone();
    let s = source.clone();
    let computed = Computed::new(move || {
        cc.fetch_add(1, Ordering::SeqCst);
        s.get() * 2
    });

    assert_eq!(computed.get(), 20);
    assert_eq!(compute_count.load(Ordering::SeqCst), 1);

    assert_eq!(computed.get(), 20);
    assert_eq!(computed.get(), 20);
    assert_eq!(compute_count.load(Ordering::SeqCst), 1);

    source.set(20);

    assert!(computed.is_dirty());

    assert_eq!(computed.get(), 40);
    assert_eq!(compute_count.load(Ordering::SeqCst), 2);

    assert_eq!(computed.get(), 40);
    assert_eq!(compute_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_computed_dynamic_dependencies() {
    let flag = signal(true);
    let a = signal(1);
    let b = signal(2);

    let f = flag.clone();
    let a_c = a.clone();
    let b_c = b.clone();

    let computed = Computed::new(move || if f.get() { a_c.get() } else { b_c.get() });

    assert_eq!(computed.get(), 1);

    a.set(10);
    assert_eq!(computed.get(), 10);

    flag.set(false);
    assert_eq!(computed.get(), 2);

    b.set(20);
    assert_eq!(computed.get(), 20);

    a.set(100);
    assert_eq!(computed.get(), 20);
}

#[test]
fn test_computed_thread_safety() {
    let computed = Arc::new(Computed::new(|| 42));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = computed.clone();
            std::thread::spawn(move || {
                for _ in 0..100 {
                    assert_eq!(c.get(), 42);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_computed_no_data_race_on_concurrent_recompute() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let computed = Arc::new(Computed::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(Duration::from_micros(100));
        42
    }));

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = computed.clone();
            std::thread::spawn(move || c.get())
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert!(results.iter().all(|&v| v == 42));
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "Computation should run exactly once, but ran {} times",
        call_count.load(Ordering::SeqCst)
    );
}

#[test]
fn test_computed_recomputes_after_invalidation_with_contention() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let computed = Arc::new(Computed::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst)
    }));

    let v1 = computed.get();
    assert_eq!(v1, 0);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    for _ in 0..10 {
        assert_eq!(computed.get(), 0);
    }
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    computed.invalidate();

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = computed.clone();
            std::thread::spawn(move || c.get())
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert!(results.iter().all(|&v| v == 1));
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

// ============================================================================
// Effect Tests (from src/reactive/effect.rs)
// ============================================================================

#[test]
fn test_effect_runs_immediately() {
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let _effect = Effect::new(move || {
        called_clone.store(true, Ordering::SeqCst);
    });

    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_effect_lazy_does_not_run() {
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let effect = Effect::lazy(move || {
        called_clone.store(true, Ordering::SeqCst);
    });

    assert!(!called.load(Ordering::SeqCst));

    effect.run();
    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_effect_stop_and_resume() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();

    let effect = Effect::lazy(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    effect.run();
    assert_eq!(count.load(Ordering::SeqCst), 1);

    effect.stop();
    effect.run();
    assert_eq!(count.load(Ordering::SeqCst), 1);

    effect.resume();
    effect.run();
    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_effect_is_active() {
    let effect = Effect::lazy(|| {});

    assert!(effect.is_active());

    effect.stop();
    assert!(!effect.is_active());

    effect.resume();
    assert!(effect.is_active());
}

#[test]
fn test_effect_dynamic_dependency_retracking() {
    let flag = signal(true);
    let a = signal(1);
    let b = signal(2);
    let result = Arc::new(AtomicUsize::new(0));

    let res = result.clone();
    let f = flag.clone();
    let a_c = a.clone();
    let b_c = b.clone();

    let _effect = Effect::new(move || {
        let val = if f.get() { a_c.get() } else { b_c.get() };
        res.store(val as usize, Ordering::SeqCst);
    });

    assert_eq!(result.load(Ordering::SeqCst), 1);

    a.set(10);
    assert_eq!(result.load(Ordering::SeqCst), 10);

    flag.set(false);
    assert_eq!(result.load(Ordering::SeqCst), 2);

    b.set(20);
    assert_eq!(result.load(Ordering::SeqCst), 20);

    a.set(100);
    assert_eq!(result.load(Ordering::SeqCst), 20);
}

// ============================================================================
// Batch Tests (from src/reactive/batch.rs)
// ============================================================================

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

#[test]
fn test_is_batching() {
    assert!(!is_batching());

    batch(|| {
        assert!(is_batching());
    });

    assert!(!is_batching());
}

#[test]
fn test_batch_guard() {
    assert!(!is_batching());

    {
        let _guard = BatchGuard::new();
        assert!(is_batching());
    }

    assert!(!is_batching());
}

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

#[test]
fn test_queue_update_no_batch() {
    let counter = Arc::new(AtomicUsize::new(0));
    let c1 = counter.clone();

    queue_update(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

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

#[test]
fn test_transaction_is_empty() {
    let tx = Transaction::new();
    assert!(tx.is_empty());
    assert_eq!(tx.len(), 0);
}

#[test]
fn test_transaction_len() {
    let mut tx = Transaction::new();
    assert_eq!(tx.len(), 0);

    tx.update(|| {});
    assert_eq!(tx.len(), 1);

    tx.update(|| {});
    assert_eq!(tx.len(), 2);
}

#[test]
fn test_batch_return_value() {
    let result = batch(|| 42);
    assert_eq!(result, 42);

    let result = batch(|| "hello".to_string());
    assert_eq!(result, "hello");
}

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

#[test]
fn test_transaction_default() {
    let tx = Transaction::default();
    assert!(tx.is_empty());
}

#[test]
fn test_end_batch_without_start() {
    let depth_before = batch_depth();
    end_batch();
    let depth_after = batch_depth();

    assert!(depth_after <= depth_before);
}

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

// ============================================================================
// Context Tests (from src/reactive/context.rs)
// ============================================================================

#[test]
fn test_create_context() {
    let ctx: Context<String> = create_context();
    assert!(ctx.default().is_none());
}

#[test]
fn test_create_context_with_default() {
    let ctx = create_context_with_default("default_value".to_string());
    assert_eq!(ctx.default(), Some(&"default_value".to_string()));
}

#[test]
fn test_provide_and_use_context() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    provide(&theme, "dark".to_string());

    let value = use_context(&theme);
    assert_eq!(value, Some("dark".to_string()));

    clear_all_contexts();
}

#[test]
fn test_use_context_default() {
    clear_all_contexts();

    let ctx = create_context_with_default(42);

    let value = use_context(&ctx);
    assert_eq!(value, Some(42));

    provide(&ctx, 100);
    let value = use_context(&ctx);
    assert_eq!(value, Some(100));

    clear_all_contexts();
}

#[test]
fn test_use_context_no_provider_no_default() {
    clear_all_contexts();

    let ctx: Context<String> = create_context();
    let value = use_context(&ctx);
    assert_eq!(value, None);

    clear_all_contexts();
}

#[test]
fn test_provide_signal_reactive() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    let signal = provide_signal(&theme, "dark".to_string());

    assert_eq!(use_context(&theme), Some("dark".to_string()));

    signal.set("light".to_string());
    assert_eq!(use_context(&theme), Some("light".to_string()));

    clear_all_contexts();
}

#[test]
fn test_use_context_signal() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    provide(&theme, "dark".to_string());

    let signal = use_context_signal(&theme);
    assert!(signal.is_some());
    assert_eq!(signal.unwrap().get(), "dark");

    clear_all_contexts();
}

#[test]
fn test_has_context() {
    clear_all_contexts();

    let ctx: Context<i32> = create_context();
    assert!(!has_context(&ctx));

    provide(&ctx, 42);
    assert!(has_context(&ctx));

    clear_context(&ctx);
    assert!(!has_context(&ctx));

    clear_all_contexts();
}

#[test]
fn test_clear_context() {
    clear_all_contexts();

    let ctx: Context<String> = create_context();
    provide(&ctx, "value".to_string());
    assert!(has_context(&ctx));

    clear_context(&ctx);
    assert!(!has_context(&ctx));
    assert_eq!(use_context(&ctx), None);

    clear_all_contexts();
}

#[test]
fn test_context_scope() {
    clear_all_contexts();

    let theme: Context<String> = create_context();

    assert_eq!(use_context(&theme), None);

    {
        let scope = ContextScope::new();
        scope.provide(&theme, "scoped_dark".to_string());

        assert_eq!(use_context(&theme), Some("scoped_dark".to_string()));
    }

    assert_eq!(use_context(&theme), None);

    clear_all_contexts();
}

#[test]
fn test_nested_context_scopes() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    provide(&theme, "global".to_string());

    assert_eq!(use_context(&theme), Some("global".to_string()));

    {
        let scope1 = ContextScope::new();
        scope1.provide(&theme, "scope1".to_string());

        assert_eq!(use_context(&theme), Some("scope1".to_string()));

        {
            let scope2 = ContextScope::new();
            scope2.provide(&theme, "scope2".to_string());

            assert_eq!(use_context(&theme), Some("scope2".to_string()));
        }

        assert_eq!(use_context(&theme), Some("scope1".to_string()));
    }

    assert_eq!(use_context(&theme), Some("global".to_string()));

    clear_all_contexts();
}

#[test]
fn test_with_context_scope() {
    clear_all_contexts();

    let count: Context<i32> = create_context();

    let result = with_context_scope(|scope| {
        scope.provide(&count, 42);
        use_context(&count).unwrap_or(0)
    });

    assert_eq!(result, 42);

    assert_eq!(use_context(&count), None);

    clear_all_contexts();
}

#[test]
fn test_multiple_contexts() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    let locale: Context<String> = create_context();
    let count: Context<i32> = create_context();

    provide(&theme, "dark".to_string());
    provide(&locale, "en-US".to_string());
    provide(&count, 100);

    assert_eq!(use_context(&theme), Some("dark".to_string()));
    assert_eq!(use_context(&locale), Some("en-US".to_string()));
    assert_eq!(use_context(&count), Some(100));

    clear_all_contexts();
}

#[test]
fn test_context_id_uniqueness() {
    let ctx1: Context<i32> = create_context();
    let ctx2: Context<i32> = create_context();

    assert_ne!(ctx1.id(), ctx2.id());
}

#[test]
fn test_provider_struct() {
    let ctx: Context<String> = create_context();
    let provider = Provider::new(&ctx, "initial".to_string());

    assert_eq!(provider.get(), "initial");

    provider.set("updated".to_string());
    assert_eq!(provider.get(), "updated");

    provider.update(|s| s.push_str("!"));
    assert_eq!(provider.get(), "updated!");
}

#[test]
fn test_context_clone() {
    let ctx = create_context_with_default(42);
    let ctx_clone = ctx.clone();

    assert_eq!(ctx.id(), ctx_clone.id());
    assert_eq!(ctx.default(), ctx_clone.default());
}

// ============================================================================
// Async State Tests (from src/reactive/async_state.rs)
// ============================================================================

#[test]
fn test_async_state_variants() {
    let idle: AsyncState<i32> = AsyncState::Idle;
    assert!(idle.is_idle());
    assert!(!idle.is_loading());
    assert!(!idle.is_ready());
    assert!(!idle.is_error());

    let loading: AsyncState<i32> = AsyncState::Loading;
    assert!(loading.is_loading());

    let ready: AsyncState<i32> = AsyncState::Ready(42);
    assert!(ready.is_ready());
    assert_eq!(ready.value(), Some(&42));

    let error: AsyncState<i32> = AsyncState::Error("failed".to_string());
    assert!(error.is_error());
    assert_eq!(error.error(), Some("failed"));
}

#[test]
fn test_async_state_map() {
    let ready: AsyncState<i32> = AsyncState::Ready(10);
    let mapped = ready.map(|v| v * 2);
    assert_eq!(mapped, AsyncState::Ready(20));

    let loading: AsyncState<i32> = AsyncState::Loading;
    let mapped_loading = loading.map(|v| v * 2);
    assert!(mapped_loading.is_loading());
}

#[test]
fn test_async_state_unwrap_or() {
    let ready: AsyncState<i32> = AsyncState::Ready(42);
    assert_eq!(ready.unwrap_or(0), 42);

    let loading: AsyncState<i32> = AsyncState::Loading;
    assert_eq!(loading.unwrap_or(0), 0);
}

#[test]
fn test_use_async() {
    let (state, trigger) = use_async(|| {
        std::thread::sleep(Duration::from_millis(10));
        Ok(42)
    });

    assert!(state.get().is_idle());

    trigger();

    for _ in 0..100 {
        if state.get().is_ready() {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    assert_eq!(state.get(), AsyncState::Ready(42));
}

#[test]
fn test_use_async_error() {
    let (state, trigger) = use_async::<i32, _>(|| Err("Something went wrong".to_string()));

    trigger();

    for _ in 0..100 {
        if state.get().is_error() {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    assert!(state.get().is_error());
    assert_eq!(state.get().error(), Some("Something went wrong"));
}

#[test]
fn test_use_async_poll() {
    let (state, start, poll) = use_async_poll(|| {
        std::thread::sleep(Duration::from_millis(10));
        Ok("done".to_string())
    });

    assert!(state.get().is_idle());

    start();
    assert!(state.get().is_loading());

    for _ in 0..20 {
        if poll() {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }

    assert_eq!(state.get(), AsyncState::Ready("done".to_string()));
}

#[test]
fn test_use_async_immediate() {
    let state = use_async_immediate(|| {
        std::thread::sleep(Duration::from_millis(5));
        Ok(100)
    });

    assert!(state.get().is_loading());

    for _ in 0..100 {
        if state.get() == AsyncState::Ready(100) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    assert_eq!(state.get(), AsyncState::Ready(100));
}

#[test]
fn test_async_state_display() {
    let idle: AsyncState<i32> = AsyncState::Idle;
    assert_eq!(format!("{}", idle), "Idle");

    let loading: AsyncState<i32> = AsyncState::Loading;
    assert_eq!(format!("{}", loading), "Loading");

    let ready: AsyncState<i32> = AsyncState::Ready(42);
    assert_eq!(format!("{}", ready), "Ready(42)");

    let error: AsyncState<i32> = AsyncState::Error("fail".to_string());
    assert_eq!(format!("{}", error), "Error(fail)");
}

#[test]
fn test_use_async_multiple_triggers() {
    let (state, trigger) = use_async(|| {
        std::thread::sleep(Duration::from_millis(5));
        Ok(42)
    });

    trigger();
    assert!(state.get().is_loading());

    trigger();
    assert!(state.get().is_loading());

    for _ in 0..100 {
        if state.get() == AsyncState::Ready(42) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    assert_eq!(state.get(), AsyncState::Ready(42));
}

#[test]
fn test_use_async_poll_cross_thread() {
    let (state, start, poll) = use_async_poll(|| {
        std::thread::sleep(Duration::from_millis(10));
        Ok(42)
    });

    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    start();
    assert!(state.get().is_loading());

    let poll_thread = std::thread::spawn(move || {
        for _ in 0..50 {
            if poll() {
                completed_clone.store(true, Ordering::SeqCst);
                return true;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        false
    });

    let result = poll_thread.join().expect("poll thread should not panic");
    assert!(result, "polling should complete successfully");
    assert!(completed.load(Ordering::SeqCst));
    assert_eq!(state.get(), AsyncState::Ready(42));
}

#[test]
fn test_use_async_poll_start_from_different_thread() {
    let (state, start, poll) = use_async_poll(|| {
        std::thread::sleep(Duration::from_millis(10));
        Ok("cross-thread".to_string())
    });

    let start_thread = std::thread::spawn(move || {
        start();
    });
    start_thread.join().expect("start thread should not panic");

    std::thread::sleep(Duration::from_millis(5));
    assert!(state.get().is_loading());

    for _ in 0..50 {
        if poll() {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }

    assert_eq!(state.get(), AsyncState::Ready("cross-thread".to_string()));
}

// ============================================================================
// Module Integration Tests (from src/reactive/mod.rs)
// ============================================================================

#[test]
fn test_automatic_dependency_tracking() {
    let count = signal(0);
    let run_count = Arc::new(AtomicUsize::new(0));

    let run_count_clone = run_count.clone();
    let count_clone = count.clone();
    let _effect = effect(move || {
        let _ = count_clone.get();
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    count.set(1);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);

    count.set(2);
    assert_eq!(run_count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_multiple_signals_dependency() {
    let a = signal(1);
    let b = signal(2);
    let sum = Arc::new(AtomicUsize::new(0));
    let run_count = Arc::new(AtomicUsize::new(0));

    let sum_clone = sum.clone();
    let run_count_clone = run_count.clone();
    let a_clone = a.clone();
    let b_clone = b.clone();
    let _effect = effect(move || {
        sum_clone.store((a_clone.get() + b_clone.get()) as usize, Ordering::SeqCst);
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(sum.load(Ordering::SeqCst), 3);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    a.set(10);
    assert_eq!(sum.load(Ordering::SeqCst), 12);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);

    b.set(20);
    assert_eq!(sum.load(Ordering::SeqCst), 30);
    assert_eq!(run_count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_effect_stop_removes_tracking() {
    let count = signal(0);
    let run_count = Arc::new(AtomicUsize::new(0));

    let run_count_clone = run_count.clone();
    let count_clone = count.clone();
    let effect_handle = effect(move || {
        let _ = count_clone.get();
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    effect_handle.stop();

    count.set(1);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_effect_with_borrow() {
    let items = signal(vec![1, 2, 3]);
    let sum = Arc::new(AtomicUsize::new(0));

    let sum_clone = sum.clone();
    let items_clone = items.clone();
    let _effect = effect(move || {
        let s: i32 = items_clone.borrow().iter().sum();
        sum_clone.store(s as usize, Ordering::SeqCst);
    });

    assert_eq!(sum.load(Ordering::SeqCst), 6);

    items.update(|v| v.push(4));
    assert_eq!(sum.load(Ordering::SeqCst), 10);
}

#[test]
fn test_effect_with_with() {
    let name = signal(String::from("World"));
    let greeting = Arc::new(std::sync::RwLock::new(String::new()));

    let greeting_clone = greeting.clone();
    let name_clone = name.clone();
    let _effect = effect(move || {
        let g = name_clone.with(|n| format!("Hello, {}!", n));
        *greeting_clone.write().unwrap() = g;
    });

    assert_eq!(*greeting.read().unwrap(), "Hello, World!");

    name.set(String::from("Revue"));
    assert_eq!(*greeting.read().unwrap(), "Hello, Revue!");
}

#[test]
fn test_effect_dropped_cleans_up() {
    let count = signal(0);
    let run_count = Arc::new(AtomicUsize::new(0));

    {
        let run_count_clone = run_count.clone();
        let count_clone = count.clone();
        let _effect = effect(move || {
            let _ = count_clone.get();
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(run_count.load(Ordering::SeqCst), 1);
        count.set(1);
        assert_eq!(run_count.load(Ordering::SeqCst), 2);
    }

    count.set(2);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_conditional_dependency() {
    let flag = signal(true);
    let a = signal(1);
    let b = signal(2);
    let result = Arc::new(AtomicUsize::new(0));
    let run_count = Arc::new(AtomicUsize::new(0));

    let result_clone = result.clone();
    let run_count_clone = run_count.clone();
    let flag_clone = flag.clone();
    let a_clone = a.clone();
    let b_clone = b.clone();

    let _effect = effect(move || {
        let value = if flag_clone.get() {
            a_clone.get()
        } else {
            b_clone.get()
        };
        result_clone.store(value as usize, Ordering::SeqCst);
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(result.load(Ordering::SeqCst), 1);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    a.set(10);
    assert_eq!(result.load(Ordering::SeqCst), 10);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);
}

// ============================================================================
// Runtime Tests (from src/reactive/runtime.rs)
// ============================================================================

#[test]
fn test_new_runtime() {
    let runtime = ReactiveRuntime::new();
    assert!(!runtime.has_pending());
}

#[test]
fn test_default_runtime() {
    let runtime = ReactiveRuntime::default();
    assert!(!runtime.has_pending());
}

#[test]
fn test_schedule_effect() {
    let mut runtime = ReactiveRuntime::new();
    assert!(!runtime.has_pending());

    runtime.schedule_effect(Box::new(|| {}));
    assert!(runtime.has_pending());
}

#[test]
fn test_flush_clears_dirty() {
    let mut runtime = ReactiveRuntime::new();
    runtime.mark_dirty(SignalId::new());
    runtime.mark_dirty(SignalId::new());
    assert!(runtime.has_pending());

    runtime.flush();
    assert!(!runtime.has_pending());
}

#[test]
fn test_flush_runs_effects() {
    let mut runtime = ReactiveRuntime::new();
    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = counter.clone();
    runtime.schedule_effect(Box::new(move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    }));

    let counter_clone = counter.clone();
    runtime.schedule_effect(Box::new(move || {
        counter_clone.fetch_add(10, Ordering::SeqCst);
    }));

    assert_eq!(counter.load(Ordering::SeqCst), 0);
    runtime.flush();
    assert_eq!(counter.load(Ordering::SeqCst), 11);
}

#[test]
fn test_has_pending_effects_only() {
    let mut runtime = ReactiveRuntime::new();
    runtime.schedule_effect(Box::new(|| {}));
    assert!(runtime.has_pending());
}

#[test]
fn test_has_pending_both() {
    let mut runtime = ReactiveRuntime::new();
    runtime.mark_dirty(SignalId::new());
    runtime.schedule_effect(Box::new(|| {}));
    assert!(runtime.has_pending());
}
