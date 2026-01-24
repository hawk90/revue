use std::any::TypeId;
use std::sync::atomic::{AtomicU64, Ordering};

use super::response::EventResponse;
use super::types::{EventMeta, EventPriority};

/// Unique handler ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomHandlerId(u64);

impl CustomHandlerId {
    /// Generate a new unique handler ID
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for CustomHandlerId {
    fn default() -> Self {
        Self::new()
    }
}

/// Handler options
#[derive(Debug, Clone, Default)]
pub struct HandlerOptions {
    /// Handle during capture phase
    pub capture: bool,
    /// Remove after first invocation
    pub once: bool,
    /// Priority for handler ordering
    pub priority: EventPriority,
}

impl HandlerOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set capture mode
    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = capture;
        self
    }

    /// Set once mode
    pub fn once(mut self, once: bool) -> Self {
        self.once = once;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }
}

// Internal handler wrapper
pub type BoxedHandler =
    Box<dyn Fn(&dyn std::any::Any, &mut EventMeta) -> EventResponse + Send + Sync>;

pub struct HandlerEntry {
    pub id: CustomHandlerId,
    pub handler: BoxedHandler,
    pub options: HandlerOptions,
    pub type_id: TypeId,
}
