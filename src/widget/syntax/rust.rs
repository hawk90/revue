//! Rust syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_rust(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Comments
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            spans.push(HighlightSpan::new(i, len, theme.comment).italic());
            break;
        }

        // Strings
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < len && (chars[i] != '"' || (i > 0 && chars[i - 1] == '\\')) {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.string));
            continue;
        }

        // Character literals
        if chars[i] == '\'' && i + 2 < len {
            let start = i;
            i += 1;
            if chars[i] == '\\' {
                i += 2;
            } else {
                i += 1;
            }
            if i < len && chars[i] == '\'' {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.string));
            continue;
        }

        // Macros
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            if i < len && chars[i] == '!' {
                spans.push(HighlightSpan::new(start, i + 1, theme.macro_call));
                i += 1;
                continue;
            }

            // Keywords
            if super::keywords::is_rust_keyword(&word) {
                spans.push(HighlightSpan::new(start, i, theme.keyword).bold());
            }
            // Types (starts with uppercase)
            else if word
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                spans.push(HighlightSpan::new(start, i, theme.type_name));
            }
            // Constants
            else if word == "true" || word == "false" || word == "None" || word == "Some" {
                spans.push(HighlightSpan::new(start, i, theme.constant));
            }
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < len
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.')
            {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.number));
            continue;
        }

        // Attributes
        if chars[i] == '#' && i + 1 < len && chars[i + 1] == '[' {
            let start = i;
            while i < len && chars[i] != ']' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.attribute));
            continue;
        }

        i += 1;
    }

    spans
}
