//! Markdown syntax highlighting

use super::types::{HighlightSpan, SyntaxTheme};

pub fn highlight_markdown(line: &str, theme: &SyntaxTheme) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    // Headers
    if !chars.is_empty() && chars[0] == '#' {
        let mut i = 0;
        while i < len && chars[i] == '#' {
            i += 1;
        }
        spans.push(HighlightSpan::new(0, len, theme.keyword).bold());
        return spans;
    }

    let mut i = 0;
    while i < len {
        // Bold **text** or __text__
        if i + 1 < len
            && ((chars[i] == '*' && chars[i + 1] == '*')
                || (chars[i] == '_' && chars[i + 1] == '_'))
        {
            let marker = chars[i];
            let start = i;
            i += 2;
            while i + 1 < len && !(chars[i] == marker && chars[i + 1] == marker) {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            }
            spans.push(HighlightSpan::new(start, i, theme.constant).bold());
            continue;
        }

        // Italic *text* or _text_
        if chars[i] == '*' || chars[i] == '_' {
            let marker = chars[i];
            let start = i;
            i += 1;
            while i < len && chars[i] != marker {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.string).italic());
            continue;
        }

        // Code `text`
        if chars[i] == '`' {
            let start = i;
            i += 1;
            while i < len && chars[i] != '`' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            spans.push(HighlightSpan::new(start, i, theme.function));
            continue;
        }

        // Links [text](url)
        if chars[i] == '[' {
            let start = i;
            while i < len && chars[i] != ']' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            if i < len && chars[i] == '(' {
                while i < len && chars[i] != ')' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
            }
            spans.push(HighlightSpan::new(start, i, theme.type_name));
            continue;
        }

        i += 1;
    }

    spans
}
