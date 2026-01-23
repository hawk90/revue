//! Tests for syntax highlighting

use crate::widget::syntax::types::{Language, SyntaxTheme};
use crate::widget::syntax::SyntaxHighlighter;

#[test]
fn test_language_detection() {
    assert_eq!(Language::from_extension("rs"), Language::Rust);
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("js"), Language::JavaScript);
    assert_eq!(Language::from_extension("json"), Language::Json);
    assert_eq!(Language::from_extension("unknown"), Language::None);
}

#[test]
fn test_rust_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("fn main() {");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_comment_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("// This is a comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_rust_string_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let s = \"hello\";");
    assert!(!spans.is_empty());
}

#[test]
fn test_python_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("def hello():");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"key\": \"value\"}");
    assert!(!spans.is_empty());
}

#[test]
fn test_no_highlighting() {
    let hl = SyntaxHighlighter::new(Language::None);
    let spans = hl.highlight_line("fn main() {}");
    assert!(spans.is_empty());
}

#[test]
fn test_syntax_theme() {
    let dark = SyntaxTheme::dark();
    let light = SyntaxTheme::light();
    let monokai = SyntaxTheme::monokai();

    // Just check they're different
    assert_ne!(dark.keyword.r, light.keyword.r);
    assert_ne!(dark.keyword.r, monokai.keyword.r);
}
