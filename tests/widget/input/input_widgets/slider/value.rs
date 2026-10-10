//! Slider value handling: clamping, stepping, increment/decrement

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

// Found by tests/event_sequences.rs: the shrunk sequence was `<set NaN>` -
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
