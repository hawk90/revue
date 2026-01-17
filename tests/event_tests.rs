//! Integration tests for the event module

use revue::event::{
    AppEvent, Candidate, CompositionEvent, CompositionState, CustomEvent, CustomEventBus,
    Direction, DispatchPhase, DragContext, DragData, DragGesture, DragState, DropTarget,
    EventContext, EventDispatcher, EventHandler, EventMeta, EventPhase, EventPriority,
    EventResponse, FocusManager, FocusTrap, FocusTrapConfig, Gesture, GestureConfig,
    GestureRecognizer, GestureState, HandlerOptions, ImeConfig, ImeState, Key, KeyBinding, KeyMap,
    LongPressGesture, MouseButton, MouseEvent, MouseEventKind, NavigateEvent, PinchDirection,
    PinchGesture, PreeditString, SwipeDirection, SwipeGesture, TapGesture,
};
use revue::event::{Event, EventReader, KeyEvent};
use revue::layout::Rect;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

// =============================================================================
// Event (mod.rs) Tests
// =============================================================================

#[test]
fn test_key_event_new() {
    let event = KeyEvent::new(Key::Enter);
    assert_eq!(event.key, Key::Enter);
    assert!(!event.ctrl);
    assert!(!event.alt);
    assert!(!event.shift);
}

#[test]
fn test_key_event_ctrl() {
    let event = KeyEvent::ctrl(Key::Char('c'));
    assert_eq!(event.key, Key::Char('c'));
    assert!(event.ctrl);
    assert!(event.is_ctrl_c());
}

#[test]
fn test_key_event_checks() {
    assert!(KeyEvent::new(Key::Escape).is_escape());
    assert!(KeyEvent::new(Key::Enter).is_enter());
    assert!(KeyEvent::new(Key::Tab).is_tab());

    let shift_tab = KeyEvent {
        key: Key::Tab,
        ctrl: false,
        alt: false,
        shift: true,
    };
    assert!(shift_tab.is_shift_tab());
    assert!(!shift_tab.is_tab());
}

#[test]
fn test_key_event_to_binding() {
    let event = KeyEvent::ctrl(Key::Char('s'));
    let binding = event.to_binding();

    assert_eq!(binding.key, Key::Char('s'));
    assert!(binding.ctrl);
}

#[test]
fn test_event_focus_gained() {
    let event = Event::FocusGained;
    assert!(matches!(event, Event::FocusGained));
}

#[test]
fn test_event_focus_lost() {
    let event = Event::FocusLost;
    assert!(matches!(event, Event::FocusLost));
}

#[test]
fn test_event_paste() {
    let event = Event::Paste("hello world".to_string());
    if let Event::Paste(text) = event {
        assert_eq!(text, "hello world");
    } else {
        panic!("Expected Paste event");
    }
}

#[test]
fn test_event_variants_equality() {
    assert_eq!(Event::FocusGained, Event::FocusGained);
    assert_eq!(Event::FocusLost, Event::FocusLost);
    assert_eq!(
        Event::Paste("test".to_string()),
        Event::Paste("test".to_string())
    );
    assert_ne!(Event::FocusGained, Event::FocusLost);
    assert_ne!(Event::Paste("a".to_string()), Event::Paste("b".to_string()));
}

// =============================================================================
// Custom Event Tests
// =============================================================================

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
// Drag Tests
// =============================================================================

#[test]
fn test_drag_data_text() {
    let data = DragData::text("Hello");
    assert_eq!(data.type_id, "text");
    assert_eq!(data.as_text(), Some("Hello"));
    assert_eq!(data.display_label(), "Hello");
}

#[test]
fn test_drag_data_list_item() {
    let data = DragData::list_item(5, "Item 5");
    assert_eq!(data.type_id, "list_item");
    assert_eq!(data.as_list_index(), Some(5));
    assert_eq!(data.display_label(), "Item 5");
}

#[test]
fn test_drag_data_custom() {
    #[derive(Debug)]
    struct MyData {
        value: i32,
    }

    let data = DragData::new("my_data", MyData { value: 42 });
    assert_eq!(data.type_id, "my_data");
    assert_eq!(data.get::<MyData>().map(|d| d.value), Some(42));
}

#[test]
fn test_drag_state() {
    assert!(!DragState::Idle.is_active());
    assert!(DragState::Dragging.is_active());
    assert!(DragState::OverTarget.is_active());
    assert!(!DragState::Dropped.is_active());

    assert!(!DragState::Dragging.is_over_target());
    assert!(DragState::OverTarget.is_over_target());
}

#[test]
fn test_drag_context_basic() {
    let mut ctx = DragContext::new();
    assert!(!ctx.is_dragging());

    ctx.start_drag(DragData::text("Test"), 10, 10);
    assert_eq!(ctx.state(), DragState::Pending);

    // Move beyond threshold
    ctx.update_position(20, 10);
    assert_eq!(ctx.state(), DragState::Dragging);
    assert!(ctx.is_dragging());
}

#[test]
fn test_drag_context_threshold() {
    let mut ctx = DragContext::new().threshold(5);

    ctx.start_drag(DragData::text("Test"), 10, 10);

    // Move less than threshold - still pending
    ctx.update_position(12, 10);
    assert_eq!(ctx.state(), DragState::Pending);

    // Move beyond threshold
    ctx.update_position(16, 10);
    assert_eq!(ctx.state(), DragState::Dragging);
}

#[test]
fn test_drop_target() {
    let target = DropTarget::new(1, Rect::new(10, 10, 20, 10)).accepts(&["text", "file"]);

    assert!(target.contains(15, 15));
    assert!(!target.contains(5, 5));

    let text_data = DragData::text("Hello");
    assert!(target.can_accept(&text_data));

    let other_data = DragData::new("other", 42);
    assert!(!target.can_accept(&other_data));
}

#[test]
fn test_drop_target_accepts_all() {
    let target = DropTarget::new(1, Rect::new(0, 0, 10, 10)).accepts_all();

    let text_data = DragData::text("Hello");
    let other_data = DragData::new("whatever", 42);

    assert!(target.can_accept(&text_data));
    assert!(target.can_accept(&other_data));
}

#[test]
fn test_drag_context_with_targets() {
    let mut ctx = DragContext::new().threshold(0);

    // Register a target
    ctx.register_target(DropTarget::new(1, Rect::new(50, 50, 20, 10)).accepts(&["text"]));

    // Start drag
    ctx.start_drag(DragData::text("Test"), 10, 10);
    ctx.update_position(11, 10); // Trigger dragging state

    // Move to target
    ctx.update_position(55, 55);
    assert_eq!(ctx.state(), DragState::OverTarget);
    assert_eq!(ctx.hovered_target(), Some(1));

    // Move away
    ctx.update_position(10, 10);
    assert_eq!(ctx.state(), DragState::Dragging);
    assert_eq!(ctx.hovered_target(), None);
}

