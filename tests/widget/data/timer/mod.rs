//! Tests for `revue::widget::data::timer`: the helper functions and `TimerFormat`

mod countdown;
mod stopwatch;

use revue::widget::data::timer::{pomodoro, stopwatch, timer, TimerFormat};

#[test]
fn test_helper_functions() {
    let t = timer(120);
    assert_eq!(t.remaining_seconds(), 120);

    let sw = stopwatch();
    assert_eq!(sw.elapsed_millis(), 0);

    let p = pomodoro();
    assert_eq!(p.remaining_seconds(), 25 * 60);
}

#[test]
fn test_timer_format_default() {
    assert_eq!(TimerFormat::default(), TimerFormat::Full);
}
