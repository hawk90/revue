//! Drawing the card (frame, header, body, footer, shadow) and measuring it

use super::super::border::BorderType;
use super::types::CardVariant;
use super::Card;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::utils::unicode::char_width;
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

impl Card {
    /// Get the collapse indicator character
    fn collapse_icon(&self) -> char {
        if self.expanded {
            '▼'
        } else {
            '▶'
        }
    }

    /// Calculate footer height
    fn footer_height(&self) -> u16 {
        if self.footer.is_some() {
            2 // separator + footer content
        } else {
            0
        }
    }

    /// Whether the parts below the title and subtitle show: always, unless
    /// the card is collapsible and collapsed
    fn shown(&self) -> bool {
        self.expanded || !self.collapsible
    }

    /// Whether anything sits above the body: a title, subtitle or header
    fn has_header(&self) -> bool {
        self.title.is_some() || self.subtitle.is_some() || self.header.is_some()
    }

    /// The columns and rows of the frame and of the `Elevated` shadow
    fn frame_and_shadow(&self) -> (u16, u16) {
        let frame = if self.border != BorderType::None {
            2
        } else {
            0
        };
        (frame, u16::from(self.variant == CardVariant::Elevated))
    }

    /// `least` for the body with the rest of the card around it
    fn around_body(&self, least: (u16, u16)) -> (u16, u16) {
        let footer = if self.shown() {
            self.footer_height()
        } else {
            0
        };
        crate::widget::layout::constraints::floor_around(
            least,
            (
                self.columns_beside_body(),
                self.rows_without_body_and_footer().saturating_add(footer),
            ),
        )
    }

    /// The columns beside the body: the frame, the padding and the shadow
    fn columns_beside_body(&self) -> u16 {
        let (frame, shadow) = self.frame_and_shadow();
        frame + self.padding.saturating_mul(2) + shadow
    }

    /// The rows of everything but the body and the footer: the frame, the
    /// shadow, the title, the subtitle, and while shown the header and the
    /// separator between it and the body.
    fn rows_without_body_and_footer(&self) -> u16 {
        let (frame, shadow) = self.frame_and_shadow();
        let mut rows = frame + shadow;
        rows += u16::from(self.title.is_some()) + u16::from(self.subtitle.is_some());
        if self.shown() {
            rows += u16::from(self.header.is_some());
            rows += u16::from(self.body.is_some() && self.has_header());
        }
        rows
    }
}

impl View for Card {
    crate::impl_view_meta!("Card");

    /// As wide as offered (the frame and background run the full width) and
    /// as tall as its rows: the frame, title, subtitle, header, separators,
    /// the body's measured height and the footer - plus the shadow row of
    /// `Elevated`. A body that fills (`None`) makes the card fill. Never
    /// shorter than the 3 rows `render` needs to draw anything.
    ///
    /// The frame counted is the builder's: a stylesheet `border-style`
    /// is applied at paint time, which `measure` cannot see.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let mut rows = self.rows_without_body_and_footer();
        if self.shown() {
            if let Some(body) = &self.body {
                let (_, h) = body.measure(
                    max_width.saturating_sub(self.columns_beside_body()),
                    max_height.saturating_sub(rows),
                )?;
                rows = rows.saturating_add(h);
            }
            rows = rows.saturating_add(self.footer_height());
        }

        let mut h = rows.max(3).max(self.min_height);
        if self.max_height > 0 {
            h = h.min(self.max_height);
        }
        let width = crate::widget::layout::constraints::constrain(
            Rect::new(0, 0, max_width, 0),
            self.min_width,
            0,
            self.max_width,
            0,
        )
        .width;
        Some((width.min(max_width), h.min(max_height)))
    }

    /// It stretches across the width it is offered.
    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }

    /// The shown body's minimum with the rest of the card around it, up to the
    /// card's maximum.
    fn min_size(&self) -> (u16, u16) {
        let body = self.body.as_ref().filter(|_| self.shown());
        let (w, h) = self.around_body(body.map_or((0, 0), |body| body.min_size()));
        let area = crate::widget::layout::constraints::constrain(
            Rect::new(0, 0, w, h),
            0,
            0,
            self.max_width,
            self.max_height,
        );
        (area.width, area.height)
    }

    /// What the shown body holds with the rest of the card around it, within
    /// the card's own bounds.
    fn held_size(&self) -> (u16, u16) {
        let body = self.body.as_ref().filter(|_| self.shown());
        let (w, h) = self.around_body(body.map_or((0, 0), |body| body.held_size()));
        let area = crate::widget::layout::constraints::constrain(
            Rect::new(0, 0, w, h),
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        );
        (area.width, area.height)
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = crate::widget::layout::constraints::constrain(
            ctx.area,
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        );
        crate::widget::layout::constraints::within(ctx, area, |ctx| self.render_constrained(ctx));
    }
}

