//! Drawing the text: alignment within the row, delegated to RichText

use super::{Alignment, Text};
use crate::utils::unicode::display_width;
use crate::widget::traits::{RenderContext, View};

impl View for Text {
    /// One row per line (a `\n` starts one), as wide as the widest. Empty
    /// text is still a row - a spacer.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let (mut width, mut rows) = (0, 0);
        for line in self.content.split('\n') {
            width = width.max(display_width(line));
            rows += 1;
        }
        let clamp = |n: usize, max: u16| n.min(max as usize) as u16;
        Some((clamp(width, max_width), clamp(rows, max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // `visibility: hidden` still occupies its box - it just paints nothing.
        if !ctx.css_visible() {
            return;
        }

        let align = self.align_with_css(ctx);

        // Each line takes a row and is aligned on its own.
        for (y, line) in (0..area.height).zip(self.content.split('\n')) {
            if align == Alignment::Justify && self.render_justified_line(ctx, y, line) {
                continue;
            }

            let text_width = display_width(line).min(u16::MAX as usize) as u16;
            let x_offset = match align {
                Alignment::Left | Alignment::Justify => 0,
                Alignment::Center => area.width.saturating_sub(text_width) / 2,
                Alignment::Right => area.width.saturating_sub(text_width),
            };

            let rich_text = self.to_rich_text_with_ctx(ctx, line);
            let adjusted_area = ctx.sub_area(x_offset, y, area.width.saturating_sub(x_offset), 1);
            // `sub_ctx` rather than `RenderContext::new`, which would drop the
            // clip and let this text escape an enclosing `overflow: hidden`.
            let mut adjusted_ctx = ctx.sub_ctx(adjusted_area);

            // Delegate to RichText for actual rendering
            rich_text.render(&mut adjusted_ctx);
        }
    }

    crate::impl_view_meta!("Text");
}
