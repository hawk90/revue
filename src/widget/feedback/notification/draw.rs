//! Drawing one notification: its frame and rows

use super::{Notification, NotificationCenter};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::char_width;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::RenderContext;

/// Where one notification's box sits and the colors it is drawn in
#[derive(Clone, Copy)]
struct Frame {
    /// Left column of the box
    x: u16,
    /// Width of the box, borders included
    width: u16,
    /// The level color (after the stylesheet)
    color: Color,
    /// The level background
    bg: Color,
    /// White when selected, the level color otherwise
    border: Color,
}

impl Frame {
    /// A border glyph at (`x`, `y`)
    fn border_cell(&self, ctx: &mut RenderContext, x: u16, y: u16, ch: char) {
        let mut cell = Cell::new(ch);
        cell.fg = Some(self.border);
        ctx.set(x, y, cell);
    }

    /// The top border
    fn top(&self, ctx: &mut RenderContext, y: u16) {
        let (x, width) = (self.x, self.width);
        self.border_cell(ctx, x, y, '╭');

        for dx in 1..width - 1 {
            self.border_cell(ctx, x + dx, y, '─');
        }

        self.border_cell(ctx, x + width - 1, y, '╮');
    }

    /// The left border of a content row and its background
    fn open_row(&self, ctx: &mut RenderContext, y: u16) {
        let (x, width) = (self.x, self.width);
        self.border_cell(ctx, x, y, '│');

        // Fill background
        for dx in 1..width - 1 {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.bg);
            ctx.set(x + dx, y, cell);
        }
    }

    /// The right border of a content row
    fn close_row(&self, ctx: &mut RenderContext, y: u16) {
        self.border_cell(ctx, self.x + self.width - 1, y, '│');
    }

    /// The level icon at (`x`, `y`)
    fn icon(&self, ctx: &mut RenderContext, x: u16, y: u16, icon: char) {
        let mut cell = Cell::new(icon);
        cell.fg = Some(self.color);
        cell.bg = Some(self.bg);
        ctx.set(x, y, cell);
    }

    /// White text from `content_x`, stopping before the last inner column
    fn text(
        &self,
        ctx: &mut RenderContext,
        content_x: u16,
        y: u16,
        text: &str,
        modifier: Modifier,
    ) {
        let (x, width) = (self.x, self.width);
        let mut dx: u16 = 0;
        for ch in text.chars() {
            let cw = char_width(ch) as u16;
            if content_x + dx >= x + width - 2 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(self.bg);
            cell.modifier |= modifier;
            ctx.set(content_x + dx, y, cell);
            dx += cw;
        }
    }
}

impl NotificationCenter {
    pub(super) fn render_notification(
        &self,
        ctx: &mut RenderContext,
        notification: &Notification,
        x: u16,
        y: u16,
        is_selected: bool,
    ) {
        // The level palette is this widget's own default and a rule naming the
        // center targets the whole node, so the author outranks it.
        let color = ctx.css_color(notification.level.color());
        let frame = Frame {
            x,
            width: self.width,
            color,
            bg: notification.level.bg_color(),
            border: if is_selected { Color::WHITE } else { color },
        };

        frame.top(ctx, y);
        let mut current_y = y + 1;

        // Title line (if present)
        if let Some(ref title) = notification.title {
            self.draw_title(ctx, &frame, current_y, notification, title);
            current_y += 1;
        }

        self.draw_message(ctx, &frame, current_y, notification);
        current_y += 1;

        // Progress line (if present)
        if let Some(progress) = notification.progress {
            draw_progress(ctx, &frame, current_y, progress);
            current_y += 1;
        }

        // Action line (if present): the label, bracketed and right-aligned
        if let Some(ref action) = notification.action {
            draw_action(ctx, &frame, current_y, action);
            current_y += 1;
        }

        self.draw_bottom(ctx, &frame, current_y, notification);
    }

