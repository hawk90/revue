//! Event bus for pub/sub messaging
//!
//! Simple event system for decoupled component communication.

use std::any::Any;
use std::collections::{HashMap, VecDeque};

/// Event identifier
pub type EventId = &'static str;

/// Subscription identifier
pub type Subscription = usize;

/// Type-erased event data
pub type EventData = Box<dyn Any + Send>;

/// Event entry in the queue
#[derive(Debug)]
pub struct Event {
    /// Event type/name
    pub id: EventId,
    /// Event payload (use downcast to get typed data)
    data: Option<EventData>,
}

impl Event {
    /// Get event data as specific type
    pub fn data<T: 'static>(&self) -> Option<&T> {
        self.data.as_ref().and_then(|d| d.downcast_ref::<T>())
    }

    /// Take ownership of event data
    pub fn take_data<T: 'static>(&mut self) -> Option<T> {
        self.data
            .take()
            .and_then(|d| d.downcast::<T>().ok())
            .map(|b| *b)
    }
}

/// Event bus for pub/sub messaging
///
/// # Example
///
/// ```ignore
/// let mut bus = EventBus::new();
///
/// // Emit events
/// bus.emit("user:login", "alice");
/// bus.emit("mount:complete", MountResult { success: true });
///
/// // Poll events in tick handler
/// while let Some(event) = bus.poll() {
///     match event.id {
///         "user:login" => {
///             if let Some(user) = event.data::<&str>() {
///                 println!("User logged in: {}", user);
///             }
///         }
///         "mount:complete" => {
///             if let Some(result) = event.data::<MountResult>() {
///                 handle_mount(result);
///             }
///         }
///         _ => {}
///     }
/// }
/// ```
#[derive(Default)]
pub struct EventBus {
    queue: VecDeque<Event>,
    /// Subscriptions: event_id -> list of subscriber callbacks
    /// Note: For simplicity, we use polling instead of callbacks
    filters: HashMap<EventId, bool>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self::default()
    }

    /// Emit an event with typed data
    pub fn emit<T: Send + 'static>(&mut self, id: EventId, data: T) {
        self.queue.push_back(Event {
            id,
            data: Some(Box::new(data)),
        });
    }

    /// Emit an event without data
    pub fn emit_signal(&mut self, id: EventId) {
        self.queue.push_back(Event { id, data: None });
    }

    /// Poll for the next event
    pub fn poll(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }

    /// Poll for events matching a specific ID
    pub fn poll_id(&mut self, target_id: EventId) -> Option<Event> {
        if let Some(pos) = self.queue.iter().position(|e| e.id == target_id) {
            self.queue.remove(pos)
        } else {
            None
        }
    }

    /// Check if there are pending events
    pub fn has_events(&self) -> bool {
        !self.queue.is_empty()
    }

    /// Get count of pending events
    pub fn event_count(&self) -> usize {
        self.queue.len()
    }

    /// Clear all pending events
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Subscribe to an event type (enables filtering)
    pub fn subscribe(&mut self, id: EventId) -> Subscription {
        self.filters.insert(id, true);
        // Return a simple subscription ID (could be enhanced)
        self.filters.len()
    }

    /// Unsubscribe from an event type
    pub fn unsubscribe(&mut self, id: EventId) {
        self.filters.remove(id);
    }

    /// Check if subscribed to an event
    pub fn is_subscribed(&self, id: EventId) -> bool {
        self.filters.get(id).copied().unwrap_or(false)
    }
}

// Note: A match_event! macro would be nice but macro_rules limitations
// make it complex. Users can use regular match statements instead:
//
// ```ignore
// while let Some(event) = bus.poll() {
//     match event.id {
//         "mount:complete" => {
//             if let Some(result) = event.data::<MountResult>() {
//                 handle_mount(result);
//             }
//         }
//         _ => {}
//     }
// }
// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_and_poll() {
        let mut bus = EventBus::new();

        bus.emit("test", 42i32);
        assert!(bus.has_events());

        let event = bus.poll().unwrap();
        assert_eq!(event.id, "test");
        assert_eq!(event.data::<i32>(), Some(&42));

        assert!(!bus.has_events());
    }

    #[test]
    fn test_emit_signal() {
        let mut bus = EventBus::new();

        bus.emit_signal("ping");
        let event = bus.poll().unwrap();
        assert_eq!(event.id, "ping");
        assert!(event.data::<()>().is_none());
    }

    #[test]
    fn test_multiple_events() {
        let mut bus = EventBus::new();

        bus.emit("a", 1i32);
        bus.emit("b", 2i32);
        bus.emit("c", 3i32);

        assert_eq!(bus.event_count(), 3);

        let e1 = bus.poll().unwrap();
        assert_eq!(e1.id, "a");

        let e2 = bus.poll().unwrap();
        assert_eq!(e2.id, "b");

        let e3 = bus.poll().unwrap();
        assert_eq!(e3.id, "c");
    }

    #[test]
    fn test_poll_by_id() {
        let mut bus = EventBus::new();

        bus.emit("first", 1i32);
        bus.emit("target", 2i32);
        bus.emit("third", 3i32);

        let target = bus.poll_id("target").unwrap();
        assert_eq!(target.data::<i32>(), Some(&2));

        // Should still have 2 events
        assert_eq!(bus.event_count(), 2);
    }

    #[test]
    fn test_typed_data() {
        #[derive(Debug, PartialEq)]
        struct MyEvent {
            value: String,
        }

        let mut bus = EventBus::new();
        bus.emit(
            "custom",
            MyEvent {
                value: "hello".into(),
            },
        );

        let event = bus.poll().unwrap();
        let data = event.data::<MyEvent>().unwrap();
        assert_eq!(data.value, "hello");
    }
}
