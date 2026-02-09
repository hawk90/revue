//! Markdown widget helper functions

use super::Markdown;

/// Create a new markdown widget
pub fn markdown(source: impl Into<String>) -> Markdown {
    Markdown::new(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_function() {
        let md = markdown("# Hello");
        let _ = md;
    }

    #[test]
    fn test_markdown_function_with_string() {
        let md = markdown("**bold** text".to_string());
        let _ = md;
    }

    #[test]
    fn test_markdown_function_with_str() {
        let md = markdown("*italic* text");
        let _ = md;
    }

    // =========================================================================
    // Additional markdown helper tests
    // =========================================================================

    #[test]
    fn test_markdown_empty() {
        let md = markdown("");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_code_blocks() {
        let md = markdown("```rust\nlet x = 5;\n```");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_links() {
        let md = markdown("[Link](https://example.com)");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_lists() {
        let md = markdown("- Item 1\n- Item 2\n- Item 3");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_tables() {
        let md = markdown("| A | B |\n|---|---|\n| 1 | 2 |");
        let _ = md;
    }

    #[test]
    fn test_markdown_long_document() {
        let long_md = "# Title\n\nSection 1\n\nLorem ipsum dolor sit amet.\n\n## Subsection\n\nMore content here.";
        let md = markdown(long_md);
        let _ = md;
    }

    #[test]
    fn test_markdown_with_unicode() {
        let md = markdown("# 你好\n\n한글 text\n\n日本語");
        let _ = md;
    }

    #[test]
    fn test_markdown_multiple_calls() {
        let md1 = markdown("# First");
        let md2 = markdown("## Second");
        let md3 = markdown("### Third");
        let _ = md1;
        let _ = md2;
        let _ = md3;
    }

    #[test]
    fn test_markdown_with_blockquotes() {
        let md = markdown("> This is a quote\n> > Nested quote");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_horizontal_rule() {
        let md = markdown("---\n\nText after rule");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_inline_code() {
        let md = markdown("Use `print()` to output");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_strikethrough() {
        let md = markdown("~~deleted text~~");
        let _ = md;
    }

    #[test]
    fn test_markdown_with_task_lists() {
        let md = markdown("- [x] Done\n- [ ] Todo");
        let _ = md;
    }

    #[test]
    fn test_markdown_helpers_do_not_panic() {
        // All helper variations should work without panicking
        let _ = markdown("test");
        let _ = markdown("test".to_string());
        let _ = markdown(String::from("test"));
    }
}
