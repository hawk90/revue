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

/// The dismiss button's glyph
const DISMISS: char = '×';

impl Alert {
    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.level.icon())
    }

    /// Columns the dismiss button takes from the text on its row: the button
    /// and one blank column before it (none when not dismissible).
    fn dismiss_reserve(&self) -> u16 {
        if self.dismissible {
            char_width(DISMISS) as u16 + 1
        } else {
            0
        }
    }

    /// The level's icon (or the custom one) at `(x, y)`
    fn draw_icon(&self, ctx: &mut RenderContext, x: u16, y: u16, fg: Color, bg: Option<Color>) {
        let icon = self.get_icon();
        let mut icon_cell = Cell::new(icon);
        icon_cell.fg = Some(fg);
        icon_cell.bg = bg;
        ctx.set(x, y, icon_cell);
    }

    /// The dismiss button at `(x, y)`
    fn draw_dismiss(ctx: &mut RenderContext, x: u16, y: u16, fg: Color, bg: Option<Color>) {
        let mut x_cell = Cell::new(DISMISS);
        x_cell.fg = Some(fg);
        x_cell.bg = bg;
        ctx.set(x, y, x_cell);
    }

    /// `text` from `x`, one cell per character, stopping before the
    /// character that would pass `max_w` columns
    fn draw_run(
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        text: &str,
        max_w: u16,
        fg: Color,
        bold: bool,
    ) {
        let mut dx: u16 = 0;
        for ch in text.chars() {
            let cw = char_width(ch) as u16;
            if dx + cw > max_w {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            if bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(x + dx, y, cell);
            dx += cw;
        }
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
            self.draw_icon(ctx, content_x, y, accent_color, Some(bg_color));
            2
        } else {
            0
        };

        // Title (if present)
        // The dismiss button shares the first row with the title (or the
        // message when there is no title).
        let text_x = content_x + icon_offset;
        let max_w = content_width.saturating_sub(icon_offset);
        let first_row_w = max_w.saturating_sub(self.dismiss_reserve());
        if let Some(ref title) = self.title {
            ctx.draw_text_clipped_bg_bold(text_x, y, title, Color::WHITE, bg_color, first_row_w);
            y += 1;
            ctx.draw_text_clipped_bg(text_x, y, &self.message, SECONDARY_TEXT, bg_color, max_w);
        } else {
            ctx.draw_text_clipped_bg(
                text_x,
                y,
                &self.message,
                Color::WHITE,
                bg_color,
                first_row_w,
            );
        }

        // Dismiss button
        if self.dismissible {
            Self::draw_dismiss(ctx, area.width - 3, 1, LIGHT_GRAY, Some(bg_color));
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
            self.draw_icon(ctx, content_x, y, accent_color, None);
            2
        } else {
            0
        };

        // Title
        if let Some(ref title) = self.title {
            let title_x = content_x + icon_offset;
            let max_w = content_width - icon_offset;
            let title_w = max_w.saturating_sub(self.dismiss_reserve());
            Self::draw_run(ctx, title_x, y, title, title_w, text_fg, true);
            y += 1;

            // Message
            let msg_x = content_x + icon_offset;
            Self::draw_run(ctx, msg_x, y, &self.message, max_w, MUTED_TEXT, false);
        } else {
            let msg_x = content_x + icon_offset;
            let max_w = (content_width - icon_offset).saturating_sub(self.dismiss_reserve());
            Self::draw_run(ctx, msg_x, y, &self.message, max_w, text_fg, false);
        }

        // Dismiss button
        if self.dismissible {
            Self::draw_dismiss(ctx, area.width - 2, 0, LIGHT_GRAY, None);
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext, accent_color: Color) {
        let text_fg = self.state.resolve_fg(ctx.style, Color::WHITE);
        let area = ctx.area;
        let mut x: u16 = 0;
        let y: u16 = 0;
        // The first row ends before the dismiss button.
        let first_row_end = area.width.saturating_sub(self.dismiss_reserve());

        // Icon
        if self.show_icon {
            self.draw_icon(ctx, x, y, accent_color, None);
            x += 2;
        }

        // Title or message
        if let Some(ref title) = self.title {
            // Title on first line
            let title_w = first_row_end.saturating_sub(x);
            Self::draw_run(ctx, x, y, title, title_w, accent_color, true);

            // Message on second line
            if area.height > 1 {
                let msg_x: u16 = if self.show_icon { 2 } else { 0 };
                let msg_w = area.width.saturating_sub(msg_x);
                Self::draw_run(ctx, msg_x, y + 1, &self.message, msg_w, MUTED_TEXT, false);
            }
        } else {
            // Just message
            let msg_w = first_row_end.saturating_sub(x);
            Self::draw_run(ctx, x, y, &self.message, msg_w, text_fg, false);
        }

        // Dismiss button
        if self.dismissible {
            Self::draw_dismiss(ctx, area.width - 1, y, DISABLED_FG, None);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    fn row(alert: &Alert, y: u16) -> String {
        let (w, h) = (20, 4);
        let mut buf = Buffer::new(w, h);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, w, h));
        alert.render(&mut ctx);
        (0..w).map(|x| buf.get(x, y).unwrap().symbol).collect()
    }

    #[test]
    fn dismiss_button_keeps_its_own_columns() {
        let long = "abcdefghijklmnopqrstuvwxyz";
        let alert = |v| Alert::info(long).icon(false).dismissible(true).variant(v);
        // Text stops one blank column before the `×`.
        assert_eq!(row(&alert(AlertVariant::Filled), 1), "│ abcdefghijklmn × │");
        assert_eq!(
            row(&alert(AlertVariant::Outlined), 0),
            "┃ abcdefghijklmno × "
        );
        assert_eq!(
            row(&alert(AlertVariant::Minimal), 0),
            "abcdefghijklmnopqr ×"
        );
        // A title on the button's row is cut the same way; the row below is not.
        let titled = Alert::info(long)
            .title(long)
            .icon(false)
            .dismissible(true)
            .variant(AlertVariant::Outlined);
        assert_eq!(row(&titled, 0), "┃ abcdefghijklmno × ");
        assert_eq!(row(&titled, 1), "┃ abcdefghijklmnopq ");
    }
}
