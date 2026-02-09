//! Number input widget with increment/decrement controls
//!
//! Provides a numeric input field with:
//! - Up/Down arrow key controls for increment/decrement
//! - Direct numeric entry
//! - Min/max value constraints
//! - Configurable step size and precision
//! - Optional prefix/suffix display (e.g., "$", "%")

use crate::event::Key;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_view_meta, impl_widget_builders};
/// A number input widget with increment/decrement controls
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{number_input, NumberInput};
///
/// // Basic number input
/// let input = number_input().value(42.0);
///
/// // With constraints
/// let input = number_input()
///     .value(50.0)
///     .min(0.0)
///     .max(100.0)
///     .step(5.0);
///
/// // Currency input
/// let price = number_input()
///     .value(19.99)
///     .prefix("$")
///     .precision(2);
///
/// // Percentage input
/// let percent = number_input()
///     .value(75.0)
///     .suffix("%")
///     .min(0.0)
///     .max(100.0);
/// ```
#[derive(Debug, Clone)]
pub struct NumberInput {
    /// Current numeric value
    value: f64,
    /// Minimum allowed value
    min: Option<f64>,
    /// Maximum allowed value
    max: Option<f64>,
    /// Increment/decrement step
    step: f64,
    /// Decimal precision for display
    precision: u8,
    /// Prefix to display before value (e.g., "$")
    prefix: Option<String>,
    /// Suffix to display after value (e.g., "%")
    suffix: Option<String>,
    /// Whether in direct editing mode
    editing: bool,
    /// Buffer for direct text input
    input_buffer: String,
    /// Cursor position in input buffer (character index)
    cursor: usize,
    /// Width of the input field (characters)
    width: u16,
    /// Whether to show +/- buttons
    show_buttons: bool,
    /// Widget state (focused, disabled, colors)
    state: WidgetState,
    /// Widget props (id, classes)
    props: WidgetProps,
}

impl NumberInput {
    /// Create a new number input widget
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: None,
            max: None,
            step: 1.0,
            precision: 0,
            prefix: None,
            suffix: None,
            editing: false,
            input_buffer: String::new(),
            cursor: 0,
            width: 10,
            show_buttons: true,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set the current value
    pub fn value(mut self, value: f64) -> Self {
        self.value = self.clamp_value(value);
        self
    }

