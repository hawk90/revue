//! Shell syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_shell(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
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

        // Variables $VAR or ${VAR}
        if chars[i] == '$' {
            let start = i;
            i += 1;
            if i < len && chars[i] == '{' {
                while i < len && chars[i] != '}' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
            } else {
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
            }
            spans.push(HighlightSpan::new(start, i, theme.variable));
            continue;
        }

        // Commands/keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            if super::keywords::is_shell_keyword(&word) {
                spans.push(HighlightSpan::new(start, i, theme.keyword).bold());
            }
            continue;
        }

        i += 1;
    }

    spans
}
