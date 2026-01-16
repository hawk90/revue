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
