//! Dialog rendering for RichTextEditor

use super::view::fill;
use super::{DialogType, RichTextEditor};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::traits::render_context::edit_line::{col_of, scroll_to_cursor};
use crate::widget::traits::RenderContext;

/// The dialog's own fill and text color
const DIALOG_BG: Color = Color::rgb(49, 50, 68);
const DIALOG_FG: Color = Color::rgb(205, 214, 244);

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

/// The dialog's fill and its single-line border
fn draw_dialog_frame(ctx: &mut RenderContext, frame: Rect) {
    let (fg, bg) = (DIALOG_FG, DIALOG_BG);
    let Rect {
        x: dialog_x,
        y: dialog_y,
        width: dialog_width,
        height: dialog_height,
    } = frame;

    // Draw dialog background
    fill(ctx, dialog_x, dialog_y, dialog_width, dialog_height, bg);

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
}

impl RichTextEditor {
    /// Render dialog
    pub(super) fn render_dialog(
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
        // No room for even the two border columns
        if dialog_width < 2 {
            return;
        }
        let dialog_x = x + (width.saturating_sub(dialog_width)) / 2;
        let dialog_y = y + (height.saturating_sub(dialog_height)) / 2;
        let frame = Rect::new(dialog_x, dialog_y, dialog_width, dialog_height);

        draw_dialog_frame(ctx, frame);

        match &self.dialog {
            DialogType::InsertLink { text, url, field } => {
                self.draw_dialog_body(
                    ctx,
                    frame,
                    "Insert Link",
                    [("Text: ", text.as_str()), ("URL:  ", url.as_str())],
                    *field,
                );
            }
            DialogType::InsertImage { alt, src, field } => {
                self.draw_dialog_body(
                    ctx,
                    frame,
                    "Insert Image",
                    [("Alt:  ", alt.as_str()), ("Src:  ", src.as_str())],
                    *field,
                );
            }
            DialogType::None => {}
        }
    }

    /// The title, centered on the first row inside the border, and the two
    /// labeled fields on the third and fourth; the `field` being typed into
    /// has the selection background.
    fn draw_dialog_body(
        &self,
        ctx: &mut RenderContext,
        frame: Rect,
        title: &str,
        fields: [(&str, &str); 2],
        field: usize,
    ) {
        let (fg, bg) = (DIALOG_FG, DIALOG_BG);
        let Rect {
            x: dialog_x,
            y: dialog_y,
            width: dialog_width,
            ..
        } = frame;

        // Title
        let title_x = dialog_x + (dialog_width.saturating_sub(display_width(title) as u16)) / 2;
        let title_w = dialog_x + dialog_width - 1 - title_x;
        ctx.put_edit_line(title_x, dialog_y + 1, title, 0, title_w, |_, ch| {
            Cell::new(ch).fg(fg).bg(bg)
        });

        // Typed text runs from column 8 to the right border's padding.
        let field_w = dialog_width.saturating_sub(10);

        for (row, (label, value)) in fields.into_iter().enumerate() {
            let row_y = dialog_y + 3 + row as u16;
            let input_bg = if field == row { self.selection_bg } else { bg };
            for (i, ch) in label.chars().enumerate() {
                ctx.set(dialog_x + 2 + i as u16, row_y, Cell::new(ch).fg(fg).bg(bg));
            }
            put_field(ctx, dialog_x + 8, row_y, field_w, value, fg, input_bg);
        }
    }
}
