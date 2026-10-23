//! Tree-sitter syntax highlighter tests
//!
//! These test `TreeSitterHighlighter` (the `syntax-highlighting` feature).
//! The keyword highlighter, `SyntaxHighlighter`, has its own tests in
//! tests/widget/syntax.

mod captures;
mod lines;

use revue::style::Color;
use revue::widget::syntax::{HighlightSpan, Language, SyntaxTheme};
use revue::widget::TreeSitterHighlighter;

/// A theme whose every color is distinct: `rgb(n, 0, 0)` with `n` the
/// field's place in `SyntaxTheme`
fn numbered() -> SyntaxTheme {
    SyntaxTheme {
        keyword: Color::rgb(1, 0, 0),
        type_name: Color::rgb(2, 0, 0),
        function: Color::rgb(3, 0, 0),
        string: Color::rgb(4, 0, 0),
        number: Color::rgb(5, 0, 0),
        comment: Color::rgb(6, 0, 0),
        operator: Color::rgb(7, 0, 0),
        punctuation: Color::rgb(8, 0, 0),
        constant: Color::rgb(9, 0, 0),
        macro_call: Color::rgb(10, 0, 0),
        attribute: Color::rgb(11, 0, 0),
        variable: Color::rgb(12, 0, 0),
    }
}

/// Spans as (covered text, theme field number, "B" bold, "I" italic); span
/// offsets are characters
fn described(line: &str, spans: &[HighlightSpan]) -> Vec<(String, u8, String)> {
    spans
        .iter()
        .map(|s| {
            let style = format!(
                "{}{}",
                if s.bold { "B" } else { "" },
                if s.italic { "I" } else { "" }
            );
            let text = line.chars().skip(s.start).take(s.end - s.start).collect();
            (text, s.fg.r, style)
        })
        .collect()
}

/// `line` highlighted as `language` with the numbered theme, described
fn highlighted(language: Language, line: &str) -> Vec<(String, u8, String)> {
    let mut hl = TreeSitterHighlighter::with_theme(language, numbered());
    described(line, &hl.highlight_line(line))
}

fn d(text: &str, field: u8, style: &str) -> (String, u8, String) {
    (text.to_string(), field, style.to_string())
}

// =========================================================================
// TreeSitterHighlighter construction tests
// =========================================================================

#[test]
fn test_tree_sitter_rust_highlight() {
    let mut hl = TreeSitterHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("fn main() {");
    // 'fn' in the default theme's keyword color, bold
    let theme = SyntaxTheme::default();
    assert_eq!((spans[0].start, spans[0].end), (0, 2));
    assert_eq!(spans[0].fg, theme.keyword);
    assert!(spans[0].bold && !spans[0].italic);
    assert_eq!(
        highlighted(Language::Rust, "fn main() {"),
        [d("fn", 1, "B"), d("(", 8, ""), d(")", 8, ""), d("{", 8, "")]
    );
}

#[test]
fn test_tree_sitter_python_highlight() {
    assert_eq!(
        highlighted(Language::Python, "def hello():"),
        [d("def", 1, "B"), d("hello", 3, "")]
    );
}

#[test]
fn test_tree_sitter_javascript_highlight() {
    assert_eq!(
        highlighted(Language::JavaScript, "const x = 42;"),
        [
            d("const", 1, "B"),
            d("x", 12, ""),
            d("=", 7, ""),
            d("42", 5, ""),
            d(";", 8, ""),
        ]
    );
}

#[test]
fn test_tree_sitter_json_highlight() {
    // Keys and values are both strings
    assert_eq!(
        highlighted(Language::Json, "{\"key\": \"value\"}"),
        [d("\"key\"", 4, ""), d("\"value\"", 4, "")]
    );
}

#[test]
fn test_tree_sitter_no_language() {
    let mut hl = TreeSitterHighlighter::new(Language::None);
    assert!(hl.highlight_line("fn main() {}").is_empty());
    // One empty span list per line
    let lines = hl.highlight_code("fn main() {\n}\n");
    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(Vec::is_empty));
}

