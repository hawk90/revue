//! Tests for rich_text_editor core module

use revue::widget::form::rich_text_editor::RichTextEditor;

    // =========================================================================
    // RichTextEditor construction tests
    // =========================================================================

    #[test]
    fn test_rich_text_editor_new() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_rich_text_editor_default() {
        let editor = RichTextEditor::default();
        assert_eq!(editor.blocks.len(), 1);
    }

    // =========================================================================
    // Content builder tests
    // =========================================================================

    #[test]
    fn test_rich_text_editor_content_plain() {
        let editor = RichTextEditor::new().content("Hello world");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_rich_text_editor_content_multiline() {
        let editor = RichTextEditor::new().content("Line 1\nLine 2\nLine 3");
        assert_eq!(editor.blocks.len(), 3);
    }

    #[test]
    fn test_rich_text_editor_content_empty() {
        let editor = RichTextEditor::new().content("");
        // Empty content should still have one block
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_rich_text_editor_from_markdown() {
        let editor = RichTextEditor::new().from_markdown("# Heading\n\nParagraph");
        // Creates 3 blocks: heading, empty paragraph, actual paragraph
        assert_eq!(editor.blocks.len(), 3);
    }

    #[test]
    fn test_rich_text_editor_from_markdown_empty() {
        let editor = RichTextEditor::new().from_markdown("");
        assert_eq!(editor.blocks.len(), 1);
    }

    // =========================================================================
    // Markdown parsing tests
    // =========================================================================

    #[test]
    fn test_markdown_heading1() {
        let editor = RichTextEditor::new().from_markdown("# Title");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_heading2() {
        let editor = RichTextEditor::new().from_markdown("## Subtitle");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_heading3() {
        let editor = RichTextEditor::new().from_markdown("### Section");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_bullet_list() {
        let editor = RichTextEditor::new().from_markdown("- Item 1\n- Item 2");
        assert_eq!(editor.blocks.len(), 2);
    }

    #[test]
    fn test_markdown_numbered_list() {
        let editor = RichTextEditor::new().from_markdown("1. First\n2. Second");
        assert_eq!(editor.blocks.len(), 2);
    }

    #[test]
    fn test_markdown_quote() {
        let editor = RichTextEditor::new().from_markdown("> Quote text");
        assert_eq!(editor.blocks.len(), 1);
    }

    #[test]
    fn test_markdown_code_block() {
        let editor = RichTextEditor::new().from_markdown("```\ncode here\n```");
