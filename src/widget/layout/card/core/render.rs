//! Drawing the card (frame, header, body, footer, shadow) and measuring it

use super::super::super::border::BorderType;
use super::super::types::CardVariant;
use super::Card;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::widget::theme::{DARK_BG, LIGHT_GRAY};
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

    /// Get effective colors based on variant and state
    /// [`effective_colors`](Self::effective_colors), with the stylesheet
    /// filling in whatever the builder left unset.
    ///
    /// The builder is the inline style and wins; a variant's defaults are the
    /// bottom row, below the stylesheet. Same precedence as every other paint
    /// property in the crate.
    fn effective_colors_with_css(&self, ctx: &RenderContext) -> (Option<Color>, Color, Color) {
        let (bg, border, title) = self.effective_colors();

        let css_bg = ctx
            .style
            .map(|s| s.visual.background)
            .filter(|c| *c != Color::default());
        let bg = self.bg_color.or(css_bg).or(bg);

        let border = match (self.border_color, ctx.css_border_or_text_color()) {
            (Some(explicit), _) => explicit,
            (None, Some(css)) => css,
            (None, None) => border,
        };

        (bg, border, title)
    }

    /// The border to draw: the stylesheet's if it named one, else the builder's.
    ///
    /// See `Border::border_type_with_css` - a stylesheet that said nothing
    /// leaves the builder's choice alone, and `border-style: none` removes the
    /// border.
    fn border_type_with_css(&self, ctx: &RenderContext) -> BorderType {
        match ctx.css_border_style() {
            None => self.border,
            Some(crate::style::BorderStyle::None) => BorderType::None,
            Some(crate::style::BorderStyle::Solid) | Some(crate::style::BorderStyle::Dashed) => {
                BorderType::Single
            }
            Some(crate::style::BorderStyle::Double) => BorderType::Double,
            Some(crate::style::BorderStyle::Rounded) => BorderType::Rounded,
        }
    }

    fn effective_colors(&self) -> (Option<Color>, Color, Color) {
        let default_bg: Option<Color> = match self.variant {
            CardVariant::Outlined => None,
            CardVariant::Filled => Some(Color::rgb(30, 30, 35)),
            CardVariant::Elevated => Some(Color::rgb(35, 35, 40)),
            CardVariant::Flat => None,
        };

        let default_border = match self.variant {
            CardVariant::Outlined => Color::rgb(60, 60, 70),
            CardVariant::Filled => Color::rgb(50, 50, 60),
            CardVariant::Elevated => Color::rgb(70, 70, 80),
            CardVariant::Flat => DARK_BG,
        };

        let default_title = Color::WHITE;

        let bg = self.bg_color.or(default_bg);
        let border = self.border_color.unwrap_or(default_border);
        let title = self.title_color.unwrap_or(default_title);

        // Adjust for focus state
        if self.state.focused && self.clickable {
            (bg, Color::CYAN, title)
        } else {
            (bg, border, title)
        }
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
        let shown = self.expanded || !self.collapsible;
        let frame: u16 = if self.border != BorderType::None {
            2
        } else {
            0
        };
        let shadow = u16::from(self.variant == CardVariant::Elevated);

        let mut rows = frame + shadow;
        rows += u16::from(self.title.is_some()) + u16::from(self.subtitle.is_some());
        let has_header = self.title.is_some() || self.subtitle.is_some() || self.header.is_some();
        if shown {
            rows += u16::from(self.header.is_some());
            if let Some(body) = &self.body {
                rows += u16::from(has_header); // separator under the header
                let side = frame + self.padding.saturating_mul(2) + shadow;
                let (_, h) = body.measure(
                    max_width.saturating_sub(side),
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

        // Draw shadow for elevated variant (inside area bounds). The shadow
        // takes the last column and row and the card is drawn in what is
        // left, so its border does not paint over the shadow.
        let area = if self.variant == CardVariant::Elevated && area.width > 4 && area.height > 3 {
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
        };

        // Draw border
        if has_border {
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

        // Content area (relative offsets within this widget)
        let content_x = if has_border {
            1 + self.padding
        } else {
            self.padding
        };
        let content_width = if has_border {
            area.width.saturating_sub(2 + self.padding * 2)
        } else {
            area.width.saturating_sub(self.padding * 2)
        };
        let mut current_y: u16 = if has_border { 1 } else { 0 };

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
            if self.expanded || !self.collapsible {
                let header_area = ctx.sub_area(content_x, current_y, content_width, 1);
                ctx.render_child(header.as_ref(), header_area);
                current_y += 1;
            }
        }

        // Draw header separator if we have header content and body
        let has_header = self.title.is_some() || self.subtitle.is_some() || self.header.is_some();
        if has_header && self.body.is_some() && (self.expanded || !self.collapsible) {
            // Separator line
            let sep_y = current_y;
            if has_border {
                ctx.set(0, sep_y, Cell::new('├').fg(border_color));
                ctx.set(area.width - 1, sep_y, Cell::new('┤').fg(border_color));
                for x in 1..area.width - 1 {
                    ctx.set(x, sep_y, Cell::new('─').fg(border_color));
                }
            } else {
                for x in 0..area.width {
                    ctx.set(x, sep_y, Cell::new('─').fg(Color::rgb(50, 50, 50)));
                }
            }
            current_y += 1;
        }

        // Draw body (only if expanded or not collapsible)
        if let Some(ref body) = self.body {
            if self.expanded || !self.collapsible {
                let footer_height = self.footer_height();
                let body_end = if has_border {
                    area.height - 1 - footer_height
                } else {
                    area.height - footer_height
                };
                let body_height = body_end.saturating_sub(current_y);

                if body_height > 0 {
                    let body_area = ctx.sub_area(content_x, current_y, content_width, body_height);
                    ctx.render_child(body.as_ref(), body_area);
                    current_y += body_height;
                }
            }
        }

        // Draw footer separator and content
        if let Some(ref footer) = self.footer {
            if self.expanded || !self.collapsible {
                let footer_y = if has_border {
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
                    if has_border {
                        ctx.set(0, sep_y, Cell::new('├').fg(border_color));
                        ctx.set(area.width - 1, sep_y, Cell::new('┤').fg(border_color));
                        for x in 1..area.width - 1 {
                            ctx.set(x, sep_y, Cell::new('─').fg(border_color));
                        }
                    }

                    // Footer content
                    let footer_area = ctx.sub_area(content_x, footer_y, content_width, 1);
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
