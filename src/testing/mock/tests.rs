//! Tests for mocking utilities

use crate::event::{Key, KeyEvent, MouseEvent, MouseEventKind};
use crate::render::Buffer;
use std::time::Duration;

use super::{
    capture_render, mock_alt_key, mock_click, mock_ctrl_key, mock_key, mock_mouse, mock_terminal,
    mock_time, simulate_user, EventSimulator, MockState, MockTerminal, MockTime, RenderCapture,
    SimulatedEvent,
};

// General integration tests

#[test]
fn test_mock_terminal() {
    let term = MockTerminal::new(100, 50);
    assert_eq!(term.size(), (100, 50));

    term.resize(120, 40);
    assert_eq!(term.size(), (120, 40));
}

#[test]
fn test_mock_time() {
    let time = MockTime::new();
    assert_eq!(time.elapsed_ms(), 0);

    time.advance_ms(100);
    assert_eq!(time.elapsed_ms(), 100);

    time.advance_secs(1);
    assert_eq!(time.elapsed_ms(), 1100);

    time.reset();
    assert_eq!(time.elapsed_ms(), 0);
}

#[test]
fn test_event_simulator() {
    let mut sim = EventSimulator::new()
        .key(Key::Enter)
        .type_text("hello")
        .wait_ms(100)
        .click(10, 5);

    assert_eq!(sim.len(), 9); // Enter + 5 chars + wait + 2 mouse events (down + up)

    // First event should be Enter key
    if let Some(SimulatedEvent::Key(key)) = sim.poll_event() {
        assert_eq!(key.key, Key::Enter);
    } else {
        panic!("Expected key event");
    }
}

#[test]
fn test_render_capture() {
    let mut capture = RenderCapture::new(20, 5);

    // Set some content
    if let Some(cell) = capture.buffer_mut().get_mut(0, 0) {
        cell.symbol = 'H';
    }
    if let Some(cell) = capture.buffer_mut().get_mut(1, 0) {
        cell.symbol = 'i';
    }

    assert!(capture.text().starts_with("Hi"));
    assert!(capture.contains("Hi"));
    assert_eq!(capture.char_at(0, 0), Some('H'));
}

#[test]
fn test_mock_state() {
    let state = MockState::new(0);
    assert_eq!(state.value(), 0);
    assert_eq!(state.change_count(), 0);

    state.set(5);
    assert_eq!(state.value(), 5);
    assert_eq!(state.change_count(), 1);

    state.update(|v| *v += 10);
    assert_eq!(state.value(), 15);
    assert_eq!(state.change_count(), 2);
}

// =========================================================================
// MockTerminal tests
// =========================================================================

#[test]
fn test_mock_terminal_default() {
    let term = MockTerminal::default();
    assert_eq!(term.size(), (80, 24));
}

#[test]
fn test_mock_terminal_width_height() {
    let term = MockTerminal::new(120, 40);
    assert_eq!(term.width(), 120);
    assert_eq!(term.height(), 40);
}

#[test]
fn test_mock_terminal_area() {
    let term = MockTerminal::new(100, 50);
    let area = term.area();
    assert_eq!(area.x, 0);
    assert_eq!(area.y, 0);
    assert_eq!(area.width, 100);
    assert_eq!(area.height, 50);
}

#[test]
fn test_mock_terminal_buffer() {
    let term = MockTerminal::new(80, 24);
    let buffer = term.buffer();
    assert_eq!(buffer.width(), 80);
    assert_eq!(buffer.height(), 24);
}

#[test]
fn test_mock_terminal_clone() {
    let term = MockTerminal::new(100, 50);
    let cloned = term.clone();
    assert_eq!(cloned.size(), (100, 50));

    // Both should share the same atomic values
    term.resize(120, 40);
    assert_eq!(cloned.size(), (120, 40));
}

// =========================================================================
// MockTime tests
// =========================================================================

#[test]
fn test_mock_time_default() {
    let time = MockTime::default();
    assert_eq!(time.elapsed_ms(), 0);
}

