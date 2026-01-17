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

    // =========================================================================
    // Action enum tests
    // =========================================================================

    #[test]
    fn test_action_key() {
        let action = Action::Key(KeyAction::Press(Key::Enter));
        assert!(matches!(action, Action::Key(_)));
    }

    #[test]
    fn test_action_mouse() {
        let action = Action::Mouse(MouseAction::Click(10, 20));
        assert!(matches!(action, Action::Mouse(_)));
    }

    #[test]
    fn test_action_wait() {
        let action = Action::Wait(Duration::from_millis(100));
        assert!(matches!(action, Action::Wait(d) if d == Duration::from_millis(100)));
    }

    #[test]
    fn test_action_resize() {
        let action = Action::Resize(120, 40);
        assert!(matches!(action, Action::Resize(120, 40)));
    }

    #[test]
    fn test_action_custom() {
        let action = Action::Custom("my-action".to_string());
        assert!(matches!(action, Action::Custom(s) if s == "my-action"));
    }

    #[test]
    fn test_action_clone() {
        let action = Action::Resize(80, 24);
        let cloned = action.clone();
        assert!(matches!(cloned, Action::Resize(80, 24)));
    }

    // =========================================================================
    // KeyAction tests
    // =========================================================================

    #[test]
    fn test_key_action_press() {
        let action = KeyAction::press(Key::Tab);
        assert_eq!(action.key(), Some(Key::Tab));
    }

    #[test]
    fn test_key_action_press_ctrl() {
        let action = KeyAction::press_ctrl(Key::Char('c'));
        assert_eq!(action.key(), Some(Key::Char('c')));
        assert!(matches!(action, KeyAction::PressCtrl(_)));
    }

    #[test]
    fn test_key_action_press_alt() {
        let action = KeyAction::press_alt(Key::Char('x'));
        assert_eq!(action.key(), Some(Key::Char('x')));
        assert!(matches!(action, KeyAction::PressAlt(_)));
    }

    #[test]
    fn test_key_action_type_text() {
        let action = KeyAction::type_text("hello");
        assert_eq!(action.key(), None);
        assert!(matches!(action, KeyAction::Type(s) if s == "hello"));
    }

    #[test]
    fn test_key_action_hold() {
        let action = KeyAction::Hold(Key::Enter, Duration::from_millis(500));
        assert_eq!(action.key(), Some(Key::Enter));
    }

    #[test]
    fn test_key_action_release() {
        let action = KeyAction::Release(Key::Escape);
        assert_eq!(action.key(), Some(Key::Escape));
    }

    #[test]
    fn test_key_action_key_none_for_type() {
        let action = KeyAction::Type("test".to_string());
        assert_eq!(action.key(), None);
    }

    #[test]
    fn test_key_action_clone() {
        let action = KeyAction::Press(Key::Enter);
        let cloned = action.clone();
        assert_eq!(cloned.key(), Some(Key::Enter));
    }

    // =========================================================================
    // MouseAction tests
    // =========================================================================

    #[test]
    fn test_mouse_action_click() {
        let action = MouseAction::click(5, 10);
        assert_eq!(action.position(), (5, 10));
        assert!(matches!(action, MouseAction::Click(5, 10)));
    }

    #[test]
    fn test_mouse_action_double_click() {
        let action = MouseAction::double_click(15, 25);
        assert_eq!(action.position(), (15, 25));
        assert!(matches!(action, MouseAction::DoubleClick(15, 25)));
    }

    #[test]
    fn test_mouse_action_right_click() {
        let action = MouseAction::right_click(30, 40);
        assert_eq!(action.position(), (30, 40));
        assert!(matches!(action, MouseAction::RightClick(30, 40)));
    }

    #[test]
    fn test_mouse_action_move_to() {
        let action = MouseAction::move_to(50, 60);
        assert_eq!(action.position(), (50, 60));
        assert!(matches!(action, MouseAction::Move(50, 60)));
    }

    #[test]
    fn test_mouse_action_drag() {
        let action = MouseAction::drag(10, 20, 30, 40);
        // Position returns the start position
        assert_eq!(action.position(), (10, 20));
        assert!(matches!(action, MouseAction::Drag(10, 20, 30, 40)));
    }

    #[test]
    fn test_mouse_action_scroll_up() {
        let action = MouseAction::ScrollUp(10, 20, 3);
        assert_eq!(action.position(), (10, 20));
    }

    #[test]
    fn test_mouse_action_scroll_down() {
        let action = MouseAction::ScrollDown(15, 25, 5);
        assert_eq!(action.position(), (15, 25));
    }

    #[test]
    fn test_mouse_action_clone() {
        let action = MouseAction::Click(10, 20);
        let cloned = action.clone();
        assert_eq!(cloned.position(), (10, 20));
    }

    // =========================================================================
    // ActionSequence tests
    // =========================================================================

    #[test]
    fn test_action_sequence_new() {
        let seq = ActionSequence::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_action_sequence_default() {
        let seq = ActionSequence::default();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_action_sequence_push() {
        let mut seq = ActionSequence::new();
        seq.push(Action::Key(KeyAction::Press(Key::Enter)));

        assert!(!seq.is_empty());
        assert_eq!(seq.len(), 1);
    }

    #[test]
    fn test_action_sequence_press() {
        let seq = ActionSequence::new().press(Key::Up).press(Key::Down);

        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_action_sequence_type_text_chars() {
        let seq = ActionSequence::new().type_text("abc");

        // Each character becomes a separate key press
        assert_eq!(seq.len(), 3);
    }

    #[test]
    fn test_action_sequence_type_text_empty() {
        let seq = ActionSequence::new().type_text("");

        assert!(seq.is_empty());
    }

    #[test]
    fn test_action_sequence_click() {
        let seq = ActionSequence::new().click(10, 20);

        assert_eq!(seq.len(), 1);
        let actions = seq.actions();
        assert!(matches!(
            actions[0],
            Action::Mouse(MouseAction::Click(10, 20))
        ));
    }

    #[test]
    fn test_action_sequence_wait() {
        let seq = ActionSequence::new().wait(Duration::from_secs(1));

        assert_eq!(seq.len(), 1);
        let actions = seq.actions();
        assert!(matches!(actions[0], Action::Wait(d) if d == Duration::from_secs(1)));
    }

    #[test]
    fn test_action_sequence_wait_ms() {
        let seq = ActionSequence::new().wait_ms(500);

        assert_eq!(seq.len(), 1);
        let actions = seq.actions();
        assert!(matches!(actions[0], Action::Wait(d) if d == Duration::from_millis(500)));
    }

    #[test]
    fn test_action_sequence_actions() {
        let seq = ActionSequence::new().press(Key::Enter).click(0, 0);

        let actions = seq.actions();
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn test_action_sequence_iter() {
        let seq = ActionSequence::new()
            .press(Key::Up)
            .press(Key::Down)
            .press(Key::Enter);

        let count = seq.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_action_sequence_into_iter() {
        let seq = ActionSequence::new().press(Key::Up).press(Key::Down);

        let collected: Vec<Action> = seq.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn test_action_sequence_chained() {
        let seq = ActionSequence::new()
            .press(Key::Tab)
            .type_text("user")
            .press(Key::Tab)
            .type_text("pass")
            .press(Key::Enter);

        // Tab + 4 chars + Tab + 4 chars + Enter = 11
        assert_eq!(seq.len(), 11);
    }

    #[test]
    fn test_action_sequence_clone() {
        let seq = ActionSequence::new().press(Key::Enter).click(10, 20);

        let cloned = seq.clone();
        assert_eq!(cloned.len(), 2);
    }

    #[test]
    fn test_action_sequence_complex() {
        let seq = ActionSequence::new()
            .click(100, 50) // Click on input
            .type_text("hello") // Type 5 chars
            .wait_ms(100) // Wait
            .press(Key::Tab) // Next field
            .type_text("world") // Type 5 chars
            .press(Key::Enter); // Submit

        // 1 + 5 + 1 + 1 + 5 + 1 = 14
        assert_eq!(seq.len(), 14);
    }
}
