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
