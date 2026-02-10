//! Dialog rendering for RichTextEditor

use super::{DialogType, RichTextEditor};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl RichTextEditor {
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
