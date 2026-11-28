//! Chart axes: tick labels, grid lines and axis titles

use super::render::PlotLayout;
use super::Chart;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::{display_width_u16, truncate_with_ellipsis};
use crate::utils::{char_width, display_width};
use crate::widget::data::chart::chart_common::AxisFormat;
use crate::widget::traits::RenderContext;

impl Chart {
    /// Format axis label
    fn format_label(&self, value: f64, format: &AxisFormat) -> String {
        match format {
            AxisFormat::Auto => {
                if value.abs() >= 1000.0 || (value != 0.0 && value.abs() < 0.01) {
                    format!("{:.1e}", value)
                } else if value.fract() == 0.0 {
                    format!("{:.0}", value)
                } else {
                    format!("{:.2}", value)
                }
            }
            AxisFormat::Integer => format!("{:.0}", value),
            AxisFormat::Fixed(decimals) => format!("{:.1$}", value, *decimals),
            AxisFormat::Percent => format!("{:.0}%", value * 100.0),
            AxisFormat::Custom(fmt) => fmt.replace("{}", &value.to_string()),
        }
    }

    /// The Y tick label of tick `i` of `ticks`, and where it goes from the
    /// bottom of the axis (0) to the top (1)
    fn y_tick_label(&self, i: usize, ticks: usize, bounds: (f64, f64, f64, f64)) -> (f64, String) {
        let (_, _, y_min, y_max) = bounds;
        let t = i as f64 / ticks as f64;
        let value = y_min + t * (y_max - y_min);
        (t, self.format_label(value, &self.y_axis.format))
    }

    /// The width of the widest Y tick label, in columns
    pub(super) fn y_label_columns(&self, bounds: (f64, f64, f64, f64)) -> u16 {
        let ticks = self.y_axis.ticks.max(1);
        (0..=ticks)
            .map(|i| display_width_u16(&self.y_tick_label(i, ticks, bounds).1))
            .max()
            .unwrap_or(0)
    }

    /// Draw Y axis labels, right-aligned in the label column; a label wider
    /// than the column is cut with `…` (#1606)
    pub(super) fn draw_y_axis(
        &self,
        ctx: &mut RenderContext,
        plot: &PlotLayout,
        bounds: (f64, f64, f64, f64),
    ) {
        let PlotLayout {
            y_label_width,
            inner_x,
            inner_y,
            inner_w,
            inner_h,
            y_label_x,
            ..
        } = *plot;
        let columns = y_label_width.saturating_sub(1);
        let ticks = self.y_axis.ticks.max(1);
        for i in 0..=ticks {
            let (t, label) = self.y_tick_label(i, ticks, bounds);
            let label = truncate_with_ellipsis(&label, columns as usize);
            let y = inner_y + inner_h - 1 - ((t * (inner_h as f64 - 1.0)) as u16);

            // Right-align label
            let label_start = y_label_x + columns.saturating_sub(display_width_u16(&label));
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let x = label_start.saturating_add(dx);
                if x >= ctx.area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.set(x, y, cell);
                dx = dx.saturating_add(char_width(ch) as u16);
            }

            // Draw grid line
            if self.y_axis.grid && i > 0 && i < ticks {
                for x in inner_x..inner_x + inner_w {
                    let mut cell = Cell::new('┄');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.set(x, y, cell);
                }
            }
        }
    }

    /// Draw X axis labels, centered on their ticks
    pub(super) fn draw_x_axis(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        plot: &PlotLayout,
        bounds: (f64, f64, f64, f64),
    ) {
        let PlotLayout {
            inner_x,
            inner_y,
            inner_w,
            inner_h,
            x_label_y,
            ..
        } = *plot;
        let (x_min, x_max, _, _) = bounds;
        let ticks = self.x_axis.ticks.max(1);
        // Each label is cut with `…` to its slot, the columns between two
        // ticks less one, so neighbors never overwrite each other (#1606)
        let slot = usize::from(inner_w.saturating_sub(1)) / ticks;
        let slot = slot.saturating_sub(1).max(1);
        for i in 0..=ticks {
            let t = i as f64 / ticks as f64;
            let value = x_min + t * (x_max - x_min);
            let label = self.format_label(value, &self.x_axis.format);
            let label = truncate_with_ellipsis(&label, slot);
            let x = inner_x + (t * (inner_w as f64 - 1.0)) as u16;

            // Center label
            let half = u16::try_from(display_width(&label) / 2).unwrap_or(u16::MAX);
            let label_start = x.saturating_sub(half);
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let px = label_start.saturating_add(dx);
                if px >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.x_axis.color);
                ctx.set(px, x_label_y, cell);
                dx = dx.saturating_add(char_width(ch) as u16);
            }

            // Draw grid line
            if self.x_axis.grid && i > 0 && i < ticks {
                for y in inner_y..inner_y + inner_h {
                    let mut cell = Cell::new('┊');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.set(x, y, cell);
                }
            }
        }
    }

    /// Draw axis titles
    pub(super) fn draw_axis_titles(&self, ctx: &mut RenderContext, area: Rect, plot: &PlotLayout) {
        let PlotLayout {
            inner_x,
            inner_y,
            inner_w,
            inner_h,
            y_label_x,
            x_label_y,
            ..
        } = *plot;
        if let Some(ref title) = self.y_axis.title {
            // Draw vertically on the left (one char per row, so char count governs height)
            let char_count = u16::try_from(title.chars().count()).unwrap_or(u16::MAX);
            let y_start = inner_y + (inner_h.saturating_sub(char_count)) / 2;
            for (y, ch) in (y_start..area.height).zip(title.chars()) {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.set(y_label_x, y, cell);
            }
        }

        if let Some(ref title) = self.x_axis.title {
            let x_start = inner_x + (inner_w.saturating_sub(display_width_u16(title))) / 2;
            let y = x_label_y + 1;
            if y < area.height {
                let mut dx: u16 = 0;
                for ch in title.chars() {
                    let x = x_start.saturating_add(dx);
                    if x >= area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.set(x, y, cell);
                    dx = dx.saturating_add(char_width(ch) as u16);
                }
            }
        }
    }
}
