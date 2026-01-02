//! Masked input widget for passwords and sensitive data
//!
//! Provides input fields that hide or mask the entered text, perfect for
//! passwords, PINs, credit card numbers, and other sensitive information.
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
                    let last_char = self.value.chars().nth(self.cursor - 1).unwrap_or(' ');
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
            let cursor_pos = self.cursor.min(padded.len());
            let before: String = padded.chars().take(cursor_pos).collect();
            let cursor_char = padded.chars().nth(cursor_pos).unwrap_or(' ');
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

    #[test]
    fn test_masked_input_new() {
        let input = MaskedInput::new();
        assert_eq!(input.get_value(), "");
        assert_eq!(input.mask_char, '●');
    }

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
    fn test_masked_input_delete() {
        let mut input = MaskedInput::new().value("hello");
        input.delete_backward();
        assert_eq!(input.get_value(), "hell");
    }

    #[test]
    fn test_masked_input_max_length() {
        let mut input = MaskedInput::new().max_length(4);
        for c in "12345678".chars() {
            input.insert_char(c);
        }
        assert_eq!(input.get_value(), "1234");
    }

    #[test]
    fn test_masked_display_full() {
        let input = MaskedInput::new().value("secret");
        assert_eq!(input.masked_display(), "●●●●●●");
    }

    #[test]
    fn test_masked_display_show_last() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowLast(4))
            .value("1234567890");
        assert_eq!(input.masked_display(), "●●●●●●7890");
    }

    #[test]
    fn test_masked_display_show_first() {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::ShowFirst(4))
            .value("1234567890");
        assert_eq!(input.masked_display(), "1234●●●●●●");
    }

    #[test]
    fn test_password_strength() {
        let weak = MaskedInput::new().value("abc");
        assert_eq!(weak.password_strength(), 0);

        let strong = MaskedInput::new().value("MyP@ssw0rd123!");
        assert!(strong.password_strength() >= 3);
    }

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
    fn test_validation() {
        let mut input = MaskedInput::new().min_length(8).value("short");

        assert!(!input.validate());
        assert!(matches!(input.validation, ValidationState::Invalid(_)));

        input.set_value("longenough");
        assert!(input.validate());
        assert!(matches!(input.validation, ValidationState::Valid));
    }

    #[test]
    fn test_cursor_movement() {
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

    #[test]
    fn test_helper_functions() {
        let pwd = password_input("Password");
        assert!(pwd.show_strength);

        let pin = pin_input(4);
        assert_eq!(pin.max_length, 4);

        let card = credit_card_input();
        assert!(matches!(card.mask_style, MaskStyle::ShowLast(4)));
    }

    #[test]
    fn test_disabled() {
        let mut input = MaskedInput::new().disabled(true);
        input.insert_char('a');
        assert_eq!(input.get_value(), "");
    }
}
