//! Event simulator for building sequences of user interactions

use crate::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::testing::mock::types::SimulatedEvent;
use std::collections::VecDeque;
use std::time::Duration;

/// Event simulator for building sequences of user interactions
#[derive(Debug, Default)]
pub struct EventSimulator {
    events: VecDeque<SimulatedEvent>,
}

impl EventSimulator {
    /// Create a new event simulator
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a key press event
    pub fn key(mut self, key: Key) -> Self {
        self.events
            .push_back(SimulatedEvent::Key(KeyEvent::new(key)));
        self
    }

    /// Add Ctrl+key
    pub fn ctrl(mut self, key: Key) -> Self {
        self.events
            .push_back(SimulatedEvent::Key(KeyEvent::ctrl(key)));
        self
    }

    /// Add Ctrl+char
    pub fn ctrl_char(self, c: char) -> Self {
        self.ctrl(Key::Char(c))
    }

    /// Add Alt+key
    pub fn alt(mut self, key: Key) -> Self {
        self.events
            .push_back(SimulatedEvent::Key(KeyEvent::alt(key)));
        self
    }

    /// Add Alt+char
    pub fn alt_char(self, c: char) -> Self {
        self.alt(Key::Char(c))
    }

    /// Add Shift+char (uppercase)
    pub fn shift(self, c: char) -> Self {
        self.key(Key::Char(c.to_ascii_uppercase()))
    }

    /// Type a string (add key events for each character)
    pub fn type_text(mut self, text: &str) -> Self {
        for ch in text.chars() {
            self.events
                .push_back(SimulatedEvent::Key(KeyEvent::new(Key::Char(ch))));
        }
        self
    }

    /// Add Enter key
    pub fn enter(self) -> Self {
        self.key(Key::Enter)
    }

    /// Add Escape key
    pub fn escape(self) -> Self {
        self.key(Key::Escape)
    }

    /// Add Tab key
    pub fn tab(self) -> Self {
        self.key(Key::Tab)
    }

    /// Add Backspace key
    pub fn backspace(self) -> Self {
        self.key(Key::Backspace)
    }

    /// Add Delete key
    pub fn delete(self) -> Self {
        self.key(Key::Delete)
    }

    /// Add up arrow key
    pub fn up(self) -> Self {
        self.key(Key::Up)
    }

    /// Add down arrow key
    pub fn down(self) -> Self {
        self.key(Key::Down)
    }

    /// Add left arrow key
    pub fn left(self) -> Self {
        self.key(Key::Left)
    }

    /// Add right arrow key
    pub fn right(self) -> Self {
        self.key(Key::Right)
    }

    /// Add a mouse click event
    pub fn click(mut self, x: u16, y: u16) -> Self {
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Down(MouseButton::Left),
        )));
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Up(MouseButton::Left),
        )));
        self
    }

    /// Add a right click event
    pub fn right_click(mut self, x: u16, y: u16) -> Self {
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Down(MouseButton::Right),
        )));
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Up(MouseButton::Right),
        )));
        self
    }

    /// Add a double click event
    pub fn double_click(mut self, x: u16, y: u16) -> Self {
        // First click
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Down(MouseButton::Left),
        )));
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Up(MouseButton::Left),
        )));
        // Small wait
        self.events
            .push_back(SimulatedEvent::Wait(Duration::from_millis(50)));
        // Second click
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Down(MouseButton::Left),
        )));
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::Up(MouseButton::Left),
        )));
        self
    }

    /// Add a mouse drag event
    pub fn drag(mut self, from: (u16, u16), to: (u16, u16)) -> Self {
        // Press at start
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            from.0,
            from.1,
            MouseEventKind::Down(MouseButton::Left),
        )));
        // Drag to end
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            to.0,
            to.1,
            MouseEventKind::Drag(MouseButton::Left),
        )));
        // Release at end
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            to.0,
            to.1,
            MouseEventKind::Up(MouseButton::Left),
        )));
        self
    }

    /// Add a scroll up event
    pub fn scroll_up(mut self, x: u16, y: u16) -> Self {
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::ScrollUp,
        )));
        self
    }

    /// Add a scroll down event
    pub fn scroll_down(mut self, x: u16, y: u16) -> Self {
        self.events.push_back(SimulatedEvent::Mouse(MouseEvent::new(
            x,
            y,
            MouseEventKind::ScrollDown,
        )));
        self
    }

    /// Add a wait duration
    pub fn wait(mut self, duration: Duration) -> Self {
        self.events.push_back(SimulatedEvent::Wait(duration));
        self
    }

    /// Add a wait in milliseconds
    pub fn wait_ms(self, ms: u64) -> Self {
        self.wait(Duration::from_millis(ms))
    }

    /// Add a custom named event (for logging/debugging)
    pub fn custom(mut self, name: impl Into<String>) -> Self {
        self.events.push_back(SimulatedEvent::Custom(name.into()));
        self
    }

    /// Poll the next simulated event
    pub fn poll_event(&mut self) -> Option<SimulatedEvent> {
        self.events.pop_front()
    }

    /// Check if there are more events
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Get number of remaining events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Get all events as a vec (consuming)
    pub fn into_vec(self) -> Vec<SimulatedEvent> {
        self.events.into_iter().collect()
    }

    /// Repeat the sequence n times
    pub fn repeat(mut self, n: usize) -> Self {
        let events: Vec<_> = self.events.iter().cloned().collect();
        for _ in 1..n {
            for event in &events {
                self.events.push_back(event.clone());
            }
        }
        self
    }
}
