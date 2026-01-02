//! Large digit display widget
//!
//! Displays numbers using large 7-segment style characters, perfect for
//! dashboards, clocks, timers, and counters.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Digits, DigitStyle};
//!
//! // Simple counter display
//! let counter = Digits::new(42);
//!
//! // Clock display with colons
//! let clock = Digits::new("12:34:56")
//!     .style(DigitStyle::Block)
//!     .fg(Color::CYAN);
//!
//! // Price display
//! let price = digits(1234.56)
//!     .prefix("$")
//!     .separator(',');
//! ```

use crate::style::Color;
use crate::widget::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// 7-segment digit patterns (3x5 characters each)
/// Each digit is represented as 5 rows of 3 characters
const DIGIT_PATTERNS_BLOCK: [[&str; 5]; 10] = [
    // 0
    ["███", "█ █", "█ █", "█ █", "███"],
    // 1
    ["  █", "  █", "  █", "  █", "  █"],
    // 2
    ["███", "  █", "███", "█  ", "███"],
    // 3
    ["███", "  █", "███", "  █", "███"],
    // 4
    ["█ █", "█ █", "███", "  █", "  █"],
    // 5
    ["███", "█  ", "███", "  █", "███"],
    // 6
    ["███", "█  ", "███", "█ █", "███"],
    // 7
    ["███", "  █", "  █", "  █", "  █"],
    // 8
    ["███", "█ █", "███", "█ █", "███"],
    // 9
    ["███", "█ █", "███", "  █", "███"],
];

/// Thin digit patterns (3x5 characters each)
const DIGIT_PATTERNS_THIN: [[&str; 5]; 10] = [
    // 0
    ["┌─┐", "│ │", "│ │", "│ │", "└─┘"],
    // 1
    ["  │", "  │", "  │", "  │", "  │"],
    // 2
    ["──┐", "  │", "┌─┘", "│  ", "└──"],
    // 3
    ["──┐", "  │", "──┤", "  │", "──┘"],
    // 4
    ["│ │", "│ │", "└─┤", "  │", "  │"],
    // 5
    ["┌──", "│  ", "└─┐", "  │", "──┘"],
    // 6
    ["┌──", "│  ", "├─┐", "│ │", "└─┘"],
    // 7
    ["──┐", "  │", "  │", "  │", "  │"],
    // 8
    ["┌─┐", "│ │", "├─┤", "│ │", "└─┘"],
    // 9
    ["┌─┐", "│ │", "└─┤", "  │", "──┘"],
];

/// ASCII digit patterns (3x5 characters each)
const DIGIT_PATTERNS_ASCII: [[&str; 5]; 10] = [
    // 0
    ["+-+", "| |", "| |", "| |", "+-+"],
    // 1
    ["  |", "  |", "  |", "  |", "  |"],
    // 2
    ["--+", "  |", "+-+", "|  ", "+--"],
    // 3
    ["--+", "  |", "--+", "  |", "--+"],
    // 4
    ["| |", "| |", "+-+", "  |", "  |"],
    // 5
    ["+--", "|  ", "+-+", "  |", "--+"],
    // 6
    ["+--", "|  ", "+-+", "| |", "+-+"],
    // 7
    ["--+", "  |", "  |", "  |", "  |"],
    // 8
    ["+-+", "| |", "+-+", "| |", "+-+"],
    // 9
    ["+-+", "| |", "+-+", "  |", "--+"],
];

/// Braille digit patterns (2x4 characters each, using braille)
const DIGIT_PATTERNS_BRAILLE: [[&str; 4]; 10] = [
    // 0
    ["⣰⣆", "⡇⢸", "⡇⢸", "⠈⠉"],
    // 1
    [" ⡆", " ⡇", " ⡇", " ⠁"],
    // 2
    ["⠤⡤", " ⡰", "⡰ ", "⠧⠤"],
    // 3
    ["⠤⡤", " ⡤", " ⡤", "⠤⠴"],
    // 4
    ["⡆⡆", "⡧⡦", " ⡇", " ⠁"],
    // 5
    ["⡤⠤", "⡤⠤", " ⢸", "⠤⠴"],
    // 6
    ["⣰⠆", "⡧⠤", "⡇⢸", "⠈⠉"],
    // 7
    ["⠤⡤", " ⡰", " ⡇", " ⠁"],
    // 8
    ["⣰⣆", "⡧⡦", "⡇⢸", "⠈⠉"],
    // 9
    ["⣰⣆", "⡧⡦", " ⢸", "⠤⠴"],
];

