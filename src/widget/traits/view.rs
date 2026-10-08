//! The View trait every widget implements, and Fill

use crate::dom::{WidgetKey, WidgetMeta};
use crate::style::Style;

use super::render_context::{RenderContext, StyledSubtree};

/// The axes along which a view takes whatever space it is offered - see
/// [`View::fills`].
///
/// ```
/// use revue::widget::Fill;
///
/// assert!(Fill::WIDTH.width && !Fill::WIDTH.height);
/// assert_eq!(Fill::new(true, true), Fill::BOTH);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Fill {
    /// Takes all the width it is offered
    pub width: bool,
    /// Takes all the height it is offered
    pub height: bool,
}

impl Fill {
    /// Fills neither axis: the view is the size it measures
    pub const NONE: Self = Self::new(false, false);
    /// Fills the width it is offered (a text field, a progress bar)
    pub const WIDTH: Self = Self::new(true, false);
    /// Fills the height it is offered (a vertical divider)
    pub const HEIGHT: Self = Self::new(false, true);
    /// Fills both axes
    pub const BOTH: Self = Self::new(true, true);

    /// Fill `width`, `height`, or both
    pub const fn new(width: bool, height: bool) -> Self {
        Self { width, height }
    }

    /// Fills each axis that either fills
    pub const fn or(self, other: Self) -> Self {
        Self::new(self.width || other.width, self.height || other.height)
    }
}

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

    /// The size this view wants, given at most `max_width` x `max_height`.
    ///
    /// `None` - the default - means "I fill whatever I am given", which is
    /// what every view did before this existed. A view whose size follows from
    /// its content answers with it, so a container that sizes children to
    /// their content (`Stack::content_sized`) can give it exactly that.
    ///
    /// Answer with what the view would *paint*, clamped to the maximum: a
    /// one-line `Text` is one row tall and as wide as its text.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let _ = (max_width, max_height);
        None
    }

    /// [`measure`](Self::measure), with this frame's computed styles for the
    /// view's own subtree in hand.
    ///
    /// A crate-internal hook: a content-sized stack calls it for a child it is
    /// about to lay out under `css_layout`, so that CSS spacing *inside* the
    /// child - a nested stack's `gap`, its children's margins - is part of
    /// the size the child gets. `measure` alone cannot see the stylesheet.
    ///
    /// The default cannot account for that spacing, so it does not guess: when
    /// any descendant's style adds space, the view fills (`None`) rather than
    /// be handed a slot its content is clipped to. Otherwise it is `measure`.
    /// `Stack` answers exactly instead.
    #[doc(hidden)]
    fn measure_styled(
        &self,
        max_width: u16,
        max_height: u16,
        subtree: StyledSubtree<'_>,
    ) -> Option<(u16, u16)> {
        if subtree.descendants_add_space() {
            None
        } else {
            self.measure(max_width, max_height)
        }
    }

    /// The axes along which this view takes whatever space it is offered;
    /// [`measure`](Self::measure) still gives its natural size on the other
    /// axis.
    ///
    /// A text field is one row tall but as wide as it is given: it answers
    /// [`Fill::WIDTH`], and `measure` answers one row. A content-sized stack
    /// (`Stack::content_sized`) treats a child that fills its main axis like
    /// one that does not measure (it shares what the measured children leave)
    /// and sizes it by `measure` otherwise. The default, [`Fill::NONE`],
    /// leaves `measure` in charge of both axes.
    fn fills(&self) -> Fill {
        Fill::NONE
    }

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

    /// Check if this widget needs re-rendering
    ///
    /// Returns `true` by default (always re-render). Widgets can override
    /// this to skip rendering when their state hasn't changed, improving
    /// performance for complex UIs.
    ///
    /// Container widgets use this to skip rendering unchanged children.
    ///
    /// # Example
    ///
    /// ```ignore
    /// struct CachedWidget {
    ///     dirty: bool,
    ///     content: String,
    /// }
    ///
    /// impl View for CachedWidget {
    ///     fn needs_render(&self) -> bool {
    ///         self.dirty
    ///     }
    ///     fn render(&self, ctx: &mut RenderContext) {
    ///         // Only called when needs_render() returns true
    ///         ctx.draw_text(0, 0, &self.content, Color::WHITE);
    ///     }
    /// }
    /// ```
    fn needs_render(&self) -> bool {
        true
    }

    /// Reconciliation key - this widget's identity across frames
    ///
    /// Returns `None` by default, which means identity is positional: the
    /// widget is reconciled against whatever node sat at the same index last
    /// frame. That is correct for a fixed layout and wrong for a dynamic
    /// collection - insert a row at the top of a list and every row below it
    /// matches the wrong node, so focus, selection and scroll offset all shift
    /// by one.
    ///
    /// Give the key the identity of the *data*, never the loop index.
    ///
    /// # Example
    ///
    /// ```ignore
    /// impl View for TodoRow {
    ///     fn key(&self) -> Option<WidgetKey> {
    ///         Some(WidgetKey::from(self.todo.id))
    ///     }
    /// }
    /// ```
    fn key(&self) -> Option<WidgetKey> {
        None
    }

    /// Inline style - the widget's own style, applied after every stylesheet
    /// rule, as an HTML `style` attribute is.
    ///
    /// Returns `None` by default. Widgets that keep a
    /// [`WidgetProps`](crate::widget::WidgetProps) and use
    /// [`impl_view_meta!`](crate::impl_view_meta) return what
    /// [`WidgetProps::style`](crate::widget::WidgetProps::style) set. The
    /// renderer copies it onto the widget's DOM node each frame.
    fn inline_style(&self) -> Option<Style> {
        None
    }

    /// Get widget metadata for DOM
    ///
    /// This method combines `widget_type()`, `id()`, `classes()` and `key()`
    /// into a `WidgetMeta` struct used by the DOM builder. You typically don't
    /// need to override this.
    fn meta(&self) -> WidgetMeta {
        let mut meta = WidgetMeta::new(self.widget_type());
        if let Some(id) = self.id() {
            meta.id = Some(id.to_string());
        }
        for class in self.classes() {
            meta.classes.insert(class.clone());
        }
        meta.key = self.key();
        meta
    }
}

/// Implement View for `Box<dyn View>` to allow boxed views to be used as children
impl View for Box<dyn View> {
    fn render(&self, ctx: &mut RenderContext) {
        (**self).render(ctx);
    }

    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        (**self).measure(max_width, max_height)
    }

    fn measure_styled(
        &self,
        max_width: u16,
        max_height: u16,
        subtree: StyledSubtree<'_>,
    ) -> Option<(u16, u16)> {
        (**self).measure_styled(max_width, max_height, subtree)
    }

    fn fills(&self) -> Fill {
        (**self).fills()
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

    fn needs_render(&self) -> bool {
        (**self).needs_render()
    }

    fn key(&self) -> Option<WidgetKey> {
        (**self).key()
    }

    fn inline_style(&self) -> Option<Style> {
        (**self).inline_style()
    }

    fn meta(&self) -> WidgetMeta {
        (**self).meta()
    }
}