#[test]
fn test_mock_time_elapsed() {
    let time = MockTime::new();
    time.advance_ms(500);
    assert_eq!(time.elapsed(), Duration::from_millis(500));
}

#[test]
fn test_mock_time_advance() {
    let time = MockTime::new();
    time.advance(Duration::from_secs(2));
    assert_eq!(time.elapsed_ms(), 2000);
}

#[test]
fn test_mock_time_set() {
    let time = MockTime::new();
    time.set(Duration::from_secs(5));
    assert_eq!(time.elapsed_ms(), 5000);
}

#[test]
fn test_mock_time_clone() {
    let time = MockTime::new();
    time.advance_ms(100);

    let cloned = time.clone();
    assert_eq!(cloned.elapsed_ms(), 100);

    // Both share the same atomic
    time.advance_ms(50);
    assert_eq!(cloned.elapsed_ms(), 150);
}

// =========================================================================
// SimulatedEvent tests
// =========================================================================

#[test]
fn test_simulated_event_key() {
    let event = SimulatedEvent::Key(KeyEvent::new(Key::Enter));
    assert!(matches!(event, SimulatedEvent::Key(_)));
}

#[test]
fn test_simulated_event_mouse() {
    let event = SimulatedEvent::Mouse(MouseEvent::new(10, 20, MouseEventKind::ScrollUp));
    assert!(matches!(event, SimulatedEvent::Mouse(_)));
}

#[test]
fn test_simulated_event_wait() {
    let event = SimulatedEvent::Wait(Duration::from_millis(100));
    assert!(matches!(event, SimulatedEvent::Wait(_)));
}

#[test]
fn test_simulated_event_custom() {
    let event = SimulatedEvent::Custom("test".to_string());
    assert!(matches!(event, SimulatedEvent::Custom(_)));
}

// =========================================================================
// EventSimulator tests
// =========================================================================

#[test]
fn test_event_simulator_new() {
    let sim = EventSimulator::new();
    assert!(sim.is_empty());
    assert_eq!(sim.len(), 0);
}

#[test]
fn test_event_simulator_key() {
    let sim = EventSimulator::new().key(Key::Tab);
    assert_eq!(sim.len(), 1);
}

#[test]
fn test_event_simulator_ctrl() {
    let mut sim = EventSimulator::new().ctrl(Key::Char('c'));
    assert_eq!(sim.len(), 1);

    if let Some(SimulatedEvent::Key(event)) = sim.poll_event() {
        assert!(event.ctrl);
    } else {
        panic!("Expected key event");
    }
}

#[test]
fn test_event_simulator_ctrl_char() {
    let mut sim = EventSimulator::new().ctrl_char('s');
    if let Some(SimulatedEvent::Key(event)) = sim.poll_event() {
        assert!(event.ctrl);
        assert_eq!(event.key, Key::Char('s'));
    } else {
        panic!("Expected key event");
    }
}

#[test]
fn test_event_simulator_alt() {
    let mut sim = EventSimulator::new().alt(Key::Char('x'));
    if let Some(SimulatedEvent::Key(event)) = sim.poll_event() {
        assert!(event.alt);
    } else {
        panic!("Expected key event");
    }
}

#[test]
fn test_event_simulator_alt_char() {
    let mut sim = EventSimulator::new().alt_char('f');
    if let Some(SimulatedEvent::Key(event)) = sim.poll_event() {
        assert!(event.alt);
        assert_eq!(event.key, Key::Char('f'));
    } else {
        panic!("Expected key event");
    }
}

#[test]
fn test_event_simulator_shift() {
    let mut sim = EventSimulator::new().shift('a');
    if let Some(SimulatedEvent::Key(event)) = sim.poll_event() {
        assert_eq!(event.key, Key::Char('A'));
    } else {
        panic!("Expected key event");
    }
}

#[test]
fn test_event_simulator_type_text() {
    let sim = EventSimulator::new().type_text("abc");
    assert_eq!(sim.len(), 3);
}