#[test]
fn test_drag_context_end_drag() {
    let mut ctx = DragContext::new().threshold(0);

    ctx.register_target(DropTarget::new(1, Rect::new(50, 50, 20, 10)).accepts_all());

    ctx.start_drag(DragData::text("Hello"), 10, 10);
    ctx.update_position(55, 55); // Over target

    let result = ctx.end_drag();
    assert!(result.is_some());

    let (data, target) = result.unwrap();
    assert_eq!(data.as_text(), Some("Hello"));
    assert_eq!(target, Some(1));

    assert_eq!(ctx.state(), DragState::Dropped);
}

#[test]
fn test_drag_context_cancel() {
    let mut ctx = DragContext::new().threshold(0);

    ctx.start_drag(DragData::text("Test"), 10, 10);
    ctx.update_position(20, 20);
    assert!(ctx.is_dragging());

    ctx.cancel();
    assert_eq!(ctx.state(), DragState::Cancelled);
    assert!(!ctx.is_dragging());
    assert!(ctx.data().is_none());
}

#[test]
fn test_drag_offset() {
    let mut ctx = DragContext::new();

    ctx.start_drag(DragData::text("Test"), 10, 20);
    ctx.update_position(25, 15);

    assert_eq!(ctx.offset(), (15, -5));
}

// =============================================================================
// Focus Tests
// =============================================================================

#[test]
fn test_focus_manager_new() {
    let fm = FocusManager::new();
    assert!(fm.current().is_none());
}

#[test]
fn test_focus_register() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    // No focus yet
    assert!(fm.current().is_none());
}

#[test]
fn test_focus_next() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.next();
    assert_eq!(fm.current(), Some(1));

    fm.next();
    assert_eq!(fm.current(), Some(2));

    fm.next();
    assert_eq!(fm.current(), Some(3));

    // Wrap around
    fm.next();
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_focus_prev() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.prev();
    assert_eq!(fm.current(), Some(3));

    fm.prev();
    assert_eq!(fm.current(), Some(2));

    fm.prev();
    assert_eq!(fm.current(), Some(1));

    // Wrap around
    fm.prev();
    assert_eq!(fm.current(), Some(3));
}

#[test]
fn test_focus_specific_widget() {
    let mut fm = FocusManager::new();
    fm.register(10);
    fm.register(20);
    fm.register(30);

    fm.focus(20);
    assert_eq!(fm.current(), Some(20));

    fm.focus(30);
    assert_eq!(fm.current(), Some(30));
}

#[test]
fn test_is_focused() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);

    fm.focus(1);
    assert!(fm.is_focused(1));
    assert!(!fm.is_focused(2));
}

#[test]
fn test_blur() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.next();
    assert!(fm.current().is_some());

    fm.blur();
    assert!(fm.current().is_none());
}

#[test]
fn test_unregister() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(2);
    fm.unregister(1);

    // Focus should adjust
    assert_eq!(fm.current(), Some(2));
}

// 2D Navigation Tests

#[test]
fn test_2d_navigation_right() {
    let mut fm = FocusManager::new();
    // Layout:  [1] [2] [3]
    //           x=0  10  20
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, 10, 0);
    fm.register_with_position(3, 20, 0);

    fm.focus(1);
    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(2));

    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(3));

    // No more to the right
    assert!(!fm.move_focus(Direction::Right));
}

#[test]
fn test_2d_navigation_down() {
    let mut fm = FocusManager::new();
    // Layout:  [1]
    //          [2]
    //          [3]
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, 0, 10);
    fm.register_with_position(3, 0, 20);

    fm.focus(1);
    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(2));

    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(3));
}

#[test]
fn test_2d_navigation_grid() {
    let mut fm = FocusManager::new();
    // Layout:  [1] [2]
    //          [3] [4]
    fm.register_with_position(1, 0, 0);
    fm.register_with_position(2, 10, 0);
    fm.register_with_position(3, 0, 10);
    fm.register_with_position(4, 10, 10);

    fm.focus(1);

    // Right to 2
    assert!(fm.move_focus(Direction::Right));
    assert_eq!(fm.current(), Some(2));

    // Down to 4
    assert!(fm.move_focus(Direction::Down));
    assert_eq!(fm.current(), Some(4));

    // Left to 3
    assert!(fm.move_focus(Direction::Left));
    assert_eq!(fm.current(), Some(3));

    // Up to 1
    assert!(fm.move_focus(Direction::Up));
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_register_with_bounds() {
    let mut fm = FocusManager::new();
    let bounds = Rect::new(10, 5, 20, 10);
    fm.register_with_bounds(1, bounds);

    fm.focus(1);
    assert_eq!(fm.current(), Some(1));
}

// Focus Trapping Tests

#[test]
fn test_focus_trap() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3); // Modal button 1
    fm.register(4); // Modal button 2

    // Focus on widget 1
    fm.focus(1);
    assert_eq!(fm.current(), Some(1));

    // Trap focus to modal (widgets 3 and 4)
    fm.trap_focus(100); // Modal container ID
    fm.add_to_trap(3);
    fm.add_to_trap(4);

    assert!(fm.is_trapped());

    // Tab should now only cycle between 3 and 4
    fm.focus(3);
    fm.next();
    assert_eq!(fm.current(), Some(4));

    fm.next();
    assert_eq!(fm.current(), Some(3)); // Wraps within trap

    // Release trap
    fm.release_trap();
    assert!(!fm.is_trapped());

    // Now Tab cycles all widgets again
    fm.focus(1);
    fm.next();
    assert_eq!(fm.current(), Some(2));
}

#[test]
fn test_trap_container() {
    let mut fm = FocusManager::new();
    fm.trap_focus(42);
    assert_eq!(fm.trap_container(), Some(42));

    fm.release_trap();
    assert_eq!(fm.trap_container(), None);
}

// Focus Restoration Tests

#[test]
fn test_focus_restoration() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);
    fm.register(4);

    // Focus on widget 2
    fm.focus(2);
    assert_eq!(fm.current(), Some(2));

    // Trap focus
    fm.trap_focus(100);
    fm.add_to_trap(3);
    fm.add_to_trap(4);
    fm.focus(3);

    // Saved focus should be 2
    assert_eq!(fm.saved_focus(), Some(2));

    // Release and restore
    fm.release_trap_and_restore();
    assert_eq!(fm.current(), Some(2));
}

#[test]
fn test_trap_with_initial_focus() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(1);
    fm.trap_focus_with_initial(100, 3);
    assert_eq!(fm.current(), Some(3));
}

// Nested Focus Trap Tests

