use super::{data::*, types::*};
use crate::event::{MouseButton, MouseEvent, MouseEventKind};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Gesture recognizer for terminal mouse interactions
///
/// Detects high-level gestures like swipes, long-presses, drags, pinches, and taps
/// from raw mouse events.
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

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
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
