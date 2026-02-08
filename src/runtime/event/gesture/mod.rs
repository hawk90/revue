//! Gesture recognition module
//!
//! Provides gesture recognition for mouse interactions in terminal applications.

/// Gesture data types and callbacks
pub mod data;

/// Gesture recognizer implementation
pub mod recognizer;

/// Gesture recognition tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::MouseButton;
    use std::time::Duration;

    // Test that SwipeDirection variants exist and can be compared
    #[test]
    fn test_swipe_direction_variants() {
        // Can't test private calculate_swipe_direction function
        // Just verify the type exists
        let _ = SwipeDirection::Up;
        let _ = SwipeDirection::Down;
        let _ = SwipeDirection::Left;
        let _ = SwipeDirection::Right;
    }

    #[test]
    fn test_swipe_direction_is_vertical() {
        assert!(SwipeDirection::Up.is_vertical());
        assert!(SwipeDirection::Down.is_vertical());
        assert!(!SwipeDirection::Left.is_vertical());
        assert!(!SwipeDirection::Right.is_vertical());
    }

    #[test]
    fn test_swipe_direction_is_horizontal() {
        assert!(SwipeDirection::Left.is_horizontal());
        assert!(SwipeDirection::Right.is_horizontal());
        assert!(!SwipeDirection::Up.is_horizontal());
        assert!(!SwipeDirection::Down.is_horizontal());
    }

    #[test]
    fn test_swipe_gesture_delta_x() {
        let gesture = SwipeGesture {
            direction: SwipeDirection::Right,
            start_x: 10,
            start_y: 20,
            end_x: 50,
            end_y: 20,
            distance: 40.0,
            velocity: 100.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
        };
        assert_eq!(gesture.delta_x(), 40);
    }

    #[test]
    fn test_swipe_gesture_delta_y() {
        let gesture = SwipeGesture {
            direction: SwipeDirection::Down,
            start_x: 20,
            start_y: 10,
            end_x: 20,
            end_y: 50,
            distance: 40.0,
            velocity: 100.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
        };
        assert_eq!(gesture.delta_y(), 40);
    }

    #[test]
    fn test_swipe_gesture_delta_x_negative() {
        let gesture = SwipeGesture {
            direction: SwipeDirection::Left,
            start_x: 50,
            start_y: 20,
            end_x: 10,
            end_y: 20,
            distance: 40.0,
            velocity: 100.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
        };
        assert_eq!(gesture.delta_x(), -40);
    }

    #[test]
    fn test_swipe_gesture_delta_y_negative() {
        let gesture = SwipeGesture {
            direction: SwipeDirection::Up,
            start_x: 20,
            start_y: 50,
            end_x: 20,
            end_y: 10,
            distance: 40.0,
            velocity: 100.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
        };
        assert_eq!(gesture.delta_y(), -40);
    }

    #[test]
    fn test_drag_gesture_delta_x() {
        let gesture = DragGesture {
            start_x: 10,
            start_y: 20,
            current_x: 50,
            current_y: 20,
            prev_x: 30,
            prev_y: 20,
            total_distance: 40.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
            state: GestureState::Active,
        };
        assert_eq!(gesture.delta_x(), 20);
    }

    #[test]
    fn test_drag_gesture_delta_y() {
        let gesture = DragGesture {
            start_x: 20,
            start_y: 10,
            current_x: 20,
            current_y: 50,
            prev_x: 20,
            prev_y: 30,
            total_distance: 40.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
            state: GestureState::Active,
        };
        assert_eq!(gesture.delta_y(), 20);
    }

    #[test]
    fn test_drag_gesture_total_delta_x() {
        let gesture = DragGesture {
            start_x: 10,
            start_y: 20,
            current_x: 50,
            current_y: 20,
            prev_x: 30,
            prev_y: 20,
            total_distance: 40.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
            state: GestureState::Active,
        };
        assert_eq!(gesture.total_delta_x(), 40);
    }

    #[test]
    fn test_drag_gesture_total_delta_y() {
        let gesture = DragGesture {
            start_x: 20,
            start_y: 10,
            current_x: 20,
            current_y: 50,
            prev_x: 20,
            prev_y: 30,
            total_distance: 40.0,
            duration: Duration::from_millis(400),
            button: MouseButton::Left,
            state: GestureState::Active,
        };
        assert_eq!(gesture.total_delta_y(), 40);
    }

    #[test]
    fn test_gesture_state_default() {
        assert_eq!(GestureState::default(), GestureState::Idle);
    }

    #[test]
    fn test_gesture_state_variants() {
        let _ = GestureState::Idle;
        let _ = GestureState::Possible;
        let _ = GestureState::Active;
        let _ = GestureState::Ended;
        let _ = GestureState::Cancelled;
    }

    #[test]
    fn test_pinch_direction_variants() {
        let _ = PinchDirection::In;
        let _ = PinchDirection::Out;
        assert_eq!(PinchDirection::In, PinchDirection::In);
        assert_ne!(PinchDirection::In, PinchDirection::Out);
    }

    #[test]
    fn test_gesture_config_default() {
        let config = GestureConfig::default();
        assert_eq!(config.swipe_threshold, 3);
        assert_eq!(config.long_press_duration, Duration::from_millis(500));
        assert_eq!(config.drag_threshold, 2);
        assert_eq!(config.pinch_scale_per_scroll, 0.1);
        assert_eq!(config.double_tap_interval, Duration::from_millis(300));
        assert_eq!(config.double_tap_distance, 2);
    }

    #[test]
    fn test_tracking_state_default() {
        let state = TrackingState::default();
        assert!(state.button_down.is_none());
        assert_eq!(state.start_x, 0);
        assert_eq!(state.start_y, 0);
        assert_eq!(state.current_x, 0);
        assert_eq!(state.current_y, 0);
        assert_eq!(state.prev_x, 0);
        assert_eq!(state.prev_y, 0);
        assert!(state.press_time.is_none());
        assert_eq!(state.total_distance, 0.0);
        assert!(!state.long_press_detected);
        assert!(!state.is_dragging);
        assert!(state.last_tap_time.is_none());
        assert_eq!(state.last_tap_x, 0);
        assert_eq!(state.last_tap_y, 0);
        assert!(state.last_tap_button.is_none());
        assert_eq!(state.pinch_scale, 1.0);
    }

    #[test]
    fn test_gesture_enum_variants() {
        // Test that all Gesture variants can be constructed
        let swipe_gesture = SwipeGesture {
            direction: SwipeDirection::Up,
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 10,
            distance: 10.0,
            velocity: 50.0,
            duration: Duration::from_millis(200),
            button: MouseButton::Left,
        };

        let long_press_gesture = LongPressGesture {
            x: 10,
            y: 20,
            duration: Duration::from_millis(600),
            button: MouseButton::Left,
        };

        let drag_gesture = DragGesture {
            start_x: 0,
            start_y: 0,
            current_x: 10,
            current_y: 10,
            prev_x: 5,
            prev_y: 5,
            total_distance: 14.14,
            duration: Duration::from_millis(100),
            button: MouseButton::Left,
            state: GestureState::Active,
        };

        let pinch_gesture = PinchGesture {
            direction: PinchDirection::Out,
            x: 50,
            y: 50,
            scale: 1.2,
            cumulative_scale: 1.5,
        };

        let tap_gesture = TapGesture {
            x: 10,
            y: 20,
            button: MouseButton::Left,
            count: 1,
        };

        let double_tap_gesture = TapGesture {
            x: 10,
            y: 20,
            button: MouseButton::Left,
            count: 2,
        };

        let _ = Gesture::Swipe(swipe_gesture);
        let _ = Gesture::LongPress(long_press_gesture);
        let _ = Gesture::Drag(drag_gesture);
        let _ = Gesture::Pinch(pinch_gesture);
        let _ = Gesture::Tap(tap_gesture);
        let _ = Gesture::DoubleTap(double_tap_gesture);
    }
}

/// Gesture type definitions
pub mod types;

pub use data::*;
pub use recognizer::*;
pub use types::*;
