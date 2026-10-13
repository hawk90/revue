//! Box plot `View`: background, title and plot area, then the boxes and axes

use super::{render, BoxPlot};
use crate::layout::Rect;
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::data::chart::chart_common::ChartOrientation;
use crate::widget::data::chart::chart_render::{fill_background, render_title};
use crate::widget::traits::{RenderContext, View};

impl View for BoxPlot {
    crate::impl_view_meta!("BoxPlot");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 10 || area.height < 8 {
            return;
        }

        // Use relative area (0,0 origin) for shared functions that use ctx.set()
        let rel_area = Rect::new(0, 0, area.width, area.height);

        // Fill background using shared function
        if let Some(bg) = self.bg_color {
            fill_background(ctx, rel_area, bg);
        }

        // Draw title using shared function
        let title_offset = render_title(ctx, rel_area, self.title.as_deref(), Color::WHITE);

        let horizontal = self.orientation == ChartOrientation::Horizontal;

        // Calculate chart area (relative coordinates)
        let chart_area = if horizontal {
            // Group labels on the left (as wide as the longest label, up to a
            // third of the width), value labels on the bottom row
            let label_width = self
                .groups
                .iter()
                .map(|g| display_width(&g.label) as u16)
                .max()
                .unwrap_or(0)
                .clamp(1, area.width / 3);
            let x = label_width + 1;
            Rect {
                x,
                y: title_offset,
                width: area.width.saturating_sub(x + 1),
                height: area.height.saturating_sub(title_offset + 1),
            }
        } else {
            let y_label_width = 6u16;
            let x_label_height = 1u16;
            Rect {
                x: y_label_width,
                y: title_offset,
                width: area.width.saturating_sub(y_label_width + 1),
                height: area
                    .height
                    .saturating_sub(title_offset + x_label_height + 1),
            }
        };

        if chart_area.width < 5 || chart_area.height < 5 {
            return;
        }

        let bounds = self.compute_bounds();

        // Render using render module
        let render_state = render::BoxPlotRender::new(
            &self.groups,
            bounds,
            chart_area,
            self.box_width,
            self.whisker_style,
            self.show_outliers,
            self.notched,
        );

        if horizontal {
            render_state.render_boxes_horizontal(ctx, &self.colors);
            render_state.render_axes_horizontal(
                ctx,
                rel_area,
                &self.value_axis,
                &self.category_axis,
            );
        } else {
            render_state.render_boxes(ctx, &self.colors);
            render_state.render_axes(ctx, rel_area, &self.value_axis, &self.category_axis);
        }
    }
}
