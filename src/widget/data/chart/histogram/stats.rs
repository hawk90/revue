//! Histogram statistics overlay: the mean and median lines

use super::Histogram;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::data::chart::chart_stats::{mean, median};
use crate::widget::traits::RenderContext;

impl Histogram {
    /// Render statistics lines
    pub(super) fn render_stats(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if !self.show_stats || self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let x_range = (x_max - x_min).max(1.0);

        // Draw mean line using shared stats function
        if let Some(mean_val) = mean(&self.data) {
            let x = chart_area.x + ((mean_val - x_min) / x_range * chart_area.width as f64) as u16;
            if x >= chart_area.x && x < chart_area.x + chart_area.width {
                for y in chart_area.y..chart_area.y + chart_area.height {
                    let mut cell = Cell::new('│');
                    cell.fg = Some(Color::rgb(224, 108, 117)); // Red
                    ctx.set(x, y, cell);
                }
                // Label
                if x + 1 < chart_area.x + chart_area.width {
                    let mut cell = Cell::new('μ');
                    cell.fg = Some(Color::rgb(224, 108, 117));
                    ctx.set(x + 1, chart_area.y, cell);
                }
            }
        }

        // Draw median line using shared stats function
        if let Some(median_val) = median(&self.data) {
            let x =
                chart_area.x + ((median_val - x_min) / x_range * chart_area.width as f64) as u16;
            if x >= chart_area.x && x < chart_area.x + chart_area.width {
                for y in chart_area.y..chart_area.y + chart_area.height {
                    let mut cell = Cell::new('┊');
                    cell.fg = Some(Color::rgb(152, 195, 121)); // Green
                    ctx.set(x, y, cell);
                }
                // Label
                if x + 1 < chart_area.x + chart_area.width {
                    let mut cell = Cell::new('M');
                    cell.fg = Some(Color::rgb(152, 195, 121));
                    ctx.set(x + 1, chart_area.y, cell);
                }
            }
        }
    }

    /// Render statistics lines for a horizontal histogram: a row across
    /// the plot at the mean and median, labeled at the right end
    pub(super) fn render_stats_horizontal(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if !self.show_stats || self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let x_range = (x_max - x_min).max(1.0);

        let lines = [
            (mean(&self.data), '─', 'μ', Color::rgb(224, 108, 117)), // Red
            (median(&self.data), '┄', 'M', Color::rgb(152, 195, 121)), // Green
        ];
        for (value, line, label, color) in lines {
            let Some(value) = value else {
                continue;
            };
            let offset = (value - x_min) / x_range * chart_area.height as f64;
            if !(0.0..chart_area.height as f64).contains(&offset) {
                continue;
            }
            let y = chart_area.y + offset as u16;
            for x in chart_area.x..chart_area.x + chart_area.width {
                let mut cell = Cell::new(line);
                cell.fg = Some(color);
                ctx.set(x, y, cell);
            }
            // Label just below the right end of the line
            if y + 1 < chart_area.y + chart_area.height {
                let mut cell = Cell::new(label);
                cell.fg = Some(color);
                ctx.set(chart_area.x + chart_area.width - 1, y + 1, cell);
            }
        }
    }
}