    /// Set minimum allowed value
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self.value = self.clamp_value(self.value);
        self
    }

    /// Set maximum allowed value
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self.value = self.clamp_value(self.value);
        self
    }

    /// Set the step size for increment/decrement
    pub fn step(mut self, step: f64) -> Self {
        self.step = step.abs();
        self
    }

    /// Set decimal precision (number of decimal places to display)
    pub fn precision(mut self, precision: u8) -> Self {
        self.precision = precision;
        self
    }

    /// Set prefix text (displayed before the value)
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set suffix text (displayed after the value)
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set the width of the input field
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(5);
        self
    }

    /// Show or hide the +/- buttons
    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    // =========================================================================
    // Value manipulation
    // =========================================================================

    /// Get the current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Get the current value as an integer
    pub fn get_int(&self) -> i64 {
        self.value.round() as i64
    }

    /// Set the value (with clamping)
    pub fn set_value(&mut self, value: f64) {
        self.value = self.clamp_value(value);
        self.editing = false;
    }

    /// Increment the value by step
    pub fn increment(&mut self) {
        self.set_value(self.value + self.step);
    }

    /// Decrement the value by step
    pub fn decrement(&mut self) {
        self.set_value(self.value - self.step);
    }

    /// Increment by large step (10x normal step)
    pub fn increment_large(&mut self) {
        self.set_value(self.value + self.step * 10.0);
    }

    /// Decrement by large step (10x normal step)
    pub fn decrement_large(&mut self) {
        self.set_value(self.value - self.step * 10.0);
    }

    /// Set to minimum value (if defined)
    pub fn set_to_min(&mut self) {
        if let Some(min) = self.min {
            self.set_value(min);
        }
    }

    /// Set to maximum value (if defined)
    pub fn set_to_max(&mut self) {
        if let Some(max) = self.max {
            self.set_value(max);
        }
    }

    /// Clamp value to min/max constraints
    fn clamp_value(&self, value: f64) -> f64 {
        let mut v = value;
        if let Some(min) = self.min {
            v = v.max(min);
        }
        if let Some(max) = self.max {
            v = v.min(max);
        }
        v
    }

    // =========================================================================
    // Display formatting
    // =========================================================================

    /// Format the current value for display
    fn format_value(&self) -> String {
        format!("{:.prec$}", self.value, prec = self.precision as usize)
    }

    /// Get the full display string (with prefix/suffix)
    pub fn display_string(&self) -> String {
        let value_str = if self.editing {
            self.input_buffer.clone()
        } else {
            self.format_value()
        };

        let mut result = String::new();
        if let Some(ref prefix) = self.prefix {
            result.push_str(prefix);
        }
        result.push_str(&value_str);
        if let Some(ref suffix) = self.suffix {
            result.push_str(suffix);
        }
        result
    }

    // =========================================================================
    // Editing mode
    // =========================================================================

    /// Enter direct editing mode
    pub fn start_editing(&mut self) {
        if !self.editing {
            self.editing = true;
            self.input_buffer = self.format_value();
            self.cursor = self.input_buffer.chars().count();
        }
    }

    /// Commit the edited value and exit editing mode
    pub fn commit_edit(&mut self) {
        if self.editing {
            if let Ok(value) = self.input_buffer.parse::<f64>() {
                self.value = self.clamp_value(value);
            }
            self.editing = false;
            self.input_buffer.clear();
        }
    }

    /// Cancel editing and restore original value
    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.input_buffer.clear();
    }

    /// Check if in editing mode
    pub fn is_editing(&self) -> bool {
        self.editing
    }

    // =========================================================================
    // UTF-8 safe buffer operations
    // =========================================================================

    fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
        s.char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(s.len())
    }

    fn buffer_char_count(&self) -> usize {
        self.input_buffer.chars().count()
    }

    fn insert_char_at_cursor(&mut self, ch: char) {
        let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
        self.input_buffer.insert(byte_idx, ch);
        self.cursor += 1;
    }

    fn delete_char_before_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
            // Get char directly from byte index for O(1) instead of O(n²) with .chars().nth()
            if let Some(ch) = self.input_buffer[byte_idx..].chars().next() {
                self.input_buffer.drain(byte_idx..byte_idx + ch.len_utf8());
            }
        }
    }

    fn delete_char_at_cursor(&mut self) {
        let char_count = self.buffer_char_count();
        if self.cursor < char_count {
            let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
            // Get char directly from byte index for O(1) instead of O(n²) with .chars().nth()
            if let Some(ch) = self.input_buffer[byte_idx..].chars().next() {
                self.input_buffer.drain(byte_idx..byte_idx + ch.len_utf8());
            }
        }
    }

    // =========================================================================
    // Key handling
    // =========================================================================

    /// Handle key input, returns true if value changed or needs redraw
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            // Increment/Decrement
            Key::Up | Key::Char('k') => {
                if self.editing {
                    self.commit_edit();
                }
                self.increment();
                true
            }
            Key::Down | Key::Char('j') => {
                if self.editing {
                    self.commit_edit();
                }
                self.decrement();
                true
            }

            // Large increment/decrement
            Key::PageUp => {
                if self.editing {
                    self.commit_edit();
                }
                self.increment_large();
                true
            }
            Key::PageDown => {
                if self.editing {
                    self.commit_edit();
                }
                self.decrement_large();
                true
            }

            // Min/Max
            Key::Home => {
                if self.editing {
                    self.commit_edit();
                }
                self.set_to_min();
                true
            }
            Key::End => {
                if self.editing {
                    self.commit_edit();
                }
                self.set_to_max();
                true
            }

            // Enter editing mode or commit
            Key::Enter => {
                if self.editing {
                    self.commit_edit();
                } else {
                    self.start_editing();
                }
                true
            }

            // Cancel editing
            Key::Escape => {
                if self.editing {
                    self.cancel_edit();
                    true
                } else {
                    false
                }
            }

            // Direct numeric input
            Key::Char(c) if c.is_ascii_digit() || *c == '.' || *c == '-' => {
                if !self.editing {
                    self.start_editing();
                    self.input_buffer.clear();
                    self.cursor = 0;
                }

                // Validate input
                let valid = match c {
                    '-' => self.cursor == 0 && !self.input_buffer.contains('-'),
                    '.' => !self.input_buffer.contains('.'),
                    _ => true,
                };

                if valid {
                    self.insert_char_at_cursor(*c);
                }
                true
            }

            // Backspace in editing mode
            Key::Backspace => {
                if self.editing {
                    self.delete_char_before_cursor();
                    true
                } else {
                    false
                }
            }

            // Delete in editing mode
            Key::Delete => {
                if self.editing {
                    self.delete_char_at_cursor();
                    true
                } else {
                    false
                }
            }

            // Cursor movement in editing mode
            Key::Left => {
                if self.editing && self.cursor > 0 {
                    self.cursor -= 1;
                    true
                } else {
                    false
                }
            }
            Key::Right => {
                if self.editing && self.cursor < self.buffer_char_count() {
                    self.cursor += 1;
                    true
                } else {
                    false
                }
            }

            _ => false,
        }
    }
}

