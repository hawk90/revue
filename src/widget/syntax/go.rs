//! Go syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_go(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
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
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            spans.push(HighlightSpan::new(i, len, theme.comment).italic());
            break;
        }

        // Strings
        if chars[i] == '"' || chars[i] == '`' {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < len && quote == '"' {
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

        // Identifiers and keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            if super::keywords::is_go_keyword(&word) {
                spans.push(HighlightSpan::new(start, i, theme.keyword).bold());
            } else if word == "true" || word == "false" || word == "nil" {
                spans.push(HighlightSpan::new(start, i, theme.constant));
            } else if word
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                spans.push(HighlightSpan::new(start, i, theme.type_name));
            } else if i < len && chars[i] == '(' {
                spans.push(HighlightSpan::new(start, i, theme.function));
            }
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < len
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
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
