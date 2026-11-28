//! ScatterChart `View`: the shared chart frame around the points

use super::ScatterChart;
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::data::chart::chart_render::{
    fill_background, render_axis_title, render_chart_x_labels, render_chart_y_labels, render_grid,
    render_legend, render_title, y_axis_label_columns, y_label_column, LegendItem,
};
use crate::widget::traits::{RenderContext, View};

impl View for ScatterChart {
    crate::impl_view_meta!("ScatterChart");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 15 || area.height < 8 {
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

        let bounds = self.compute_bounds();
        let (x_min, x_max, y_min, y_max) = bounds;

        // Calculate chart area (relative coordinates): the Y label column
        // fits the widest Y label, at most a third of the area (#1614)
        let x_label_height = 1u16;
        let chart_height = area
            .height
            .saturating_sub(title_offset + x_label_height + 1);
        let y_label_width = y_label_column(
            y_axis_label_columns(chart_height, &self.y_axis, y_min, y_max),
            area.width,
        );

        let chart_area = Rect {
            x: y_label_width,
            y: title_offset,
            width: area.width.saturating_sub(y_label_width + 1),
            height: chart_height,
        };

        if chart_area.width < 5 || chart_area.height < 3 {
            return;
        }

        // Render components using shared functions
        render_grid(
            ctx,
            chart_area,
            &self.grid,
            self.x_axis.ticks,
            self.y_axis.ticks,
        );
        self.render_points(ctx, chart_area, bounds);

        // Render axes: the labels on the rows and columns of the grid and
        // the points (#1622)
        render_chart_y_labels(
            ctx,
            chart_area,
            0,
            &self.y_axis,
            (y_min, y_max),
            y_label_width - 1,
        );
        render_chart_x_labels(
            ctx,
            chart_area,
            area.height - 1,
            (0, area.width),
            &self.x_axis,
            (x_min, x_max),
        );
        render_axis_title(
            ctx,
            rel_area,
            self.x_axis.title.as_deref(),
            self.x_axis.color,
            true,
        );

        // Render legend using shared function
        let legend_items: Vec<LegendItem<'_>> = self
            .series
            .iter()
            .enumerate()
            .map(|(i, s)| LegendItem {
                label: &s.name,
                color: self.series_color(i),
            })
            .collect();
        render_legend(ctx, rel_area, &self.legend, &legend_items);
    }
}
