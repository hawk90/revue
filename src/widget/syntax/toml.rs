//! TOML syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_toml(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
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

        // Section headers
        if chars[i] == '[' {
            let start = i;
            while i < len && chars[i] != ']' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.keyword).bold());
            continue;
        }

        // Strings
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < len {
                    i += 1;
                }
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.string));
            continue;
        }

        // Keys (before =)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            // Check if followed by =
            let mut j = i;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '=' {
                spans.push(HighlightSpan::new(start, i, theme.variable));
            } else if word == "true" || word == "false" {
                spans.push(HighlightSpan::new(start, i, theme.constant));
            }
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() || chars[i] == '-' || chars[i] == '+' {
            let start = i;
            while i < len
                && (chars[i].is_ascii_alphanumeric()
                    || chars[i] == '_'
                    || chars[i] == '.'
                    || chars[i] == '-'
                    || chars[i] == '+'
                    || chars[i] == ':')
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
