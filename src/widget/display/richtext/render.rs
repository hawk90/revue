//! Drawing the spans: styles, hyperlinks, wide characters and line breaks

use super::{RichText, Style};
use crate::render::{Cell, Modifier};
use crate::widget::traits::{RenderContext, View};

impl Style {
    /// Get modifier flags
    fn to_modifier(&self) -> Modifier {
        let mut m = Modifier::empty();
        if self.bold {
            m |= Modifier::BOLD;
        }
        if self.italic {
            m |= Modifier::ITALIC;
        }
        if self.underline {
            m |= Modifier::UNDERLINE;
        }
        if self.dim {
            m |= Modifier::DIM;
        }
        if self.strikethrough {
            m |= Modifier::CROSSED_OUT;
        }
        if self.reverse {
            m |= Modifier::REVERSE;
        }
        m
    }
}

impl View for RichText {
    crate::impl_view_meta!("RichText");

    /// One row per line (a `\n` in any span starts one), as wide as the
    /// widest line.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let lines = 1 + self
            .spans
            .iter()
            .map(|span| span.text.matches('\n').count())
            .sum::<usize>();
        let clamp = |v: usize, max: u16| v.min(max as usize) as u16;
        Some((clamp(self.width(), max_width), clamp(lines, max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // A span that names its own color keeps it - that is the whole point of
        // a rich text run. Spans that name none had no color at all until now,
        // which is exactly the base a `color` rule should reach.
        let base_fg = ctx.css_color_if_set();
        let base_bg = ctx.css_background_if_set();

        // A `\n` in any span starts a new row at the left edge of the area;
        // the text after it keeps its span's style. Rows past the area's
        // height are clipped, and a row wider than the area is truncated
        // without swallowing the rows after it.
        let mut x: u16 = 0;
        let mut y: u16 = 0;

        for span in &self.spans {
            // Register hyperlink if present
            let hyperlink_id = span
                .link
                .as_ref()
                .map(|url| ctx.buffer.register_hyperlink(url));

            let modifier = span.style.to_modifier();

            for ch in span.text.chars() {
                if ch == '\n' {
                    x = 0;
                    y += 1;
                    if y >= area.height {
                        return;
                    }
                    continue;
                }
                if x >= area.width {
                    continue;
                }

                let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;

                let mut cell = Cell::new(ch);
                cell.fg = span.style.fg.or(base_fg);
                cell.bg = span.style.bg.or(base_bg);
                cell.modifier = modifier;
                cell.hyperlink_id = hyperlink_id;

                ctx.set(x, y, cell);

                // Handle wide characters
                if char_width == 2 && x + 1 < area.width {
                    let mut cont = Cell::continuation();
                    cont.bg = span.style.bg.or(base_bg);
                    cont.hyperlink_id = hyperlink_id;
                    ctx.set(x + 1, y, cont);
                }

                x += char_width;
            }
        }
    }
}
