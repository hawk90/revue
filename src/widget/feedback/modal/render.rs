//! Drawing the modal: centered box, title, body or text content, and buttons

use super::{Modal, ModalButtonStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl View for Modal {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible {
            return;
        }

        let area = ctx.area;
        let modal_width = self.width.min(area.width.saturating_sub(4));
        let modal_height = self.required_height().min(area.height.saturating_sub(2));

        // Center the modal (relative coordinates)
        let x = (area.width.saturating_sub(modal_width)) / 2;
        let y = (area.height.saturating_sub(modal_height)) / 2;

        // Draw border
        self.render_border(ctx, x, y, modal_width, modal_height);

        // Draw title
        if !self.title.is_empty() && modal_width > 4 {
            let title_x = x + 2;
            let title_width = modal_width.saturating_sub(4);
            let title_fg = self.title_fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
            ctx.draw_text_clipped_bold(title_x, y + 1, &self.title, title_fg, title_width);

            // Title separator
            for dx in 1..modal_width.saturating_sub(1) {
                ctx.set(x + dx, y + 2, Cell::new('─'));
            }
            ctx.set(x, y + 2, Cell::new('├'));
            ctx.set(x + modal_width.saturating_sub(1), y + 2, Cell::new('┤'));
        }

        // Draw content — adjust for title presence
        let has_title = !self.title.is_empty() && modal_width > 4;
        let content_y = if has_title { y + 3 } else { y + 1 };
        let content_width = modal_width.saturating_sub(4);
        // borders(2) + buttons(1) + padding(1) + title+separator(2 if present)
        let content_height = if has_title {
            modal_height.saturating_sub(6)
        } else {
            modal_height.saturating_sub(4)
        };

        if let Some(ref body_widget) = self.body {
            // Render child widget
            let content_area = ctx.sub_area(x + 2, content_y, content_width, content_height);
            ctx.render_child(body_widget.as_ref(), content_area);
        } else {
            // Render text content
            for (i, line) in self.content.iter().enumerate() {
                let cy = content_y + i as u16;
                if cy >= y + modal_height - 2 {
                    break;
                }
                ctx.draw_text_clipped(x + 2, cy, line, Color::rgb(220, 220, 220), content_width);
            }
        }

        // Draw buttons
        if !self.buttons.is_empty() && modal_height > 2 {
            let button_y = y + modal_height.saturating_sub(2);
            let total_button_width: usize = self
                .buttons
                .iter()
                .map(|b| b.label.len() + 4) // [ label ]
                .sum::<usize>()
                + (self.buttons.len() - 1) * 2; // spacing

            // Skip drawing buttons if they don't fit
            if total_button_width as u16 > modal_width {
                return;
            }
            let start_x = x + (modal_width.saturating_sub(total_button_width as u16)) / 2;
            let mut bx = start_x;

            for (i, button) in self.buttons.iter().enumerate() {
                let is_selected = i == self.selected_button;
                let button_text = format!("[ {} ]", button.label);

                let (fg, bg) = if is_selected {
                    match button.style {
                        ModalButtonStyle::Primary => (Some(Color::WHITE), Some(Color::BLUE)),
                        ModalButtonStyle::Danger => (Some(Color::WHITE), Some(Color::RED)),
                        ModalButtonStyle::Default => (Some(Color::BLACK), Some(Color::WHITE)),
                    }
                } else {
                    (None, None)
                };

                let mut btn_x = bx;
                for ch in button_text.chars() {
                    let cw = crate::utils::char_width(ch) as u16;
                    let mut cell = Cell::new(ch);
                    cell.fg = fg;
                    cell.bg = bg;
                    if is_selected {
                        cell.modifier |= crate::render::Modifier::BOLD;
                    }
                    ctx.set(btn_x, button_y, cell);
                    btn_x += cw;
                }

                bx = btn_x + 2;
            }
        }
    }

    crate::impl_view_meta!("Modal");
}

impl Modal {
    fn render_border(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        if width < 2 || height < 2 {
            return;
        }

        // Clear interior with spaces
        for dy in 1..height.saturating_sub(1) {
            for dx in 1..width.saturating_sub(1) {
                ctx.set(x + dx, y + dy, Cell::new(' '));
            }
        }

        // Top border
        let mut corner = Cell::new('┌');
        corner.fg = self.border_fg;
        ctx.set(x, y, corner);

        for dx in 1..width.saturating_sub(1) {
            let mut cell = Cell::new('─');
            cell.fg = self.border_fg;
            ctx.set(x + dx, y, cell);
        }

        let mut corner = Cell::new('┐');
        corner.fg = self.border_fg;
        ctx.set(x + width.saturating_sub(1), y, corner);

        // Sides
        for dy in 1..height.saturating_sub(1) {
            let mut cell = Cell::new('│');
            cell.fg = self.border_fg;
            ctx.set(x, y + dy, cell);
            ctx.set(x + width.saturating_sub(1), y + dy, cell);
        }

        // Bottom border
        let mut corner = Cell::new('└');
        corner.fg = self.border_fg;
        ctx.set(x, y + height.saturating_sub(1), corner);

        for dx in 1..width.saturating_sub(1) {
            let mut cell = Cell::new('─');
            cell.fg = self.border_fg;
            ctx.set(x + dx, y + height.saturating_sub(1), cell);
        }

        let mut corner = Cell::new('┘');
        corner.fg = self.border_fg;
        ctx.set(
            x + width.saturating_sub(1),
            y + height.saturating_sub(1),
            corner,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_modal_render_hidden() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let m = Modal::new().title("Test");
        m.render(&mut ctx);

        // Hidden modal shouldn't render anything special
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_modal_render_visible() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("Test Dialog").content("Hello").ok();
        m.show();
        m.render(&mut ctx);

        // Modal should render centered - check for border characters
        // The exact position depends on centering calculation
        let center_x = (80 - 40) / 2;
        let center_y = (24 - m.required_height()) / 2;

        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_body_render() {
        use crate::widget::Text;

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new()
            .title("Body Test")
            .body(Text::new("Widget content"))
            .width(50)
            .height(12)
            .ok();
        m.show();
        m.render(&mut ctx);

        // Modal with body should render
        let center_x = (80 - 50) / 2;
        let center_y = (24 - 12) / 2;
        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_render_small_area_no_panic() {
        // Test that rendering in very small areas doesn't panic
        // This is the fix for issue #154

        // Width = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Width = 1
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 1, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Width = 2
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 2, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 1
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 2
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Both width and height = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_modal_render_width_2_border() {
        // Specific test for width=2 which was mentioned in the issue
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 4, 10); // Small width after subtracting 4 for margins
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("X").width(2).height(4);
        m.show();
        m.render(&mut ctx); // Should not panic
    }
}
