//! Histogram `View`: background, title and plot area, then bars, statistics and axes

use super::Histogram;
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::data::chart::chart_common::ChartOrientation;
use crate::widget::data::chart::chart_render::{fill_background, render_title};
use crate::widget::traits::{RenderContext, View};

impl View for Histogram {
    crate::impl_view_meta!("Histogram");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 15 || area.height < 5 {
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

        // Calculate chart area (relative coordinates)
        let y_label_width = 6u16;
        let x_label_height = 1u16;

        let chart_area = Rect {
            x: y_label_width,
            y: title_offset,
            width: area.width.saturating_sub(y_label_width + 1),
            height: area
                .height
                .saturating_sub(title_offset + x_label_height + 1),
        };

        if chart_area.width < 5 || chart_area.height < 3 {
            return;
        }

        // Render components
        if self.orientation == ChartOrientation::Horizontal {
            self.render_bars_horizontal(ctx, chart_area);
            self.render_stats_horizontal(ctx, chart_area);
            self.render_axes_horizontal(ctx, rel_area, chart_area);
        } else {
            self.render_bars(ctx, chart_area);
            self.render_stats(ctx, chart_area);
            self.render_axes(ctx, rel_area);
        }
    }
}