#[test]
fn test_push_pop_trap() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);
    fm.register(4);
    fm.register(5);

    // Start on widget 1
    fm.focus(1);
    assert_eq!(fm.current(), Some(1));

    // Push first trap (modal 1)
    fm.push_trap(100, &[2, 3]);
    assert_eq!(fm.current(), Some(2));
    assert_eq!(fm.trap_depth(), 1);

    // Push second trap (modal 2)
    fm.push_trap(200, &[4, 5]);
    assert_eq!(fm.current(), Some(4));
    assert_eq!(fm.trap_depth(), 2);

    // Pop second trap - should restore to modal 1
    fm.pop_trap();
    assert_eq!(fm.current(), Some(2));
    assert_eq!(fm.trap_depth(), 1);

    // Pop first trap - should restore to original
    fm.pop_trap();
    assert_eq!(fm.current(), Some(1));
    assert_eq!(fm.trap_depth(), 0);
}

#[test]
fn test_trap_depth() {
    let mut fm = FocusManager::new();
    fm.register(1);

    assert_eq!(fm.trap_depth(), 0);

    fm.push_trap(100, &[1]);
    assert_eq!(fm.trap_depth(), 1);

    fm.push_trap(200, &[1]);
    assert_eq!(fm.trap_depth(), 2);

    fm.pop_trap();
    assert_eq!(fm.trap_depth(), 1);

    fm.pop_trap();
    assert_eq!(fm.trap_depth(), 0);
}

// FocusTrap Helper Tests

#[test]
fn test_focus_trap_helper() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(3);

    fm.focus(1);

    let mut trap = FocusTrap::new(100).with_children(&[2, 3]).initial_focus(3);

    assert!(!trap.is_active());

    trap.activate(&mut fm);
    assert!(trap.is_active());
    assert_eq!(fm.current(), Some(3));

    trap.deactivate(&mut fm);
    assert!(!trap.is_active());
    assert_eq!(fm.current(), Some(1)); // Restored
}

#[test]
fn test_focus_trap_add_child() {
    let trap = FocusTrap::new(100).add_child(1).add_child(2).add_child(2); // Duplicate should be ignored

    assert_eq!(trap.container_id(), 100);
}

#[test]
fn test_focus_trap_config() {
    let config = FocusTrapConfig::default();
    assert!(config.restore_on_release);
    assert!(config.loop_focus);
    assert!(config.initial_focus.is_none());
}

// =============================================================================
// Gesture Tests
// =============================================================================

#[test]
fn test_swipe_direction() {
    assert!(SwipeDirection::Up.is_vertical());
    assert!(SwipeDirection::Down.is_vertical());
    assert!(SwipeDirection::Left.is_horizontal());
    assert!(SwipeDirection::Right.is_horizontal());
}

#[test]
fn test_gesture_config_default() {
    let config = GestureConfig::default();
    assert_eq!(config.swipe_threshold, 3);
    assert_eq!(config.long_press_duration, Duration::from_millis(500));
}

#[test]
fn test_recognizer_creation() {
    let recognizer = GestureRecognizer::new();
    assert!(recognizer.is_enabled());
}

#[test]
fn test_recognizer_enable_disable() {
    let mut recognizer = GestureRecognizer::new();
    assert!(recognizer.is_enabled());

    recognizer.set_enabled(false);
    assert!(!recognizer.is_enabled());

    recognizer.set_enabled(true);
    assert!(recognizer.is_enabled());
}

#[test]
fn test_tap_gesture() {
    let mut recognizer = GestureRecognizer::new();
    let tapped = Arc::new(AtomicBool::new(false));
    let tapped_clone = Arc::clone(&tapped);

    recognizer.on_tap(move |tap| {
        assert_eq!(tap.x, 10);
        assert_eq!(tap.y, 5);
        tapped_clone.store(true, Ordering::SeqCst);
    });

    // Press
    let down_event = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Release at same position (tap)
    let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up_event);

    assert!(tapped.load(Ordering::SeqCst));
}

#[test]
fn test_swipe_gesture() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_swipe_threshold(2);

    let swiped = Arc::new(AtomicBool::new(false));
    let swiped_clone = Arc::clone(&swiped);

    recognizer.on_swipe(move |swipe| {
        assert_eq!(swipe.direction, SwipeDirection::Right);
        swiped_clone.store(true, Ordering::SeqCst);
    });

    // Press
    let down_event = MouseEvent::new(0, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Release far to the right (swipe)
    let up_event = MouseEvent::new(20, 5, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up_event);

    assert!(swiped.load(Ordering::SeqCst));
}

#[test]
fn test_drag_gesture() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_drag_threshold(1);

    let drag_count = Arc::new(AtomicUsize::new(0));
    let count_clone = Arc::clone(&drag_count);

    recognizer.on_drag(move |_| {
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    // Press
    let down_event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Drag
    let drag_event = MouseEvent::new(10, 10, MouseEventKind::Drag(MouseButton::Left));
    recognizer.handle_mouse_event(&drag_event);

    // Release
    let up_event = MouseEvent::new(15, 15, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up_event);

    // Should have at least 2 drag events (start and end)
    assert!(drag_count.load(Ordering::SeqCst) >= 2);
}

#[test]
fn test_pinch_gesture() {
    let mut recognizer = GestureRecognizer::new();

    let pinched = Arc::new(AtomicBool::new(false));
    let pinched_clone = Arc::clone(&pinched);

    recognizer.on_pinch(move |pinch| {
        assert_eq!(pinch.direction, PinchDirection::Out);
        pinched_clone.store(true, Ordering::SeqCst);
    });

    // Ctrl+scroll up = pinch out
    let scroll_event = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollUp,
        ctrl: true,
        alt: false,
        shift: false,
    };
    recognizer.handle_mouse_event(&scroll_event);

    assert!(pinched.load(Ordering::SeqCst));
}

#[test]
fn test_long_press() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_long_press_duration(Duration::from_millis(10));

    let long_pressed = Arc::new(AtomicBool::new(false));
    let lp_clone = Arc::clone(&long_pressed);

    recognizer.on_long_press(move |_| {
        lp_clone.store(true, Ordering::SeqCst);
    });

    // Press
    let down_event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Wait for long press
    std::thread::sleep(Duration::from_millis(20));

    // Check for long press
    recognizer.check_long_press();

    assert!(long_pressed.load(Ordering::SeqCst));
}

#[test]
fn test_gesture_data_methods() {
    let swipe = SwipeGesture {
        direction: SwipeDirection::Right,
        start_x: 0,
        start_y: 10,
        end_x: 20,
        end_y: 15,
        distance: 21.2,
        velocity: 100.0,
        duration: Duration::from_millis(200),
        button: MouseButton::Left,
    };

    assert_eq!(swipe.delta_x(), 20);
    assert_eq!(swipe.delta_y(), 5);

    let drag = DragGesture {
        start_x: 0,
        start_y: 0,
        current_x: 10,
        current_y: 10,
        prev_x: 8,
        prev_y: 8,
        total_distance: 14.14,
        duration: Duration::from_millis(100),
        button: MouseButton::Left,
        state: GestureState::Active,
    };

    assert_eq!(drag.delta_x(), 2);
    assert_eq!(drag.delta_y(), 2);
    assert_eq!(drag.total_delta_x(), 10);
    assert_eq!(drag.total_delta_y(), 10);
}

