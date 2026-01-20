//! Block types and content for rich text editor

use super::text_format::TextFormat;

/// Block type for paragraphs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BlockType {
    /// Normal paragraph
    #[default]
    Paragraph,
    /// Heading level 1
    Heading1,
    /// Heading level 2
    Heading2,
    /// Heading level 3
    Heading3,
    /// Heading level 4
    Heading4,
    /// Heading level 5
    Heading5,
    /// Heading level 6
    Heading6,
    /// Blockquote
    Quote,
    /// Code block
    CodeBlock,
    /// Unordered list item
    BulletList,
    /// Ordered list item
    NumberedList,
    /// Horizontal rule
    HorizontalRule,
}

impl BlockType {
    /// Get markdown prefix for this block type
    pub fn markdown_prefix(&self) -> &'static str {
        match self {
            BlockType::Paragraph => "",
            BlockType::Heading1 => "# ",
            BlockType::Heading2 => "## ",
            BlockType::Heading3 => "### ",
            BlockType::Heading4 => "#### ",
            BlockType::Heading5 => "##### ",
            BlockType::Heading6 => "###### ",
            BlockType::Quote => "> ",
            BlockType::CodeBlock => "```\n",
            BlockType::BulletList => "- ",
            BlockType::NumberedList => "1. ",
            BlockType::HorizontalRule => "---",
        }
    }
}

/// A formatted text span
#[derive(Clone, Debug)]
pub struct FormattedSpan {
    /// Text content
    pub text: String,
    /// Format applied
    pub format: TextFormat,
}

impl FormattedSpan {
    /// Create new span
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            format: TextFormat::default(),
        }
    }

    /// Set format
    pub fn with_format(mut self, format: TextFormat) -> Self {
        self.format = format;
        self
    }
}

/// A line/block in the document
#[derive(Clone, Debug)]
pub struct Block {
    /// Block type
    pub block_type: BlockType,
    /// Spans of formatted text
    pub spans: Vec<FormattedSpan>,
    /// Language for code blocks
    pub language: Option<String>,
}

impl Block {
    /// Create new paragraph
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self {
            block_type: BlockType::Paragraph,
            spans: vec![FormattedSpan::new(text)],
            language: None,
        }
    }

    /// Create new block with type
    pub fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            spans: vec![FormattedSpan::new("")],
            language: None,
        }
    }

    /// Get plain text content
    pub fn text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_str()).collect()
    }

    /// Set text content (single span)
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.spans = vec![FormattedSpan::new(text)];
    }

    /// Get text length
    pub fn len(&self) -> usize {
        self.spans.iter().map(|s| s.text.len()).sum()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        let prefix = self.block_type.markdown_prefix();
        let text = self.spans_to_markdown();

        match self.block_type {
            BlockType::CodeBlock => {
                let lang = self.language.as_deref().unwrap_or("");
                format!("```{}\n{}\n```", lang, text)
            }
            BlockType::HorizontalRule => "---".to_string(),
            _ => format!("{}{}", prefix, text),
        }
    }

    /// Convert spans to markdown
    fn spans_to_markdown(&self) -> String {
        self.spans
            .iter()
            .map(|span| {
                let mut text = span.text.clone();
                if span.format.code {
                    text = format!("`{}`", text);
                }
                if span.format.bold {
                    text = format!("**{}**", text);
                }
                if span.format.italic {
                    text = format!("*{}*", text);
                }
                if span.format.strikethrough {
                    text = format!("~~{}~~", text);
                }
                text
            })
            .collect()
    }
}
