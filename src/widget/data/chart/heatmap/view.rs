//! Heat map widget rendering

use super::HeatMap;
use crate::style::Color;
use crate::widget::RenderContext;
use crate::widget::View;
use crate::widget::{hstack, vstack, Text};

impl View for HeatMap {
    crate::impl_view_meta!("HeatMap");

    fn render(&self, ctx: &mut RenderContext) {
        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Column labels
        if let Some(labels) = &self.col_labels {
            let label_offset = if self.row_labels.is_some() { 8 } else { 0 };
            let mut col_header = hstack();
            col_header = col_header.child(Text::new(" ".repeat(label_offset)));

            for label in labels.iter().take(self.cols) {
                let truncated = if label.len() > self.cell_width {
                    &label[..self.cell_width]
                } else {
                    label
                };
                col_header = col_header.child(
                    Text::new(format!("{:^width$}", truncated, width = self.cell_width))
                        .fg(Color::rgb(150, 150, 150)),
                );
            }
            content = content.child(col_header);
        }

        // Data rows
        for (row_idx, row) in self.data.iter().enumerate() {
            for _ in 0..self.cell_height {
                let mut row_view = hstack();

                // Row label
                if let Some(labels) = &self.row_labels {
                    if let Some(label) = labels.get(row_idx) {
                        let truncated = if label.len() > 6 { &label[..6] } else { label };
                        row_view = row_view.child(
                            Text::new(format!("{:>6} ", truncated)).fg(Color::rgb(150, 150, 150)),
                        );
                    }
                }

                // Cells
                for (col_idx, &value) in row.iter().enumerate() {
                    let color = self.color_for(value);
                    let cell_str = self.render_cell(value);

                    let is_highlighted = self.highlighted == Some((row_idx, col_idx));

                    let mut cell_text = Text::new(&cell_str);

                    if self.show_values {
                        // Show value with colored background
                        cell_text = cell_text.bg(color);
                        // Contrast text color
                        let brightness = (color.r as u32 + color.g as u32 + color.b as u32) / 3;
                        if brightness > 128 {
                            cell_text = cell_text.fg(Color::BLACK);
                        } else {
                            cell_text = cell_text.fg(Color::WHITE);
                        }
                    } else {
                        cell_text = cell_text.fg(color);
                    }

                    if is_highlighted {
                        cell_text = cell_text.bold();
                    }

                    row_view = row_view.child(cell_text);
                }

                content = content.child(row_view);
            }
        }

        // Legend
        if self.show_legend {
            let mut legend = hstack();
            legend = legend.child(Text::new("Low ").fg(Color::rgb(128, 128, 128)));

            for i in 0..10 {
                let v = i as f64 / 9.0;
                let color = self.color_scale.color_at(v);
                legend = legend.child(Text::new("█").fg(color));
            }

            legend = legend.child(Text::new(" High").fg(Color::rgb(128, 128, 128)));
            legend = legend.child(
                Text::new(format!("  ({:.1} - {:.1})", self.min_val, self.max_val))
                    .fg(Color::rgb(100, 100, 100)),
            );

            content = content.child(legend);
        }

        content.render(ctx);
    }
}
