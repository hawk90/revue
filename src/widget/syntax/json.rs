//! JSON syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_json(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_key = true;

    while i < len {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Strings
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < len {
                    i += 1;
                }
                i += 1;
            }
            if i < len {
                i += 1;
            }
            let color = if in_key { theme.variable } else { theme.string };
            spans.push(HighlightSpan::new(start, i, color));

            // Check for colon after key
            let mut j = i;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == ':' {
                in_key = false;
            }
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() || chars[i] == '-' {
            let start = i;
            if chars[i] == '-' {
                i += 1;
            }
            while i < len
                && (chars[i].is_ascii_digit()
                    || chars[i] == '.'
                    || chars[i] == 'e'
                    || chars[i] == 'E'
                    || chars[i] == '+'
                    || chars[i] == '-')
            {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.number));
            continue;
        }

        // Boolean and null
        if chars[i].is_alphabetic() {
            let start = i;
            while i < len && chars[i].is_alphabetic() {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            if word == "true" || word == "false" || word == "null" {
                spans.push(HighlightSpan::new(start, i, theme.constant));
            }
            continue;
        }

        // Reset key detection on comma or opening brace/bracket
        if chars[i] == ',' || chars[i] == '{' || chars[i] == '[' {
            in_key = true;
        }

        i += 1;
    }

    spans
}
