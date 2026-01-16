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

use super::{MouseButton, MouseEvent, MouseEventKind};
use std::sync::Arc;
use std::time::{Duration, Instant};

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

/// Configuration for gesture recognition
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
            swipe_threshold: 3,
            swipe_max_duration: Duration::from_millis(300),
            swipe_min_velocity: 10.0,
            long_press_duration: Duration::from_millis(500),
            drag_threshold: 2,
            pinch_scale_per_scroll: 0.1,
            double_tap_interval: Duration::from_millis(300),
            double_tap_distance: 2,
        }
    }
}

// =============================================================================
// Gesture Tracking State
// =============================================================================

/// Internal state for tracking gestures
#[derive(Debug, Clone)]
struct TrackingState {
    /// Is a button currently pressed
    button_down: Option<MouseButton>,
    /// Start position
    start_x: u16,
    start_y: u16,
    /// Current position
    current_x: u16,
    current_y: u16,
    /// Previous position (for drag delta)
    prev_x: u16,
    prev_y: u16,
    /// When button was pressed
    press_time: Option<Instant>,
    /// Total distance traveled
    total_distance: f64,
    /// Is long press detected
    long_press_detected: bool,
    /// Is currently dragging
    is_dragging: bool,
    /// Last tap info for double-tap detection
    last_tap_time: Option<Instant>,
    last_tap_x: u16,
    last_tap_y: u16,
    last_tap_button: Option<MouseButton>,
    /// Cumulative pinch scale
    pinch_scale: f64,
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
// Gesture Callbacks
// =============================================================================

type SwipeCallback = Arc<dyn Fn(&SwipeGesture) + Send + Sync>;
type LongPressCallback = Arc<dyn Fn(&LongPressGesture) + Send + Sync>;
type DragCallback = Arc<dyn Fn(&DragGesture) + Send + Sync>;
type PinchCallback = Arc<dyn Fn(&PinchGesture) + Send + Sync>;
type TapCallback = Arc<dyn Fn(&TapGesture) + Send + Sync>;
type GestureCallback = Arc<dyn Fn(&Gesture) + Send + Sync>;

// =============================================================================
// Gesture Recognizer
// =============================================================================

/// Gesture recognizer for mouse events
pub struct GestureRecognizer {
    /// Configuration
    config: GestureConfig,
    /// Internal tracking state
    state: TrackingState,
    /// Swipe callbacks
    swipe_handlers: Vec<SwipeCallback>,
    /// Long press callbacks
    long_press_handlers: Vec<LongPressCallback>,
    /// Drag callbacks
    drag_handlers: Vec<DragCallback>,
    /// Pinch callbacks
    pinch_handlers: Vec<PinchCallback>,
    /// Tap callbacks
    tap_handlers: Vec<TapCallback>,
    /// Double tap callbacks
    double_tap_handlers: Vec<TapCallback>,
    /// Generic gesture callbacks
    gesture_handlers: Vec<GestureCallback>,
    /// Whether gesture recognition is enabled
    enabled: bool,
}

impl GestureRecognizer {
    /// Create a new gesture recognizer
    pub fn new() -> Self {
        Self {
            config: GestureConfig::default(),
            state: TrackingState::default(),
            swipe_handlers: Vec::new(),
            long_press_handlers: Vec::new(),
            drag_handlers: Vec::new(),
            pinch_handlers: Vec::new(),
            tap_handlers: Vec::new(),
            double_tap_handlers: Vec::new(),
            gesture_handlers: Vec::new(),
            enabled: true,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: GestureConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    // -------------------------------------------------------------------------
    // Configuration
    // -------------------------------------------------------------------------

    /// Set the configuration
    pub fn set_config(&mut self, config: GestureConfig) {
        self.config = config;
    }

    /// Get the configuration
    pub fn config(&self) -> &GestureConfig {
        &self.config
    }

    /// Set long press duration threshold
    pub fn set_long_press_duration(&mut self, duration: Duration) {
        self.config.long_press_duration = duration;
    }

    /// Set swipe threshold distance
    pub fn set_swipe_threshold(&mut self, threshold: u16) {
        self.config.swipe_threshold = threshold;
    }

    /// Set drag threshold distance
    pub fn set_drag_threshold(&mut self, threshold: u16) {
        self.config.drag_threshold = threshold;
    }

    /// Enable or disable gesture recognition
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.cancel_gesture();
        }
    }

