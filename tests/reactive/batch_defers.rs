//! `batch` defers what reacts to a change - effects and `Signal::subscribe`
//! callbacks - until the outermost batch ends, then runs each once. Values
//! themselves change immediately.

use revue::reactive::{batch, computed, effect, flush, queue_update, signal, BatchGuard};

/// An effect over two signals that records every (x, y) it sees.
fn watch_pair(
    x: &revue::reactive::Signal<i32>,
    y: &revue::reactive::Signal<i32>,
) -> (
    revue::reactive::Effect,
    revue::reactive::Signal<Vec<(i32, i32)>>,
) {
    let seen = signal(Vec::new());
    let e = effect({
        let (x, y, seen) = (x.clone(), y.clone(), seen.clone());
        move || {
            let pair = (x.get(), y.get());
            seen.update(|v| v.push(pair));
        }
    });
    (e, seen)
}

#[test]
fn an_effect_runs_once_after_the_batch_and_never_sees_half_an_update() {
    let (x, y) = (signal(0), signal(0));
    let (_e, seen) = watch_pair(&x, &y);
    assert_eq!(seen.get(), [(0, 0)]);

    batch(|| {
        x.set(1);
        y.set(2);
        assert_eq!(seen.get(), [(0, 0)], "an effect ran inside the batch");
    });
    assert_eq!(seen.get(), [(0, 0), (1, 2)]);
}

#[test]
fn values_and_computed_values_are_current_inside_the_batch() {
    let x = signal(1);
    let doubled = computed({
        let x = x.clone();
        move || x.get() * 2
    });
    batch(|| {
        x.set(5);
        assert_eq!(x.get(), 5);
        assert_eq!(doubled.get(), 10);
    });
}

#[test]
fn an_effect_reading_a_computed_runs_once_after_the_batch() {
    let x = signal(1);
    let doubled = computed({
        let x = x.clone();
        move || x.get() * 2
    });
    let seen = signal(Vec::new());
    let _e = effect({
        let (doubled, seen) = (doubled.clone(), seen.clone());
        move || seen.update(|v| v.push(doubled.get()))
    });
    batch(|| {
        x.set(2);
        x.set(3);
    });
    assert_eq!(seen.get(), [2, 6]);
}

#[test]
fn nested_batches_flush_only_at_the_outermost() {
    let (x, y) = (signal(0), signal(0));
    let (_e, seen) = watch_pair(&x, &y);
    batch(|| {
        x.set(1);
        batch(|| y.set(1));
        assert_eq!(seen.get().len(), 1, "the inner batch flushed");
        let _guard = BatchGuard::new();
        x.set(2);
    });
    assert_eq!(seen.get(), [(0, 0), (2, 1)]);
}

#[test]
fn a_subscription_is_called_once_per_batch() {
    let x = signal(0);
    let calls = signal(0);
    let _sub = x.subscribe({
        let calls = calls.clone();
        move || calls.update(|n| *n += 1)
    });
    batch(|| {
        x.set(1);
        x.set(2);
        x.set(3);
    });
    assert_eq!(calls.get(), 1);
    x.set(4);
    assert_eq!(calls.get(), 2, "outside a batch it is called right away");
}

#[test]
fn queued_updates_land_before_the_deferred_effects_run() {
    let (x, y) = (signal(0), signal(0));
    let (_e, seen) = watch_pair(&x, &y);
    batch(|| {
        x.set(1);
        let y = y.clone();
        queue_update(move || y.set(9));
    });
    assert_eq!(seen.get(), [(0, 0), (1, 9)]);
}

#[test]
fn flush_runs_the_deferred_effects_inside_the_batch() {
    let (x, y) = (signal(0), signal(0));
    let (_e, seen) = watch_pair(&x, &y);
    batch(|| {
        x.set(1);
        flush();
        assert_eq!(seen.get(), [(0, 0), (1, 0)]);
        y.set(1);
    });
    assert_eq!(seen.get(), [(0, 0), (1, 0), (1, 1)]);
}

#[test]
fn an_effect_dropped_during_the_batch_does_not_run_after_it() {
    let x = signal(0);
    let runs = signal(0);
    let e = effect({
        let (x, runs) = (x.clone(), runs.clone());
        move || {
            x.get();
            runs.update(|n| *n += 1);
        }
    });
    batch(|| {
        x.set(1);
        drop(e);
    });
    assert_eq!(runs.get(), 1);
}

#[test]
fn a_panicking_batch_drops_its_deferred_effects_and_leaves_no_trace() {
    let (x, y) = (signal(0), signal(0));
    let (_e, seen) = watch_pair(&x, &y);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        batch(|| {
            x.set(1);
            std::panic::resume_unwind(Box::new("boom"));
        })
    }));
    assert!(r.is_err());
    assert_eq!(
        seen.get(),
        [(0, 0)],
        "a deferred effect ran during the unwind"
    );
    // Nothing is left pending: the next change outside a batch runs at once
    y.set(2);
    assert_eq!(seen.get(), [(0, 0), (1, 2)]);
}

#[test]
fn an_effect_that_writes_a_signal_settles_after_the_batch() {
    let x = signal(0);
    let mirror = signal(0);
    let _e = effect({
        let (x, mirror) = (x.clone(), mirror.clone());
        move || mirror.set(x.get() * 10)
    });
    let (_e2, seen) = {
        let seen = signal(Vec::new());
        let e = effect({
            let (mirror, seen) = (mirror.clone(), seen.clone());
            move || seen.update(|v| v.push(mirror.get()))
        });
        (e, seen)
    };
    batch(|| x.set(1));
    assert_eq!(mirror.get(), 10);
    assert_eq!(seen.get().last(), Some(&10));
}

#[test]
fn a_queued_update_that_panics_still_closes_the_batch() {
    let (x, y) = (signal(0), signal(0));
    let (_e, seen) = watch_pair(&x, &y);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        batch(|| {
            x.set(1);
            queue_update(|| std::panic::resume_unwind(Box::new("boom")));
        })
    }));
    assert!(r.is_err());
    assert!(!revue::reactive::is_batching(), "the batch stayed open");
    y.set(3);
    assert_eq!(
        seen.get().last(),
        Some(&(1, 3)),
        "a later change was still deferred"
    );
}

#[test]
fn a_subscription_dropped_during_the_batch_is_not_called_after_it() {
    let x = signal(0);
    let calls = signal(0);
    let sub = x.subscribe({
        let calls = calls.clone();
        move || calls.update(|n| *n += 1)
    });
    batch(|| {
        x.set(1);
        drop(sub);
    });
    assert_eq!(calls.get(), 0, "a dropped subscription was called");
}
