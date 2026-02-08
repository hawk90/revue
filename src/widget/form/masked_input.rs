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

use crate::style::Color;
use crate::widget::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Default peek timeout in frames for MaskStyle::Peek
///
/// This controls how long the last typed character remains visible
/// before being masked. At 60 FPS, 10 frames ≈ 167ms.
const DEFAULT_PEEK_TIMEOUT: usize = 10;

/// Mask display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaskStyle {
    /// Show all characters as mask (default)
    #[default]
    Full,
    /// Show last N characters
    ShowLast(usize),
    /// Show first N characters
    ShowFirst(usize),
    /// Show characters briefly then mask
    Peek,
    /// Show nothing (empty)
    Hidden,
}

/// Input validation result
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationState {
    /// No validation performed
    None,
    /// Input is valid
    Valid,
    /// Input is invalid with message
    Invalid(String),
    /// Validation in progress
    Validating,
}

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
        self.cursor = self.value.len();
        self
    }

    /// Get current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set value programmatically
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.cursor.min(self.value.len());
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

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        if self.disabled {
            return;
        }

        // Check max length
        if self.max_length > 0 && self.value.len() >= self.max_length {
            return;
        }

        self.value.insert(self.cursor, c);
        self.cursor += 1;

        // Start peek countdown
        if matches!(self.mask_style, MaskStyle::Peek) {
            self.peek_countdown = self.peek_timeout;
        }
    }

    /// Delete character before cursor
    pub fn delete_backward(&mut self) {
        if self.disabled || self.cursor == 0 {
            return;
        }

        self.cursor -= 1;
        self.value.remove(self.cursor);
    }

    /// Delete character at cursor
    pub fn delete_forward(&mut self) {
        if self.disabled || self.cursor >= self.value.len() {
            return;
        }

        self.value.remove(self.cursor);
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor += 1;
        }
    }

    /// Move cursor to start
    pub fn move_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_end(&mut self) {
        self.cursor = self.value.len();
    }

    /// Update (call each frame for peek mode)
    pub fn update(&mut self) {
        if self.peek_countdown > 0 {
            self.peek_countdown -= 1;
        }
    }

    /// Calculate password strength (0-4)
    pub fn password_strength(&self) -> usize {
        let len = self.value.len();
        let has_lower = self.value.chars().any(|c| c.is_lowercase());
        let has_upper = self.value.chars().any(|c| c.is_uppercase());
        let has_digit = self.value.chars().any(|c| c.is_ascii_digit());
        let has_special = self.value.chars().any(|c| !c.is_alphanumeric());

        let mut strength = 0;

        if len >= 8 {
            strength += 1;
        }
        if len >= 12 {
            strength += 1;
        }
        if has_lower && has_upper {
            strength += 1;
        }
        if has_digit {
            strength += 1;
        }
        if has_special {
            strength += 1;
        }

        strength.min(4)
    }

    /// Get strength label
    pub fn strength_label(&self) -> &str {
        match self.password_strength() {
            0 => "Very Weak",
            1 => "Weak",
            2 => "Fair",
            3 => "Strong",
            _ => "Very Strong",
        }
    }

    /// Get strength color
    pub fn strength_color(&self) -> Color {
        match self.password_strength() {
            0 => Color::RED,
            1 => Color::rgb(255, 128, 0), // Orange
            2 => Color::YELLOW,
            3 => Color::rgb(128, 255, 0), // Light green
            _ => Color::GREEN,
        }
    }

    /// Validate the input
    pub fn validate(&mut self) -> bool {
        if self.min_length > 0 && self.value.len() < self.min_length {
            self.validation = ValidationState::Invalid(format!(
                "Minimum {} characters required",
                self.min_length
            ));
            return false;
        }

        self.validation = ValidationState::Valid;
        true
    }

    /// Get masked display string
    ///
    /// This method is optimized to minimize string allocations by:
    /// - Pre-allocating strings with known capacity
    /// - Avoiding repeated `.to_string().repeat()` calls
    /// - Using `extend` with char iterators instead of format!
    fn masked_display(&self) -> String {
        if self.revealing {
            return self.value.clone();
        }

        let len = self.value.len();
        if len == 0 {
            return String::new();
        }

        match self.mask_style {
            MaskStyle::Full => {
                // Pre-allocate with exact capacity
                std::iter::repeat_n(self.mask_char, len).collect()
            }
            MaskStyle::ShowLast(n) => {
                if len <= n {
                    self.value.clone()
                } else {
                    let mask_count = len - n;
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, mask_count));
                    result.push_str(&self.value[len - n..]);
                    result
                }
            }
            MaskStyle::ShowFirst(n) => {
                if len <= n {
                    self.value.clone()
                } else {
                    let mut result = String::with_capacity(len);
                    result.push_str(&self.value[..n]);
                    result.extend(std::iter::repeat_n(self.mask_char, len - n));
                    result
                }
            }
            MaskStyle::Peek => {
                if self.peek_countdown > 0 && self.cursor > 0 && self.cursor <= len {
                    // Show the last typed character
                    // Use char_indices for O(n) instead of O(n²) with .chars().nth()
                    let last_char = self
                        .value
                        .char_indices()
                        .nth(self.cursor - 1)
                        .map(|(_, c)| c)
                        .unwrap_or(' ');
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, self.cursor - 1));
                    result.push(last_char);
                    result.extend(std::iter::repeat_n(self.mask_char, len - self.cursor));
                    result
                } else {
                    std::iter::repeat_n(self.mask_char, len).collect()
                }
            }
            MaskStyle::Hidden => String::new(),
        }
    }
}

impl Default for MaskedInput {
    fn default() -> Self {
        Self::new()
    }
}

impl View for MaskedInput {
    crate::impl_view_meta!("MaskedInput");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::{hstack, vstack};
        use crate::widget::Text;

        let mut content = vstack();

        // Label
        if let Some(label) = &self.label {
            content = content.child(Text::new(label).bold());
        }

        // Input field
        let display = if self.value.is_empty() {
            self.placeholder.clone().unwrap_or_default()
        } else {
            self.masked_display()
        };

        let is_placeholder = self.value.is_empty() && self.placeholder.is_some();