    /// Check if gesture recognition is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    // -------------------------------------------------------------------------
    // Handler Registration
    // -------------------------------------------------------------------------

    /// Register a swipe handler
    pub fn on_swipe<F>(&mut self, handler: F)
    where
        F: Fn(&SwipeGesture) + Send + Sync + 'static,
    {
        self.swipe_handlers.push(Arc::new(handler));
    }

    /// Register a long press handler
    pub fn on_long_press<F>(&mut self, handler: F)
    where
        F: Fn(&LongPressGesture) + Send + Sync + 'static,
    {
        self.long_press_handlers.push(Arc::new(handler));
    }

    /// Register a drag handler
    pub fn on_drag<F>(&mut self, handler: F)
    where
        F: Fn(&DragGesture) + Send + Sync + 'static,
    {
        self.drag_handlers.push(Arc::new(handler));
    }

    /// Register a pinch handler
    pub fn on_pinch<F>(&mut self, handler: F)
    where
        F: Fn(&PinchGesture) + Send + Sync + 'static,
    {
        self.pinch_handlers.push(Arc::new(handler));
    }

    /// Register a tap handler
    pub fn on_tap<F>(&mut self, handler: F)
    where
        F: Fn(&TapGesture) + Send + Sync + 'static,
    {
        self.tap_handlers.push(Arc::new(handler));
    }

    /// Register a double-tap handler
    pub fn on_double_tap<F>(&mut self, handler: F)
    where
        F: Fn(&TapGesture) + Send + Sync + 'static,
    {
        self.double_tap_handlers.push(Arc::new(handler));
    }

    /// Register a generic gesture handler
    pub fn on_gesture<F>(&mut self, handler: F)
    where
        F: Fn(&Gesture) + Send + Sync + 'static,
    {
        self.gesture_handlers.push(Arc::new(handler));
    }

    /// Clear all handlers
    pub fn clear_handlers(&mut self) {
        self.swipe_handlers.clear();
        self.long_press_handlers.clear();
        self.drag_handlers.clear();
        self.pinch_handlers.clear();
        self.tap_handlers.clear();
        self.double_tap_handlers.clear();
        self.gesture_handlers.clear();
    }

    // -------------------------------------------------------------------------
    // Event Processing
    // -------------------------------------------------------------------------

    /// Process a mouse event and detect gestures
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> Option<Gesture> {
        if !self.enabled {
            return None;
        }

        match event.kind {
            MouseEventKind::Down(button) => self.handle_button_down(event.x, event.y, button),
            MouseEventKind::Up(button) => self.handle_button_up(event.x, event.y, button),
            MouseEventKind::Drag(button) => self.handle_drag(event.x, event.y, button),
            MouseEventKind::Move => self.handle_move(event.x, event.y),
            MouseEventKind::ScrollUp => self.handle_scroll(event.x, event.y, event.ctrl, true),
            MouseEventKind::ScrollDown => self.handle_scroll(event.x, event.y, event.ctrl, false),
            // Horizontal scroll events - currently no gesture mapped
            MouseEventKind::ScrollLeft | MouseEventKind::ScrollRight => None,
        }
    }

    /// Check for long press (call this periodically, e.g., on tick)
    pub fn check_long_press(&mut self) -> Option<Gesture> {
        if !self.enabled {
            return None;
        }

        if let (Some(button), Some(press_time)) = (self.state.button_down, self.state.press_time) {
            if !self.state.long_press_detected && !self.state.is_dragging {
                let elapsed = press_time.elapsed();
                if elapsed >= self.config.long_press_duration {
                    self.state.long_press_detected = true;

                    let gesture = LongPressGesture {
                        x: self.state.start_x,
                        y: self.state.start_y,
                        duration: elapsed,
                        button,
                    };

                    self.emit_long_press(&gesture);
                    return Some(Gesture::LongPress(gesture));
                }
            }
        }

        None
    }

