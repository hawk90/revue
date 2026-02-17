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
// File Size Constants
// =============================================================================

/// One kilobyte in bytes (1024 bytes)
pub const KB: u64 = 1024;

/// One megabyte in bytes (1024 KB)
pub const MB: u64 = 1024 * KB;

/// One gigabyte in bytes (1024 MB)
pub const GB: u64 = 1024 * MB;

/// Maximum CSS file size (1MB)
///
/// Used to prevent DoS attacks when loading CSS files.
pub const MAX_CSS_FILE_SIZE: u64 = MB;

/// Maximum config file size (1MB)
///
/// Used to prevent DoS attacks when loading config files.
pub const MAX_CONFIG_FILE_SIZE: u64 = MB;

/// Maximum snapshot file size (10MB)
///
/// Snapshots can be large, so we allow more space than CSS/config files.
pub const MAX_SNAPSHOT_FILE_SIZE: u64 = 10 * MB;

/// Maximum clipboard content size (10MB)
///
/// Used to prevent DoS attacks when reading from the system clipboard.
pub const MAX_CLIPBOARD_SIZE: usize = 10 * MB as usize;

/// Maximum paste event size (100KB)
///
/// Used to prevent DoS attacks through terminal paste events.
/// Terminal paste can be triggered by bracketed paste mode, which
/// could potentially allow injection of very large content.
pub const MAX_PASTE_SIZE: usize = 100 * KB as usize;

/// Maximum task queue size for pooled task runner
///
/// Maximum number of pending tasks in the thread pool queue.
/// Prevents unbounded memory growth when spawning many tasks.
pub const MAX_TASK_QUEUE_SIZE: usize = 1000;

/// Maximum comment length in CSS (100KB)
///
/// Used to prevent unreasonably long comments in CSS parsing.
pub const MAX_COMMENT_LENGTH: usize = 100 * KB as usize;

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

// Tests extracted to tests/core_constants_tests.rs
// All constant value tests use only public constants
