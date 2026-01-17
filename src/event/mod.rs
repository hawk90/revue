//! Event handling and keyboard input

pub mod custom;
pub mod drag;
mod focus;
pub mod gesture;
mod handler;
pub mod ime;
mod keymap;
mod reader;

pub use custom::{
    AppEvent, CustomEvent, CustomEventBus, CustomHandlerId, DispatchPhase, DispatchResult,
    ErrorEvent, EventDispatcher, EventEnvelope, EventId, EventMeta, EventPriority, EventRecord,
    EventResponse, HandlerOptions, NavigateEvent, StateChangeEvent,
};
pub use drag::{
    cancel_drag, drag_context, end_drag, is_dragging, start_drag, update_drag_position,
    DragContext, DragData, DragId, DragState, DropResult, DropTarget,
};
pub use focus::{Direction, FocusManager, FocusTrap, FocusTrapConfig, WidgetId};
pub use gesture::{
    DragGesture, Gesture, GestureConfig, GestureRecognizer, GestureState, LongPressGesture,
    PinchDirection, PinchGesture, SwipeDirection, SwipeGesture, TapGesture,
};
pub use handler::{EventContext, EventHandler, EventPhase, HandlerId};
pub use ime::{
    Candidate, CompositionEvent, CompositionState, CompositionStyle, ImeConfig, ImeState,
    PreeditSegment, PreeditString,
};
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
    /// Scroll wheel left (horizontal)
    ScrollLeft,
    /// Scroll wheel right (horizontal)
    ScrollRight,
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
            MouseEventKind::ScrollDown
                | MouseEventKind::ScrollUp
                | MouseEventKind::ScrollLeft
                | MouseEventKind::ScrollRight
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
    /// Terminal gained focus
    FocusGained,
    /// Terminal lost focus
    FocusLost,
    /// Pasted text (requires bracketed paste mode)
    Paste(String),
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

// Tests moved to tests/event_tests.rs
