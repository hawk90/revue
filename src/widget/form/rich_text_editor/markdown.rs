//! Markdown import for RichTextEditor: one block per line, fenced code
//! blocks gathered into one block.

use super::{Block, BlockType, RichTextEditor};

/// Line prefixes that open a typed block, in the order they are tried:
/// longer heading markers first, so `## ` is not read as `# `.
const LINE_PREFIXES: [(&str, BlockType); 9] = [
    ("###### ", BlockType::Heading6),
    ("##### ", BlockType::Heading5),
    ("#### ", BlockType::Heading4),
    ("### ", BlockType::Heading3),
    ("## ", BlockType::Heading2),
    ("# ", BlockType::Heading1),
    ("> ", BlockType::Quote),
    ("- ", BlockType::BulletList),
    ("* ", BlockType::BulletList),
];

/// A block of `block_type` holding `text`
fn typed_block(block_type: BlockType, text: &str) -> Block {
    let mut block = Block::new(block_type);
    block.set_text(text);
    block
}

impl RichTextEditor {
    /// Parse markdown content
    pub(super) fn parse_markdown(&mut self, markdown: &str) {
        self.forget_history();
        self.blocks.clear();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();

        for line in markdown.lines() {
            if let Some(lang_suffix) = line.strip_prefix("```") {
                if in_code_block {
                    // End code block
                    let mut block = typed_block(BlockType::CodeBlock, &code_block_content);
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
            let block = Self::parse_markdown_line(line);
            self.blocks.push(block);
        }

        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll = 0;
    }

    /// Parse a single markdown line
    fn parse_markdown_line(line: &str) -> Block {
        // Horizontal rule
        if line == "---" || line == "***" || line == "___" {
            return Block::new(BlockType::HorizontalRule);
        }

        // Headings, quote and bullet list
        for (prefix, block_type) in LINE_PREFIXES {
            if let Some(rest) = line.strip_prefix(prefix) {
                return typed_block(block_type, rest);
            }
        }

        // Numbered list
        if line.len() > 2 {
            let first_char = line.chars().next().unwrap_or(' ');
            if first_char.is_ascii_digit() {
                if let Some(idx) = line.find(". ") {
                    return typed_block(BlockType::NumberedList, &line[idx + 2..]);
                }
            }
        }

        // Regular paragraph
        Block::paragraph(line)
    }
}
