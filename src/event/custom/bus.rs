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
    /// Event history for debugging
    history: Arc<RwLock<Vec<EventRecord>>>,
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
            history: Arc::new(RwLock::new(Vec::new())),
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
            history.push(EventRecord {
                id: result.event_id,
                event_type: E::event_type(),
                timestamp: SystemTime::now(),
                cancelled: result.cancelled,
                handler_count: result.handler_count,
            });

            // Trim history
            while history.len() > self.max_history {
                history.remove(0);
            }
        }

        result
    }

    /// Get event history
    pub fn history(&self) -> Vec<EventRecord> {
        self.history.read().map(|h| h.clone()).unwrap_or_default()
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
