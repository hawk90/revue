//! Gesture recognition tests

use revue::event::{
    DragGesture, Gesture, GestureConfig, GestureRecognizer, GestureState, LongPressGesture,
    MouseButton, MouseEvent, MouseEventKind, PinchDirection, PinchGesture, SwipeDirection,
    SwipeGesture, TapGesture,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

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
fn test_drag_gesture() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_drag_threshold(1);

    let drag_count = Arc::new(AtomicUsize::new(0));
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

#[test]
fn test_gesture_state_default() {
    let state = GestureState::default();
    assert_eq!(state, GestureState::Idle);
}

#[test]
fn test_gesture_state_clone() {
    let state = GestureState::Active;
    let cloned = state;
    assert_eq!(state, cloned);
}

#[test]
fn test_pinch_direction_clone() {
    let dir = PinchDirection::In;
    let cloned = dir;
    assert_eq!(dir, cloned);
}

#[test]
fn test_swipe_direction_clone() {
    let dir = SwipeDirection::Up;
    let cloned = dir;
    assert_eq!(dir, cloned);
}

#[test]
fn test_gesture_config_fields() {
    let config = GestureConfig {
        swipe_threshold: 5,
        swipe_max_duration: Duration::from_millis(400),
        swipe_min_velocity: 15.0,
        long_press_duration: Duration::from_millis(800),
        drag_threshold: 3,
        pinch_scale_per_scroll: 0.2,
        double_tap_interval: Duration::from_millis(400),
        double_tap_distance: 3,
    };

    assert_eq!(config.swipe_threshold, 5);
    assert_eq!(config.long_press_duration, Duration::from_millis(800));
}

#[test]
fn test_recognizer_with_config() {
    let config = GestureConfig {
        swipe_threshold: 10,
        ..GestureConfig::default()
    };
    let recognizer = GestureRecognizer::with_config(config);
    assert_eq!(recognizer.config().swipe_threshold, 10);
}

#[test]
fn test_recognizer_set_config() {
    let mut recognizer = GestureRecognizer::new();
    let config = GestureConfig {
        swipe_threshold: 10,
        ..GestureConfig::default()
    };
    recognizer.set_config(config);
    assert_eq!(recognizer.config().swipe_threshold, 10);
}

#[test]
fn test_recognizer_set_long_press_duration() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_long_press_duration(Duration::from_millis(1000));
    assert_eq!(
        recognizer.config().long_press_duration,
        Duration::from_millis(1000)
    );
}

#[test]
fn test_recognizer_set_swipe_threshold() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_swipe_threshold(10);
    assert_eq!(recognizer.config().swipe_threshold, 10);
}

#[test]
fn test_recognizer_set_drag_threshold() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_drag_threshold(5);
    assert_eq!(recognizer.config().drag_threshold, 5);
}

#[test]
fn test_recognizer_reset() {
    let mut recognizer = GestureRecognizer::new();

    // Press a button
    let down_event = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Reset should clear state
    recognizer.reset();

    // After reset, the button should be considered released
    let up_event = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Left));
    let result = recognizer.handle_mouse_event(&up_event);
    assert!(result.is_none()); // Should be None because button wasn't tracked
}

#[test]
fn test_recognizer_cancel_gesture() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_drag_threshold(1);

    // Start drag
    let down_event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    let drag_event = MouseEvent::new(20, 20, MouseEventKind::Drag(MouseButton::Left));
    recognizer.handle_mouse_event(&drag_event);

    // Cancel should emit a cancelled drag
    recognizer.cancel_gesture();

    // After cancel, state should be reset
    let up_event = MouseEvent::new(20, 20, MouseEventKind::Up(MouseButton::Left));
    let result = recognizer.handle_mouse_event(&up_event);
    assert!(result.is_none());
}

#[test]
fn test_recognizer_default() {
    let recognizer = GestureRecognizer::default();
    assert!(recognizer.is_enabled());
}

#[test]
fn test_swipe_gesture_negative_delta() {
    let swipe = SwipeGesture {
        direction: SwipeDirection::Left,
        start_x: 20,
        start_y: 10,
        end_x: 5,
        end_y: 10,
        distance: 15.0,
        velocity: 75.0,
        duration: Duration::from_millis(200),
        button: MouseButton::Left,
    };

    assert_eq!(swipe.delta_x(), -15);
    assert_eq!(swipe.delta_y(), 0);
}

#[test]
fn test_drag_gesture_negative_delta() {
    let drag = DragGesture {
        start_x: 20,
        start_y: 20,
        current_x: 10,
        current_y: 10,
        prev_x: 15,
        prev_y: 15,
        total_distance: 14.14,
        duration: Duration::from_millis(100),
        button: MouseButton::Left,
        state: GestureState::Active,
    };

    assert_eq!(drag.delta_x(), -5);
    assert_eq!(drag.delta_y(), -5);
    assert_eq!(drag.total_delta_x(), -10);
    assert_eq!(drag.total_delta_y(), -10);
}

#[test]
fn test_tap_gesture_fields() {
    let tap = TapGesture {
        x: 10,
        y: 20,
        button: MouseButton::Right,
        count: 2,
    };

    assert_eq!(tap.x, 10);
    assert_eq!(tap.y, 20);
    assert_eq!(tap.button, MouseButton::Right);
    assert_eq!(tap.count, 2);
}