#[test]
fn test_event_simulator_convenience_keys() {
    let sim = EventSimulator::new()
        .enter()
        .escape()
        .tab()
        .backspace()
        .delete()
        .up()
        .down()
        .left()
        .right();

    assert_eq!(sim.len(), 9);
}

#[test]
fn test_event_simulator_click() {
    let sim = EventSimulator::new().click(10, 20);
    // Click = mouse down + mouse up
    assert_eq!(sim.len(), 2);
}

#[test]
fn test_event_simulator_right_click() {
    let sim = EventSimulator::new().right_click(5, 10);
    assert_eq!(sim.len(), 2);
}

#[test]
fn test_event_simulator_double_click() {
    let sim = EventSimulator::new().double_click(15, 25);
    // Double click = down + up + wait + down + up
    assert_eq!(sim.len(), 5);
}

#[test]
fn test_event_simulator_drag() {
    let sim = EventSimulator::new().drag((10, 10), (50, 50));
    // Drag = down + drag + up
    assert_eq!(sim.len(), 3);
}

#[test]
fn test_event_simulator_scroll() {
    let sim = EventSimulator::new().scroll_up(10, 10).scroll_down(10, 10);
    assert_eq!(sim.len(), 2);
}

#[test]
fn test_event_simulator_wait() {
    let sim = EventSimulator::new().wait(Duration::from_secs(1));
    assert_eq!(sim.len(), 1);
}

#[test]
fn test_event_simulator_wait_ms() {
    let sim = EventSimulator::new().wait_ms(500);
    assert_eq!(sim.len(), 1);
}

#[test]
fn test_event_simulator_custom() {
    let sim = EventSimulator::new().custom("my-event");
    assert_eq!(sim.len(), 1);
}

#[test]
fn test_event_simulator_has_events() {
    let mut sim = EventSimulator::new().key(Key::Enter);
    assert!(sim.has_events());

    sim.poll_event();
    assert!(!sim.has_events());
}

#[test]
fn test_event_simulator_clear() {
    let mut sim = EventSimulator::new().key(Key::Enter).key(Key::Tab);
    assert_eq!(sim.len(), 2);

    sim.clear();
    assert!(sim.is_empty());
}

