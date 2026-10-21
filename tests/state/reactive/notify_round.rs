//! A callback dropped earlier in a notify round is not called in that round
//! (#1197), as one dropped during a batch is not called after it.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use revue::reactive::{
    dispose_subscriber, signal, signal_vec, start_tracking, stop_tracking, Subscriber,
    SubscriberId, Subscription, VecSubscription,
};

/// Two handles in slots, each callback dropping the other's: whichever runs
/// first drops the second, so one call in all.
type Slot<H> = Arc<Mutex<Option<H>>>;

fn slots<H>() -> (Slot<H>, Slot<H>) {
    (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))
}

#[test]
fn a_subscription_dropped_by_an_earlier_subscriber_is_not_called() {
    let x = signal(0);
    let calls = Arc::new(AtomicUsize::new(0));
    let (a, b): (Slot<Subscription>, Slot<Subscription>) = slots();
    let subscribe = |other: &Slot<Subscription>| {
        let (calls, other) = (calls.clone(), other.clone());
        x.subscribe(move || {
            calls.fetch_add(1, Ordering::SeqCst);
            other.lock().unwrap().take();
        })
    };
    *a.lock().unwrap() = Some(subscribe(&b));
    *b.lock().unwrap() = Some(subscribe(&a));

    x.set(1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn a_diff_subscription_dropped_by_an_earlier_subscriber_is_not_called() {
    let v = signal_vec(vec![0]);
    let calls = Arc::new(AtomicUsize::new(0));
    let (a, b): (Slot<VecSubscription<i32>>, Slot<VecSubscription<i32>>) = slots();
    let subscribe = |other: &Slot<VecSubscription<i32>>| {
        let (calls, other) = (calls.clone(), other.clone());
        v.subscribe_diff(move |_| {
            calls.fetch_add(1, Ordering::SeqCst);
            other.lock().unwrap().take();
        })
    };
    *a.lock().unwrap() = Some(subscribe(&b));
    *b.lock().unwrap() = Some(subscribe(&a));

    v.push(1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

/// `Effect` checks its own active flag, so this goes through the tracker API
/// directly, where nothing but the tracker's lookup stops a disposed one.
#[test]
fn a_dependent_disposed_by_an_earlier_dependent_is_not_called() {
    let x = signal(0);
    let calls = Arc::new(AtomicUsize::new(0));
    let (a, b): (Slot<SubscriberId>, Slot<SubscriberId>) = slots();
    let depend = |other: &Slot<SubscriberId>| {
        let (calls, other) = (calls.clone(), other.clone());
        let subscriber = Subscriber::new(move || {
            calls.fetch_add(1, Ordering::SeqCst);
            if let Some(id) = other.lock().unwrap().take() {
                dispose_subscriber(id);
            }
        });
        let id = subscriber.id;
        start_tracking(subscriber);
        x.get();
        stop_tracking();
        id
    };
    *a.lock().unwrap() = Some(depend(&b));
    *b.lock().unwrap() = Some(depend(&a));

    x.set(1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    for id in [a, b].iter().filter_map(|slot| slot.lock().unwrap().take()) {
        dispose_subscriber(id);
    }
}
