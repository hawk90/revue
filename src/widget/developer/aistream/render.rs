//! Drawing the AI stream: visible text, cursor and thinking indicator

use super::{AiStream, StreamCursor, StreamStatus};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

impl AiStream {
    /// Get visible text
    fn visible_text(&self) -> String {
        self.content.chars().take(self.visible_chars).collect()
    }

    /// Render thinking indicator
    fn render_thinking(&self, ctx: &mut RenderContext) {
        let indicators = ['⠋', '⠙', '⠹', '⠸'];
        let ch = indicators[self.thinking_frame % indicators.len()];

        let mut cell = Cell::new(ch);
        cell.fg = Some(self.cursor_color);
        ctx.set(0, 0, cell);

        let text = " Thinking...";
        for (i, c) in text.chars().enumerate() {
            let mut cell = Cell::new(c);
            cell.fg = Some(LIGHT_GRAY);
            cell.modifier = Modifier::ITALIC;
            ctx.set(1 + i as u16, 0, cell);
        }
    }
}

impl View for AiStream {
    crate::impl_view_meta!("AiStream");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // The caret color marks where the stream is writing, which a `color`
        // rule cannot say separately from the text - so it stays put and the
        // text is what the stylesheet reaches.
        let text_fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let text_bg = self.bg.or_else(|| ctx.css_background_if_set());

        // Show thinking indicator if no content yet
        if self.content.is_empty() && self.show_thinking && self.status == StreamStatus::Streaming {
            self.render_thinking(ctx);
            return;
        }

        let visible = self.visible_text();
        let _width = area.width as usize;

        // Simple wrap and render, in terminal columns: a wide glyph takes two
        // and wraps whole, a zero-width char takes none
        let mut x = 0u16;
        let mut y = 0u16;
        let mut glyph = [0u8; 4];

        for ch in visible.chars() {
            if ch == '\n' {
                x = 0;
                y += 1;
                continue;
            }

            let w = crate::utils::char_width(ch) as u16;
            if w == 0 {
                continue;
            }

            if self.wrap && x > 0 && x.saturating_add(w) > area.width {
                x = 0;
                y += 1;
            }

            if y >= area.height {
                break;
            }

            ctx.put_str_with(x, y, ch.encode_utf8(&mut glyph), area.width, |ch| {
                let mut cell = Cell::new(ch).fg(text_fg);
                cell.bg = text_bg;
                cell
            });

            x = x.saturating_add(w);
        }

        // Render cursor
        if self.status == StreamStatus::Streaming
            && self.cursor != StreamCursor::None
            && y < area.height
        {
            let cursor_char = match self.cursor {
                StreamCursor::Block => '█',
                StreamCursor::Underline => '_',
                StreamCursor::Bar => '│',
                StreamCursor::None => ' ',
            };

            let mut cell = Cell::new(cursor_char);
            cell.fg = Some(self.cursor_color);
            // Blink effect
            if self.thinking_frame.is_multiple_of(2) {
                ctx.set(x, y, cell);
            }
        }
    }
}
