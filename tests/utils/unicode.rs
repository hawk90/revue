//! Unicode width utilities tests

use revue::utils::unicode::{
    center_to_width, char_width, display_width, pad_to_width, right_align_to_width, split_at_width,
    truncate_to_width, truncate_with_ellipsis, truncate_with_suffix, wrap_to_width,
};

#[test]
fn test_ascii_width() {
    assert_eq!(display_width("hello"), 5);
    assert_eq!(display_width("Hello, World!"), 13);
}

#[test]
fn test_cjk_width() {
    assert_eq!(display_width("世界"), 4);
    assert_eq!(display_width("안녕"), 4);
    assert_eq!(display_width("こんにちは"), 10);
}

#[test]
fn test_mixed_width() {
    assert_eq!(display_width("Hello世界"), 9); // 5 + 4
    assert_eq!(display_width("a한b글c"), 7); // 1 + 2 + 1 + 2 + 1
}

#[test]
fn test_emoji_width() {
    assert_eq!(display_width("🎉"), 2);
    assert_eq!(display_width("👍"), 2);
    assert_eq!(display_width("Hello 🎉"), 8); // 6 + 2
}

#[test]
fn test_truncate_ascii() {
    assert_eq!(truncate_to_width("Hello, World!", 5), "Hello");
    assert_eq!(truncate_to_width("Hello", 10), "Hello");
}

#[test]
fn test_truncate_cjk() {
    // "안녕하세요" = 10 width (5 chars × 2)
    assert_eq!(truncate_to_width("안녕하세요", 4), "안녕");
    assert_eq!(truncate_to_width("안녕하세요", 5), "안녕"); // Can't fit half of 하
    assert_eq!(truncate_to_width("안녕하세요", 6), "안녕하");
}

#[test]
fn test_truncate_mixed() {
    // "Hello世界" = 9 width
    assert_eq!(truncate_to_width("Hello世界", 7), "Hello世");
    assert_eq!(truncate_to_width("Hello世界", 6), "Hello"); // Can't fit 世 (width 2)
}

#[test]
fn test_truncate_with_ellipsis() {
    assert_eq!(truncate_with_ellipsis("Hello, World!", 8), "Hello, …");
    assert_eq!(truncate_with_ellipsis("Hi", 8), "Hi");
    assert_eq!(truncate_with_ellipsis("안녕하세요", 5), "안녕…");
}

#[test]
fn test_pad_to_width() {
    assert_eq!(pad_to_width("Hi", 5), "Hi   ");
    assert_eq!(pad_to_width("Hello", 3), "Hello");
    assert_eq!(pad_to_width("안녕", 6), "안녕  ");
}

#[test]
fn test_center_to_width() {
    assert_eq!(center_to_width("Hi", 6), "  Hi  ");
    assert_eq!(center_to_width("Hi", 5), " Hi  ");
}

#[test]
fn test_right_align_to_width() {
    assert_eq!(right_align_to_width("Hi", 5), "   Hi");
    assert_eq!(right_align_to_width("안녕", 6), "  안녕");
}

#[test]
fn test_wrap_to_width() {
    let lines = wrap_to_width("Hello World", 6);
    assert_eq!(lines, vec!["Hello", "World"]);

    let lines = wrap_to_width("안녕 세계", 6);
    assert_eq!(lines, vec!["안녕", "세계"]);
}

#[test]
fn test_char_width() {
    assert_eq!(char_width('a'), 1);
    assert_eq!(char_width('가'), 2);
    assert_eq!(char_width('あ'), 2);
    assert_eq!(char_width('漢'), 2);
}

#[test]
fn test_zero_width_chars() {
    // Combining character shouldn't add width
    assert_eq!(char_width('\u{0301}'), 0); // Combining acute accent
}

#[test]
fn symbols_drawn_as_text_are_one_column() {
    // Miscellaneous Symbols and Dingbats: terminals draw these in one cell
    // (East Asian Width N or A), not two (#1026)
    for s in ["☐", "☑", "★", "✓", "✗", "➜", "♥", "⚙"] {
        assert_eq!(display_width(s), 1, "{s}");
    }
    // The ones shown as emoji by default stay wide
    for s in ["✅", "⚡", "❌", "⭐"] {
        assert_eq!(display_width(s), 2, "{s}");
    }
}

#[test]
fn decomposed_hangul_is_one_syllable_wide() {
    // NFD 한: a leading consonant (2) and a conjoining vowel and final (0),
    // as macOS writes file names (#879)
    assert_eq!(display_width("\u{1112}\u{1161}\u{11AB}"), 2);
    assert_eq!(display_width("한"), 2);
}

#[test]
fn a_flag_is_two_columns() {
    // Two regional indicator letters, drawn as one flag
    assert_eq!(display_width("🇰🇷"), 2);
}

