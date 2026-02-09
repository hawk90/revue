//! Helper functions for creating mock objects

use crate::event::{Key, KeyEvent, MouseEvent, MouseEventKind};
use crate::testing::mock::capture::RenderCapture;
use crate::testing::mock::simulator::EventSimulator;
use crate::testing::mock::terminal::MockTerminal;
use crate::testing::mock::time::MockTime;

/// Create a mock key event
pub fn mock_key(key: Key) -> KeyEvent {
    KeyEvent::new(key)
}

/// Create a mock Ctrl+key event
pub fn mock_ctrl_key(key: Key) -> KeyEvent {
    KeyEvent::ctrl(key)
}

/// Create a mock Alt+key event
pub fn mock_alt_key(key: Key) -> KeyEvent {
    KeyEvent::alt(key)
}

/// Create a mock mouse click event
pub fn mock_click(x: u16, y: u16) -> MouseEvent {
    use crate::event::MouseButton;
    MouseEvent::new(x, y, MouseEventKind::Down(MouseButton::Left))
}

/// Create a mock mouse event
pub fn mock_mouse(x: u16, y: u16, kind: MouseEventKind) -> MouseEvent {
    MouseEvent::new(x, y, kind)
}

/// Create a mock terminal
pub fn mock_terminal(width: u16, height: u16) -> MockTerminal {
    MockTerminal::new(width, height)
}

/// Create a mock time controller
pub fn mock_time() -> MockTime {
    MockTime::new()
}

/// Create an event simulator
pub fn simulate_user() -> EventSimulator {
    EventSimulator::new()
}

