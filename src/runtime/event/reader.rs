//! Event reader using crossterm

use crossterm::event::{
    self, poll, Event as CrosstermEvent, KeyCode, KeyEvent as CrosstermKeyEvent, KeyEventKind,
    KeyModifiers, MouseButton as CrosstermMouseButton, MouseEvent as CrosstermMouseEvent,
    MouseEventKind as CrosstermMouseEventKind,
};
use std::time::{Duration, Instant};

use super::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::constants::{MAX_PASTE_SIZE, POLL_IMMEDIATE, TICK_RATE_DEFAULT};
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
    /// Key releases are skipped (see [`Event::Key`]).
    ///
    /// # Errors
    ///
    /// Returns `Err(io::Error)` if:
    /// - Terminal event polling fails
    /// - Event reading fails (e.g., terminal disconnected)
    pub fn read(&self) -> Result<Event> {
        let deadline = Instant::now() + self.tick_rate;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if !poll(left)? {
                return Ok(Event::Tick);
            }
            if let Some(event) = convert_event(event::read()?) {
                return Ok(event);
            }
        }
    }

    /// Try to read event without blocking
    ///
    /// Returns immediately with `Some(event)` if an event is available,
    /// or `None` if no event is pending.
    ///
    /// # Errors
    ///
    /// Returns `Err(io::Error)` if terminal event polling or reading fails.
    pub fn try_read(&self) -> Result<Option<Event>> {
        while poll(POLL_IMMEDIATE)? {
            if let Some(event) = convert_event(event::read()?) {
                return Ok(Some(event));
            }
        }
        Ok(None)
    }

    /// Check if an event is available
    ///
    /// Returns `true` if an event is available to read, `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns `Err(io::Error)` if terminal event polling fails.
    pub fn has_event(&self) -> Result<bool> {
        Ok(poll(Duration::from_millis(0))?)
    }
}

impl Default for EventReader {
    fn default() -> Self {
        Self::default_rate()
    }
}

/// Convert a crossterm event to ours. Key releases, which crossterm reports
/// on Windows, give `None`: a key acts once, when pressed (or repeated).
fn convert_event(event: CrosstermEvent) -> Option<Event> {
    Some(match event {
        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Release => return None,
        CrosstermEvent::Key(key) => Event::Key(convert_key_event(key)),
        CrosstermEvent::Mouse(mouse) => Event::Mouse(convert_mouse_event(mouse)),
        CrosstermEvent::Resize(width, height) => Event::Resize(width, height),
        CrosstermEvent::FocusGained => Event::FocusGained,
        CrosstermEvent::FocusLost => Event::FocusLost,
        // Truncate paste to prevent DoS through large paste events
        CrosstermEvent::Paste(text) => Event::Paste(truncate_paste(text)),
    })
}

/// Cut `text` to at most `MAX_PASTE_SIZE` bytes, on a char boundary
fn truncate_paste(mut text: String) -> String {
    let end = text.floor_char_boundary(MAX_PASTE_SIZE);
    text.truncate(end);
    text
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

    /// A real Shift+Tab press: crossterm sends BackTab with SHIFT set. It must
    /// match bindings written as `s-tab` or `backtab`, and be Shift+Tab.
    #[test]
    fn test_shift_tab_from_crossterm_matches_backtab_bindings() {
        use crate::event::KeyMap;
        use crate::utils::keymap::{parse_key_binding, KeymapConfig, LookupResult, Mode};

        let pressed = convert_key_event(CrosstermKeyEvent::new(
            KeyCode::BackTab,
            KeyModifiers::SHIFT,
        ));
        assert!(pressed.is_shift_tab());
        assert!(!pressed.is_tab());

        for spelling in ["s-tab", "S-Tab", "backtab", "shift-backtab"] {
            let mut keymap = KeymapConfig::new();
            keymap.bind(Mode::Normal, spelling, "focus_prev");
            assert_eq!(
                keymap.lookup(pressed.to_binding()),
                LookupResult::Action("focus_prev".to_string()),
                "{spelling}"
            );

            let mut map = KeyMap::new();
            map.bind(parse_key_binding(spelling).unwrap(), "focus_prev");
            assert_eq!(
                map.get(&pressed.to_binding()),
                Some(&"focus_prev"),
                "{spelling}"
            );
        }

        // A BackTab without the SHIFT flag (some terminals) is the same key
        let plain = convert_key_event(CrosstermKeyEvent::new(
            KeyCode::BackTab,
            KeyModifiers::empty(),
        ));
        assert!(plain.is_shift_tab());
        assert_eq!(plain.to_binding(), pressed.to_binding());
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

    #[test]
    fn test_key_release_is_dropped() {
        use crossterm::event::KeyEventState;
        let key = |kind| {
            CrosstermEvent::Key(CrosstermKeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::NONE,
                kind,
                state: KeyEventState::NONE,
            })
        };
        assert!(convert_event(key(KeyEventKind::Release)).is_none());
        for kind in [KeyEventKind::Press, KeyEventKind::Repeat] {
            assert!(matches!(
                convert_event(key(kind)),
                Some(Event::Key(KeyEvent {
                    key: Key::Char('a'),
                    ..
                }))
            ));
        }
    }

    #[test]
    fn test_large_paste_is_cut_on_a_char_boundary() {
        // 3-byte chars, so MAX_PASTE_SIZE falls inside one
        let text = "한".repeat(MAX_PASTE_SIZE / 3 + 10);
        assert!(!text.is_char_boundary(MAX_PASTE_SIZE));

        let Some(Event::Paste(pasted)) = convert_event(CrosstermEvent::Paste(text)) else {
            panic!("expected a paste event");
        };
        assert!(pasted.len() <= MAX_PASTE_SIZE);
        assert_eq!(pasted.chars().count(), MAX_PASTE_SIZE / 3);
    }

    #[test]
    fn test_small_paste_is_kept() {
        let pasted = convert_event(CrosstermEvent::Paste("붙여넣기".into()));
        assert!(matches!(pasted, Some(Event::Paste(t)) if t == "붙여넣기"));
    }
}
