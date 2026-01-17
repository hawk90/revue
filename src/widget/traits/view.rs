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

    // =========================================================================
    // Test View Implementation
    // =========================================================================

    struct TestWidget {
        id: Option<String>,
        classes: Vec<String>,
    }

    impl TestWidget {
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

    impl View for TestWidget {
        fn render(&self, _ctx: &mut RenderContext) {
            // No-op for testing
        }

        fn id(&self) -> Option<&str> {
            self.id.as_deref()
        }

        fn classes(&self) -> &[String] {
            &self.classes
        }
    }

    // =========================================================================
    // View Trait Tests
    // =========================================================================

    #[test]
    fn test_view_widget_type() {
        let widget = TestWidget::new();
        let type_name = widget.widget_type();
        assert!(type_name.contains("TestWidget"));
    }

    #[test]
    fn test_view_id_none() {
        let widget = TestWidget::new();
        assert!(widget.id().is_none());
    }

    #[test]
    fn test_view_id_some() {
        let widget = TestWidget::new().with_id("my-widget");
        assert_eq!(widget.id(), Some("my-widget"));
    }

    #[test]
    fn test_view_classes_empty() {
        let widget = TestWidget::new();
        assert!(widget.classes().is_empty());
    }

    #[test]
    fn test_view_classes_with_values() {
        let widget = TestWidget::new().with_class("primary").with_class("active");
        let classes = widget.classes();
        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0], "primary");
        assert_eq!(classes[1], "active");
    }

    #[test]
    fn test_view_children_default() {
        let widget = TestWidget::new();
        assert!(widget.children().is_empty());
    }

    #[test]
    fn test_view_meta_basic() {
        let widget = TestWidget::new();
        let meta = widget.meta();
        assert!(meta.widget_type.contains("TestWidget"));
        assert!(meta.id.is_none());
        assert!(meta.classes.is_empty());
    }

    #[test]
    fn test_view_meta_with_id() {
        let widget = TestWidget::new().with_id("test-id");
        let meta = widget.meta();
        assert_eq!(meta.id, Some("test-id".to_string()));
    }

    #[test]
    fn test_view_meta_with_classes() {
        let widget = TestWidget::new().with_class("foo").with_class("bar");
        let meta = widget.meta();
        assert!(meta.classes.contains("foo"));
        assert!(meta.classes.contains("bar"));
    }

    #[test]
    fn test_view_render() {
        let widget = TestWidget::new();
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        widget.render(&mut ctx);
        // Just verify it doesn't panic
    }

    // =========================================================================
    // Box<dyn View> Tests
    // =========================================================================

    #[test]
    fn test_boxed_view_widget_type() {
        let widget: Box<dyn View> = Box::new(TestWidget::new());
        assert!(widget.widget_type().contains("TestWidget"));
    }

    #[test]
    fn test_boxed_view_id() {
        let widget: Box<dyn View> = Box::new(TestWidget::new().with_id("boxed"));
        assert_eq!(widget.id(), Some("boxed"));
    }

    #[test]
    fn test_boxed_view_classes() {
        let widget: Box<dyn View> = Box::new(TestWidget::new().with_class("test"));
        assert_eq!(widget.classes().len(), 1);
        assert_eq!(widget.classes()[0], "test");
    }

    #[test]
    fn test_boxed_view_children() {
        let widget: Box<dyn View> = Box::new(TestWidget::new());
        assert!(widget.children().is_empty());
    }

    #[test]
    fn test_boxed_view_meta() {
        let widget: Box<dyn View> =
            Box::new(TestWidget::new().with_id("box-id").with_class("box-class"));
        let meta = widget.meta();
        assert_eq!(meta.id, Some("box-id".to_string()));
        assert!(meta.classes.contains("box-class"));
    }

    #[test]
    fn test_boxed_view_render() {
        let widget: Box<dyn View> = Box::new(TestWidget::new());
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        widget.render(&mut ctx);
    }

    // =========================================================================
    // Interactive Trait Tests
    // =========================================================================

    struct TestInteractive {
        focused: bool,
        focusable: bool,
    }

    impl TestInteractive {
        fn new() -> Self {
            Self {
                focused: false,
                focusable: true,
            }
        }

        fn non_focusable() -> Self {
            Self {
                focused: false,
                focusable: false,
            }
        }
    }

    impl View for TestInteractive {
        fn render(&self, _ctx: &mut RenderContext) {}
    }

    impl Interactive for TestInteractive {
        fn focusable(&self) -> bool {
            self.focusable
        }

        fn on_focus(&mut self) {
            self.focused = true;
        }

        fn on_blur(&mut self) {
            self.focused = false;
        }
    }

    #[test]
    fn test_interactive_handle_key_default() {
        use crate::event::Key;
        let mut widget = TestInteractive::new();
        let event = KeyEvent::new(Key::Enter);
        let result = widget.handle_key(&event);
        assert_eq!(result, EventResult::Ignored);
    }

    #[test]
    fn test_interactive_handle_mouse_default() {
        use crate::event::MouseEventKind;
        let mut widget = TestInteractive::new();
        let event = MouseEvent::new(5, 5, MouseEventKind::Move);
        let area = Rect::new(0, 0, 10, 5);
        let result = widget.handle_mouse(&event, area);
        assert_eq!(result, EventResult::Ignored);
    }

    #[test]
    fn test_interactive_focusable_true() {
        let widget = TestInteractive::new();
        assert!(widget.focusable());
    }

    #[test]
    fn test_interactive_focusable_false() {
        let widget = TestInteractive::non_focusable();
        assert!(!widget.focusable());
    }

    #[test]
    fn test_interactive_on_focus() {
        let mut widget = TestInteractive::new();
        assert!(!widget.focused);
        widget.on_focus();
        assert!(widget.focused);
    }

    #[test]
    fn test_interactive_on_blur() {
        let mut widget = TestInteractive::new();
        widget.on_focus();
        assert!(widget.focused);
        widget.on_blur();
        assert!(!widget.focused);
    }

    // =========================================================================
    // Draggable Trait Tests
    // =========================================================================

    struct TestDraggable {
        can_drag: bool,
        can_drop: bool,
        accepted_types: Vec<&'static str>,
    }

    impl TestDraggable {
        fn new() -> Self {
            Self {
                can_drag: false,
                can_drop: false,
                accepted_types: Vec::new(),
            }
        }

        fn draggable() -> Self {
            Self {
                can_drag: true,
                can_drop: false,
                accepted_types: Vec::new(),
            }
        }

        fn droppable(types: Vec<&'static str>) -> Self {
            Self {
                can_drag: false,
                can_drop: true,
                accepted_types: types,
            }
        }
    }

    impl View for TestDraggable {
        fn render(&self, _ctx: &mut RenderContext) {}
    }

    impl Draggable for TestDraggable {
        fn can_drag(&self) -> bool {
            self.can_drag
        }

        fn can_drop(&self) -> bool {
            self.can_drop
        }

        fn accepted_types(&self) -> &[&'static str] {
            &self.accepted_types
        }
    }

    #[test]
    fn test_draggable_can_drag_default() {
        let widget = TestDraggable::new();
        assert!(!widget.can_drag());
    }

    #[test]
    fn test_draggable_can_drag_enabled() {
        let widget = TestDraggable::draggable();
        assert!(widget.can_drag());
    }

    #[test]
    fn test_draggable_drag_data_default() {
        let widget = TestDraggable::new();
        assert!(widget.drag_data().is_none());
    }

    #[test]
    fn test_draggable_drag_preview_default() {
        let widget = TestDraggable::new();
        assert!(widget.drag_preview().is_none());
    }

    #[test]
    fn test_draggable_on_drag_start() {
        let mut widget = TestDraggable::draggable();
        widget.on_drag_start();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draggable_on_drag_end() {
        let mut widget = TestDraggable::draggable();
        widget.on_drag_end(DropResult::Accepted);
        widget.on_drag_end(DropResult::Cancelled);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draggable_can_drop_default() {
        let widget = TestDraggable::new();
        assert!(!widget.can_drop());
    }

    #[test]
    fn test_draggable_can_drop_enabled() {
        let widget = TestDraggable::droppable(vec!["text"]);
        assert!(widget.can_drop());
    }

    #[test]
    fn test_draggable_accepted_types_empty() {
        let widget = TestDraggable::new();
        assert!(widget.accepted_types().is_empty());
    }

    #[test]
    fn test_draggable_accepted_types_with_values() {
        let widget = TestDraggable::droppable(vec!["text", "file"]);
        let types = widget.accepted_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"text"));
        assert!(types.contains(&"file"));
    }

    #[test]
    fn test_draggable_can_accept_empty_types() {
        let widget = TestDraggable::droppable(vec![]);
        let data = DragData::text("anything");
        // Empty types means accept all
        assert!(widget.can_accept(&data));
    }

    #[test]
    fn test_draggable_can_accept_matching_type() {
        let widget = TestDraggable::droppable(vec!["text", "file"]);
        let data = DragData::text("test-text");
        assert!(widget.can_accept(&data));
    }

    #[test]
    fn test_draggable_can_accept_non_matching_type() {
        let widget = TestDraggable::droppable(vec!["text", "file"]);
        let data = DragData::new("image", "image-data");
        assert!(!widget.can_accept(&data));
    }

    #[test]
    fn test_draggable_on_drag_enter() {
        let mut widget = TestDraggable::droppable(vec!["text"]);
        let data = DragData::text("test-text");
        widget.on_drag_enter(&data);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draggable_on_drag_leave() {
        let mut widget = TestDraggable::droppable(vec!["text"]);
        widget.on_drag_leave();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draggable_on_drop_default() {
        let mut widget = TestDraggable::droppable(vec!["text"]);
        let data = DragData::text("test-text");
        let accepted = widget.on_drop(data);
        assert!(!accepted);
    }

    #[test]
    fn test_draggable_drop_bounds() {
        let widget = TestDraggable::new();
        let area = Rect::new(10, 20, 30, 40);
        let bounds = widget.drop_bounds(area);
        assert_eq!(bounds, area);
    }
}
