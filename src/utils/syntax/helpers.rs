//! Helper functions for syntax highlighting

use super::{Language, SyntaxHighlighter, Token};

/// Highlight code and return tokens
pub fn highlight(code: &str, lang: &str) -> Vec<Vec<Token>> {
    let highlighter = SyntaxHighlighter::new();
    let language = Language::from_fence(lang);
    highlighter.highlight(code, language)
}

/// Highlight a single line and return tokens
pub fn highlight_line(line: &str, lang: &str) -> Vec<Token> {
    let highlighter = SyntaxHighlighter::new();
    let language = Language::from_fence(lang);
    highlighter.highlight_line(line, language)
}