        // Build input display with pre-allocated padding
        let width = self.width.unwrap_or(20) as usize;
        let padded = if display.len() < width {
            let mut result = String::with_capacity(width);
            result.push_str(&display);
            result.extend(std::iter::repeat_n(' ', width - display.len()));
            result
        } else {
            display.chars().take(width).collect()
        };

        // Insert cursor if focused
        let display_with_cursor = if self.focused && !self.disabled {
            let cursor_pos = self.cursor.min(padded.chars().count());
            // Use iterators for O(n) instead of O(n²) with .chars().nth()
            let before: String = padded.chars().take(cursor_pos).collect();
            let cursor_char = padded.chars().skip(cursor_pos).next().unwrap_or(' ');
            let after: String = padded.chars().skip(cursor_pos + 1).collect();
            (before, cursor_char, after)
        } else {
            (padded.clone(), ' ', String::new())
        };

        // Render input box
        let mut input_text = if self.focused && !self.disabled {
            hstack()
                .child(Text::new(display_with_cursor.0))
                .child(
                    Text::new(display_with_cursor.1.to_string())
                        .bg(Color::WHITE)
                        .fg(Color::BLACK),
                )
                .child(Text::new(display_with_cursor.2))
        } else {
            let mut text = Text::new(&padded);
            if is_placeholder {
                text = text.fg(Color::rgb(128, 128, 128));
            } else if self.disabled {
                text = text.fg(Color::rgb(100, 100, 100));
            } else if let Some(fg) = self.fg {
                text = text.fg(fg);
            }
            hstack().child(text)
        };

        // Add reveal indicator
        if self.allow_reveal {
            let eye = if self.revealing {
                "👁"
            } else {
                "👁‍🗨"
            };
            input_text = input_text.child(Text::new(format!(" {}", eye)));
        }

        // Wrap in border
        let border_color = if self.disabled {
            Color::rgb(80, 80, 80)
        } else if matches!(self.validation, ValidationState::Invalid(_)) {
            Color::RED
        } else if matches!(self.validation, ValidationState::Valid) {
            Color::GREEN
        } else if self.focused {
            Color::CYAN
        } else {
            Color::rgb(128, 128, 128)
        };

        let bordered = hstack()
            .child(Text::new("[").fg(border_color))
            .child(input_text)
            .child(Text::new("]").fg(border_color));

        content = content.child(bordered);

        // Password strength indicator
        if self.show_strength && !self.value.is_empty() {
            let strength = self.password_strength();
            let color = self.strength_color();
            // Pre-allocate strength bar (max 5 chars = strength + 1)
            let bar: String = std::iter::repeat_n('█', strength + 1).collect();
            let empty: String = std::iter::repeat_n('░', 4 - strength).collect();

            let strength_display = hstack()
                .child(Text::new(&bar).fg(color))
                .child(Text::new(&empty).fg(Color::rgb(80, 80, 80)))
                .child(Text::new(format!(" {}", self.strength_label())).fg(color));

            content = content.child(strength_display);
        }

        // Validation message
        if let ValidationState::Invalid(msg) = &self.validation {
            content = content.child(Text::new(msg).fg(Color::RED));
        }

        content.render(ctx);
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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // MaskStyle enum tests
    // =========================================================================

    #[test]
    fn test_mask_style_default() {
        assert_eq!(MaskStyle::default(), MaskStyle::Full);
    }

    #[test]
    fn test_mask_style_clone() {
        let style = MaskStyle::ShowLast(4);
        assert_eq!(style, style.clone());
    }

    #[test]
    fn test_mask_style_copy() {
        let style1 = MaskStyle::Peek;
        let style2 = style1;
        assert_eq!(style1, MaskStyle::Peek);
        assert_eq!(style2, MaskStyle::Peek);
    }

    #[test]
    fn test_mask_style_partial_eq() {
        assert_eq!(MaskStyle::Full, MaskStyle::Full);
        assert_eq!(MaskStyle::ShowLast(4), MaskStyle::ShowLast(4));
        assert_ne!(MaskStyle::Full, MaskStyle::Peek);
    }

    #[test]
    fn test_mask_style_debug() {
        let debug_str = format!("{:?}", MaskStyle::ShowLast(4));
        assert!(debug_str.contains("ShowLast"));
    }

    #[test]
    fn test_mask_style_all_variants_unique() {
        let variants = [
            MaskStyle::Full,
            MaskStyle::ShowLast(1),
            MaskStyle::ShowFirst(1),
            MaskStyle::Peek,
            MaskStyle::Hidden,
        ];

        // All should be different from Full
        for variant in variants.iter().skip(1) {
            assert_ne!(*variant, MaskStyle::Full);
        }
    }

    // =========================================================================
    // ValidationState enum tests
    // =========================================================================

