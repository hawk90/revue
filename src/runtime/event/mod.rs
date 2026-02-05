//! Event handling and keyboard input for TUI applications
//!
//! This module provides comprehensive event handling including keyboard, mouse,
//! drag-and-drop, gestures, focus management, and custom events.
//!
//! # Core Event Types
//!
//! | Event | Description | Use Case |
//! |-------|-------------|----------|
//! | [`KeyEvent`] | Keyboard input | All keyboard interaction |
//! | [`MouseEvent`] | Mouse input | Clicks, scrolls, movement |
//! | [`Event`] | Unified event type | Main event loop handling |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! struct MyApp;
//!
//! impl MyApp {
//!     fn handle_event(&mut self, event: &Event) -> bool {
//!         match event {
//!             Event::Key(key) => self.handle_key(key),
//!             Event::Mouse(mouse) => self.handle_mouse(mouse),
//!             Event::Resize(width, height) => self.handle_resize(*width, *height),
//!             Event::Tick => self.update_animation(),
//!         }
//!     }
//! }
//! ```
//!
//! # Keyboard Input
//!
//! ```rust,ignore
//! use revue::event::{Key, KeyEvent, Modifiers};
//!
//! fn handle_key(key: &KeyEvent) {
//!     match key.key {
//!         Key::Char('c') if key.ctrl => println!("Ctrl+C pressed"),
//!         Key::Char('q') => println!("Quit"),
//!         Key::Up => println!("Arrow up"),
//!         Key::Enter => println!("Enter"),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! # Mouse Input
//!
//! ```rust,ignore
//! use revue::event::{MouseEvent, MouseEventKind, MouseButton};
//!
//! fn handle_mouse(mouse: &MouseEvent) {
//!     match mouse.kind {
//!         MouseEventKind::Down(MouseButton::Left) => {
//!             println!("Click at {}, {}", mouse.x, mouse.y);
//!         }
//!         MouseEventKind::ScrollUp => {
//!             println!("Scroll up");
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//!
//! # Drag and Drop
//!
//! ```rust,ignore
//! use revue::event::{start_drag, DragData, DragId};
//!
//! // Start dragging
//! let drag_id = start_drag(
//!     DragId::new(1),
//!     DragData::new("my_data")
//! );
//!
//! // Check if dragging
//! if is_dragging(drag_id) {
//!     // Update drag position
//! }
//!
//! // End drag with result
//! end_drag(drag_id);
//! ```
//!
//! # Focus Management
//!
//! ```rust,ignore
//! use revue::event::{FocusManager, Direction};
//!
//! let mut focus = FocusManager::new();
//!
//! // Add widgets to focus system
//! focus.register("input1");
//! focus.register("input2");
//!
//! // Navigate focus
//! focus.move_focus(Direction::Forward);  // Tab
//! focus.move_focus(Direction::Backward); // Shift+Tab
//! ```
//!
//! # Gestures
//!
//! ```rust,ignore
//! use revue::event::{Gesture, SwipeGesture, TapGesture};
//!
//! // Tap gesture
//! let tap = TapGesture::new()
//!     .on_tap(|point| println!("Tap at {:?}", point));
//!
//! // Swipe gesture
//! let swipe = SwipeGesture::new()
//!     .on_swipe(|direction| println!("Swiped {:?}", direction));
//! ```
//!
//! # Custom Events
//!
//! ```rust,ignore
//! use revue::event::{CustomEvent, EventDispatcher};
//!
//! #[derive(Debug, Clone)]
//! pub struct MyEvent {
//!     pub data: String,
//! }
//!
//! impl CustomEvent for MyEvent {
//!     fn id(&self) -> &'static str { "my_event" }
//! }
//!
//! // Dispatch custom event
//! dispatcher.dispatch(MyEvent { data: "Hello".to_string() });
//! ```

pub mod click;
pub mod custom;
pub mod drag;
mod focus;
pub mod gesture;
mod handler;
pub mod ime;
mod keymap;
mod reader;

