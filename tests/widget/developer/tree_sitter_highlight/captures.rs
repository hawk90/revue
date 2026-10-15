//! Capture names to theme colors: each name a grammar tags a range with
//! reaches one `SyntaxTheme` field, keywords and constants bold, comments
//! italic

use revue::widget::syntax::Language;

use super::{d, highlighted};

/// (language, line, covered text, theme field number, style)
const CASES: &[(Language, &str, &str, u8, &str)] = &[
    (Language::Rust, "fn f(&self) -> bool { true }", "fn", 1, "B"),
    (Language::Rust, "let x: i32 = 1;", "i32", 2, ""),
    (Language::Rust, "#[derive(Debug)]", "Debug", 2, ""),
    (Language::Rust, "fn f(&self) -> bool { true }", "f", 3, ""),
    (Language::Python, "def hello():", "hello", 3, ""),
    (Language::Rust, "let s = \"s\";", "\"s\"", 4, ""),
    (Language::JavaScript, "const x = 42;", "42", 5, ""),
    (Language::Rust, "let x = 1; // c", "// c", 6, "I"),
    (Language::JavaScript, "const x = 42;", "=", 7, ""),
    (Language::Rust, "fn f(&self) -> bool { true }", "&", 7, ""),
    (Language::Rust, "let x = 1;", ";", 8, ""),
    (
        Language::Rust,
        "fn f(&self) -> bool { true }",
        "true",
        9,
        "B",
    ),
    (Language::Rust, "let x = 1;", "1", 9, "B"),
    (Language::Rust, "#[derive(Debug)]", "#", 11, ""),
    (Language::Html, "<div class=\"a\">t</div>", "div", 11, ""),
    (Language::Html, "<div class=\"a\">t</div>", "class", 11, ""),
    (
        Language::Rust,
        "fn f(&self) -> bool { true }",
        "self",
        12,
        "",
    ),
    (Language::JavaScript, "const x = 42;", "x", 12, ""),
];

#[test]
fn each_capture_reaches_its_theme_color_and_style() {
    for &(language, line, text, field, style) in CASES {
        let spans = highlighted(language, line);
        assert!(
            spans.contains(&d(text, field, style)),
            "{language:?} {line:?}: {text:?} -> {field}{style}, got {spans:?}"
        );
    }
    // Every field is reached but `macro_call` (10): macros take the
    // function color (#799)
    let mut fields: Vec<u8> = CASES.iter().map(|c| c.3).collect();
    fields.sort();
    fields.dedup();
    assert_eq!(fields, [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12]);
}

#[test]
fn only_keywords_and_constants_are_bold_and_only_comments_italic() {
    for &(language, line, ..) in CASES {
        for (text, field, style) in highlighted(language, line) {
            let want = match field {
                1 | 9 => "B",
                6 => "I",
                _ => "",
            };
            assert_eq!(style, want, "{language:?} {text:?} ({field})");
        }
    }
}

#[test]
fn empty_and_blank_lines_have_no_spans() {
    for language in [Language::Rust, Language::Python, Language::Json] {
        assert!(highlighted(language, "").is_empty(), "{language:?}");
        assert!(highlighted(language, "   ").is_empty(), "{language:?}");
    }
}
