//! View and related traits for widgets

use crate::dom::WidgetMeta;
use crate::event::drag::{DragData, DropResult};
use crate::event::{KeyEvent, MouseEvent};
use crate::layout::Rect;

use super::event::EventResult;
use super::render_context::RenderContext;

/// The core trait for all renderable components
///
/// Every widget in Revue implements the `View` trait, which provides:
/// - Rendering via [`render()`][Self::render]
/// - CSS selector support via [`id()`][Self::id], [`classes()`][Self::classes], and [`widget_type()`][Self::widget_type]
/// - Child exposure for container widgets via [`children()`][Self::children]
/// - Metadata generation via [`meta()`][Self::meta]
///
/// # Implementing View
///
/// At minimum, you only need to implement `render`:
///
/// ```ignore
/// use revue::prelude::*;
///
/// struct MyWidget {
///     text: String,
/// }
///
/// impl View for MyWidget {
///     fn render(&self, ctx: &mut RenderContext) {
///         ctx.draw_text(0, 0, &self.text, Color::WHITE);
///     }
/// }
/// ```
///
/// # CSS Selector Support
///
/// Widgets can be styled via CSS using three selector types:
///
/// 1. **Type selector** - All widgets of a given type
///    ```css
///    MyWidget { color: red; }
///    ```
///
/// 2. **ID selector** - A specific widget (unique identifier)
///    ```ignore
///    widget.id("my-special-widget");
///    ```
///    ```css
///    #my-special-widget { color: blue; }
///    ```
///
/// 3. **Class selector** - Widgets with a specific class
///    ```ignore
///    widget.class("primary").class("active");
///    ```
///    ```css
///    .primary { color: green; }
///    .active { font-weight: bold; }
///    ```
///
/// # Container Widgets
///
/// Container widgets (like `Stack`, `Grid`) should override [`children()`][Self::children]
/// to expose their children for DOM traversal:
///
/// ```ignore
/// impl View for MyContainer {
///     fn children(&self) -> &[Box<dyn View>] {
///         &self.children
///     }
///     // ...
/// }
/// ```
///
/// # View Trait Object
///
/// `View` can be used as a trait object (`Box<dyn View>`) for dynamic polymorphism:
///
/// ```ignore
/// fn render_any_widget(widget: Box<dyn View>, ctx: &mut RenderContext) {
///     widget.render(ctx);
/// }
/// ```
pub trait View {
    /// Render the view to the given context
    ///
    /// The `RenderContext` provides drawing primitives like:
    /// - `draw_text()` - Render text at a position
    /// - `fill()` - Fill a region with a character/color
    /// - `draw_border()` - Draw borders
    /// - `clear()` - Clear a region
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn render(&self, ctx: &mut RenderContext) {
    ///     ctx.clear(ctx.area());
    ///     ctx.draw_text(0, 0, &self.label, Color::WHITE);
    /// }
    /// ```
    fn render(&self, ctx: &mut RenderContext);

    /// Get widget type name (for CSS type selectors)
    ///
    /// The default implementation extracts the type name from the Rust type.
    /// For example, `MyApp::MyWidget` becomes `"MyWidget"`.
    ///
    /// Override this if you want a custom type name for CSS matching:
    ///
    /// ```ignore
    /// fn widget_type(&self) -> &'static str {
    ///     "button"  // Always match as "button" regardless of Rust type
    /// }
    /// ```
    fn widget_type(&self) -> &'static str {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap_or("Unknown")
    }

    /// Get element ID (for CSS #id selectors)
    ///
    /// Returns `None` by default. Override to provide a unique ID:
    ///
    /// ```ignore
    /// struct MyWidget {
    ///     id: Option<String>,
    /// }
    ///
    /// impl View for MyWidget {
    ///     fn id(&self) -> Option<&str> {
    ///         self.id.as_deref()
    ///     }
    /// }
    /// ```
    fn id(&self) -> Option<&str> {
        None
    }

    /// Get CSS classes (for CSS .class selectors)
    ///
    /// Returns an empty slice by default. Override to provide classes:
    ///
    /// ```ignore
    /// struct MyWidget {
    ///     classes: Vec<String>,
    /// }
    ///
    /// impl View for MyWidget {
    ///     fn classes(&self) -> &[String] {
    ///         &self.classes
    ///     }
    /// }
    /// ```
    fn classes(&self) -> &[String] {
        &[]
    }

    /// Get child views (for container widgets)
    ///
    /// Container widgets (Stack, Grid, etc.) should override this to expose
    /// their children, enabling the DOM builder to traverse the full widget tree.
    ///
    /// The returned slice should contain **boxed trait objects** to enable
    /// heterogeneous child collections.
    ///
    /// # Example
    ///
    /// ```ignore
    /// struct MyContainer {
    ///     children: Vec<Box<dyn View>>,
    /// }
    ///
    /// impl View for MyContainer {
    ///     fn children(&self) -> &[Box<dyn View>] {
    ///         &self.children
    ///     }
    /// }
    /// ```
    fn children(&self) -> &[Box<dyn View>] {
        &[]
    }

