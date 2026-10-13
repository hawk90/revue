//! Pie chart `View`: background, title, pie size, then the slices, labels and legend

use super::PieChart;
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::data::chart::chart_render::{
    fill_background, render_legend, render_title, LegendItem,
};
use crate::widget::traits::{RenderContext, View};

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
