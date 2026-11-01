//! Vertical bars: value above, bar rising from the label row, label below

use super::BarChart;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::traits::RenderContext;

impl BarChart {
    /// Render vertical bars
    pub(super) fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.bars.is_empty() {
            return;
        }

        let max_value = self.calculate_max();

        // Reserve space for labels and values
        let label_height = 1;
        let value_height = if self.show_values { 1 } else { 0 };
        let bar_area_height = area.height.saturating_sub(label_height + value_height);

        let total_bar_width = self.bar_width.saturating_add(self.gap);
        let mut x = 0u16;

        for bar in &self.bars {
            if x.saturating_add(self.bar_width) > area.width {
                break;
            }

            // Calculate bar height (use absolute value, as horizontal bars do)
            let bar_height = if max_value > 0.0 {
                // A `max` below the data still draws inside the chart.
                ((bar.value.abs() / max_value) * bar_area_height as f64)
                    .min(f64::from(bar_area_height)) as u16
            } else {
                0
            };

            // A bar's own color wins; otherwise the stylesheet, then the chart's.
            let color = bar
                .color
                .or(self.fg)
                .unwrap_or_else(|| ctx.css_color(Color::CYAN));

            // Draw bar (from bottom up), below the reserved value row
            for row in 0..bar_height {
                let y = value_height + bar_area_height - 1 - row;
                for col in 0..self.bar_width.min(area.width.saturating_sub(x)) {
                    if x + col < area.width {
                        let mut cell = Cell::new('█');
                        cell.fg = Some(color);
                        ctx.set(x + col, y, cell);
                    }
                }
            }

            // Draw value above bar
            if self.show_values && bar_area_height > 0 {
                let value_str = format!("{:.0}", bar.value);
                let value_y = bar_area_height - bar_height.saturating_sub(1).min(bar_area_height);
                let mut dx: u16 = 0;
                for ch in value_str.chars() {
                    if x + dx < area.width && value_y > 0 {
                        ctx.set(x + dx, value_y - 1, Cell::new(ch));
                    }
                    dx += char_width(ch) as u16;
                }
            }

            // Draw label below
            if label_height > 0 {
                let label_y = area.height - 1;
                let label = truncate_to_width(&bar.label, self.bar_width as usize);
                let mut dx: u16 = 0;
                for ch in label.chars() {
                    if x + dx < area.width {
                        ctx.set(x + dx, label_y, Cell::new(ch));
                    }
                    dx += char_width(ch) as u16;
                }
            }

            x += total_bar_width;
        }
    }
}
