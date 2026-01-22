//! SQL syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_sql(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
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
        if i + 1 < len && chars[i] == '-' && chars[i + 1] == '-' {
            spans.push(HighlightSpan::new(i, len, theme.comment).italic());
            break;
        }

        // Strings
        if chars[i] == '\'' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '\'' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.string));
            continue;
        }

        // Keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            if super::keywords::is_sql_keyword(&word) {
                spans.push(HighlightSpan::new(start, i, theme.keyword).bold());
            } else if word.to_uppercase() == "NULL"
                || word.to_uppercase() == "TRUE"
                || word.to_uppercase() == "FALSE"
            {
                spans.push(HighlightSpan::new(start, i, theme.constant));
            }
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.number));
            continue;
        }

        i += 1;
    }

    spans
}
