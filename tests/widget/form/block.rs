//! Tests for rich_text_editor block module

use revue::widget::form::rich_text_editor::block::{Block, BlockType, FormattedSpan, TextFormat};

    // =========================================================================
    // BlockType enum tests
    // =========================================================================

    #[test]
    fn test_block_type_default() {
        assert_eq!(BlockType::default(), BlockType::Paragraph);
    }

    #[test]
    fn test_block_type_clone() {
        let bt = BlockType::Heading1;
        assert_eq!(bt, bt.clone());
    }

    #[test]
    fn test_block_type_copy() {
        let bt1 = BlockType::Quote;
        let bt2 = bt1;
        assert_eq!(bt1, BlockType::Quote);
        assert_eq!(bt2, BlockType::Quote);
    }

    #[test]
    fn test_block_type_equality() {
        assert_eq!(BlockType::Paragraph, BlockType::Paragraph);
        assert_eq!(BlockType::Heading1, BlockType::Heading1);
        assert_ne!(BlockType::Heading1, BlockType::Heading2);
        assert_ne!(BlockType::BulletList, BlockType::NumberedList);
    }

    #[test]
    fn test_block_type_debug() {
        let debug_str = format!("{:?}", BlockType::CodeBlock);
        assert!(debug_str.contains("CodeBlock"));
    }

    #[test]
    fn test_block_type_markdown_prefix() {
        assert_eq!(BlockType::Paragraph.markdown_prefix(), "");
        assert_eq!(BlockType::Heading1.markdown_prefix(), "# ");
        assert_eq!(BlockType::Heading2.markdown_prefix(), "## ");
        assert_eq!(BlockType::Heading3.markdown_prefix(), "### ");
        assert_eq!(BlockType::Heading4.markdown_prefix(), "#### ");
        assert_eq!(BlockType::Heading5.markdown_prefix(), "##### ");
        assert_eq!(BlockType::Heading6.markdown_prefix(), "###### ");
        assert_eq!(BlockType::Quote.markdown_prefix(), "> ");
        assert_eq!(BlockType::CodeBlock.markdown_prefix(), "```\n");
        assert_eq!(BlockType::BulletList.markdown_prefix(), "- ");
        assert_eq!(BlockType::NumberedList.markdown_prefix(), "1. ");
        assert_eq!(BlockType::HorizontalRule.markdown_prefix(), "---");
    }

    #[test]
    fn test_block_type_all_heading_variants() {
        assert_ne!(BlockType::Heading1, BlockType::Heading2);
        assert_ne!(BlockType::Heading2, BlockType::Heading3);
        assert_ne!(BlockType::Heading3, BlockType::Heading4);
        assert_ne!(BlockType::Heading4, BlockType::Heading5);
        assert_ne!(BlockType::Heading5, BlockType::Heading6);
    }

    #[test]
    fn test_block_type_all_list_variants() {
        assert_ne!(BlockType::BulletList, BlockType::NumberedList);
        assert_eq!(BlockType::BulletList, BlockType::BulletList);
        assert_eq!(BlockType::NumberedList, BlockType::NumberedList);
    }

    // =========================================================================
    // FormattedSpan tests
    // =========================================================================

    #[test]
    fn test_formatted_span_new() {
        let span = FormattedSpan::new("test");
        assert_eq!(span.text, "test");
        assert!(!span.format.bold);
        assert!(!span.format.italic);
    }

    #[test]
    fn test_formatted_span_clone() {
        let span1 = FormattedSpan::new("test");
        let span2 = span1.clone();
        assert_eq!(span1.text, span2.text);
    }

    #[test]
    fn test_formatted_span_debug() {
        let span = FormattedSpan::new("test");
        let debug_str = format!("{:?}", span);
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_formatted_span_with_format() {
        let mut fmt = TextFormat::default();
        fmt.bold = true;
        fmt.italic = true;

        let span = FormattedSpan::new("test").with_format(fmt);
        assert!(span.format.bold);
        assert!(span.format.italic);
    }

    #[test]
    fn test_formatted_span_empty() {
        let span = FormattedSpan::new("");
        assert_eq!(span.text, "");
    }

    // =========================================================================
    // Block::paragraph tests
    // =========================================================================

    #[test]
    fn test_block_paragraph() {
        let block = Block::paragraph("Hello world");
        assert_eq!(block.block_type, BlockType::Paragraph);
        assert_eq!(block.text(), "Hello world");
        assert!(block.language.is_none());
    }

    #[test]
    fn test_block_paragraph_empty() {
        let block = Block::paragraph("");
        assert_eq!(block.block_type, BlockType::Paragraph);
        assert_eq!(block.text(), "");
        assert!(block.is_empty());
    }

    #[test]
    fn test_block_paragraph_with_string() {
        let s = String::from("owned string");
        let block = Block::paragraph(s);
        assert_eq!(block.text(), "owned string");
    }

    // =========================================================================
    // Block::new tests
    // =========================================================================

    #[test]
    fn test_block_new_heading1() {
        let block = Block::new(BlockType::Heading1);
        assert_eq!(block.block_type, BlockType::Heading1);
        assert_eq!(block.text(), "");
    }

    #[test]
    fn test_block_new_quote() {
        let block = Block::new(BlockType::Quote);
        assert_eq!(block.block_type, BlockType::Quote);
    }

    #[test]
    fn test_block_new_code_block() {
        let block = Block::new(BlockType::CodeBlock);
        assert_eq!(block.block_type, BlockType::CodeBlock);
    }

    #[test]
    fn test_block_new_bullet_list() {
        let block = Block::new(BlockType::BulletList);
        assert_eq!(block.block_type, BlockType::BulletList);
    }

    // =========================================================================
    // Block::text tests
    // =========================================================================

    #[test]
    fn test_block_text_single_span() {
        let block = Block::paragraph("test");
        assert_eq!(block.text(), "test");
    }

    #[test]
    fn test_block_text_multiple_spans() {
        let mut block = Block::paragraph("");
        block.spans = vec![
            FormattedSpan::new("Hello"),
            FormattedSpan::new(" "),
            FormattedSpan::new("world"),
        ];
        assert_eq!(block.text(), "Hello world");
    }

    // =========================================================================
    // Block::set_text tests
    // =========================================================================

    #[test]
    fn test_block_set_text() {
        let mut block = Block::paragraph("old");
        block.set_text("new");
        assert_eq!(block.text(), "new");
        assert_eq!(block.spans.len(), 1);
    }

    #[test]
    fn test_block_set_text_to_empty() {
        let mut block = Block::paragraph("something");
        block.set_text("");
        assert_eq!(block.text(), "");
        assert!(block.is_empty());
    }

    // =========================================================================
    // Block::len tests
    // =========================================================================

    #[test]
    fn test_block_len_empty() {
        let block = Block::paragraph("");
        assert_eq!(block.len(), 0);
    }

    #[test]
    fn test_block_len_single_span() {
        let block = Block::paragraph("hello");
        assert_eq!(block.len(), 5);
    }

    #[test]
    fn test_block_len_multiple_spans() {
        let mut block = Block::paragraph("");
        block.spans = vec![
            FormattedSpan::new("Hi"),
            FormattedSpan::new(" "),
            FormattedSpan::new("there"),
        ];
        assert_eq!(block.len(), 8); // "Hi" + " " + "there" = 2 + 1 + 5 = 8
    }

    // =========================================================================
    // Block::is_empty tests
    // =========================================================================

    #[test]
    fn test_block_is_empty_true() {
        let block = Block::paragraph("");
        assert!(block.is_empty());
    }

    #[test]
    fn test_block_is_empty_false() {
        let block = Block::paragraph("text");
        assert!(!block.is_empty());
    }

    #[test]
    fn test_block_is_empty_multiple_empty_spans() {
        let mut block = Block::paragraph("");
        block.spans = vec![FormattedSpan::new(""), FormattedSpan::new("")];
        assert!(block.is_empty());
    }

    // =========================================================================
    // Block::to_markdown tests
    // =========================================================================

    #[test]
    fn test_block_to_markdown_paragraph() {
        let block = Block::paragraph("Hello");
        assert_eq!(block.to_markdown(), "Hello");
    }

    #[test]
    fn test_block_to_markdown_heading1() {
        let mut block = Block::new(BlockType::Heading1);
        block.set_text("Title");
        assert_eq!(block.to_markdown(), "# Title");
    }

    #[test]
    fn test_block_to_markdown_heading2() {
        let mut block = Block::new(BlockType::Heading2);
        block.set_text("Subtitle");
        assert_eq!(block.to_markdown(), "## Subtitle");
    }

    #[test]
    fn test_block_to_markdown_quote() {
        let mut block = Block::new(BlockType::Quote);
        block.set_text("Famous quote");
        assert_eq!(block.to_markdown(), "> Famous quote");
    }

    #[test]
    fn test_block_to_markdown_bullet_list() {
        let mut block = Block::new(BlockType::BulletList);
        block.set_text("Item");
        assert_eq!(block.to_markdown(), "- Item");
    }

    #[test]
    fn test_block_to_markdown_numbered_list() {
        let mut block = Block::new(BlockType::NumberedList);
        block.set_text("Item");
        assert_eq!(block.to_markdown(), "1. Item");
    }

    #[test]
    fn test_block_to_markdown_horizontal_rule() {
        let block = Block::new(BlockType::HorizontalRule);
        assert_eq!(block.to_markdown(), "---");
    }

    #[test]
    fn test_block_to_markdown_code_block_no_lang() {
        let mut block = Block::new(BlockType::CodeBlock);
        block.set_text("code here");
        assert_eq!(block.to_markdown(), "```\ncode here\n```");
    }

    #[test]
    fn test_block_to_markdown_code_block_with_lang() {
        let mut block = Block::new(BlockType::CodeBlock);
        block.set_text("let x = 1;");
        block.language = Some("rust".to_string());
        assert_eq!(block.to_markdown(), "```rust\nlet x = 1;\n```");
    }

    #[test]
    fn test_block_to_markdown_with_bold() {
        let mut block = Block::paragraph("");
        block.spans[0].format.bold = true;
        block.spans[0].text = "bold".to_string();
        assert_eq!(block.to_markdown(), "**bold**");
    }

    #[test]
    fn test_block_to_markdown_with_italic() {
        let mut block = Block::paragraph("");
        block.spans[0].format.italic = true;
        block.spans[0].text = "italic".to_string();
        assert_eq!(block.to_markdown(), "*italic*");
    }

    #[test]
    fn test_block_to_markdown_with_strikethrough() {
        let mut block = Block::paragraph("");
        block.spans[0].format.strikethrough = true;
        block.spans[0].text = "deleted".to_string();
        assert_eq!(block.to_markdown(), "~~deleted~~");
    }

    #[test]
    fn test_block_to_markdown_with_code() {
        let mut block = Block::paragraph("");
        block.spans[0].format.code = true;
        block.spans[0].text = "code".to_string();
        assert_eq!(block.to_markdown(), "`code`");
    }

    #[test]
    fn test_block_to_markdown_with_multiple_formats() {
        let mut block = Block::paragraph("");
        block.spans[0].format.bold = true;
        block.spans[0].format.italic = true;
        block.spans[0].text = "text".to_string();
        // Order: bold applied first (**text**), then italic wraps it (***text***)
        assert_eq!(block.to_markdown(), "***text***");
    }
