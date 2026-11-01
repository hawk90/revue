//! Drawing: the header line, then the content lines and their border.

use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::layout::constraints::constrain;
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::{RenderContext, View};

use super::Collapsible;

impl View for Collapsible {
    /// Its own bounds, which `render` puts back.
    fn held_size(&self) -> (u16, u16) {
        (self.min_width, self.min_height)
    }

    /// The header, and while expanded the content with its bottom border; the
    /// header and border stretch across the offered width.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let body = if self.expanded {
            let lines = u16::try_from(self.content.len()).unwrap_or(u16::MAX);
            lines.saturating_add(u16::from(self.show_border))
        } else {
            0
        };
        let area = crate::widget::layout::constraints::constrain(
            crate::layout::Rect::new(0, 0, max_width, body.saturating_add(1)),
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        );
        Some((area.width.min(max_width), area.height.min(max_height)))
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }
    crate::impl_view_meta!("Collapsible", focusable, disabled: state);

    fn render(&self, ctx: &mut RenderContext) {
        // The header takes `color`; the content and border keep their own.
        let header_fg = self
            .header_fg
            .unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let area = constrain(
            ctx.area,
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        );
        if area.width < 4 || area.height < 1 {
            return;
        }

        let is_focused = self.state.focused || ctx.is_focused();
        let header_fg = if self.state.disabled {
            DISABLED_FG
        } else {
            header_fg
        };

        self.render_header(ctx, area.width, header_fg, is_focused);

        // Render content if expanded
        if self.expanded && area.height > 1 {
            self.render_content(ctx, area);
        }
    }
}

impl Collapsible {
    /// The icon and the title on row 0, bold while focused.
    fn render_header(&self, ctx: &mut RenderContext, width: u16, header_fg: Color, bold: bool) {
        let x: u16 = 2; // icon + space

        // Background for header (if set)
        if let Some(bg) = self.header_bg {
            ctx.fill_bg(0, 0, width, 1, bg);
        }

        // Icon
        let mut icon_cell = Cell::new(self.icon());
        icon_cell.fg = Some(header_fg);
        if let Some(bg) = self.header_bg {
            icon_cell.bg = Some(bg);
        }
        if bold {
            icon_cell.modifier |= Modifier::BOLD;
        }
        ctx.set(0, 0, icon_cell);

        // Title
        let max_title_width = (width.saturating_sub(3)) as usize;
        let title_display = crate::utils::truncate_to_width(&self.title, max_title_width);
        let header_bg = self.header_bg;
        ctx.put_str_with(x, 0, title_display, width, |ch| {
            let mut cell = Cell::new(ch).fg(header_fg);
            cell.bg = header_bg;
            if bold {
                cell.modifier |= Modifier::BOLD;
            }
            cell
        });
    }

    /// The content lines from row 1, each with the left border, then the
    /// bottom border under the last one shown.
    fn render_content(&self, ctx: &mut RenderContext, area: Rect) {
        let content_start_y: u16 = 1;
        let available_height = area.height.saturating_sub(1);
        let content_width = area.width.saturating_sub(2);

        // Calculate how many lines we can show
        let lines_to_show = if self.show_border {
            available_height.saturating_sub(1) as usize
        } else {
            available_height as usize
        };

        // Draw content lines with left border
        for (i, line) in self.content.iter().take(lines_to_show).enumerate() {
            let y = content_start_y + i as u16;
            if y >= area.height {
                break;
            }
            self.render_content_line(ctx, y, line, area.width, content_width);
        }

        // Draw bottom border
        if self.show_border {
            let bottom_y = content_start_y + lines_to_show.min(self.content.len()) as u16;
            if bottom_y < area.height {
                // Corner, then the horizontal line
                ctx.draw_char(0, bottom_y, '└', self.border_color);
                let line_width = area.width.saturating_sub(1);
                ctx.draw_hline(
                    1,
                    bottom_y,
                    line_width.saturating_sub(1),
                    '─',
                    self.border_color,
                );
            }
        }
    }

    /// One content row: its background, the left border, and the text.
    fn render_content_line(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        line: &str,
        width: u16,
        content_width: u16,
    ) {
        // Background for content
        if let Some(bg) = self.content_bg {
            ctx.fill_bg(0, y, width, 1, bg);
        }

        // Left border
        if self.show_border {
            ctx.draw_char(0, y, '│', self.border_color);
        }

        // Content text
        let text_x: u16 = if self.show_border { 2 } else { 1 };
        let max_content_width = content_width.saturating_sub(1);
        let (content_fg, content_bg) = (self.content_fg, self.content_bg);
        ctx.put_str_with(text_x, y, line, text_x + max_content_width, |ch| {
            let mut cell = Cell::new(ch).fg(content_fg);
            cell.bg = content_bg;
            cell
        });
    }
}
