//! Test actions (key presses, mouse clicks, etc.)

use crate::event::Key;
use std::time::Duration;

/// A recorded test action
#[derive(Debug, Clone)]
pub enum Action {
    /// Key action
    Key(KeyAction),
    /// Mouse action
    Mouse(MouseAction),
    /// Wait action
    Wait(Duration),
    /// Resize action
    Resize(u16, u16),
    /// Custom action
    Custom(String),
}

/// Key-related actions
#[derive(Debug, Clone)]
pub enum KeyAction {
    /// Press a key
    Press(Key),
    /// Press a key with Ctrl
    PressCtrl(Key),
    /// Press a key with Alt
    PressAlt(Key),
    /// Type a string
    Type(String),
    /// Hold a key (for repeat)
    Hold(Key, Duration),
    /// Release a held key
    Release(Key),
}

impl KeyAction {
    /// Create a press action
    pub fn press(key: Key) -> Self {
        Self::Press(key)
    }

    /// Create a press with Ctrl action
    pub fn press_ctrl(key: Key) -> Self {
        Self::PressCtrl(key)
    }

    /// Create a press with Alt action
    pub fn press_alt(key: Key) -> Self {
        Self::PressAlt(key)
    }

    /// Create a type action
    pub fn type_text(text: impl Into<String>) -> Self {
        Self::Type(text.into())
    }

    /// Get the key involved
    pub fn key(&self) -> Option<Key> {
        match self {
            KeyAction::Press(k) => Some(*k),
            KeyAction::PressCtrl(k) => Some(*k),
            KeyAction::PressAlt(k) => Some(*k),
            KeyAction::Hold(k, _) => Some(*k),
            KeyAction::Release(k) => Some(*k),
            KeyAction::Type(_) => None,
        }
    }
}

/// Mouse-related actions
#[derive(Debug, Clone)]
pub enum MouseAction {
    /// Click at position
    Click(u16, u16),
    /// Double click at position
    DoubleClick(u16, u16),
    /// Right click at position
    RightClick(u16, u16),
    /// Mouse move to position
    Move(u16, u16),
    /// Drag from one position to another
    Drag(u16, u16, u16, u16),
    /// Scroll up at position
    ScrollUp(u16, u16, u16),
    /// Scroll down at position
    ScrollDown(u16, u16, u16),
}

impl MouseAction {
    /// Create a click action
    pub fn click(x: u16, y: u16) -> Self {
        Self::Click(x, y)
    }

    /// Create a double click action
    pub fn double_click(x: u16, y: u16) -> Self {
        Self::DoubleClick(x, y)
    }

    /// Create a right click action
    pub fn right_click(x: u16, y: u16) -> Self {
        Self::RightClick(x, y)
    }

    /// Create a move action
    pub fn move_to(x: u16, y: u16) -> Self {
        Self::Move(x, y)
    }

    /// Create a drag action
    pub fn drag(from_x: u16, from_y: u16, to_x: u16, to_y: u16) -> Self {
        Self::Drag(from_x, from_y, to_x, to_y)
    }

    /// Get position
    pub fn position(&self) -> (u16, u16) {
        match self {
            MouseAction::Click(x, y) => (*x, *y),
            MouseAction::DoubleClick(x, y) => (*x, *y),
            MouseAction::RightClick(x, y) => (*x, *y),
            MouseAction::Move(x, y) => (*x, *y),
            MouseAction::Drag(x, y, _, _) => (*x, *y),
            MouseAction::ScrollUp(x, y, _) => (*x, *y),
            MouseAction::ScrollDown(x, y, _) => (*x, *y),
        }
    }
}

/// A sequence of actions that can be replayed
#[derive(Debug, Clone, Default)]
pub struct ActionSequence {
    actions: Vec<Action>,
}

impl ActionSequence {
    /// Create a new empty sequence
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an action
    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Add a key press
    pub fn press(mut self, key: Key) -> Self {
        self.actions.push(Action::Key(KeyAction::Press(key)));
        self
    }

    /// Add typing
    pub fn type_text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        for ch in text.chars() {
            self.actions
                .push(Action::Key(KeyAction::Press(Key::Char(ch))));
        }
        self
    }

    /// Add a click
    pub fn click(mut self, x: u16, y: u16) -> Self {
        self.actions.push(Action::Mouse(MouseAction::Click(x, y)));
        self
    }

    /// Add a wait
    pub fn wait(mut self, duration: Duration) -> Self {
        self.actions.push(Action::Wait(duration));
        self
    }

    /// Add a wait in milliseconds
    pub fn wait_ms(self, ms: u64) -> Self {
        self.wait(Duration::from_millis(ms))
    }

    /// Get all actions
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Get action count
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Iterate over actions
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter()
    }
}

impl IntoIterator for ActionSequence {
    type Item = Action;
    type IntoIter = std::vec::IntoIter<Action>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_sequence() {
        let seq = ActionSequence::new()
            .press(Key::Up)
            .press(Key::Down)
            .type_text("hello")
            .click(10, 20);

        assert_eq!(seq.len(), 8); // 2 arrows + 5 chars + 1 click = 8
    }

    #[test]
    fn test_key_action() {
        let action = KeyAction::press(Key::Enter);
        assert_eq!(action.key(), Some(Key::Enter));
    }

    #[test]
    fn test_mouse_action() {
        let action = MouseAction::click(10, 20);
        assert_eq!(action.position(), (10, 20));
    }
}
