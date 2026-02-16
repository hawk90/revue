//! Tests for View, Interactive, and Draggable traits
//!
//! Extracted from src/widget/traits/view.rs

use revue::event::drag::{DragData, DropResult};
use revue::event::{KeyEvent, MouseEvent, MouseEventKind, MouseButton};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::event::EventResult;
use revue::widget::traits::render_context::RenderContext;
use revue::widget::traits::{Draggable, Interactive, StyledView, View};

// Test View implementation
struct TestView {
    id: Option<String>,
    classes: Vec<String>,
}

impl TestView {
    fn new() -> Self {
        Self {
            id: None,
            classes: Vec::new(),
        }
    }

    fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    fn with_class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }
}

impl View for TestView {
    fn render(&self, _ctx: &mut RenderContext) {}

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }
}

#[test]
fn test_view_widget_type() {
    let view = TestView::new();
    assert!(view.widget_type().contains("TestView"));
}

#[test]
fn test_view_id_none() {
    let view = TestView::new();
    assert!(view.id().is_none());
}

#[test]
fn test_view_id_some() {
    let view = TestView::new().with_id("my-view");
    assert_eq!(view.id(), Some("my-view"));
}

#[test]
fn test_view_classes_empty() {
    let view = TestView::new();
    assert!(view.classes().is_empty());
}

#[test]
fn test_view_classes_with_values() {
    let view = TestView::new().with_class("primary").with_class("active");
    assert_eq!(view.classes().len(), 2);
}

#[test]
fn test_view_children_default() {
    let view = TestView::new();
    assert!(view.children().is_empty());
}

#[test]
fn test_view_meta() {
    let view = TestView::new().with_id("test-id").with_class("test-class");
    let meta = view.meta();
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_view_render() {
    let view = TestView::new();
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
}

#[test]
fn test_boxed_view() {
    let view: Box<dyn View> = Box::new(TestView::new().with_id("boxed"));
    assert_eq!(view.id(), Some("boxed"));
    assert!(view.widget_type().contains("TestView"));
    assert!(view.children().is_empty());
}

#[test]
fn test_boxed_view_meta() {
    let view: Box<dyn View> = Box::new(TestView::new().with_class("box-class"));
    let meta = view.meta();
    assert!(meta.classes.contains("box-class"));
}

// Interactive trait tests
struct TestInteractive;

impl View for TestInteractive {
    fn render(&self, _ctx: &mut RenderContext) {}
}

impl Interactive for TestInteractive {}

#[test]
fn test_interactive_handle_key_default() {
    use revue::event::Key;
    let mut widget = TestInteractive;
    let event = KeyEvent::new(Key::Enter);
    assert_eq!(widget.handle_key(&event), EventResult::Ignored);
}

#[test]
fn test_interactive_handle_mouse_default() {
    let mut widget = TestInteractive;
    let event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
    let area = Rect::new(0, 0, 10, 10);
    assert_eq!(widget.handle_mouse(&event, area), EventResult::Ignored);
}

#[test]
fn test_interactive_focusable_default() {
    let widget = TestInteractive;
    assert!(widget.focusable());
}

#[test]
fn test_interactive_focus_blur() {
    let mut widget = TestInteractive;
    widget.on_focus();
    widget.on_blur();
    // Just verify they don't panic
}

// Draggable trait tests
struct TestDraggable;

impl View for TestDraggable {
    fn render(&self, _ctx: &mut RenderContext) {}
}

impl Draggable for TestDraggable {}

#[test]
fn test_draggable_can_drag_default() {
    let widget = TestDraggable;
    assert!(!widget.can_drag());
}

#[test]
fn test_draggable_drag_data_default() {
    let widget = TestDraggable;
    assert!(widget.drag_data().is_none());
}

#[test]
fn test_draggable_drag_preview_default() {
    let widget = TestDraggable;
    assert!(widget.drag_preview().is_none());
}

#[test]
fn test_draggable_on_drag_start_end() {
    let mut widget = TestDraggable;
    widget.on_drag_start();
    widget.on_drag_end(DropResult::Cancelled);
}

#[test]
fn test_draggable_can_drop_default() {
    let widget = TestDraggable;
    assert!(!widget.can_drop());
}

#[test]
fn test_draggable_accepted_types_default() {
    let widget = TestDraggable;
    assert!(widget.accepted_types().is_empty());
}

#[test]
fn test_draggable_can_accept_empty_types() {
    let widget = TestDraggable;
    let data = DragData::text("test");
    // Empty accepted_types means accept all
    assert!(widget.can_accept(&data));
}

#[test]
fn test_draggable_on_drag_enter_leave() {
    let mut widget = TestDraggable;
    let data = DragData::text("test");
    widget.on_drag_enter(&data);
    widget.on_drag_leave();
}

#[test]
fn test_draggable_on_drop_default() {
    let mut widget = TestDraggable;
    let data = DragData::text("test");
    assert!(!widget.on_drop(data));
}

#[test]
fn test_draggable_drop_bounds_default() {
    let widget = TestDraggable;
    let area = Rect::new(10, 20, 30, 40);
    assert_eq!(widget.drop_bounds(area), area);
}

#[test]
fn test_boxed_view_render() {
    let view: Box<dyn View> = Box::new(TestView::new());
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
}

#[test]
fn test_boxed_view_classes() {
    let view: Box<dyn View> = Box::new(TestView::new().with_class("a").with_class("b"));
    assert_eq!(view.classes().len(), 2);
}
