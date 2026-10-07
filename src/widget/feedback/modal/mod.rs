//! Modal/Dialog widget for displaying overlays

mod key_handling;
mod render;
mod types;

pub use types::{ModalButton, ModalButtonStyle};

use crate::event::{FocusManager, FocusTrap};
use crate::style::Color;
use crate::widget::traits::{View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A modal dialog widget
pub struct Modal {
    title: String,
    /// Text content (for simple messages)
    content: Vec<String>,
    /// Child widget content (takes precedence over text content)
    body: Option<Box<dyn View>>,
    buttons: Vec<ModalButton>,
    selected_button: usize,
    visible: bool,
    width: u16,
    height: Option<u16>,
    title_fg: Option<Color>,
    border_fg: Option<Color>,
    props: WidgetProps,
    /// Focus trap for keyboard focus management
    focus_trap: Option<FocusTrap>,
}

impl Modal {
    /// Create a new modal dialog
    pub fn new() -> Self {
        Self {
            title: String::new(),
            content: Vec::new(),
            body: None,
            buttons: Vec::new(),
            selected_button: 0,
            visible: false,
            width: 40,
            height: None,
            title_fg: Some(Color::WHITE),
            border_fg: Some(Color::WHITE),
            props: WidgetProps::new(),
            focus_trap: None,
        }
    }

    /// Set modal title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set modal content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into().lines().map(|s| s.to_string()).collect();
        self
    }

    /// Add a line to content
    pub fn line(mut self, line: impl Into<String>) -> Self {
        self.content.push(line.into());
        self
    }

    /// Set buttons
    pub fn buttons(mut self, buttons: Vec<ModalButton>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Add OK button
    pub fn ok(mut self) -> Self {
        self.buttons.push(ModalButton::primary("OK"));
        self
    }

    /// Add Cancel button
    pub fn cancel(mut self) -> Self {
        self.buttons.push(ModalButton::new("Cancel"));
        self
    }

    /// Add OK and Cancel buttons
    pub fn ok_cancel(mut self) -> Self {
        self.buttons.push(ModalButton::primary("OK"));
        self.buttons.push(ModalButton::new("Cancel"));
        self
    }

    /// Add Yes and No buttons
    pub fn yes_no(mut self) -> Self {
        self.buttons.push(ModalButton::primary("Yes"));
        self.buttons.push(ModalButton::new("No"));
        self
    }

    /// Add Yes, No, and Cancel buttons
    pub fn yes_no_cancel(mut self) -> Self {
        self.buttons.push(ModalButton::primary("Yes"));
        self.buttons.push(ModalButton::new("No"));
        self.buttons.push(ModalButton::new("Cancel"));
        self
    }

    /// Set modal width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set modal height (None = auto)
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Set a child widget as body content
    ///
    /// When a body widget is set, it takes precedence over text content.
    /// The widget will be rendered inside the modal's content area.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use revue::prelude::*;
    ///
    /// let modal = Modal::new()
    ///     .title("User Form")
    ///     .body(
    ///         vstack()
    ///             .gap(1)
    ///             .child(Input::new().placeholder("Name"))
    ///             .child(Input::new().placeholder("Email"))
    ///     )
    ///     .ok_cancel();
    /// ```
    pub fn body(mut self, widget: impl View + 'static) -> Self {
        self.body = Some(Box::new(widget));
        self
    }

    /// Set title color
    pub fn title_fg(mut self, color: Color) -> Self {
        self.title_fg = Some(color);
        self
    }

    /// Set border color
    pub fn border_fg(mut self, color: Color) -> Self {
        self.border_fg = Some(color);
        self
    }

    /// Show the modal
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Show the modal and activate focus trapping
    ///
    /// Traps keyboard focus within the modal's buttons so Tab/Shift+Tab
    /// only cycles between modal buttons and doesn't escape to background widgets.
    ///
    /// # Arguments
    /// * `fm` - The focus manager to trap focus in
    /// * `container_id` - Unique ID for this modal's focus trap container
    /// * `button_ids` - Widget IDs for each button (must match button count)
    pub fn show_with_focus_trap(
        &mut self,
        fm: &mut FocusManager,
        container_id: u64,
        button_ids: &[u64],
    ) {
        self.visible = true;
        let mut trap = FocusTrap::new(container_id).with_children(button_ids);
        trap.activate(fm);
        self.focus_trap = Some(trap);
    }

    /// Hide the modal
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Hide the modal and release focus trapping
    ///
    /// Releases the focus trap and restores focus to the previously focused widget.
    pub fn hide_with_focus_restore(&mut self, fm: &mut FocusManager) {
        self.visible = false;
        if let Some(mut trap) = self.focus_trap.take() {
            trap.deactivate(fm);
        }
    }

    /// Check if this modal has an active focus trap
    pub fn has_focus_trap(&self) -> bool {
        self.focus_trap.as_ref().is_some_and(|t| t.is_active())
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if modal is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Create alert dialog
    pub fn alert(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new().title(title).content(message).ok()
    }

    /// Create confirmation dialog
    pub fn confirm(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new().title(title).content(message).yes_no()
    }

    /// Create error dialog
    pub fn error(message: impl Into<String>) -> Self {
        Self::new()
            .title("Error")
            .title_fg(Color::RED)
            .border_fg(Color::RED)
            .content(message)
            .ok()
    }

    /// Create warning dialog
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new()
            .title("Warning")
            .title_fg(Color::YELLOW)
            .border_fg(Color::YELLOW)
            .content(message)
            .ok()
    }

    /// Calculate required height
    #[doc(hidden)]
    pub fn required_height(&self) -> u16 {
        // If height is explicitly set, use it
        if let Some(h) = self.height {
            return h;
        }

        // For body widget, use a default content height of 5 lines
        let content_lines = if self.body.is_some() {
            5u16
        } else {
            self.content.len() as u16
        };

        let button_line = if self.buttons.is_empty() { 0 } else { 1 };
        // top border + title + title separator + content + padding + buttons + bottom border
        3 + content_lines + 1 + button_line + 1
    }

    // Getters for testing
    #[doc(hidden)]
    pub fn get_title(&self) -> &str {
        &self.title
    }

    #[doc(hidden)]
    pub fn get_content(&self) -> &[String] {
        &self.content
    }

    #[doc(hidden)]
    pub fn get_buttons(&self) -> &[ModalButton] {
        &self.buttons
    }

    #[doc(hidden)]
    pub fn get_body(&self) -> bool {
        self.body.is_some()
    }

    #[doc(hidden)]
    pub fn get_height(&self) -> Option<u16> {
        self.height
    }

    #[doc(hidden)]
    pub fn get_title_fg(&self) -> Option<Color> {
        self.title_fg
    }

    #[doc(hidden)]
    pub fn get_border_fg(&self) -> Option<Color> {
        self.border_fg
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a modal
pub fn modal() -> Modal {
    Modal::new()
}

impl_styled_view!(Modal);
impl_props_builders!(Modal);

// KEEP HERE - Private implementation tests (all tests access private fields: title, content, buttons, is_visible, etc.)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_new() {
        let m = Modal::new();
        assert!(!m.is_visible());
        assert!(m.title.is_empty());
        assert!(m.content.is_empty());
        assert!(m.buttons.is_empty());
    }

    #[test]
    fn test_modal_builder() {
        let m = Modal::new()
            .title("Test")
            .content(
                "Hello
World",
            )
            .ok_cancel();

        assert_eq!(m.title, "Test");
        assert_eq!(m.content.len(), 2);
        assert_eq!(m.buttons.len(), 2);
    }

    #[test]
    fn test_modal_visibility() {
        let mut m = Modal::new();
        assert!(!m.is_visible());

        m.show();
        assert!(m.is_visible());

        m.hide();
        assert!(!m.is_visible());

        m.toggle();
        assert!(m.is_visible());
    }

    #[test]
    fn test_modal_presets() {
        let alert = Modal::alert("Title", "Message");
        assert_eq!(alert.title, "Title");
        assert_eq!(alert.buttons.len(), 1);

        let confirm = Modal::confirm("Title", "Question?");
        assert_eq!(confirm.buttons.len(), 2);

        let error = Modal::error("Something went wrong");
        assert_eq!(error.title, "Error");
    }

    #[test]
    fn test_modal_helper() {
        let m = modal().title("Quick").ok();

        assert_eq!(m.title, "Quick");
    }

    #[test]
    fn test_modal_with_body() {
        use crate::widget::Text;

        let m = Modal::new()
            .title("Form")
            .body(Text::new("Custom content"))
            .height(10)
            .ok();

        assert!(m.body.is_some());
        assert_eq!(m.height, Some(10));
    }

    // =========================================================================
    // Modal builder method tests for edge cases
    // =========================================================================

    #[test]
    fn test_modal_empty_title() {
        let m = Modal::new().title("");
        assert_eq!(m.title, "");
    }

    #[test]
    fn test_modal_empty_content() {
        let m = Modal::new().content("");
        assert!(m.content.is_empty());
    }

    #[test]
    fn test_modal_content_with_multiline() {
        let m = Modal::new().content("Line 1\nLine 2\nLine 3");
        assert_eq!(m.content.len(), 3);
        assert_eq!(m.content[0], "Line 1");
        assert_eq!(m.content[1], "Line 2");
        assert_eq!(m.content[2], "Line 3");
    }

    #[test]
    fn test_modal_line_multiple() {
        let m = Modal::new().line("Line 1").line("Line 2").line("Line 3");
        assert_eq!(m.content.len(), 3);
    }

    #[test]
    fn test_modal_buttons_empty() {
        let m = Modal::new().buttons(vec![]);
        assert!(m.buttons.is_empty());
    }

    #[test]
    fn test_modal_width_zero() {
        let m = Modal::new().width(0);
        assert_eq!(m.width, 0);
    }

    #[test]
    fn test_modal_height_zero() {
        let m = Modal::new().height(0);
        assert_eq!(m.height, Some(0));
    }

    #[test]
    fn test_modal_title_colors() {
        let m = Modal::new().title_fg(Color::CYAN);
        assert_eq!(m.title_fg, Some(Color::CYAN));
    }

    #[test]
    fn test_modal_border_colors() {
        let m = Modal::new().border_fg(Color::MAGENTA);
        assert_eq!(m.border_fg, Some(Color::MAGENTA));
    }

    // =========================================================================
    // Modal builder chain tests
    // =========================================================================

    #[test]
    fn test_modal_builder_chain_full() {
        let m = Modal::new()
            .title("Chain Title")
            .content("Chain content")
            .width(60)
            .height(10)
            .title_fg(Color::YELLOW)
            .border_fg(Color::GREEN);

        assert_eq!(m.title, "Chain Title");
        assert_eq!(m.content.len(), 1);
        assert_eq!(m.content[0], "Chain content");
        assert_eq!(m.width, 60);
        assert_eq!(m.height, Some(10));
        assert_eq!(m.title_fg, Some(Color::YELLOW));
        assert_eq!(m.border_fg, Some(Color::GREEN));
    }

    #[test]
    fn test_modal_buttons_builder_chain() {
        let buttons = vec![
            ModalButton::new("One"),
            ModalButton::primary("Two"),
            ModalButton::danger("Three"),
        ];
        let m = Modal::new().buttons(buttons.clone());

        assert_eq!(m.buttons.len(), 3);
        assert_eq!(m.buttons[0].label, "One");
        assert_eq!(m.buttons[1].label, "Two");
        assert_eq!(m.buttons[2].label, "Three");
    }

    // =========================================================================
    // Focus trap integration tests
    // =========================================================================

    #[test]
    fn test_modal_show_with_focus_trap() {
        let mut fm = FocusManager::new();
        fm.register(1); // background widget
        fm.register(2); // background widget
        fm.register(10); // modal button 1
        fm.register(11); // modal button 2
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        assert!(m.is_visible());
        assert!(m.has_focus_trap());
        assert!(fm.is_trapped());
        // Focus should be on first trapped child
        assert_eq!(fm.current(), Some(10));
    }

    #[test]
    fn test_modal_hide_with_focus_restore() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);
        assert_eq!(fm.current(), Some(10));

        m.hide_with_focus_restore(&mut fm);
        assert!(!m.is_visible());
        assert!(!m.has_focus_trap());
        assert!(!fm.is_trapped());
        // Focus should be restored to widget 1
        assert_eq!(fm.current(), Some(1));
    }

    #[test]
    fn test_modal_no_focus_trap_by_default() {
        let m = Modal::new().ok();
        assert!(!m.has_focus_trap());
    }
}