/// The card's box after the shadow, and where its content goes (offsets
/// within the widget)
struct Inner {
    area: Rect,
    has_border: bool,
    border_color: Color,
    content_x: u16,
    content_width: u16,
}

impl Inner {
    fn new(area: Rect, has_border: bool, border_color: Color, padding: u16) -> Self {
        // Content area (relative offsets within this widget)
        let content_x = if has_border { 1 + padding } else { padding };
        let content_width = if has_border {
            area.width.saturating_sub(2 + padding * 2)
        } else {
            area.width.saturating_sub(padding * 2)
        };
        Self {
            area,
            has_border,
            border_color,
            content_x,
            content_width,
        }
    }

    /// The four corners and edges of the frame
    fn draw_frame(&self, ctx: &mut RenderContext, chars: BorderChars) {
        let area = self.area;
        let border_color = self.border_color;
        // Corners
        ctx.set(0, 0, Cell::new(chars.top_left).fg(border_color));
        ctx.set(
            area.width - 1,
            0,
            Cell::new(chars.top_right).fg(border_color),
        );
        ctx.set(
            0,
            area.height - 1,
            Cell::new(chars.bottom_left).fg(border_color),
        );
        ctx.set(
            area.width - 1,
            area.height - 1,
            Cell::new(chars.bottom_right).fg(border_color),
        );

        // Top and bottom borders
        for x in 1..area.width - 1 {
            ctx.set(x, 0, Cell::new(chars.horizontal).fg(border_color));
            ctx.set(
                x,
                area.height - 1,
                Cell::new(chars.horizontal).fg(border_color),
            );
        }

        // Side borders
        for y in 1..area.height - 1 {
            ctx.set(0, y, Cell::new(chars.vertical).fg(border_color));
            ctx.set(
                area.width - 1,
                y,
                Cell::new(chars.vertical).fg(border_color),
            );
        }
    }

    /// A `├───┤` row across the frame
    fn draw_framed_separator(&self, ctx: &mut RenderContext, sep_y: u16) {
        let area = self.area;
        let border_color = self.border_color;
        ctx.set(0, sep_y, Cell::new('├').fg(border_color));
        ctx.set(area.width - 1, sep_y, Cell::new('┤').fg(border_color));
        for x in 1..area.width - 1 {
            ctx.set(x, sep_y, Cell::new('─').fg(border_color));
        }
    }
}

impl Card {
    /// Draw into `ctx.area`, already constrained.
    fn render_constrained(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 || area.height < 3 {
            return;
        }

        let (bg_color, border_color, title_color) = self.effective_colors_with_css(ctx);
        let border_type = self.border_type_with_css(ctx);
        let chars = border_type.chars();
        let has_border = border_type != BorderType::None;

        // Fill background for filled/elevated variants
        if let Some(bg) = bg_color {
            ctx.fill_box_background(bg);
        }

        let area = self.draw_shadow(ctx, area);
        let inner = Inner::new(area, has_border, border_color, self.padding);

        // Draw border
        if has_border {
            inner.draw_frame(ctx, chars);
        }

        let mut current_y: u16 = if has_border { 1 } else { 0 };
        current_y = self.draw_heading(ctx, &inner, title_color, current_y);
        current_y = self.draw_header_separator(ctx, &inner, current_y);
        current_y = self.draw_body(ctx, &inner, current_y);
        self.draw_footer(ctx, &inner, current_y);
    }

    /// Draw shadow for elevated variant (inside area bounds). The shadow
    /// takes the last column and row and the card is drawn in what is
    /// left, so its border does not paint over the shadow. Returns the
    /// card's box.
    fn draw_shadow(&self, ctx: &mut RenderContext, area: Rect) -> Rect {
        if self.variant == CardVariant::Elevated && area.width > 4 && area.height > 3 {
            let shadow_color = Color::rgb(20, 20, 20);
            // Right shadow (last column, below the card's top row)
            for y in 1..area.height {
                let mut cell = Cell::new('▌');
                cell.fg = Some(shadow_color);
                ctx.set(area.width - 1, y, cell);
            }
            // Bottom shadow (last row, right of the card's left edge)
            for x in 1..area.width {
                let mut cell = Cell::new('▀');
                cell.fg = Some(shadow_color);
                ctx.set(x, area.height - 1, cell);
            }
            Rect::new(area.x, area.y, area.width - 1, area.height - 1)
        } else {
            area
        }
    }