/// Special character patterns
const COLON_BLOCK: [&str; 5] = [" ", "█", " ", "█", " "];
const COLON_THIN: [&str; 5] = [" ", "●", " ", "●", " "];
const COLON_ASCII: [&str; 5] = [" ", "o", " ", "o", " "];
const COLON_BRAILLE: [&str; 4] = [" ", "⠂", "⠂", " "];

const DOT_BLOCK: [&str; 5] = [" ", " ", " ", " ", "█"];
const DOT_THIN: [&str; 5] = [" ", " ", " ", " ", "●"];
const DOT_ASCII: [&str; 5] = [" ", " ", " ", " ", "o"];
const DOT_BRAILLE: [&str; 4] = [" ", " ", " ", "⠂"];

const MINUS_BLOCK: [&str; 5] = ["   ", "   ", "███", "   ", "   "];
const MINUS_THIN: [&str; 5] = ["   ", "   ", "───", "   ", "   "];
const MINUS_ASCII: [&str; 5] = ["   ", "   ", "---", "   ", "   "];
const MINUS_BRAILLE: [&str; 4] = ["  ", "⠤⠤", "  ", "  "];

const SPACE_BLOCK: [&str; 5] = ["   ", "   ", "   ", "   ", "   "];
const SPACE_BRAILLE: [&str; 4] = ["  ", "  ", "  ", "  "];

/// Digit display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DigitStyle {
    /// Block characters (███)
    #[default]
    Block,
    /// Thin box drawing characters (┌─┐)
    Thin,
    /// ASCII characters (+-)
    Ascii,
    /// Braille characters (more compact)
    Braille,
}

/// Large digit display widget
#[derive(Clone, Debug)]
pub struct Digits {
    /// The value to display
    value: String,
    /// Display style
    style: DigitStyle,
    /// Foreground color
    fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Prefix (e.g., "$", "€")
    prefix: Option<String>,
    /// Suffix (e.g., "%", "kg")
    suffix: Option<String>,
    /// Thousands separator
    separator: Option<char>,
    /// Minimum width (pad with zeros)
    min_width: Option<usize>,
    /// Show leading zeros
    leading_zeros: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Digits {
    /// Create new digits display from a number
    pub fn new(value: impl ToString) -> Self {
        Self {
            value: value.to_string(),
            style: DigitStyle::default(),
            fg: None,
            bg: None,
            prefix: None,
            suffix: None,
            separator: None,
            min_width: None,
            leading_zeros: false,
            props: WidgetProps::new(),
        }
    }

    /// Create from integer
    pub fn from_int(value: i64) -> Self {
        Self::new(value)
    }

    /// Create from float with decimal places
    pub fn from_float(value: f64, decimals: usize) -> Self {
        Self::new(format!("{:.prec$}", value, prec = decimals))
    }

    /// Create time display (HH:MM:SS)
    pub fn time(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self::new(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }

    /// Create clock display (HH:MM)
    pub fn clock(hours: u32, minutes: u32) -> Self {
        Self::new(format!("{:02}:{:02}", hours, minutes))
    }

    /// Create timer display from seconds
    pub fn timer(total_seconds: u64) -> Self {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            Self::time(hours as u32, minutes as u32, seconds as u32)
        } else {
            Self::new(format!("{:02}:{:02}", minutes, seconds))
        }
    }

    /// Set display style
    pub fn style(mut self, style: DigitStyle) -> Self {
        self.style = style;
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

    /// Set prefix (displayed before digits)
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set suffix (displayed after digits)
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set thousands separator
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = Some(sep);
        self
    }

    /// Set minimum width (pads with zeros)
    pub fn min_width(mut self, width: usize) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Show leading zeros
    pub fn leading_zeros(mut self, show: bool) -> Self {
        self.leading_zeros = show;
        self
    }

    /// Get the height of digits in this style
    pub fn height(&self) -> usize {
        match self.style {
            DigitStyle::Braille => 4,
            _ => 5,
        }
    }

    /// Get the width of a single digit in this style
    pub fn digit_width(&self) -> usize {
        match self.style {
            DigitStyle::Braille => 2,
            _ => 3,
        }
    }

    /// Render a single character pattern
    fn get_char_pattern(&self, c: char) -> Vec<&'static str> {
        match self.style {
            DigitStyle::Block => self.get_block_pattern(c),
            DigitStyle::Thin => self.get_thin_pattern(c),
            DigitStyle::Ascii => self.get_ascii_pattern(c),
            DigitStyle::Braille => self.get_braille_pattern(c),
        }
    }

