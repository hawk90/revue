//! YAML syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_yaml(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
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
        if chars[i] == '#' {
            spans.push(HighlightSpan::new(i, len, theme.comment).italic());
            break;
        }

        // Keys (word followed by colon)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-') {
                i += 1;
            }

            // Check if followed by :
            if i < len && chars[i] == ':' {
                spans.push(HighlightSpan::new(start, i, theme.variable));
                i += 1;
                continue;
            }

            let word: String = chars[start..i].iter().collect();
            if word == "true" || word == "false" || word == "null" || word == "yes" || word == "no"
            {
                spans.push(HighlightSpan::new(start, i, theme.constant));
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

        // Numbers
        if chars[i].is_ascii_digit() || chars[i] == '-' {
            let start = i;
            while i < len
                && (chars[i].is_ascii_alphanumeric()
                    || chars[i] == '.'
                    || chars[i] == '-'
                    || chars[i] == '+')
            {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.number));
            continue;
        }

        i += 1;
    }

    spans
}
