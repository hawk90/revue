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
}
