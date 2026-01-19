//! Custom event tests

use revue::event::{
    AppEvent, CustomEvent, CustomEventBus, EventDispatcher, EventMeta, EventPriority,
    EventResponse, HandlerOptions, NavigateEvent,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

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
    use revue::event::EventId;
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
    use revue::event::{DispatchResult, EventId};

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

// =============================================================================
