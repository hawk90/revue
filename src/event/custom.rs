//! Custom Event Dispatching System
//!
//! Provides a type-safe, extensible event system for user-defined events.
//!
//! # Features
//!
//! - Custom event type registration
//! - Type-safe event dispatching
//! - Event bubbling/capturing support
//! - Event metadata (timestamp, source)
//! - Event cancellation
//! - Priority levels
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::event::custom::*;
//!
//! // Define a custom event
//! struct ChatMessage {
//!     user: String,
//!     text: String,
//! }
//!
//! impl CustomEvent for ChatMessage {
//!     fn event_type() -> &'static str { "chat_message" }
//! }
//!
//! // Create dispatcher and register handler
//! let mut dispatcher = EventDispatcher::new();
//!
//! dispatcher.on::<ChatMessage>(|event, ctx| {
//!     println!("{}: {}", event.user, event.text);
//!     EventResponse::Handled
//! });
//!
//! // Dispatch event
//! dispatcher.dispatch(ChatMessage {
//!     user: "Alice".to_string(),
//!     text: "Hello!".to_string(),
//! });
//! ```

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Instant, SystemTime};

// =============================================================================
// Event Trait
// =============================================================================

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

// =============================================================================
// Event Envelope
// =============================================================================

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

// =============================================================================
// Event Response
// =============================================================================

/// Response from an event handler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResponse {
    /// Event was handled, continue propagation
    Handled,
    /// Event was not handled, continue propagation
    Ignored,
    /// Event was handled, stop propagation
    StopPropagation,
    /// Event was handled, cancel default action
    Cancel,
    /// Event was handled, stop propagation and cancel
    StopAndCancel,
}

impl EventResponse {
    /// Check if the event was handled
    pub fn is_handled(&self) -> bool {
        !matches!(self, Self::Ignored)
    }

    /// Check if propagation should stop
    pub fn should_stop(&self) -> bool {
        matches!(self, Self::StopPropagation | Self::StopAndCancel)
    }

    /// Check if event should be cancelled
    pub fn should_cancel(&self) -> bool {
        matches!(self, Self::Cancel | Self::StopAndCancel)
    }
}

// =============================================================================
// Handler Registration
// =============================================================================

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
type BoxedHandler = Box<dyn Fn(&dyn Any, &mut EventMeta) -> EventResponse + Send + Sync>;

struct HandlerEntry {
    id: CustomHandlerId,
    handler: BoxedHandler,
    options: HandlerOptions,
    type_id: TypeId,
}

// =============================================================================
// Event Dispatcher
// =============================================================================

/// Thread-safe event dispatcher
pub struct EventDispatcher {
    handlers: Arc<RwLock<Vec<HandlerEntry>>>,
    pending_removals: Arc<RwLock<Vec<CustomHandlerId>>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
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
            handlers.push(entry);
            // Sort by priority (higher priority first)
            handlers.sort_by(|a, b| b.options.priority.cmp(&a.options.priority));
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
            handlers.retain(|h| h.id != handler_id);
        }
    }

    /// Remove all handlers for a specific event type
    pub fn off_all<E: CustomEvent>(&mut self) {
        let type_id = TypeId::of::<E>();
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.retain(|h| h.type_id != type_id);
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

        // Get read lock and process handlers
        let handlers = match self.handlers.read() {
            Ok(h) => h,
            Err(_) => return DispatchResult::error("Failed to acquire lock"),
        };

        // Process capture phase handlers first
        for entry in handlers.iter().filter(|h| h.type_id == type_id) {
            if entry.options.capture {
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
        }

        // Process target/bubble phase handlers
        if !meta.is_propagation_stopped() {
            for entry in handlers.iter().filter(|h| h.type_id == type_id) {
                if !entry.options.capture {
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
            .map(|h| h.iter().any(|entry| entry.type_id == type_id))
            .unwrap_or(false)
    }

    /// Get the number of handlers for an event type
    pub fn handler_count<E: CustomEvent>(&self) -> usize {
        let type_id = TypeId::of::<E>();
        self.handlers
            .read()
            .map(|h| h.iter().filter(|entry| entry.type_id == type_id).count())
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
                handlers.retain(|h| !pending.contains(&h.id));
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

// =============================================================================
// Dispatch Result
// =============================================================================

/// Result of dispatching an event
#[derive(Debug)]
pub struct DispatchResult {
    /// The event ID
    pub event_id: EventId,
    /// Whether the event was cancelled
    pub cancelled: bool,
    /// Whether propagation was stopped
    pub propagation_stopped: bool,
    /// Number of handlers that processed the event
    pub handler_count: usize,
    /// Error message if dispatch failed
    pub error: Option<String>,
}

impl DispatchResult {
    fn error(msg: &str) -> Self {
        Self {
            event_id: EventId::new(),
            cancelled: false,
            propagation_stopped: false,
            handler_count: 0,
            error: Some(msg.to_string()),
        }
    }

    /// Check if dispatch was successful
    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    /// Check if the event was handled by at least one handler
    pub fn was_handled(&self) -> bool {
        self.handler_count > 0
    }
}

// =============================================================================
// Event Bus (Global Dispatcher)
// =============================================================================

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

// =============================================================================
// Common Event Types
// =============================================================================

/// Application lifecycle event
#[derive(Debug, Clone)]
pub struct AppEvent {
    /// Event name
    pub name: String,
    /// Event data
    pub data: HashMap<String, String>,
}

impl AppEvent {
    /// Create a new app event
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: HashMap::new(),
        }
    }

    /// Add data
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }
}

impl CustomEvent for AppEvent {
    fn event_type() -> &'static str {
        "app"
    }
}

/// State change event
#[derive(Debug, Clone)]
pub struct StateChangeEvent {
    /// State key
    pub key: String,
    /// Old value (as string)
    pub old_value: Option<String>,
    /// New value (as string)
    pub new_value: Option<String>,
}

impl StateChangeEvent {
    /// Create a new state change event
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            old_value: None,
            new_value: None,
        }
    }

    /// Set old value
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.old_value = Some(value.into());
        self
    }

    /// Set new value
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.new_value = Some(value.into());
        self
    }
}

