//! Tree-sitter syntax highlighter tests

#[cfg(feature = "syntax-highlighting")]
mod tests {
    use revue::widget::syntax::Language;
    use revue::widget::developer::TreeSitterHighlighter;

    // =========================================================================
    // TreeSitterHighlighter construction tests
    // =========================================================================

    #[test]
    fn test_tree_sitter_rust_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::Rust);
        let spans = hl.highlight_line("fn main() {");
        // Should have at least the keyword 'fn' highlighted
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_python_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::Python);
        let spans = hl.highlight_line("def hello():");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_javascript_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::JavaScript);
        let spans = hl.highlight_line("const x = 42;");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_json_highlight() {
        let mut hl = TreeSitterHighlighter::new(Language::Json);
        let spans = hl.highlight_line("{\"key\": \"value\"}");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_no_language() {
        let mut hl = TreeSitterHighlighter::new(Language::None);
        let spans = hl.highlight_line("fn main() {}");
        assert!(spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_multiline() {
        let mut hl = TreeSitterHighlighter::new(Language::Rust);
        let code = "fn main() {\n    println!(\"hello\");\n}";
        let line_spans = hl.highlight_code(code);
        assert_eq!(line_spans.len(), 3);
    }

    #[test]
    fn test_is_supported() {
        assert!(TreeSitterHighlighter::is_supported(Language::Rust));
        assert!(TreeSitterHighlighter::is_supported(Language::Python));
        assert!(!TreeSitterHighlighter::is_supported(Language::None));
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_tree_sitter_with_theme() {
        use revue::widget::syntax::SyntaxTheme;

        let theme = SyntaxTheme::default();
        let hl = TreeSitterHighlighter::with_theme(Language::Rust, theme);
        // Just verify it compiles
        let _ = hl.highlight_line("fn test() {}");
    }

    #[test]
    fn test_tree_sitter_language_builder() {
        let hl = TreeSitterHighlighter::new(Language::Python).language(Language::JavaScript);
        // Just verify it compiles
        let _ = hl.highlight_line("const x = 42;");
    }

    #[test]
    fn test_tree_sitter_theme_builder() {
        use revue::widget::syntax::SyntaxTheme;

        let hl = TreeSitterHighlighter::new(Language::Rust).theme(SyntaxTheme::default());
        // Just verify it compiles
        let _ = hl.highlight_line("fn test() {}");
    }

    #[test]
    fn test_tree_sitter_builder_chain() {
        use revue::widget::syntax::SyntaxTheme;
        use revue::style::Color;

        let mut theme = SyntaxTheme::default();
        theme.keyword = Color::MAGENTA;

        let hl = TreeSitterHighlighter::new(Language::Rust)
            .language(Language::Python)
            .theme(theme);
        // Just verify it compiles
        let _ = hl.highlight_line("def test():");
    }
}
