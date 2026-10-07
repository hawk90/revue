//! Masked input widget for passwords and sensitive data
//!
//! Provides input fields that hide or mask the entered text, perfect for
//! passwords, PINs, credit card numbers, and other sensitive information.

#![allow(clippy::iter_skip_next)]
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{MaskedInput, MaskStyle, masked_input, password_input};
//!
//! // Password input (dots)
//! let password = MaskedInput::password()
//!     .placeholder("Enter password");
//!
//! // PIN input (asterisks)
//! let pin = MaskedInput::new()
//!     .mask_char('*')
//!     .max_length(4);
//!
//! // Credit card input (show last 4)
//! let card = MaskedInput::new()
//!     .mask_style(MaskStyle::ShowLast(4))
//!     .placeholder("Card number");
//!
//! // Using helper
//! let pwd = password_input("Password");
//! ```

mod editing;
mod render;
mod types;
mod validation;

pub use types::{MaskStyle, ValidationState};

use crate::style::Color;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Default peek timeout in frames for MaskStyle::Peek
///
/// This controls how long the last typed character remains visible
/// before being masked. At 60 FPS, 10 frames ≈ 167ms.
const DEFAULT_PEEK_TIMEOUT: usize = 10;

/// Masked input widget
#[derive(Clone, Debug)]
pub struct MaskedInput {
    /// Current value
    value: String,
    /// Mask character
    mask_char: char,
    /// Mask style
    mask_style: MaskStyle,
    /// Placeholder text
    placeholder: Option<String>,
    /// Label text
    label: Option<String>,
    /// Maximum length (0 = unlimited)
    max_length: usize,
    /// Minimum length for validation
    min_length: usize,
    /// Cursor position
    cursor: usize,
    /// Whether input is focused
    focused: bool,
    /// Whether input is disabled
    disabled: bool,
    /// Foreground color
    fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Width of input field
    width: Option<u16>,
    /// Validation state
    validation: ValidationState,
    /// Show strength indicator (for passwords)
    show_strength: bool,
    /// Allow reveal toggle
    allow_reveal: bool,
    /// Currently revealing
    revealing: bool,
    /// Peek timeout (frames)
    peek_timeout: usize,
    /// Current peek countdown
    peek_countdown: usize,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl MaskedInput {
    /// Create new masked input
    pub fn new() -> Self {
        Self {
            value: String::new(),
            mask_char: '●',
            mask_style: MaskStyle::Full,
            placeholder: None,
            label: None,
            max_length: 0,
            min_length: 0,
            cursor: 0,
            focused: false,
            disabled: false,
            fg: None,
            bg: None,
            width: None,
            validation: ValidationState::None,
            show_strength: false,
            allow_reveal: false,
            revealing: false,
            peek_timeout: DEFAULT_PEEK_TIMEOUT,
            peek_countdown: 0,
            props: WidgetProps::new(),
        }
    }

    /// Create password input with defaults
    pub fn password() -> Self {
        Self::new()
            .mask_char('●')
            .mask_style(MaskStyle::Full)
            .show_strength(true)
    }

    /// Create PIN input
    pub fn pin(length: usize) -> Self {
        Self::new()
            .mask_char('*')
            .max_length(length)
            .mask_style(MaskStyle::Full)
    }

    /// Create credit card input
    pub fn credit_card() -> Self {
        Self::new()
            .mask_char('•')
            .mask_style(MaskStyle::ShowLast(4))
            .max_length(16)
    }

    /// Set mask character
    pub fn mask_char(mut self, c: char) -> Self {
        self.mask_char = c;
        self
    }

    /// Set mask style
    pub fn mask_style(mut self, style: MaskStyle) -> Self {
        self.mask_style = style;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set label
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = len;
        self
    }

    /// Set minimum length
    pub fn min_length(mut self, len: usize) -> Self {
        self.min_length = len;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Show password strength indicator
    pub fn show_strength(mut self, show: bool) -> Self {
        self.show_strength = show;
        self
    }

    /// Allow reveal toggle
    pub fn allow_reveal(mut self, allow: bool) -> Self {
        self.allow_reveal = allow;
        self
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.char_count();
        self
    }

    /// Get current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Get mask character
    pub fn get_mask_char(&self) -> char {
        self.mask_char
    }

    /// Get mask style
    pub fn get_mask_style(&self) -> MaskStyle {
        self.mask_style
    }

    /// Get placeholder text
    pub fn get_placeholder(&self) -> Option<&String> {
        self.placeholder.as_ref()
    }

    /// Get label text
    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    /// Get maximum length
    pub fn get_max_length(&self) -> usize {
        self.max_length
    }

    /// Get minimum length
    pub fn get_min_length(&self) -> usize {
        self.min_length
    }

    /// Get cursor position
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos.min(self.char_count());
    }

    /// Get focused state
    pub fn get_focused(&self) -> bool {
        self.focused
    }

    /// Get disabled state
    pub fn get_disabled(&self) -> bool {
        self.disabled
    }

    /// Get foreground color
    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    /// Get background color
    pub fn get_bg(&self) -> Option<Color> {
        self.bg
    }

    /// Get width
    pub fn get_width(&self) -> Option<u16> {
        self.width
    }

    /// Get show strength indicator state
    pub fn get_show_strength(&self) -> bool {
        self.show_strength
    }

    /// Get allow reveal state
    pub fn get_allow_reveal(&self) -> bool {
        self.allow_reveal
    }

    /// Get revealing state
    pub fn get_revealing(&self) -> bool {
        self.revealing
    }

    /// Set revealing state
    pub fn set_revealing(&mut self, revealing: bool) {
        self.revealing = revealing;
    }

    /// Get peek countdown
    pub fn get_peek_countdown(&self) -> usize {
        self.peek_countdown
    }

    /// Set peek countdown
    pub fn set_peek_countdown(&mut self, countdown: usize) {
        self.peek_countdown = countdown;
    }

    /// Get validation state
    pub fn get_validation(&self) -> &ValidationState {
        &self.validation
    }

    /// Set value programmatically
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.cursor.min(self.char_count());
    }