#[test]
fn test_recognizer_disabled() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_enabled(false);

    let tapped = Arc::new(AtomicBool::new(false));
    let tapped_clone = Arc::clone(&tapped);

    recognizer.on_tap(move |_| {
        tapped_clone.store(true, Ordering::SeqCst);
    });

    let down_event = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up_event);

    // Should not have been called because recognizer is disabled
    assert!(!tapped.load(Ordering::SeqCst));
}

#[test]
fn test_clear_handlers() {
    let mut recognizer = GestureRecognizer::new();

    recognizer.on_tap(|_| {});
    recognizer.on_swipe(|_| {});
    recognizer.on_drag(|_| {});

    recognizer.clear_handlers();

    // After clearing, handlers should not fire
    // (This is a basic check - actual verification would need more setup)
}

#[test]
fn test_gesture_state_default() {
    let state = GestureState::default();
    assert_eq!(state, GestureState::Idle);
}

#[test]
fn test_gesture_state_clone() {
    let state = GestureState::Active;
    let cloned = state;
    assert_eq!(state, cloned);
}

#[test]
fn test_pinch_direction_clone() {
    let dir = PinchDirection::In;
    let cloned = dir;
    assert_eq!(dir, cloned);
}

#[test]
fn test_swipe_direction_clone() {
    let dir = SwipeDirection::Up;
    let cloned = dir;
    assert_eq!(dir, cloned);
}

#[test]
fn test_gesture_config_fields() {
    let config = GestureConfig {
        swipe_threshold: 5,
        swipe_max_duration: Duration::from_millis(400),
        swipe_min_velocity: 15.0,
        long_press_duration: Duration::from_millis(800),
        drag_threshold: 3,
        pinch_scale_per_scroll: 0.2,
        double_tap_interval: Duration::from_millis(400),
        double_tap_distance: 3,
    };

    assert_eq!(config.swipe_threshold, 5);
    assert_eq!(config.long_press_duration, Duration::from_millis(800));
}

#[test]
fn test_recognizer_with_config() {
    let config = GestureConfig {
        swipe_threshold: 10,
        ..GestureConfig::default()
    };
    let recognizer = GestureRecognizer::with_config(config);
    assert_eq!(recognizer.config().swipe_threshold, 10);
}

#[test]
fn test_recognizer_set_config() {
    let mut recognizer = GestureRecognizer::new();
    let config = GestureConfig {
        swipe_threshold: 10,
        ..GestureConfig::default()
    };
    recognizer.set_config(config);
    assert_eq!(recognizer.config().swipe_threshold, 10);
}

#[test]
fn test_recognizer_set_long_press_duration() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_long_press_duration(Duration::from_millis(1000));
    assert_eq!(
        recognizer.config().long_press_duration,
        Duration::from_millis(1000)
    );
}

#[test]
fn test_recognizer_set_swipe_threshold() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_swipe_threshold(10);
    assert_eq!(recognizer.config().swipe_threshold, 10);
}

#[test]
fn test_recognizer_set_drag_threshold() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_drag_threshold(5);
    assert_eq!(recognizer.config().drag_threshold, 5);
}

#[test]
fn test_recognizer_reset() {
    let mut recognizer = GestureRecognizer::new();

    // Press a button
    let down_event = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Reset should clear state
    recognizer.reset();

    // After reset, the button should be considered released
    let up_event = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Left));
    let result = recognizer.handle_mouse_event(&up_event);
    assert!(result.is_none()); // Should be None because button wasn't tracked
}

#[test]
fn test_recognizer_cancel_gesture() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_drag_threshold(1);

    // Start drag
    let down_event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    let drag_event = MouseEvent::new(20, 20, MouseEventKind::Drag(MouseButton::Left));
    recognizer.handle_mouse_event(&drag_event);

    // Cancel should emit a cancelled drag
    recognizer.cancel_gesture();

    // After cancel, state should be reset
    let up_event = MouseEvent::new(20, 20, MouseEventKind::Up(MouseButton::Left));
    let result = recognizer.handle_mouse_event(&up_event);
    assert!(result.is_none());
}

#[test]
fn test_recognizer_default() {
    let recognizer = GestureRecognizer::default();
    assert!(recognizer.is_enabled());
}

#[test]
fn test_swipe_gesture_negative_delta() {
    let swipe = SwipeGesture {
        direction: SwipeDirection::Left,
        start_x: 20,
        start_y: 10,
        end_x: 5,
        end_y: 10,
        distance: 15.0,
        velocity: 75.0,
        duration: Duration::from_millis(200),
        button: MouseButton::Left,
    };

    assert_eq!(swipe.delta_x(), -15);
    assert_eq!(swipe.delta_y(), 0);
}

#[test]
fn test_drag_gesture_negative_delta() {
    let drag = DragGesture {
        start_x: 20,
        start_y: 20,
        current_x: 10,
        current_y: 10,
        prev_x: 15,
        prev_y: 15,
        total_distance: 14.14,
        duration: Duration::from_millis(100),
        button: MouseButton::Left,
        state: GestureState::Active,
    };

    assert_eq!(drag.delta_x(), -5);
    assert_eq!(drag.delta_y(), -5);
    assert_eq!(drag.total_delta_x(), -10);
    assert_eq!(drag.total_delta_y(), -10);
}

#[test]
fn test_tap_gesture_fields() {
    let tap = TapGesture {
        x: 10,
        y: 20,
        button: MouseButton::Right,
        count: 2,
    };

    assert_eq!(tap.x, 10);
    assert_eq!(tap.y, 20);
    assert_eq!(tap.button, MouseButton::Right);
    assert_eq!(tap.count, 2);
}

#[test]
fn test_long_press_gesture_fields() {
    let gesture = LongPressGesture {
        x: 15,
        y: 25,
        duration: Duration::from_millis(600),
        button: MouseButton::Left,
    };

    assert_eq!(gesture.x, 15);
    assert_eq!(gesture.y, 25);
    assert_eq!(gesture.duration, Duration::from_millis(600));
    assert_eq!(gesture.button, MouseButton::Left);
}

#[test]
fn test_pinch_gesture_fields() {
    let pinch = PinchGesture {
        direction: PinchDirection::Out,
        x: 50,
        y: 50,
        scale: 1.1,
        cumulative_scale: 1.21,
    };

    assert_eq!(pinch.direction, PinchDirection::Out);
    assert_eq!(pinch.x, 50);
    assert_eq!(pinch.y, 50);
    assert_eq!(pinch.scale, 1.1);
    assert_eq!(pinch.cumulative_scale, 1.21);
}

