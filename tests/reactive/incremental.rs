//! An `IncrementalComputed` follows its source vector: each change runs the
//! matching handler, and whatever reads it hears about the change.

use revue::reactive::{
    computed, effect, signal, signal_vec, IncrementalComputed, IncrementalHandlers, SignalVec,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn evens(items: &SignalVec<i32>) -> IncrementalComputed<i32, Vec<i32>> {
    IncrementalComputed::new(
        items.clone(),
        |v| v.iter().filter(|x| *x % 2 == 0).copied().collect(),
        IncrementalHandlers::<i32, Vec<i32>>::new()
            .insert(|result, _, value| {
                if value % 2 == 0 {
                    result.push(value);
                }
            })
            .update(|result, _, old, new| {
                result.retain(|x| *x != old);
                if new % 2 == 0 {
                    result.push(new);
                }
            })
            .remove(|result, _, value| result.retain(|x| *x != value))
            .replace(|items| items.get().into_iter().filter(|x| x % 2 == 0).collect()),
    )
}

#[test]
fn each_change_to_the_source_runs_its_handler() {
    let items = signal_vec(vec![1, 2, 3]);
    let evens = evens(&items);
    assert_eq!(evens.get(), vec![2]);

    items.push(4);
    assert_eq!(evens.get(), vec![2, 4], "insert was not applied");
    items.update(0, 6);
    assert_eq!(evens.get(), vec![2, 4, 6], "update was not applied");
    items.remove(1);
    assert_eq!(evens.get(), vec![4, 6], "remove was not applied");
    items.replace(vec![8, 9, 10]);
    assert_eq!(evens.get(), vec![8, 10], "replace was not applied");
}

#[test]
fn a_clone_keeps_following_after_the_original_is_dropped() {
    let items = signal_vec(vec![2]);
    let original = evens(&items);
    let clone = original.clone();
    drop(original);

    items.push(4);
    assert_eq!(clone.get(), vec![2, 4]);
}

#[test]
fn dropping_every_handle_stops_the_handlers() {
    let items = signal_vec(Vec::<i32>::new());
    let calls = Arc::new(AtomicUsize::new(0));
    let counted = IncrementalComputed::new(
        items.clone(),
        |v| v.len(),
        IncrementalHandlers::<i32, usize>::new().insert({
            let calls = calls.clone();
            move |len, _, _| {
                calls.fetch_add(1, Ordering::SeqCst);
                *len += 1;
            }
        }),
    );
    items.push(1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    drop(counted);
    items.push(2);
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "the handler still ran with no handle left to read the result"
    );
}

#[test]
fn an_effect_reading_it_reruns_with_the_new_value() {
    let items = signal_vec(vec![2]);
    let evens = evens(&items);
    let seen = signal(Vec::new());
    let _e = effect({
        let (evens, seen) = (evens.clone(), seen.clone());
        move || seen.set(evens.get())
    });
    assert_eq!(seen.get(), vec![2]);

    items.push(4);
    assert_eq!(seen.get(), vec![2, 4], "the effect did not see the change");
}

#[test]
fn a_computed_reading_it_is_invalidated() {
    let items = signal_vec(vec![2]);
    let evens = evens(&items);
    let count = computed({
        let evens = evens.clone();
        move || evens.get().len()
    });
    assert_eq!(count.get(), 1);

    items.push(4);
    assert_eq!(count.get(), 2, "the computed kept its stale value");
}
