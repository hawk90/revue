//! Editor view rendering for RichTextEditor

use super::view::fill;
use super::{BlockType, RichTextEditor, TextFormat};
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::RenderContext;

/// The marker drawn before a block's text in the editor
fn block_prefix(block_type: BlockType) -> &'static str {
    match block_type {
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
    }
}

/// The cell modifiers of a span's format
fn format_modifier(format: &TextFormat) -> Modifier {
    let mut modifier = Modifier::empty();
    if format.bold {
        modifier |= Modifier::BOLD;
    }
    if format.italic {
        modifier |= Modifier::ITALIC;
    }
    if format.underline {
        modifier |= Modifier::UNDERLINE;
    }
    if format.strikethrough {
        modifier |= Modifier::CROSSED_OUT;
    }
    if format.code {
        modifier |= Modifier::DIM;
    }
    modifier
}

impl RichTextEditor {
    /// Render editor
    pub(super) fn render_editor(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        // The heading, quote and link colors mark up the *content*, so a
        // `color` rule cannot have them. The body text is the base.
        let bg = self
            .bg
            .unwrap_or_else(|| ctx.css_background(super::EDITOR_BG));
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(super::EDITOR_FG));

        // Fill editor background
        fill(ctx, x, y, width, height, bg);

        // Render visible blocks
        for (row, block_idx) in (self.scroll.offset()..).take(height as usize).enumerate() {
            if block_idx >= self.blocks.len() {
                break;
            }
            let line = Rect::new(x, y + row as u16, width, 1);
            let col = self.draw_block_prefix(ctx, line, block_idx, (fg, bg));
            let col = self.draw_block_text(ctx, line, col, block_idx, (fg, bg));

            // Render cursor at end of line
            let text_len = self.blocks[block_idx].char_count();
            if self.focused
                && block_idx == self.cursor.0
                && self.cursor.1 >= text_len
                && col < x + width
            {
                ctx.set(col, line.y, Cell::new(' ').bg(self.cursor_bg));
            }
        }
    }

    /// Draw the block type's marker at the start of `line`, in the
    /// heading, quote or code color; returns the column after it.
    fn draw_block_prefix(
        &self,
        ctx: &mut RenderContext,
        line: Rect,
        block_idx: usize,
        (fg, bg): (Color, Color),
    ) -> u16 {
        let block = &self.blocks[block_idx];
        let prefix = block_prefix(block.block_type);

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

        let mut col = line.x;
        for ch in prefix.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if col + cw > line.x + line.width {
                break;
            }
            ctx.set(col, line.y, Cell::new(ch).fg(prefix_fg).bg(bg));
            col += cw;
        }
        col
    }

    /// Draw the block's text from `col` with each span's format, the cursor
    /// and selection backgrounds, and links in the link color; returns the
    /// column after it.
    fn draw_block_text(
        &self,
        ctx: &mut RenderContext,
        line: Rect,
        mut col: u16,
        block_idx: usize,
        (fg, bg): (Color, Color),
    ) -> u16 {
        let block = &self.blocks[block_idx];
        let selection = self.selection_range();

        // Links keep their markdown in the editor; they are told apart
        // by color. A code block is literal text and has none.
        let links = if block.block_type == BlockType::CodeBlock {
            Vec::new()
        } else {
            super::link::link_char_ranges(&block.text())
        };

        // Render block content with per-span formatting
        let mut char_idx = 0;
        for span in &block.spans {
            for ch in span.text.chars() {
                if col >= line.x + line.width {
                    break;
                }

                let is_cursor =
                    self.focused && block_idx == self.cursor.0 && char_idx == self.cursor.1;

                let is_selected = selection.is_some_and(|(start, end)| {
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

                // Build cell with span formatting
                let text_fg = if links.iter().any(|r| r.contains(&char_idx)) {
                    self.link_fg
                } else {
                    fg
                };
                let mut cell = Cell::new(ch).fg(text_fg).bg(cell_bg);
                cell.modifier |= format_modifier(&span.format);

                let cw = crate::utils::char_width(ch) as u16;
                ctx.set(col, line.y, cell);
                col += cw;
                char_idx += 1;
            }
        }
        col
    }
}