    /// The title row, with the icon when icons are shown
    fn draw_title(
        &self,
        ctx: &mut RenderContext,
        frame: &Frame,
        y: u16,
        notification: &Notification,
        title: &str,
    ) {
        frame.open_row(ctx, y);

        // Icon
        let mut content_x = frame.x + 1;
        if self.show_icons {
            frame.icon(ctx, content_x, y, notification.level.icon());
            content_x += 2;
        }

        // Title text
        frame.text(ctx, content_x, y, title, Modifier::BOLD);

        frame.close_row(ctx, y);
    }

    /// The message row, with the icon when icons are shown and there is no
    /// title to carry it
    fn draw_message(
        &self,
        ctx: &mut RenderContext,
        frame: &Frame,
        y: u16,
        notification: &Notification,
    ) {
        frame.open_row(ctx, y);

        // Icon (if no title)
        let mut content_x = frame.x + 1;
        if self.show_icons && notification.title.is_none() {
            frame.icon(ctx, content_x, y, notification.level.icon());
            content_x += 2;
        }

        // Message text
        frame.text(ctx, content_x, y, &notification.message, Modifier::empty());

        frame.close_row(ctx, y);
    }

    /// The bottom border, drawn as the draining timer when the timer is
    /// shown and the notification expires
    fn draw_bottom(
        &self,
        ctx: &mut RenderContext,
        frame: &Frame,
        y: u16,
        notification: &Notification,
    ) {
        let (x, width) = (frame.x, frame.width);
        frame.border_cell(ctx, x, y, '╰');

        // Timer indicator
        if self.show_timer && notification.duration > 0 {
            let remaining = notification.remaining();
            let timer_width = (width - 4) as f64;
            let timer_filled = (remaining * timer_width).round() as u16;

            for dx in 1..width - 1 {
                let ch = if dx <= timer_filled { '━' } else { '─' };
                let fg = if dx <= timer_filled {
                    frame.color
                } else {
                    frame.border
                };
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                ctx.set(x + dx, y, cell);
            }
        } else {
            for dx in 1..width - 1 {
                frame.border_cell(ctx, x + dx, y, '─');
            }
        }

        frame.border_cell(ctx, x + width - 1, y, '╯');
    }
}

/// The progress row: a bar filled in the level color
fn draw_progress(ctx: &mut RenderContext, frame: &Frame, y: u16, progress: f64) {
    let (x, width) = (frame.x, frame.width);
    frame.open_row(ctx, y);

    // Progress bar
    let bar_width = width - 4;
    let filled = (progress * bar_width as f64).round() as u16;
    for dx in 0..bar_width {
        let ch = if dx < filled { '█' } else { '░' };
        let fg = if dx < filled {
            frame.color
        } else {
            SEPARATOR_COLOR
        };
        let mut cell = Cell::new(ch);
        cell.fg = Some(fg);
        cell.bg = Some(frame.bg);
        ctx.set(x + 2 + dx, y, cell);
    }

    frame.close_row(ctx, y);
}

/// The action row: the label, bracketed, bold and right-aligned
fn draw_action(ctx: &mut RenderContext, frame: &Frame, y: u16, action: &str) {
    let (x, width) = (frame.x, frame.width);
    frame.open_row(ctx, y);

    // Inner columns, keeping one blank on each side of the label
    let inner = width.saturating_sub(4);
    let label = format!("[{}]", action);
    let label_width = (crate::utils::display_width(&label) as u16).min(inner);
    let mut dx: u16 = 0;
    let start = x + 2 + inner - label_width;
    for ch in label.chars() {
        let cw = char_width(ch) as u16;
        if dx + cw > label_width {
            break;
        }
        let mut cell = Cell::new(ch);
        cell.fg = Some(frame.color);
        cell.bg = Some(frame.bg);
        cell.modifier |= Modifier::BOLD;
        ctx.set(start + dx, y, cell);
        dx += cw;
    }

    frame.close_row(ctx, y);
}
