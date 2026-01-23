//! HTML syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_html(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Tags
        if chars[i] == '<' {
            let start = i;
            i += 1;
            // Tag name
            while i < len && !chars[i].is_whitespace() && chars[i] != '>' && chars[i] != '/' {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.keyword));

            // Attributes
            while i < len && chars[i] != '>' {
                if chars[i].is_alphabetic() {
                    let attr_start = i;
                    while i < len && (chars[i].is_alphanumeric() || chars[i] == '-') {
                        i += 1;
                    }
                    spans.push(HighlightSpan::new(attr_start, i, theme.variable));
                } else if chars[i] == '"' || chars[i] == '\'' {
                    let quote = chars[i];
                    let str_start = i;
                    i += 1;
                    while i < len && chars[i] != quote {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    spans.push(HighlightSpan::new(str_start, i, theme.string));
                } else {
                    i += 1;
                }
            }
            if i < len && chars[i] == '>' {
                i += 1;
            }
            continue;
        }

        i += 1;
    }

    spans
}
