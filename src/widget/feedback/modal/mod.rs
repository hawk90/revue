//! Modal/Dialog widget for displaying overlays
//!
//! The struct, its builders and presets live here; the child modules see its
//! private fields:
//! - `visibility`: showing and hiding, with or without a focus trap;
//! - `key_handling`: button selection and keys;
//! - `render`: the size and the drawing.

mod key_handling;
mod render;
#[cfg(test)]
mod tests;
mod types;
mod visibility;

pub use types::{ModalButton, ModalButtonStyle};

use crate::event::FocusTrap;
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
