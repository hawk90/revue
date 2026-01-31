//! Gesture Recognition for Terminal Applications
//!
//! Provides gesture recognition for mouse interactions in terminal applications.
//! Maps mouse events to higher-level gestures like swipes, long-presses, and drags.
//!
//! # Gesture Mappings
//!
//! | Gesture | Mouse Equivalent |
//! |---------|------------------|
//! | Swipe | Click-drag-release quickly |
//! | Long-press | Click and hold |
//! | Pinch | Ctrl + scroll |
//! | Drag | Click and move |
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::event::gesture::*;
//!
//! let mut recognizer = GestureRecognizer::new();
//!
//! // Configure thresholds
//! recognizer.set_long_press_duration(Duration::from_millis(500));
//! recognizer.set_swipe_threshold(5);
//!
//! // Register gesture handlers
//! recognizer.on_swipe(|gesture| {
//!     println!("Swipe {:?} with velocity {:.2}", gesture.direction, gesture.velocity);
//! });
//!
//! recognizer.on_long_press(|gesture| {
//!     println!("Long press at ({}, {})", gesture.x, gesture.y);
//! });
//!
//! // Process mouse events
//! recognizer.handle_mouse_event(&mouse_event);
//! ```

use crate::event::MouseButton;
use std::time::{Duration, Instant};

// =============================================================================
// Constants
// =============================================================================

/// Default minimum distance for swipe detection (pixels)
const DEFAULT_SWIPE_THRESHOLD: u16 = 3;

/// Default maximum duration for swipe gesture (300ms)
const DEFAULT_SWIPE_MAX_DURATION: Duration = Duration::from_millis(300);

/// Default minimum velocity for swipe (pixels/second)
const DEFAULT_SWIPE_MIN_VELOCITY: f64 = 10.0;

/// Default duration for long press gesture (500ms)
const DEFAULT_LONG_PRESS_DURATION: Duration = Duration::from_millis(500);

/// Default minimum distance for drag detection (pixels)
const DEFAULT_DRAG_THRESHOLD: u16 = 2;

/// Default pinch scale per scroll wheel tick
const DEFAULT_PINCH_SCALE_PER_SCROLL: f64 = 0.1;

/// Default interval between double taps (300ms)
const DEFAULT_DOUBLE_TAP_INTERVAL: Duration = Duration::from_millis(300);

/// Default maximum distance between double taps (pixels)
const DEFAULT_DOUBLE_TAP_DISTANCE: u16 = 2;

// =============================================================================
// Gesture Types
// =============================================================================

/// Direction of a swipe gesture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    /// Swipe upward
    Up,
    /// Swipe downward
    Down,
    /// Swipe left
    Left,
    /// Swipe right
    Right,
}

impl SwipeDirection {
    /// Check if this is a vertical swipe
    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }

    /// Check if this is a horizontal swipe
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

/// Pinch direction (zoom in/out)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinchDirection {
    /// Pinch in (zoom out)
    In,
    /// Pinch out (zoom in)
    Out,
}

/// Gesture state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GestureState {
    /// No gesture in progress
    #[default]
    Idle,
    /// Potential gesture started
    Possible,
    /// Gesture recognized and in progress
    Active,
    /// Gesture completed successfully
    Ended,
    /// Gesture was cancelled
    Cancelled,
}

// =============================================================================
// Gesture Data
// =============================================================================

/// Swipe gesture data
#[derive(Debug, Clone)]
pub struct SwipeGesture {
    /// Swipe direction
    pub direction: SwipeDirection,
    /// Starting X position
    pub start_x: u16,
    /// Starting Y position
    pub start_y: u16,
    /// Ending X position
    pub end_x: u16,
    /// Ending Y position
    pub end_y: u16,
    /// Distance traveled
    pub distance: f64,
    /// Velocity (distance per second)
    pub velocity: f64,
    /// Duration of the swipe
    pub duration: Duration,
    /// Which mouse button was used
    pub button: MouseButton,
}

/// Swipe gesture data with methods
impl SwipeGesture {
    /// Get the delta X
    pub fn delta_x(&self) -> i32 {
        self.end_x as i32 - self.start_x as i32
    }

    /// Get the delta Y
    pub fn delta_y(&self) -> i32 {
        self.end_y as i32 - self.start_y as i32
    }
}

/// Long press gesture data
#[derive(Debug, Clone)]
pub struct LongPressGesture {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Duration held
    pub duration: Duration,
    /// Which mouse button was used
    pub button: MouseButton,
}

/// Drag gesture data
#[derive(Debug, Clone)]
pub struct DragGesture {
    /// Starting X position
    pub start_x: u16,
    /// Starting Y position
    pub start_y: u16,
    /// Current X position
    pub current_x: u16,
    /// Current Y position
    pub current_y: u16,
    /// Previous X position (for delta calculation)
    pub prev_x: u16,
    /// Previous Y position (for delta calculation)
    pub prev_y: u16,
    /// Total distance traveled
    pub total_distance: f64,
    /// Duration of drag
    pub duration: Duration,
    /// Which mouse button is used
    pub button: MouseButton,
    /// Gesture state
    pub state: GestureState,
}