/// Capture render output
pub fn capture_render(width: u16, height: u16) -> RenderCapture {
    RenderCapture::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // =========================================================================
    // mock_key tests
    // =========================================================================

    #[test]
    fn test_mock_key_function_creates_key_event() {
        let key_event = mock_key(Key::Char('a'));
        assert_eq!(key_event.key, Key::Char('a'));
    }

    #[test]
    fn test_mock_key_with_special_key() {
        let key_event = mock_key(Key::Enter);
        assert_eq!(key_event.key, Key::Enter);
    }

    #[test]
    fn test_mock_key_no_modifiers() {
        let key_event = mock_key(Key::Char('b'));
        assert!(!key_event.ctrl);
        assert!(!key_event.alt);
        assert!(!key_event.shift);
    }

    // =========================================================================
    // mock_ctrl_key tests
    // =========================================================================

    #[test]
    fn test_mock_ctrl_key_function_has_ctrl_modifier() {
        let key_event = mock_ctrl_key(Key::Char('c'));
        assert!(key_event.ctrl);
        assert_eq!(key_event.key, Key::Char('c'));
    }

    #[test]
    fn test_mock_ctrl_key_no_other_modifiers() {
        let key_event = mock_ctrl_key(Key::Char('d'));
        assert!(key_event.ctrl);
        assert!(!key_event.alt);
        assert!(!key_event.shift);
    }

    // =========================================================================
    // mock_alt_key tests
    // =========================================================================

    #[test]
    fn test_mock_alt_key_function_has_alt_modifier() {
        let key_event = mock_alt_key(Key::Char('x'));
        assert!(key_event.alt);
        assert_eq!(key_event.key, Key::Char('x'));
    }

    #[test]
    fn test_mock_alt_key_no_other_modifiers() {
        let key_event = mock_alt_key(Key::Char('y'));
        assert!(key_event.alt);
        assert!(!key_event.ctrl);
        assert!(!key_event.shift);
    }

    // =========================================================================
    // mock_click tests
    // =========================================================================

    #[test]
    fn test_mock_click_function_has_correct_position() {
        let mouse_event = mock_click(10, 20);
        assert_eq!(mouse_event.x, 10);
        assert_eq!(mouse_event.y, 20);
    }

    #[test]
    fn test_mock_click_is_left_button_down() {
        use crate::event::MouseButton;
        let mouse_event = mock_click(5, 15);
        assert_eq!(mouse_event.kind, MouseEventKind::Down(MouseButton::Left));
    }

    // =========================================================================
    // mock_mouse tests
    // =========================================================================

    #[test]
    fn test_mock_mouse_function_with_custom_kind() {
        use crate::event::MouseButton;
        let mouse_event = mock_mouse(5, 10, MouseEventKind::Up(MouseButton::Left));
        assert_eq!(mouse_event.x, 5);
        assert_eq!(mouse_event.y, 10);
        assert_eq!(mouse_event.kind, MouseEventKind::Up(MouseButton::Left));
    }

    #[test]
    fn test_mock_mouse_with_scroll_up() {
        let mouse_event = mock_mouse(0, 0, MouseEventKind::ScrollUp);
        assert_eq!(mouse_event.kind, MouseEventKind::ScrollUp);
    }

    // =========================================================================
    // mock_terminal tests
    // =========================================================================

    #[test]
    fn test_mock_terminal_function_has_correct_dimensions() {
        let terminal = mock_terminal(80, 24);
        assert_eq!(terminal.width(), 80);
        assert_eq!(terminal.height(), 24);
    }

    #[test]
    fn test_mock_terminal_size() {
        let terminal = mock_terminal(120, 30);
        assert_eq!(terminal.size(), (120, 30));
    }

    #[test]
    fn test_mock_terminal_area() {
        let terminal = mock_terminal(100, 40);
        let area = terminal.area();
        assert_eq!(area.x, 0);
        assert_eq!(area.y, 0);
        assert_eq!(area.width, 100);
        assert_eq!(area.height, 40);
    }

    #[test]
    fn test_mock_terminal_buffer() {
        let terminal = mock_terminal(60, 20);
        let buffer = terminal.buffer();
        assert_eq!(buffer.width(), 60);
        assert_eq!(buffer.height(), 20);
    }

    #[test]
    fn test_mock_terminal_resize() {
        let terminal = mock_terminal(80, 24);
        terminal.resize(100, 30);
        assert_eq!(terminal.width(), 100);
        assert_eq!(terminal.height(), 30);
    }

    // =========================================================================
    // mock_time tests
    // =========================================================================

    #[test]
    fn test_mock_time_function_starts_at_zero() {
        let time = mock_time();
        assert_eq!(time.elapsed_ms(), 0);
        assert_eq!(time.elapsed(), Duration::from_millis(0));
    }

    #[test]
    fn test_mock_time_advance_ms() {
        let time = mock_time();
        time.advance_ms(500);
        assert_eq!(time.elapsed_ms(), 500);
    }

    #[test]
    fn test_mock_time_advance() {
        let time = mock_time();
        time.advance(Duration::from_secs(2));
        assert_eq!(time.elapsed_ms(), 2000);
    }

    #[test]
    fn test_mock_time_advance_secs() {
        let time = mock_time();
        time.advance_secs(5);
        assert_eq!(time.elapsed_ms(), 5000);
    }

    #[test]
    fn test_mock_time_reset() {
        let time = mock_time();
        time.advance_ms(1000);
        time.reset();
        assert_eq!(time.elapsed_ms(), 0);
    }

    #[test]
    fn test_mock_time_set() {
        let time = mock_time();
        time.set(Duration::from_millis(2500));
        assert_eq!(time.elapsed_ms(), 2500);
    }

    #[test]
    fn test_mock_time_multiple_advances() {
        let time = mock_time();
        time.advance_ms(100);
        time.advance_ms(200);
        time.advance_ms(300);
        assert_eq!(time.elapsed_ms(), 600);
    }

    // =========================================================================
    // simulate_user tests
    // =========================================================================

    #[test]
    fn test_simulate_user_function_creates_simulator() {
        let simulator = simulate_user();
        assert!(simulator.is_empty());
        assert_eq!(simulator.len(), 0);
    }

    #[test]
    fn test_simulate_user_key_adds_event() {
        let simulator = simulate_user().key(Key::Char('a'));
        assert_eq!(simulator.len(), 1);
        assert!(simulator.has_events());
    }

    #[test]
    fn test_simulate_user_poll_event() {
        let mut simulator = simulate_user().key(Key::Char('x'));
        assert!(simulator.poll_event().is_some());
        assert!(simulator.is_empty());
    }

    #[test]
    fn test_simulate_user_clear() {
        let mut simulator = simulate_user().key(Key::Char('a')).key(Key::Char('b'));
        assert_eq!(simulator.len(), 2);
        simulator.clear();
        assert!(simulator.is_empty());
    }

    #[test]
    fn test_simulate_user_type_text() {
        let simulator = simulate_user().type_text("hello");
        assert_eq!(simulator.len(), 5);
    }

    #[test]
    fn test_simulate_user_click() {
        let simulator = simulate_user().click(10, 20);
        // Click adds down and up events
        assert_eq!(simulator.len(), 2);
    }

    #[test]
    fn test_simulate_user_wait() {
        let simulator = simulate_user().wait_ms(100);
        assert_eq!(simulator.len(), 1);
    }

    #[test]
    fn test_simulate_user_chain() {
        let simulator = simulate_user()
            .key(Key::Char('a'))
            .enter()
            .wait_ms(50)
            .click(5, 10);
        assert_eq!(simulator.len(), 5); // 1 key + 1 enter + 1 wait + 2 for click
    }

    #[test]
    fn test_simulate_user_into_vec() {
        let simulator = simulate_user().key(Key::Char('x')).escape();
        let events = simulator.into_vec();
        assert_eq!(events.len(), 2);
    }

    // =========================================================================
    // capture_render tests
    // =========================================================================

    #[test]
    fn test_capture_render_function_has_correct_dimensions() {
        let capture = capture_render(40, 10);
        assert_eq!(capture.size(), (40, 10));
    }

    #[test]
    fn test_capture_render_text_empty() {
        let capture = capture_render(20, 5);
        assert_eq!(capture.text(), "");
    }

    #[test]
    fn test_capture_render_contains() {
        let mut capture = capture_render(20, 5);
        capture.buffer_mut().put_str(0, 0, "hello");
        assert!(capture.contains("hello"));
        assert!(!capture.contains("world"));
    }

    #[test]
    fn test_capture_render_line() {
        let mut capture = capture_render(20, 5);
        capture.buffer_mut().put_str(0, 0, "test");
        assert_eq!(capture.line(0), "test");
    }

    #[test]
    fn test_capture_render_line_out_of_bounds() {
        let capture = capture_render(20, 5);
        assert_eq!(capture.line(100), "");
    }

    #[test]
    fn test_capture_render_char_at() {
        let mut capture = capture_render(20, 5);
        capture.buffer_mut().put_str(0, 0, "abc");
        assert_eq!(capture.char_at(0, 0), Some('a'));
        assert_eq!(capture.char_at(1, 0), Some('b'));
        assert_eq!(capture.char_at(10, 10), None);
    }

    #[test]
    fn test_capture_render_clear() {
        let mut capture = capture_render(20, 5);
        capture.buffer_mut().put_str(0, 0, "hello");
        capture.clear();
        assert_eq!(capture.text(), "");
    }
}
