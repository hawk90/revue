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
            let legend_width = self
                .series
                .iter()
                .map(|s| s.name.len() + 4)
                .max()
                .unwrap_or(10) as u16;
            let legend_height = self.series.len() as u16 + 2;

            // A legend larger than the plot is pinned to its top/left edge
            // (and clipped below) instead of underflowing.
            let left = inner_x + 1;
            let center_x = inner_x + inner_w.saturating_sub(legend_width) / 2;
            let right = inner_x + inner_w.saturating_sub(legend_width + 1);
            let top = inner_y + 1;
            let middle_y = inner_y + inner_h.saturating_sub(legend_height) / 2;
            let bottom = inner_y + inner_h.saturating_sub(legend_height + 1);
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

            // Draw legend background
            for dy in 0..legend_height {
                for dx in 0..legend_width {
                    let x = legend_x + dx;
                    let y = legend_y + dy;
                    if x < inner_x + inner_w && y < inner_y + inner_h {
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
            }

            // Draw legend entries
            for (i, series) in self.series.iter().enumerate() {
                let y = legend_y + 1 + i as u16;
                if y >= inner_y + inner_h - 1 {
                    break;
                }

                // Color indicator
                let mut indicator = Cell::new('■');
                indicator.fg = Some(series.color);
                ctx.set(legend_x + 1, y, indicator);

                // Series name
                let mut dx: u16 = 0;
                for ch in series.name.chars() {
                    let x = legend_x + 3 + dx;
                    if x < legend_x + legend_width - 1 {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(chrome);
                        cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                        ctx.set(x, y, cell);
                    }
                    dx += char_width(ch) as u16;
                }
            }
        }
    }
}