impl Default for NumberInput {
    fn default() -> Self {
        Self::new()
    }
}

impl View for NumberInput {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let (fg, bg) =
            self.state
                .resolve_colors_interactive(ctx.style, Color::WHITE, Color::rgb(60, 60, 60));

        let mut x = area.x;

        // Draw prefix
        if let Some(ref prefix) = self.prefix {
            ctx.draw_text(x, area.y, prefix, Color::rgb(150, 150, 150));
            x += prefix.chars().count() as u16;
        }

        // Draw value (or input buffer when editing)
        let value_str = if self.editing {
            &self.input_buffer
        } else {
            &self.format_value()
        };

        for (i, ch) in value_str.chars().enumerate() {
            if x >= area.x + area.width {
                break;
            }

            let is_cursor_at_pos = self.editing && self.state.focused && i == self.cursor;

            let mut cell = Cell::new(ch);
            if is_cursor_at_pos {
                cell.fg = Some(Color::BLACK);
                cell.bg = Some(Color::WHITE);
            } else {
                cell.fg = Some(fg);
                cell.bg = Some(bg);
            }

            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }

        // Draw cursor at end if editing and cursor is at end
        if self.editing
            && self.state.focused
            && self.cursor >= value_str.chars().count()
            && x < area.x + area.width
        {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.fg = Some(Color::BLACK);
            cursor_cell.bg = Some(Color::WHITE);
            ctx.buffer.set(x, area.y, cursor_cell);
            x += 1;
        }

        // Draw suffix
        if let Some(ref suffix) = self.suffix {
            ctx.draw_text(x, area.y, suffix, Color::rgb(150, 150, 150));
            x += suffix.chars().count() as u16;
        }

        // Draw buttons if enabled and space available
        if self.show_buttons && x + 4 <= area.x + area.width {
            let button_x = area.x + area.width - 4;
            let button_fg = if self.state.disabled {
                Color::rgb(80, 80, 80)
            } else {
                Color::rgb(150, 150, 150)
            };
            ctx.draw_text(button_x, area.y, " -+", button_fg);
        }
    }

    impl_view_meta!("NumberInput");
}