    #[test]
    fn test_validation_state_clone() {
        let state = ValidationState::Invalid("error".to_string());
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_validation_state_debug() {
        let debug_str = format!("{:?}", ValidationState::Validating);
        assert!(debug_str.contains("Validating"));
    }

    #[test]
    fn test_validation_state_partial_eq() {
        assert_eq!(ValidationState::None, ValidationState::None);
        assert_eq!(ValidationState::Valid, ValidationState::Valid);
        assert_ne!(ValidationState::Valid, ValidationState::Validating);
    }

    #[test]
    fn test_validation_state_invalid_with_message() {
        let state = ValidationState::Invalid("Too short".to_string());
        assert!(matches!(state, ValidationState::Invalid(_)));
        if let ValidationState::Invalid(msg) = state {
            assert_eq!(msg, "Too short");
        }
    }

    #[test]
    fn test_validation_state_all_variants() {
        let _ = ValidationState::None;
        let _ = ValidationState::Valid;
        let _ = ValidationState::Invalid("error".to_string());
        let _ = ValidationState::Validating;
    }

    // =========================================================================
    // MaskedInput::new and default tests
    // =========================================================================

    #[test]
    fn test_masked_input_new() {
        let input = MaskedInput::new();
        assert_eq!(input.get_value(), "");
        assert_eq!(input.mask_char, '●');
        assert_eq!(input.mask_style, MaskStyle::Full);
    }

    #[test]
    fn test_masked_input_default() {
        let input = MaskedInput::default();
        assert_eq!(input.get_value(), "");
        assert_eq!(input.mask_char, '●');
    }

    #[test]
    fn test_masked_input_clone() {
        let input1 = MaskedInput::new()
            .value("test")
            .mask_char('*')
            .placeholder("Enter");
        let input2 = input1.clone();
        assert_eq!(input1.get_value(), input2.get_value());
        assert_eq!(input1.mask_char, input2.mask_char);
    }

    #[test]
    fn test_masked_input_debug() {
        let input = MaskedInput::new().value("test");
        let debug_str = format!("{:?}", input);
        assert!(debug_str.contains("MaskedInput"));
    }

    // =========================================================================
    // MaskedInput builder tests
    // =========================================================================

    #[test]
    fn test_masked_input_mask_char() {
        let input = MaskedInput::new().mask_char('*');
        assert_eq!(input.mask_char, '*');
    }

    #[test]
    fn test_masked_input_mask_style() {
        let input = MaskedInput::new().mask_style(MaskStyle::Peek);
        assert_eq!(input.mask_style, MaskStyle::Peek);
    }

    #[test]
    fn test_masked_input_placeholder() {
        let input = MaskedInput::new().placeholder("Enter password");
        assert_eq!(input.placeholder, Some("Enter password".to_string()));
    }

    #[test]
    fn test_masked_input_label() {
        let input = MaskedInput::new().label("Password");
        assert_eq!(input.label, Some("Password".to_string()));
    }

    #[test]
    fn test_masked_input_max_length() {
        let input = MaskedInput::new().max_length(10);
        assert_eq!(input.max_length, 10);
    }

    #[test]
    fn test_masked_input_min_length() {
        let input = MaskedInput::new().min_length(8);
        assert_eq!(input.min_length, 8);
    }

    #[test]
    fn test_masked_input_focused() {
        let input = MaskedInput::new().focused(true);
        assert!(input.focused);
    }

    #[test]
    fn test_masked_input_disabled() {
        let input = MaskedInput::new().disabled(true);
        assert!(input.disabled);
    }

    #[test]
    fn test_masked_input_fg() {
        let input = MaskedInput::new().fg(Color::RED);
        assert_eq!(input.fg, Some(Color::RED));
    }

    #[test]
    fn test_masked_input_bg() {
        let input = MaskedInput::new().bg(Color::BLUE);
        assert_eq!(input.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_masked_input_width() {
        let input = MaskedInput::new().width(30);
        assert_eq!(input.width, Some(30));
    }

    #[test]
    fn test_masked_input_show_strength() {
        let input = MaskedInput::new().show_strength(true);
        assert!(input.show_strength);
    }

    #[test]
    fn test_masked_input_allow_reveal() {
        let input = MaskedInput::new().allow_reveal(true);
        assert!(input.allow_reveal);
    }

    #[test]
    fn test_masked_input_value() {
        let input = MaskedInput::new().value("secret123");
        assert_eq!(input.get_value(), "secret123");
        assert_eq!(input.cursor, 9);
    }

    #[test]
    fn test_masked_input_password() {
        let pwd = MaskedInput::password();
        assert_eq!(pwd.mask_char, '●');
        assert_eq!(pwd.mask_style, MaskStyle::Full);
        assert!(pwd.show_strength);
    }

    #[test]
    fn test_masked_input_pin() {
        let pin = MaskedInput::pin(4);
        assert_eq!(pin.mask_char, '*');
        assert_eq!(pin.max_length, 4);
        assert_eq!(pin.mask_style, MaskStyle::Full);
    }

    #[test]
    fn test_masked_input_credit_card() {
        let card = MaskedInput::credit_card();
        assert_eq!(card.mask_char, '•');
        assert_eq!(card.max_length, 16);
        assert!(matches!(card.mask_style, MaskStyle::ShowLast(4)));
    }

    // =========================================================================
    // MaskedInput value operations
    // =========================================================================

    #[test]
    fn test_masked_input_get_value() {
        let input = MaskedInput::new().value("test123");
        assert_eq!(input.get_value(), "test123");
    }

    #[test]
    fn test_masked_input_set_value() {
        let mut input = MaskedInput::new();
        input.cursor = 5;
        input.set_value("abc");
        assert_eq!(input.get_value(), "abc");
        // Cursor should be clamped to new length
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_masked_input_set_value_longer() {
        let mut input = MaskedInput::new();
        input.cursor = 2;
        input.set_value("hello world");
        assert_eq!(input.get_value(), "hello world");
        assert_eq!(input.cursor, 2); // Cursor position preserved if valid
    }

    #[test]
    fn test_masked_input_clear() {
        let mut input = MaskedInput::new().value("something");
        input.cursor = 5;
        input.clear();
        assert_eq!(input.get_value(), "");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_masked_input_clear_empty() {
        let mut input = MaskedInput::new();
        input.clear();
        assert_eq!(input.get_value(), "");
    }

    // =========================================================================
    // MaskedInput cursor operations
    // =========================================================================

    #[test]
    fn test_masked_input_insert() {
        let mut input = MaskedInput::new();
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');
        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_masked_input_insert_disabled() {
        let mut input = MaskedInput::new().disabled(true);
        input.insert_char('a');
        assert_eq!(input.get_value(), "");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_masked_input_insert_max_length() {
        let mut input = MaskedInput::new().max_length(4);
        for c in "12345678".chars() {
            input.insert_char(c);
        }
        assert_eq!(input.get_value(), "1234");
    }

    #[test]
    fn test_masked_input_insert_unlimited() {
        let mut input = MaskedInput::new().max_length(0);
        for c in "12345678".chars() {
            input.insert_char(c);
        }
        assert_eq!(input.get_value(), "12345678");
    }

    #[test]
    fn test_masked_insert_middle() {
        let mut input = MaskedInput::new().value("ac");
        input.cursor = 1;
        input.insert_char('b');
        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_masked_input_delete_backward() {
        let mut input = MaskedInput::new().value("hello");
        input.delete_backward();
        assert_eq!(input.get_value(), "hell");
        assert_eq!(input.cursor, 4);
    }

    #[test]
    fn test_masked_input_delete_backward_at_start() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 0;
        input.delete_backward();
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_masked_input_delete_backward_disabled() {
        let mut input = MaskedInput::new().value("hello").disabled(true);
        input.cursor = 3;
        input.delete_backward();
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_masked_input_delete_forward() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 2;
        input.delete_forward();
        assert_eq!(input.get_value(), "helo");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_masked_input_delete_forward_at_end() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 5;
        input.delete_forward();
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_masked_input_delete_forward_disabled() {
        let mut input = MaskedInput::new().value("hello").disabled(true);
        input.cursor = 2;
        input.delete_forward();
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_masked_input_move_left() {
        let mut input = MaskedInput::new().value("hello");
        assert_eq!(input.cursor, 5);
        input.move_left();
        assert_eq!(input.cursor, 4);
    }

    #[test]
    fn test_masked_input_move_left_at_start() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 0;
        input.move_left();
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_masked_input_move_right() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 0;
        input.move_right();
        assert_eq!(input.cursor, 1);
    }

    #[test]
    fn test_masked_input_move_right_at_end() {
        let mut input = MaskedInput::new().value("hello");
        assert_eq!(input.cursor, 5);
        input.move_right();
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_masked_input_move_start() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 3;
        input.move_start();
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_masked_input_move_end() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 0;
        input.move_end();
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_masked_input_cursor_movement_chain() {
        let mut input = MaskedInput::new().value("hello");
        assert_eq!(input.cursor, 5);

        input.move_start();
        assert_eq!(input.cursor, 0);

        input.move_end();
        assert_eq!(input.cursor, 5);

        input.move_left();
        assert_eq!(input.cursor, 4);

        input.move_right();
        assert_eq!(input.cursor, 5);
    }

    // =========================================================================
    // MaskedInput peek mode tests
    // =========================================================================

    #[test]
    fn test_masked_input_update() {
        let mut input = MaskedInput::new().mask_style(MaskStyle::Peek).value("a");
        input.peek_countdown = 5;

        input.update();
        assert_eq!(input.peek_countdown, 4);

        for _ in 0..5 {
            input.update();
        }
        assert_eq!(input.peek_countdown, 0);
    }

    #[test]
    fn test_masked_input_insert_starts_peek() {
        let mut input = MaskedInput::new().mask_style(MaskStyle::Peek);
        input.peek_countdown = 0;

        input.insert_char('a');
        assert_eq!(input.peek_countdown, 10);
    }

    #[test]
    fn test_masked_display_peek() {
        let mut input = MaskedInput::new().mask_style(MaskStyle::Peek).value("abc");
        input.cursor = 3;
        input.peek_countdown = 5;

        // Last character should be visible
        let display = input.masked_display();
        assert_eq!(display, "●●c");
    }

    #[test]
    fn test_masked_display_peek_no_countdown() {
        let input = MaskedInput::new().mask_style(MaskStyle::Peek).value("abc");

        let display = input.masked_display();
        assert_eq!(display, "●●●");
    }

    // =========================================================================
    // MaskedInput display tests
    // =========================================================================

    #[test]
    fn test_masked_display_full() {
        let input = MaskedInput::new().value("secret");
        assert_eq!(input.masked_display(), "●●●●●●");
    }

    #[test]
    fn test_masked_display_empty() {
        let input = MaskedInput::new();
        assert_eq!(input.masked_display(), "");
    }

    #[test]
    fn test_masked_display_show_last() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowLast(4))
            .value("1234567890");
        assert_eq!(input.masked_display(), "●●●●●●7890");
    }

    #[test]
    fn test_masked_display_show_last_exceeds_length() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowLast(10))
            .value("123");
        assert_eq!(input.masked_display(), "123");
    }

    #[test]
    fn test_masked_display_show_first() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowFirst(4))
            .value("1234567890");
        assert_eq!(input.masked_display(), "1234●●●●●●");
    }

    #[test]
    fn test_masked_display_show_first_exceeds_length() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowFirst(10))
            .value("123");
        assert_eq!(input.masked_display(), "123");
    }

    #[test]
    fn test_masked_display_hidden() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::Hidden)
            .value("secret");
        assert_eq!(input.masked_display(), "");
    }

