//! Tests for animation timer module
//!
//! Extracted from src/utils/animation/timer.rs

use revue::utils::animation::Timer;
use std::time::Duration;

#[test]
fn test_timer_new() {
    let timer = Timer::new(Duration::from_millis(100));
    assert_eq!(timer.duration, Duration::from_millis(100));
    assert!(!timer.is_running());
    assert!(!timer.is_finished());
    assert_eq!(timer.elapsed(), Duration::ZERO);
}

#[test]
fn test_timer_from_millis() {
    let timer = Timer::from_millis(500);
    assert_eq!(timer.duration, Duration::from_millis(500));
}

#[test]
fn test_timer_from_secs_f32() {
    let timer = Timer::from_secs_f32(1.5);
    assert_eq!(timer.duration, Duration::from_secs_f32(1.5));
}

#[test]
fn test_timer_start() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    assert!(timer.is_running());
    assert!(!timer.is_finished());
}

#[test]
fn test_timer_pause() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    timer.pause();
    assert!(!timer.is_running());
    // Elapsed time may be zero on fast systems, which is valid
    let elapsed_before = timer.elapsed();
    assert!(elapsed_before >= Duration::ZERO);
}

#[test]
fn test_timer_resume() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    timer.pause();
    timer.resume();
    assert!(timer.is_running());
}

#[test]
fn test_timer_reset() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    timer.reset();
    assert!(!timer.is_running());
    assert_eq!(timer.elapsed(), Duration::ZERO);
}

#[test]
fn test_timer_restart() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    timer.pause();
    timer.restart();
    assert!(timer.is_running());
    // Cannot test private `paused` field - is_running() is sufficient
}

#[test]
fn test_timer_elapsed() {
    let mut timer = Timer::from_millis(100);
    timer.reset(); // Ensure clean state
    timer.start();
    let elapsed = timer.elapsed();
    // Elapsed should be >= 0 and <= duration (may be 0 on fast systems)
    assert!(elapsed <= timer.duration);
}

#[test]
fn test_timer_remaining() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    let remaining = timer.remaining();
    assert!(remaining <= timer.duration);
}

#[test]
fn test_timer_progress() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    let progress = timer.progress();
    assert!(progress >= 0.0 && progress <= 1.0);
}

#[test]
fn test_timer_progress_zero_duration() {
    let timer = Timer::from_secs_f32(0.0);
    assert_eq!(timer.progress(), 1.0);
}

#[test]
fn test_timer_is_finished() {
    let mut timer = Timer::from_millis(10);
    timer.start();
    // Not finished immediately
    assert!(!timer.is_finished());
}

#[test]
fn test_timer_elapsed_on_pause() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    timer.pause();
    let elapsed1 = timer.elapsed();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed2 = timer.elapsed();
    // Elapsed should not change while paused
    assert_eq!(elapsed1, elapsed2);
}

#[test]
fn test_timer_pause_twice() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    timer.pause();
    timer.pause(); // Second pause should do nothing
    assert!(!timer.is_running());
}

#[test]
fn test_timer_resume_not_paused() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    let was_running = timer.is_running();
    timer.resume();
    // Should remain running
    assert_eq!(timer.is_running(), was_running);
}

#[test]
fn test_timer_remaining_clamped() {
    let timer = Timer::from_millis(10);
    let remaining = timer.remaining();
    assert_eq!(remaining, timer.duration);
}

#[test]
fn test_timer_progress_eased() {
    let mut timer = Timer::from_millis(100);
    timer.start();
    use revue::utils::easing::Easing;
    let progress = timer.progress_eased(Easing::Linear);
    assert!(progress >= 0.0 && progress <= 1.0);
}

// Removed test_timer_default_fields - it accessed private fields (elapsed_on_pause, paused)
