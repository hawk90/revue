//! Pie chart labels around the pie

use super::{PieChart, PieLabelStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl PieChart {
    /// Render labels around the pie
    pub(super) fn render_labels(
        &self,
        ctx: &mut RenderContext,
        center_x: u16,
        center_y: u16,
        radius: u16,
    ) {
        if matches!(self.labels, PieLabelStyle::None) {
            return;
        }

        let area = ctx.area;
        let total = self.total();
        if total == 0.0 {
            return;
        }

        let mut current_angle = self.start_angle;

        for slice in &self.slices {
            let slice_angle = self.slice_angle(slice.value);
            let mid_angle = current_angle + slice_angle / 2.0;
            let rad = mid_angle.to_radians();

            // Position label just outside the pie, which is `radius` columns
            // wide and `radius / 2` rows tall from its center
            let label_distance = radius as f64 * 1.3;
            let label_x = center_x as f64 + label_distance * rad.cos();
            let label_y = center_y as f64 + label_distance * rad.sin() / 2.0;

            let label_text = match self.labels {
                PieLabelStyle::None => String::new(),
                PieLabelStyle::Value => format!("{:.1}", slice.value),
                PieLabelStyle::Percent => {
                    format!("{:.0}%", (slice.value / total) * 100.0)
                }
                PieLabelStyle::Label => slice.label.clone(),
                PieLabelStyle::LabelPercent => {
                    format!("{} ({:.0}%)", slice.label, (slice.value / total) * 100.0)
                }
            };

            // Draw label
            let start_x = if mid_angle.cos() < 0.0 {
                (label_x - crate::utils::display_width(&label_text) as f64).max(0.0) as u16
            } else {
                label_x as u16
            };

            let y = label_y as u16;
            if y < area.height {
                ctx.put_str_with(start_x, y, &label_text, area.width, |ch| {
                    Cell::new(ch).fg(Color::WHITE)
                });
            }

            current_angle += slice_angle;
        }
    }
}
