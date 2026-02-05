use std::any::Any;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime};

/// Trait for custom events
pub trait CustomEvent: Any + Send + Sync + 'static {
    /// Get the event type name
    fn event_type() -> &'static str
    where
        Self: Sized;

    /// Whether this event can be cancelled
    fn cancellable() -> bool
    where
        Self: Sized,
    {
        true
    }

    /// Whether this event bubbles up the component tree
    fn bubbles() -> bool
    where
        Self: Sized,
    {
        true
    }
}

/// Unique event ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(u64);

impl EventId {
    /// Generate a new unique event ID
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Get the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EventPriority {
    /// Low priority - processed last
    Low = 0,
    /// Normal priority (default)
    #[default]
    Normal = 1,
    /// High priority - processed first
    High = 2,
    /// Critical priority - always processed first
    Critical = 3,
}

/// Event phase in the dispatch lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DispatchPhase {
    /// Capturing phase (root to target)
    Capture,
    /// At the target element
    #[default]
    Target,
    /// Bubbling phase (target to root)
    Bubble,
}

/// Event metadata
#[derive(Debug, Clone)]
pub struct EventMeta {
    /// Unique event ID
    pub id: EventId,
    /// Event type name
    pub event_type: &'static str,
    /// When the event was created
    pub timestamp: SystemTime,
    /// Creation instant for timing
    pub instant: Instant,
    /// Source component/widget ID
    pub source: Option<String>,
    /// Target component/widget ID
    pub target: Option<String>,
    /// Current dispatch phase
    pub phase: DispatchPhase,
    /// Priority level
    pub priority: EventPriority,
    /// Whether the event has been cancelled
    cancelled: bool,
    /// Whether propagation has been stopped
    propagation_stopped: bool,
    /// Whether immediate propagation has been stopped
    immediate_propagation_stopped: bool,
}

impl EventMeta {
    /// Create new event metadata
    pub fn new(event_type: &'static str) -> Self {
        Self {
            id: EventId::new(),
            event_type,
            timestamp: SystemTime::now(),
            instant: Instant::now(),
            source: None,
            target: None,
            phase: DispatchPhase::Target,
            priority: EventPriority::Normal,
            cancelled: false,
            propagation_stopped: false,
            immediate_propagation_stopped: false,
        }
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set target
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Cancel the event (prevents default action)
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    /// Check if event is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Stop event propagation (no more bubbling/capturing)
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    /// Check if propagation is stopped
    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }

    /// Stop immediate propagation (no more handlers on this element)
    pub fn stop_immediate_propagation(&mut self) {
        self.immediate_propagation_stopped = true;
        self.propagation_stopped = true;
    }

    /// Check if immediate propagation is stopped
    pub fn is_immediate_propagation_stopped(&self) -> bool {
        self.immediate_propagation_stopped
    }

    /// Get elapsed time since event creation
    pub fn elapsed(&self) -> std::time::Duration {
        self.instant.elapsed()
    }
}

/// Event envelope containing the event and its metadata
pub struct EventEnvelope<E: CustomEvent> {
    /// The event data
    pub event: E,
    /// Event metadata
    pub meta: EventMeta,
}

impl<E: CustomEvent> EventEnvelope<E> {
    /// Create a new event envelope
    pub fn new(event: E) -> Self {
        Self {
            meta: EventMeta::new(E::event_type()),
            event,
        }
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.meta = self.meta.with_source(source);
        self
    }

    /// Set target
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.meta = self.meta.with_target(target);
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.meta = self.meta.with_priority(priority);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock event for testing
    #[derive(Debug, Clone)]
    struct TestEvent {
        pub data: String,
    }

