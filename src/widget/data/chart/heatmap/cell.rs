//! Heat map cells: a value's position in the bounds, its color, its text and its styled cell

use super::view::width_of;
use super::{CellDisplay, ColorScale, HeatMap};
use crate::style::Color;
use crate::widget::Text;

impl HeatMap {
    /// Normalize value to 0.0-1.0
    pub fn normalize(&self, value: f64) -> f64 {
        if self.max_val == self.min_val {
            0.5
        } else {
            (value - self.min_val) / (self.max_val - self.min_val)
        }
    }

    /// Get color for value
    pub fn color_for(&self, value: f64) -> Color {
        let normalized = self.normalize(value);

        if let (ColorScale::Custom, Some((low, high))) = (self.color_scale, &self.custom_colors) {
            // Linear interpolation
            Color::rgb(
                (low.r as f64 + normalized * (high.r as f64 - low.r as f64)) as u8,
                (low.g as f64 + normalized * (high.g as f64 - low.g as f64)) as u8,
                (low.b as f64 + normalized * (high.b as f64 - low.b as f64)) as u8,
            )
        } else {
            self.color_scale.color_at(normalized)
        }
    }

    /// Render cell content
    pub fn render_cell(&self, value: f64) -> String {
        if self.show_values {
            format!(
                "{:>width$.prec$}",
                value,
                width = self.drawn_cell_width(),
                prec = self.value_decimals
            )
        } else {
            match self.cell_display {
                CellDisplay::Block => "█".repeat(self.drawn_cell_width()),
                CellDisplay::HalfBlock => "▀".repeat(self.drawn_cell_width()),
                CellDisplay::Value => {
                    format!(
                        "{:>width$.prec$}",
                        value,
                        width = self.drawn_cell_width(),
                        prec = self.value_decimals
                    )
                }
                CellDisplay::Custom => "■".repeat(self.drawn_cell_width()),
            }
        }
    }

    /// The cell for `value` at (`row_idx`, `col_idx`) and its width: colored
    /// text, or the value on a colored background, bold when highlighted
    pub(super) fn cell_text(&self, row_idx: usize, col_idx: usize, value: f64) -> (Text, u16) {
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
                (299 * color.r as u32 + 587 * color.g as u32 + 114 * color.b as u32) / 1000;
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

        (cell_text, cell_width)
    }
}