#[test]
fn test_pinch_in_gesture() {
    let mut recognizer = GestureRecognizer::new();

    let pinched_in = Arc::new(AtomicBool::new(false));
    let pinched_clone = Arc::clone(&pinched_in);

    recognizer.on_pinch(move |pinch| {
        if pinch.direction == PinchDirection::In {
            pinched_clone.store(true, Ordering::SeqCst);
        }
    });

    // Ctrl+scroll down = pinch in
    let scroll_event = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollDown,
        ctrl: true,
        alt: false,
        shift: false,
    };
    recognizer.handle_mouse_event(&scroll_event);

    assert!(pinched_in.load(Ordering::SeqCst));
}

#[test]
fn test_scroll_without_ctrl() {
    let mut recognizer = GestureRecognizer::new();

    // Scroll without Ctrl should not trigger pinch
    let scroll_event = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollUp,
        ctrl: false,
        alt: false,
        shift: false,
    };
    let result = recognizer.handle_mouse_event(&scroll_event);
    assert!(result.is_none());
}

#[test]
fn test_double_tap() {
    let mut recognizer = GestureRecognizer::new();

    let double_tapped = Arc::new(AtomicBool::new(false));
    let dt_clone = Arc::clone(&double_tapped);

    recognizer.on_double_tap(move |tap| {
        assert_eq!(tap.count, 2);
        dt_clone.store(true, Ordering::SeqCst);
    });

    // First tap
    let down1 = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down1);
    let up1 = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up1);

    // Second tap (quickly)
    let down2 = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down2);
    let up2 = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up2);

    assert!(double_tapped.load(Ordering::SeqCst));
}

#[test]
fn test_generic_gesture_handler() {
    let mut recognizer = GestureRecognizer::new();

    let gesture_received = Arc::new(AtomicBool::new(false));
    let gr_clone = Arc::clone(&gesture_received);

    recognizer.on_gesture(move |_| {
        gr_clone.store(true, Ordering::SeqCst);
    });

    // Tap should trigger generic handler
    let down_event = MouseEvent::new(10, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);
    let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    recognizer.handle_mouse_event(&up_event);

    assert!(gesture_received.load(Ordering::SeqCst));
}

#[test]
fn test_move_event() {
    let mut recognizer = GestureRecognizer::new();

    // Move events just update position, don't trigger gestures
    let move_event = MouseEvent {
        x: 20,
        y: 30,
        kind: MouseEventKind::Move,
        ctrl: false,
        alt: false,
        shift: false,
    };
    let result = recognizer.handle_mouse_event(&move_event);
    assert!(result.is_none());
}

#[test]
fn test_horizontal_scroll_events() {
    let mut recognizer = GestureRecognizer::new();

    // Horizontal scroll events don't trigger gestures currently
    let scroll_left = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollLeft,
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert!(recognizer.handle_mouse_event(&scroll_left).is_none());

    let scroll_right = MouseEvent {
        x: 10,
        y: 10,
        kind: MouseEventKind::ScrollRight,
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert!(recognizer.handle_mouse_event(&scroll_right).is_none());
}

#[test]
fn test_check_long_press_disabled() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_enabled(false);

    let result = recognizer.check_long_press();
    assert!(result.is_none());
}

#[test]
fn test_check_long_press_no_button_down() {
    let mut recognizer = GestureRecognizer::new();

    let result = recognizer.check_long_press();
    assert!(result.is_none());
}

#[test]
fn test_button_up_wrong_button() {
    let mut recognizer = GestureRecognizer::new();

    // Press left button
    let down_event = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Release right button (wrong button)
    let up_event = MouseEvent::new(10, 10, MouseEventKind::Up(MouseButton::Right));
    let result = recognizer.handle_mouse_event(&up_event);
    assert!(result.is_none());
}

