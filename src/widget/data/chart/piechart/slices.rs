//! Pie chart slices: each slice's share, angle and color, and the filled pie

use super::{PieChart, PieStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl PieChart {
    /// Get total of all slice values
    pub(super) fn total(&self) -> f64 {
        self.slices.iter().map(|s| s.value).sum()
    }

    /// Get color for slice at index
    pub(super) fn slice_color(&self, index: usize) -> Color {
        self.slices
            .get(index)
            .and_then(|s| s.color)
            .unwrap_or_else(|| self.colors.get(index))
    }

    /// Calculate angle for a slice
    pub(super) fn slice_angle(&self, value: f64) -> f64 {
        let total = self.total();
        if total == 0.0 {
            0.0
        } else {
            (value / total) * 360.0
        }
    }

    /// Render the pie chart using simple ASCII/Unicode
    pub(super) fn render_pie(
        &self,
        ctx: &mut RenderContext,
        center_x: u16,
        center_y: u16,
        radius: u16,
    ) {
        let total = self.total();
        if total == 0.0 || self.slices.is_empty() {
            return;
        }

        let area = ctx.area;

        // Aspect ratio correction for terminal characters (typically 2:1)
        let aspect_ratio = 2.0;

        // Draw the pie using polar coordinates
        let mut current_angle = self.start_angle;

        for (slice_idx, slice) in self.slices.iter().enumerate() {
            let slice_angle = self.slice_angle(slice.value);
            let color = self.slice_color(slice_idx);

            // Calculate explode offset if this slice is exploded
            let (offset_x, offset_y) = if self.explode == Some(slice_idx) {
                let mid_angle = current_angle + slice_angle / 2.0;
                let rad = mid_angle.to_radians();
                let offset = self.explode_distance * radius as f64;
                (
                    (offset * rad.cos() * aspect_ratio) as i16,
                    (offset * rad.sin()) as i16,
                )
            } else {
                (0, 0)
            };

            // Draw filled slice
            for y in 0..=(radius * 2) {
                for x in 0..=(radius * 2) {
                    let dx = x as f64 - radius as f64;
                    let dy = (y as f64 - radius as f64) * aspect_ratio;

                    // Check if point is within the slice
                    let distance = (dx * dx + dy * dy).sqrt();
                    let inner_radius = if self.style == PieStyle::Donut {
                        radius as f64 * self.donut_ratio
                    } else {
                        0.0
                    };

                    if distance > radius as f64 || distance < inner_radius {
                        continue;
                    }

                    // Calculate angle of this point
                    let point_angle = dy.atan2(dx).to_degrees();
                    let point_angle = ((point_angle - self.start_angle) % 360.0 + 360.0) % 360.0;

                    // Check if within slice
                    let slice_start = ((current_angle - self.start_angle) % 360.0 + 360.0) % 360.0;
                    let slice_end = slice_start + slice_angle;

                    let in_slice = if slice_end <= 360.0 {
                        point_angle >= slice_start && point_angle < slice_end
                    } else {
                        point_angle >= slice_start || point_angle < (slice_end - 360.0)
                    };

                    if in_slice {
                        let screen_x =
                            (center_x as i16 + offset_x + x as i16 - radius as i16) as u16;
                        // `dy` already doubled the vertical distance, so each
                        // step of `y` is one terminal row.
                        let screen_y =
                            (center_y as i16 + offset_y + (y as i16 - radius as i16)) as u16;

                        if screen_x < area.width && screen_y < area.height {
                            let mut cell = Cell::new('█');
                            cell.fg = Some(color);
                            ctx.set(screen_x, screen_y, cell);
                        }
                    }
                }
            }

            current_angle += slice_angle;
        }
    }
}
