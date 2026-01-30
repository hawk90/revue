//! Integration tests for timer and stopwatch widgets

use revue::style::Color;
use revue::widget::{Stopwatch, Timer, TimerFormat, TimerState};

#[test]
fn test_timer_countdown_basic() {
    let timer = Timer::countdown(60);

    assert_eq!(timer.remaining_seconds(), 60);
    // Default format is Full, which shows "00:01:00" for 60 seconds
    assert!(timer.format_remaining().contains("01:00"));
    assert_eq!(timer.progress(), 0.0);
}

#[test]
fn test_timer_pomodoro() {
    let timer = Timer::pomodoro();

    assert_eq!(timer.remaining_seconds(), 25 * 60);
}

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

#[test]
fn test_timer_format_full() {
    let timer = Timer::countdown(3661); // 1h 1m 1s
    assert_eq!(timer.format_remaining(), "01:01:01");
}

#[test]
fn test_timer_format_short() {
    let timer = Timer::countdown(65).format(TimerFormat::Short);
    assert_eq!(timer.format_remaining(), "01:05");
}

#[test]
fn test_timer_format_compact() {
    let timer = Timer::countdown(90).format(TimerFormat::Compact);
    assert_eq!(timer.format_remaining(), "1m 30s");
}

#[test]
fn test_timer_format_precise() {
    let timer = Timer::countdown(65).format(TimerFormat::Precise);
    assert_eq!(timer.format_remaining(), "05.000");
}

#[test]
fn test_timer_state_transitions() {
    let mut timer = Timer::countdown(60);

    assert_eq!(timer.state(), TimerState::Stopped);
    assert!(!timer.is_running());
    assert!(!timer.is_completed());

    timer.start();
    assert_eq!(timer.state(), TimerState::Running);
    assert!(timer.is_running());

    timer.pause();
    assert_eq!(timer.state(), TimerState::Paused);
    assert!(!timer.is_running());

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_timer_stop() {
    let mut timer = Timer::countdown(60);
    timer.start();

    timer.stop();
    assert_eq!(timer.state(), TimerState::Stopped);
    assert_eq!(timer.remaining_seconds(), 60);
}

#[test]
fn test_timer_reset() {
    let mut timer = Timer::countdown(60);

    timer.reset();
    assert_eq!(timer.remaining_seconds(), 60);
}

#[test]
fn test_timer_progress_calculation() {
    let timer = Timer::countdown(100);
    assert_eq!(timer.progress(), 0.0);

    // Progress is based on remaining_ms / total_ms
    assert!(timer.progress() >= 0.0);
    assert!(timer.progress() <= 1.0);
}

#[test]
fn test_timer_progress_zero_total() {
    let timer = Timer::countdown(0);
    assert_eq!(timer.progress(), 1.0);
}

#[test]
fn test_timer_builder() {
    // Test builder pattern - we can't directly access private fields
    // but we can verify the timer was created successfully
    let _timer = Timer::countdown(60)
        .format(TimerFormat::Compact)
        .show_progress(false)
        .progress_width(50)
        .fg(Color::CYAN)
        .warning_threshold(30)
        .danger_threshold(5)
        .title("Test Timer")
        .large_digits(true)
        .auto_restart(true);

    // If we get here without panicking, the builder works
}

#[test]
fn test_stopwatch_basic() {
    let sw = Stopwatch::new();

    assert_eq!(sw.elapsed_millis(), 0);
    assert_eq!(sw.elapsed_seconds(), 0.0);
    assert!(!sw.is_running());
}

#[test]
fn test_stopwatch_format_elapsed() {
    let sw = Stopwatch::new();
    // format_elapsed uses internal state
    assert!(sw.format_elapsed().contains(":"));
}

#[test]
fn test_stopwatch_state_transitions() {
    let mut sw = Stopwatch::new();

    // Initially not running
    assert!(!sw.is_running());

    sw.start();
    assert!(sw.is_running());

    sw.pause();
    assert!(!sw.is_running());
}

#[test]
fn test_stopwatch_toggle() {
    let mut sw = Stopwatch::new();

    sw.toggle();
    assert!(sw.is_running());

    sw.toggle();
    assert!(!sw.is_running());
}

#[test]
fn test_stopwatch_stop() {
    let mut sw = Stopwatch::new();
    sw.start();

    sw.stop();
    assert!(!sw.is_running());
    assert_eq!(sw.elapsed_millis(), 0);
}

#[test]
fn test_stopwatch_reset() {
    let mut sw = Stopwatch::new();
    sw.start();
    sw.lap(); // Add a lap first

    sw.reset();
    assert_eq!(sw.elapsed_millis(), 0);
    assert!(sw.laps().is_empty());
}

#[test]
fn test_stopwatch_laps() {
    let mut sw = Stopwatch::new();
    // Start and update to add laps
    sw.start();
    sw.update();
    sw.lap(); // Can add lap through the public method

    let laps = sw.laps();
    assert!(!laps.is_empty());
}

#[test]
fn test_stopwatch_builder() {
    // Test builder pattern - we can't directly access private fields
    // but we can verify the stopwatch was created successfully
    let _sw = Stopwatch::new()
        .format(TimerFormat::Precise)
        .show_laps(false)
        .max_laps(10)
        .fg(Color::MAGENTA)
        .title("Test Stopwatch")
        .large_digits(true);

    // If we get here without panicking, the builder works
}
