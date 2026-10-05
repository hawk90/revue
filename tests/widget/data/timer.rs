//! Timer widget tests not covered by tests/widget/timer.rs or tests/timer_tests.rs

use revue::widget::data::timer::{pomodoro, stopwatch, timer, Timer, TimerFormat, TimerState};

#[test]
fn test_timer_toggle() {
    let mut timer = Timer::countdown(60);
    assert_eq!(timer.state(), TimerState::Stopped);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Paused);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);
}

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
