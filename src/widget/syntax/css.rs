//! CSS syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_css(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Comments
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            let start = i;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            }
            spans.push(HighlightSpan::new(start, i, theme.comment).italic());
            continue;
        }

        // Selectors (class, id, element)
        if chars[i] == '.' || chars[i] == '#' || chars[i].is_alphabetic() {
            let start = i;
            while i < len
                && (chars[i].is_alphanumeric()
                    || chars[i] == '-'
                    || chars[i] == '_'
                    || chars[i] == '.'
                    || chars[i] == '#')
            {
                i += 1;
            }
            // Check if it's before a { (selector) or : (property)
            let mut j = i;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '{' {
                spans.push(HighlightSpan::new(start, i, theme.type_name));
            } else if j < len && chars[j] == ':' {
                spans.push(HighlightSpan::new(start, i, theme.variable));
            }
            continue;
        }

        // Strings
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len && chars[i] != quote {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.string));
            continue;
        }

        // Numbers and units
        if chars[i].is_ascii_digit() || chars[i] == '-' {
            let start = i;
            if chars[i] == '-' {
                i += 1;
            }
            while i < len
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '%')
            {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.number));
            continue;
        }

        // Colors #hex
        if chars[i] == '#' {
            let start = i;
            i += 1;
            while i < len && chars[i].is_ascii_hexdigit() {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.constant));
            continue;
        }

        i += 1;
    }

    spans
}