#[test]
fn test_long_press_gesture_fields() {
    let gesture = LongPressGesture {
        x: 15,
        y: 25,
        duration: Duration::from_millis(600),
        button: MouseButton::Left,
    };

    assert_eq!(gesture.x, 15);
    assert_eq!(gesture.y, 25);
    assert_eq!(gesture.duration, Duration::from_millis(600));
    assert_eq!(gesture.button, MouseButton::Left);
}

#[test]
fn test_pinch_gesture_fields() {
    let pinch = PinchGesture {
        direction: PinchDirection::Out,
        x: 50,
        y: 50,
        scale: 1.1,
        cumulative_scale: 1.21,
    };

    assert_eq!(pinch.direction, PinchDirection::Out);
    assert_eq!(pinch.x, 50);
    assert_eq!(pinch.y, 50);
    assert_eq!(pinch.scale, 1.1);
    assert_eq!(pinch.cumulative_scale, 1.21);
}

#[test]
fn test_pinch_in_gesture() {
    let mut recognizer = GestureRecognizer::new();

    let pinched_in = Arc::new(AtomicBool::new(false));
    let pinched_clone = Arc::clone(&pinched_in);

    recognizer.on_pinch(move |pinch| {
        if pinch.direction == PinchDirection::In {
            pinched_clone.store(true, Ordering::SeqCst);
        }
    });

    // Ctrl+scroll down = pinch in
    let scroll_event = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollDown,
        ctrl: true,
        alt: false,
        shift: false,
    };
    recognizer.handle_mouse_event(&scroll_event);

    assert!(pinched_in.load(Ordering::SeqCst));
}

#[test]
fn test_scroll_without_ctrl() {
    let mut recognizer = GestureRecognizer::new();

    // Scroll without Ctrl should not trigger pinch
    let scroll_event = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollUp,
        ctrl: false,
        alt: false,
        shift: false,
    };
    let result = recognizer.handle_mouse_event(&scroll_event);
    assert!(result.is_none());
}

#[test]
fn test_double_tap() {
    let mut recognizer = GestureRecognizer::new();

    let double_tapped = Arc::new(AtomicBool::new(false));
    let dt_clone = Arc::clone(&double_tapped);

    recognizer.on_double_tap(move |tap| {
        assert_eq!(tap.count, 2);
        dt_clone.store(true, Ordering::SeqCst);
    });

    // First tap
    let down1 = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down1);
    let up1 = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up1);

    // Second tap (quickly)
    let down2 = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down2);
    let up2 = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up2);

    assert!(double_tapped.load(Ordering::SeqCst));
}

#[test]
fn test_generic_gesture_handler() {
    let mut recognizer = GestureRecognizer::new();

    let gesture_received = Arc::new(AtomicBool::new(false));
    let gr_clone = Arc::clone(&gesture_received);

    recognizer.on_gesture(move |_| {
        gr_clone.store(true, Ordering::SeqCst);
    });

    // Tap should trigger generic handler
    let down_event = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);
    let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up_event);

    assert!(gesture_received.load(Ordering::SeqCst));
}

#[test]
fn test_move_event() {
    let mut recognizer = GestureRecognizer::new();

    // Move events just update position, don't trigger gestures
    let move_event = MouseEvent {
        x: 20,
        y: 30,
        kind: MouseEventKind::Move,
        ctrl: false,
        alt: false,
        shift: false,
    };
    let result = recognizer.handle_mouse_event(&move_event);
    assert!(result.is_none());
}

#[test]
fn test_horizontal_scroll_events() {
    let mut recognizer = GestureRecognizer::new();

    // Horizontal scroll events don't trigger gestures currently
    let scroll_left = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollLeft,
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert!(recognizer.handle_mouse_event(&scroll_left).is_none());

    let scroll_right = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollRight,
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert!(recognizer.handle_mouse_event(&scroll_right).is_none());
}

#[test]
fn test_check_long_press_disabled() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_enabled(false);

    let result = recognizer.check_long_press();
    assert!(result.is_none());
}

#[test]
fn test_check_long_press_no_button_down() {
    let mut recognizer = GestureRecognizer::new();

    let result = recognizer.check_long_press();
    assert!(result.is_none());
}

#[test]
fn test_button_up_wrong_button() {
    let mut recognizer = GestureRecognizer::new();

    // Press left button
    let down_event = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Release right button (wrong button)
    let up_event = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Right));
    let result = recognizer.handle_mouse_event(&up_event);
    assert!(result.is_none());
}

#[test]
fn test_drag_wrong_button() {
    let mut recognizer = GestureRecognizer::new();

    // Press left button
    let down_event = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Drag with right button (wrong button)
    let drag_event = MouseEvent::new(20, 20, MouseEventKind::Drag(MouseButton::Right));
    let result = recognizer.handle_mouse_event(&drag_event);
    assert!(result.is_none());
}

#[test]
fn test_swipe_too_slow() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_swipe_threshold(2);
    // Set very high min velocity to make swipes fail
    let mut config = recognizer.config().clone();
    config.swipe_min_velocity = 100000.0;
    recognizer.set_config(config);

    // Press
    let down_event = MouseEvent::new(0, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Wait a bit to make it slow
    std::thread::sleep(Duration::from_millis(10));

    // Release (should be tap instead of swipe)
    let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    let result = recognizer.handle_mouse_event(&up_event);

    // Should be a tap, not a swipe
    assert!(matches!(result, Some(Gesture::Tap(_))));
}

// =============================================================================
