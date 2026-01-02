//! Event handling and keyboard input

pub mod drag;
mod focus;
mod handler;
mod keymap;
mod reader;

pub use drag::{
    cancel_drag, drag_context, end_drag, is_dragging, start_drag, update_drag_position,
    DragContext, DragData, DragId, DragState, DropResult, DropTarget,
};
pub use focus::{Direction, FocusManager, FocusTrap, FocusTrapConfig, WidgetId};
pub use handler::{EventContext, EventHandler, EventPhase, HandlerId};
pub use keymap::{Key, KeyBinding, KeyMap};
pub use reader::EventReader;

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (scroll wheel click)
    Middle,
}

/// Mouse event kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    /// Button pressed down
    Down(MouseButton),
    /// Button released
    Up(MouseButton),
    /// Mouse dragged while button held
    Drag(MouseButton),
    /// Mouse moved (no button pressed)
    Move,
    /// Scroll wheel down
    ScrollDown,
    /// Scroll wheel up
    ScrollUp,
}

/// Mouse event with position and modifiers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    /// X coordinate (column)
    pub x: u16,
    /// Y coordinate (row)
    pub y: u16,
    /// Kind of mouse event
    pub kind: MouseEventKind,
    /// Control modifier held
    pub ctrl: bool,
    /// Alt modifier held
    pub alt: bool,
    /// Shift modifier held
    pub shift: bool,
}

impl MouseEvent {
    /// Create a new mouse event
    pub fn new(x: u16, y: u16, kind: MouseEventKind) -> Self {
        Self {
            x,
            y,
            kind,
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    /// Check if this is a left click (down)
    pub fn is_left_click(&self) -> bool {
        matches!(self.kind, MouseEventKind::Down(MouseButton::Left))
    }

    /// Check if this is a right click (down)
    pub fn is_right_click(&self) -> bool {
        matches!(self.kind, MouseEventKind::Down(MouseButton::Right))
    }

    /// Check if this is a scroll event
    pub fn is_scroll(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::ScrollDown | MouseEventKind::ScrollUp
        )
    }
}

/// Application event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Keyboard event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize event
    Resize(u16, u16),
    /// Tick event (for animations, updates)
    Tick,
}

/// Key press event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key that was pressed
    pub key: Key,
    /// Control modifier
    pub ctrl: bool,
    /// Alt modifier
    pub alt: bool,
    /// Shift modifier
    pub shift: bool,
}

impl KeyEvent {
    /// Create a new key event
    pub fn new(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    /// Create with control modifier
    pub fn ctrl(key: Key) -> Self {
        Self {
            key,
            ctrl: true,
            alt: false,
            shift: false,
        }
    }

    /// Create with alt modifier
    pub fn alt(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: true,
            shift: false,
        }
    }

    /// Check if this is Ctrl+C
    pub fn is_ctrl_c(&self) -> bool {
        self.ctrl && self.key == Key::Char('c')
    }

    /// Check if this is Escape
    pub fn is_escape(&self) -> bool {
        self.key == Key::Escape
    }

    /// Check if this is Enter
    pub fn is_enter(&self) -> bool {
        self.key == Key::Enter
    }

    /// Check if this is Tab
    pub fn is_tab(&self) -> bool {
        self.key == Key::Tab && !self.shift
    }

    /// Check if this is Shift+Tab
    pub fn is_shift_tab(&self) -> bool {
        self.key == Key::Tab && self.shift
    }

    /// Convert to KeyBinding for keymap lookup
    pub fn to_binding(&self) -> KeyBinding {
        KeyBinding {
            key: self.key,
            ctrl: self.ctrl,
            alt: self.alt,
            shift: self.shift,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_event_new() {
        let event = KeyEvent::new(Key::Enter);
        assert_eq!(event.key, Key::Enter);
        assert!(!event.ctrl);
        assert!(!event.alt);
        assert!(!event.shift);
    }

    #[test]
    fn test_key_event_ctrl() {
        let event = KeyEvent::ctrl(Key::Char('c'));
        assert_eq!(event.key, Key::Char('c'));
        assert!(event.ctrl);
        assert!(event.is_ctrl_c());
    }

    #[test]
    fn test_key_event_checks() {
        assert!(KeyEvent::new(Key::Escape).is_escape());
        assert!(KeyEvent::new(Key::Enter).is_enter());
        assert!(KeyEvent::new(Key::Tab).is_tab());

        let shift_tab = KeyEvent {
            key: Key::Tab,
            ctrl: false,
            alt: false,
            shift: true,
        };
        assert!(shift_tab.is_shift_tab());
        assert!(!shift_tab.is_tab());
    }

    #[test]
    fn test_key_event_to_binding() {
        let event = KeyEvent::ctrl(Key::Char('s'));
        let binding = event.to_binding();

        assert_eq!(binding.key, Key::Char('s'));
        assert!(binding.ctrl);
    }
}
