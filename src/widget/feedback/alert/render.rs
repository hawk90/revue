//! Drawing the alert: filled, outlined and minimal variants

use super::{Alert, AlertVariant};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::char_width;
use crate::widget::layout::border::{draw_border, BorderType};
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY, MUTED_TEXT, SECONDARY_TEXT};
use crate::widget::traits::{RenderContext, View};

impl View for Alert {
    crate::impl_view_meta!("Alert");

    /// [`height`](Alert::height) rows, as wide as offered (the box and its
    /// background run the full width). A dismissed alert takes no room.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        if self.dismissed {
            return Some((0, 0));
        }
        Some((max_width, self.height().min(max_height)))
    }

    /// It stretches across the width it is offered - unless dismissed.
    fn fills(&self) -> crate::widget::Fill {
        if self.dismissed {
            crate::widget::Fill::NONE
        } else {
            crate::widget::Fill::WIDTH
        }
    }

    fn render(&self, ctx: &mut RenderContext) {
        if self.dismissed {
            return;
        }

        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        // The level's palette is the widget's own default - the bottom row of
        // the precedence table - so a rule that named a color outranks it.
        let accent_color = ctx.css_color(self.level.color());
        let bg_color = ctx.css_background(self.level.bg_color());
        let border_color = ctx
            .css_border_or_text_color()
            .unwrap_or(self.level.border_color());

        match self.variant {
            AlertVariant::Filled => {
                self.render_filled(ctx, accent_color, bg_color, border_color);
            }
            AlertVariant::Outlined => {
                self.render_outlined(ctx, accent_color, border_color);
            }
            AlertVariant::Minimal => {
                self.render_minimal(ctx, accent_color);
            }
        }
    }
}

impl Alert {
    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.level.icon())
    }

    fn render_filled(
        &self,
        ctx: &mut RenderContext,
        accent_color: Color,
        bg_color: Color,
        border_color: Color,
    ) {
        let area = ctx.area;

        // Fill background
        for y in 0..area.height {
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg_color);
                ctx.set(x, y, cell);
            }
        }

        // Draw border
        self.draw_alert_border(ctx, border_color, bg_color);

        // Content area
        let content_x: u16 = 2;
        let content_width = area.width.saturating_sub(4);
        let mut y: u16 = 1;

        // Icon and title/message
        let icon_offset = if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            icon_cell.bg = Some(bg_color);
            ctx.set(content_x, y, icon_cell);
            2
        } else {
            0
        };

        // Title (if present)
        if let Some(ref title) = self.title {
            let text_x = content_x + icon_offset;
            let max_w = content_width.saturating_sub(icon_offset);
            ctx.draw_text_clipped_bg_bold(text_x, y, title, Color::WHITE, bg_color, max_w);
            y += 1;
            ctx.draw_text_clipped_bg(text_x, y, &self.message, SECONDARY_TEXT, bg_color, max_w);
        } else {
            let text_x = content_x + icon_offset;
            let max_w = content_width.saturating_sub(icon_offset);
            ctx.draw_text_clipped_bg(text_x, y, &self.message, Color::WHITE, bg_color, max_w);
        }

        // Dismiss button
        if self.dismissible {
            let dismiss_x = area.width - 3;
            let mut x_cell = Cell::new('×');
            x_cell.fg = Some(LIGHT_GRAY);
            x_cell.bg = Some(bg_color);
            ctx.set(dismiss_x, 1, x_cell);
        }
    }

    fn render_outlined(&self, ctx: &mut RenderContext, accent_color: Color, _border_color: Color) {
        let text_fg = self.state.resolve_fg(ctx.style, Color::WHITE);
        let area = ctx.area;

        // Draw left accent border
        for y in 0..area.height {
            let mut cell = Cell::new('┃');
            cell.fg = Some(accent_color);
            ctx.set(0, y, cell);
        }

        // Content
        let content_x: u16 = 2;
        let content_width = area.width.saturating_sub(3);
        let mut y: u16 = 0;

        // Icon
        let icon_offset = if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.set(content_x, y, icon_cell);
            2
        } else {
            0
        };

        // Title
        if let Some(ref title) = self.title {
            let title_x = content_x + icon_offset;
            let max_w = content_width - icon_offset;
            let mut dx: u16 = 0;
            for ch in title.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > max_w {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(text_fg);
                cell.modifier |= Modifier::BOLD;
                ctx.set(title_x + dx, y, cell);
                dx += cw;
            }
            y += 1;

            // Message
            let msg_x = content_x + icon_offset;
            let mut dx: u16 = 0;
            for ch in self.message.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > max_w {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(MUTED_TEXT);
                ctx.set(msg_x + dx, y, cell);
                dx += cw;
            }
        } else {
            let msg_x = content_x + icon_offset;
            let max_w = content_width - icon_offset;
            let mut dx: u16 = 0;
            for ch in self.message.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > max_w {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(text_fg);
                ctx.set(msg_x + dx, y, cell);
                dx += cw;
            }
        }

        // Dismiss button
        if self.dismissible {
            let dismiss_x = area.width - 2;
            let mut x_cell = Cell::new('×');
            x_cell.fg = Some(LIGHT_GRAY);
            ctx.set(dismiss_x, 0, x_cell);
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext, accent_color: Color) {
        let text_fg = self.state.resolve_fg(ctx.style, Color::WHITE);
        let area = ctx.area;
        let mut x: u16 = 0;
        let y: u16 = 0;

        // Icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.set(x, y, icon_cell);
            x += 2;
        }

        // Title or message
        if let Some(ref title) = self.title {
            // Title on first line
            let mut dx: u16 = 0;
            for ch in title.chars() {
                let cw = char_width(ch) as u16;
                if x + dx + cw > area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(accent_color);
                cell.modifier |= Modifier::BOLD;
                ctx.set(x + dx, y, cell);
                dx += cw;
            }

            // Message on second line
            if area.height > 1 {
                let msg_x: u16 = if self.show_icon { 2 } else { 0 };
                let mut dx: u16 = 0;
                for ch in self.message.chars() {
                    let cw = char_width(ch) as u16;
                    if msg_x + dx + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(MUTED_TEXT);
                    ctx.set(msg_x + dx, y + 1, cell);
                    dx += cw;
                }
            }
        } else {
            // Just message
            let mut dx: u16 = 0;
            for ch in self.message.chars() {
                let cw = char_width(ch) as u16;
                if x + dx + cw > area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(text_fg);
                ctx.set(x + dx, y, cell);
                dx += cw;
            }
        }

        // Dismiss button
        if self.dismissible {
            let dismiss_x = area.width - 1;
            let mut x_cell = Cell::new('×');
            x_cell.fg = Some(DISABLED_FG);
            ctx.set(dismiss_x, y, x_cell);
        }
    }

    fn draw_alert_border(&self, ctx: &mut RenderContext, border_color: Color, bg_color: Color) {
        // Use centralized border drawing utility
        draw_border(
            ctx.buffer,
            ctx.area,
            BorderType::Rounded,
            Some(border_color),
            Some(bg_color),
        );
    }
}
