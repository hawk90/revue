//! Helper functions for error creation

use crate::style::{ErrorCode, ParseError};

/// Create a ParseError at the given position
pub fn make_error(css: &str, pos: usize, message: &str, code: ErrorCode) -> ParseError {
    ParseError::at_offset(message, css, pos).with_code(code)
}

/// Create a ParseError for missing brace
pub fn missing_brace_error(css: &str, pos: usize, expected: char) -> ParseError {
    make_error(
        css,
        pos,
        &format!("expected '{}' but found end of input", expected),
        ErrorCode::MissingBrace,
    )
}
