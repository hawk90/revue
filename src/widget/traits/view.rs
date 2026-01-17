//! View and related traits for widgets

use crate::dom::WidgetMeta;
use crate::event::drag::{DragData, DropResult};
use crate::event::{KeyEvent, MouseEvent};
use crate::layout::Rect;

use super::event::EventResult;
use super::render_context::RenderContext;

/// The core trait for all renderable components
pub trait View {
    /// Render the view
    fn render(&self, ctx: &mut RenderContext);

    /// Get widget type name (for CSS type selectors)
    fn widget_type(&self) -> &'static str {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap_or("Unknown")
    }

    /// Get element ID (for CSS #id selectors)
    fn id(&self) -> Option<&str> {
        None
    }

    /// Get CSS classes (for CSS .class selectors)
    fn classes(&self) -> &[String] {
        &[]
    }

    /// Get child views (for container widgets)
    ///
    /// Container widgets (Stack, Grid, etc.) should override this to expose
    /// their children, enabling the DOM builder to traverse the full widget tree.
    fn children(&self) -> &[Box<dyn View>] {
        &[]
    }

    /// Get widget metadata for DOM
    fn meta(&self) -> WidgetMeta {
        let mut meta = WidgetMeta::new(self.widget_type());
        if let Some(id) = self.id() {
            meta.id = Some(id.to_string());
        }
        for class in self.classes() {
            meta.classes.insert(class.clone());
        }
        meta
    }
}

/// Implement View for `Box<dyn View>` to allow boxed views to be used as children
impl View for Box<dyn View> {
    fn render(&self, ctx: &mut RenderContext) {
        (**self).render(ctx);
    }

    fn widget_type(&self) -> &'static str {
        (**self).widget_type()
    }

    fn id(&self) -> Option<&str> {
        (**self).id()
    }

    fn classes(&self) -> &[String] {
        (**self).classes()
    }

    fn children(&self) -> &[Box<dyn View>] {
        (**self).children()
    }

    fn meta(&self) -> WidgetMeta {
        (**self).meta()
    }
}

/// Trait for interactive widgets that handle events
///
/// This trait extends View with keyboard and mouse handling capabilities.
/// Widgets that need to respond to user input should implement this trait.
pub trait Interactive: View {
    /// Handle keyboard event
    ///
    /// Returns `EventResult::Consumed` or `EventResult::ConsumedAndRender` if
    /// the event was handled, `EventResult::Ignored` to let it propagate.
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        let _ = event;
        EventResult::Ignored
    }

    /// Handle mouse event
    ///
    /// Returns `EventResult` indicating if event was handled.
    fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
        let _ = (event, area);
        EventResult::Ignored
    }

    /// Check if the widget can receive focus
    fn focusable(&self) -> bool {
        true
    }

    /// Called when the widget receives focus
    fn on_focus(&mut self) {}

    /// Called when the widget loses focus
    fn on_blur(&mut self) {}
}

/// Trait for widgets that support drag-and-drop
pub trait Draggable: View {
    /// Check if this widget can be dragged
    fn can_drag(&self) -> bool {
        false
    }

    /// Get the drag data when a drag starts
    ///
    /// Return None to cancel the drag.
    fn drag_data(&self) -> Option<DragData> {
        None
    }

    /// Get a text preview for the drag operation
    ///
    /// This is shown near the cursor during drag.
    fn drag_preview(&self) -> Option<String> {
        None
    }

    /// Called when drag starts
    fn on_drag_start(&mut self) {}

    /// Called when drag ends (regardless of outcome)
    fn on_drag_end(&mut self, _result: DropResult) {}

    /// Check if this widget accepts drops
    fn can_drop(&self) -> bool {
        false
    }

    /// Get the types this widget accepts for drops
    fn accepted_types(&self) -> &[&'static str] {
        &[]
    }

    /// Check if this widget can accept specific drag data
    fn can_accept(&self, data: &DragData) -> bool {
        let types = self.accepted_types();
        types.is_empty() || types.contains(&data.type_id)
    }

    /// Called when a drag enters this widget's bounds
    fn on_drag_enter(&mut self, _data: &DragData) {}

    /// Called when a drag leaves this widget's bounds
    fn on_drag_leave(&mut self) {}

    /// Called when a drop occurs on this widget
    ///
    /// Return true if the drop was accepted, false to reject.
    fn on_drop(&mut self, _data: DragData) -> bool {
        false
    }

    /// Get the drop zone bounds for this widget
    ///
    /// Override this if the drop zone differs from the render area.
    fn drop_bounds(&self, area: Rect) -> Rect {
        area
    }
}

/// Extended View trait with styling support
pub trait StyledView: View {
    /// Set element ID
    fn set_id(&mut self, id: impl Into<String>);

    /// Add a CSS class
    fn add_class(&mut self, class: impl Into<String>);

    /// Remove a CSS class
    fn remove_class(&mut self, class: &str);

    /// Toggle a CSS class
    fn toggle_class(&mut self, class: &str);

    /// Check if has class
    fn has_class(&self, class: &str) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

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
        use crate::event::Key;
        let mut widget = TestInteractive;
        let event = KeyEvent::new(Key::Enter);
        assert_eq!(widget.handle_key(&event), EventResult::Ignored);
    }

    #[test]
    fn test_interactive_handle_mouse_default() {
        use crate::event::MouseEventKind;
        let mut widget = TestInteractive;
        let event = MouseEvent::new(5, 5, MouseEventKind::Down(crate::event::MouseButton::Left));
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
}
