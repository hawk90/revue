//! Drawing the form: border, title, one block of rows per field and the status line

use super::{ErrorDisplayStyle, Form};
use crate::patterns::form::{FieldType, FormField};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, SECONDARY_TEXT, SUBTLE_GRAY};
use crate::widget::traits::{RenderContext, View};

impl Form {
    /// Render form border
    fn render_border(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 2 {
            return;
        }

        // A valid border takes `border-color` (falling back to `color`, as CSS
        // does); the invalid red is validity state and stays put - a rule has no
        // way to address it separately, so flattening it would hide the signal.
        let border_color = if self.is_valid() {
            ctx.css_border_or_text_color().unwrap_or(DISABLED_FG)
        } else {
            Color::rgb(200, 80, 80) // Red for invalid
        };

        ctx.draw_box_single(0, 0, area.width, area.height, border_color);
    }

    /// Render form title
    fn render_title(&self, ctx: &mut RenderContext) {
        // The title takes `color`; the error red below is validity state and a
        // rule cannot address it separately, so it stays.
        let title_fg = ctx.css_color(Color::WHITE);
        let area = ctx.area;
        if area.width < 4 {
            return;
        }

        let title = "Form";
        let title_x: u16 = 2;

        let max_x = area.width - 1;
        ctx.put_edit_line(title_x, 0, title, 0, max_x - title_x, |_, ch| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_fg);
            cell.bg = Some(Color::BLACK);
            cell
        });
    }

    /// Render the fields inside the border, from the top down
    ///
    /// Each field takes 3 rows: label (row 0), value (row 1), helper/error
    /// (row 2), plus 1 row gap between fields. A field cut by the bottom
    /// keeps the rows that fit.
    fn render_fields(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let content_x: u16 = 2;
        let content_width = area.width - 4;
        let max_x = content_x + content_width;
        let mut current_y: u16 = 1;
        let max_y = area.height - 2;
        let show_inline = self.error_style == ErrorDisplayStyle::Inline
            || self.error_style == ErrorDisplayStyle::Both;

        for (_name, field) in self.form_state.iter() {
            if current_y >= max_y {
                break;
            }

            // Row 0: Label
            render_field_label(ctx, field, content_x, current_y, max_x);
            current_y += 1;
            if current_y >= max_y {
                break;
            }

            // Row 1: Value or placeholder
            render_field_value(ctx, field, content_x, current_y, max_x);
            current_y += 1;
            if current_y >= max_y {
                break;
            }

            // Row 2: Error text (red, dim) if touched+errors, otherwise helper text (gray, dim)
            if show_inline && self.show_errors {
                render_field_message(ctx, field, content_x, current_y, max_x);
            }
            current_y += 1;

            // 1 row gap between fields
            current_y += 1;
        }
    }

    /// Render the status line above the bottom border: "Valid", or the
    /// error count when the error style shows a summary
    fn render_status(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Render validation summary at bottom if Summary or Both style
        let show_summary = self.error_style == ErrorDisplayStyle::Summary
            || self.error_style == ErrorDisplayStyle::Both;

        let status_y = area.height - 2;
        if status_y > 0 && area.width > 4 {
            if self.is_valid() {
                let status_text = "Valid";
                let status_color = Color::rgb(80, 200, 80);
                put_text(
                    ctx,
                    2,
                    status_y,
                    status_text,
                    area.width - 2,
                    status_color,
                    Modifier::empty(),
                );
            } else if show_summary {
                let error_count = self.error_count();
                let status_text = format!("{} error(s)", error_count);
                let status_color = Color::rgb(200, 80, 80);
                put_text(
                    ctx,
                    2,
                    status_y,
                    &status_text,
                    area.width - 2,
                    status_color,
                    Modifier::empty(),
                );
            }
        }
    }
}

/// Row 0 of a field: its label, when it has one
fn render_field_label(ctx: &mut RenderContext, field: &FormField, x: u16, y: u16, max_x: u16) {
    let label = &field.label;
    if !label.is_empty() {
        put_text(ctx, x, y, label, max_x, SECONDARY_TEXT, Modifier::empty());
    }
}

/// Row 1 of a field: its value (one bullet per char for a password), or
/// the placeholder in gray while it is empty
fn render_field_value(ctx: &mut RenderContext, field: &FormField, x: u16, y: u16, max_x: u16) {
    let value = field.value();
    let (display_text, text_color) = if value.is_empty() {
        (field.placeholder.clone(), SUBTLE_GRAY)
    } else if field.field_type == FieldType::Password {
        // One bullet per char, never the text itself.
        (
            "•".repeat(value.chars().count()),
            ctx.css_color(Color::WHITE),
        )
    } else {
        (value, ctx.css_color(Color::WHITE))
    };

    put_text(
        ctx,
        x,
        y,
        &display_text,
        max_x,
        text_color,
        Modifier::empty(),
    );
}

/// Row 2 of a field: its first error once it is touched, otherwise its
/// helper text
fn render_field_message(ctx: &mut RenderContext, field: &FormField, x: u16, y: u16, max_x: u16) {
    let show_error = field.is_touched() && field.has_errors();
    if show_error {
        if let Some(error_msg) = field.first_error() {
            let error_color = Color::rgb(200, 80, 80);
            put_text(ctx, x, y, &error_msg, max_x, error_color, Modifier::DIM);
        }
    } else {
        let helper = field.helper_text();
        if !helper.is_empty() {
            let helper_color = Color::rgb(140, 140, 140);
            put_text(ctx, x, y, helper, max_x, helper_color, Modifier::DIM);
        }
    }
}

/// Draw `text` at `(x, y)` up to column `max_x` (exclusive), in terminal
/// columns: a wide glyph (Hangul, CJK, emoji) takes two cells.
pub(super) fn put_text(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    text: &str,
    max_x: u16,
    fg: Color,
    modifier: Modifier,
) {
    ctx.put_edit_line(x, y, text, 0, max_x.saturating_sub(x), |_, ch| {
        let mut cell = Cell::new(ch);
        cell.fg = Some(fg);
        cell.modifier |= modifier;
        cell
    });
}

impl View for Form {
    crate::impl_view_meta!("Form");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Render border
        self.render_border(ctx);

        // Render title
        self.render_title(ctx);

        // Render form fields inside the border
        if area.width <= 4 || area.height <= 2 {
            return;
        }
        self.render_fields(ctx);
        self.render_status(ctx);
    }
}