#[test]
fn control_characters_take_no_column() {
    for ch in ['\0', '\t', '\n', '\u{1b}', '\u{7f}', '\u{85}'] {
        assert_eq!(char_width(ch), 0, "{ch:?}");
    }
}

#[test]
fn test_fullwidth_chars() {
    // Full-width ASCII characters
    assert_eq!(display_width("ＡＢＣ"), 6); // Full-width ABC
    assert_eq!(char_width('Ａ'), 2);
}

// =========================================================================
// Edge Case Tests - UTF-8 Boundary Safety
// =========================================================================

#[test]
fn test_truncate_empty_string() {
    assert_eq!(truncate_to_width("", 10), "");
    assert_eq!(truncate_to_width("", 0), "");
}

#[test]
fn test_truncate_zero_width() {
    assert_eq!(truncate_to_width("hello", 0), "");
    assert_eq!(truncate_to_width("안녕", 0), "");
}

#[test]
fn test_truncate_single_wide_char() {
    // Can't fit even one wide character
    assert_eq!(truncate_to_width("안", 1), "");
    // Can fit exactly one wide character
    assert_eq!(truncate_to_width("안", 2), "안");
}

#[test]
fn test_truncate_wider_than_available() {
    assert_eq!(truncate_to_width("hi", 100), "hi");
    assert_eq!(truncate_to_width("안녕", 100), "안녕");
}

#[test]
fn test_split_at_width_edge_cases() {
    // Empty string
    let (left, right) = split_at_width("", 5);
    assert_eq!(left, "");
    assert_eq!(right, "");

    // Zero width
    let (left, right) = split_at_width("hello", 0);
    assert_eq!(left, "");
    assert_eq!(right, "hello");

    // Width larger than string
    let (left, right) = split_at_width("hi", 100);
    assert_eq!(left, "hi");
    assert_eq!(right, "");
}

#[test]
fn test_split_at_width_wide_chars() {
    let (left, _right) = split_at_width("안녕하세요", 4);
    assert_eq!(left, "안녕");
    assert_eq!(display_width(left), 4);
}

#[test]
fn test_truncate_with_suffix_edge_cases() {
    // Empty string
    assert_eq!(truncate_with_suffix("", 5, "…"), "");

    // Zero max width - can't fit anything, returns empty
    assert_eq!(truncate_with_suffix("hello", 0, "…"), "");

    // String already fits
    assert_eq!(truncate_with_suffix("hi", 5, "…"), "hi");

    // Max width exactly fits suffix
    assert_eq!(truncate_with_suffix("hello world", 1, "…"), "…");
}

#[test]
fn test_pad_to_width_edge_cases() {
    // Empty string
    assert_eq!(pad_to_width("", 5), "     ");

    // Zero width
    assert_eq!(pad_to_width("hello", 0), "hello");

    // Already at target width
    assert_eq!(pad_to_width("abc", 3), "abc");
}

#[test]
fn test_center_to_width_edge_cases() {
    // Empty string
    assert_eq!(center_to_width("", 5), "     ");

    // Zero width
    assert_eq!(center_to_width("hello", 0), "hello");

    // Odd padding
    assert_eq!(center_to_width("x", 4), " x  ");
}

#[test]
fn test_right_align_to_width_edge_cases() {
    // Empty string
    assert_eq!(right_align_to_width("", 5), "     ");

    // Zero width
    assert_eq!(right_align_to_width("hello", 0), "hello");
}

#[test]
fn test_wrap_to_width_edge_cases() {
    // Empty text is one empty row, as a blank line is (#1474)
    assert_eq!(wrap_to_width("", 10), vec![""]);

    // Zero width: as width 1, nothing lost (#1474)
    assert_eq!(wrap_to_width("hello", 0), vec!["h", "e", "l", "l", "o"]);

    // A character wider than the width gets a row of its own (#1474)
    assert_eq!(wrap_to_width("안", 1), vec!["안"]);
    assert_eq!(wrap_to_width("a안b", 1), vec!["a", "안", "b"]);
}

#[test]
fn wrap_to_width_keeps_spaces_and_starts_a_row_at_each_newline() {
    // #1474: spaces inside a row are kept, and a newline is a row break
    assert_eq!(wrap_to_width("a  b   c", 4), vec!["a  b", "c"]);
    assert_eq!(wrap_to_width("one\ntwo three", 9), vec!["one", "two three"]);
    assert_eq!(wrap_to_width("well-known", 6), vec!["well-", "known"]);
}

#[test]
fn wrap_to_width_is_text_wrap_words() {
    let texts = [
        "Hello World",
        "a  b",
        "x\ny",
        "가나다 라",
        "abc-def ghi",
        "e\u{301}x",
    ];
    for text in texts {
        for width in 0..=8 {
            assert_eq!(
                wrap_to_width(text, width),
                revue::text::wrap_words(text, width),
                "{text:?} at {width}"
            );
        }
    }
}