    /// Cancel any ongoing gesture
    pub fn cancel_gesture(&mut self) {
        if self.state.is_dragging {
            if let Some(button) = self.state.button_down {
                let gesture = DragGesture {
                    start_x: self.state.start_x,
                    start_y: self.state.start_y,
                    current_x: self.state.current_x,
                    current_y: self.state.current_y,
                    prev_x: self.state.prev_x,
                    prev_y: self.state.prev_y,
                    total_distance: self.state.total_distance,
                    duration: self
                        .state
                        .press_time
                        .map(|t| t.elapsed())
                        .unwrap_or_default(),
                    button,
                    state: GestureState::Cancelled,
                };
                self.emit_drag(&gesture);
            }
        }

        self.state = TrackingState::default();
    }

    /// Reset state
    pub fn reset(&mut self) {
        self.state = TrackingState::default();
    }

    // -------------------------------------------------------------------------
    // Internal Event Handlers
    // -------------------------------------------------------------------------

    fn handle_button_down(&mut self, x: u16, y: u16, button: MouseButton) -> Option<Gesture> {
        self.state.button_down = Some(button);
        self.state.start_x = x;
        self.state.start_y = y;
        self.state.current_x = x;
        self.state.current_y = y;
        self.state.prev_x = x;
        self.state.prev_y = y;
        self.state.press_time = Some(Instant::now());
        self.state.total_distance = 0.0;
        self.state.long_press_detected = false;
        self.state.is_dragging = false;

        None
    }

    fn handle_button_up(&mut self, x: u16, y: u16, button: MouseButton) -> Option<Gesture> {
        // Check if this matches the button that was pressed
        if self.state.button_down != Some(button) {
            return None;
        }

        let press_time = self.state.press_time?;
        let elapsed = press_time.elapsed();

        // Update final position
        self.state.current_x = x;
        self.state.current_y = y;

        let gesture = if self.state.is_dragging {
            // End drag
            let drag = DragGesture {
                start_x: self.state.start_x,
                start_y: self.state.start_y,
                current_x: x,
                current_y: y,
                prev_x: self.state.prev_x,
                prev_y: self.state.prev_y,
                total_distance: self.state.total_distance,
                duration: elapsed,
                button,
                state: GestureState::Ended,
            };
            self.emit_drag(&drag);
            Some(Gesture::Drag(drag))
        } else if self.state.long_press_detected {
            // Long press already handled
            None
        } else {
            // Check for swipe or tap
            let dx = (x as i32 - self.state.start_x as i32).abs();
            let dy = (y as i32 - self.state.start_y as i32).abs();
            let distance = ((dx * dx + dy * dy) as f64).sqrt();

            if distance >= self.config.swipe_threshold as f64
                && elapsed <= self.config.swipe_max_duration
            {
                // Swipe detected
                let velocity = distance / elapsed.as_secs_f64();

                if velocity >= self.config.swipe_min_velocity {
                    let direction = Self::calculate_swipe_direction(
                        self.state.start_x,
                        self.state.start_y,
                        x,
                        y,
                    );

                    let swipe = SwipeGesture {
                        direction,
                        start_x: self.state.start_x,
                        start_y: self.state.start_y,
                        end_x: x,
                        end_y: y,
                        distance,
                        velocity,
                        duration: elapsed,
                        button,
                    };

                    self.emit_swipe(&swipe);
                    Some(Gesture::Swipe(swipe))
                } else {
                    self.emit_tap(x, y, button)
                }
            } else {
                // Tap
                self.emit_tap(x, y, button)
            }
        };

        // Reset state
        self.state.button_down = None;
        self.state.press_time = None;
        self.state.is_dragging = false;

        gesture
    }

