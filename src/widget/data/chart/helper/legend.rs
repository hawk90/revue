//! Chart legend: a box listing each series with its color

use super::render::PlotLayout;
use super::Chart;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::char_width;
use crate::widget::data::chart::chart_common::LegendPosition;
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::RenderContext;

impl Chart {
    /// Draw legend
    pub(super) fn draw_legend(&self, ctx: &mut RenderContext, plot: &PlotLayout, chrome: Color) {
        let PlotLayout {
            inner_x,
            inner_y,
            inner_w,
            inner_h,
            ..
        } = *plot;
        if !matches!(self.legend, LegendPosition::None) && !self.series.is_empty() {
            // Sized in usize and clamped to u16: a count cast to u16 and
            // then added to overflows near 65535 (#1552)
            let clamp = |n: usize| u16::try_from(n).unwrap_or(u16::MAX);
            let legend_width = clamp(
                self.series
                    .iter()
                    .map(|s| s.name.len().saturating_add(4))
                    .max()
                    .unwrap_or(10),
            );
            let legend_height = clamp(self.series.len().saturating_add(2));

            // A legend larger than the plot is pinned to its top/left edge
            // (and clipped below) instead of underflowing.
            let left = inner_x + 1;
            let center_x = inner_x + inner_w.saturating_sub(legend_width) / 2;
            let right = inner_x + inner_w.saturating_sub(legend_width.saturating_add(1));
            let top = inner_y + 1;
            let middle_y = inner_y + inner_h.saturating_sub(legend_height) / 2;
            let bottom = inner_y + inner_h.saturating_sub(legend_height.saturating_add(1));
            let (legend_x, legend_y) = match self.legend {
                LegendPosition::TopLeft => (left, top),
                LegendPosition::TopCenter => (center_x, top),
                LegendPosition::TopRight => (right, top),
                LegendPosition::BottomLeft => (left, bottom),
                LegendPosition::BottomCenter => (center_x, bottom),
                LegendPosition::BottomRight => (right, bottom),
                LegendPosition::Left => (left, middle_y),
                LegendPosition::Right => (right, middle_y),
                // None is filtered out by the if condition above, but provide fallback
                LegendPosition::None => (inner_x + 1, inner_y + 1),
            };

            // Draw legend background, as much of it as is in the plot
            let plot_right = inner_x.saturating_add(inner_w);
            let plot_bottom = inner_y.saturating_add(inner_h);
            for dy in 0..legend_height.min(plot_bottom.saturating_sub(legend_y)) {
                for dx in 0..legend_width.min(plot_right.saturating_sub(legend_x)) {
                    let x = legend_x + dx;
                    let y = legend_y + dy;
                    let ch = if dy == 0 && dx == 0 {
                        '┌'
                    } else if dy == 0 && dx == legend_width - 1 {
                        '┐'
                    } else if dy == legend_height - 1 && dx == 0 {
                        '└'
                    } else if dy == legend_height - 1 && dx == legend_width - 1 {
                        '┘'
                    } else if dy == 0 || dy == legend_height - 1 {
                        '─'
                    } else if dx == 0 || dx == legend_width - 1 {
                        '│'
                    } else {
                        ' '
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(DARK_GRAY);
                    cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                    ctx.set(x, y, cell);
                }
            }

            // Draw legend entries, one per row above the plot's last row
            let label_end = legend_x.saturating_add(legend_width).saturating_sub(1);
            let rows = legend_y.saturating_add(1)..plot_bottom.saturating_sub(1);
            for (y, series) in rows.zip(&self.series) {
                // Color indicator
                let mut indicator = Cell::new('■');
                indicator.fg = Some(series.color);
                ctx.set(legend_x.saturating_add(1), y, indicator);

                // Series name
                let mut x = legend_x.saturating_add(3);
                for ch in series.name.chars() {
                    if x >= label_end {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(chrome);
                    cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                    ctx.set(x, y, cell);
                    x = x.saturating_add(char_width(ch) as u16);
                }
            }
        }
    }
}
