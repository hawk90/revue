//! Keyboard and mouse handling for widgets, and the shared toggle behavior

use crate::event::{KeyEvent, MouseEvent};
use crate::layout::Rect;

use super::event::EventResult;
use super::view::View;

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

/// Trait for boolean toggle widgets (checkbox, switch, etc.)
///
/// Provides default implementations for toggling, key handling, and focus
/// management. Implementors supply accessors for their `on`/`checked` and
/// `disabled`/`focused` fields; the trait supplies the shared behavior.
///
/// # Example
///
/// ```ignore
/// impl ToggleWidget for MyToggle {
///     fn is_on(&self) -> bool { self.checked }
///     fn set_on(&mut self, on: bool) { self.checked = on; }
///     fn is_toggle_disabled(&self) -> bool { self.disabled }
///     fn is_toggle_focused(&self) -> bool { self.focused }
///     fn set_toggle_focused(&mut self, focused: bool) { self.focused = focused; }
/// }
/// ```
pub trait ToggleWidget {
    /// Whether the toggle is currently on/checked
    fn is_on(&self) -> bool;
    /// Set the on/checked state directly
    fn set_on(&mut self, on: bool);
    /// Whether the widget is disabled (cannot be toggled)
    fn is_toggle_disabled(&self) -> bool;
    /// Whether the widget currently has focus
    fn is_toggle_focused(&self) -> bool;
    /// Set focus state
    fn set_toggle_focused(&mut self, focused: bool);

    /// Flip the toggle if not disabled
    fn toggle(&mut self) {
        if !self.is_toggle_disabled() {
            self.set_on(!self.is_on());
        }
    }

    /// Handle Enter/Space key to toggle. Returns `EventResult`.
    fn handle_toggle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.is_toggle_disabled() {
            return EventResult::Ignored;
        }
        match event.key {
            crate::event::Key::Enter | crate::event::Key::Char(' ') => {
                self.toggle();
                EventResult::ConsumedAndRender
            }
            _ => EventResult::Ignored,
        }
    }

    /// Whether this toggle can receive focus (true unless disabled)
    fn toggle_focusable(&self) -> bool {
        !self.is_toggle_disabled()
    }
}