    fn get_block_pattern(&self, c: char) -> Vec<&'static str> {
        match c {
            '0'..='9' => DIGIT_PATTERNS_BLOCK[(c as usize) - ('0' as usize)].to_vec(),
            ':' => COLON_BLOCK.to_vec(),
            '.' => DOT_BLOCK.to_vec(),
            '-' => MINUS_BLOCK.to_vec(),
            _ => SPACE_BLOCK.to_vec(),
        }
    }

    fn get_thin_pattern(&self, c: char) -> Vec<&'static str> {
        match c {
            '0'..='9' => DIGIT_PATTERNS_THIN[(c as usize) - ('0' as usize)].to_vec(),
            ':' => COLON_THIN.to_vec(),
            '.' => DOT_THIN.to_vec(),
            '-' => MINUS_THIN.to_vec(),
            _ => SPACE_BLOCK.to_vec(),
        }
    }

    fn get_ascii_pattern(&self, c: char) -> Vec<&'static str> {
        match c {
            '0'..='9' => DIGIT_PATTERNS_ASCII[(c as usize) - ('0' as usize)].to_vec(),
            ':' => COLON_ASCII.to_vec(),
            '.' => DOT_ASCII.to_vec(),
            '-' => MINUS_ASCII.to_vec(),
            _ => SPACE_BLOCK.to_vec(),
        }
    }

    fn get_braille_pattern(&self, c: char) -> Vec<&'static str> {
        match c {
            '0'..='9' => DIGIT_PATTERNS_BRAILLE[(c as usize) - ('0' as usize)].to_vec(),
            ':' => COLON_BRAILLE.to_vec(),
            '.' => DOT_BRAILLE.to_vec(),
            '-' => MINUS_BRAILLE.to_vec(),
            _ => SPACE_BRAILLE.to_vec(),
        }
    }

    /// Build the display string with formatting
    fn format_value(&self) -> String {
        let mut result = self.value.clone();

        // Apply minimum width with leading zeros
        if let Some(width) = self.min_width {
            if result.len() < width {
                let pad = "0".repeat(width - result.len());
                result = format!("{}{}", pad, result);
            }
        }

        // Apply thousands separator
        if let Some(sep) = self.separator {
            result = self.add_thousands_separator(&result, sep);
        }

        result
    }

    fn add_thousands_separator(&self, s: &str, sep: char) -> String {
        // Handle negative numbers
        let (sign, num) = if s.starts_with('-') {
            ("-", &s[1..])
        } else {
            ("", s)
        };

        // Split on decimal point
        let parts: Vec<&str> = num.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1);

        // Add separators to integer part
        let mut result = String::new();
        for (i, c) in integer_part.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(sep);
            }
            result.push(c);
        }
        let integer_with_sep: String = result.chars().rev().collect();

        // Reconstruct
        match decimal_part {
            Some(dec) => format!("{}{}.{}", sign, integer_with_sep, dec),
            None => format!("{}{}", sign, integer_with_sep),
        }
    }

    /// Render to lines
    pub fn render_lines(&self) -> Vec<String> {
        let value = self.format_value();
        let height = self.height();
        let mut lines: Vec<String> = vec![String::new(); height];

        // Add prefix (at bottom line, small text)
        // For now, we'll skip prefix/suffix in the large display

        // Render each character
        for c in value.chars() {
            let pattern = self.get_char_pattern(c);
            for (i, row) in pattern.iter().enumerate() {
                if i < height {
                    lines[i].push_str(row);
                    lines[i].push(' '); // spacing between digits
                }
            }
        }

        lines
    }
}

