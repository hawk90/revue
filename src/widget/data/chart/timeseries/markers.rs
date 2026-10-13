//! Time markers: a line or a point at a timestamp, with its label under the plot

use super::types::MarkerStyle;
use super::view::Plot;
use super::TimeSeries;
use crate::render::Cell;
use crate::utils::{char_width, display_width};
use crate::widget::traits::RenderContext;

impl TimeSeries {
    /// Every marker inside the time range
    pub(super) fn draw_markers(&self, ctx: &mut RenderContext, plot: &Plot) {
        let Plot {
            x: plot_x,
            y: plot_y,
            width: plot_width,
            height: plot_height,
            time_min,
            time_max,
            time_range,
            ..
        } = *plot;
        let area = ctx.area;

        for marker in &self.markers {
            if marker.timestamp >= time_min && marker.timestamp <= time_max && time_range > 0 {
                let x_pos = ((marker.timestamp - time_min) as f64 / time_range as f64
                    * (plot_width - 1) as f64) as u16;
                let x = plot_x + x_pos;

                match marker.style {
                    MarkerStyle::Line => {
                        for y in plot_y..plot_y + plot_height {
                            let mut cell = Cell::new('│');
                            cell.fg = Some(marker.color);
                            ctx.set(x, y, cell);
                        }
                    }
                    MarkerStyle::Point | MarkerStyle::Region => {
                        let mut cell = Cell::new('▼');
                        cell.fg = Some(marker.color);
                        ctx.set(x, plot_y, cell);
                    }
                }

                if !marker.label.is_empty() {
                    let label_x = x.saturating_sub(display_width(&marker.label) as u16 / 2);
                    let mut dx: u16 = 0;
                    for ch in marker.label.chars() {
                        let lx = label_x + dx;
                        if lx < area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(marker.color);
                            cell.bg = self.bg_color;
                            ctx.set(lx, plot_y + plot_height + 1, cell);
                        }
                        dx += char_width(ch) as u16;
                    }
                }
            }
        }
    }
}