#[test]
fn test_tree_sitter_multiline() {
    let mut hl = TreeSitterHighlighter::with_theme(Language::Rust, numbered());
    let code = "fn main() {\n    println!(\"hello\");\n}";
    let lines: Vec<&str> = code.lines().collect();
    let spans = hl.highlight_code(code);
    assert_eq!(spans.len(), 3);
    // Offsets are relative to each line; with the whole function parsed,
    // 'main' is a function
    assert_eq!(
        described(lines[0], &spans[0]),
        [
            d("fn", 1, "B"),
            d("main", 3, ""),
            d("(", 8, ""),
            d(")", 8, ""),
            d("{", 8, "")
        ]
    );
    assert_eq!(
        described(lines[1], &spans[1]),
        [
            d("println", 3, ""),
            d("!", 3, ""),
            d("(", 8, ""),
            d("\"hello\"", 4, ""),
            d(")", 8, ""),
            d(";", 8, "")
        ]
    );
    assert_eq!(described(lines[2], &spans[2]), [d("}", 8, "")]);
}

#[test]
fn test_is_supported() {
    let all = [
        Language::Rust,
        Language::Python,
        Language::JavaScript,
        Language::Json,
        Language::Toml,
        Language::Yaml,
        Language::Markdown,
        Language::Shell,
        Language::Sql,
        Language::Html,
        Language::Css,
        Language::Go,
    ];
    for language in all {
        assert!(
            TreeSitterHighlighter::is_supported(language),
            "{language:?}"
        );
    }
    assert!(!TreeSitterHighlighter::is_supported(Language::None));
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_tree_sitter_with_theme() {
    let mut hl = TreeSitterHighlighter::with_theme(Language::Rust, numbered());
    let spans = hl.highlight_line("fn test() {}");
    assert_eq!(spans[0].fg, Color::rgb(1, 0, 0));
    // Not the default theme's color
    assert_ne!(spans[0].fg, SyntaxTheme::default().keyword);
}

#[test]
fn test_tree_sitter_language_builder() {
    // `const` is a keyword in JavaScript, a name in Python
    let mut hl = TreeSitterHighlighter::with_theme(Language::Python, numbered())
        .language(Language::JavaScript);
    let line = "const x = 42;";
    assert_eq!(
        described(line, &hl.highlight_line(line))[0],
        d("const", 1, "B")
    );
    assert_ne!(highlighted(Language::Python, line)[0], d("const", 1, "B"));
}

#[test]
fn test_tree_sitter_theme_builder() {
    let mut hl = TreeSitterHighlighter::new(Language::Rust).theme(numbered());
    let line = "fn test() {}";
    assert_eq!(
        described(line, &hl.highlight_line(line)),
        highlighted(Language::Rust, line)
    );
}

#[test]
fn test_tree_sitter_builder_chain() {
    let theme = SyntaxTheme {
        keyword: Color::MAGENTA,
        ..Default::default()
    };

    let mut hl = TreeSitterHighlighter::new(Language::Rust)
        .language(Language::Python)
        .theme(theme);
    let spans = hl.highlight_line("def test():");
    // Python's `def`, in the chained theme's keyword color
    assert_eq!((spans[0].start, spans[0].end), (0, 3));
    assert_eq!(spans[0].fg, Color::MAGENTA);
    assert_eq!(spans[1].fg, SyntaxTheme::default().function);
}

// Nested captures, and offsets in characters (#1088)

#[test]
fn text_after_a_nested_capture_keeps_the_outer_color() {
    // The escape is a capture inside the string: after it, `b"` is still
    // string
    let spans = highlighted(Language::Rust, r#"let s = "a\nb";"#);
    let strings: String = spans
        .iter()
        .filter(|s| s.1 == 4)
        .map(|s| s.0.as_str())
        .collect();
    assert!(strings.ends_with("b\""), "{spans:?}");
    assert!(strings.starts_with("\"a"), "{spans:?}");
}

#[test]
fn columns_are_characters() {
    let mut hl = TreeSitterHighlighter::with_theme(Language::Python, numbered());
    let spans = hl.highlight_line("한 = \"x\"");
    let s = spans.iter().find(|s| s.fg.r == 4).unwrap();
    assert_eq!((s.start, s.end), (4, 7));
    // The same in a file's second line
    let lines = hl.highlight_code("x = 1\n한글 = \"é\"\n");
    let s = lines[1].iter().find(|s| s.fg.r == 4).unwrap();
    assert_eq!((s.start, s.end), (5, 8));
}
