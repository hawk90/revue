//! Drag event tests

use revue::event::{DragContext, DragData, DragState, DropTarget};
use revue::layout::Rect;
use serial_test::serial;

#[serial]
#[test]
fn test_drag_data_text() {
    let data = DragData::text("Hello");
    assert_eq!(data.type_id, "text");
    assert_eq!(data.as_text(), Some("Hello"));
    assert_eq!(data.display_label(), "Hello");
}

#[serial]
#[test]
fn test_drag_data_list_item() {
    let data = DragData::list_item(5, "Item 5");
    assert_eq!(data.type_id, "list_item");
    assert_eq!(data.as_list_index(), Some(5));
    assert_eq!(data.display_label(), "Item 5");
}

#[serial]
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

#[serial]
#[test]
fn test_drag_state() {
    assert!(!DragState::Idle.is_active());
    assert!(DragState::Dragging.is_active());
    assert!(DragState::OverTarget.is_active());
    assert!(!DragState::Dropped.is_active());

    assert!(!DragState::Dragging.is_over_target());
    assert!(DragState::OverTarget.is_over_target());
}

#[serial]
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

#[serial]
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

#[serial]
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

#[serial]
#[test]
fn test_drop_target_accepts_all() {
    let target = DropTarget::new(1, Rect::new(0, 0, 10, 10)).accepts_all();

    let text_data = DragData::text("Hello");
    let other_data = DragData::new("whatever", 42);

    assert!(target.can_accept(&text_data));
    assert!(target.can_accept(&other_data));
}

#[serial]
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

#[serial]
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

#[serial]
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

#[serial]
#[test]
fn test_drag_offset() {
    let mut ctx = DragContext::new();

    ctx.start_drag(DragData::text("Test"), 10, 20);
    ctx.update_position(25, 15);

    assert_eq!(ctx.offset(), (15, -5));
}
