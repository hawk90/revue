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
        self.clear_selection();
        self.blocks.clear();
        // Inside a fence: its language. Each line of a fence is a code block
        // of its own, as the editor keeps one line per block, so the lines
        // are drawn and edited on rows of their own (#1076); export joins
        // them into one fence again.
        let mut fence: Option<Option<String>> = None;
        let mut fence_lines = 0usize;
        let code_line = |text: &str, lang: &Option<String>| {
            let mut block = typed_block(BlockType::CodeBlock, text);
            block.language = lang.clone();
            block
        };

        for line in markdown.lines() {
            if let Some(lang_suffix) = line.strip_prefix("```") {
                match fence.take() {
                    // An empty fence is still a code block, of one empty line
                    Some(lang) => {
                        if fence_lines == 0 {
                            self.blocks.push(code_line("", &lang));
                        }
                    }
                    None => {
                        fence = Some((!lang_suffix.is_empty()).then(|| lang_suffix.to_string()));
                        fence_lines = 0;
                    }
                }
                continue;
            }

            if let Some(lang) = &fence {
                self.blocks.push(code_line(line, lang));
                fence_lines += 1;
                continue;
            }

            // Parse block type from line
            let block = Self::parse_markdown_line(line);
            self.blocks.push(block);
        }

        // A fence never closed runs to the end of the text, as in CommonMark:
        // its lines are kept (#1076)
        if let Some(lang) = fence {
            if fence_lines == 0 {
                self.blocks.push(code_line("", &lang));
            }
        }

        if self.blocks.is_empty() {
            self.blocks.push(Block::paragraph(""));
        }
        self.cursor = (0, 0);
        self.scroll.scroll_to_top();
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

        // Numbered list: digits, then ". " (#1076: any line starting with a
        // digit and holding ". " anywhere was one)
        let digits = line.bytes().take_while(u8::is_ascii_digit).count();
        if digits > 0 {
            if let Some(rest) = line[digits..].strip_prefix(". ") {
                return typed_block(BlockType::NumberedList, rest);
            }
        }

        // Regular paragraph
        Block::paragraph(line)
    }
}