    /// Clear the input
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
    }

    /// Toggle reveal mode
    pub fn toggle_reveal(&mut self) {
        if self.allow_reveal {
            self.revealing = !self.revealing;
        }
    }

    /// Length of the value in chars: the cursor, `max_length` and
    /// `min_length` all count chars, not bytes.
    fn char_count(&self) -> usize {
        self.value.chars().count()
    }

    /// Byte offset of char index `idx` in the value.
    fn byte_at(&self, idx: usize) -> usize {
        crate::utils::text::char_to_byte_index(&self.value, idx)
    }
}

impl Default for MaskedInput {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(MaskedInput);
impl_props_builders!(MaskedInput);

impl MaskedInput {
    /// Set element ID for CSS selector (#id)
    pub fn set_id(&mut self, id: impl Into<String>) {
        self.props.id = Some(id.into());
    }

    /// Add a CSS class
    pub fn add_class(&mut self, class: impl Into<String>) {
        let class_str = class.into();
        if !self.props.classes.contains(&class_str) {
            self.props.classes.push(class_str);
        }
    }

    /// Remove a CSS class
    pub fn remove_class(&mut self, class: &str) {
        self.props.classes.retain(|c| c != class);
    }

    /// Toggle a CSS class
    pub fn toggle_class(&mut self, class: &str) {
        if self.has_class(class) {
            self.remove_class(class);
        } else {
            self.props.classes.push(class.to_string());
        }
    }

    /// Check if widget has a CSS class
    pub fn has_class(&self, class: &str) -> bool {
        self.props.classes.iter().any(|c| c == class)
    }

    /// Get the CSS classes as a slice
    pub fn get_classes(&self) -> &[String] {
        &self.props.classes
    }

    /// Get the element ID
    pub fn get_id(&self) -> Option<&str> {
        self.props.id.as_deref()
    }
}

/// Create a masked input
pub fn masked_input() -> MaskedInput {
    MaskedInput::new()
}

/// Create a password input
pub fn password_input(placeholder: impl Into<String>) -> MaskedInput {
    MaskedInput::password().placeholder(placeholder)
}

/// Create a PIN input
pub fn pin_input(length: usize) -> MaskedInput {
    MaskedInput::pin(length)
}

/// Create a credit card input
pub fn credit_card_input() -> MaskedInput {
    MaskedInput::credit_card()
}
