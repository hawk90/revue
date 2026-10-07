//! Pie chart rendering: slice geometry, labels, legend and the `View` impl

use super::{PieChart, PieLabelStyle, PieStyle};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::data::chart::chart_render::{
    fill_background, render_legend, render_title, LegendItem,
};
use crate::widget::traits::{RenderContext, View};

impl PieChart {
    /// Get total of all slice values
    fn total(&self) -> f64 {
        self.slices.iter().map(|s| s.value).sum()
    }

    /// Get color for slice at index
    fn slice_color(&self, index: usize) -> Color {
        self.slices
            .get(index)
            .and_then(|s| s.color)
            .unwrap_or_else(|| self.colors.get(index))
    }

    /// Calculate angle for a slice
    fn slice_angle(&self, value: f64) -> f64 {
        let total = self.total();
        if total == 0.0 {
            0.0
        } else {
            (value / total) * 360.0
        }
    }

    /// Render the pie chart using simple ASCII/Unicode
    fn render_pie(&self, ctx: &mut RenderContext, center_x: u16, center_y: u16, radius: u16) {
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

    /// Render labels around the pie
    fn render_labels(&self, ctx: &mut RenderContext, center_x: u16, center_y: u16, radius: u16) {
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

impl View for PieChart {
    crate::impl_view_meta!("PieChart");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 3 || area.height < 3 {
            return;
        }

        // Use relative area (0,0 origin) for shared functions that use ctx.set()
        let rel_area = Rect::new(0, 0, area.width, area.height);

        // Fill background if set
        if let Some(bg) = self.bg_color {
            fill_background(ctx, rel_area, bg);
        }

        // Draw title using shared function
        let title_offset = render_title(ctx, rel_area, self.title.as_deref(), Color::WHITE);

        // Calculate pie center and radius (relative coordinates)
        let chart_area_height = area.height.saturating_sub(title_offset);
        let radius = (chart_area_height.min(area.width / 2))
            .saturating_sub(2)
            .max(1);
        let center_x = area.width / 2;
        let center_y = title_offset + chart_area_height / 2;

        // Render pie
        self.render_pie(ctx, center_x, center_y, radius);

        // Render labels
        self.render_labels(ctx, center_x, center_y, radius);

        // Render legend using shared function
        let legend_items: Vec<LegendItem<'_>> = self
            .slices
            .iter()
            .enumerate()
            .map(|(i, s)| LegendItem {
                label: &s.label,
                color: self.slice_color(i),
            })
            .collect();
        render_legend(ctx, rel_area, &self.legend, &legend_items);
    }
}
