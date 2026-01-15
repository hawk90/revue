//! Mocking utilities for testing Revue applications
//!
//! Provides utilities for mocking events, time, terminal size, and more.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::testing::{MockTerminal, MockTime, EventSimulator};
//!
//! // Mock terminal size
//! let terminal = MockTerminal::new(80, 24);
//!
//! // Mock time progression
//! let time = MockTime::new();
//! time.advance(Duration::from_secs(1));
//!
//! // Simulate user interactions
//! let mut sim = EventSimulator::new();
//! sim.key(Key::Enter).wait_ms(100).key(Key::Escape);
//! ```

use crate::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::render::Buffer;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// Mock Terminal
// =============================================================================

/// Mock terminal for testing with configurable size
#[derive(Debug, Clone)]
pub struct MockTerminal {
    width: Arc<AtomicU64>,
    height: Arc<AtomicU64>,
}

impl MockTerminal {
    /// Create a new mock terminal with given dimensions
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width: Arc::new(AtomicU64::new(width as u64)),
            height: Arc::new(AtomicU64::new(height as u64)),
        }
    }

    /// Get current width
    pub fn width(&self) -> u16 {
        self.width.load(Ordering::Relaxed) as u16
    }

    /// Get current height
    pub fn height(&self) -> u16 {
        self.height.load(Ordering::Relaxed) as u16
    }

    /// Get size as tuple
    pub fn size(&self) -> (u16, u16) {
        (self.width(), self.height())
    }

    /// Resize the terminal
    pub fn resize(&self, width: u16, height: u16) {
        self.width.store(width as u64, Ordering::Relaxed);
        self.height.store(height as u64, Ordering::Relaxed);
    }

    /// Get area
    pub fn area(&self) -> Rect {
        Rect::new(0, 0, self.width(), self.height())
    }

    /// Create a buffer matching terminal size
    pub fn buffer(&self) -> Buffer {
        Buffer::new(self.width(), self.height())
    }
}

impl Default for MockTerminal {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

// =============================================================================
// Mock Time
// =============================================================================

/// Mock time controller for testing time-dependent code
#[derive(Debug, Clone)]
pub struct MockTime {
    elapsed_ms: Arc<AtomicU64>,
}

impl MockTime {
    /// Create a new mock time starting at 0
    pub fn new() -> Self {
        Self {
            elapsed_ms: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(self.elapsed_ms.load(Ordering::Relaxed))
    }

    /// Get elapsed milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms.load(Ordering::Relaxed)
    }

    /// Advance time by duration
    pub fn advance(&self, duration: Duration) {
        self.elapsed_ms
            .fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
    }

    /// Advance time by milliseconds
    pub fn advance_ms(&self, ms: u64) {
        self.elapsed_ms.fetch_add(ms, Ordering::Relaxed);
    }

    /// Advance time by seconds
    pub fn advance_secs(&self, secs: u64) {
        self.advance_ms(secs * 1000);
    }

    /// Reset time to 0
    pub fn reset(&self) {
        self.elapsed_ms.store(0, Ordering::Relaxed);
    }

    /// Set time to specific value
    pub fn set(&self, duration: Duration) {
        self.elapsed_ms
            .store(duration.as_millis() as u64, Ordering::Relaxed);
    }
}

impl Default for MockTime {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Event Simulator
// =============================================================================

/// Simulated event for testing
#[derive(Debug, Clone)]
pub enum SimulatedEvent {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Wait for duration
    Wait(Duration),
    /// Custom callback
    Custom(String),
}

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

// =============================================================================
// Render Capture
// =============================================================================

/// Captured render output for assertions
#[derive(Debug, Clone)]
pub struct RenderCapture {
    buffer: Buffer,
    width: u16,
    height: u16,
}

impl RenderCapture {
    /// Create a new render capture
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: Buffer::new(width, height),
            width,
            height,
        }
    }

    /// Create from existing buffer with dimensions
    pub fn from_buffer(buffer: Buffer, width: u16, height: u16) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    /// Get buffer reference
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get mutable buffer reference
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Get size
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Get all text content
    pub fn text(&self) -> String {
        let mut lines = Vec::new();
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if let Some(cell) = self.buffer.get(x, y) {
                    line.push(cell.symbol);
                } else {
                    line.push(' ');
                }
            }
            lines.push(line.trim_end().to_string());
        }
        // Remove trailing empty lines
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        lines.join("\n")
    }

    /// Get a specific line
    pub fn line(&self, row: u16) -> String {
        if row >= self.height {
            return String::new();
        }
        let mut line = String::new();
        for x in 0..self.width {
            if let Some(cell) = self.buffer.get(x, row) {
                line.push(cell.symbol);
            } else {
                line.push(' ');
            }
        }
        line.trim_end().to_string()
    }

    /// Check if contains text
    pub fn contains(&self, text: &str) -> bool {
        self.text().contains(text)
    }

    /// Find text position
    pub fn find(&self, text: &str) -> Option<(u16, u16)> {
        for y in 0..self.height {
            let line = self.line(y);
            if let Some(x) = line.find(text) {
                return Some((x as u16, y));
            }
        }
        None
    }

    /// Get character at position
    pub fn char_at(&self, x: u16, y: u16) -> Option<char> {
        self.buffer.get(x, y).map(|c| c.symbol)
    }

    /// Count occurrences of a character
    pub fn count_char(&self, ch: char) -> usize {
        self.text().chars().filter(|&c| c == ch).count()
    }

    /// Count occurrences of a string
    pub fn count_str(&self, s: &str) -> usize {
        self.text().matches(s).count()
    }

    /// Clear the capture
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Compare with another capture
    pub fn diff(&self, other: &RenderCapture) -> Vec<(u16, u16, char, char)> {
        let mut diffs = Vec::new();
        let max_y = self.height.max(other.height);
        let max_x = self.width.max(other.width);

        for y in 0..max_y {
            for x in 0..max_x {
                let c1 = self.char_at(x, y).unwrap_or(' ');
                let c2 = other.char_at(x, y).unwrap_or(' ');
                if c1 != c2 {
                    diffs.push((x, y, c1, c2));
                }
            }
        }
        diffs
    }
}

// =============================================================================
// Mock Signal State
// =============================================================================

/// Mock state for testing without reactive system
pub struct MockState<T> {
    value: Rc<RefCell<T>>,
    change_count: Rc<RefCell<usize>>,
}

impl<T> MockState<T> {
    /// Create a new mock state
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            change_count: Rc::new(RefCell::new(0)),
        }
    }

    /// Get the current value
    pub fn get(&self) -> std::cell::Ref<'_, T> {
        self.value.borrow()
    }

    /// Get mutable access to the value
    pub fn get_mut(&self) -> std::cell::RefMut<'_, T> {
        self.value.borrow_mut()
    }

    /// Set a new value
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        *self.change_count.borrow_mut() += 1;
    }

    /// Get the number of times the value has changed
    pub fn change_count(&self) -> usize {
        *self.change_count.borrow()
    }

    /// Reset change count
    pub fn reset_count(&self) {
        *self.change_count.borrow_mut() = 0;
    }
}

impl<T: Clone> MockState<T> {
    /// Get a cloned value
    pub fn value(&self) -> T {
        self.value.borrow().clone()
    }

    /// Update value with a function
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.value.borrow_mut());
        *self.change_count.borrow_mut() += 1;
    }
}

impl<T: Clone> Clone for MockState<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            change_count: Rc::clone(&self.change_count),
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

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
}