#[test]
fn test_display_width_edge_cases() {
    // Empty string
    assert_eq!(display_width(""), 0);

    // Control characters
    assert_eq!(display_width("\n\t"), 0);

    // Zero-width characters
    assert_eq!(display_width("a\u{0301}"), 1); // a + combining acute = 1 width
}

#[test]
fn test_char_width_control_chars() {
    assert_eq!(char_width('\0'), 0);
    assert_eq!(char_width('\n'), 0);
    assert_eq!(char_width('\t'), 0);
    assert_eq!(char_width('\r'), 0);
}

#[test]
fn test_char_width_ascii_printable() {
    for c in '!'..='~' {
        assert_eq!(char_width(c), 1, "Character '{}' should have width 1", c);
    }
}

#[test]
fn test_truncate_preserves_valid_utf8() {
    // Test that truncated strings are always valid UTF-8
    let s = "Hello世界안녕🎉";

    for width in 0..=20 {
        let truncated = truncate_to_width(s, width);
        // Verify it's valid UTF-8 by trying to count chars
        let _ = truncated.chars().count();
    }
}

#[test]
fn test_split_preserves_valid_utf8() {
    // Test that both parts of split are valid UTF-8
    let s = "Hello世界안녕🎉";

    for width in 0..=20 {
        let (left, right) = split_at_width(s, width);
        // Verify both are valid UTF-8
        let _ = left.chars().count();
        let _ = right.chars().count();
    }
}

// A width cast to u16 wraps past 65535 columns: 65536 reads as 0 (#1607).
// Only `display_width_u16` (clamped to u16::MAX) may turn a width into a
// cell count; this ratchet fails on any new `display_width(..) as u16`.

fn sources(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The index after the parenthesized group that opens at `open`
fn after_group(code: &[u8], open: usize) -> usize {
    let mut depth = 0;
    for (i, &b) in code.iter().enumerate().skip(open) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
    }
    code.len()
}

/// Whether the `display_width(` call at `start` is cast to u16, directly or
/// after closing parens, `.min(..)` / `.max(..)` or `+ <number>`
fn cast_to_u16(code: &[u8], start: usize) -> bool {
    let open = start + "display_width".len();
    let mut i = after_group(code, open);
    loop {
        while i < code.len() && code[i].is_ascii_whitespace() {
            i += 1;
        }
        let rest = &code[i..];
        if rest.starts_with(b")") {
            i += 1;
        } else if rest.starts_with(b".min(") || rest.starts_with(b".max(") {
            i = after_group(code, i + 4);
        } else if rest.starts_with(b"+ ") || rest.starts_with(b"- ") {
            let digits = rest[2..].iter().take_while(|b| b.is_ascii_digit()).count();
            if digits == 0 {
                return false;
            }
            i += 2 + digits;
        } else {
            return rest.starts_with(b"as u16");
        }
    }
}

/// `display_width(..) as u16` casts in `source`, as line numbers
fn width_casts(source: &str) -> Vec<usize> {
    // Comments may name the cast; only code counts
    let code: String = source
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("//") {
                ""
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let bytes = code.as_bytes();
    code.match_indices("display_width(")
        .map(|(start, _)| start)
        .filter(|&start| {
            let before = bytes[..start].last();
            !before.is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
        })
        .filter(|&start| cast_to_u16(bytes, start))
        .map(|start| code[..start].lines().count())
        .collect()
}

#[test]
fn width_casts_are_found() {
    assert_eq!(width_casts("let w = display_width(s) as u16;"), [1]);
    assert_eq!(
        width_casts("x += (display_width(&a.b) as u16).min(c);"),
        [1]
    );
    assert_eq!(
        width_casts(
            "a\nlet w = crate::utils::display_width(f(x))\n    .min(u16::MAX as usize) as u16;"
        ),
        [2]
    );
    assert_eq!(width_casts("let w = (display_width(s) + 2) as u16;"), [1]);
    assert_eq!(
        width_casts("let w = display_width_u16(s);"),
        [] as [usize; 0]
    );
    assert_eq!(
        width_casts("let w = display_width(s) as f64;"),
        [] as [usize; 0]
    );
    assert_eq!(
        width_casts("let w = display_width(s) + n;"),
        [] as [usize; 0]
    );
    assert_eq!(width_casts("// display_width(s) as u16"), [] as [usize; 0]);
}

#[test]
fn no_display_width_is_cast_to_u16() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    sources(&root.join("src"), &mut files);
    let mut casts = Vec::new();
    for file in files {
        let name = file
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        // The helper's own file: `display_width_u16` clamps there
        if name == "src/utils/unicode.rs" {
            continue;
        }
        let source = std::fs::read_to_string(&file).unwrap();
        casts.extend(
            width_casts(&source)
                .into_iter()
                .map(|line| format!("{name}:{line}")),
        );
    }
    assert!(
        casts.is_empty(),
        "use display_width_u16 (clamped to u16::MAX), not display_width(..) as u16: {casts:?}"
    );
}
