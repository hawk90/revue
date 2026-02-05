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
    #[allow(dead_code)]
    pub type_id: TypeId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_handler_id_new() {
        let id1 = CustomHandlerId::new();
        let id2 = CustomHandlerId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_custom_handler_id_default() {
        let id = CustomHandlerId::default();
        assert!(id.0 > 0);
    }

    #[test]
    fn test_handler_options_new() {
        let options = HandlerOptions::new();
        assert!(!options.capture);
        assert!(!options.once);
        assert_eq!(options.priority, EventPriority::Normal);
    }

    #[test]
    fn test_handler_options_default() {
        let options = HandlerOptions::default();
        assert!(!options.capture);
        assert!(!options.once);
        assert_eq!(options.priority, EventPriority::Normal);
    }

    #[test]
    fn test_handler_options_capture() {
        let options = HandlerOptions::new().capture(true);
        assert!(options.capture);
        assert!(!options.once);
    }

    #[test]
    fn test_handler_options_once() {
        let options = HandlerOptions::new().once(true);
        assert!(options.once);
        assert!(!options.capture);
    }

    #[test]
    fn test_handler_options_priority() {
        let options = HandlerOptions::new().priority(EventPriority::High);
        assert_eq!(options.priority, EventPriority::High);
    }

    #[test]
    fn test_handler_options_builder_pattern() {
        let options = HandlerOptions::new()
            .capture(true)
            .once(false)
            .priority(EventPriority::Critical);
        assert!(options.capture);
        assert!(!options.once);
        assert_eq!(options.priority, EventPriority::Critical);
    }

    #[test]
    fn test_handler_options_all_true() {
        let options = HandlerOptions::new()
            .capture(true)
            .once(true)
            .priority(EventPriority::Low);
        assert!(options.capture);
        assert!(options.once);
        assert_eq!(options.priority, EventPriority::Low);
    }
}
