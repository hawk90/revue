//! Syntax highlighting themes

use super::types::TokenType;
use crate::style::Color;

/// Theme for syntax highlighting
#[derive(Debug, Clone)]
pub struct SyntaxTheme {
    /// Color for keywords (fn, let, if, etc.)
    pub keyword: Color,
    /// Color for types (int, String, etc.)
    pub type_: Color,
    /// Color for string literals
    pub string: Color,
    /// Color for number literals
    pub number: Color,
    /// Color for comments
    pub comment: Color,
    /// Color for function names
    pub function: Color,
    /// Color for operators and punctuation
    pub operator: Color,
    /// Color for macros (Rust)
    pub macro_: Color,
    /// Color for attributes/decorators
    pub attribute: Color,
    /// Color for normal text
    pub normal: Color,
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        Self::monokai()
    }
}

impl SyntaxTheme {
    /// Monokai-inspired theme
    pub fn monokai() -> Self {
        Self {
            keyword: Color::rgb(249, 38, 114),    // Pink
            type_: Color::rgb(102, 217, 239),     // Cyan
            string: Color::rgb(230, 219, 116),    // Yellow
            number: Color::rgb(174, 129, 255),    // Purple
            comment: Color::rgb(117, 113, 94),    // Gray
            function: Color::rgb(166, 226, 46),   // Green
            operator: Color::rgb(249, 38, 114),   // Pink
            macro_: Color::rgb(102, 217, 239),    // Cyan
            attribute: Color::rgb(174, 129, 255), // Purple
            normal: Color::rgb(248, 248, 242),    // White
        }
    }

    /// Nord-inspired theme
    pub fn nord() -> Self {
        Self {
            keyword: Color::rgb(129, 161, 193),   // Blue
            type_: Color::rgb(143, 188, 187),     // Teal
            string: Color::rgb(163, 190, 140),    // Green
            number: Color::rgb(180, 142, 173),    // Purple
            comment: Color::rgb(76, 86, 106),     // Gray
            function: Color::rgb(136, 192, 208),  // Light Blue
            operator: Color::rgb(129, 161, 193),  // Blue
            macro_: Color::rgb(208, 135, 112),    // Orange
            attribute: Color::rgb(180, 142, 173), // Purple
            normal: Color::rgb(236, 239, 244),    // White
        }
    }

    /// Dracula-inspired theme
    pub fn dracula() -> Self {
        Self {
            keyword: Color::rgb(255, 121, 198),   // Pink
            type_: Color::rgb(139, 233, 253),     // Cyan
            string: Color::rgb(241, 250, 140),    // Yellow
            number: Color::rgb(189, 147, 249),    // Purple
            comment: Color::rgb(98, 114, 164),    // Gray
            function: Color::rgb(80, 250, 123),   // Green
            operator: Color::rgb(255, 121, 198),  // Pink
            macro_: Color::rgb(255, 184, 108),    // Orange
            attribute: Color::rgb(189, 147, 249), // Purple
            normal: Color::rgb(248, 248, 242),    // White
        }
    }

    /// One Dark-inspired theme
    pub fn one_dark() -> Self {
        Self {
            keyword: Color::rgb(198, 120, 221),   // Purple
            type_: Color::rgb(229, 192, 123),     // Yellow
            string: Color::rgb(152, 195, 121),    // Green
            number: Color::rgb(209, 154, 102),    // Orange
            comment: Color::rgb(92, 99, 112),     // Gray
            function: Color::rgb(97, 175, 239),   // Blue
            operator: Color::rgb(171, 178, 191),  // Light gray
            macro_: Color::rgb(86, 182, 194),     // Cyan
            attribute: Color::rgb(209, 154, 102), // Orange
            normal: Color::rgb(171, 178, 191),    // Light gray
        }
    }

    /// Get color for token type
    pub fn color(&self, token_type: TokenType) -> Color {
        match token_type {
            TokenType::Normal => self.normal,
            TokenType::Keyword => self.keyword,
            TokenType::Type => self.type_,
            TokenType::String => self.string,
            TokenType::Number => self.number,
            TokenType::Comment => self.comment,
            TokenType::Function => self.function,
            TokenType::Operator => self.operator,
            TokenType::Macro => self.macro_,
            TokenType::Attribute => self.attribute,
        }
    }
}