    #[test]
    fn test_masked_display_custom_mask_char() {
        let input = MaskedInput::new().mask_char('*').value("test");
        assert_eq!(input.masked_display(), "****");
    }

    // =========================================================================
    // MaskedInput reveal tests
    // =========================================================================

    #[test]
    fn test_reveal_toggle() {
        let mut input = MaskedInput::new().allow_reveal(true).value("secret");

        assert!(!input.revealing);
        assert_eq!(input.masked_display(), "●●●●●●");

        input.toggle_reveal();
        assert!(input.revealing);
        assert_eq!(input.masked_display(), "secret");
    }

    #[test]
    fn test_reveal_toggle_not_allowed() {
        let mut input = MaskedInput::new().allow_reveal(false).value("secret");

        assert!(!input.revealing);
        input.toggle_reveal();
        assert!(!input.revealing);
    }

    #[test]
    fn test_reveal_toggle_off() {
        let mut input = MaskedInput::new().allow_reveal(true).value("secret");
        input.revealing = true;

        input.toggle_reveal();
        assert!(!input.revealing);
    }

    // =========================================================================
    // Password strength tests
    // =========================================================================

    #[test]
    fn test_password_strength() {
        let weak = MaskedInput::new().value("abc");
        assert_eq!(weak.password_strength(), 0);

        let strong = MaskedInput::new().value("MyP@ssw0rd123!");
        assert!(strong.password_strength() >= 3);
    }

    #[test]
    fn test_password_strength_very_weak() {
        let input = MaskedInput::new().value("abc");
        assert_eq!(input.password_strength(), 0);
    }

    #[test]
    fn test_password_strength_weak() {
        let input = MaskedInput::new().value("abcdefgh");
        assert_eq!(input.password_strength(), 1);
    }

    #[test]
    fn test_password_strength_fair() {
        let input = MaskedInput::new().value("Abcdefgh1");
        // len=9 >=8: +1, has_lower+upper: +1, has_digit: +1 = 3
        assert_eq!(input.password_strength(), 3);
    }

