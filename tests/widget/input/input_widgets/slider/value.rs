//! Slider value handling: clamping, stepping, increment/decrement

use proptest::prelude::*;
use revue::widget::{slider, slider_range};

#[test]
fn test_slider_set_value() {
    let mut s = slider();
    s.set_value(70.0);
    assert_eq!(s.get_value(), 70.0);
}

#[test]
fn test_slider_value_is_clamped_to_range() {
    assert_eq!(slider().value(150.0).get_value(), 100.0);
    assert_eq!(slider().value(-5.0).get_value(), 0.0);

    let mut s = slider_range(10.0, 20.0);
    s.set_value(25.0);
    assert_eq!(s.get_value(), 20.0);
    s.set_value(0.0);
    assert_eq!(s.get_value(), 10.0);
}

#[test]
fn test_slider_range_reclamps_existing_value() {
    let s = slider().value(80.0).range(0.0, 50.0);
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_step_snaps_value() {
    let mut s = slider().step(10.0);
    s.set_value(34.0);
    assert_eq!(s.get_value(), 30.0);
    s.set_value(36.0);
    assert_eq!(s.get_value(), 40.0);
}

#[test]
fn test_slider_negative_step_is_treated_as_positive() {
    let mut s = slider().step(-25.0);
    s.set_value(40.0);
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_step_snaps_relative_to_min() {
    let mut s = slider_range(1.0, 11.0).step(5.0);
    s.set_value(5.0);
    assert_eq!(s.get_value(), 6.0);
}

#[test]
fn test_slider_increment_and_decrement_with_step() {
    let mut s = slider().step(5.0).value(50.0);
    s.increment();
    assert_eq!(s.get_value(), 55.0);
    s.decrement();
    s.decrement();
    assert_eq!(s.get_value(), 45.0);
}

#[test]
fn test_slider_continuous_increment_is_one_percent_of_range() {
    let mut s = slider_range(0.0, 200.0);
    s.increment();
    assert_eq!(s.get_value(), 2.0);
    s.decrement();
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_increment_stops_at_bounds() {
    let mut s = slider().step(25.0).value(75.0);
    s.increment();
    assert_eq!(s.get_value(), 100.0);
    s.increment();
    assert_eq!(s.get_value(), 100.0);

    s.set_min();
    s.decrement();
    assert_eq!(s.get_value(), 0.0);
}

// Found by tests/widget/catalog/sequences/mod.rs: the shrunk sequence was `<set NaN>` -
// NaN clamps to NaN, so the value left the range for good (stepping from
// NaN gives NaN again).
#[test]
fn test_set_value_nan_keeps_the_value() {
    let mut s = slider_range(-10.0, 10.0).step(3.0).value(2.0);
    let before = s.get_value();
    s.set_value(f64::NAN);
    assert_eq!(s.get_value(), before);
    s.increment();
    assert!((-10.0..=10.0).contains(&s.get_value()));
}

#[test]
fn test_slider_range_given_high_to_low_is_normalized() {
    // A range given high to low used to panic in `f64::clamp`; the bounds
    // are swapped instead
    let mut s = slider().value(80.0).range(50.0, 0.0);
    assert_eq!(s.get_value(), 50.0);
    s.set_value(-5.0);
    assert_eq!(s.get_value(), 0.0);
    s.set_value(25.0);
    assert_eq!(s.get_value(), 25.0);

    let s = slider_range(20.0, 10.0);
    assert_eq!(s.get_value(), 10.0);
}

#[test]
fn test_slider_range_with_nan_bound_keeps_the_old_bound() {
    let mut s = slider().value(80.0).range(f64::NAN, 50.0);
    assert_eq!(s.get_value(), 50.0);
    s.set_value(-5.0);
    assert_eq!(s.get_value(), 0.0);

    let s = slider_range(10.0, f64::NAN).value(500.0);
    assert_eq!(s.get_value(), 100.0);
}

#[derive(Clone, Debug)]
enum Op {
    Set(f64),
    Increment,
    Decrement,
    Min,
    Max,
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        prop_oneof![
            -1e4f64..1e4,
            Just(f64::NAN),
            Just(f64::INFINITY),
            Just(f64::NEG_INFINITY),
        ]
        .prop_map(Op::Set),
        Just(Op::Increment),
        Just(Op::Decrement),
        Just(Op::Min),
        Just(Op::Max),
    ]
}

proptest! {
    /// After any operations the value is inside the range and on the step
    /// grid from `min`, or the maximum itself.
    #[test]
    fn prop_slider_value_stays_in_range_on_the_step(
        a in -1e3f64..1e3,
        b in -1e3f64..1e3,
        step in prop_oneof![Just(0.0), 0.01f64..50.0],
        ops in prop::collection::vec(op(), 0..30),
    ) {
        // The step before the range, so the first value is snapped too
        // (`step` does not snap the value it has, #1011)
        let mut s = slider().step(step).range(a, b);
        let (min, max) = (a.min(b), a.max(b));
        for o in &ops {
            match *o {
                Op::Set(v) => s.set_value(v),
                Op::Increment => s.increment(),
                Op::Decrement => s.decrement(),
                Op::Min => s.set_min(),
                Op::Max => s.set_max(),
            }
            let v = s.get_value();
            prop_assert!((min..=max).contains(&v), "{} not in {}..={} after {:?}", v, min, max, o);
            if step > 0.0 && v != max {
                let steps = (v - min) / step;
                prop_assert!((steps - steps.round()).abs() < 1e-6, "{} off the grid of {} after {:?}", v, step, o);
            }
        }
    }

    /// With a step, an increment that moves the value and stays below the
    /// maximum is undone by a decrement, exactly. (An increment that would
    /// pass the maximum may not move at all, #1012.)
    #[test]
    fn prop_slider_increment_then_decrement_returns(
        // Mostly below the top, so few cases are skipped
        step in 0.01f64..25.0,
        start in 0.0f64..=75.0,
    ) {
        let mut s = slider().step(step);
        s.set_value(start);
        let before = s.get_value();
        s.increment();
        prop_assume!(s.get_value() > before && s.get_value() < 100.0);
        s.decrement();
        prop_assert_eq!(s.get_value(), before);
    }
}
