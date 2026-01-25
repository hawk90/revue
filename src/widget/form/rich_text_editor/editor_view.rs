//! Editor view rendering for RichTextEditor

use super::{BlockType, RichTextEditor};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl RichTextEditor {
    /// Render editor
    pub(crate) fn render_editor(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        let bg = self.bg.unwrap_or(Color::rgb(30, 30, 46));
        let fg = self.fg.unwrap_or(Color::rgb(205, 214, 244));

        // Fill editor background
        for row in 0..height {
            for col in 0..width {
                ctx.buffer.set(x + col, y + row, Cell::new(' ').bg(bg));
            }
        }

        // Render visible blocks
        for (row, block_idx) in (self.scroll..).take(height as usize).enumerate() {
            if block_idx >= self.blocks.len() {
                break;
            }

            let block = &self.blocks[block_idx];
            let row_y = y + row as u16;

            // Block type indicator
            let prefix = match block.block_type {
                BlockType::Heading1 => "# ",
                BlockType::Heading2 => "## ",
                BlockType::Heading3 => "### ",
                BlockType::Heading4 => "#### ",
                BlockType::Heading5 => "##### ",
                BlockType::Heading6 => "###### ",
                BlockType::Quote => "> ",
                BlockType::BulletList => "• ",
                BlockType::NumberedList => "1. ",
                BlockType::CodeBlock => "` ",
                BlockType::HorizontalRule => "──",
                BlockType::Paragraph => "",
            };

            let prefix_fg = match block.block_type {
                BlockType::Heading1
                | BlockType::Heading2
                | BlockType::Heading3
                | BlockType::Heading4
                | BlockType::Heading5
                | BlockType::Heading6 => self.heading_fg,
                BlockType::Quote => self.quote_fg,
                BlockType::CodeBlock => self.code_bg,
                _ => fg,
            };

            // Render prefix
            let mut col = x;
            for ch in prefix.chars() {
                if col < x + width {
                    ctx.buffer
                        .set(col, row_y, Cell::new(ch).fg(prefix_fg).bg(bg));
                    col += 1;
                }
            }

            // Render block content
            let text = block.text();
            for (char_idx, ch) in text.chars().enumerate() {
                if col >= x + width {
                    break;
                }

                let is_cursor =
                    self.focused && block_idx == self.cursor.0 && char_idx == self.cursor.1;

                let is_selected = self.anchor.is_some_and(|anchor| {
                    let (start, end) = if anchor < self.cursor {
                        (anchor, self.cursor)
                    } else {
                        (self.cursor, anchor)
                    };
                    block_idx >= start.0
                        && block_idx <= end.0
                        && (block_idx > start.0 || char_idx >= start.1)
                        && (block_idx < end.0 || char_idx < end.1)
                });

                let cell_bg = if is_cursor {
                    self.cursor_bg
                } else if is_selected {
                    self.selection_bg
                } else {
                    bg
                };

                ctx.buffer.set(col, row_y, Cell::new(ch).fg(fg).bg(cell_bg));
                col += 1;
            }

            // Render cursor at end of line
            if self.focused
                && block_idx == self.cursor.0
                && self.cursor.1 >= text.len()
                && col < x + width
            {
                ctx.buffer
                    .set(col, row_y, Cell::new(' ').bg(self.cursor_bg));
            }
        }
    }
}
