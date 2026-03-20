//! Heat map widget rendering

use super::HeatMap;
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
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
                let truncated = crate::utils::truncate_to_width(label, self.cell_width);
                col_header = col_header.child(
                    Text::new(format!("{:^width$}", truncated, width = self.cell_width))
                        .fg(LIGHT_GRAY),
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
                        let truncated = crate::utils::truncate_to_width(label, 6);
                        row_view = row_view.child(
                            Text::new(format!("{:>6} ", truncated)).fg(LIGHT_GRAY),
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
                        // Contrast text color using perceptual luminance (ITU-R BT.601)
                        let luminance =
                            (299 * color.r as u32 + 587 * color.g as u32 + 114 * color.b as u32)
                                / 1000;
                        if luminance > 128 {
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
            legend = legend.child(Text::new("Low ").fg(PLACEHOLDER_FG));

            for i in 0..10 {
                let v = i as f64 / 9.0;
                let color = self.color_scale.color_at(v);
                legend = legend.child(Text::new("█").fg(color));
            }

            legend = legend.child(Text::new(" High").fg(PLACEHOLDER_FG));
            legend = legend.child(
                Text::new(format!("  ({:.1} - {:.1})", self.min_val, self.max_val)).fg(DISABLED_FG),
            );

            content = content.child(legend);
        }

        content.render(ctx);
    }
}
