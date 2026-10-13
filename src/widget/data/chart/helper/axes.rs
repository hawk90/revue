//! Chart axes: tick labels, grid lines and axis titles

use super::render::PlotLayout;
use super::Chart;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
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

    /// Draw Y axis labels
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
        let (_, _, y_min, y_max) = bounds;
        for i in 0..=self.y_axis.ticks {
            let t = i as f64 / self.y_axis.ticks as f64;
            let value = y_min + t * (y_max - y_min);
            let label = self.format_label(value, &self.y_axis.format);
            let y = inner_y + inner_h - 1 - ((t * (inner_h as f64 - 1.0)) as u16);

            // Right-align label
            let label_start =
                y_label_x + y_label_width.saturating_sub(display_width(&label) as u16 + 1);
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.set(label_start + dx, y, cell);
                dx += char_width(ch) as u16;
            }

            // Draw grid line
            if self.y_axis.grid && i > 0 && i < self.y_axis.ticks {
                for x in inner_x..inner_x + inner_w {
                    let mut cell = Cell::new('┄');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.set(x, y, cell);
                }
            }
        }
    }

    /// Draw X axis labels
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
        for i in 0..=self.x_axis.ticks {
            let t = i as f64 / self.x_axis.ticks as f64;
            let value = x_min + t * (x_max - x_min);
            let label = self.format_label(value, &self.x_axis.format);
            let x = inner_x + (t * (inner_w as f64 - 1.0)) as u16;

            // Center label
            let label_start = x.saturating_sub(display_width(&label) as u16 / 2);
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let px = label_start + dx;
                if px < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.set(px, x_label_y, cell);
                }
                dx += char_width(ch) as u16;
            }

            // Draw grid line
            if self.x_axis.grid && i > 0 && i < self.x_axis.ticks {
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
            let char_count = title.chars().count() as u16;
            let y_start = inner_y + (inner_h.saturating_sub(char_count)) / 2;
            for (i, ch) in title.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.set(y_label_x, y_start + i as u16, cell);
            }
        }

        if let Some(ref title) = self.x_axis.title {
            let x_start = inner_x + (inner_w.saturating_sub(display_width(title) as u16)) / 2;
            let y = x_label_y + 1;
            if y < area.height {
                let mut dx: u16 = 0;
                for ch in title.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.set(x_start + dx, y, cell);
                    dx += char_width(ch) as u16;
                }
            }
        }
    }
}
