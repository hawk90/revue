//! Heat map widget rendering

use super::HeatMap;
use crate::style::Color;
use crate::utils::{display_width, truncate_to_width};
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
use crate::widget::RenderContext;
use crate::widget::View;
use crate::widget::{hstack, vstack, Text};

/// Width of the row-label column: up to 6 columns of label plus a gap.
const ROW_LABEL_WIDTH: usize = 7;

/// Columns `text` takes, as a stack child size.
fn width_of(text: &str) -> u16 {
    display_width(text).min(u16::MAX as usize) as u16
}

impl View for HeatMap {
    crate::impl_view_meta!("HeatMap");

    fn render(&self, ctx: &mut RenderContext) {
        // Every line is one row tall and every piece of a line exactly as
        // wide as its text, so the column header, row labels and cells line
        // up instead of being spread over equal shares of the area.
        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child_sized(Text::new(title).bold(), 1);
        }

        // Column labels, indented past the row-label column
        if let Some(labels) = &self.col_labels {
            let mut col_header = hstack();
            if self.row_labels.is_some() {
                col_header = col_header.child_sized(
                    Text::new(" ".repeat(ROW_LABEL_WIDTH)),
                    ROW_LABEL_WIDTH as u16,
                );
            }

            for label in labels.iter().take(self.cols) {
                let truncated = truncate_to_width(label, self.cell_width);
                let pad = self.cell_width.saturating_sub(display_width(truncated));
                let centered = format!(
                    "{}{}{}",
                    " ".repeat(pad / 2),
                    truncated,
                    " ".repeat(pad - pad / 2)
                );
                let width = width_of(&centered);
                col_header = col_header.child_sized(Text::new(centered).fg(LIGHT_GRAY), width);
            }
            content = content.child_sized(col_header, 1);
        }

        // Data rows
        for (row_idx, row) in self.data.iter().enumerate() {
            for _ in 0..self.cell_height {
                let mut row_view = hstack();

                // Row label, right-aligned in its column (blank for rows
                // past the end of the labels, so the cells stay aligned)
                if let Some(labels) = &self.row_labels {
                    let label = labels.get(row_idx).map(String::as_str).unwrap_or("");
                    let truncated = truncate_to_width(label, ROW_LABEL_WIDTH - 1);
                    let pad = (ROW_LABEL_WIDTH - 1).saturating_sub(display_width(truncated));
                    let text = format!("{}{} ", " ".repeat(pad), truncated);
                    row_view = row_view
                        .child_sized(Text::new(text).fg(LIGHT_GRAY), ROW_LABEL_WIDTH as u16);
                }

                // Cells
                for (col_idx, &value) in row.iter().enumerate() {
                    let color = self.color_for(value);
                    let cell_str = self.render_cell(value);

                    let is_highlighted = self.highlighted == Some((row_idx, col_idx));

                    let cell_width = width_of(&cell_str);
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

                    row_view = row_view.child_sized(cell_text, cell_width);
                }

                content = content.child_sized(row_view, 1);
            }
        }

        // Legend
        if self.show_legend {
            let mut legend = hstack();
            legend = legend.child_sized(Text::new("Low ").fg(PLACEHOLDER_FG), 4);

            for i in 0..10 {
                let v = i as f64 / 9.0;
                let color = self.color_scale.color_at(v);
                legend = legend.child_sized(Text::new("█").fg(color), 1);
            }

            legend = legend.child_sized(Text::new(" High").fg(PLACEHOLDER_FG), 5);
            let range = format!("  ({:.1} - {:.1})", self.min_val, self.max_val);
            let range_width = width_of(&range);
            legend = legend.child_sized(Text::new(range).fg(DISABLED_FG), range_width);

            content = content.child_sized(legend, 1);
        }

        content.render(ctx);
    }
}