impl View for Digits {
    crate::impl_view_meta!("Digits");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        let lines = self.render_lines();
        let mut stack = vstack();

        for line in lines {
            let mut text = Text::new(line);
            if let Some(fg) = self.fg {
                text = text.fg(fg);
            }
            if let Some(bg) = self.bg {
                text = text.bg(bg);
            }
            stack = stack.child(text);
        }

        // Add prefix/suffix as regular text below
        if self.prefix.is_some() || self.suffix.is_some() {
            let label = format!(
                "{}{}{}",
                self.prefix.as_deref().unwrap_or(""),
                self.value,
                self.suffix.as_deref().unwrap_or("")
            );
            let mut text = Text::new(label);
            if let Some(fg) = self.fg {
                text = text.fg(fg);
            }
            stack = stack.child(text);
        }

        stack.render(ctx);
    }
}

impl_styled_view!(Digits);
impl_props_builders!(Digits);

/// Create a digits display
pub fn digits(value: impl ToString) -> Digits {
    Digits::new(value)
}

/// Create a clock display
pub fn clock(hours: u32, minutes: u32) -> Digits {
    Digits::clock(hours, minutes)
}

/// Create a timer display
pub fn timer(seconds: u64) -> Digits {
    Digits::timer(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_new() {
        let d = Digits::new(42);
        assert_eq!(d.value, "42");
    }

    #[test]
    fn test_digits_from_float() {
        let d = Digits::from_float(12.345, 2);
        assert_eq!(d.value, "12.35");
    }

    #[test]
    fn test_digits_time() {
        let d = Digits::time(12, 34, 56);
        assert_eq!(d.value, "12:34:56");
    }

    #[test]
    fn test_digits_clock() {
        let d = Digits::clock(9, 30);
        assert_eq!(d.value, "09:30");
    }

    #[test]
    fn test_digits_timer() {
        let d = Digits::timer(3661); // 1h 1m 1s
        assert_eq!(d.value, "01:01:01");

        let d2 = Digits::timer(65); // 1m 5s
        assert_eq!(d2.value, "01:05");
    }

    #[test]
    fn test_digits_style() {
        let d = Digits::new(0).style(DigitStyle::Thin);
        assert_eq!(d.style, DigitStyle::Thin);
    }

    #[test]
    fn test_digits_separator() {
        let d = Digits::new(1234567).separator(',');
        assert_eq!(d.format_value(), "1,234,567");
    }

    #[test]
    fn test_digits_min_width() {
        let d = Digits::new(42).min_width(5);
        assert_eq!(d.format_value(), "00042");
    }

    #[test]
    fn test_digits_height() {
        assert_eq!(Digits::new(0).style(DigitStyle::Block).height(), 5);
        assert_eq!(Digits::new(0).style(DigitStyle::Braille).height(), 4);
    }

    #[test]
    fn test_digits_render_lines() {
        let d = Digits::new(1).style(DigitStyle::Block);
        let lines = d.render_lines();
        assert_eq!(lines.len(), 5);
        assert!(lines[0].contains("█"));
    }

    #[test]
    fn test_digits_negative() {
        let d = Digits::new(-42).separator(',');
        let formatted = d.format_value();
        assert!(formatted.starts_with('-'));
    }

    #[test]
    fn test_digits_decimal_separator() {
        let d = Digits::new("1234.56").separator(',');
        assert_eq!(d.format_value(), "1,234.56");
    }

    #[test]
    fn test_helper_functions() {
        let d = digits(100);
        assert_eq!(d.value, "100");

        let c = clock(12, 0);
        assert_eq!(c.value, "12:00");

        let t = timer(90);
        assert_eq!(t.value, "01:30");
    }
}
