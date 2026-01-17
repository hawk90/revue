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

    #[test]
    fn test_action_variants() {
        let key_action = Action::Key(KeyAction::Press(Key::Enter));
        assert!(matches!(key_action, Action::Key(_)));

        let mouse_action = Action::Mouse(MouseAction::Click(5, 10));
        assert!(matches!(mouse_action, Action::Mouse(_)));

        let wait_action = Action::Wait(Duration::from_millis(100));
        assert!(matches!(wait_action, Action::Wait(_)));

        let resize_action = Action::Resize(80, 24);
        assert!(matches!(resize_action, Action::Resize(80, 24)));

        let custom_action = Action::Custom("test".to_string());
        assert!(matches!(custom_action, Action::Custom(_)));
    }

    #[test]
    fn test_key_action_constructors() {
        let press = KeyAction::press(Key::Tab);
        assert_eq!(press.key(), Some(Key::Tab));

        let ctrl = KeyAction::press_ctrl(Key::Char('c'));
        assert_eq!(ctrl.key(), Some(Key::Char('c')));

        let alt = KeyAction::press_alt(Key::Char('x'));
        assert_eq!(alt.key(), Some(Key::Char('x')));

        let type_action = KeyAction::type_text("hello");
        assert_eq!(type_action.key(), None);
    }

    #[test]
    fn test_key_action_hold_release() {
        let hold = KeyAction::Hold(Key::Char('a'), Duration::from_millis(500));
        assert_eq!(hold.key(), Some(Key::Char('a')));

        let release = KeyAction::Release(Key::Char('a'));
        assert_eq!(release.key(), Some(Key::Char('a')));
    }

    #[test]
    fn test_mouse_action_constructors() {
        assert_eq!(MouseAction::double_click(5, 10).position(), (5, 10));
        assert_eq!(MouseAction::right_click(15, 20).position(), (15, 20));
        assert_eq!(MouseAction::move_to(25, 30).position(), (25, 30));
    }

    #[test]
    fn test_mouse_action_drag() {
        let drag = MouseAction::drag(0, 0, 100, 100);
        assert_eq!(drag.position(), (0, 0)); // Start position
    }

    #[test]
    fn test_mouse_action_scroll_positions() {
        let scroll_up = MouseAction::ScrollUp(10, 20, 3);
        assert_eq!(scroll_up.position(), (10, 20));

        let scroll_down = MouseAction::ScrollDown(30, 40, 5);
        assert_eq!(scroll_down.position(), (30, 40));
    }

    #[test]
    fn test_action_sequence_empty() {
        let seq = ActionSequence::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_action_sequence_push() {
        let mut seq = ActionSequence::new();
        seq.push(Action::Key(KeyAction::Press(Key::Enter)));
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_action_sequence_wait() {
        let seq = ActionSequence::new()
            .wait(Duration::from_secs(1))
            .wait_ms(500);
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_action_sequence_iter() {
        let seq = ActionSequence::new().press(Key::Up).press(Key::Down);
        let actions: Vec<_> = seq.iter().collect();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_action_sequence_into_iter() {
        let seq = ActionSequence::new().press(Key::Left).press(Key::Right);
        let actions: Vec<_> = seq.into_iter().collect();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_action_sequence_actions() {
        let seq = ActionSequence::new().click(5, 10);
        let actions = seq.actions();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_action_clone() {
        let action = Action::Key(KeyAction::Press(Key::Enter));
        let cloned = action.clone();
        assert!(matches!(cloned, Action::Key(KeyAction::Press(Key::Enter))));
    }

    #[test]
    fn test_action_debug() {
        let action = Action::Resize(100, 50);
        let debug = format!("{:?}", action);
        assert!(debug.contains("Resize"));
    }
}
