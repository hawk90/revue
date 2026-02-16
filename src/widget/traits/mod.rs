//! Widget traits and common types for Revue components
//!
//! This module defines the core traits that all widgets implement, along with
//! supporting types for rendering, events, and state management.
//!
//! # Core Traits
//!
//! | Trait | Description | Use Case |
//! |-------|-------------|----------|
//! | [`View`] | Core rendering trait | All renderable widgets |
//! | [`StyledView`] | View with CSS styling | Styled widgets |
//! | [`Interactive`] | Handle keyboard/mouse | Interactive widgets |
//! | [`Draggable`] | Drag and drop support | Draggable widgets |
//!
//! # View Trait
//!
//! The [`View`] trait is the foundation of all widgets:
//!
//! ```rust,ignore
//! use revue::widget::View;
//!
//! pub struct MyWidget;
//!
//! impl View for MyWidget {
//!     fn render(&self, ctx: &mut RenderContext) {
//!         // Render widget to context
//!         Text::new("Hello").render(ctx);
//!     }
//! }
//! ```
//!
//! # StyledView Trait
//!
//! [`StyledView`] extends View with CSS styling support:
//!
//! ```rust,ignore
//! use revue::widget::StyledView;
//!
//! impl StyledView for MyWidget {
//!     // Inherit CSS styling support
//!     fn style(&self) -> Style {
//!         Style::default()
//!     }
//! }
//! ```
//!
//! # Interactive Trait
//!
//! [`Interactive`] enables keyboard and mouse event handling:
//!
//! ```rust,ignore
//! use revue::widget::Interactive;
//!
//! impl Interactive for MyWidget {
//!     fn handle_key(&mut self, key: &KeyEvent) -> EventResult {
//!         match key.key {
//!             Key::Enter => EventResult::Consumed,
//!             _ => EventResult::Ignored,
//!         }
//!     }
//! }
//! ```
//!
//! # Common Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Element`] | Widget element container |
//! | [`RenderContext`] | Rendering context and utilities |
//! | [`WidgetState`] | Common widget state (focus, disabled, colors) |
//! | [`EventResult`] | Event handling result |
//! | [`FocusStyle`] | Focus indicator style |
//!
//! # Builder Macros
//!
//! The `impl_state_builders!` macro generates builder methods for widgets:
//!
//! ```rust,ignore
//! struct MyWidget {
//!     state: WidgetState,
//! }
//!
//! impl_state_builders!(MyWidget);
//!
//! // Now available:
//! let widget = MyWidget { state: WidgetState::default() }
//!     .focused(true)
//!     .disabled(false)
//!     .fg(Color::Blue)
//!     .bg(Color::Black);
//! ```

mod element;
mod event;
mod render_context;
mod symbols;
mod timeout;
mod view;
mod widget_state;

// Re-export all public types
pub use element::Element;
pub use event::{EventResult, FocusStyle};
pub use render_context::{ProgressBarConfig, RenderContext};
pub use symbols::Symbols;
pub use timeout::Timeout;
pub use view::{Draggable, Interactive, StyledView, View};
pub use widget_state::{WidgetProps, WidgetState, DISABLED_BG, DISABLED_FG};

// =============================================================================
// Builder Macros
// =============================================================================

/// Generate builder methods for widgets with `state: WidgetState` field.
///
/// This macro generates the following methods:
/// - `focused(self, bool) -> Self` - Set focused state
/// - `disabled(self, bool) -> Self` - Set disabled state
/// - `fg(self, Color) -> Self` - Set foreground color
/// - `bg(self, Color) -> Self` - Set background color
/// - `is_focused(&self) -> bool` - Check if focused
/// - `is_disabled(&self) -> bool` - Check if disabled
/// - `set_focused(&mut self, bool)` - Mutably set focused state
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     state: WidgetState,
///     props: WidgetProps,
/// }
///
/// impl_state_builders!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_state_builders {
    ($widget:ty) => {
        impl $widget {
            /// Set focused state
            pub fn focused(mut self, focused: bool) -> Self {
                self.state.focused = focused;
                self
            }

            /// Set disabled state
            pub fn disabled(mut self, disabled: bool) -> Self {
                self.state.disabled = disabled;
                self
            }

            /// Set foreground color
            pub fn fg(mut self, color: $crate::style::Color) -> Self {
                self.state.fg = Some(color);
                self
            }

            /// Set background color
            pub fn bg(mut self, color: $crate::style::Color) -> Self {
                self.state.bg = Some(color);
                self
            }

            /// Check if widget is focused
            pub fn is_focused(&self) -> bool {
                self.state.focused
            }

            /// Check if widget is disabled
            pub fn is_disabled(&self) -> bool {
                self.state.disabled
            }

            /// Set focused state (mutable)
            pub fn set_focused(&mut self, focused: bool) {
                self.state.focused = focused;
            }
        }
    };
}

