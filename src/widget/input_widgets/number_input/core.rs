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
            if let Some(ch) = self.input_buffer.chars().nth(self.cursor) {
                self.input_buffer.drain(byte_idx..byte_idx + ch.len_utf8());
            }
        }
    }

    fn delete_char_at_cursor(&mut self) {
        let char_count = self.buffer_char_count();
        if self.cursor < char_count {
            let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
            if let Some(ch) = self.input_buffer.chars().nth(self.cursor) {
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
