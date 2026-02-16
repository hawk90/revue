//! Tests for core constants extracted from src/core/constants.rs
//!
//! All tests use only public constants from the revue::constants module.

use revue::constants::*;
use std::time::Duration;

// =========================================================================
// Frame duration tests
// =========================================================================

#[test]
fn test_frame_durations_are_correct() {
    assert_eq!(FRAME_DURATION_60FPS.as_millis(), 16);
    assert_eq!(FRAME_DURATION_30FPS.as_millis(), 33);
}

#[test]
fn test_frame_duration_60fps_value() {
    assert_eq!(FRAME_DURATION_60FPS, Duration::from_millis(16));
}

#[test]
fn test_frame_duration_30fps_value() {
    assert_eq!(FRAME_DURATION_30FPS, Duration::from_millis(33));
}

#[test]
fn test_60fps_faster_than_30fps() {
    assert!(FRAME_DURATION_60FPS < FRAME_DURATION_30FPS);
}

// =========================================================================
// Animation duration tests
// =========================================================================

#[test]
fn test_animation_durations_ascending() {
    assert!(ANIMATION_FAST_DURATION < ANIMATION_DEFAULT_DURATION);
    assert!(ANIMATION_DEFAULT_DURATION < ANIMATION_SLOW_DURATION);
    assert!(ANIMATION_SLOW_DURATION < ANIMATION_VERY_SLOW_DURATION);
}

#[test]
fn test_animation_default_duration_value() {
    assert_eq!(ANIMATION_DEFAULT_DURATION, Duration::from_millis(300));
}

#[test]
fn test_animation_fast_duration_value() {
    assert_eq!(ANIMATION_FAST_DURATION, Duration::from_millis(150));
}

#[test]
fn test_animation_slow_duration_value() {
    assert_eq!(ANIMATION_SLOW_DURATION, Duration::from_millis(500));
}

#[test]
fn test_animation_very_slow_duration_value() {
    assert_eq!(ANIMATION_VERY_SLOW_DURATION, Duration::from_millis(1000));
}

#[test]
fn test_animation_order() {
    // Fast < Default < Slow < Very Slow
    let durations = [
        ANIMATION_FAST_DURATION,
        ANIMATION_DEFAULT_DURATION,
        ANIMATION_SLOW_DURATION,
        ANIMATION_VERY_SLOW_DURATION,
    ];

    for i in 0..durations.len() - 1 {
        assert!(
            durations[i] < durations[i + 1],
            "Expected {:?} < {:?}",
            durations[i],
            durations[i + 1]
        );
    }
}

// =========================================================================
// Message duration tests
// =========================================================================

#[test]
fn test_message_durations_ascending() {
    assert!(MESSAGE_QUICK_DURATION < MESSAGE_DEFAULT_DURATION);
    assert!(MESSAGE_DEFAULT_DURATION < MESSAGE_LONG_DURATION);
}

#[test]
fn test_message_default_duration_value() {
    assert_eq!(MESSAGE_DEFAULT_DURATION, Duration::from_secs(3));
}

#[test]
fn test_message_quick_duration_value() {
    assert_eq!(MESSAGE_QUICK_DURATION, Duration::from_secs(2));
}

#[test]
fn test_message_long_duration_value() {
    assert_eq!(MESSAGE_LONG_DURATION, Duration::from_secs(5));
}

#[test]
fn test_message_order() {
    // Quick < Default < Long
    let durations = [
        MESSAGE_QUICK_DURATION,
        MESSAGE_DEFAULT_DURATION,
        MESSAGE_LONG_DURATION,
    ];

    for i in 0..durations.len() - 1 {
        assert!(
            durations[i] < durations[i + 1],
            "Expected {:?} < {:?}",
            durations[i],
            durations[i + 1]
        );
    }
}

// =========================================================================
// Test sleep duration tests
// =========================================================================

#[test]
fn test_test_sleep_durations_ascending() {
    assert!(TEST_SLEEP_SHORT < TEST_SLEEP_MEDIUM);
    assert!(TEST_SLEEP_MEDIUM < TEST_SLEEP_LONG);
}

#[test]
fn test_test_sleep_short_value() {
    assert_eq!(TEST_SLEEP_SHORT, Duration::from_millis(10));
}

#[test]
fn test_test_sleep_medium_value() {
    assert_eq!(TEST_SLEEP_MEDIUM, Duration::from_millis(50));
}

#[test]
fn test_test_sleep_long_value() {
    assert_eq!(TEST_SLEEP_LONG, Duration::from_millis(100));
}

// =========================================================================
// Debounce tests
// =========================================================================

#[test]
fn test_debounce_default_value() {
    assert_eq!(DEBOUNCE_DEFAULT, Duration::from_millis(100));
}

#[test]
fn test_debounce_search_value() {
    assert_eq!(DEBOUNCE_SEARCH, Duration::from_millis(150));
}

