use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use super::dispatcher::EventDispatcher;
use super::handler::{CustomHandlerId, HandlerOptions};
use super::response::EventResponse;
use super::result::DispatchResult;
use super::types::{CustomEvent, EventId, EventMeta};

/// Global event bus for application-wide events
pub struct CustomEventBus {
    dispatcher: EventDispatcher,
    /// Event history for debugging (use VecDeque for O(1) front removal)
    history: Arc<RwLock<VecDeque<EventRecord>>>,
    /// Maximum history size
    max_history: usize,
}

/// Record of a dispatched event
#[derive(Debug, Clone)]
pub struct EventRecord {
    /// Event ID
    pub id: EventId,
    /// Event type name
    pub event_type: &'static str,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Was cancelled
    pub cancelled: bool,
    /// Handler count
    pub handler_count: usize,
}

impl CustomEventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            dispatcher: EventDispatcher::new(),
            history: Arc::new(RwLock::new(VecDeque::new())),
            max_history: 100,
        }
    }

    /// Set maximum history size
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Register an event handler
    pub fn on<E: CustomEvent>(
        &mut self,
        handler: impl Fn(&E, &mut EventMeta) -> EventResponse + Send + Sync + 'static,
    ) -> CustomHandlerId {
        self.dispatcher.on(handler)
    }

    /// Register with options
    pub fn on_with_options<E: CustomEvent>(
        &mut self,
        handler: impl Fn(&E, &mut EventMeta) -> EventResponse + Send + Sync + 'static,
        options: HandlerOptions,
    ) -> CustomHandlerId {
        self.dispatcher.on_with_options(handler, options)
    }

    /// Register a one-time handler
    pub fn once<E: CustomEvent>(
        &mut self,
        handler: impl Fn(&E, &mut EventMeta) -> EventResponse + Send + Sync + 'static,
    ) -> CustomHandlerId {
        self.dispatcher.once(handler)
    }

    /// Remove a handler
    pub fn off(&mut self, handler_id: CustomHandlerId) {
        self.dispatcher.off(handler_id);
    }

    /// Dispatch an event
    pub fn emit<E: CustomEvent>(&self, event: E) -> DispatchResult {
        let result = self.dispatcher.dispatch(event);

        // Record in history
        if let Ok(mut history) = self.history.write() {
            history.push_back(EventRecord {
                id: result.event_id,
                event_type: E::event_type(),
                timestamp: SystemTime::now(),
                cancelled: result.cancelled,
                handler_count: result.handler_count,
            });

            // Trim history (O(1) with VecDeque::pop_front)
            while history.len() > self.max_history {
                history.pop_front();
            }
        }

        result
    }

    /// Get event history
    pub fn history(&self) -> Vec<EventRecord> {
        self.history
            .read()
            .map(|h| h.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Clear history
    pub fn clear_history(&self) {
        if let Ok(mut history) = self.history.write() {
            history.clear();
        }
    }

    /// Clear all handlers
    pub fn clear(&mut self) {
        self.dispatcher.clear();
    }
}

impl Default for CustomEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::event::custom::events::AppEvent;
    use crate::runtime::event::custom::response::EventResponse;
    use crate::runtime::event::custom::types::EventMeta;

    #[test]
    fn test_event_bus_new() {
        let bus = CustomEventBus::new();
        let history = bus.history();
        assert!(history.is_empty());
    }

    #[test]
    fn test_event_bus_default() {
        let bus = CustomEventBus::default();
        let history = bus.history();
        assert!(history.is_empty());
    }

    #[test]
    fn test_event_bus_with_max_history() {
        let bus = CustomEventBus::new().with_max_history(50);
        // Can't directly access max_history, but we can verify it works by adding events
        assert!(bus.history().is_empty());
    }

    #[test]
    fn test_event_bus_emit() {
        let bus = CustomEventBus::new();
        let event = AppEvent::new("test_event");
        let result = bus.emit(event);
        assert!(!result.cancelled);
        let history = bus.history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].event_type, "app");
    }

    #[test]
    fn test_event_bus_emit_with_history_limit() {
        let bus = CustomEventBus::new().with_max_history(3);
        for i in 0..5 {
            let event = AppEvent::new(format!("event_{}", i));
            bus.emit(event);
        }
        let history = bus.history();
        assert_eq!(history.len(), 3); // Should be limited to max_history
    }

    #[test]
    fn test_event_bus_on_handler() {
        let mut bus = CustomEventBus::new();
        let handler_id = bus.on(|_event: &AppEvent, _meta: &mut EventMeta| EventResponse::Handled);
        bus.emit(AppEvent::new("test"));
        let history = bus.history();
        assert_eq!(history.len(), 1);
        // Handler was called (handler_count should be 1)
        assert_eq!(history[0].handler_count, 1);
        bus.off(handler_id);
    }

    #[test]
    fn test_event_bus_once_handler() {
        let mut bus = CustomEventBus::new();
        let handler_id =
            bus.once(|_event: &AppEvent, _meta: &mut EventMeta| EventResponse::Handled);

        // First emit - handler should be called
        bus.emit(AppEvent::new("test1"));
        let history1 = bus.history();
        assert_eq!(history1.len(), 1);
        assert_eq!(history1[0].handler_count, 1);

        // Second emit - handler should not be called (removed after once)
        bus.emit(AppEvent::new("test2"));
        let history2 = bus.history();
        assert_eq!(history2.len(), 2);
        assert_eq!(history2[1].handler_count, 0); // No handlers called

        bus.off(handler_id);
    }

    #[test]
    fn test_event_bus_off_handler() {
        let mut bus = CustomEventBus::new();
        let handler_id = bus.on(|_event: &AppEvent, _meta: &mut EventMeta| EventResponse::Handled);

        // Emit with handler
        bus.emit(AppEvent::new("test1"));
        let history1 = bus.history();
        assert_eq!(history1[0].handler_count, 1);

        // Remove handler
        bus.off(handler_id);

        // Emit without handler
        bus.emit(AppEvent::new("test2"));
        let history2 = bus.history();
        assert_eq!(history2.len(), 2);
        assert_eq!(history2[1].handler_count, 0);
    }

    #[test]
    fn test_event_bus_clear_history() {
        let bus = CustomEventBus::new();
        bus.emit(AppEvent::new("test1"));
        bus.emit(AppEvent::new("test2"));
        assert_eq!(bus.history().len(), 2);

        bus.clear_history();
        assert!(bus.history().is_empty());
    }

    #[test]
    fn test_event_bus_clear() {
        let mut bus = CustomEventBus::new();
        bus.on(|_event: &AppEvent, _meta: &mut EventMeta| EventResponse::Handled);
        bus.emit(AppEvent::new("test"));

        bus.clear();
        // After clear, no handlers should be called
        bus.emit(AppEvent::new("test2"));
        let history = bus.history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[1].handler_count, 0);
    }

    #[test]
    fn test_event_record_fields() {
        let bus = CustomEventBus::new();
        bus.emit(AppEvent::new("test"));

        let history = bus.history();
        assert_eq!(history.len(), 1);
        let record = &history[0];

        // Verify public fields are accessible
        assert!(record.id.value() > 0);
        assert_eq!(record.event_type, "app");
        assert!(!record.cancelled);
        assert_eq!(record.handler_count, 0); // No handlers registered
    }

    #[test]
    fn test_event_bus_multiple_handlers() {
        let mut bus = CustomEventBus::new();
        bus.on(|_event: &AppEvent, _meta: &mut EventMeta| EventResponse::Handled);
        bus.on(|_event: &AppEvent, _meta: &mut EventMeta| EventResponse::Handled);

        bus.emit(AppEvent::new("test"));
        let history = bus.history();
        assert_eq!(history[0].handler_count, 2);
    }

    #[test]
    fn test_event_bus_builder_pattern() {
        let bus = CustomEventBus::new().with_max_history(10);
        bus.emit(AppEvent::new("test"));
        assert!(!bus.history().is_empty());
    }
}
