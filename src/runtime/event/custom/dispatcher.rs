use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::handler::{BoxedHandler, CustomHandlerId, HandlerEntry, HandlerOptions};
use super::response::EventResponse;
use super::types::{CustomEvent, DispatchPhase, EventEnvelope, EventMeta};
use super::DispatchResult;

/// Thread-safe event dispatcher
///
/// Uses HashMap indexed by TypeId for O(1) handler lookup instead of O(n) linear scan.
pub struct EventDispatcher {
    /// Handlers indexed by event TypeId for O(1) lookup
    /// Each TypeId maps to a list of handlers sorted by priority
    handlers: Arc<RwLock<HashMap<TypeId, Vec<HandlerEntry>>>>,
    pending_removals: Arc<RwLock<Vec<CustomHandlerId>>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            pending_removals: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register an event handler
    pub fn on<E: CustomEvent>(
        &mut self,
        handler: impl Fn(&E, &mut EventMeta) -> EventResponse + Send + Sync + 'static,
    ) -> CustomHandlerId {
        self.on_with_options::<E>(handler, HandlerOptions::default())
    }

    /// Register an event handler with options
    pub fn on_with_options<E: CustomEvent>(
        &mut self,
        handler: impl Fn(&E, &mut EventMeta) -> EventResponse + Send + Sync + 'static,
        options: HandlerOptions,
    ) -> CustomHandlerId {
        let id = CustomHandlerId::new();
        let type_id = TypeId::of::<E>();

        let boxed: BoxedHandler = Box::new(move |any, meta| {
            if let Some(event) = any.downcast_ref::<E>() {
                handler(event, meta)
            } else {
                EventResponse::Ignored
            }
        });

        let entry = HandlerEntry {
            id,
            handler: boxed,
            options,
            type_id,
        };

        if let Ok(mut handlers) = self.handlers.write() {
            // Get or create the handler list for this event type
            let type_handlers = handlers.entry(type_id).or_default();
            type_handlers.push(entry);
            // Sort by priority (higher priority first)
            type_handlers.sort_by(|a, b| b.options.priority.cmp(&a.options.priority));
        }

        id
    }

    /// Register a one-time event handler
    pub fn once<E: CustomEvent>(
        &mut self,
        handler: impl Fn(&E, &mut EventMeta) -> EventResponse + Send + Sync + 'static,
    ) -> CustomHandlerId {
        self.on_with_options::<E>(handler, HandlerOptions::default().once(true))
    }

    /// Remove an event handler
    pub fn off(&mut self, handler_id: CustomHandlerId) {
        if let Ok(mut handlers) = self.handlers.write() {
            // Remove from all type lists
            for type_handlers in handlers.values_mut() {
                type_handlers.retain(|h| h.id != handler_id);
            }
            // Clean up empty type lists
            handlers.retain(|_, list| !list.is_empty());
        }
    }

    /// Remove all handlers for a specific event type
    pub fn off_all<E: CustomEvent>(&mut self) {
        let type_id = TypeId::of::<E>();
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.remove(&type_id);
        }
    }

    /// Clear all handlers
    pub fn clear(&mut self) {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.clear();
        }
    }

    /// Dispatch an event
    pub fn dispatch<E: CustomEvent>(&self, event: E) -> DispatchResult {
        self.dispatch_envelope(EventEnvelope::new(event))
    }

    /// Dispatch an event with custom envelope
    pub fn dispatch_envelope<E: CustomEvent>(&self, envelope: EventEnvelope<E>) -> DispatchResult {
        let EventEnvelope { event, mut meta } = envelope;
        let type_id = TypeId::of::<E>();
        let mut handlers_to_remove = Vec::new();
        let mut handler_count = 0;

        // Get read lock and process handlers - O(1) lookup by TypeId
        let handlers = match self.handlers.read() {
            Ok(h) => h,
            Err(_) => return DispatchResult::error("Failed to acquire lock"),
        };

        // Get handlers for this specific event type (O(1) lookup)
        let type_handlers = match handlers.get(&type_id) {
            Some(h) => h,
            None => {
                // No handlers for this event type
                return DispatchResult {
                    event_id: meta.id,
                    cancelled: false,
                    propagation_stopped: false,
                    handler_count: 0,
                    error: None,
                };
            }
        };

        // Process capture phase handlers first
        for entry in type_handlers.iter().filter(|h| h.options.capture) {
            meta.phase = DispatchPhase::Capture;
            let response = (entry.handler)(&event, &mut meta);
            handler_count += 1;

            if entry.options.once {
                handlers_to_remove.push(entry.id);
            }

            if response.should_cancel() {
                meta.cancel();
            }
            if response.should_stop() {
                meta.stop_propagation();
            }
            if meta.is_propagation_stopped() || meta.is_immediate_propagation_stopped() {
                break;
            }
        }

        // Process target/bubble phase handlers
        if !meta.is_propagation_stopped() {
            for entry in type_handlers.iter().filter(|h| !h.options.capture) {
                meta.phase = DispatchPhase::Target;
                let response = (entry.handler)(&event, &mut meta);
                handler_count += 1;

                if entry.options.once {
                    handlers_to_remove.push(entry.id);
                }

                if response.should_cancel() {
                    meta.cancel();
                }
                if response.should_stop() {
                    meta.stop_propagation();
                }
                if meta.is_propagation_stopped() || meta.is_immediate_propagation_stopped() {
                    break;
                }
            }
        }

        // Drop the read lock before writing
        drop(handlers);

        // Remove one-time handlers
        if !handlers_to_remove.is_empty() {
            if let Ok(mut pending) = self.pending_removals.write() {
                pending.extend(handlers_to_remove);
            }
        }

        // Clean up pending removals
        self.cleanup_handlers();

        DispatchResult {
            event_id: meta.id,
            cancelled: meta.is_cancelled(),
            propagation_stopped: meta.is_propagation_stopped(),
            handler_count,
            error: None,
        }
    }

    /// Check if there are any handlers for an event type
    pub fn has_handlers<E: CustomEvent>(&self) -> bool {
        let type_id = TypeId::of::<E>();
        self.handlers
            .read()
            .map(|h| h.get(&type_id).is_some_and(|list| !list.is_empty()))
            .unwrap_or(false)
    }

    /// Get the number of handlers for an event type
    pub fn handler_count<E: CustomEvent>(&self) -> usize {
        let type_id = TypeId::of::<E>();
        self.handlers
            .read()
            .map(|h| h.get(&type_id).map_or(0, |list| list.len()))
            .unwrap_or(0)
    }

    fn cleanup_handlers(&self) {
        let pending = {
            let mut pending = match self.pending_removals.write() {
                Ok(p) => p,
                Err(_) => return,
            };
            std::mem::take(&mut *pending)
        };

        if !pending.is_empty() {
            if let Ok(mut handlers) = self.handlers.write() {
                // Remove pending handler IDs from all type lists
                for type_handlers in handlers.values_mut() {
                    type_handlers.retain(|h| !pending.contains(&h.id));
                }
                // Clean up empty type lists
                handlers.retain(|_, list| !list.is_empty());
            }
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        Self {
            handlers: Arc::clone(&self.handlers),
            pending_removals: Arc::clone(&self.pending_removals),
        }
    }
}