    fn handle_drag(&mut self, x: u16, y: u16, button: MouseButton) -> Option<Gesture> {
        if self.state.button_down != Some(button) {
            return None;
        }

        // Calculate distance from last position
        let dx = (x as i32 - self.state.current_x as i32).abs();
        let dy = (y as i32 - self.state.current_y as i32).abs();
        let step_distance = ((dx * dx + dy * dy) as f64).sqrt();

        // Update positions
        self.state.prev_x = self.state.current_x;
        self.state.prev_y = self.state.current_y;
        self.state.current_x = x;
        self.state.current_y = y;
        self.state.total_distance += step_distance;

        // Check if we should start dragging
        if !self.state.is_dragging {
            let total_dx = (x as i32 - self.state.start_x as i32).abs();
            let total_dy = (y as i32 - self.state.start_y as i32).abs();
            let total_distance = ((total_dx * total_dx + total_dy * total_dy) as f64).sqrt();

            if total_distance >= self.config.drag_threshold as f64 {
                self.state.is_dragging = true;

                // Emit drag start
                let drag = DragGesture {
                    start_x: self.state.start_x,
                    start_y: self.state.start_y,
                    current_x: x,
                    current_y: y,
                    prev_x: self.state.prev_x,
                    prev_y: self.state.prev_y,
                    total_distance: self.state.total_distance,
                    duration: self
                        .state
                        .press_time
                        .map(|t| t.elapsed())
                        .unwrap_or_default(),
                    button,
                    state: GestureState::Active,
                };

                self.emit_drag(&drag);
                return Some(Gesture::Drag(drag));
            }
        } else {
            // Continue drag
            let drag = DragGesture {
                start_x: self.state.start_x,
                start_y: self.state.start_y,
                current_x: x,
                current_y: y,
                prev_x: self.state.prev_x,
                prev_y: self.state.prev_y,
                total_distance: self.state.total_distance,
                duration: self
                    .state
                    .press_time
                    .map(|t| t.elapsed())
                    .unwrap_or_default(),
                button,
                state: GestureState::Active,
            };

            self.emit_drag(&drag);
            return Some(Gesture::Drag(drag));
        }

        None
    }

    fn handle_move(&mut self, x: u16, y: u16) -> Option<Gesture> {
        // Just update current position for hover tracking
        self.state.current_x = x;
        self.state.current_y = y;
        None
    }

    fn handle_scroll(&mut self, x: u16, y: u16, ctrl: bool, up: bool) -> Option<Gesture> {
        if ctrl {
            // Ctrl+scroll = pinch gesture
            let direction = if up {
                PinchDirection::Out
            } else {
                PinchDirection::In
            };

            let scale_delta = if up {
                1.0 + self.config.pinch_scale_per_scroll
            } else {
                1.0 - self.config.pinch_scale_per_scroll
            };

            self.state.pinch_scale *= scale_delta;

            let gesture = PinchGesture {
                direction,
                x,
                y,
                scale: scale_delta,
                cumulative_scale: self.state.pinch_scale,
            };

            self.emit_pinch(&gesture);
            return Some(Gesture::Pinch(gesture));
        }

        None
    }

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    fn calculate_swipe_direction(
        start_x: u16,
        start_y: u16,
        end_x: u16,
        end_y: u16,
    ) -> SwipeDirection {
        let dx = end_x as i32 - start_x as i32;
        let dy = end_y as i32 - start_y as i32;

        // Determine primary direction
        if dx.abs() > dy.abs() {
            if dx > 0 {
                SwipeDirection::Right
            } else {
                SwipeDirection::Left
            }
        } else if dy > 0 {
            SwipeDirection::Down
        } else {
            SwipeDirection::Up
        }
    }

    fn emit_tap(&mut self, x: u16, y: u16, button: MouseButton) -> Option<Gesture> {
        let now = Instant::now();

        // Check for double tap
        if let Some(last_time) = self.state.last_tap_time {
            if now.duration_since(last_time) <= self.config.double_tap_interval
                && self.state.last_tap_button == Some(button)
            {
                let dx = (x as i32 - self.state.last_tap_x as i32).unsigned_abs() as u16;
                let dy = (y as i32 - self.state.last_tap_y as i32).unsigned_abs() as u16;

                if dx <= self.config.double_tap_distance && dy <= self.config.double_tap_distance {
                    // Double tap
                    self.state.last_tap_time = None; // Reset to prevent triple-tap as double
                    let tap = TapGesture {
                        x,
                        y,
                        button,
                        count: 2,
                    };

                    self.emit_double_tap(&tap);
                    return Some(Gesture::DoubleTap(tap));
                }
            }
        }

        // Single tap
        self.state.last_tap_time = Some(now);
        self.state.last_tap_x = x;
        self.state.last_tap_y = y;
        self.state.last_tap_button = Some(button);

        let tap = TapGesture {
            x,
            y,
            button,
            count: 1,
        };

        for handler in &self.tap_handlers {
            handler(&tap);
        }

        let gesture = Gesture::Tap(tap.clone());
        for handler in &self.gesture_handlers {
            handler(&gesture);
        }

        Some(gesture)
    }

