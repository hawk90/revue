//! Preview mode rendering for RichTextEditor

use super::{BlockType, RichTextEditor};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl RichTextEditor {
    /// Render preview
    pub(crate) fn render_preview(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        let fg = self.fg.unwrap_or(Color::rgb(205, 214, 244));

        // Fill preview background
        for row in 0..height {
            for col in 0..width {
                ctx.buffer
                    .set(x + col, y + row, Cell::new(' ').bg(self.preview_bg));
            }
        }

        // Render blocks as formatted text
        for (row, block_idx) in (self.scroll..).take(height as usize).enumerate() {
            if block_idx >= self.blocks.len() {
                break;
            }

            let block = &self.blocks[block_idx];
            let row_y = y + row as u16;

            match block.block_type {
                BlockType::Heading1 => {
                    self.render_heading(ctx, x, row_y, width, &block.text(), 1);
                }
                BlockType::Heading2 => {
                    self.render_heading(ctx, x, row_y, width, &block.text(), 2);
                }
                BlockType::Heading3 => {
                    self.render_heading(ctx, x, row_y, width, &block.text(), 3);
                }
                BlockType::Quote => {
                    let mut col = x;
                    ctx.buffer.set(
                        col,
                        row_y,
                        Cell::new('│').fg(self.quote_fg).bg(self.preview_bg),
                    );
                    col += 2;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer.set(
                            col,
                            row_y,
                            Cell::new(ch).fg(self.quote_fg).bg(self.preview_bg),
                        );
                        col += 1;
                    }
                }
                BlockType::BulletList => {
                    let mut col = x;
                    ctx.buffer
                        .set(col, row_y, Cell::new('•').fg(fg).bg(self.preview_bg));
                    col += 2;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                        col += 1;
                    }
                }
                BlockType::NumberedList => {
                    let num = (block_idx + 1).to_string();
                    let mut col = x;
                    for ch in num.chars() {
                        if col < x + width {
                            ctx.buffer
                                .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                            col += 1;
                        }
                    }
                    ctx.buffer
                        .set(col, row_y, Cell::new('.').fg(fg).bg(self.preview_bg));
                    col += 2;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                        col += 1;
                    }
                }
                BlockType::CodeBlock => {
                    for col in 0..width {
                        ctx.buffer
                            .set(x + col, row_y, Cell::new(' ').bg(self.code_bg));
                    }
                    let mut col = x + 1;
                    for ch in block.text().chars() {
                        if col >= x + width - 1 {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.code_bg));
                        col += 1;
                    }
                }
                BlockType::HorizontalRule => {
                    for col in 0..width {
                        ctx.buffer.set(
                            x + col,
                            row_y,
                            Cell::new('─').fg(self.quote_fg).bg(self.preview_bg),
                        );
                    }
                }
                _ => {
                    let mut col = x;
                    for ch in block.text().chars() {
                        if col >= x + width {
                            break;
                        }
                        ctx.buffer
                            .set(col, row_y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                        col += 1;
                    }
                }
            }
        }
    }

    /// Render heading in preview
    pub(crate) fn render_heading(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        text: &str,
        level: usize,
    ) {
        let modifier = if level == 1 {
            Modifier::BOLD | Modifier::UNDERLINE
        } else {
            Modifier::BOLD
        };

        let mut col = x;
        for ch in text.chars() {
            if col >= x + width {
                break;
            }
            let mut cell = Cell::new(ch).fg(self.heading_fg).bg(self.preview_bg);
            cell.modifier = modifier;
            ctx.buffer.set(col, y, cell);
            col += 1;
        }
    }
}
