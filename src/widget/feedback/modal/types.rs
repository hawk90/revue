//! Modal button configuration and style presets

/// Button configuration for modal dialogs
///
/// This is distinct from the interactive `Button` widget.
/// `ModalButton` configures the appearance and label of buttons
/// shown at the bottom of modal dialogs.
#[derive(Clone)]
pub struct ModalButton {
    /// Button label
    pub label: String,
    /// Button style
    pub style: ModalButtonStyle,
}

/// Style preset for modal buttons
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ModalButtonStyle {
    /// Default neutral button
    #[default]
    Default,
    /// Primary action button (highlighted)
    Primary,
    /// Danger/destructive action button
    Danger,
}

impl ModalButton {
    /// Create a new button with default style
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Default,
        }
    }

    /// Create a primary action button
    pub fn primary(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Primary,
        }
    }

    /// Create a danger/destructive action button
    pub fn danger(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Danger,
        }
    }

    /// Set button style
    pub fn style(mut self, style: ModalButtonStyle) -> Self {
        self.style = style;
        self
    }
}
