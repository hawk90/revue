//! Event handler tests

use revue::event::{Event, EventContext, EventHandler, Key, KeyEvent};

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
