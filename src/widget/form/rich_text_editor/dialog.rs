//! Dialog rendering for RichTextEditor

use super::{DialogType, RichTextEditor};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::traits::render_context::edit_line::{col_of, scroll_to_cursor};
use crate::widget::traits::RenderContext;

/// Draw a dialog field's text in `width` columns: a wide glyph takes two
/// cells, and a text longer than the field scrolls so its end - where
/// typing happens - stays in view.
fn put_field(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    width: u16,
    text: &str,
    fg: Color,
    bg: Color,
) {
    let text_w = col_of(text, usize::MAX);
    let scroll = scroll_to_cursor(0, text_w, 0, text_w, width as usize);
    ctx.put_edit_line(x, y, text, scroll, width, |_, ch| {
        Cell::new(ch).fg(fg).bg(bg)
    });
}

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
                ctx.set(dialog_x + col, dialog_y + row, Cell::new(' ').bg(bg));
            }
        }

        // Draw border
        ctx.set(dialog_x, dialog_y, Cell::new('┌').fg(fg).bg(bg));
        ctx.set(
            dialog_x + dialog_width - 1,
            dialog_y,
            Cell::new('┐').fg(fg).bg(bg),
        );
        ctx.set(
            dialog_x,
            dialog_y + dialog_height - 1,
            Cell::new('└').fg(fg).bg(bg),
        );
        ctx.set(
            dialog_x + dialog_width - 1,
            dialog_y + dialog_height - 1,
            Cell::new('┘').fg(fg).bg(bg),
        );
        for col in 1..dialog_width - 1 {
            ctx.set(dialog_x + col, dialog_y, Cell::new('─').fg(fg).bg(bg));
            ctx.set(
                dialog_x + col,
                dialog_y + dialog_height - 1,
                Cell::new('─').fg(fg).bg(bg),
            );
        }
        for row in 1..dialog_height - 1 {
            ctx.set(dialog_x, dialog_y + row, Cell::new('│').fg(fg).bg(bg));
            ctx.set(
                dialog_x + dialog_width - 1,
                dialog_y + row,
                Cell::new('│').fg(fg).bg(bg),
            );
        }

        // Typed text runs from column 8 to the right border's padding.
        let field_w = dialog_width.saturating_sub(10);

        match &self.dialog {
            DialogType::InsertLink { text, url, field } => {
                // Title
                let title = "Insert Link";
                let title_x =
                    dialog_x + (dialog_width.saturating_sub(display_width(title) as u16)) / 2;
                let title_w = dialog_x + dialog_width - 1 - title_x;
                ctx.put_edit_line(title_x, dialog_y + 1, title, 0, title_w, |_, ch| {
                    Cell::new(ch).fg(fg).bg(bg)
                });

                // Text field
                let label = "Text: ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                put_field(ctx, dialog_x + 8, dialog_y + 3, field_w, text, fg, input_bg);

                // URL field
                let label = "URL:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                put_field(ctx, dialog_x + 8, dialog_y + 4, field_w, url, fg, input_bg);
            }
            DialogType::InsertImage { alt, src, field } => {
                // Title
                let title = "Insert Image";
                let title_x =
                    dialog_x + (dialog_width.saturating_sub(display_width(title) as u16)) / 2;
                let title_w = dialog_x + dialog_width - 1 - title_x;
                ctx.put_edit_line(title_x, dialog_y + 1, title, 0, title_w, |_, ch| {
                    Cell::new(ch).fg(fg).bg(bg)
                });

                // Alt field
                let label = "Alt:  ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                put_field(ctx, dialog_x + 8, dialog_y + 3, field_w, alt, fg, input_bg);

                // Src field
                let label = "Src:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                put_field(ctx, dialog_x + 8, dialog_y + 4, field_w, src, fg, input_bg);
            }
            DialogType::None => {}
        }
    }
}
