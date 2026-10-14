//! RichTextEditor's `View`: the toolbar row, then the editor, the preview or
//! both side by side, and an open dialog on top.

use super::{EditorViewMode, RichTextEditor};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

/// Fill `width` x `height` cells from (`x`, `y`) with blanks on `bg`
pub(super) fn fill(ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16, bg: Color) {
    for row in 0..height {
        for col in 0..width {
            ctx.set(x + col, y + row, Cell::new(' ').bg(bg));
        }
    }
}

impl View for RichTextEditor {
    crate::impl_view_meta!("RichTextEditor");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 1 {
            return;
        }

        // Fill background
        let bg = self
            .bg
            .unwrap_or_else(|| ctx.css_background(super::EDITOR_BG));
        ctx.fill_box_background(bg);

        let mut y: u16 = 0;

        // Render toolbar if enabled
        if self.show_toolbar {
            self.render_toolbar(ctx, 0, y, area.width);
            y += 1;
        }

        let content_height = area
            .height
            .saturating_sub(if self.show_toolbar { 1 } else { 0 });

        match self.view_mode {
            EditorViewMode::Editor => {
                self.render_editor(ctx, 0, y, area.width, content_height);
            }
            EditorViewMode::Preview => {
                self.render_preview(ctx, 0, y, area.width, content_height);
            }
            EditorViewMode::Split => {
                let half_width = area.width / 2;
                self.render_editor(ctx, 0, y, half_width, content_height);
                self.render_preview(ctx, half_width, y, area.width - half_width, content_height);
            }
        }

        // Render dialog if open
        if self.is_dialog_open() {
            self.render_dialog(ctx, 0, 0, area.width, area.height);
        }
    }
}