impl DragGesture {
    /// Get delta X from previous position
    pub fn delta_x(&self) -> i32 {
        self.current_x as i32 - self.prev_x as i32
    }

    /// Get delta Y from previous position
    pub fn delta_y(&self) -> i32 {
        self.current_y as i32 - self.prev_y as i32
    }

    /// Get total delta X from start
    pub fn total_delta_x(&self) -> i32 {
        self.current_x as i32 - self.start_x as i32
    }

    /// Get total delta Y from start
    pub fn total_delta_y(&self) -> i32 {
        self.current_y as i32 - self.start_y as i32
    }
}

/// Pinch gesture data (simulated via Ctrl+scroll)
#[derive(Debug, Clone)]
pub struct PinchGesture {
    /// Pinch direction (in/out)
    pub direction: PinchDirection,
    /// Center X position
    pub x: u16,
    /// Center Y position
    pub y: u16,
    /// Scale factor (1.0 = no change)
    pub scale: f64,
    /// Cumulative scale during gesture
    pub cumulative_scale: f64,
}

/// Tap gesture data
#[derive(Debug, Clone)]
pub struct TapGesture {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Which button was tapped
    pub button: MouseButton,
    /// Number of taps (1 = single, 2 = double, etc.)
    pub count: u8,
}

// =============================================================================
// Gesture Tracking State
// =============================================================================

/// Internal state for tracking gestures
#[derive(Debug, Clone)]
pub struct TrackingState {
    /// Is a button currently pressed
    pub button_down: Option<MouseButton>,
    /// Start position X
    pub start_x: u16,
    /// Start position Y
    pub start_y: u16,
    /// Current position X
    pub current_x: u16,
    /// Current position Y
    pub current_y: u16,
    /// Previous position X (for drag delta)
    pub prev_x: u16,
    /// Previous position Y (for drag delta)
    pub prev_y: u16,
    /// When button was pressed
    pub press_time: Option<Instant>,
    /// Total distance traveled
    pub total_distance: f64,
    /// Is long press detected
    pub long_press_detected: bool,
    /// Is currently dragging
    pub is_dragging: bool,
    /// Last tap info for double-tap detection
    pub last_tap_time: Option<Instant>,
    /// Last tap X position
    pub last_tap_x: u16,
    /// Last tap Y position
    pub last_tap_y: u16,
    /// Last tap button
    pub last_tap_button: Option<MouseButton>,
    /// Cumulative pinch scale
    pub pinch_scale: f64,
}

impl Default for TrackingState {
    fn default() -> Self {
        Self {
            button_down: None,
            start_x: 0,
            start_y: 0,
            current_x: 0,
            current_y: 0,
            prev_x: 0,
            prev_y: 0,
            press_time: None,
            total_distance: 0.0,
            long_press_detected: false,
            is_dragging: false,
            last_tap_time: None,
            last_tap_x: 0,
            last_tap_y: 0,
            last_tap_button: None,
            pinch_scale: 1.0,
        }
    }
}

// =============================================================================
// Gesture Configuration
// =============================================================================

/// Configuration for gesture recognition
///
/// Defines thresholds and timing for detecting different types of gestures.
#[derive(Debug, Clone)]
pub struct GestureConfig {
    /// Minimum distance (in cells) for a swipe to be recognized
    pub swipe_threshold: u16,
    /// Maximum duration for a swipe (longer = drag)
    pub swipe_max_duration: Duration,
    /// Minimum velocity for a swipe
    pub swipe_min_velocity: f64,
    /// Duration threshold for long press
    pub long_press_duration: Duration,
    /// Minimum distance for drag recognition
    pub drag_threshold: u16,
    /// Scale factor per scroll step for pinch
    pub pinch_scale_per_scroll: f64,
    /// Maximum time between taps for double-tap
    pub double_tap_interval: Duration,
    /// Maximum distance between taps for double-tap
    pub double_tap_distance: u16,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            swipe_threshold: DEFAULT_SWIPE_THRESHOLD,
            swipe_max_duration: DEFAULT_SWIPE_MAX_DURATION,
            swipe_min_velocity: DEFAULT_SWIPE_MIN_VELOCITY,
            long_press_duration: DEFAULT_LONG_PRESS_DURATION,
            drag_threshold: DEFAULT_DRAG_THRESHOLD,
            pinch_scale_per_scroll: DEFAULT_PINCH_SCALE_PER_SCROLL,
            double_tap_interval: DEFAULT_DOUBLE_TAP_INTERVAL,
            double_tap_distance: DEFAULT_DOUBLE_TAP_DISTANCE,
        }
    }
}

// =============================================================================
// Gesture Event
// =============================================================================

/// High-level gesture event
#[derive(Debug, Clone)]
pub enum Gesture {
    /// Swipe gesture
    Swipe(SwipeGesture),
    /// Long press gesture
    LongPress(LongPressGesture),
    /// Drag gesture (start, move, or end)
    Drag(DragGesture),
    /// Pinch/zoom gesture
    Pinch(PinchGesture),
    /// Tap gesture
    Tap(TapGesture),
    /// Double tap gesture
    DoubleTap(TapGesture),
}

// =============================================================================
// Gesture Configuration
// =============================================================================