impl_styled_view!(NumberInput);
impl_widget_builders!(NumberInput);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;

    // =========================================================================
    // Constructor Tests
    // =========================================================================

    #[test]
    fn test_number_input_new_creates_default_widget() {
        let input = NumberInput::new();
        assert_eq!(input.value, 0.0);
        assert!(input.min.is_none());
        assert!(input.max.is_none());
        assert_eq!(input.step, 1.0);
        assert_eq!(input.precision, 0);
        assert!(input.prefix.is_none());
        assert!(input.suffix.is_none());
        assert!(!input.editing);
        assert_eq!(input.input_buffer, "");
        assert_eq!(input.cursor, 0);
        assert_eq!(input.width, 10);
        assert!(input.show_buttons);
    }

    #[test]
    fn test_number_input_default_trait() {
        let input = NumberInput::default();
        assert_eq!(input.value, 0.0);
        assert_eq!(input.step, 1.0);
    }

    // =========================================================================
    // Builder Method Tests
    // =========================================================================

    #[test]
    fn test_number_input_value_builder() {
        let input = NumberInput::new().value(42.0);
        assert_eq!(input.value, 42.0);
    }

    #[test]
    fn test_number_input_value_with_clamping() {
        let input = NumberInput::new().min(0.0).max(100.0).value(150.0);
        assert_eq!(input.value, 100.0); // Clamped to max

        let input = NumberInput::new().min(0.0).max(100.0).value(-50.0);
        assert_eq!(input.value, 0.0); // Clamped to min
    }

    #[test]
    fn test_number_input_min_builder() {
        let input = NumberInput::new().min(10.0);
        assert_eq!(input.min, Some(10.0));
    }

    #[test]
    fn test_number_input_min_clamps_current_value() {
        let input = NumberInput::new().value(5.0).min(10.0);
        assert_eq!(input.value, 10.0); // Clamped up to min
    }

    #[test]
    fn test_number_input_max_builder() {
        let input = NumberInput::new().max(100.0);
        assert_eq!(input.max, Some(100.0));
    }

    #[test]
    fn test_number_input_max_clamps_current_value() {
        let input = NumberInput::new().value(150.0).max(100.0);
        assert_eq!(input.value, 100.0); // Clamped down to max
    }

    #[test]
    fn test_number_input_step_builder() {
        let input = NumberInput::new().step(5.0);
        assert_eq!(input.step, 5.0);

        let input = NumberInput::new().step(-5.0);
        assert_eq!(input.step, 5.0); // Absolute value applied
    }

    #[test]
    fn test_number_input_precision_builder() {
        let input = NumberInput::new().precision(2);
        assert_eq!(input.precision, 2);
    }

    #[test]
    fn test_number_input_prefix_builder() {
        let input = NumberInput::new().prefix("$");
        assert_eq!(input.prefix, Some("$".to_string()));
    }

    #[test]
    fn test_number_input_suffix_builder() {
        let input = NumberInput::new().suffix("%");
        assert_eq!(input.suffix, Some("%".to_string()));
    }

    #[test]
    fn test_number_input_width_builder() {
        let input = NumberInput::new().width(20);
        assert_eq!(input.width, 20);

        let input = NumberInput::new().width(3);
        assert_eq!(input.width, 5); // Minimum width enforced
    }

    #[test]
    fn test_number_input_show_buttons_builder() {
        let input = NumberInput::new().show_buttons(true);
        assert!(input.show_buttons);

        let input = NumberInput::new().show_buttons(false);
        assert!(!input.show_buttons);
    }

    #[test]
    fn test_number_input_builder_chaining() {
        let input = NumberInput::new()
            .value(50.0)
            .min(0.0)
            .max(100.0)
            .step(5.0)
            .precision(2)
            .prefix("$")
            .suffix(" USD")
            .width(15)
            .show_buttons(true);

        assert_eq!(input.value, 50.0);
        assert_eq!(input.min, Some(0.0));
        assert_eq!(input.max, Some(100.0));
        assert_eq!(input.step, 5.0);
        assert_eq!(input.precision, 2);
        assert_eq!(input.prefix, Some("$".to_string()));
        assert_eq!(input.suffix, Some(" USD".to_string()));
        assert_eq!(input.width, 15);
        assert!(input.show_buttons);
    }

    // =========================================================================
    // Value Getter Tests
    // =========================================================================

    #[test]
    fn test_number_input_get_value() {
        let input = NumberInput::new().value(42.5);
        assert_eq!(input.get_value(), 42.5);
    }

    #[test]
    fn test_number_input_get_int() {
        let input = NumberInput::new().value(42.7);
        assert_eq!(input.get_int(), 43);

        let input = NumberInput::new().value(42.2);
        assert_eq!(input.get_int(), 42);
    }

    // =========================================================================
    // Value Setter Tests
    // =========================================================================

    #[test]
    fn test_number_input_set_value() {
        let mut input = NumberInput::new();
        input.set_value(42.0);
        assert_eq!(input.value, 42.0);
    }

    #[test]
    fn test_number_input_set_value_with_clamping() {
        let mut input = NumberInput::new().min(0.0).max(100.0);
        input.set_value(150.0);
        assert_eq!(input.value, 100.0);

        input.set_value(-50.0);
        assert_eq!(input.value, 0.0);
    }

    #[test]
    fn test_number_input_set_value_exits_editing_mode() {
        let mut input = NumberInput::new();
        input.editing = true;
        input.set_value(42.0);
        assert!(!input.editing);
    }

    // =========================================================================
    // Increment/Decrement Tests
    // =========================================================================

    #[test]
    fn test_number_input_increment() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        input.increment();
        assert_eq!(input.value, 15.0);
    }

    #[test]
    fn test_number_input_decrement() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        input.decrement();
        assert_eq!(input.value, 5.0);
    }

    #[test]
    fn test_number_input_increment_with_max() {
        let mut input = NumberInput::new().value(95.0).max(100.0).step(10.0);
        input.increment();
        assert_eq!(input.value, 100.0); // Clamped to max
    }

    #[test]
    fn test_number_input_decrement_with_min() {
        let mut input = NumberInput::new().value(5.0).min(0.0).step(10.0);
        input.decrement();
        assert_eq!(input.value, 0.0); // Clamped to min
    }

    #[test]
    fn test_number_input_increment_large() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        input.increment_large();
        assert_eq!(input.value, 60.0); // +10 * 5
    }

    #[test]
    fn test_number_input_decrement_large() {
        let mut input = NumberInput::new().value(60.0).step(5.0);
        input.decrement_large();
        assert_eq!(input.value, 10.0); // -10 * 5
    }

    #[test]
    fn test_number_input_set_to_min() {
        let mut input = NumberInput::new().value(50.0).min(10.0);
        input.set_to_min();
        assert_eq!(input.value, 10.0);
    }

    #[test]
    fn test_number_input_set_to_min_without_min() {
        let mut input = NumberInput::new().value(50.0);
        input.set_to_min();
        assert_eq!(input.value, 50.0); // No change when no min
    }

    #[test]
    fn test_number_input_set_to_max() {
        let mut input = NumberInput::new().value(50.0).max(100.0);
        input.set_to_max();
        assert_eq!(input.value, 100.0);
    }

    #[test]
    fn test_number_input_set_to_max_without_max() {
        let mut input = NumberInput::new().value(50.0);
        input.set_to_max();
        assert_eq!(input.value, 50.0); // No change when no max
    }

    // =========================================================================
    // Editing Mode Tests
    // =========================================================================

    #[test]
    fn test_number_input_start_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        assert!(input.editing);
        assert_eq!(input.input_buffer, "42");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_number_input_start_editing_already_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "100".to_string();
        input.cursor = 3;

        input.start_editing(); // Should not reset
        assert!(input.editing);
        assert_eq!(input.input_buffer, "100");
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_number_input_commit_edit_valid() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "100".to_string();
        input.commit_edit();
        assert!(!input.editing);
        assert_eq!(input.value, 100.0);
        assert_eq!(input.input_buffer, "");
    }

    #[test]
    fn test_number_input_commit_edit_invalid() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "abc".to_string();
        input.commit_edit();
        assert!(!input.editing);
        assert_eq!(input.value, 42.0); // Value unchanged
        assert_eq!(input.input_buffer, "");
    }

    #[test]
    fn test_number_input_commit_edit_with_clamping() {
        let mut input = NumberInput::new().value(50.0).min(0.0).max(100.0);
        input.start_editing();
        input.input_buffer = "150".to_string();
        input.commit_edit();
        assert_eq!(input.value, 100.0); // Clamped to max
    }

    #[test]
    fn test_number_input_cancel_edit() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "100".to_string();
        input.cancel_edit();
        assert!(!input.editing);
        assert_eq!(input.value, 42.0); // Original value preserved
        assert_eq!(input.input_buffer, "");
    }

    #[test]
    fn test_number_input_is_editing() {
        let mut input = NumberInput::new();
        assert!(!input.is_editing());

        input.start_editing();
        assert!(input.is_editing());

        input.cancel_edit();
        assert!(!input.is_editing());
    }

    // =========================================================================
    // Display String Tests
    // =========================================================================

    #[test]
    fn test_number_input_display_string_basic() {
        let input = NumberInput::new().value(42.0);
        assert_eq!(input.display_string(), "42");
    }

    #[test]
    fn test_number_input_display_string_with_precision() {
        let input = NumberInput::new().value(42.5678).precision(2);
        assert_eq!(input.display_string(), "42.57");
    }

    #[test]
    fn test_number_input_display_string_with_prefix() {
        let input = NumberInput::new().value(42.0).prefix("$");
        assert_eq!(input.display_string(), "$42");
    }

    #[test]
    fn test_number_input_display_string_with_suffix() {
        let input = NumberInput::new().value(42.0).suffix("%");
        assert_eq!(input.display_string(), "42%");
    }

    #[test]
    fn test_number_input_display_string_with_prefix_and_suffix() {
        let input = NumberInput::new().value(42.0).prefix("$").suffix(" USD");
        assert_eq!(input.display_string(), "$42 USD");
    }

    #[test]
    fn test_number_input_display_string_while_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "50".to_string();
        assert_eq!(input.display_string(), "50");
    }

    // =========================================================================
    // Key Handling Tests
    // =========================================================================

    #[test]
    fn test_number_input_handle_key_up() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        let handled = input.handle_key(&Key::Up);
        assert!(handled);
        assert_eq!(input.value, 15.0);
    }

    #[test]
    fn test_number_input_handle_key_char_k() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        let handled = input.handle_key(&Key::Char('k'));
        assert!(handled);
        assert_eq!(input.value, 15.0);
    }

    #[test]
    fn test_number_input_handle_key_down() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        let handled = input.handle_key(&Key::Down);
        assert!(handled);
        assert_eq!(input.value, 5.0);
    }

    #[test]
    fn test_number_input_handle_key_char_j() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        let handled = input.handle_key(&Key::Char('j'));
        assert!(handled);
        assert_eq!(input.value, 5.0);
    }

    #[test]
    fn test_number_input_handle_key_page_up() {
        let mut input = NumberInput::new().value(10.0).step(5.0);
        let handled = input.handle_key(&Key::PageUp);
        assert!(handled);
        assert_eq!(input.value, 60.0); // +10 * 5
    }

    #[test]
    fn test_number_input_handle_key_page_down() {
        let mut input = NumberInput::new().value(60.0).step(5.0);
        let handled = input.handle_key(&Key::PageDown);
        assert!(handled);
        assert_eq!(input.value, 10.0); // -10 * 5
    }

    #[test]
    fn test_number_input_handle_key_home() {
        let mut input = NumberInput::new().value(50.0).min(10.0);
        let handled = input.handle_key(&Key::Home);
        assert!(handled);
        assert_eq!(input.value, 10.0);
    }

    #[test]
    fn test_number_input_handle_key_home_without_min() {
        let mut input = NumberInput::new().value(50.0);
        let handled = input.handle_key(&Key::Home);
        assert!(handled);
        assert_eq!(input.value, 50.0); // No change
    }

    #[test]
    fn test_number_input_handle_key_end() {
        let mut input = NumberInput::new().value(50.0).max(100.0);
        let handled = input.handle_key(&Key::End);
        assert!(handled);
        assert_eq!(input.value, 100.0);
    }

    #[test]
    fn test_number_input_handle_key_end_without_max() {
        let mut input = NumberInput::new().value(50.0);
        let handled = input.handle_key(&Key::End);
        assert!(handled);
        assert_eq!(input.value, 50.0); // No change
    }

    #[test]
    fn test_number_input_handle_key_enter_starts_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Enter);
        assert!(handled);
        assert!(input.editing);
    }

    #[test]
    fn test_number_input_handle_key_enter_commits_edit() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "100".to_string();
        let handled = input.handle_key(&Key::Enter);
        assert!(handled);
        assert!(!input.editing);
        assert_eq!(input.value, 100.0);
    }

    #[test]
    fn test_number_input_handle_key_escape_cancels_edit() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "100".to_string();
        let handled = input.handle_key(&Key::Escape);
        assert!(handled);
        assert!(!input.editing);
        assert_eq!(input.value, 42.0); // Original value
    }

    #[test]
    fn test_number_input_handle_key_escape_not_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Escape);
        assert!(!handled);
    }

    #[test]
    fn test_number_input_handle_key_digit_starts_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Char('5'));
        assert!(handled);
        assert!(input.editing);
        assert_eq!(input.input_buffer, "5");
    }

    #[test]
    fn test_number_input_handle_key_digit_while_editing() {
        let mut input = NumberInput::new().value(0.0);
        input.start_editing();
        input.input_buffer.clear(); // Clear for clean test
        input.handle_key(&Key::Char('5'));
        assert_eq!(input.input_buffer, "5");
        input.handle_key(&Key::Char('0'));
        assert_eq!(input.input_buffer, "50");
    }

    #[test]
    fn test_number_input_handle_key_decimal_point() {
        let mut input = NumberInput::new().value(0.0);
        input.start_editing();
        input.input_buffer.clear(); // Clear for clean test
        input.handle_key(&Key::Char('5'));
        input.handle_key(&Key::Char('.'));
        input.handle_key(&Key::Char('5'));
        assert_eq!(input.input_buffer, "5.5");
    }

    #[test]
    fn test_number_input_handle_key_decimal_point_only_once() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "5.5".to_string();
        input.cursor = 3;
        input.handle_key(&Key::Char('.'));
        assert_eq!(input.input_buffer, "5.5"); // No second decimal
    }

    #[test]
    fn test_number_input_handle_key_minus_at_start() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer.clear();
        input.cursor = 0; // Reset cursor to start
        input.handle_key(&Key::Char('-'));
        assert_eq!(input.input_buffer, "-");
    }

    #[test]
    fn test_number_input_handle_key_minus_not_at_start() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "10".to_string();
        input.handle_key(&Key::Char('-'));
        assert_eq!(input.input_buffer, "10"); // No minus added
    }

    #[test]
    fn test_number_input_handle_key_minus_only_once() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "-10".to_string();
        input.handle_key(&Key::Char('-'));
        assert_eq!(input.input_buffer, "-10"); // No second minus
    }

    #[test]
    fn test_number_input_handle_key_backspace_while_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 3;
        input.handle_key(&Key::Backspace);
        assert_eq!(input.input_buffer, "12");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_number_input_handle_key_backspace_not_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Backspace);
        assert!(!handled);
    }

    #[test]
    fn test_number_input_handle_key_delete_while_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 1;
        input.handle_key(&Key::Delete);
        assert_eq!(input.input_buffer, "13");
    }

    #[test]
    fn test_number_input_handle_key_delete_not_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Delete);
        assert!(!handled);
    }

    #[test]
    fn test_number_input_handle_key_left_while_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 3;
        input.handle_key(&Key::Left);
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_number_input_handle_key_left_not_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Left);
        assert!(!handled);
    }

    #[test]
    fn test_number_input_handle_key_left_at_start() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 0;
        let handled = input.handle_key(&Key::Left);
        assert!(!handled);
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_number_input_handle_key_right_while_editing() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 1;
        input.handle_key(&Key::Right);
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_number_input_handle_key_right_not_editing() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Right);
        assert!(!handled);
    }

    #[test]
    fn test_number_input_handle_key_right_at_end() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 3;
        let handled = input.handle_key(&Key::Right);
        assert!(!handled);
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_number_input_handle_key_unknown() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::Escape);
        assert!(!handled);
    }

    #[test]
    fn test_number_input_handle_key_f1() {
        let mut input = NumberInput::new().value(42.0);
        let handled = input.handle_key(&Key::F(1));
        assert!(!handled);
    }

    // =========================================================================
    // Clone Tests
    // =========================================================================

    #[test]
    fn test_number_input_clone() {
        let input = NumberInput::new()
            .value(42.0)
            .min(0.0)
            .max(100.0)
            .step(5.0)
            .precision(2)
            .prefix("$")
            .suffix("%");

        let cloned = input.clone();
        assert_eq!(cloned.value, 42.0);
        assert_eq!(cloned.min, Some(0.0));
        assert_eq!(cloned.max, Some(100.0));
        assert_eq!(cloned.step, 5.0);
        assert_eq!(cloned.precision, 2);
        assert_eq!(cloned.prefix, Some("$".to_string()));
        assert_eq!(cloned.suffix, Some("%".to_string()));
    }

    // =========================================================================
    // UTF-8 Tests
    // =========================================================================

    #[test]
    fn test_number_input_utf8_char_count() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        assert_eq!(input.buffer_char_count(), 3);
    }

    #[test]
    fn test_number_input_insert_char_at_cursor() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "13".to_string();
        input.cursor = 1;
        input.insert_char_at_cursor('2');
        assert_eq!(input.input_buffer, "123");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_number_input_delete_char_before_cursor() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 2;
        input.delete_char_before_cursor();
        assert_eq!(input.input_buffer, "13");
        assert_eq!(input.cursor, 1);
    }

    #[test]
    fn test_number_input_delete_char_at_cursor() {
        let mut input = NumberInput::new().value(42.0);
        input.start_editing();
        input.input_buffer = "123".to_string();
        input.cursor = 1;
        input.delete_char_at_cursor();
        assert_eq!(input.input_buffer, "13");
    }
}