#[test]
fn test_drag_wrong_button() {
    let mut recognizer = GestureRecognizer::new();

    // Press left button
    let down_event = MouseEvent::new(10, 10, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Drag with right button (wrong button)
    let drag_event = MouseEvent::new(20, 20, MouseEventKind::Drag(MouseButton::Right));
    let result = recognizer.handle_mouse_event(&drag_event);
    assert!(result.is_none());
}

#[test]
fn test_swipe_too_slow() {
    let mut recognizer = GestureRecognizer::new();
    recognizer.set_swipe_threshold(2);
    // Set very high min velocity to make swipes fail
    let mut config = recognizer.config().clone();
    config.swipe_min_velocity = 100000.0;
    recognizer.set_config(config);

    // Press
    let down_event = MouseEvent::new(0, 5, MouseEventKind::Down(MouseButton::Left));
    recognizer.handle_mouse_event(&down_event);

    // Wait a bit to make it slow
    std::thread::sleep(Duration::from_millis(10));

    // Release (should be tap instead of swipe)
    let up_event = MouseEvent::new(10, 5, MouseEventKind::Up(MouseButton::Left));
    let result = recognizer.handle_mouse_event(&up_event);

    // Should be a tap, not a swipe
    assert!(matches!(result, Some(Gesture::Tap(_))));
}

// =============================================================================
// Handler Tests
// =============================================================================

#[test]
fn test_event_context() {
    let mut ctx = EventContext::new();

    assert!(!ctx.is_propagation_stopped());
    assert!(!ctx.is_default_prevented());
    assert!(!ctx.is_handled());

    ctx.stop_propagation();
    assert!(ctx.is_propagation_stopped());

    ctx.prevent_default();
    assert!(ctx.is_default_prevented());

    ctx.set_handled();
    assert!(ctx.is_handled());
}

#[test]
fn test_simple_handler() {
    let mut handler = EventHandler::new();

    handler.on(|event| matches!(event, Event::Key(k) if k.key == Key::Enter));

    let enter = Event::Key(KeyEvent::new(Key::Enter));
    let esc = Event::Key(KeyEvent::new(Key::Escape));

    assert!(handler.dispatch(&enter));
    assert!(!handler.dispatch(&esc));
}

#[test]
fn test_stop_propagation_in_handler() {
    let mut handler = EventHandler::new();

    // Second handler should not be called (added first, called last due to reverse order)
    handler.on_bubble(|_, _| {
        panic!("This should not be called");
    });

    // First handler stops propagation (added last, called first due to reverse order)
    handler.on_bubble(|_, ctx| {
        ctx.stop_propagation();
        true
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    let ctx = handler.dispatch_with_context(&event);

    assert!(ctx.is_propagation_stopped());
    assert!(ctx.is_handled());
}

#[test]
fn test_capture_before_bubble() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let order = Rc::new(RefCell::new(Vec::new()));
    let mut handler = EventHandler::new();

    let order_capture = order.clone();
    handler.on_capture(move |_, _| {
        order_capture.borrow_mut().push("capture");
        false
    });

    let order_bubble = order.clone();
    handler.on_bubble(move |_, _| {
        order_bubble.borrow_mut().push("bubble");
        false
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    handler.dispatch_with_context(&event);

    let result = order.borrow();
    assert_eq!(&*result, &["capture", "bubble"]);
}

#[test]
fn test_handler_removal() {
    let mut handler = EventHandler::new();

    let id1 = handler.on(|_| true);
    let id2 = handler.on(|_| true);

    assert_eq!(handler.handler_count(), 2);

    // Remove first handler
    assert!(handler.remove(id1));
    assert_eq!(handler.handler_count(), 1);

    // Try to remove again - should fail
    assert!(!handler.remove(id1));
    assert_eq!(handler.handler_count(), 1);

    // Remove second handler
    assert!(handler.remove(id2));
    assert!(handler.is_empty());
}

#[test]
fn test_handler_removal_by_type() {
    let mut handler = EventHandler::new();

    let simple_id = handler.on(|_| true);
    let bubble_id = handler.on_bubble(|_, _| true);
    let capture_id = handler.on_capture(|_, _| true);

    assert_eq!(handler.handler_count(), 3);

    // Remove bubble handler
    assert!(handler.remove(bubble_id));
    assert_eq!(handler.handler_count(), 2);

    // Remove capture handler
    assert!(handler.remove(capture_id));
    assert_eq!(handler.handler_count(), 1);

    // Remove simple handler
    assert!(handler.remove(simple_id));
    assert!(handler.is_empty());
}

#[test]
fn test_handler_id_uniqueness() {
    let mut handler = EventHandler::new();

    let id1 = handler.on(|_| true);
    let id2 = handler.on(|_| true);
    let id3 = handler.on_bubble(|_, _| true);

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
}

#[test]
fn test_multiple_capture_handlers_order() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let order = Rc::new(RefCell::new(Vec::new()));
    let mut handler = EventHandler::new();

    // Add three capture handlers - they should be called in order
    let order1 = order.clone();
    handler.on_capture(move |_, _| {
        order1.borrow_mut().push(1);
        false
    });

    let order2 = order.clone();
    handler.on_capture(move |_, _| {
        order2.borrow_mut().push(2);
        false
    });

    let order3 = order.clone();
    handler.on_capture(move |_, _| {
        order3.borrow_mut().push(3);
        false
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    handler.dispatch_with_context(&event);

    assert_eq!(*order.borrow(), vec![1, 2, 3]);
}

#[test]
fn test_multiple_bubble_handlers_reverse_order() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let order = Rc::new(RefCell::new(Vec::new()));
    let mut handler = EventHandler::new();

    // Add three bubble handlers - they should be called in reverse order
    let order1 = order.clone();
    handler.on_bubble(move |_, _| {
        order1.borrow_mut().push(1);
        false
    });

    let order2 = order.clone();
    handler.on_bubble(move |_, _| {
        order2.borrow_mut().push(2);
        false
    });

    let order3 = order.clone();
    handler.on_bubble(move |_, _| {
        order3.borrow_mut().push(3);
        false
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    handler.dispatch_with_context(&event);

    // Bubble handlers are called in reverse order (3, 2, 1)
    assert_eq!(*order.borrow(), vec![3, 2, 1]);
}

#[test]
fn test_stop_propagation_in_capture_phase() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let called = Rc::new(RefCell::new(Vec::new()));
    let mut handler = EventHandler::new();

    // Capture handler that stops propagation
    let called1 = called.clone();
    handler.on_capture(move |_, ctx| {
        called1.borrow_mut().push("capture1");
        ctx.stop_propagation();
        true
    });

    // This capture handler should NOT be called
    let called2 = called.clone();
    handler.on_capture(move |_, _| {
        called2.borrow_mut().push("capture2");
        false
    });

    // Bubble handler should NOT be called
    let called3 = called.clone();
    handler.on_bubble(move |_, _| {
        called3.borrow_mut().push("bubble");
        false
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    let ctx = handler.dispatch_with_context(&event);

    assert!(ctx.is_propagation_stopped());
    assert_eq!(*called.borrow(), vec!["capture1"]);
}

#[test]
fn test_prevent_default() {
    let mut handler = EventHandler::new();

    handler.on_bubble(|_, ctx| {
        ctx.prevent_default();
        true
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    let ctx = handler.dispatch_with_context(&event);

    assert!(ctx.is_default_prevented());
    assert!(ctx.is_handled());
}

#[test]
fn test_handler_returns_false_continues() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let called = Rc::new(RefCell::new(0));
    let mut handler = EventHandler::new();

    // Handler that returns false (not handled)
    let called1 = called.clone();
    handler.on(move |_| {
        *called1.borrow_mut() += 1;
        false
    });

    // Second handler should still be called
    let called2 = called.clone();
    handler.on(move |_| {
        *called2.borrow_mut() += 1;
        true
    });

    let event = Event::Key(KeyEvent::new(Key::Enter));
    let result = handler.dispatch(&event);

    assert!(result);
    assert_eq!(*called.borrow(), 2);
}

#[test]
fn test_clear_removes_all_handlers() {
    let mut handler = EventHandler::new();

    handler.on(|_| true);
    handler.on_bubble(|_, _| true);
    handler.on_capture(|_, _| true);

    assert_eq!(handler.handler_count(), 3);

    handler.clear();

    assert!(handler.is_empty());
    assert_eq!(handler.handler_count(), 0);
}

#[test]
fn test_empty_handler_dispatch() {
    let handler = EventHandler::new();

    let event = Event::Key(KeyEvent::new(Key::Enter));
    let result = handler.dispatch(&event);

    assert!(!result);

    let ctx = handler.dispatch_with_context(&event);
    assert!(!ctx.is_handled());
}

// =============================================================================
// IME Tests
// =============================================================================

#[test]
fn test_composition_state_default() {
    let ime = ImeState::new();
    assert_eq!(ime.state(), CompositionState::Idle);
    assert!(!ime.is_composing());
}

#[test]
fn test_start_composition() {
    let mut ime = ImeState::new();
    ime.start_composition();

    assert_eq!(ime.state(), CompositionState::Composing);
    assert!(ime.is_composing());
}

#[test]
fn test_update_composition() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("かん", 2);

    assert_eq!(ime.composing_text(), "かん");
    assert_eq!(ime.cursor(), 2);
}

#[test]
fn test_commit() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("かん", 2);

    let result = ime.commit("漢");

    assert_eq!(result, Some("漢".to_string()));
    assert!(!ime.is_composing());
}

#[test]
fn test_cancel() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("test", 4);
    ime.cancel();

    assert!(!ime.is_composing());
    assert!(ime.composing_text().is_empty());
}

#[test]
fn test_candidates() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("kan", 3);

    ime.set_candidates(vec![
        Candidate::new("漢"),
        Candidate::new("感"),
        Candidate::new("間"),
    ]);

    assert_eq!(ime.candidates().len(), 3);
    assert_eq!(ime.selected_candidate(), 0);
    assert_eq!(ime.selected_text(), Some("漢"));
}

#[test]
fn test_candidate_navigation() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.set_candidates(vec![
        Candidate::new("a"),
        Candidate::new("b"),
        Candidate::new("c"),
    ]);

    assert_eq!(ime.selected_candidate(), 0);

    ime.next_candidate();
    assert_eq!(ime.selected_candidate(), 1);

    ime.next_candidate();
    assert_eq!(ime.selected_candidate(), 2);

    ime.next_candidate(); // Wraps around
    assert_eq!(ime.selected_candidate(), 0);

    ime.prev_candidate(); // Wraps around
    assert_eq!(ime.selected_candidate(), 2);
}

#[test]
fn test_commit_selected() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.set_candidates(vec![Candidate::new("first"), Candidate::new("second")]);
    ime.next_candidate();

    let result = ime.commit_selected();
    assert_eq!(result, Some("second".to_string()));
}

#[test]
fn test_backspace() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("test", 4);

    assert!(ime.backspace());
    assert_eq!(ime.composing_text(), "tes");
    assert_eq!(ime.cursor(), 3);
}

#[test]
fn test_backspace_cancels_empty() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("a", 1);

    ime.backspace();
    assert!(!ime.is_composing());
}