/// Generate builder methods for widgets with `props: WidgetProps` field.
///
/// This macro generates the following methods:
/// - `element_id(self, impl Into<String>) -> Self` - Set CSS element ID
/// - `class(self, impl Into<String>) -> Self` - Add a CSS class
/// - `classes(self, IntoIterator<Item=S>) -> Self` - Add multiple CSS classes
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     props: WidgetProps,
/// }
///
/// impl_props_builders!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_props_builders {
    ($widget:ty) => {
        impl $widget {
            /// Set element ID for CSS selector (#id)
            pub fn element_id(mut self, id: impl Into<String>) -> Self {
                self.props.id = Some(id.into());
                self
            }

            /// Add a CSS class
            pub fn class(mut self, class: impl Into<String>) -> Self {
                let class_str = class.into();
                if !self.props.classes.contains(&class_str) {
                    self.props.classes.push(class_str);
                }
                self
            }

            /// Add multiple CSS classes
            pub fn classes<I, S>(mut self, classes: I) -> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>,
            {
                for class in classes {
                    let class_str = class.into();
                    if !self.props.classes.contains(&class_str) {
                        self.props.classes.push(class_str);
                    }
                }
                self
            }
        }
    };
}

/// Generate all common builder methods for widgets with both `state: WidgetState`
/// and `props: WidgetProps` fields.
///
/// This is a convenience macro that combines `impl_state_builders!` and
/// `impl_props_builders!`.
///
/// Generated methods:
/// - State: `focused`, `disabled`, `fg`, `bg`, `is_focused`, `is_disabled`, `set_focused`
/// - Props: `element_id`, `class`, `classes`
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     label: String,
///     state: WidgetState,
///     props: WidgetProps,
/// }
///
/// impl MyWidget {
///     pub fn new(label: impl Into<String>) -> Self {
///         Self {
///             label: label.into(),
///             state: WidgetState::new(),
///             props: WidgetProps::new(),
///         }
///     }
/// }
///
/// // Generates: focused, disabled, fg, bg, is_focused, is_disabled,
/// //            set_focused, element_id, class, classes
/// impl_widget_builders!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_widget_builders {
    ($widget:ty) => {
        $crate::impl_state_builders!($widget);
        $crate::impl_props_builders!($widget);
    };
}

/// Generate View trait id(), classes(), and meta() methods for widgets with props.
///
/// This macro generates the id(), classes(), and meta() methods for the View trait
/// that delegate to WidgetProps.
///
/// # Example
/// ```rust,ignore
/// impl View for MyWidget {
///     fn render(&self, ctx: &mut RenderContext) {
///         // ... rendering logic
///     }
///
///     crate::impl_view_meta!("MyWidget");
/// }
/// ```
#[macro_export]
macro_rules! impl_view_meta {
    ($name:expr) => {
        fn id(&self) -> Option<&str> {
            self.props.id.as_deref()
        }

        fn classes(&self) -> &[String] {
            &self.props.classes
        }

        fn meta(&self) -> $crate::dom::WidgetMeta {
            let mut meta = $crate::dom::WidgetMeta::new($name);
            if let Some(ref id) = self.props.id {
                meta.id = Some(id.clone());
            }
            for class in &self.props.classes {
                meta.classes.insert(class.clone());
            }
            meta
        }
    };
}

/// Generate View trait implementation for StyledView widgets.
///
/// This macro generates View trait methods that delegate to WidgetProps
/// for id() and classes() methods.
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     props: WidgetProps,
/// }
///
/// impl View for MyWidget {
///     fn render(&self, ctx: &mut RenderContext) {
///         // ... rendering logic
///     }
/// }
///
/// impl_styled_view!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_styled_view {
    ($widget:ty) => {
        impl $crate::widget::traits::StyledView for $widget {
            fn set_id(&mut self, id: impl Into<String>) {
                self.props.id = Some(id.into());
            }

            fn add_class(&mut self, class: impl Into<String>) {
                let class_str = class.into();
                if !self.props.classes.contains(&class_str) {
                    self.props.classes.push(class_str);
                }
            }

            fn remove_class(&mut self, class: &str) {
                self.props.classes.retain(|c| c != class);
            }

            fn toggle_class(&mut self, class: &str) {
                if self.props.classes.contains(&class.to_string()) {
                    self.remove_class(class);
                } else {
                    self.add_class(class);
                }
            }

            fn has_class(&self, class: &str) -> bool {
                self.props.classes.contains(&class.to_string())
            }
        }
    };
}
