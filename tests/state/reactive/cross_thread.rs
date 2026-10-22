//! A signal set on another thread reaches the computed values and effects
//! tracked on this one (#873): the tracker is per thread, so the change is
//! posted here and applied when this thread drains (reading a computed,
//! `flush`, or the app's next frame).

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use revue::reactive::{computed, effect, flush, signal};

#[test]
fn a_computed_sees_a_set_from_another_thread() {
    let s = signal(0);
    let c = computed({
        let s = s.clone();
        move || s.get() * 2
    });
    assert_eq!(c.get(), 0);
    let s2 = s.clone();
    std::thread::spawn(move || s2.set(5)).join().unwrap();
    assert_eq!(c.get(), 10); // 0 before
}

#[test]
fn an_effect_reruns_after_a_set_from_another_thread() {
    let s = signal(0);
    let runs = Arc::new(AtomicUsize::new(0));
    let _e = effect({
        let (s, runs) = (s.clone(), runs.clone());
        move || {
            s.get();
            runs.fetch_add(1, Ordering::SeqCst);
        }
    });
    assert_eq!(runs.load(Ordering::SeqCst), 1);
    let s2 = s.clone();
    std::thread::spawn(move || {
        s2.set(1);
        s2.set(2); // posted once
    })
    .join()
    .unwrap();
    assert_eq!(
        runs.load(Ordering::SeqCst),
        1,
        "applied when this thread drains"
    );
    flush();
    assert_eq!(runs.load(Ordering::SeqCst), 2);
    flush();
    assert_eq!(runs.load(Ordering::SeqCst), 2, "nothing left to apply");
}

/// A set on this thread is not posted back to it, and a thread that tracks
/// nothing gets nothing.
#[test]
fn a_set_here_reaches_dependents_once() {
    let s = signal(0);
    let runs = Arc::new(AtomicUsize::new(0));
    let _e = effect({
        let (s, runs) = (s.clone(), runs.clone());
        move || {
            s.get();
            runs.fetch_add(1, Ordering::SeqCst);
        }
    });
    s.set(1);
    flush();
    assert_eq!(runs.load(Ordering::SeqCst), 2);
}

/// use_async_immediate sets its state from its own thread: a computed over
/// it used to stay at Loading.
#[test]
fn a_computed_over_async_state_sees_the_result() {
    use revue::reactive::{use_async_immediate, AsyncState};
    let state = use_async_immediate(|| Ok(7));
    let ready = computed({
        let state = state.clone();
        move || matches!(state.get(), AsyncState::Ready(7))
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !ready.get() {
        assert!(
            std::time::Instant::now() < deadline,
            "the result never reached the computed"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