    /// Get widget metadata for DOM
    ///
    /// This method combines `widget_type()`, `id()`, and `classes()` into
    /// a `WidgetMeta` struct used by the DOM builder. You typically don't need
    /// to override this.
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
/// This trait extends [`View`] with keyboard and mouse handling capabilities.
/// Widgets that need to respond to user input should implement this trait.
///
/// # Implementing Interactive
///
/// ```ignore
/// use revue::prelude::*;
///
/// struct MyButton {
///     label: String,
///     focused: bool,
/// }
///
/// impl View for MyButton {
///     fn render(&self, ctx: &mut RenderContext) {
///         // Render button with focus indication
///     }
/// }
///
/// impl Interactive for MyButton {
///     fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
///         match event.key {
///             Key::Enter => {
///                 // Handle button click
///                 EventResult::ConsumedAndRender
///             }
///             _ => EventResult::Ignored,
///         }
///     }
///
///     fn focusable(&self) -> bool {
///         true
///     }
///
///     fn on_focus(&mut self) {
///         self.focused = true;
///     }
///
///     fn on_blur(&mut self) {
///         self.focused = false;
///     }
/// }
/// ```
///
/// # Event Result Types
///
/// - `EventResult::Ignored` - Event not handled, propagate to parent
/// - `EventResult::Consumed` - Event handled, no redraw needed
/// - `EventResult::ConsumedAndRender` - Event handled, redraw needed
pub trait Interactive: View {
    /// Handle keyboard event
    ///
    /// Returns `EventResult::Consumed` or `EventResult::ConsumedAndRender` if
    /// the event was handled, `EventResult::Ignored` to let it propagate.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
    ///     match event.key {
    ///         Key::Char('q') => EventResult::Consumed,
    ///         Key::Enter => EventResult::ConsumedAndRender,
    ///         _ => EventResult::Ignored,
    ///     }
    /// }
    /// ```
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        let _ = event;
        EventResult::Ignored
    }

    /// Handle mouse event
    ///
    /// Returns `EventResult` indicating if event was handled.
    /// The `area` parameter is the widget's layout bounds.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
    ///     if !area.contains(event.x, event.y) {
    ///         return EventResult::Ignored;
    ///     }
    ///     match event.kind {
    ///         MouseEventKind::Down(_) => EventResult::ConsumedAndRender,
    ///         _ => EventResult::Ignored,
    ///     }
    /// }
    /// ```
    fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
        let _ = (event, area);
        EventResult::Ignored
    }

    /// Check if the widget can receive focus
    ///
    /// Return `false` for widgets that shouldn't be focusable (e.g., labels).
    fn focusable(&self) -> bool {
        true
    }

    /// Called when the widget receives focus
    ///
    /// Use this to update visual state or perform setup.
    fn on_focus(&mut self) {}

    /// Called when the widget loses focus
    ///
    /// Use this to clean up or update visual state.
    fn on_blur(&mut self) {}
}

/// Trait for widgets that support drag-and-drop
///
/// This trait enables widgets to participate in drag-and-drop operations.
/// Widgets can be drag sources, drop targets, or both.
///
/// # Implementing Draggable
///
/// ## As a Drag Source
///
/// ```ignore
/// impl Draggable for MyWidget {
///     fn can_drag(&self) -> bool {
///         !self.is_empty
///     }
///
///     fn drag_data(&self) -> Option<DragData> {
///         Some(DragData::text(self.content.clone()))
///     }
///
///     fn drag_preview(&self) -> Option<String> {
///         Some(format!("Move: {}", self.label))
///     }
/// }
/// ```
///
/// ## As a Drop Target
///
/// ```ignore
/// impl Draggable for MyDropZone {
///     fn can_drop(&self) -> bool {
///         true
///     }
///
///     fn accepted_types(&self) -> &[&str] {
///         &["text", "file"]
///     }
///
///     fn on_drop(&mut self, data: DragData) -> bool {
///         match data.type_id {
///             "text" => {
///                 self.handle_text_drop(data.content);
///                 true
///             }
///             _ => false,
///         }
///     }
/// }
/// ```
pub trait Draggable: View {
    /// Check if this widget can be dragged
    ///
    /// Return `true` to allow drag operations on this widget.
    fn can_drag(&self) -> bool {
        false
    }

    /// Get the drag data when a drag starts
    ///
    /// Return `None` to cancel the drag.
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
    ///
    /// Return an empty slice to accept all types.
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
    /// Return `true` if the drop was accepted, `false` to reject.
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
///
/// This trait provides runtime mutable access to CSS styling properties.
/// Unlike the base `View` trait which returns immutable references,
/// `StyledView` allows modifying IDs and classes after widget creation.
///
/// # Example
///
/// ```ignore
/// struct MyWidget {
///     id: String,
///     classes: Vec<String>,
/// }
///
/// impl StyledView for MyWidget {
///     fn set_id(&mut self, id: impl Into<String>) {
///         self.id = id.into();
///     }
///
///     fn add_class(&mut self, class: impl Into<String>) {
///         self.classes.push(class.into());
///     }
///
///     fn remove_class(&mut self, class: &str) {
///         self.classes.retain(|c| c != class);
///     }
///
///     fn toggle_class(&mut self, class: &str) {
///         if self.has_class(class) {
///             self.remove_class(class);
///         } else {
///             self.add_class(class);
///         }
///     }
///
///     fn has_class(&self, class: &str) -> bool {
///         self.classes.iter().any(|c| c == class)
///     }
/// }
/// ```
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
