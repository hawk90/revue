//! Preview mode rendering for RichTextEditor

use super::view::fill;
use super::{BlockType, RichTextEditor};
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl RichTextEditor {
    /// Render preview
    pub(super) fn render_preview(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(super::EDITOR_FG));

        // Fill preview background
        fill(ctx, x, y, width, height, self.preview_bg);

        // Render blocks as formatted text
        for (row, block_idx) in (self.scroll..).take(height as usize).enumerate() {
            if block_idx >= self.blocks.len() {
                break;
            }

            let block = &self.blocks[block_idx];
            let line = Rect::new(x, y + row as u16, width, 1);
            let text = block.text();
            let links = super::link::link_char_ranges(&text);
            let text_fg = |i: usize| {
                if links.iter().any(|r| r.contains(&i)) {
                    self.link_fg
                } else {
                    fg
                }
            };

            match block.block_type {
                BlockType::Heading1 => {
                    self.render_heading(ctx, x, line.y, width, &text, 1);
                }
                BlockType::Heading2 => {
                    self.render_heading(ctx, x, line.y, width, &text, 2);
                }
                BlockType::Heading3 => {
                    self.render_heading(ctx, x, line.y, width, &text, 3);
                }
                BlockType::Quote => {
                    ctx.set(
                        x,
                        line.y,
                        Cell::new('│').fg(self.quote_fg).bg(self.preview_bg),
                    );
                    self.draw_preview_text(ctx, line, x + 2, &text, |_| self.quote_fg);
                }
                BlockType::BulletList => {
                    ctx.set(x, line.y, Cell::new('•').fg(fg).bg(self.preview_bg));
                    self.draw_preview_text(ctx, line, x + 2, &text, text_fg);
                }
                BlockType::NumberedList => {
                    self.draw_preview_numbered(ctx, line, block_idx, &text, fg, text_fg);
                }
                BlockType::CodeBlock => {
                    self.draw_preview_code(ctx, line, &text, fg);
                }
                BlockType::HorizontalRule => {
                    self.draw_preview_rule(ctx, line);
                }
                _ => {
                    self.draw_preview_text(ctx, line, x, &text, text_fg);
                }
            }
        }
    }

    /// Draw `text` from column `from` to the end of `line`, one column per
    /// character, the i-th character in `fg(i)`.
    fn draw_preview_text(
        &self,
        ctx: &mut RenderContext,
        line: Rect,
        from: u16,
        text: &str,
        fg: impl Fn(usize) -> Color,
    ) {
        for (i, (col, ch)) in (from..).zip(text.chars()).enumerate() {
            if col >= line.x + line.width {
                break;
            }
            ctx.set(col, line.y, Cell::new(ch).fg(fg(i)).bg(self.preview_bg));
        }
    }

    /// A numbered list item: its number - the block's position in the
    /// document - a dot, and the text.
    fn draw_preview_numbered(
        &self,
        ctx: &mut RenderContext,
        line: Rect,
        block_idx: usize,
        text: &str,
        fg: Color,
        text_fg: impl Fn(usize) -> Color,
    ) {
        let x = line.x;
        let num = (block_idx + 1).to_string();
        let mut col = x;
        for ch in num.chars() {
            if col < x + line.width {
                ctx.set(col, line.y, Cell::new(ch).fg(fg).bg(self.preview_bg));
                col += 1;
            }
        }
        ctx.set(col, line.y, Cell::new('.').fg(fg).bg(self.preview_bg));
        self.draw_preview_text(ctx, line, col + 2, text, text_fg);
    }

    /// A horizontal rule across the line
    fn draw_preview_rule(&self, ctx: &mut RenderContext, line: Rect) {
        for col in 0..line.width {
            ctx.set(
                line.x + col,
                line.y,
                Cell::new('─').fg(self.quote_fg).bg(self.preview_bg),
            );
        }
    }

    /// A code block's row: the code background across the line and the text
    /// inset by one column on each side.
    fn draw_preview_code(&self, ctx: &mut RenderContext, line: Rect, text: &str, fg: Color) {
        let Rect { x, y, width, .. } = line;
        for col in 0..width {
            ctx.set(x + col, y, Cell::new(' ').bg(self.code_bg));
        }
        for (col, ch) in (x + 1..).zip(text.chars()) {
            if col >= x + width - 1 {
                break;
            }
            ctx.set(col, y, Cell::new(ch).fg(fg).bg(self.code_bg));
        }
    }

    /// Render heading in preview
    fn render_heading(
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

        for (col, ch) in (x..).zip(text.chars()) {
            if col >= x + width {
                break;
            }
            let mut cell = Cell::new(ch).fg(self.heading_fg).bg(self.preview_bg);
            cell.modifier = modifier;
            ctx.set(col, y, cell);
        }
    }
}
