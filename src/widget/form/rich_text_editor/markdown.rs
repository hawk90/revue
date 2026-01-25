//! Markdown parsing and export functionality for RichTextEditor

use super::{Block, BlockType, RichTextEditor};

impl RichTextEditor {
    /// Parse markdown content into blocks
    pub(crate) fn parse_markdown(&mut self, markdown: &str) {
        self.blocks.clear();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();

        for line in markdown.lines() {
            if let Some(lang_suffix) = line.strip_prefix("```") {
                if in_code_block {
                    // End code block
                    let mut block = Block::new(BlockType::CodeBlock);
                    block.set_text(&code_block_content);
                    block.language = if code_block_lang.is_empty() {
                        None
                    } else {
                        Some(code_block_lang.clone())
                    };
                    self.blocks.push(block);
                    code_block_content.clear();
                    code_block_lang.clear();
                    in_code_block = false;
                } else {
                    // Start code block
                    in_code_block = true;
                    code_block_lang = lang_suffix.to_string();
                }
                continue;
            }

            if in_code_block {
                if !code_block_content.is_empty() {
                    code_block_content.push('\n');
                }
                code_block_content.push_str(line);
                continue;
            }

            // Parse block type from line
            let block = self.parse_markdown_line(line);
            self.blocks.push(block);
        }

        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
    }

    /// Parse a single markdown line into a Block
    pub(crate) fn parse_markdown_line(&self, line: &str) -> Block {
        // Horizontal rule
        if line == "---" || line == "***" || line == "___" {
            return Block::new(BlockType::HorizontalRule);
        }

        // Headings
        if let Some(rest) = line.strip_prefix("###### ") {
            let mut block = Block::new(BlockType::Heading6);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("##### ") {
            let mut block = Block::new(BlockType::Heading5);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("#### ") {
            let mut block = Block::new(BlockType::Heading4);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("### ") {
            let mut block = Block::new(BlockType::Heading3);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            let mut block = Block::new(BlockType::Heading2);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("# ") {
            let mut block = Block::new(BlockType::Heading1);
            block.set_text(rest);
            return block;
        }

        // Quote
        if let Some(rest) = line.strip_prefix("> ") {
            let mut block = Block::new(BlockType::Quote);
            block.set_text(rest);
            return block;
        }

        // Bullet list
        if let Some(rest) = line.strip_prefix("- ") {
            let mut block = Block::new(BlockType::BulletList);
            block.set_text(rest);
            return block;
        }
        if let Some(rest) = line.strip_prefix("* ") {
            let mut block = Block::new(BlockType::BulletList);
            block.set_text(rest);
            return block;
        }

        // Numbered list
        if line.len() > 2 {
            let first_char = line.chars().next().unwrap_or(' ');
            if first_char.is_ascii_digit() {
                if let Some(idx) = line.find(". ") {
                    let mut block = Block::new(BlockType::NumberedList);
                    block.set_text(&line[idx + 2..]);
                    return block;
                }
            }
        }

        // Regular paragraph
        Block::paragraph(line)
    }
}
