//! Chart frame: the plot layout, border and title, and the `View` impl that
//! draws the frame, axes, series and legend in order

use super::Chart;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::traits::{RenderContext, View};

/// Where the plot and its labels go inside the widget's area
#[derive(Clone, Copy)]
pub(super) struct PlotLayout {
    pub(super) has_border: bool,
    pub(super) y_label_width: u16,
    pub(super) inner_x: u16,
    pub(super) inner_y: u16,
    pub(super) inner_w: u16,
    pub(super) inner_h: u16,
    pub(super) chart_bounds: Rect,
    pub(super) y_label_x: u16,
    pub(super) x_label_y: u16,
}

impl Chart {
    /// Lay out the plot inside `area`; `None` when it is too small to draw
    fn plot_layout(&self, area: Rect) -> Option<PlotLayout> {
        let has_border = self.border_color.is_some();
        let has_title = self.title.is_some();
        let y_label_width = 8u16; // Space for Y axis labels
        let x_label_height = 2u16; // Space for X axis labels

        let inner_x = if has_border { 1 } else { 0 } + y_label_width;
        let inner_y = if has_border { 1 } else { 0 } + if has_title { 1u16 } else { 0 };
        let inner_w = area
            .width
            .saturating_sub(y_label_width + if has_border { 2 } else { 0 });
        let inner_h = area.height.saturating_sub(
            x_label_height + if has_border { 2 } else { 0 } + if has_title { 1 } else { 0 },
        );

        if inner_w < 5 || inner_h < 3 {
            return None;
        }

        let chart_bounds = Rect::new(inner_x, inner_y, inner_w, inner_h);
        let y_label_x = if has_border { 1u16 } else { 0 };
        let x_label_y = inner_y + inner_h;

        Some(PlotLayout {
            has_border,
            y_label_width,
            inner_x,
            inner_y,
            inner_w,
            inner_h,
            chart_bounds,
            y_label_x,
            x_label_y,
        })
    }

    /// Draw border
    fn draw_border(&self, ctx: &mut RenderContext, area: Rect) {
        if let Some(border_color) = self.border_color {
            // Top and bottom
            for x in 0..area.width {
                let mut top = Cell::new('─');
                top.fg = Some(border_color);
                ctx.set(x, 0, top);

                let mut bottom = Cell::new('─');
                bottom.fg = Some(border_color);
                ctx.set(x, area.height - 1, bottom);
            }
            // Left and right
            for y in 0..area.height {
                let mut left = Cell::new('│');
                left.fg = Some(border_color);
                ctx.set(0, y, left);

                let mut right = Cell::new('│');
                right.fg = Some(border_color);
                ctx.set(area.width - 1, y, right);
            }
            // Corners
            let corners = [
                (0u16, 0u16, '┌'),
                (area.width - 1, 0, '┐'),
                (0, area.height - 1, '└'),
                (area.width - 1, area.height - 1, '┘'),
            ];
            for (x, y, ch) in corners {
                let mut cell = Cell::new(ch);
                cell.fg = Some(border_color);
                ctx.set(x, y, cell);
            }
        }
    }

    /// Draw title
    fn draw_title(&self, ctx: &mut RenderContext, area: Rect, plot: &PlotLayout, chrome: Color) {
        let has_border = plot.has_border;
        if let Some(ref title) = self.title {
            let title_x = (area.width.saturating_sub(display_width(title) as u16)) / 2;
            let title_y = if has_border { 1u16 } else { 0 };
            let mut dx: u16 = 0;
            for ch in title.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(chrome);
                cell.modifier |= crate::render::Modifier::BOLD;
                ctx.set(title_x + dx, title_y, cell);
                dx += char_width(ch) as u16;
            }
        }
    }
}

impl View for Chart {
    crate::impl_view_meta!("Chart");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 5 {
            return;
        }

        // The series palette carries the data and each axis has its own color
        // field; the chrome around them - title and legend labels - is
        // ordinary text.
        let chrome = ctx.css_color(Color::WHITE);

        // Fill background
        if let Some(bg) = self.bg_color {
            ctx.fill_box_background(bg);
        }

        self.draw_border(ctx, area);

        let Some(plot) = self.plot_layout(area) else {
            return;
        };

        self.draw_title(ctx, area, &plot, chrome);

        // Compute bounds
        let bounds = self.compute_bounds();

        self.draw_y_axis(ctx, &plot, bounds);
        self.draw_x_axis(ctx, area, &plot, bounds);
        self.draw_axis_titles(ctx, area, &plot);
        self.draw_series(ctx, &plot, bounds);
        self.draw_legend(ctx, &plot, chrome);
    }
}