    impl CustomEvent for TestEvent {
        fn event_type() -> &'static str {
            "test"
        }
    }

    #[test]
    fn test_event_id_new() {
        let id1 = EventId::new();
        let id2 = EventId::new();
        assert_ne!(id1, id2);
        assert!(id2.value() > id1.value());
    }

    #[test]
    fn test_event_id_value() {
        let id = EventId::new();
        assert_eq!(id.value(), id.0);
    }

    #[test]
    fn test_event_id_default() {
        let id = EventId::default();
        assert!(id.value() > 0);
    }

    #[test]
    fn test_event_priority_ordering() {
        assert!(EventPriority::Critical > EventPriority::High);
        assert!(EventPriority::High > EventPriority::Normal);
        assert!(EventPriority::Normal > EventPriority::Low);
    }

    #[test]
    fn test_dispatch_phase_default() {
        assert_eq!(DispatchPhase::default(), DispatchPhase::Target);
    }

    #[test]
    fn test_event_meta_new() {
        let meta = EventMeta::new("test_event");
        assert_eq!(meta.event_type, "test_event");
        assert!(meta.source.is_none());
        assert!(meta.target.is_none());
        assert_eq!(meta.phase, DispatchPhase::Target);
        assert_eq!(meta.priority, EventPriority::Normal);
        assert!(!meta.is_cancelled());
        assert!(!meta.is_propagation_stopped());
    }

    #[test]
    fn test_event_meta_with_source() {
        let meta = EventMeta::new("test").with_source("widget_1");
        assert_eq!(meta.source, Some("widget_1".to_string()));
    }

    #[test]
    fn test_event_meta_with_target() {
        let meta = EventMeta::new("test").with_target("widget_2");
        assert_eq!(meta.target, Some("widget_2".to_string()));
    }

    #[test]
    fn test_event_meta_with_priority() {
        let meta = EventMeta::new("test").with_priority(EventPriority::High);
        assert_eq!(meta.priority, EventPriority::High);
    }

    #[test]
    fn test_event_meta_cancel() {
        let mut meta = EventMeta::new("test");
        assert!(!meta.is_cancelled());
        meta.cancel();
        assert!(meta.is_cancelled());
    }

    #[test]
    fn test_event_meta_stop_propagation() {
        let mut meta = EventMeta::new("test");
        assert!(!meta.is_propagation_stopped());
        meta.stop_propagation();
        assert!(meta.is_propagation_stopped());
    }

    #[test]
    fn test_event_meta_stop_immediate_propagation() {
        let mut meta = EventMeta::new("test");
        assert!(!meta.is_immediate_propagation_stopped());
        assert!(!meta.is_propagation_stopped());
        meta.stop_immediate_propagation();
        assert!(meta.is_immediate_propagation_stopped());
        assert!(meta.is_propagation_stopped());
    }

    #[test]
    fn test_event_meta_elapsed() {
        let meta = EventMeta::new("test");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = meta.elapsed();
        assert!(elapsed.as_millis() >= 10);
    }

    #[test]
    fn test_event_meta_builder_pattern() {
        let meta = EventMeta::new("test")
            .with_source("src")
            .with_target("dest")
            .with_priority(EventPriority::Critical);
        assert_eq!(meta.source, Some("src".to_string()));
        assert_eq!(meta.target, Some("dest".to_string()));
        assert_eq!(meta.priority, EventPriority::Critical);
    }

    #[test]
    fn test_event_envelope_new() {
        let event = TestEvent {
            data: "test".to_string(),
        };
        let envelope = EventEnvelope::new(event);
        assert_eq!(envelope.event.data, "test");
        assert_eq!(envelope.meta.event_type, "test");
    }

    #[test]
    fn test_event_envelope_with_source() {
        let event = TestEvent {
            data: "test".to_string(),
        };
        let envelope = EventEnvelope::new(event).with_source("source");
        assert_eq!(envelope.meta.source, Some("source".to_string()));
    }

    #[test]
    fn test_event_envelope_with_target() {
        let event = TestEvent {
            data: "test".to_string(),
        };
        let envelope = EventEnvelope::new(event).with_target("target");
        assert_eq!(envelope.meta.target, Some("target".to_string()));
    }

    #[test]
    fn test_event_envelope_with_priority() {
        let event = TestEvent {
            data: "test".to_string(),
        };
        let envelope = EventEnvelope::new(event).with_priority(EventPriority::Low);
        assert_eq!(envelope.meta.priority, EventPriority::Low);
    }

    #[test]
    fn test_event_envelope_builder_pattern() {
        let event = TestEvent {
            data: "test".to_string(),
        };
        let envelope = EventEnvelope::new(event)
            .with_source("src")
            .with_target("dest")
            .with_priority(EventPriority::High);
        assert_eq!(envelope.meta.source, Some("src".to_string()));
        assert_eq!(envelope.meta.target, Some("dest".to_string()));
        assert_eq!(envelope.meta.priority, EventPriority::High);
    }

    #[test]
    fn test_custom_event_default_bubbles() {
        assert!(TestEvent::bubbles());
    }

    #[test]
    fn test_custom_event_default_cancellable() {
        assert!(TestEvent::cancellable());
    }
}
