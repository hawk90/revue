//! Syntax highlighting types

/// Token type for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Normal text
    Normal,
    /// Language keywords (fn, let, if, etc.)
    Keyword,
    /// Built-in types (int, str, bool, etc.)
    Type,
    /// String literals
    String,
    /// Number literals
    Number,
    /// Comments
    Comment,
    /// Function names
    Function,
    /// Operators and punctuation
    Operator,
    /// Macros (Rust)
    Macro,
    /// Attributes/decorators
    Attribute,
}

/// A highlighted token
#[derive(Debug, Clone)]
pub struct Token {
    /// The text content of the token
    pub text: String,
    /// The type of the token (keyword, string, etc.)
    pub token_type: TokenType,
}

impl Token {
    /// Create a new token
    pub fn new(text: impl Into<String>, token_type: TokenType) -> Self {
        Self {
            text: text.into(),
            token_type,
        }
    }
}