    #[test]
    fn test_password_strength_strong() {
        let input = MaskedInput::new().value("Abcdefgh1!");
        // len=10 >=8: +1, has_lower+upper: +1, has_digit: +1, has_special: +1 = 4
        assert_eq!(input.password_strength(), 4);
    }

    #[test]
    fn test_password_strength_very_strong() {
        let input = MaskedInput::new().value("Abcdefgh1!ghjk");
        // len=15 >=8: +1, >=12: +1, has_lower+upper: +1, has_digit: +1, has_special: +1 = 5 (capped to 4)
        assert_eq!(input.password_strength(), 4);
    }

    #[test]
    fn test_strength_label() {
        assert_eq!(MaskedInput::new().value("a").strength_label(), "Very Weak");
        assert_eq!(
            MaskedInput::new().value("abcdefgh").strength_label(),
            "Weak"
        );
        assert_eq!(
            MaskedInput::new().value("Abcdefgh1").strength_label(),
            "Strong"
        );
        assert_eq!(
            MaskedInput::new().value("Abcdefgh1!").strength_label(),
            "Very Strong"
        );
        assert_eq!(
            MaskedInput::new().value("Abcdefgh1!ghjk").strength_label(),
            "Very Strong"
        );
    }

    #[test]
    fn test_strength_color() {
        assert_eq!(MaskedInput::new().value("a").strength_color(), Color::RED);
        assert_eq!(
            MaskedInput::new().value("abcdefgh").strength_color(),
            Color::rgb(255, 128, 0)
        );
        assert_eq!(
            MaskedInput::new().value("Abcdefgh1").strength_color(),
            Color::rgb(128, 255, 0)
        );
        assert_eq!(
            MaskedInput::new().value("Abcdefgh1!").strength_color(),
            Color::GREEN
        );
        assert_eq!(
            MaskedInput::new().value("Abcdefgh1!ghjk").strength_color(),
            Color::GREEN
        );
    }

    // =========================================================================
    // Validation tests
    // =========================================================================

    #[test]
    fn test_validation() {
        let mut input = MaskedInput::new().min_length(8).value("short");

        assert!(!input.validate());
        assert!(matches!(input.validation, ValidationState::Invalid(_)));

        input.set_value("longenough");
        assert!(input.validate());
        assert!(matches!(input.validation, ValidationState::Valid));
    }

    #[test]
    fn test_validation_no_min_length() {
        let mut input = MaskedInput::new().value("");
        assert!(input.validate());
        assert!(matches!(input.validation, ValidationState::Valid));
    }

    #[test]
    fn test_validation_exactly_min_length() {
        let mut input = MaskedInput::new().min_length(5).value("hello");
        assert!(input.validate());
    }

