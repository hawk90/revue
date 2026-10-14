//! Drawing the text: alignment within the row, delegated to RichText

use super::{Alignment, Text};
use crate::widget::traits::{RenderContext, View};

impl View for Text {
    /// One row, as wide as the text. Empty text is still a row - a spacer.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let width = unicode_width::UnicodeWidthStr::width(self.content.as_str());
        Some((
            (width.min(u16::MAX as usize) as u16).min(max_width),
            1.min(max_height),
        ))
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

        // Handle Justify alignment specially
        if align == Alignment::Justify {
            self.render_justified(ctx);
            return;
        }

        // Extract CSS colors before creating adjusted context (avoids borrow conflict)
        let rich_text = self.to_rich_text_with_ctx(ctx);

        // Calculate start position based on alignment
        let text_width = unicode_width::UnicodeWidthStr::width(self.content.as_str()) as u16;
        let x_offset = match align {
            Alignment::Left | Alignment::Justify => 0,
            Alignment::Center => area.width.saturating_sub(text_width) / 2,
            Alignment::Right => area.width.saturating_sub(text_width),
        };

        // Create adjusted context with alignment offset
        let adjusted_area = ctx.sub_area(
            x_offset,
            0,
            area.width.saturating_sub(x_offset),
            area.height,
        );
        // `sub_ctx` rather than `RenderContext::new`, which would drop the clip
        // and let this text escape an enclosing `overflow: hidden`.
        let mut adjusted_ctx = ctx.sub_ctx(adjusted_area);

        // Delegate to RichText for actual rendering
        rich_text.render(&mut adjusted_ctx);
    }

    crate::impl_view_meta!("Text");
}
