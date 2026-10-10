//! A computed value is a dependency like a signal: whatever reads it - an
//! effect, another computed - must hear when its inputs change.

use revue::reactive::{computed, effect, signal};

#[test]
fn an_effect_reading_a_computed_reruns_when_its_input_changes() {
    let x = signal(1);
    let doubled = computed({
        let x = x.clone();
        move || x.get() * 2
    });
    let seen = signal(0);
    let _e = effect({
        let (doubled, seen) = (doubled.clone(), seen.clone());
        move || seen.set(doubled.get())
    });
    assert_eq!(seen.get(), 2);

    x.set(5);
    assert_eq!(seen.get(), 10, "the effect did not see the computed change");
    x.set(7);
    assert_eq!(seen.get(), 14);
}

#[test]
fn a_computed_reading_a_cached_computed_is_invalidated_through_it() {
    let x = signal(1);
    let plus_one = computed({
        let x = x.clone();
        move || x.get() + 1
    });
    let times_two = computed({
        let plus_one = plus_one.clone();
        move || plus_one.get() * 2
    });
    // Read the inner one first, so the outer finds it cached
    assert_eq!(plus_one.get(), 2);
    assert_eq!(times_two.get(), 4);

    x.set(2);
    assert_eq!(times_two.get(), 6, "the outer computed kept a stale value");
}

#[test]
fn an_effect_at_the_end_of_a_chain_follows_the_signal() {
    let x = signal(1);
    let a = computed({
        let x = x.clone();
        move || x.get() + 1
    });
    let b = computed({
        let a = a.clone();
        move || a.get() * 10
    });
    let seen = signal(0);
    let _e = effect({
        let (b, seen) = (b.clone(), seen.clone());
        move || seen.set(b.get())
    });
    assert_eq!(seen.get(), 20);
    x.set(4);
    assert_eq!(seen.get(), 50);
}

#[test]
fn a_dropped_effect_stops_hearing_the_computed() {
    let x = signal(1);
    let doubled = computed({
        let x = x.clone();
        move || x.get() * 2
    });
    let runs = signal(0);
    {
        let _e = effect({
            let (doubled, runs) = (doubled.clone(), runs.clone());
            move || {
                doubled.get();
                runs.update(|n| *n += 1);
            }
        });
        x.set(2);
    }
    let before = runs.get();
    x.set(3);
    assert_eq!(runs.get(), before, "a dropped effect still ran");
}

/// An effect that reads a signal and a value computed from it ends on the
/// pair as it is, whichever of the two notifies it first.
#[test]
fn an_effect_reading_a_signal_and_its_computed_ends_consistent() {
    let x = signal(1);
    let doubled = computed({
        let x = x.clone();
        move || x.get() * 2
    });
    let pair = signal((0, 0));
    let _e = effect({
        let (x, doubled, pair) = (x.clone(), doubled.clone(), pair.clone());
        move || pair.set((x.get(), doubled.get()))
    });
    for v in [2, 3, 10] {
        x.set(v);
        assert_eq!(pair.get(), (v, v * 2));
    }
}
