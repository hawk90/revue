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
    MouseEvent::new(x, y, MouseEventKind::Down(crate::event::MouseButton::Left))
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