    fn emit_swipe(&self, gesture: &SwipeGesture) {
        for handler in &self.swipe_handlers {
            handler(gesture);
        }

        let g = Gesture::Swipe(gesture.clone());
        for handler in &self.gesture_handlers {
            handler(&g);
        }
    }

    fn emit_long_press(&self, gesture: &LongPressGesture) {
        for handler in &self.long_press_handlers {
            handler(gesture);
        }

        let g = Gesture::LongPress(gesture.clone());
        for handler in &self.gesture_handlers {
            handler(&g);
        }
    }

    fn emit_drag(&self, gesture: &DragGesture) {
        for handler in &self.drag_handlers {
            handler(gesture);
        }

        let g = Gesture::Drag(gesture.clone());
        for handler in &self.gesture_handlers {
            handler(&g);
        }
    }

    fn emit_pinch(&self, gesture: &PinchGesture) {
        for handler in &self.pinch_handlers {
            handler(gesture);
        }

        let g = Gesture::Pinch(gesture.clone());
        for handler in &self.gesture_handlers {
            handler(&g);
        }
    }

    fn emit_double_tap(&self, gesture: &TapGesture) {
        for handler in &self.double_tap_handlers {
            handler(gesture);
        }

        let g = Gesture::DoubleTap(gesture.clone());
        for handler in &self.gesture_handlers {
            handler(&g);
        }
    }
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_swipe_direction() {
        assert!(SwipeDirection::Up.is_vertical());
        assert!(SwipeDirection::Down.is_vertical());
        assert!(SwipeDirection::Left.is_horizontal());
        assert!(SwipeDirection::Right.is_horizontal());
    }

    #[test]
    fn test_gesture_config_default() {
        let config = GestureConfig::default();
        assert_eq!(config.swipe_threshold, 3);
        assert_eq!(config.long_press_duration, Duration::from_millis(500));
    }

    #[test]
    fn test_recognizer_creation() {
        let recognizer = GestureRecognizer::new();
        assert!(recognizer.is_enabled());
    }

    #[test]
    fn test_recognizer_enable_disable() {
        let mut recognizer = GestureRecognizer::new();
        assert!(recognizer.is_enabled());

        recognizer.set_enabled(false);
        assert!(!recognizer.is_enabled());

        recognizer.set_enabled(true);
        assert!(recognizer.is_enabled());
    }

    #[test]
    fn test_tap_gesture() {
        let mut recognizer = GestureRecognizer::new();
        let tapped = Arc::new(AtomicBool::new(false));
        let tapped_clone = Arc::clone(&tapped);

        recognizer.on_tap(move |tap| {
            assert_eq!(tap.x, 10);
            assert_eq!(tap.y, 5);
            tapped_clone.store(true, Ordering::SeqCst);
        });

        // Press
        let down_event = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
        recognizer.handle_mouse_event(&down_event);

        // Release at same position (tap)
        let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
        recognizer.handle_mouse_event(&up_event);

        assert!(tapped.load(Ordering::SeqCst));
    }

    #[test]
    fn test_swipe_gesture() {
        let mut recognizer = GestureRecognizer::new();
        recognizer.set_swipe_threshold(2);

        let swiped = Arc::new(AtomicBool::new(false));
        let swiped_clone = Arc::clone(&swiped);

        recognizer.on_swipe(move |swipe| {
            assert_eq!(swipe.direction, SwipeDirection::Right);
            swiped_clone.store(true, Ordering::SeqCst);
        });

        // Press
        let down_event = MouseEvent::new(0, 5, MouseEventKind::Down(MouseButton::Left));
        recognizer.handle_mouse_event(&down_event);

        // Release far to the right (swipe)
        let up_event = MouseEvent::new(20, 5, MouseEventKind::Up(MouseButton::Left));
        recognizer.handle_mouse_event(&up_event);

        assert!(swiped.load(Ordering::SeqCst));
    }

