//! Widget traits and common types

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;

    #[test]
    fn test_event_result_default() {
        let result = EventResult::default();
        assert!(!result.is_consumed());
        assert!(!result.needs_render());
    }

    #[test]
    fn test_event_result_consumed() {
        let consumed = EventResult::Consumed;
        assert!(consumed.is_consumed());
        assert!(!consumed.needs_render());
    }

    #[test]
    fn test_event_result_consumed_and_render() {
        let result = EventResult::ConsumedAndRender;
        assert!(result.is_consumed());
        assert!(result.needs_render());
    }

    #[test]
    fn test_event_result_from_bool() {
        let handled: EventResult = true.into();
        assert_eq!(handled, EventResult::ConsumedAndRender);

        let ignored: EventResult = false.into();
        assert_eq!(ignored, EventResult::Ignored);
    }

    #[test]
    fn test_event_result_or() {
        assert_eq!(
            EventResult::Ignored.or(EventResult::ConsumedAndRender),
            EventResult::ConsumedAndRender
        );
        assert_eq!(
            EventResult::ConsumedAndRender.or(EventResult::Ignored),
            EventResult::ConsumedAndRender
        );
        assert_eq!(
            EventResult::Ignored.or(EventResult::Consumed),
            EventResult::Consumed
        );
        assert_eq!(
            EventResult::Ignored.or(EventResult::Ignored),
            EventResult::Ignored
        );
    }

    #[test]
    fn test_widget_state_new() {
        let state = WidgetState::new();
        assert!(!state.is_focused());
        assert!(!state.is_disabled());
        assert!(!state.is_pressed());
        assert!(!state.is_hovered());
        assert!(!state.is_interactive());
    }

    #[test]
    fn test_widget_state_builder() {
        let state = WidgetState::new()
            .focused(true)
            .disabled(false)
            .fg(Color::RED)
            .bg(Color::BLUE);

        assert!(state.is_focused());
        assert!(!state.is_disabled());
        assert_eq!(state.fg, Some(Color::RED));
        assert_eq!(state.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_widget_state_effective_colors() {
        let default_color = Color::rgb(128, 128, 128);

        let normal = WidgetState::new().fg(Color::WHITE);
        assert_eq!(normal.effective_fg(default_color), Color::WHITE);

        let disabled = WidgetState::new().fg(Color::WHITE).disabled(true);
        assert_eq!(disabled.effective_fg(default_color), DISABLED_FG);
    }

    #[test]
    fn test_widget_state_reset_transient() {
        let mut state = WidgetState::new()
            .focused(true)
            .disabled(false)
            .pressed(true)
            .hovered(true);

        state.reset_transient();

        assert!(state.focused);
        assert!(!state.disabled);
        assert!(!state.pressed);
        assert!(!state.hovered);
    }

    #[test]
    fn test_widget_classes_exposure() {
        use crate::widget::Text;

        let widget = Text::new("Test").class("btn").class("primary");

        let classes = View::classes(&widget);
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"btn".to_string()));
        assert!(classes.contains(&"primary".to_string()));

        let meta = widget.meta();
        assert!(meta.classes.contains("btn"));
        assert!(meta.classes.contains("primary"));
    }

    // Wide character tests
    #[test]
    fn test_draw_text_wide_chars() {
        let mut buf = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buf, area);

        ctx.draw_text(0, 0, "한글", Color::WHITE);

        assert_eq!(buf.get(0, 0).unwrap().symbol, '한');
        assert!(buf.get(1, 0).unwrap().is_continuation());
        assert_eq!(buf.get(2, 0).unwrap().symbol, '글');
        assert!(buf.get(3, 0).unwrap().is_continuation());
        assert_eq!(buf.get(4, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_draw_text_mixed_width() {
        let mut buf = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buf, area);

        ctx.draw_text(0, 0, "A한B", Color::WHITE);

        assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buf.get(1, 0).unwrap().symbol, '한');
        assert!(buf.get(2, 0).unwrap().is_continuation());
        assert_eq!(buf.get(3, 0).unwrap().symbol, 'B');
    }

    #[test]
    fn test_draw_text_centered_wide_chars() {
        let mut buf = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buf, area);

        ctx.draw_text_centered(0, 0, 10, "한글", Color::WHITE);

        assert_eq!(buf.get(3, 0).unwrap().symbol, '한');
        assert!(buf.get(4, 0).unwrap().is_continuation());
        assert_eq!(buf.get(5, 0).unwrap().symbol, '글');
        assert!(buf.get(6, 0).unwrap().is_continuation());
    }

    #[test]
    fn test_draw_text_right_wide_chars() {
        let mut buf = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buf, area);

        ctx.draw_text_right(0, 0, 10, "한글", Color::WHITE);

        assert_eq!(buf.get(6, 0).unwrap().symbol, '한');
        assert!(buf.get(7, 0).unwrap().is_continuation());
        assert_eq!(buf.get(8, 0).unwrap().symbol, '글');
        assert!(buf.get(9, 0).unwrap().is_continuation());
    }
}