    /// The title (with the collapse icon), the subtitle and the custom
    /// header, from row `current_y`. Returns the row below them.
    fn draw_heading(
        &self,
        ctx: &mut RenderContext,
        inner: &Inner,
        title_color: Color,
        mut current_y: u16,
    ) -> u16 {
        let content_x = inner.content_x;
        let content_width = inner.content_width;

        // Draw title
        if let Some(ref title) = self.title {
            let title_x = content_x;

            // Collapse icon
            if self.collapsible {
                let mut icon_cell = Cell::new(self.collapse_icon());
                icon_cell.fg = Some(title_color);
                ctx.set(title_x, current_y, icon_cell);

                TextDraw {
                    text: title,
                    x: title_x + 2,
                    y: current_y,
                    color: title_color,
                    max_width: content_width.saturating_sub(2),
                    bold: true,
                }
                .draw(ctx);
            } else {
                TextDraw {
                    text: title,
                    x: title_x,
                    y: current_y,
                    color: title_color,
                    max_width: content_width,
                    bold: true,
                }
                .draw(ctx);
            }
            current_y += 1;
        }

        // Draw subtitle
        if let Some(ref subtitle) = self.subtitle {
            TextDraw {
                text: subtitle,
                x: content_x,
                y: current_y,
                color: LIGHT_GRAY,
                max_width: content_width,
                bold: false,
            }
            .draw(ctx);
            current_y += 1;
        }

        // Draw custom header
        if let Some(ref header) = self.header {
            if self.shown() {
                let header_area = ctx.sub_area(content_x, current_y, content_width, 1);
                ctx.render_child(header.as_ref(), header_area);
                current_y += 1;
            }
        }
        current_y
    }

    /// Draw header separator if we have header content and body. Returns
    /// the row below it.
    fn draw_header_separator(
        &self,
        ctx: &mut RenderContext,
        inner: &Inner,
        mut current_y: u16,
    ) -> u16 {
        if self.has_header() && self.body.is_some() && self.shown() {
            // Separator line
            let sep_y = current_y;
            if inner.has_border {
                inner.draw_framed_separator(ctx, sep_y);
            } else {
                for x in 0..inner.area.width {
                    ctx.set(x, sep_y, Cell::new('─').fg(Color::rgb(50, 50, 50)));
                }
            }
            current_y += 1;
        }
        current_y
    }

    /// Draw body (only if expanded or not collapsible) in the rows down to
    /// the footer. Returns the row below it.
    fn draw_body(&self, ctx: &mut RenderContext, inner: &Inner, mut current_y: u16) -> u16 {
        let area = inner.area;
        if let Some(ref body) = self.body {
            if self.shown() {
                let footer_height = self.footer_height();
                let body_end = if inner.has_border {
                    area.height - 1 - footer_height
                } else {
                    area.height - footer_height
                };
                let body_height = body_end.saturating_sub(current_y);

                if body_height > 0 {
                    let body_area =
                        ctx.sub_area(inner.content_x, current_y, inner.content_width, body_height);
                    ctx.render_child(body.as_ref(), body_area);
                    current_y += body_height;
                }
            }
        }
        current_y
    }

    /// Draw footer separator and content on the card's last rows, if
    /// `current_y` has not passed the separator
    fn draw_footer(&self, ctx: &mut RenderContext, inner: &Inner, current_y: u16) {
        let area = inner.area;
        if let Some(ref footer) = self.footer {
            if self.shown() {
                let footer_y = if inner.has_border {
                    area.height - 2
                } else {
                    area.height - 1
                };

                // Footer separator.
                //
                // `>=`, not `>`: the body stops at `body_end`, which is this
                // same row, so it occupies up to `sep_y - 1` and leaves this one
                // free. Requiring a gap meant a card with both a body and a
                // footer never drew the footer at all.
                let sep_y = footer_y - 1;
                if sep_y >= current_y {
                    if inner.has_border {
                        inner.draw_framed_separator(ctx, sep_y);
                    }

                    // Footer content
                    let footer_area =
                        ctx.sub_area(inner.content_x, footer_y, inner.content_width, 1);
                    ctx.render_child(footer.as_ref(), footer_area);
                }
            }
        }
    }
}

/// Text drawing parameters for Card rendering
struct TextDraw<'a> {
    text: &'a str,
    x: u16,
    y: u16,
    color: Color,
    max_width: u16,
    bold: bool,
}

impl TextDraw<'_> {
    /// Draw text with clipping and wide character support
    fn draw(self, ctx: &mut RenderContext) {
        let mut offset = 0u16;
        for ch in self.text.chars() {
            let ch_width = char_width(ch) as u16;
            if ch_width == 0 {
                continue;
            }
            if offset + ch_width > self.max_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(self.color);
            if self.bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(self.x + offset, self.y, cell);
            for i in 1..ch_width {
                ctx.set(self.x + offset + i, self.y, Cell::continuation());
            }
            offset += ch_width;
        }
    }
}
