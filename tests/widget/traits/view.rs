//! Tests for the View, Interactive and Draggable trait defaults

use revue::event::drag::DragData;
use revue::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{Draggable, EventResult, Interactive, RenderContext, View};
use revue::widget::Text;

/// A view with an id and classes that draws a marker character
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
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_char(0, 0, '#', revue::style::Color::WHITE);
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }
}

// =========================================================================
// View defaults
// =========================================================================

#[test]
fn test_view_widget_type_is_the_type_name() {
    assert_eq!(TestView::new().widget_type(), "TestView");
}

#[test]
fn test_view_id_and_classes() {
    let plain = TestView::new();
    assert!(plain.id().is_none());
    assert!(plain.classes().is_empty());

    let view = TestView::new()
        .with_id("my-view")
        .with_class("primary")
        .with_class("active");
    assert_eq!(view.id(), Some("my-view"));
    assert_eq!(view.classes(), &["primary", "active"]);
}

#[test]
fn test_view_defaults() {
    let view = TestView::new();
    assert!(view.children().is_empty());
    assert!(view.needs_render());
    assert!(view.key().is_none());
}

#[test]
fn test_view_meta_combines_type_id_and_classes() {
    let view = TestView::new().with_id("test-id").with_class("test-class");
    let meta = view.meta();
    assert_eq!(meta.widget_type, "TestView");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
    assert!(meta.key.is_none());
}

#[test]
fn test_widget_classes_exposure() {
    let widget = Text::new("Test").class("btn").class("primary");

    let classes = View::classes(&widget);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"btn".to_string()));
    assert!(classes.contains(&"primary".to_string()));

    let meta = widget.meta();
    assert!(meta.classes.contains("btn"));
    assert!(meta.classes.contains("primary"));
}

// =========================================================================
// Box<dyn View> delegates to the inner view
// =========================================================================

#[test]
fn test_boxed_view_delegates_metadata() {
    let view: Box<dyn View> = Box::new(TestView::new().with_id("boxed").with_class("a"));
    assert_eq!(view.widget_type(), "TestView");
    assert_eq!(view.id(), Some("boxed"));
    assert_eq!(view.classes(), &["a"]);
    assert!(view.children().is_empty());

    let meta = view.meta();
    assert_eq!(meta.widget_type, "TestView");
    assert!(meta.classes.contains("a"));
}

#[test]
fn test_boxed_view_delegates_render() {
    let view: Box<dyn View> = Box::new(TestView::new());
    let mut buffer = Buffer::new(4, 1);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 1));
        view.render(&mut ctx);
    }
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
}

// =========================================================================
// Interactive defaults
// =========================================================================

struct TestInteractive;

impl View for TestInteractive {
    fn render(&self, _ctx: &mut RenderContext) {}
}

impl Interactive for TestInteractive {}

#[test]
fn test_interactive_ignores_events_by_default() {
    let mut widget = TestInteractive;
    assert_eq!(
        widget.handle_key(&KeyEvent::new(Key::Enter)),
        EventResult::Ignored
    );

    let event = MouseEvent::new(5, 5, MouseEventKind::Down(MouseButton::Left));
    assert_eq!(
        widget.handle_mouse(&event, Rect::new(0, 0, 10, 10)),
        EventResult::Ignored
    );
}

#[test]
fn test_interactive_focusable_by_default() {
    assert!(TestInteractive.focusable());
}

// =========================================================================
// Draggable defaults
// =========================================================================

struct TestDraggable;

impl View for TestDraggable {
    fn render(&self, _ctx: &mut RenderContext) {}
}

impl Draggable for TestDraggable {}

struct FileDropTarget;

impl View for FileDropTarget {
    fn render(&self, _ctx: &mut RenderContext) {}
}

impl Draggable for FileDropTarget {
    fn accepted_types(&self) -> &[&'static str] {
        &["file"]
    }
}

#[test]
fn test_draggable_is_inert_by_default() {
    let mut widget = TestDraggable;
    assert!(!widget.can_drag());
    assert!(widget.drag_data().is_none());
    assert!(widget.drag_preview().is_none());
    assert!(!widget.can_drop());
    assert!(widget.accepted_types().is_empty());
    assert!(!widget.on_drop(DragData::text("test")));
}

#[test]
fn test_draggable_empty_accepted_types_accepts_anything() {
    let widget = TestDraggable;
    assert!(widget.can_accept(&DragData::text("test")));
    assert!(widget.can_accept(&DragData::new("file", 42u32)));
}

#[test]
fn test_draggable_accepted_types_filters_by_type_id() {
    let widget = FileDropTarget;
    assert!(widget.can_accept(&DragData::new("file", 42u32)));
    assert!(!widget.can_accept(&DragData::text("test")));
}

#[test]
fn test_draggable_drop_bounds_default_is_the_area() {
    let area = Rect::new(10, 20, 30, 40);
    assert_eq!(TestDraggable.drop_bounds(area), area);
}
