//! Common constants used throughout Revue
//!
//! This module centralizes magic numbers and commonly used values to improve
//! maintainability and ensure consistency across the codebase.
//!
//! # Duration Constants
//!
//! ```rust
//! use revue::constants::*;
//! use std::time::Duration;
//!
//! // Animation durations
//! assert_eq!(ANIMATION_DEFAULT_DURATION, Duration::from_millis(300));
//! assert_eq!(ANIMATION_FAST_DURATION, Duration::from_millis(150));
//!
//! // Frame rates
//! assert_eq!(FRAME_DURATION_60FPS, Duration::from_millis(16));
//! ```

use std::time::Duration;

// =============================================================================
// Frame Rate Constants
// =============================================================================

/// Duration for ~60 FPS frame rate (16.67ms rounded to 16ms)
///
/// Used for the main event loop tick rate when targeting smooth animations.
pub const FRAME_DURATION_60FPS: Duration = Duration::from_millis(16);

/// Duration for ~30 FPS frame rate
///
/// Used for less demanding applications or when battery life is a concern.
pub const FRAME_DURATION_30FPS: Duration = Duration::from_millis(33);

// =============================================================================
// Animation Duration Constants
// =============================================================================

/// Default animation duration (300ms)
///
/// Standard duration for most transitions and animations.
/// Provides a smooth feel without being too slow.
pub const ANIMATION_DEFAULT_DURATION: Duration = Duration::from_millis(300);

/// Fast animation duration (150ms)
///
/// Used for quick feedback animations like button presses or hover states.
pub const ANIMATION_FAST_DURATION: Duration = Duration::from_millis(150);

/// Slow animation duration (500ms)
///
/// Used for more dramatic animations like modal appearances or page transitions.
pub const ANIMATION_SLOW_DURATION: Duration = Duration::from_millis(500);

/// Very slow animation duration (1000ms)
///
/// Used for complex choreographed animations or loading sequences.
pub const ANIMATION_VERY_SLOW_DURATION: Duration = Duration::from_millis(1000);

// =============================================================================
// Debounce / Throttle Constants
// =============================================================================

/// Default debounce duration (100ms)
///
/// Standard delay for input debouncing to prevent excessive updates.
pub const DEBOUNCE_DEFAULT: Duration = Duration::from_millis(100);

/// Search debounce duration (150ms)
///
/// Slightly longer debounce for search inputs to reduce API calls.
pub const DEBOUNCE_SEARCH: Duration = Duration::from_millis(150);

/// File system debounce duration (100ms)
///
/// Debounce for file watcher events (hot reload).
pub const DEBOUNCE_FILE_SYSTEM: Duration = Duration::from_millis(100);

// =============================================================================
// Tick / Poll Constants
// =============================================================================

/// Default tick rate (50ms)
///
/// Standard polling interval for event readers.
pub const TICK_RATE_DEFAULT: Duration = Duration::from_millis(50);

/// Immediate poll (0ms)
///
/// Non-blocking poll for checking event availability.
pub const POLL_IMMEDIATE: Duration = Duration::from_millis(0);

// =============================================================================
// Screen Transition Constants
// =============================================================================

/// Screen transition duration (200ms)
///
/// Duration for screen/page transitions in multi-screen apps.
pub const SCREEN_TRANSITION_DURATION: Duration = Duration::from_millis(200);

// =============================================================================
// Stagger / Delay Constants
// =============================================================================

/// Default stagger delay (50ms)
///
/// Delay between animated items in a staggered animation sequence.
pub const STAGGER_DELAY_DEFAULT: Duration = Duration::from_millis(50);

// =============================================================================
// Message / Toast Constants
// =============================================================================

/// Default message display duration (3 seconds)
///
/// How long toast messages and notifications are shown by default.
pub const MESSAGE_DEFAULT_DURATION: Duration = Duration::from_secs(3);

/// Quick message display duration (2 seconds)
///
/// Shorter duration for less important notifications.
pub const MESSAGE_QUICK_DURATION: Duration = Duration::from_secs(2);

/// Long message display duration (5 seconds)
///
/// Longer duration for important messages that need more reading time.
pub const MESSAGE_LONG_DURATION: Duration = Duration::from_secs(5);

// =============================================================================
// Test / Debug Constants
// =============================================================================

/// Test sleep duration (10ms)
///
/// Short sleep used in tests to allow async operations to complete.
pub const TEST_SLEEP_SHORT: Duration = Duration::from_millis(10);

/// Test sleep duration (50ms)
///
/// Medium sleep used in tests for operations that need more time.
pub const TEST_SLEEP_MEDIUM: Duration = Duration::from_millis(50);

/// Test sleep duration (100ms)
///
/// Longer sleep for tests that need to wait for multiple operations.
pub const TEST_SLEEP_LONG: Duration = Duration::from_millis(100);

// =============================================================================
// Profiler Constants
// =============================================================================

/// Profiler report interval (5 seconds)
///
/// How often the profiler plugin reports metrics.
pub const PROFILER_REPORT_INTERVAL: Duration = Duration::from_secs(5);

/// FPS counter window (1 second)
///
/// Time window for calculating average FPS.
pub const FPS_COUNTER_WINDOW: Duration = Duration::from_secs(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_durations_are_correct() {
        assert_eq!(FRAME_DURATION_60FPS.as_millis(), 16);
        assert_eq!(FRAME_DURATION_30FPS.as_millis(), 33);
    }

    #[test]
    fn test_animation_durations_ascending() {
        assert!(ANIMATION_FAST_DURATION < ANIMATION_DEFAULT_DURATION);
        assert!(ANIMATION_DEFAULT_DURATION < ANIMATION_SLOW_DURATION);
        assert!(ANIMATION_SLOW_DURATION < ANIMATION_VERY_SLOW_DURATION);
    }

    #[test]
    fn test_message_durations_ascending() {
        assert!(MESSAGE_QUICK_DURATION < MESSAGE_DEFAULT_DURATION);
        assert!(MESSAGE_DEFAULT_DURATION < MESSAGE_LONG_DURATION);
    }

    #[test]
    fn test_test_sleep_durations_ascending() {
        assert!(TEST_SLEEP_SHORT < TEST_SLEEP_MEDIUM);
        assert!(TEST_SLEEP_MEDIUM < TEST_SLEEP_LONG);
    }

    // =========================================================================
    // Frame rate constant tests
    // =========================================================================

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
    // Animation duration constant tests
    // =========================================================================

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

    // =========================================================================
    // Debounce constant tests
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
    // Tick/poll constant tests
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
    // Screen transition constant tests
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
    // Stagger delay constant tests
    // =========================================================================

    #[test]
    fn test_stagger_delay_default_value() {
        assert_eq!(STAGGER_DELAY_DEFAULT, Duration::from_millis(50));
    }

    // =========================================================================
    // Message duration constant tests
    // =========================================================================

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

    // =========================================================================
    // Test sleep constant tests
    // =========================================================================

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
    // Profiler constant tests
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
    // Relationship tests
    // =========================================================================

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
}
