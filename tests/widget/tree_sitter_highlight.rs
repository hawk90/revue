//! Tree-sitter syntax highlighter tests

#[cfg(feature = "syntax-highlighting")]
mod tests {
    use revue::widget::syntax::Language;
    use revue::widget::SyntaxHighlighter;

    // =========================================================================
    // SyntaxHighlighter construction tests
    // =========================================================================

    #[test]
    fn test_tree_sitter_rust_highlight() {
        let hl = SyntaxHighlighter::new(Language::Rust);
        let spans = hl.highlight_line("fn main() {");
        // Should have at least the keyword 'fn' highlighted
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_python_highlight() {
        let hl = SyntaxHighlighter::new(Language::Python);
        let spans = hl.highlight_line("def hello():");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_javascript_highlight() {
        let hl = SyntaxHighlighter::new(Language::JavaScript);
        let spans = hl.highlight_line("const x = 42;");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_json_highlight() {
        let hl = SyntaxHighlighter::new(Language::Json);
        let spans = hl.highlight_line("{\"key\": \"value\"}");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_no_language() {
        let hl = SyntaxHighlighter::new(Language::None);
        let spans = hl.highlight_line("fn main() {}");
        assert!(spans.is_empty());
    }

    #[test]
    fn test_tree_sitter_multiline() {
        let hl = SyntaxHighlighter::new(Language::Rust);
        let code = "fn main() {\n    println!(\"hello\");\n}";
        let lines: Vec<&str> = code.lines().collect();
        assert_eq!(lines.len(), 3);
        // Each line should be highlightable
        for line in &lines {
            let _spans = hl.highlight_line(line);
        }
    }

    #[test]
    fn test_is_supported() {
        // Language::None should return empty spans (unsupported / no-op)
        let hl_rust = SyntaxHighlighter::new(Language::Rust);
        let hl_python = SyntaxHighlighter::new(Language::Python);
        let hl_none = SyntaxHighlighter::new(Language::None);

        assert!(!hl_rust.highlight_line("fn test() {}").is_empty());
        assert!(!hl_python.highlight_line("def test():").is_empty());
        assert!(hl_none.highlight_line("anything").is_empty());
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_tree_sitter_with_theme() {
        use revue::widget::syntax::SyntaxTheme;

        let theme = SyntaxTheme::default();
        let hl = SyntaxHighlighter::with_theme(Language::Rust, theme);
        // Just verify it compiles
        let _ = hl.highlight_line("fn test() {}");
    }

    #[test]
    fn test_tree_sitter_language_builder() {
        let hl = SyntaxHighlighter::new(Language::Python).language(Language::JavaScript);
        // Just verify it compiles
        let _ = hl.highlight_line("const x = 42;");
    }

    #[test]
    fn test_tree_sitter_theme_builder() {
        use revue::widget::syntax::SyntaxTheme;

        let hl = SyntaxHighlighter::new(Language::Rust).theme(SyntaxTheme::default());
        // Just verify it compiles
        let _ = hl.highlight_line("fn test() {}");
    }

    #[test]
    fn test_tree_sitter_builder_chain() {
        use revue::style::Color;
        use revue::widget::syntax::SyntaxTheme;

        let mut theme = SyntaxTheme::default();
        theme.keyword = Color::MAGENTA;

        let hl = SyntaxHighlighter::new(Language::Rust)
            .language(Language::Python)
            .theme(theme);
        // Just verify it compiles
        let _ = hl.highlight_line("def test():");
    }
}
