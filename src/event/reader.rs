//! Event reader using crossterm

use crossterm::event::{
    self, poll, Event as CrosstermEvent, KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers,
    MouseButton as CrosstermMouseButton, MouseEvent as CrosstermMouseEvent,
    MouseEventKind as CrosstermMouseEventKind,
};
use std::time::Duration;

use super::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::constants::{POLL_IMMEDIATE, TICK_RATE_DEFAULT};
use crate::Result;

/// Event reader for terminal input
pub struct EventReader {
    /// Tick rate for polling
    tick_rate: Duration,
}

impl EventReader {
    /// Create a new event reader
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Create with default tick rate (50ms)
    pub fn default_rate() -> Self {
        Self::new(TICK_RATE_DEFAULT)
    }

    /// Read next event, blocking
    ///
    /// Polls for events up to `tick_rate` duration. If an event is available,
    /// it is returned. If the timeout expires with no event, returns `Event::Tick`.
    pub fn read(&self) -> Result<Event> {
        if poll(self.tick_rate)? {
            let event = match event::read()? {
                CrosstermEvent::Key(key) => Event::Key(convert_key_event(key)),
                CrosstermEvent::Mouse(mouse) => Event::Mouse(convert_mouse_event(mouse)),
                CrosstermEvent::Resize(width, height) => Event::Resize(width, height),
                CrosstermEvent::FocusGained => Event::FocusGained,
                CrosstermEvent::FocusLost => Event::FocusLost,
                CrosstermEvent::Paste(text) => Event::Paste(text),
            };
            Ok(event)
        } else {
            Ok(Event::Tick)
        }
    }

    /// Try to read event without blocking
    pub fn try_read(&self) -> Result<Option<Event>> {
        if poll(POLL_IMMEDIATE)? {
            match event::read()? {
                CrosstermEvent::Key(key) => Ok(Some(Event::Key(convert_key_event(key)))),
                CrosstermEvent::Mouse(mouse) => Ok(Some(Event::Mouse(convert_mouse_event(mouse)))),
                CrosstermEvent::Resize(width, height) => Ok(Some(Event::Resize(width, height))),
                CrosstermEvent::FocusGained => Ok(Some(Event::FocusGained)),
                CrosstermEvent::FocusLost => Ok(Some(Event::FocusLost)),
                CrosstermEvent::Paste(text) => Ok(Some(Event::Paste(text))),
            }
        } else {
            Ok(None)
        }
    }

    /// Check if an event is available
    pub fn has_event(&self) -> Result<bool> {
        Ok(poll(Duration::from_millis(0))?)
    }
}

impl Default for EventReader {
    fn default() -> Self {
        Self::default_rate()
    }
}

/// Convert crossterm KeyEvent to our KeyEvent
fn convert_key_event(key: CrosstermKeyEvent) -> KeyEvent {
    let k = match key.code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Enter => Key::Enter,
        KeyCode::Esc => Key::Escape,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Delete => Key::Delete,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::F(n) => Key::F(n),
        KeyCode::BackTab => Key::BackTab,
        KeyCode::Insert => Key::Insert,
        KeyCode::Null => Key::Null,
        _ => Key::Unknown,
    };

    KeyEvent {
        key: k,
        ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
        alt: key.modifiers.contains(KeyModifiers::ALT),
        shift: key.modifiers.contains(KeyModifiers::SHIFT),
    }
}

/// Convert crossterm MouseEvent to our MouseEvent
fn convert_mouse_event(mouse: CrosstermMouseEvent) -> MouseEvent {
    let kind = match mouse.kind {
        CrosstermMouseEventKind::Down(CrosstermMouseButton::Left) => {
            MouseEventKind::Down(MouseButton::Left)
        }
        CrosstermMouseEventKind::Down(CrosstermMouseButton::Right) => {
            MouseEventKind::Down(MouseButton::Right)
        }
        CrosstermMouseEventKind::Down(CrosstermMouseButton::Middle) => {
            MouseEventKind::Down(MouseButton::Middle)
        }
        CrosstermMouseEventKind::Up(CrosstermMouseButton::Left) => {
            MouseEventKind::Up(MouseButton::Left)
        }
        CrosstermMouseEventKind::Up(CrosstermMouseButton::Right) => {
            MouseEventKind::Up(MouseButton::Right)
        }
        CrosstermMouseEventKind::Up(CrosstermMouseButton::Middle) => {
            MouseEventKind::Up(MouseButton::Middle)
        }
        CrosstermMouseEventKind::Drag(CrosstermMouseButton::Left) => {
            MouseEventKind::Drag(MouseButton::Left)
        }
        CrosstermMouseEventKind::Drag(CrosstermMouseButton::Right) => {
            MouseEventKind::Drag(MouseButton::Right)
        }
        CrosstermMouseEventKind::Drag(CrosstermMouseButton::Middle) => {
            MouseEventKind::Drag(MouseButton::Middle)
        }
        CrosstermMouseEventKind::Moved => MouseEventKind::Move,
        CrosstermMouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
        CrosstermMouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
        CrosstermMouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
        CrosstermMouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
    };

    MouseEvent {
        x: mouse.column,
        y: mouse.row,
        kind,
        ctrl: mouse.modifiers.contains(KeyModifiers::CONTROL),
        alt: mouse.modifiers.contains(KeyModifiers::ALT),
        shift: mouse.modifiers.contains(KeyModifiers::SHIFT),
    }
}

// Note: All tests for EventReader stay inline because they access private
// fields (tick_rate) or private functions (convert_key_event).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_reader_creation() {
        let reader = EventReader::new(Duration::from_millis(100));
        assert_eq!(reader.tick_rate, Duration::from_millis(100));
    }

    #[test]
    fn test_event_reader_default() {
        let reader = EventReader::default();
        assert_eq!(reader.tick_rate, Duration::from_millis(50));
    }

    #[test]
    fn test_convert_key_event_char() {
        let ct_key = CrosstermKeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        let key_event = convert_key_event(ct_key);

        assert_eq!(key_event.key, Key::Char('a'));
        assert!(!key_event.ctrl);
        assert!(!key_event.alt);
        assert!(!key_event.shift);
    }

    #[test]
    fn test_convert_key_event_with_modifiers() {
        let ct_key = CrosstermKeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let key_event = convert_key_event(ct_key);

        assert_eq!(key_event.key, Key::Char('c'));
        assert!(key_event.ctrl);
        assert!(!key_event.alt);
    }

    #[test]
    fn test_convert_key_event_special_keys() {
        let keys = [
            (KeyCode::Enter, Key::Enter),
            (KeyCode::Esc, Key::Escape),
            (KeyCode::Tab, Key::Tab),
            (KeyCode::Backspace, Key::Backspace),
            (KeyCode::Up, Key::Up),
            (KeyCode::Down, Key::Down),
            (KeyCode::Left, Key::Left),
            (KeyCode::Right, Key::Right),
            (KeyCode::Home, Key::Home),
            (KeyCode::End, Key::End),
            (KeyCode::PageUp, Key::PageUp),
            (KeyCode::PageDown, Key::PageDown),
            (KeyCode::F(1), Key::F(1)),
            (KeyCode::F(12), Key::F(12)),
        ];

        for (ct_code, expected_key) in keys {
            let ct_key = CrosstermKeyEvent::new(ct_code, KeyModifiers::empty());
            let key_event = convert_key_event(ct_key);
            assert_eq!(key_event.key, expected_key, "Failed for {:?}", ct_code);
        }
    }
}