#[test]
fn test_event_simulator_into_vec() {
    let sim = EventSimulator::new().key(Key::Up).key(Key::Down);
    let events = sim.into_vec();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_event_simulator_repeat() {
    let sim = EventSimulator::new().key(Key::Enter).repeat(3);
    assert_eq!(sim.len(), 3);
}

// =========================================================================
// RenderCapture tests
// =========================================================================

#[test]
fn test_render_capture_new() {
    let capture = RenderCapture::new(80, 24);
    assert_eq!(capture.size(), (80, 24));
}

#[test]
fn test_render_capture_from_buffer() {
    let buffer = Buffer::new(40, 10);
    let capture = RenderCapture::from_buffer(buffer, 40, 10);
    assert_eq!(capture.size(), (40, 10));
}

#[test]
fn test_render_capture_buffer_access() {
    let mut capture = RenderCapture::new(20, 5);
    let buffer = capture.buffer();
    assert_eq!(buffer.width(), 20);

    let buffer_mut = capture.buffer_mut();
    assert_eq!(buffer_mut.height(), 5);
}

#[test]
fn test_render_capture_line() {
    let mut capture = RenderCapture::new(10, 3);
    if let Some(cell) = capture.buffer_mut().get_mut(0, 1) {
        cell.symbol = 'X';
    }

    let line = capture.line(1);
    assert!(line.starts_with('X'));
}

#[test]
fn test_render_capture_line_out_of_bounds() {
    let capture = RenderCapture::new(10, 3);
    let line = capture.line(100);
    assert!(line.is_empty());
}

#[test]
fn test_render_capture_find() {
    let mut capture = RenderCapture::new(20, 3);
    // Set "Hello" at position (5, 1)
    for (i, ch) in "Hello".chars().enumerate() {
        if let Some(cell) = capture.buffer_mut().get_mut(5 + i as u16, 1) {
            cell.symbol = ch;
        }
    }

    let pos = capture.find("Hello");
    assert_eq!(pos, Some((5, 1)));
}

#[test]
fn test_render_capture_find_not_found() {
    let capture = RenderCapture::new(20, 3);
    let pos = capture.find("NotThere");
    assert_eq!(pos, None);
}

#[test]
fn test_render_capture_count_char() {
    let mut capture = RenderCapture::new(10, 1);
    for i in 0..5 {
        if let Some(cell) = capture.buffer_mut().get_mut(i, 0) {
            cell.symbol = 'X';
        }
    }

    assert_eq!(capture.count_char('X'), 5);
}

#[test]
fn test_render_capture_count_str() {
    let mut capture = RenderCapture::new(20, 1);
    for (i, ch) in "ab ab ab".chars().enumerate() {
        if let Some(cell) = capture.buffer_mut().get_mut(i as u16, 0) {
            cell.symbol = ch;
        }
    }

    assert_eq!(capture.count_str("ab"), 3);
}

#[test]
fn test_render_capture_clear() {
    let mut capture = RenderCapture::new(10, 5);
    if let Some(cell) = capture.buffer_mut().get_mut(0, 0) {
        cell.symbol = 'X';
    }

    capture.clear();
    assert_eq!(capture.char_at(0, 0), Some(' '));
}

#[test]
fn test_render_capture_diff() {
    let mut capture1 = RenderCapture::new(5, 1);
    let mut capture2 = RenderCapture::new(5, 1);

    if let Some(cell) = capture1.buffer_mut().get_mut(0, 0) {
        cell.symbol = 'A';
    }
    if let Some(cell) = capture2.buffer_mut().get_mut(0, 0) {
        cell.symbol = 'B';
    }

    let diffs = capture1.diff(&capture2);
    assert!(!diffs.is_empty());
    assert_eq!(diffs[0], (0, 0, 'A', 'B'));
}

// =========================================================================
// MockState tests
// =========================================================================

#[test]
fn test_mock_state_get() {
    let state = MockState::new(42);
    assert_eq!(*state.get(), 42);
}

#[test]
fn test_mock_state_get_mut() {
    let state = MockState::new(10);
    *state.get_mut() = 20;
    assert_eq!(state.value(), 20);
}

#[test]
fn test_mock_state_reset_count() {
    let state = MockState::new(0);
    state.set(1);
    state.set(2);
    assert_eq!(state.change_count(), 2);

    state.reset_count();
    assert_eq!(state.change_count(), 0);
}

#[test]
fn test_mock_state_clone() {
    let state = MockState::new(100);
    let cloned = state.clone();

    state.set(200);
    // Both share the same Rc
    assert_eq!(cloned.value(), 200);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_mock_key_helper() {
    let event = mock_key(Key::Enter);
    assert_eq!(event.key, Key::Enter);
}

#[test]
fn test_mock_ctrl_key_helper() {
    let event = mock_ctrl_key(Key::Char('c'));
    assert!(event.ctrl);
}

#[test]
fn test_mock_alt_key_helper() {
    let event = mock_alt_key(Key::Char('x'));
    assert!(event.alt);
}

#[test]
fn test_mock_click_helper() {
    let event = mock_click(10, 20);
    assert_eq!(event.x, 10);
    assert_eq!(event.y, 20);
}

#[test]
fn test_mock_mouse_helper() {
    let event = mock_mouse(5, 15, MouseEventKind::ScrollDown);
    assert_eq!(event.x, 5);
    assert_eq!(event.y, 15);
}

#[test]
fn test_mock_terminal_helper() {
    let term = mock_terminal(100, 50);
    assert_eq!(term.size(), (100, 50));
}

#[test]
fn test_mock_time_helper() {
    let time = mock_time();
    assert_eq!(time.elapsed_ms(), 0);
}

#[test]
fn test_simulate_user_helper() {
    let sim = simulate_user();
    assert!(sim.is_empty());
}

#[test]
fn test_capture_render_helper() {
    let capture = capture_render(80, 24);
    assert_eq!(capture.size(), (80, 24));
}