#[test]
fn test_cursor_movement() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("test", 4);

    ime.move_cursor_left();
    assert_eq!(ime.cursor(), 3);

    ime.move_cursor_left();
    ime.move_cursor_left();
    ime.move_cursor_left();
    assert_eq!(ime.cursor(), 0);

    ime.move_cursor_left(); // Should not go below 0
    assert_eq!(ime.cursor(), 0);

    ime.move_cursor_right();
    assert_eq!(ime.cursor(), 1);
}

#[test]
fn test_ime_callback() {
    let mut ime = ImeState::new();
    let event_count = Arc::new(AtomicUsize::new(0));
    let count_clone = Arc::clone(&event_count);

    ime.on_composition(move |_| {
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    ime.start_composition();
    ime.update_composition("a", 1);
    ime.commit("A");

    assert_eq!(event_count.load(Ordering::SeqCst), 3); // start, update, end
}

#[test]
fn test_disabled_ime() {
    let mut ime = ImeState::new();
    ime.disable();

    ime.start_composition();
    assert!(!ime.is_composing());
}

#[test]
fn test_candidate_builder() {
    let candidate = Candidate::new("漢")
        .with_label("1")
        .with_annotation("Chinese character");

    assert_eq!(candidate.text, "漢");
    assert_eq!(candidate.label, Some("1".to_string()));
    assert_eq!(candidate.annotation, Some("Chinese character".to_string()));
}

#[test]
fn test_preedit_string() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("hello", 2);

    let preedit = PreeditString::from_ime(&ime);
    assert!(!preedit.is_empty());
    assert_eq!(preedit.text(), "hello");
}

#[test]
fn test_ime_config() {
    use revue::event::CompositionStyle;

    let config = ImeConfig {
        composition_style: CompositionStyle::Highlight,
        show_candidates: false,
        max_candidates: 5,
        candidate_offset: (1, 2),
        inline_composition: true,
    };

    let ime = ImeState::with_config(config);
    assert_eq!(ime.config().max_candidates, 5);
}

// =============================================================================
// KeyMap Tests
// =============================================================================

#[test]
fn test_key_char() {
    let key = Key::Char('a');
    assert_eq!(key, Key::Char('a'));
}

#[test]
fn test_key_enter() {
    let key = Key::Enter;
    assert_eq!(key, Key::Enter);
}

#[test]
fn test_key_escape() {
    let key = Key::Escape;
    assert_eq!(key, Key::Escape);
}

#[test]
fn test_key_tab() {
    let key = Key::Tab;
    assert_eq!(key, Key::Tab);
}

#[test]
fn test_key_backtab() {
    let key = Key::BackTab;
    assert_eq!(key, Key::BackTab);
}

#[test]
fn test_key_backspace() {
    let key = Key::Backspace;
    assert_eq!(key, Key::Backspace);
}

#[test]
fn test_key_delete() {
    let key = Key::Delete;
    assert_eq!(key, Key::Delete);
}

#[test]
fn test_key_arrows() {
    assert_eq!(Key::Up, Key::Up);
    assert_eq!(Key::Down, Key::Down);
    assert_eq!(Key::Left, Key::Left);
    assert_eq!(Key::Right, Key::Right);
}

#[test]
fn test_key_home_end() {
    assert_eq!(Key::Home, Key::Home);
    assert_eq!(Key::End, Key::End);
}

#[test]
fn test_key_page_up_down() {
    assert_eq!(Key::PageUp, Key::PageUp);
    assert_eq!(Key::PageDown, Key::PageDown);
}

#[test]
fn test_key_function() {
    let f1 = Key::F(1);
    let f12 = Key::F(12);
    assert_eq!(f1, Key::F(1));
    assert_eq!(f12, Key::F(12));
    assert_ne!(f1, f12);
}

#[test]
fn test_key_insert() {
    assert_eq!(Key::Insert, Key::Insert);
}

#[test]
fn test_key_null() {
    assert_eq!(Key::Null, Key::Null);
}

#[test]
fn test_key_unknown() {
    assert_eq!(Key::Unknown, Key::Unknown);
}

#[test]
fn test_key_equality() {
    assert_eq!(Key::Char('a'), Key::Char('a'));
    assert_ne!(Key::Char('a'), Key::Char('b'));
    assert_ne!(Key::Char('a'), Key::Enter);
}

#[test]
fn test_key_clone() {
    let key = Key::Char('x');
    let cloned = key;
    assert_eq!(key, cloned);
}

#[test]
fn test_key_debug() {
    let key = Key::Enter;
    let debug = format!("{:?}", key);
    assert!(debug.contains("Enter"));
}

#[test]
fn test_key_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Key::Char('a'));
    set.insert(Key::Enter);
    set.insert(Key::Char('a')); // Duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn test_key_ctrl() {
    let binding = Key::ctrl('c');
    assert_eq!(binding.key, Key::Char('c'));
    assert!(binding.ctrl);
    assert!(!binding.alt);
    assert!(!binding.shift);
}

#[test]
fn test_key_alt() {
    let binding = Key::alt('x');
    assert_eq!(binding.key, Key::Char('x'));
    assert!(!binding.ctrl);
    assert!(binding.alt);
    assert!(!binding.shift);
}

