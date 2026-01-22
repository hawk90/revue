//! Rendering functions for RichTextEditor

use super::super::traits::RenderContext;
use super::{BlockType, DialogType, RichTextEditor};
use crate::render::{Cell, Modifier};
use crate::style::Color;

impl RichTextEditor {
    pub(crate) fn render_toolbar(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        // Fill toolbar background
        for col in 0..width {
            ctx.buffer
                .set(x + col, y, Cell::new(' ').bg(self.toolbar_bg));
        }

        let toolbar_items = [
            ("B", self.current_format.bold),
            ("I", self.current_format.italic),
            ("U", self.current_format.underline),
            ("S", self.current_format.strikethrough),
            ("`", self.current_format.code),
            ("|", false),
            ("H1", false),
            ("H2", false),
            ("H3", false),
            ("|", false),
            ("\"", false),
            ("•", false),
            ("1.", false),
            ("|", false),
            ("🔗", false),
            ("📷", false),
        ];

        let mut col = x + 1;
        for (label, active) in toolbar_items {
            if label == "|" {
                ctx.buffer.set(
                    col,
                    y,
                    Cell::new('│').fg(self.toolbar_fg).bg(self.toolbar_bg),
                );
                col += 1;
            } else {
                let (bg, fg) = if active {
                    (self.toolbar_active_bg, Color::rgb(30, 30, 46))
                } else {
                    (self.toolbar_bg, self.toolbar_fg)
                };

                for ch in label.chars() {
                    if col < x + width {
                        ctx.buffer.set(col, y, Cell::new(ch).fg(fg).bg(bg));
                        col += 1;
                    }
                }
                col += 1; // Space between items
            }
        }
    }

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

    /// Render dialog
    pub(crate) fn render_dialog(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        // Calculate dialog position (centered)
        let dialog_width = 40.min(width.saturating_sub(4));
        let dialog_height = 7;
        let dialog_x = x + (width.saturating_sub(dialog_width)) / 2;
        let dialog_y = y + (height.saturating_sub(dialog_height)) / 2;

        let bg = Color::rgb(49, 50, 68);
        let fg = Color::rgb(205, 214, 244);

        // Draw dialog background
        for row in 0..dialog_height {
            for col in 0..dialog_width {
                ctx.buffer
                    .set(dialog_x + col, dialog_y + row, Cell::new(' ').bg(bg));
            }
        }

        // Draw border
        ctx.buffer
            .set(dialog_x, dialog_y, Cell::new('┌').fg(fg).bg(bg));
        ctx.buffer.set(
            dialog_x + dialog_width - 1,
            dialog_y,
            Cell::new('┐').fg(fg).bg(bg),
        );
        ctx.buffer.set(
            dialog_x,
            dialog_y + dialog_height - 1,
            Cell::new('└').fg(fg).bg(bg),
        );
        ctx.buffer.set(
            dialog_x + dialog_width - 1,
            dialog_y + dialog_height - 1,
            Cell::new('┘').fg(fg).bg(bg),
        );
        for col in 1..dialog_width - 1 {
            ctx.buffer
                .set(dialog_x + col, dialog_y, Cell::new('─').fg(fg).bg(bg));
            ctx.buffer.set(
                dialog_x + col,
                dialog_y + dialog_height - 1,
                Cell::new('─').fg(fg).bg(bg),
            );
        }
        for row in 1..dialog_height - 1 {
            ctx.buffer
                .set(dialog_x, dialog_y + row, Cell::new('│').fg(fg).bg(bg));
            ctx.buffer.set(
                dialog_x + dialog_width - 1,
                dialog_y + row,
                Cell::new('│').fg(fg).bg(bg),
            );
        }

        match &self.dialog {
            DialogType::InsertLink { text, url, field } => {
                // Title
                let title = "Insert Link";
                let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;
                for (i, ch) in title.chars().enumerate() {
                    ctx.buffer.set(
                        title_x + i as u16,
                        dialog_y + 1,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }

                // Text field
                let label = "Text: ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in text.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 3,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }

                // URL field
                let label = "URL:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in url.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 4,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }
            }
            DialogType::InsertImage { alt, src, field } => {
                // Title
                let title = "Insert Image";
                let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;
                for (i, ch) in title.chars().enumerate() {
                    ctx.buffer.set(
                        title_x + i as u16,
                        dialog_y + 1,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }

                // Alt field
                let label = "Alt:  ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in alt.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 3,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }

                // Src field
                let label = "Src:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in src.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 4,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }
            }
            DialogType::None => {}
        }
    }
}
