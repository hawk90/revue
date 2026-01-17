//! Event handler with bubbling and propagation control

use super::Event;

/// Event propagation context
///
/// Provides control over event propagation similar to DOM events.
/// Use `stop_propagation()` to prevent the event from bubbling up,
/// and `prevent_default()` to cancel default behavior.
#[derive(Default)]
pub struct EventContext {
    /// Whether propagation has been stopped
    propagation_stopped: bool,
    /// Whether default action should be prevented
    default_prevented: bool,
    /// Whether the event was handled
    handled: bool,
}

impl EventContext {
    /// Create a new event context
    pub fn new() -> Self {
        Self::default()
    }

    /// Stop the event from propagating to parent handlers
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    /// Check if propagation was stopped
    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }

    /// Prevent the default action for this event
    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    /// Check if default was prevented
    pub fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    /// Mark the event as handled
    pub fn set_handled(&mut self) {
        self.handled = true;
    }

    /// Check if the event was handled
    pub fn is_handled(&self) -> bool {
        self.handled
    }
}

/// Event phase for bubbling
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventPhase {
    /// Capturing phase (top-down)
    Capture,
    /// Target phase (at target)
    Target,
    /// Bubbling phase (bottom-up)
    Bubble,
}

/// Handler ID for removal
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HandlerId(u64);

/// Type alias for event handler function with context
type HandlerFn = Box<dyn Fn(&Event, &mut EventContext) -> bool>;

/// Type alias for simple event handler (no context)
type SimpleHandlerFn = Box<dyn Fn(&Event) -> bool>;

/// Event handler registry with bubbling support
pub struct EventHandler {
    /// Capture phase handlers (called first, top-down)
    capture_handlers: Vec<(HandlerId, HandlerFn)>,
    /// Bubble phase handlers (called last, bottom-up)
    bubble_handlers: Vec<(HandlerId, HandlerFn)>,
    /// Simple handlers (for backward compatibility)
    simple_handlers: Vec<(HandlerId, SimpleHandlerFn)>,
    /// Next handler ID
    next_id: u64,
}

impl EventHandler {
    /// Create a new event handler registry
    pub fn new() -> Self {
        Self {
            capture_handlers: Vec::new(),
            bubble_handlers: Vec::new(),
            simple_handlers: Vec::new(),
            next_id: 1,
        }
    }

    /// Generate next handler ID
    fn next_handler_id(&mut self) -> HandlerId {
        let id = HandlerId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Register a bubble phase event handler (returns ID for removal)
    pub fn on<F>(&mut self, handler: F) -> HandlerId
    where
        F: Fn(&Event) -> bool + 'static,
    {
        let id = self.next_handler_id();
        self.simple_handlers.push((id, Box::new(handler)));
        id
    }

    /// Register a handler with context for bubble phase (returns ID for removal)
    pub fn on_bubble<F>(&mut self, handler: F) -> HandlerId
    where
        F: Fn(&Event, &mut EventContext) -> bool + 'static,
    {
        let id = self.next_handler_id();
        self.bubble_handlers.push((id, Box::new(handler)));
        id
    }

    /// Register a handler for capture phase (returns ID for removal)
    pub fn on_capture<F>(&mut self, handler: F) -> HandlerId
    where
        F: Fn(&Event, &mut EventContext) -> bool + 'static,
    {
        let id = self.next_handler_id();
        self.capture_handlers.push((id, Box::new(handler)));
        id
    }

    /// Remove a handler by ID
    pub fn remove(&mut self, id: HandlerId) -> bool {
        // Try to remove from each list
        let len_before =
            self.capture_handlers.len() + self.bubble_handlers.len() + self.simple_handlers.len();

        self.capture_handlers.retain(|(hid, _)| *hid != id);
        self.bubble_handlers.retain(|(hid, _)| *hid != id);
        self.simple_handlers.retain(|(hid, _)| *hid != id);

        let len_after =
            self.capture_handlers.len() + self.bubble_handlers.len() + self.simple_handlers.len();

        len_after < len_before
    }

    /// Dispatch an event with full propagation control
    pub fn dispatch_with_context(&self, event: &Event) -> EventContext {
        let mut ctx = EventContext::new();

        // Capture phase (handlers are called in order)
        for (_, handler) in &self.capture_handlers {
            if ctx.is_propagation_stopped() {
                break;
            }
            if handler(event, &mut ctx) {
                ctx.set_handled();
            }
        }

        // Bubble phase (handlers are called in reverse order)
        for (_, handler) in self.bubble_handlers.iter().rev() {
            if ctx.is_propagation_stopped() {
                break;
            }
            if handler(event, &mut ctx) {
                ctx.set_handled();
            }
        }

        ctx
    }

    /// Dispatch an event to handlers (simple, backward compatible)
    pub fn dispatch(&self, event: &Event) -> bool {
        // First try simple handlers
        for (_, handler) in &self.simple_handlers {
            if handler(event) {
                return true;
            }
        }

        // Then try context-aware handlers
        let ctx = self.dispatch_with_context(event);
        ctx.is_handled()
    }

    /// Clear all handlers
    pub fn clear(&mut self) {
        self.capture_handlers.clear();
        self.bubble_handlers.clear();
        self.simple_handlers.clear();
    }

    /// Check if there are any handlers
    pub fn is_empty(&self) -> bool {
        self.capture_handlers.is_empty()
            && self.bubble_handlers.is_empty()
            && self.simple_handlers.is_empty()
    }

    /// Get the number of registered handlers
    pub fn handler_count(&self) -> usize {
        self.capture_handlers.len() + self.bubble_handlers.len() + self.simple_handlers.len()
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

// Tests moved to tests/event_tests.rs
