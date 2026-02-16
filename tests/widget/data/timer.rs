//! Timer widget tests using only public APIs

use revue::widget::data::timer::{
    pomodoro, stopwatch, timer, Stopwatch, Timer, TimerFormat, TimerState,
};

// =========================================================================
// Timer::countdown tests
// =========================================================================

#[test]
fn test_timer_new() {
    let timer = Timer::countdown(60);
    assert_eq!(timer.remaining_seconds(), 60);
    assert_eq!(timer.state(), TimerState::Stopped);
}

// =========================================================================
// Timer::format tests
// =========================================================================

#[test]
fn test_timer_format() {
    let timer = Timer::countdown(3661); // 1h 1m 1s
    assert_eq!(timer.format_remaining(), "01:01:01");

    let timer2 = Timer::countdown(65).format(TimerFormat::Short);
    assert_eq!(timer2.format_remaining(), "01:05");

    let timer3 = Timer::countdown(90).format(TimerFormat::Compact);
    assert_eq!(timer3.format_remaining(), "1m 30s");
}

#[test]
fn test_timer_format_precise() {
    let timer = Timer::countdown(65).format(TimerFormat::Precise);
    assert_eq!(timer.format_remaining(), "05.000");
}

// =========================================================================
// Timer state control tests
// =========================================================================

#[test]
fn test_timer_start_pause() {
    let mut timer = Timer::countdown(60);
    assert_eq!(timer.state(), TimerState::Stopped);

    timer.start();
    assert_eq!(timer.state(), TimerState::Running);

    timer.pause();
    assert_eq!(timer.state(), TimerState::Paused);

    timer.start();
    assert_eq!(timer.state(), TimerState::Running);
}

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
fn test_timer_is_running() {
    let mut timer = Timer::countdown(60);
    assert!(!timer.is_running());

    timer.start();
    assert!(timer.is_running());

    timer.pause();
    assert!(!timer.is_running());
}

// =========================================================================
// Timer::progress tests
// =========================================================================

#[test]
fn test_timer_progress_zero_total() {
    let timer = Timer::countdown(0);
    assert_eq!(timer.progress(), 1.0);
}

// =========================================================================
// Preset timers
// =========================================================================

#[test]
fn test_timer_short_break() {
    let timer = Timer::short_break();
    assert_eq!(timer.remaining_seconds(), 5 * 60);
}

#[test]
fn test_timer_long_break() {
    let timer = Timer::long_break();
    assert_eq!(timer.remaining_seconds(), 15 * 60);
}

// =========================================================================
// Helper functions
// =========================================================================

#[test]
fn test_helper_functions() {
    let t = timer(120);
    assert_eq!(t.remaining_seconds(), 120);

    let sw = stopwatch();
    assert_eq!(sw.elapsed_millis(), 0);

    let p = pomodoro();
    assert_eq!(p.remaining_seconds(), 25 * 60);
}

// =========================================================================
// Stopwatch tests
// =========================================================================

#[test]
fn test_stopwatch_is_running() {
    let mut sw = Stopwatch::new();
    assert!(!sw.is_running());

    sw.start();
    assert!(sw.is_running());

    sw.pause();
    assert!(!sw.is_running());
}

// =========================================================================
// Enum tests
// =========================================================================

#[test]
fn test_timer_state_equality() {
    assert_eq!(TimerState::Stopped, TimerState::Stopped);
    assert_ne!(TimerState::Running, TimerState::Stopped);
}

#[test]
fn test_timer_format_default() {
    assert_eq!(TimerFormat::default(), TimerFormat::Full);
}