    #[test]
    fn test_calculate_swipe_direction() {
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(0, 0, 10, 0),
            SwipeDirection::Right
        );
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(10, 0, 0, 0),
            SwipeDirection::Left
        );
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(0, 0, 0, 10),
            SwipeDirection::Down
        );
        assert_eq!(
            GestureRecognizer::calculate_swipe_direction(0, 10, 0, 0),
            SwipeDirection::Up
        );
    }

    #[test]
    fn test_drag_gesture() {
        let mut recognizer = GestureRecognizer::new();
        recognizer.set_drag_threshold(1);

        let drag_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let count_clone = Arc::clone(&drag_count);

        recognizer.on_drag(move |_| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Press
        let down_event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
        recognizer.handle_mouse_event(&down_event);

        // Drag
        let drag_event = MouseEvent::new(10, 10, MouseEventKind::Drag(MouseButton::Left));
        recognizer.handle_mouse_event(&drag_event);

        // Release
        let up_event = MouseEvent::new(15, 15, MouseEventKind::Up(MouseButton::Left));
        recognizer.handle_mouse_event(&up_event);

        // Should have at least 2 drag events (start and end)
        assert!(drag_count.load(Ordering::SeqCst) >= 2);
    }

    #[test]
    fn test_pinch_gesture() {
        let mut recognizer = GestureRecognizer::new();

        let pinched = Arc::new(AtomicBool::new(false));
        let pinched_clone = Arc::clone(&pinched);

        recognizer.on_pinch(move |pinch| {
            assert_eq!(pinch.direction, PinchDirection::Out);
            pinched_clone.store(true, Ordering::SeqCst);
        });

        // Ctrl+scroll up = pinch out
        let scroll_event = MouseEvent {
            x: 10,
            y: 10,
            kind: MouseEventKind::ScrollUp,
            ctrl: true,
            alt: false,
            shift: false,
        };
        recognizer.handle_mouse_event(&scroll_event);

        assert!(pinched.load(Ordering::SeqCst));
    }

    #[test]
    fn test_long_press() {
        let mut recognizer = GestureRecognizer::new();
        recognizer.set_long_press_duration(Duration::from_millis(10));

        let long_pressed = Arc::new(AtomicBool::new(false));
        let lp_clone = Arc::clone(&long_pressed);

        recognizer.on_long_press(move |_| {
            lp_clone.store(true, Ordering::SeqCst);
        });

        // Press
        let down_event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
        recognizer.handle_mouse_event(&down_event);

        // Wait for long press
        std::thread::sleep(Duration::from_millis(20));

        // Check for long press
        recognizer.check_long_press();

        assert!(long_pressed.load(Ordering::SeqCst));
    }

    #[test]
    fn test_gesture_data_methods() {
        let swipe = SwipeGesture {
            direction: SwipeDirection::Right,
            start_x: 0,
            start_y: 10,
            end_x: 20,
            end_y: 15,
            distance: 21.2,
            velocity: 100.0,
            duration: Duration::from_millis(200),
            button: MouseButton::Left,
        };

        assert_eq!(swipe.delta_x(), 20);
        assert_eq!(swipe.delta_y(), 5);

        let drag = DragGesture {
            start_x: 0,
            start_y: 0,
            current_x: 10,
            current_y: 10,
            prev_x: 8,
            prev_y: 8,
            total_distance: 14.14,
            duration: Duration::from_millis(100),
            button: MouseButton::Left,
            state: GestureState::Active,
        };

        assert_eq!(drag.delta_x(), 2);
        assert_eq!(drag.delta_y(), 2);
        assert_eq!(drag.total_delta_x(), 10);
        assert_eq!(drag.total_delta_y(), 10);
    }

    #[test]
    fn test_recognizer_disabled() {
        let mut recognizer = GestureRecognizer::new();
        recognizer.set_enabled(false);

        let tapped = Arc::new(AtomicBool::new(false));
        let tapped_clone = Arc::clone(&tapped);

        recognizer.on_tap(move |_| {
            tapped_clone.store(true, Ordering::SeqCst);
        });

        let down_event = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
        recognizer.handle_mouse_event(&down_event);

        let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
        recognizer.handle_mouse_event(&up_event);

        // Should not have been called because recognizer is disabled
        assert!(!tapped.load(Ordering::SeqCst));
    }

    #[test]
    fn test_clear_handlers() {
        let mut recognizer = GestureRecognizer::new();

        recognizer.on_tap(|_| {});
        recognizer.on_swipe(|_| {});
        recognizer.on_drag(|_| {});

        recognizer.clear_handlers();

        // After clearing, handlers should not fire
        // (This is a basic check - actual verification would need more setup)
    }
}
