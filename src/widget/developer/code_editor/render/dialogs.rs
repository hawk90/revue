//! The go-to-line and find boxes on row 0

use crate::render::Cell;
use crate::style::Color;
use crate::utils::EditState;
use crate::widget::code_editor::CodeEditor;
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
use crate::widget::traits::RenderContext;

/// Draw a dialog's text field on row 0 in `width` columns, with its cursor
/// and selection, scrolled to the cursor
fn put_query(
    ctx: &mut RenderContext,
    x: u16,
    width: u16,
    state: &EditState,
    scroll: &std::cell::Cell<usize>,
) {
    let field = EditField::of(state, "", true);
    let at = field.scroll(scroll.get(), width);
    scroll.set(at);
    let bg = Color::rgb(49, 50, 68);
    let selected = crate::widget::traits::render_context::edit_line::selection_colors();
    ctx.put_edit_field(x, 0, width, &field, at, |kind, ch| {
        let mut cell = Cell::new(ch);
        (cell.fg, cell.bg) = match kind {
            FieldCell::Cursor => (Some(Color::BLACK), Some(Color::rgb(166, 227, 161))),
            FieldCell::Selected => (Some(selected.0), Some(selected.1)),
            FieldCell::Text | FieldCell::Placeholder => (Some(Color::rgb(166, 227, 161)), Some(bg)),
        };
        cell
    });
}

/// Draw a dialog box `dialog_width` columns wide, centered on row 0, with
/// `label` at its start; returns the box's x
fn draw_dialog_box(ctx: &mut RenderContext, dialog_width: u16, label: &str) -> u16 {
    let dialog_x = (ctx.area.width.saturating_sub(dialog_width)) / 2;

    // Background
    for x in 0..dialog_width {
        let mut cell = Cell::new(' ');
        cell.bg = Some(Color::rgb(49, 50, 68));
        ctx.set(dialog_x + x, 0, cell);
    }

    // Label
    for (i, ch) in label.chars().enumerate() {
        let mut cell = Cell::new(ch);
        cell.fg = Some(Color::WHITE);
        cell.bg = Some(Color::rgb(49, 50, 68));
        ctx.set(dialog_x + i as u16, 0, cell);
    }
    dialog_x
}

impl CodeEditor {
    /// Draw the go-to-line box with the typed line number
    pub(super) fn draw_goto_dialog(&self, ctx: &mut RenderContext) {
        let dialog_width = 20u16;
        let label = "Go to line: ";
        let dialog_x = draw_dialog_box(ctx, dialog_width, label);

        // Input
        let input_x = dialog_x + label.len() as u16;
        let input_w = dialog_width.saturating_sub(label.len() as u16);
        put_query(
            ctx,
            input_x,
            input_w,
            &self.goto_line_input,
            &self.goto_scroll,
        );
    }

    /// Draw the find box with the query and the match count
    pub(super) fn draw_find_dialog(&self, ctx: &mut RenderContext) {
        let dialog_width = 30u16;
        let label = "Find: ";
        let dialog_x = draw_dialog_box(ctx, dialog_width, label);

        // Query, leaving 8 columns for the match count
        let query_x = dialog_x + label.len() as u16;
        let query_w = dialog_width.saturating_sub(label.len() as u16 + 8);
        put_query(ctx, query_x, query_w, &self.find_query, &self.find_scroll);

        // Match count
        let count = format!(" {}/{}", self.current_find_index(), self.find_match_count());
        let count_x = dialog_x + dialog_width - count.len() as u16;
        for (i, ch) in count.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(180, 190, 254));
            cell.bg = Some(Color::rgb(49, 50, 68));
            ctx.set(count_x + i as u16, 0, cell);
        }
    }
}