#[test]
fn test_debounce_file_system_value() {
    assert_eq!(DEBOUNCE_FILE_SYSTEM, Duration::from_millis(100));
}

#[test]
fn test_search_debounce_longer_than_default() {
    assert!(DEBOUNCE_SEARCH >= DEBOUNCE_DEFAULT);
}

// =========================================================================
// Tick/poll tests
// =========================================================================

#[test]
fn test_tick_rate_default_value() {
    assert_eq!(TICK_RATE_DEFAULT, Duration::from_millis(50));
}

#[test]
fn test_poll_immediate_value() {
    assert_eq!(POLL_IMMEDIATE, Duration::from_millis(0));
}

#[test]
fn test_poll_immediate_is_zero() {
    assert!(POLL_IMMEDIATE.is_zero());
}

// =========================================================================
// Screen transition tests
// =========================================================================

#[test]
fn test_screen_transition_duration_value() {
    assert_eq!(SCREEN_TRANSITION_DURATION, Duration::from_millis(200));
}

#[test]
fn test_screen_transition_reasonable() {
    // Screen transitions should be noticeable but not too slow
    assert!(SCREEN_TRANSITION_DURATION >= Duration::from_millis(100));
    assert!(SCREEN_TRANSITION_DURATION <= Duration::from_millis(500));
}

// =========================================================================
// Stagger delay tests
// =========================================================================

#[test]
fn test_stagger_delay_default_value() {
    assert_eq!(STAGGER_DELAY_DEFAULT, Duration::from_millis(50));
}

// =========================================================================
// Profiler tests
// =========================================================================

#[test]
fn test_profiler_report_interval_value() {
    assert_eq!(PROFILER_REPORT_INTERVAL, Duration::from_secs(5));
}

#[test]
fn test_fps_counter_window_value() {
    assert_eq!(FPS_COUNTER_WINDOW, Duration::from_secs(1));
}

// =========================================================================
// All durations positive tests
// =========================================================================

#[test]
fn test_all_durations_positive() {
    // All durations except POLL_IMMEDIATE should be positive
    let durations = [
        FRAME_DURATION_60FPS,
        FRAME_DURATION_30FPS,
        ANIMATION_DEFAULT_DURATION,
        ANIMATION_FAST_DURATION,
        ANIMATION_SLOW_DURATION,
        ANIMATION_VERY_SLOW_DURATION,
        DEBOUNCE_DEFAULT,
        DEBOUNCE_SEARCH,
        DEBOUNCE_FILE_SYSTEM,
        TICK_RATE_DEFAULT,
        SCREEN_TRANSITION_DURATION,
        STAGGER_DELAY_DEFAULT,
        MESSAGE_DEFAULT_DURATION,
        MESSAGE_QUICK_DURATION,
        MESSAGE_LONG_DURATION,
        TEST_SLEEP_SHORT,
        TEST_SLEEP_MEDIUM,
        TEST_SLEEP_LONG,
        PROFILER_REPORT_INTERVAL,
        FPS_COUNTER_WINDOW,
    ];

    for duration in &durations {
        assert!(
            !duration.is_zero(),
            "Duration should not be zero: {:?}",
            duration
        );
    }
}

// =========================================================================
// File size constant tests
// =========================================================================

#[test]
fn test_kb_value() {
    assert_eq!(KB, 1024);
}

#[test]
fn test_mb_value() {
    assert_eq!(MB, 1024 * 1024);
}

#[test]
fn test_gb_value() {
    assert_eq!(GB, 1024 * 1024 * 1024);
}

#[test]
fn test_size_units_relationship() {
    assert!(KB < MB);
    assert!(MB < GB);
}

#[test]
fn test_max_css_file_size() {
    assert_eq!(MAX_CSS_FILE_SIZE, MB);
}

#[test]
fn test_max_config_file_size() {
    assert_eq!(MAX_CONFIG_FILE_SIZE, MB);
}

#[test]
fn test_max_snapshot_file_size() {
    assert_eq!(MAX_SNAPSHOT_FILE_SIZE, 10 * MB);
}

#[test]
fn test_max_clipboard_size() {
    assert_eq!(MAX_CLIPBOARD_SIZE, 10 * MB as usize);
}

#[test]
fn test_max_comment_length() {
    assert_eq!(MAX_COMMENT_LENGTH, 100 * KB as usize);
}

#[test]
fn test_file_sizes_are_positive() {
    assert!(KB > 0);
    assert!(MB > 0);
    assert!(GB > 0);
    assert!(MAX_CSS_FILE_SIZE > 0);
    assert!(MAX_CONFIG_FILE_SIZE > 0);
    assert!(MAX_SNAPSHOT_FILE_SIZE > 0);
    assert!(MAX_CLIPBOARD_SIZE > 0);
    assert!(MAX_COMMENT_LENGTH > 0);
}
