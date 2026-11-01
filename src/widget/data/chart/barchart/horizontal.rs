//! Horizontal bars: label, bar and value on each bar's first row

use super::BarChart;
use crate::render::Cell;
use crate::utils::char_width;
use crate::widget::traits::RenderContext;

impl BarChart {
    /// Render horizontal bars
    pub(super) fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.bars.is_empty() {
            return;
        }

        let max_value = self.calculate_max();

        // Calculate label width
        let label_width = self.label_width.unwrap_or_else(|| {
            self.bars
                .iter()
                .map(|b| crate::utils::display_width(&b.label) as u16)
                .max()
                .unwrap_or(0)
                .min(area.width / 3)
        });

        // Calculate available bar space
        let value_width = if self.show_values { 8 } else { 0 };
        let bar_area_width = area
            .width
            .saturating_sub(label_width.saturating_add(2).saturating_add(value_width));

        let mut y = 0u16;
        for bar in &self.bars {
            if y >= area.height {
                break;
            }

            // Calculate bar length (use absolute value for rendering)
            let bar_length = if max_value > 0.0 {
                ((bar.value.abs() / max_value) * bar_area_width as f64) as u16
            } else {
                0
            };

            // A bar's own color wins; otherwise the stylesheet, then the chart's.
            let color = bar.color.unwrap_or_else(|| ctx.css_color(self.fg));

            // Render for each row of bar_width
            for row in 0..self.bar_width {
                if y + row >= area.height {
                    break;
                }

                // Draw label (only on first row)
                if row == 0 {
                    let label_dw = crate::utils::display_width(&bar.label);
                    let label: String = if label_dw > label_width as usize {
                        crate::utils::truncate_to_width(&bar.label, label_width as usize)
                            .to_string()
                    } else {
                        crate::utils::unicode::right_align_to_width(
                            &bar.label,
                            label_width as usize,
                        )
                    };

                    let mut dx: u16 = 0;
                    for ch in label.chars() {
                        if dx < area.width {
                            ctx.set(dx, y, Cell::new(ch));
                        }
                        dx += char_width(ch) as u16;
                    }
                }

                // Draw bar
                let bar_start = label_width.saturating_add(1);
                for i in 0..bar_length.min(area.width.saturating_sub(bar_start)) {
                    if bar_start + i < area.width {
                        let mut cell = Cell::new('█');
                        cell.fg = Some(color);
                        ctx.set(bar_start + i, y + row, cell);
                    }
                }

                // Draw value (only on first row)
                if row == 0 && self.show_values {
                    let value_str = format!(" {:.1}", bar.value);
                    let value_x = bar_start.saturating_add(bar_length);
                    let mut dx: u16 = 0;
                    for ch in value_str.chars() {
                        if value_x.saturating_add(dx) < area.width {
                            ctx.set(value_x + dx, y, Cell::new(ch));
                        }
                        dx += char_width(ch) as u16;
                    }
                }
            }

            y = y.saturating_add(self.bar_width).saturating_add(self.gap);
        }
    }
}
