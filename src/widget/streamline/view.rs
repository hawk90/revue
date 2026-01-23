//! View implementation for Streamline chart

use super::core::Streamline;
use crate::render::Cell;
use crate::widget::traits::{RenderContext, View};

impl View for Streamline {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 5 || height < 3 {
            return;
        }

        // Background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + height.min(area.height) {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        let mut chart_y = area.y;
        let mut chart_height = height.min(area.height);

        // Title
        if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            ctx.buffer.put_str_styled(
                title_x,
                chart_y,
                title,
                Some(crate::style::Color::WHITE),
                self.bg_color,
            );
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        // Legend
        if self.show_legend && !self.layers.is_empty() {
            let mut x = area.x + 1;
            for (i, layer) in self.layers.iter().enumerate() {
                let color = self.get_layer_color(i);
                let mut cell = Cell::new('█');
                cell.fg = Some(color);
                ctx.buffer.set(x, chart_y, cell);
                x += 2;
                ctx.buffer.put_str_styled(
                    x,
                    chart_y,
                    &layer.name,
                    Some(crate::style::Color::WHITE),
                    self.bg_color,
                );
                x += layer.name.len() as u16 + 2;

                if x > area.x + area.width - 10 {
                    break;
                }
            }
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        // Reserve space for x-axis labels
        let plot_height = if self.x_labels.is_empty() {
            chart_height
        } else {
            chart_height.saturating_sub(1)
        };

        if plot_height < 2 {
            return;
        }

        let stacks = self.compute_stacks();
        if stacks.is_empty() || stacks[0].is_empty() {
            return;
        }

        let num_points = stacks[0].len();

        // Find global min/max for scaling
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for layer_stack in &stacks {
            for &(y0, y1) in layer_stack {
                min_y = min_y.min(y0);
                max_y = max_y.max(y1);
            }
        }

        if min_y == max_y {
            max_y = min_y + 1.0;
        }

        let y_range = max_y - min_y;

        // Render each layer
        for (layer_idx, layer_stack) in stacks.iter().enumerate() {
            let color = self.get_layer_color(layer_idx);
            let is_highlighted = self.highlighted.is_none() || self.highlighted == Some(layer_idx);

            let display_color = if is_highlighted {
                color
            } else {
                crate::style::Color::rgb(
                    (color.r as u16 / 3) as u8,
                    (color.g as u16 / 3) as u8,
                    (color.b as u16 / 3) as u8,
                )
            };

            for x_idx in 0..num_points {
                let (y0, y1) = layer_stack[x_idx];

                let screen_x = area.x
                    + (x_idx as f64 / (num_points - 1).max(1) as f64 * (area.width - 1) as f64)
                        as u16;

                let screen_y0 = chart_y + plot_height
                    - 1
                    - ((y0 - min_y) / y_range * (plot_height - 1) as f64) as u16;
                let screen_y1 = chart_y + plot_height
                    - 1
                    - ((y1 - min_y) / y_range * (plot_height - 1) as f64) as u16;

                let (top_y, bottom_y) = if screen_y0 <= screen_y1 {
                    (screen_y0, screen_y1)
                } else {
                    (screen_y1, screen_y0)
                };

                for y in top_y..=bottom_y {
                    if y >= chart_y && y < chart_y + plot_height {
                        let mut cell = Cell::new('█');
                        cell.fg = Some(display_color);
                        ctx.buffer.set(screen_x, y, cell);
                    }
                }
            }

            // Draw label on the stream
            if self.show_labels && !self.layers[layer_idx].name.is_empty() {
                let mut max_width_x = 0;
                let mut max_width = 0.0f64;

                for (x_idx, &(y0, y1)) in layer_stack.iter().enumerate() {
                    let width = (y1 - y0).abs();
                    if width > max_width {
                        max_width = width;
                        max_width_x = x_idx;
                    }
                }

                let (y0, y1) = layer_stack[max_width_x];
                let mid_y = (y0 + y1) / 2.0;
                let screen_x = area.x
                    + (max_width_x as f64 / (num_points - 1).max(1) as f64
                        * (area.width - 1) as f64) as u16;
                let screen_y = chart_y + plot_height
                    - 1
                    - ((mid_y - min_y) / y_range * (plot_height - 1) as f64) as u16;

                let label = &self.layers[layer_idx].name;
                let label_x = screen_x.saturating_sub(label.len() as u16 / 2);

                if screen_y >= chart_y && screen_y < chart_y + plot_height {
                    ctx.buffer.put_str_styled(
                        label_x,
                        screen_y,
                        label,
                        Some(crate::style::Color::WHITE),
                        Some(display_color),
                    );
                }
            }
        }

        // X-axis labels
        if !self.x_labels.is_empty() {
            let label_y = chart_y + plot_height;
            let num_labels = self.x_labels.len().min(area.width as usize / 8);

            for (i, label) in self.x_labels.iter().take(num_labels).enumerate() {
                let x = area.x
                    + (i as f64 / (num_labels - 1).max(1) as f64 * (area.width - 1) as f64) as u16;
                let label_x = x.saturating_sub(label.len() as u16 / 2);
                ctx.buffer.put_str_styled(
                    label_x,
                    label_y,
                    label,
                    Some(crate::style::Color::WHITE),
                    self.bg_color,
                );
            }
        }
    }

    crate::impl_view_meta!("Streamline");
}
