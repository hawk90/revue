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

// Tests moved to tests/event_tests.rs
