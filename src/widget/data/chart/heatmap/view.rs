//! Heat map widget rendering: title, column header, data lines and legend, one line each

use super::HeatMap;
use crate::utils::{display_width, truncate_to_width};
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
use crate::widget::RenderContext;
use crate::widget::View;
use crate::widget::{hstack, vstack, Stack, Text};

/// Width of the row-label column: up to 6 columns of label plus a gap.
const ROW_LABEL_WIDTH: usize = 7;

/// Columns `text` takes, as a stack child size.
pub(super) fn width_of(text: &str) -> u16 {
    display_width(text).min(u16::MAX as usize) as u16
}

impl HeatMap {
    /// Column labels, indented past the row-label column
    fn column_header(&self, labels: &[String]) -> Stack {
        let mut col_header = hstack();
        if self.row_labels.is_some() {
            col_header = col_header.child_sized(
                Text::new(" ".repeat(ROW_LABEL_WIDTH)),
                ROW_LABEL_WIDTH as u16,
            );
        }

        for label in labels.iter().take(self.cols) {
            let truncated = truncate_to_width(label, self.drawn_cell_width());
            let pad = self
                .drawn_cell_width()
                .saturating_sub(display_width(truncated));
            let centered = format!(
                "{}{}{}",
                " ".repeat(pad / 2),
                truncated,
                " ".repeat(pad - pad / 2)
            );
            let width = width_of(&centered);
            col_header = col_header.child_sized(Text::new(centered).fg(LIGHT_GRAY), width);
        }
        col_header
    }

    /// One terminal line of data row `row_idx`: its label and its cells
    fn data_line(&self, row_idx: usize, row: &[f64]) -> Stack {
        let mut row_view = hstack();

        // Row label, right-aligned in its column (blank for rows
        // past the end of the labels, so the cells stay aligned)
        if let Some(labels) = &self.row_labels {
            let label = labels.get(row_idx).map(String::as_str).unwrap_or("");
            let truncated = truncate_to_width(label, ROW_LABEL_WIDTH - 1);
            let pad = (ROW_LABEL_WIDTH - 1).saturating_sub(display_width(truncated));
            let text = format!("{}{} ", " ".repeat(pad), truncated);
            row_view = row_view.child_sized(Text::new(text).fg(LIGHT_GRAY), ROW_LABEL_WIDTH as u16);
        }

        // Cells
        for (col_idx, &value) in row.iter().enumerate() {
            let (cell_text, cell_width) = self.cell_text(row_idx, col_idx, value);
            row_view = row_view.child_sized(cell_text, cell_width);
        }
        row_view
    }

    /// The color scale from low to high, then the value range
    fn legend_line(&self) -> Stack {
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
        legend
    }
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
            content = content.child_sized(self.column_header(labels), 1);
        }

        // Data rows
        for (row_idx, row) in self.data.iter().enumerate() {
            for _ in 0..self.cell_height {
                content = content.child_sized(self.data_line(row_idx, row), 1);
            }
        }

        // Legend
        if self.show_legend {
            content = content.child_sized(self.legend_line(), 1);
        }

        content.render(ctx);
    }
}