pub use click::{ClickDetector, ClickType};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_event_new() {
        let event = MouseEvent::new(10, 20, MouseEventKind::Down(MouseButton::Left));
        assert_eq!(event.x, 10);
        assert_eq!(event.y, 20);
        assert!(!event.ctrl);
        assert!(!event.alt);
        assert!(!event.shift);
    }

    #[test]
    fn test_mouse_event_is_left_click() {
        let event = MouseEvent::new(0, 0, MouseEventKind::Down(MouseButton::Left));
        assert!(event.is_left_click());

        let event = MouseEvent::new(0, 0, MouseEventKind::Down(MouseButton::Right));
        assert!(!event.is_left_click());

        let event = MouseEvent::new(0, 0, MouseEventKind::Up(MouseButton::Left));
        assert!(!event.is_left_click());
    }

    #[test]
    fn test_mouse_event_is_right_click() {
        let event = MouseEvent::new(0, 0, MouseEventKind::Down(MouseButton::Right));
        assert!(event.is_right_click());

        let event = MouseEvent::new(0, 0, MouseEventKind::Down(MouseButton::Left));
        assert!(!event.is_right_click());
    }

    #[test]
    fn test_mouse_event_is_scroll() {
        assert!(MouseEvent::new(0, 0, MouseEventKind::ScrollUp).is_scroll());
        assert!(MouseEvent::new(0, 0, MouseEventKind::ScrollDown).is_scroll());
        assert!(MouseEvent::new(0, 0, MouseEventKind::ScrollLeft).is_scroll());
        assert!(MouseEvent::new(0, 0, MouseEventKind::ScrollRight).is_scroll());
        assert!(!MouseEvent::new(0, 0, MouseEventKind::Move).is_scroll());
    }

    #[test]
    fn test_key_event_new() {
        let event = KeyEvent::new(Key::Char('a'));
        assert_eq!(event.key, Key::Char('a'));
        assert!(!event.ctrl);
        assert!(!event.alt);
        assert!(!event.shift);
    }

    #[test]
    fn test_key_event_ctrl() {
        let event = KeyEvent::ctrl(Key::Char('c'));
        assert!(event.ctrl);
        assert!(!event.alt);
        assert!(!event.shift);
    }

    #[test]
    fn test_key_event_alt() {
        let event = KeyEvent::alt(Key::Char('x'));
        assert!(!event.ctrl);
        assert!(event.alt);
        assert!(!event.shift);
    }

    #[test]
    fn test_key_event_is_ctrl_c() {
        let event = KeyEvent::ctrl(Key::Char('c'));
        assert!(event.is_ctrl_c());

        let event = KeyEvent::new(Key::Char('c'));
        assert!(!event.is_ctrl_c());
    }

    #[test]
    fn test_key_event_is_escape() {
        let event = KeyEvent::new(Key::Escape);
        assert!(event.is_escape());

        let event = KeyEvent::new(Key::Enter);
        assert!(!event.is_escape());
    }

    #[test]
    fn test_key_event_is_enter() {
        let event = KeyEvent::new(Key::Enter);
        assert!(event.is_enter());

        let event = KeyEvent::new(Key::Tab);
        assert!(!event.is_enter());
    }

    #[test]
    fn test_key_event_is_tab() {
        let event = KeyEvent::new(Key::Tab);
        assert!(event.is_tab());

        let event = KeyEvent {
            key: Key::Tab,
            ctrl: false,
            alt: false,
            shift: true,
        };
        assert!(!event.is_tab());
    }

    #[test]
    fn test_key_event_is_shift_tab() {
        let event = KeyEvent {
            key: Key::Tab,
            ctrl: false,
            alt: false,
            shift: true,
        };
        assert!(event.is_shift_tab());

        let event = KeyEvent::new(Key::Tab);
        assert!(!event.is_shift_tab());
    }

    #[test]
    fn test_key_event_to_binding() {
        let event = KeyEvent {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: true,
        };
        let binding = event.to_binding();
        assert_eq!(binding.key, Key::Char('a'));
        assert!(binding.ctrl);
        assert!(!binding.alt);
        assert!(binding.shift);
    }

    #[test]
    fn test_event_enum_variants() {
        let key_event = Event::Key(KeyEvent::new(Key::Char('a')));
        let mouse_event = Event::Mouse(MouseEvent::new(0, 0, MouseEventKind::Move));
        let resize_event = Event::Resize(80, 24);
        let tick_event = Event::Tick;
        let focus_gained = Event::FocusGained;
        let focus_lost = Event::FocusLost;
        let paste_event = Event::Paste("test".to_string());

        // Just verify they all compile - variants exist
        match key_event {
            Event::Key(_) => {}
            _ => panic!("Expected Key event"),
        }
        match mouse_event {
            Event::Mouse(_) => {}
            _ => panic!("Expected Mouse event"),
        }
        match resize_event {
            Event::Resize(_, _) => {}
            _ => panic!("Expected Resize event"),
        }
        match tick_event {
            Event::Tick => {}
            _ => panic!("Expected Tick event"),
        }
        match focus_gained {
            Event::FocusGained => {}
            _ => panic!("Expected FocusGained event"),
        }
        match focus_lost {
            Event::FocusLost => {}
            _ => panic!("Expected FocusLost event"),
        }
        match paste_event {
            Event::Paste(_) => {}
            _ => panic!("Expected Paste event"),
        }
    }

    #[test]
    fn test_mouse_button_variants() {
        let _left = MouseButton::Left;
        let _right = MouseButton::Right;
        let _middle = MouseButton::Middle;
    }

    #[test]
    fn test_mouse_event_kind_variants() {
        let _down = MouseEventKind::Down(MouseButton::Left);
        let _up = MouseEventKind::Up(MouseButton::Right);
        let _drag = MouseEventKind::Drag(MouseButton::Middle);
        let _move = MouseEventKind::Move;
        let _scroll_down = MouseEventKind::ScrollDown;
        let _scroll_up = MouseEventKind::ScrollUp;
        let _scroll_left = MouseEventKind::ScrollLeft;
        let _scroll_right = MouseEventKind::ScrollRight;
    }

    #[test]
    fn test_mouse_event_with_modifiers() {
        let event = MouseEvent {
            x: 10,
            y: 20,
            kind: MouseEventKind::Down(MouseButton::Left),
            ctrl: true,
            alt: false,
            shift: true,
        };
        assert!(event.ctrl);
        assert!(event.shift);
        assert!(!event.alt);
    }
}