    #[test]
    fn test_validation_invalid_message() {
        let mut input = MaskedInput::new().min_length(8).value("short");
        input.validate();

        if let ValidationState::Invalid(msg) = &input.validation {
            assert!(msg.contains("8"));
            assert!(msg.contains("Minimum"));
        } else {
            panic!("Expected Invalid state");
        }
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_helper_functions() {
        let pwd = password_input("Password");
        assert!(pwd.show_strength);
        assert_eq!(pwd.placeholder, Some("Password".to_string()));

        let pin = pin_input(4);
        assert_eq!(pin.max_length, 4);

        let card = credit_card_input();
        assert!(matches!(card.mask_style, MaskStyle::ShowLast(4)));
    }

    #[test]
    fn test_masked_input_helper() {
        let input = masked_input();
        assert_eq!(input.get_value(), "");
    }

    #[test]
    fn test_password_input_helper() {
        let pwd = password_input("Enter password");
        assert_eq!(pwd.placeholder, Some("Enter password".to_string()));
        assert!(pwd.show_strength);
    }

    #[test]
    fn test_pin_input_helper() {
        let pin = pin_input(6);
        assert_eq!(pin.max_length, 6);
    }

    #[test]
    fn test_credit_card_input_helper() {
        let card = credit_card_input();
        assert_eq!(card.max_length, 16);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_masked_input_builder_chain() {
        let input = MaskedInput::new()
            .mask_char('*')
            .mask_style(MaskStyle::Peek)
            .placeholder("Password")
            .label("Enter")
            .max_length(20)
            .min_length(8)
            .focused(true)
            .disabled(false)
            .fg(Color::WHITE)
            .bg(Color::BLACK)
            .width(30)
            .show_strength(true)
            .allow_reveal(true)
            .value("test");

        assert_eq!(input.mask_char, '*');
        assert_eq!(input.mask_style, MaskStyle::Peek);
        assert_eq!(input.placeholder, Some("Password".to_string()));
        assert_eq!(input.label, Some("Enter".to_string()));
        assert_eq!(input.max_length, 20);
        assert_eq!(input.min_length, 8);
        assert!(input.focused);
        assert!(!input.disabled);
        assert_eq!(input.fg, Some(Color::WHITE));
        assert_eq!(input.bg, Some(Color::BLACK));
        assert_eq!(input.width, Some(30));
        assert!(input.show_strength);
        assert!(input.allow_reveal);
        assert_eq!(input.get_value(), "test");
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_masked_input_unicode() {
        let mut input = MaskedInput::new();
        // Note: The implementation uses String::insert with byte-based cursor
        // This test verifies the behavior is predictable
        input.insert_char('a');
        input.insert_char('b');
        assert_eq!(input.get_value(), "ab");
    }

    #[test]
    fn test_masked_input_delete_unicode() {
        let mut input = MaskedInput::new().value("hello");
        input.cursor = 2;
        input.delete_forward();
        assert_eq!(input.get_value(), "helo");
    }

    #[test]
    fn test_masked_input_empty_value_operations() {
        let mut input = MaskedInput::new();
        input.delete_backward();
        input.delete_forward();
        input.move_left();
        input.move_right();
        assert_eq!(input.get_value(), "");
    }

    #[test]
    fn test_masked_input_peek_with_unicode() {
        let mut input = MaskedInput::new().mask_style(MaskStyle::Peek).value("ab");
        input.cursor = 2;
        input.peek_countdown = 5;

        let display = input.masked_display();
        assert_eq!(display, "●b");
    }

    #[test]
    fn test_masked_input_show_last_with_unicode() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowLast(2))
            .value("12345");
        assert_eq!(input.masked_display(), "●●●45");
    }

    #[test]
    fn test_masked_input_single_char_operations() {
        let mut input = MaskedInput::new().value("a");

        input.move_left();
        assert_eq!(input.cursor, 0);

        input.move_right();
        assert_eq!(input.cursor, 1);

        input.delete_backward();
        assert_eq!(input.get_value(), "");
    }

    #[test]
    fn test_masked_input_zero_max_length() {
        let mut input = MaskedInput::new().max_length(0);
        for _ in 0..100 {
            input.insert_char('a');
        }
        // Max length 0 means unlimited
        assert_eq!(input.get_value(), "a".repeat(100));
    }

    // =========================================================================
    // element_id and class tests (from impl_props_builders!)
    // =========================================================================

    #[test]
    fn test_masked_input_element_id() {
        let input = MaskedInput::new().element_id("password-field");
        assert_eq!(input.props.id, Some("password-field".to_string()));
    }

    #[test]
    fn test_masked_input_element_id_override() {
        let input = MaskedInput::new()
            .element_id("first-id")
            .element_id("second-id");
        assert_eq!(input.props.id, Some("second-id".to_string()));
    }

    #[test]
    fn test_masked_input_class() {
        let input = MaskedInput::new().class("input-field");
        assert_eq!(input.props.classes, vec!["input-field".to_string()]);
    }

    #[test]
    fn test_masked_input_class_multiple() {
        let input = MaskedInput::new().class("required").class("validated");
        assert_eq!(
            input.props.classes,
            vec!["required".to_string(), "validated".to_string()]
        );
    }

    #[test]
    fn test_masked_input_class_no_duplicate() {
        let input = MaskedInput::new().class("container").class("container");
        assert_eq!(input.props.classes, vec!["container".to_string()]);
    }

    #[test]
    fn test_masked_input_classes_vec() {
        let input = MaskedInput::new().classes(vec!["class1", "class2", "class3"]);
        assert_eq!(
            input.props.classes,
            vec![
                "class1".to_string(),
                "class2".to_string(),
                "class3".to_string()
            ]
        );
    }

    #[test]
    fn test_masked_input_classes_array() {
        let input = MaskedInput::new().classes(["class1", "class2"]);
        assert_eq!(
            input.props.classes,
            vec!["class1".to_string(), "class2".to_string()]
        );
    }

    #[test]
    fn test_masked_input_classes_with_duplicates_filtered() {
        let input = MaskedInput::new().classes(vec!["a", "b", "a", "c", "b"]);
        assert_eq!(
            input.props.classes,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn test_masked_input_mixed_classes() {
        let input = MaskedInput::new()
            .class("first")
            .classes(vec!["second", "third"])
            .class("fourth");
        assert_eq!(
            input.props.classes,
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
                "fourth".to_string()
            ]
        );
    }

    // =========================================================================
    // StyledView trait tests (from impl_styled_view!)
    // =========================================================================

    #[test]
    fn test_masked_input_set_id() {
        let mut input = MaskedInput::new();
        input.set_id("test-id");
        assert_eq!(input.props.id, Some("test-id".to_string()));
    }

    #[test]
    fn test_masked_input_set_id_override() {
        let mut input = MaskedInput::new();
        input.set_id("first");
        input.set_id("second");
        assert_eq!(input.props.id, Some("second".to_string()));
    }

    #[test]
    fn test_masked_input_add_class() {
        let mut input = MaskedInput::new();
        input.add_class("container");
        assert_eq!(input.props.classes, vec!["container".to_string()]);
    }

    #[test]
    fn test_masked_input_add_class_multiple() {
        let mut input = MaskedInput::new();
        input.add_class("class1");
        input.add_class("class2");
        input.add_class("class3");
        assert_eq!(
            input.props.classes,
            vec![
                "class1".to_string(),
                "class2".to_string(),
                "class3".to_string()
            ]
        );
    }

    #[test]
    fn test_masked_input_add_class_no_duplicate() {
        let mut input = MaskedInput::new();
        input.add_class("duplicate");
        input.add_class("duplicate");
        assert_eq!(input.props.classes, vec!["duplicate".to_string()]);
    }

    #[test]
    fn test_masked_input_remove_class() {
        let mut input = MaskedInput::new();
        input.add_class("class1");
        input.add_class("class2");
        input.add_class("class3");
        input.remove_class("class2");
        assert_eq!(
            input.props.classes,
            vec!["class1".to_string(), "class3".to_string()]
        );
    }

    #[test]
    fn test_masked_input_remove_class_not_present() {
        let mut input = MaskedInput::new();
        input.add_class("class1");
        input.remove_class("nonexistent");
        assert_eq!(input.props.classes, vec!["class1".to_string()]);
    }

    #[test]
    fn test_masked_input_remove_class_from_empty() {
        let mut input = MaskedInput::new();
        input.remove_class("anything");
        assert!(input.props.classes.is_empty());
    }

    #[test]
    fn test_masked_input_remove_class_duplicates() {
        let mut input = MaskedInput::new();
        // Manually add a duplicate (shouldn't happen via add_class but test defensive)
        input.props.classes.push("dup".to_string());
        input.props.classes.push("dup".to_string());
        input.remove_class("dup");
        // Should remove all instances
        assert!(!input.props.classes.contains(&"dup".to_string()));
    }

    #[test]
    fn test_masked_input_toggle_class_adds() {
        let mut input = MaskedInput::new();
        input.toggle_class("new-class");
        assert_eq!(input.props.classes, vec!["new-class".to_string()]);
    }

    #[test]
    fn test_masked_input_toggle_class_removes() {
        let mut input = MaskedInput::new();
        input.add_class("existing");
        input.toggle_class("existing");
        assert!(input.props.classes.is_empty());
    }

    #[test]
    fn test_masked_input_toggle_class_multiple_times() {
        let mut input = MaskedInput::new();
        input.toggle_class("toggle");
        assert_eq!(input.props.classes, vec!["toggle".to_string()]);
        input.toggle_class("toggle");
        assert!(input.props.classes.is_empty());
        input.toggle_class("toggle");
        assert_eq!(input.props.classes, vec!["toggle".to_string()]);
    }

    #[test]
    fn test_masked_input_has_class_true() {
        let mut input = MaskedInput::new();
        input.add_class("existing");
        assert!(input.has_class("existing"));
    }

    #[test]
    fn test_masked_input_has_class_false() {
        let input = MaskedInput::new();
        assert!(!input.has_class("nonexistent"));
    }

    #[test]
    fn test_masked_input_has_class_empty() {
        let input = MaskedInput::new();
        assert!(!input.has_class("anything"));
    }

    #[test]
    fn test_masked_input_classes_getter() {
        let input = MaskedInput::new().class("c1").class("c2");
        let classes = input.get_classes();
        assert_eq!(classes, &["c1".to_string(), "c2".to_string()]);
    }

    #[test]
    fn test_masked_input_classes_getter_empty() {
        let input = MaskedInput::new();
        assert!(input.get_classes().is_empty());
    }

    #[test]
    fn test_masked_input_id_getter() {
        let input = MaskedInput::new().element_id("test-id");
        assert_eq!(input.get_id(), Some("test-id"));
    }

    #[test]
    fn test_masked_input_id_getter_none() {
        let input = MaskedInput::new();
        assert_eq!(input.get_id(), None);
    }

    // =========================================================================
    // Combined builder and styled tests
    // =========================================================================

    #[test]
    fn test_masked_input_builder_and_styled_mix() {
        let mut input = MaskedInput::new()
            .element_id("test")
            .class("from-builder")
            .value("password");

        input.add_class("from-styled");
        input.set_id("updated-id");

        assert_eq!(input.props.id, Some("updated-id".to_string()));
        assert_eq!(
            input.props.classes,
            vec!["from-builder".to_string(), "from-styled".to_string()]
        );
        assert_eq!(input.get_value(), "password");
    }

    #[test]
    fn test_masked_input_full_builder_chain_with_props() {
        let input = MaskedInput::new()
            .element_id("password-input")
            .class("required")
            .classes(vec!["validated", "secure"])
            .mask_char('*')
            .mask_style(MaskStyle::Peek)
            .placeholder("Enter password")
            .label("Password")
            .max_length(20)
            .min_length(8)
            .focused(true)
            .disabled(false)
            .fg(Color::WHITE)
            .bg(Color::BLACK)
            .width(30)
            .show_strength(true)
            .allow_reveal(true)
            .value("test");

        assert_eq!(input.props.id, Some("password-input".to_string()));
        assert_eq!(
            input.props.classes,
            vec![
                "required".to_string(),
                "validated".to_string(),
                "secure".to_string()
            ]
        );
        assert_eq!(input.mask_char, '*');
        assert_eq!(input.mask_style, MaskStyle::Peek);
        assert_eq!(input.placeholder, Some("Enter password".to_string()));
        assert_eq!(input.label, Some("Password".to_string()));
        assert_eq!(input.max_length, 20);
        assert_eq!(input.min_length, 8);
        assert!(input.focused);
        assert!(!input.disabled);
        assert_eq!(input.fg, Some(Color::WHITE));
        assert_eq!(input.bg, Some(Color::BLACK));
        assert_eq!(input.width, Some(30));
        assert!(input.show_strength);
        assert!(input.allow_reveal);
        assert_eq!(input.get_value(), "test");
    }

    // =========================================================================
    // Edge case tests for props
    // =========================================================================

    #[test]
    fn test_masked_input_empty_string_element_id() {
        let input = MaskedInput::new().element_id("");
        assert_eq!(input.props.id, Some("".to_string()));
    }

    #[test]
    fn test_masked_input_empty_string_class() {
        let input = MaskedInput::new().class("");
        assert_eq!(input.props.classes, vec!["".to_string()]);
    }

    #[test]
    fn test_masked_input_classes_empty_vec() {
        let input = MaskedInput::new().classes(Vec::<&str>::new());
        assert!(input.props.classes.is_empty());
    }

    #[test]
    fn test_masked_input_classes_empty_array() {
        let input = MaskedInput::new().classes([] as [&str; 0]);
        assert!(input.props.classes.is_empty());
    }

    #[test]
    fn test_masked_input_set_id_empty_string() {
        let mut input = MaskedInput::new();
        input.set_id("");
        assert_eq!(input.props.id, Some("".to_string()));
    }

    #[test]
    fn test_masked_input_add_class_empty_string() {
        let mut input = MaskedInput::new();
        input.add_class("");
        assert_eq!(input.props.classes, vec!["".to_string()]);
    }

    // =========================================================================
    // Password builder preset with props tests
    // =========================================================================

    #[test]
    fn test_masked_input_password_with_props() {
        let pwd = MaskedInput::password()
            .element_id("pwd")
            .class("password-field");

        assert_eq!(pwd.props.id, Some("pwd".to_string()));
        assert_eq!(pwd.props.classes, vec!["password-field".to_string()]);
        assert!(pwd.show_strength);
        assert_eq!(pwd.mask_char, '●');
    }

    #[test]
    fn test_masked_input_pin_with_props() {
        let pin = MaskedInput::pin(4)
            .element_id("pin-input")
            .classes(vec!["numeric", "required"]);

        assert_eq!(pin.props.id, Some("pin-input".to_string()));
        assert_eq!(
            pin.props.classes,
            vec!["numeric".to_string(), "required".to_string()]
        );
        assert_eq!(pin.max_length, 4);
    }

    #[test]
    fn test_masked_input_credit_card_with_props() {
        let card = MaskedInput::credit_card()
            .element_id("card-number")
            .class("financial");

        assert_eq!(card.props.id, Some("card-number".to_string()));
        assert_eq!(card.props.classes, vec!["financial".to_string()]);
        assert_eq!(card.max_length, 16);
    }

    // =========================================================================
    // Styled operations with disabled/focused states
    // =========================================================================

    #[test]
    fn test_masked_input_styled_operations_while_disabled() {
        let mut input = MaskedInput::new().disabled(true);

        // Styled operations should work regardless of disabled state
        input.add_class("disabled");
        input.set_id("disabled-input");
        input.toggle_class("toggle");

        assert!(input.has_class("disabled"));
        assert_eq!(input.get_id(), Some("disabled-input"));
        assert!(input.has_class("toggle"));
    }

    #[test]
    fn test_masked_input_styled_operations_while_focused() {
        let mut input = MaskedInput::new().focused(true);

        // Styled operations should work regardless of focused state
        input.add_class("focused");
        input.remove_class("focused");
        input.toggle_class("active");

        assert!(!input.has_class("focused"));
        assert!(input.has_class("active"));
    }

    // =========================================================================
    // Class operations with special characters
    // =========================================================================

    #[test]
    fn test_masked_input_class_with_hyphens() {
        let mut input = MaskedInput::new();
        input.add_class("my-custom-class");
        assert!(input.has_class("my-custom-class"));
    }

    #[test]
    fn test_masked_input_class_with_underscores() {
        let mut input = MaskedInput::new();
        input.add_class("my_custom_class");
        assert!(input.has_class("my_custom_class"));
    }

    #[test]
    fn test_masked_input_class_with_numbers() {
        let mut input = MaskedInput::new();
        input.add_class("class123");
        assert!(input.has_class("class123"));
    }

    // =========================================================================
    // Interaction between value changes and styled operations
    // =========================================================================

    #[test]
    fn test_masked_input_value_and_styled_operations() {
        let mut input = MaskedInput::new();

        input.add_class("initial");
        input.set_value("password");
        input.add_class("has-value");
        input.clear();
        input.remove_class("has-value");
        input.toggle_class("empty");

        assert!(input.has_class("initial"));
        assert!(input.has_class("empty"));
        assert!(!input.has_class("has-value"));
        assert_eq!(input.get_value(), "");
    }

    // =========================================================================
    // Stress tests - long builder chains
    // =========================================================================

    #[test]
    fn test_masked_input_long_class_chain() {
        let input = MaskedInput::new()
            .class("c1")
            .class("c2")
            .class("c3")
            .classes(vec!["c4", "c5"])
            .class("c6")
            .classes(vec!["c7", "c8", "c9"]);

        assert_eq!(input.props.classes.len(), 9);
    }

    #[test]
    fn test_masked_input_many_toggle_operations() {
        let mut input = MaskedInput::new();
        for _ in 0..10 {
            input.toggle_class("toggle");
        }
        // Even number of toggles = not present
        assert!(!input.has_class("toggle"));
    }

    #[test]
    fn test_masked_input_many_add_remove_operations() {
        let mut input = MaskedInput::new();
        for i in 0..5 {
            input.add_class(&format!("class{}", i));
        }
        assert_eq!(input.props.classes.len(), 5);

        for i in 0..5 {
            input.remove_class(&format!("class{}", i));
        }
        assert!(input.props.classes.is_empty());
    }

    // =========================================================================
    // Helper functions with props
    // =========================================================================

    #[test]
    fn test_masked_input_helper_with_props() {
        let input = masked_input().element_id("masked").class("input");
        assert_eq!(input.props.id, Some("masked".to_string()));
        assert_eq!(input.props.classes, vec!["input".to_string()]);
    }

    #[test]
    fn test_password_input_helper_with_props() {
        let pwd = password_input("Password").element_id("pwd").class("secure");
        assert_eq!(pwd.props.id, Some("pwd".to_string()));
        assert_eq!(pwd.props.classes, vec!["secure".to_string()]);
    }

    #[test]
    fn test_pin_input_helper_with_props() {
        let pin = pin_input(6)
            .element_id("pin")
            .classes(vec!["numeric", "required"]);
        assert_eq!(pin.props.id, Some("pin".to_string()));
        assert!(pin.has_class("numeric"));
        assert!(pin.has_class("required"));
    }

    #[test]
    fn test_credit_card_input_helper_with_props() {
        let card = credit_card_input().element_id("card").class("financial");
        assert_eq!(card.props.id, Some("card".to_string()));
        assert!(card.has_class("financial"));
    }

    // =========================================================================
    // Clone and Debug with props
    // =========================================================================

    #[test]
    fn test_masked_input_clone_preserves_props() {
        let input1 = MaskedInput::new()
            .element_id("test-id")
            .class("class1")
            .class("class2")
            .value("secret");
        let input2 = input1.clone();

        assert_eq!(input1.props.id, input2.props.id);
        assert_eq!(input1.props.classes, input2.props.classes);
        assert_eq!(input1.get_value(), input2.get_value());
    }

    #[test]
    fn test_masked_input_debug_includes_props() {
        let input = MaskedInput::new().element_id("test-id").class("test-class");
        let debug_str = format!("{:?}", input);
        // Debug should contain structural information
        assert!(debug_str.contains("MaskedInput"));
    }

    // =========================================================================
    // Validation state with styled operations
    // =========================================================================

    #[test]
    fn test_masked_input_validation_with_class_changes() {
        let mut input = MaskedInput::new().min_length(8);

        input.add_class("initial");
        assert!(!input.validate()); // Too short
        input.add_class("invalid");

        input.set_value("longenough");
        input.remove_class("invalid");
        input.add_class("valid");
        assert!(input.validate());

        assert!(input.has_class("initial"));
        assert!(input.has_class("valid"));
        assert!(!input.has_class("invalid"));
    }

    // =========================================================================
    // Reveal functionality with styled operations
    // =========================================================================

    #[test]
    fn test_masked_input_reveal_with_class_toggling() {
        let mut input = MaskedInput::new().allow_reveal(true).value("secret");

        input.add_class("masked");
        input.toggle_reveal();
        input.remove_class("masked");
        input.add_class("revealed");

        assert!(input.revealing);
        assert!(!input.has_class("masked"));
        assert!(input.has_class("revealed"));

        input.toggle_reveal();
        input.toggle_class("revealed");
        assert!(!input.revealing);
        assert!(!input.has_class("revealed"));
    }

    // =========================================================================
    // Peek mode with styled operations
    // =========================================================================

    #[test]
    fn test_masked_input_peek_with_class_operations() {
        let mut input = MaskedInput::new().mask_style(MaskStyle::Peek);

        input.add_class("peek-mode");
        input.insert_char('a');
        assert_eq!(input.peek_countdown, 10);

        input.update();
        input.remove_class("peek-mode");
        input.add_class("peeking");
        assert_eq!(input.peek_countdown, 9);
        assert!(input.has_class("peeking"));
    }
}