impl CustomEvent for StateChangeEvent {
    fn event_type() -> &'static str {
        "state_change"
    }
}

/// Navigation event
#[derive(Debug, Clone)]
pub struct NavigateEvent {
    /// Target path/route
    pub path: String,
    /// Navigation parameters
    pub params: HashMap<String, String>,
    /// Replace history instead of push
    pub replace: bool,
}

impl NavigateEvent {
    /// Create a new navigation event
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            params: HashMap::new(),
            replace: false,
        }
    }

    /// Add parameter
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Set replace mode
    pub fn replace(mut self, replace: bool) -> Self {
        self.replace = replace;
        self
    }
}

impl CustomEvent for NavigateEvent {
    fn event_type() -> &'static str {
        "navigate"
    }
}

/// Error event
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error source
    pub source: Option<String>,
    /// Is recoverable
    pub recoverable: bool,
}

impl ErrorEvent {
    /// Create a new error event
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            source: None,
            recoverable: true,
        }
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set recoverable flag
    pub fn recoverable(mut self, recoverable: bool) -> Self {
        self.recoverable = recoverable;
        self
    }
}

impl CustomEvent for ErrorEvent {
    fn event_type() -> &'static str {
        "error"
    }

    fn cancellable() -> bool {
        false
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    // Test event
    struct TestEvent {
        value: i32,
    }

    impl CustomEvent for TestEvent {
        fn event_type() -> &'static str {
            "test"
        }
    }

    #[test]
    fn test_event_id_unique() {
        let id1 = EventId::new();
        let id2 = EventId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_event_meta() {
        let meta = EventMeta::new("test")
            .with_source("component")
            .with_priority(EventPriority::High);

        assert_eq!(meta.event_type, "test");
        assert_eq!(meta.source, Some("component".to_string()));
        assert_eq!(meta.priority, EventPriority::High);
        assert!(!meta.is_cancelled());
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
    fn test_dispatcher_on() {
        let mut dispatcher = EventDispatcher::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let count_clone = Arc::clone(&call_count);

        dispatcher.on::<TestEvent>(move |event, _meta| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            assert_eq!(event.value, 42);
            EventResponse::Handled
        });

        dispatcher.dispatch(TestEvent { value: 42 });
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_dispatcher_multiple_handlers() {
        let mut dispatcher = EventDispatcher::new();
        let call_count = Arc::new(AtomicUsize::new(0));

        let count1 = Arc::clone(&call_count);
        dispatcher.on::<TestEvent>(move |_, _| {
            count1.fetch_add(1, Ordering::SeqCst);
            EventResponse::Handled
        });

        let count2 = Arc::clone(&call_count);
        dispatcher.on::<TestEvent>(move |_, _| {
            count2.fetch_add(1, Ordering::SeqCst);
            EventResponse::Handled
        });

        dispatcher.dispatch(TestEvent { value: 1 });
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_dispatcher_off() {
        let mut dispatcher = EventDispatcher::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let count_clone = Arc::clone(&call_count);

        let id = dispatcher.on::<TestEvent>(move |_, _| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            EventResponse::Handled
        });

        dispatcher.dispatch(TestEvent { value: 1 });
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        dispatcher.off(id);
        dispatcher.dispatch(TestEvent { value: 1 });
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // Still 1
    }

    #[test]
    fn test_dispatcher_once() {
        let mut dispatcher = EventDispatcher::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let count_clone = Arc::clone(&call_count);

        dispatcher.once::<TestEvent>(move |_, _| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            EventResponse::Handled
        });

        dispatcher.dispatch(TestEvent { value: 1 });
        dispatcher.dispatch(TestEvent { value: 2 });

        assert_eq!(call_count.load(Ordering::SeqCst), 1); // Only called once
    }

    #[test]
    fn test_dispatcher_stop_propagation() {
        let mut dispatcher = EventDispatcher::new();
        let call_count = Arc::new(AtomicUsize::new(0));

        let count1 = Arc::clone(&call_count);
        dispatcher.on_with_options::<TestEvent>(
            move |_, _| {
                count1.fetch_add(1, Ordering::SeqCst);
                EventResponse::StopPropagation
            },
            HandlerOptions::new().priority(EventPriority::High),
        );

        let count2 = Arc::clone(&call_count);
        dispatcher.on::<TestEvent>(move |_, _| {
            count2.fetch_add(1, Ordering::SeqCst);
            EventResponse::Handled
        });

        let result = dispatcher.dispatch(TestEvent { value: 1 });
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // Second handler not called
        assert!(result.propagation_stopped);
    }

    #[test]
    fn test_dispatcher_cancel() {
        let mut dispatcher = EventDispatcher::new();

        dispatcher.on::<TestEvent>(move |_, _| EventResponse::Cancel);

        let result = dispatcher.dispatch(TestEvent { value: 1 });
        assert!(result.cancelled);
    }

    #[test]
    fn test_dispatcher_has_handlers() {
        let mut dispatcher = EventDispatcher::new();
        assert!(!dispatcher.has_handlers::<TestEvent>());

        dispatcher.on::<TestEvent>(|_, _| EventResponse::Handled);
        assert!(dispatcher.has_handlers::<TestEvent>());
    }

    #[test]
    fn test_dispatcher_handler_count() {
        let mut dispatcher = EventDispatcher::new();
        assert_eq!(dispatcher.handler_count::<TestEvent>(), 0);

        dispatcher.on::<TestEvent>(|_, _| EventResponse::Handled);
        dispatcher.on::<TestEvent>(|_, _| EventResponse::Handled);
        assert_eq!(dispatcher.handler_count::<TestEvent>(), 2);
    }

    #[test]
    fn test_event_bus() {
        let mut bus = CustomEventBus::new();
        let received = Arc::new(AtomicUsize::new(0));
        let received_clone = Arc::clone(&received);

        bus.on::<TestEvent>(move |event, _| {
            received_clone.store(event.value as usize, Ordering::SeqCst);
            EventResponse::Handled
        });

        bus.emit(TestEvent { value: 99 });
        assert_eq!(received.load(Ordering::SeqCst), 99);
    }

    #[test]
    fn test_event_bus_history() {
        let mut bus = CustomEventBus::new();
        bus.on::<TestEvent>(|_, _| EventResponse::Handled);

        bus.emit(TestEvent { value: 1 });
        bus.emit(TestEvent { value: 2 });

        let history = bus.history();
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_app_event() {
        let event = AppEvent::new("startup")
            .with_data("version", "1.0")
            .with_data("mode", "debug");

        assert_eq!(event.name, "startup");
        assert_eq!(event.data.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_navigate_event() {
        let event = NavigateEvent::new("/users")
            .with_param("id", "123")
            .replace(true);

        assert_eq!(event.path, "/users");
        assert_eq!(event.params.get("id"), Some(&"123".to_string()));
        assert!(event.replace);
    }

    #[test]
    fn test_event_response() {
        assert!(EventResponse::Handled.is_handled());
        assert!(!EventResponse::Ignored.is_handled());

        assert!(EventResponse::StopPropagation.should_stop());
        assert!(!EventResponse::Handled.should_stop());

        assert!(EventResponse::Cancel.should_cancel());
        assert!(EventResponse::StopAndCancel.should_stop());
        assert!(EventResponse::StopAndCancel.should_cancel());
    }

    #[test]
    fn test_dispatch_result() {
        let result = DispatchResult {
            event_id: EventId::new(),
            cancelled: false,
            propagation_stopped: false,
            handler_count: 2,
            error: None,
        };

        assert!(result.is_ok());
        assert!(result.was_handled());
    }

    #[test]
    fn test_priority_ordering() {
        let mut dispatcher = EventDispatcher::new();
        let order = Arc::new(RwLock::new(Vec::new()));

        let order1 = Arc::clone(&order);
        dispatcher.on_with_options::<TestEvent>(
            move |_, _| {
                order1.write().unwrap().push(1);
                EventResponse::Handled
            },
            HandlerOptions::new().priority(EventPriority::Low),
        );

        let order2 = Arc::clone(&order);
        dispatcher.on_with_options::<TestEvent>(
            move |_, _| {
                order2.write().unwrap().push(2);
                EventResponse::Handled
            },
            HandlerOptions::new().priority(EventPriority::High),
        );

        let order3 = Arc::clone(&order);
        dispatcher.on_with_options::<TestEvent>(
            move |_, _| {
                order3.write().unwrap().push(3);
                EventResponse::Handled
            },
            HandlerOptions::new().priority(EventPriority::Normal),
        );

        dispatcher.dispatch(TestEvent { value: 1 });

        let result = order.read().unwrap().clone();
        // High (2), Normal (3), Low (1)
        assert_eq!(result, vec![2, 3, 1]);
    }
}
