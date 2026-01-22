//! Toolbar rendering for RichTextEditor

use super::super::traits::RenderContext;
use super::RichTextEditor;
use crate::render::Cell;
use crate::style::Color;

impl RichTextEditor {
    /// Render toolbar
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
}