#[test]
fn test_keybinding_creation() {
    let binding = KeyBinding {
        key: Key::Enter,
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert_eq!(binding.key, Key::Enter);
    assert!(!binding.ctrl);
    assert!(!binding.alt);
    assert!(!binding.shift);
}

#[test]
fn test_keybinding_with_ctrl() {
    let binding = KeyBinding {
        key: Key::Char('s'),
        ctrl: true,
        alt: false,
        shift: false,
    };
    assert!(binding.ctrl);
}

#[test]
fn test_keybinding_with_alt() {
    let binding = KeyBinding {
        key: Key::Char('f'),
        ctrl: false,
        alt: true,
        shift: false,
    };
    assert!(binding.alt);
}

#[test]
fn test_keybinding_with_shift() {
    let binding = KeyBinding {
        key: Key::Tab,
        ctrl: false,
        alt: false,
        shift: true,
    };
    assert!(binding.shift);
}

#[test]
fn test_keybinding_with_multiple_modifiers() {
    let binding = KeyBinding {
        key: Key::Char('k'),
        ctrl: true,
        alt: true,
        shift: true,
    };
    assert!(binding.ctrl);
    assert!(binding.alt);
    assert!(binding.shift);
}

#[test]
fn test_keybinding_equality() {
    let b1 = KeyBinding {
        key: Key::Char('a'),
        ctrl: true,
        alt: false,
        shift: false,
    };
    let b2 = KeyBinding {
        key: Key::Char('a'),
        ctrl: true,
        alt: false,
        shift: false,
    };
    let b3 = KeyBinding {
        key: Key::Char('a'),
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert_eq!(b1, b2);
    assert_ne!(b1, b3);
}

#[test]
fn test_keybinding_clone() {
    let binding = KeyBinding {
        key: Key::Escape,
        ctrl: false,
        alt: true,
        shift: false,
    };
    let cloned = binding.clone();
    assert_eq!(binding, cloned);
}

#[test]
fn test_keybinding_debug() {
    let binding = Key::ctrl('c');
    let debug = format!("{:?}", binding);
    assert!(debug.contains("Char"));
    assert!(debug.contains("ctrl"));
}

#[test]
fn test_keybinding_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Key::ctrl('c'));
    set.insert(Key::ctrl('v'));
    set.insert(Key::ctrl('c')); // Duplicate
    assert_eq!(set.len(), 2);
}

#[derive(Debug, Clone, PartialEq)]
enum TestAction {
    Save,
    Quit,
    Copy,
    Paste,
    Undo,
}

#[test]
fn test_keymap_new() {
    let map: KeyMap<TestAction> = KeyMap::new();
    assert!(map.get(&Key::ctrl('s')).is_none());
}

#[test]
fn test_keymap_default() {
    let map: KeyMap<TestAction> = KeyMap::default();
    assert!(map.get(&Key::ctrl('s')).is_none());
}

#[test]
fn test_keymap_bind() {
    let mut map = KeyMap::new();
    map.bind(Key::ctrl('s'), TestAction::Save);
    assert!(map.get(&Key::ctrl('s')).is_some());
}

#[test]
fn test_keymap_get() {
    let mut map = KeyMap::new();
    map.bind(Key::ctrl('s'), TestAction::Save);

    let result = map.get(&Key::ctrl('s'));
    assert_eq!(result, Some(&TestAction::Save));
}

#[test]
fn test_keymap_get_not_found() {
    let map: KeyMap<TestAction> = KeyMap::new();
    let result = map.get(&Key::ctrl('x'));
    assert!(result.is_none());
}

#[test]
fn test_keymap_multiple_bindings() {
    let mut map = KeyMap::new();
    map.bind(Key::ctrl('s'), TestAction::Save);
    map.bind(Key::ctrl('q'), TestAction::Quit);
    map.bind(Key::ctrl('c'), TestAction::Copy);
    map.bind(Key::ctrl('v'), TestAction::Paste);
    map.bind(Key::ctrl('z'), TestAction::Undo);

    assert_eq!(map.get(&Key::ctrl('s')), Some(&TestAction::Save));
    assert_eq!(map.get(&Key::ctrl('q')), Some(&TestAction::Quit));
    assert_eq!(map.get(&Key::ctrl('c')), Some(&TestAction::Copy));
    assert_eq!(map.get(&Key::ctrl('v')), Some(&TestAction::Paste));
    assert_eq!(map.get(&Key::ctrl('z')), Some(&TestAction::Undo));
}

#[test]
fn test_keymap_overwrite_binding() {
    let mut map = KeyMap::new();
    map.bind(Key::ctrl('s'), TestAction::Save);
    map.bind(Key::ctrl('s'), TestAction::Quit); // Overwrite

    assert_eq!(map.get(&Key::ctrl('s')), Some(&TestAction::Quit));
}

#[test]
fn test_keymap_with_different_modifiers() {
    let mut map = KeyMap::new();

    // Same key, different modifiers
    let ctrl_a = KeyBinding {
        key: Key::Char('a'),
        ctrl: true,
        alt: false,
        shift: false,
    };
    let alt_a = KeyBinding {
        key: Key::Char('a'),
        ctrl: false,
        alt: true,
        shift: false,
    };

    map.bind(ctrl_a.clone(), TestAction::Copy);
    map.bind(alt_a.clone(), TestAction::Paste);

    assert_eq!(map.get(&ctrl_a), Some(&TestAction::Copy));
    assert_eq!(map.get(&alt_a), Some(&TestAction::Paste));
}

#[test]
fn test_keymap_with_non_char_keys() {
    let mut map = KeyMap::new();

    let f1_binding = KeyBinding {
        key: Key::F(1),
        ctrl: false,
        alt: false,
        shift: false,
    };
    let enter_binding = KeyBinding {
        key: Key::Enter,
        ctrl: false,
        alt: false,
        shift: false,
    };

    map.bind(f1_binding.clone(), TestAction::Save);
    map.bind(enter_binding.clone(), TestAction::Quit);

    assert_eq!(map.get(&f1_binding), Some(&TestAction::Save));
    assert_eq!(map.get(&enter_binding), Some(&TestAction::Quit));
}

#[test]
fn test_keymap_with_string_action() {
    let mut map: KeyMap<String> = KeyMap::new();
    map.bind(Key::ctrl('s'), "save".to_string());
    map.bind(Key::ctrl('q'), "quit".to_string());

    assert_eq!(map.get(&Key::ctrl('s')), Some(&"save".to_string()));
    assert_eq!(map.get(&Key::ctrl('q')), Some(&"quit".to_string()));
}

#[test]
fn test_keymap_with_integer_action() {
    let mut map: KeyMap<i32> = KeyMap::new();
    map.bind(Key::ctrl('1'), 1);
    map.bind(Key::ctrl('2'), 2);
    map.bind(Key::ctrl('3'), 3);

    assert_eq!(map.get(&Key::ctrl('1')), Some(&1));
    assert_eq!(map.get(&Key::ctrl('2')), Some(&2));
    assert_eq!(map.get(&Key::ctrl('3')), Some(&3));
}

// =============================================================================
// EventReader Tests
// =============================================================================

#[test]
fn test_event_reader_creation() {
    let reader = EventReader::new(Duration::from_millis(100));
    // Can't easily test tick_rate without accessing private fields
    // Just verify it creates without panic
    let _ = reader;
}

#[test]
fn test_event_reader_default() {
    let reader = EventReader::default();
    // Verify default creates without panic
    let _ = reader;
}
