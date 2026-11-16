//! View implementation for Accordion

use crate::style::{Color, Style};
use crate::utils::border::render_border;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::render_context::{border_or_text_color, StyledSubtree};
use crate::widget::traits::{RenderContext, View};

use super::{Accordion, AccordionSection};

impl View for Accordion {
    /// Its own bounds, which `render` puts back.
    fn held_size(&self) -> (u16, u16) {
        (self.min_width, self.min_height)
    }

    /// Every row of its sections, inside the builder's border; the headers
    /// and dividers stretch across the offered width.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        self.measure_bordered(max_width, max_height, self.border_color.is_some())
    }

    /// [`measure`](View::measure), inside the border the stylesheet may add
    /// too (#883), so a content-sized stack leaves room for it.
    fn measure_styled(
        &self,
        max_width: u16,
        max_height: u16,
        subtree: StyledSubtree<'_>,
    ) -> Option<(u16, u16)> {
        let bordered = self.resolved_border(subtree.style()).is_some();
        self.measure_bordered(max_width, max_height, bordered)
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }
    crate::impl_view_meta!("Accordion");

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

impl Accordion {
    /// The border's color: the builder's, else the stylesheet's `border-color`,
    /// else its `color` (CSS's `currentColor`). `None` draws no border.
    ///
    /// Drawing, the insets and the measured size all ask this, so they agree
    /// on whether there is a border (#883).
    fn resolved_border(&self, style: Option<&Style>) -> Option<Color> {
        self.border_color
            .or_else(|| style.and_then(border_or_text_color))
    }

    /// Every row of the sections, plus two for a border.
    fn measure_bordered(
        &self,
        max_width: u16,
        max_height: u16,
        bordered: bool,
    ) -> Option<(u16, u16)> {
        let border = if bordered { 2 } else { 0 };
        let rows = u16::try_from(self.rows().len()).unwrap_or(u16::MAX);
        let area = crate::widget::layout::constraints::constrain(
            crate::layout::Rect::new(0, 0, max_width, rows.saturating_add(border)),
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        );
        Some((area.width.min(max_width), area.height.min(max_height)))
    }
}

/// The columns inside the border that headers, content and dividers fill.
#[derive(Clone, Copy)]
struct Inner {
    x: u16,
    width: u16,
}

/// One row of the accordion: a section's header, one of its content lines,
/// or the divider after it
#[derive(Clone, Copy)]
enum Row {
    Header(usize),
    Line(usize, usize),
    Divider,
}

impl Accordion {
    /// Draw into `ctx.area`, already constrained.
    fn render_constrained(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let border = self.resolved_border(ctx.style);
        let (content_x_off, content_y_off, content_width, content_height) = if border.is_some() {
            (
                1u16,
                1u16,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            )
        } else {
            (0u16, 0u16, area.width, area.height)
        };

        if let Some(border_color) = border {
            render_border(ctx, area, border_color);
        }

        // Every row draws inside the border, so a glyph past the inner width
        // (the icon at three columns) is clipped instead of covering a side.
        let inside = ctx.sub_area(content_x_off, content_y_off, content_width, content_height);
        let ctx = &mut ctx.sub_ctx(inside);
        let inner = Inner {
            x: 0,
            width: content_width,
        };
        // The rows, scrolled to keep the selected section in view (#973)
        let rows = self.rows();
        self.scroll.set_content(rows.len());
        self.scroll.set_viewport(content_height as usize);
        let selected = self.selection.index;
        if let Some(start) = rows
            .iter()
            .position(|r| matches!(r, Row::Header(i) if *i == selected))
        {
            let end = rows[start + 1..]
                .iter()
                .position(|r| !matches!(r, Row::Line(i, _) if *i == selected))
                .map_or(rows.len(), |n| start + 1 + n);
            self.scroll.track(start..end);
        }

        for (k, row) in rows[self.scroll.visible()].iter().enumerate() {
            let y = k as u16;
            match *row {
                Row::Header(i) => {
                    let is_selected = self.selection.is_selected(i);
                    self.render_header(ctx, inner, y, &self.sections[i], is_selected);
                }
                Row::Line(i, line) => {
                    self.render_content_line(ctx, inner, y, &self.sections[i].content[line]);
                }
                Row::Divider => ctx.draw_hline(inner.x, y, inner.width, '─', SEPARATOR_COLOR),
            }
        }
    }

    /// Every row the sections take, top to bottom
    fn rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for (i, section) in self.sections.iter().enumerate() {
            rows.push(Row::Header(i));
            if section.expanded {
                rows.extend((0..section.content.len()).map(|line| Row::Line(i, line)));
            }
            if self.show_dividers && i + 1 < self.sections.len() {
                rows.push(Row::Divider);
            }
        }
        rows
    }

    /// A section's header row: its background (the selected one's while
    /// selected), the icon, and the title, bold while selected.
    fn render_header(
        &self,
        ctx: &mut RenderContext,
        inner: Inner,
        y: u16,
        section: &AccordionSection,
        is_selected: bool,
    ) {
        let header_bg = if is_selected {
            self.selected_bg
        } else {
            self.header_bg
        };

        // Fill header background
        ctx.fill_bg(inner.x, y, inner.width, 1, header_bg);

        // Icon
        ctx.draw_char_bg(inner.x + 1, y, section.icon(), self.header_fg, header_bg);

        // Title
        let title_x = inner.x + 3;
        let max_title_width = inner.width.saturating_sub(4);
        if is_selected {
            ctx.draw_text_clipped_bg_bold(
                title_x,
                y,
                &section.title,
                self.header_fg,
                header_bg,
                max_title_width,
            );
        } else {
            ctx.draw_text_clipped_bg(
                title_x,
                y,
                &section.title,
                self.header_fg,
                header_bg,
                max_title_width,
            );
        }
    }

    /// One content row of an expanded section, indented under the title.
    fn render_content_line(&self, ctx: &mut RenderContext, inner: Inner, y: u16, line: &str) {
        // Fill content background
        ctx.fill_bg(inner.x, y, inner.width, 1, self.content_bg);

        // Content with indent
        let content_x = inner.x + 3;
        let max_content_width = inner.width.saturating_sub(4);
        ctx.draw_text_clipped_bg(
            content_x,
            y,
            line,
            self.content_fg,
            self.content_bg,
            max_content_width,
        );
    }
}
