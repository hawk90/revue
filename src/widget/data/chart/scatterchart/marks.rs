//! ScatterChart marks: points, markers and bubbles

use super::ScatterChart;
use crate::layout::Rect;
use crate::render::Cell;
use crate::widget::traits::RenderContext;

impl ScatterChart {
    /// Render data points
    pub(super) fn render_points(
        &self,
        ctx: &mut RenderContext,
        chart_area: Rect,
        bounds: (f64, f64, f64, f64),
    ) {
        for (series_idx, series) in self.series.iter().enumerate() {
            let color = self.series_color(series_idx);
            let marker_char = series.marker.char();

            for (point_idx, &(x, y)) in series.data.iter().enumerate() {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }

                let (screen_x, screen_y) = self.map_to_screen(x, y, bounds, chart_area);

                // For bubble chart, draw larger markers based on size
                if let Some(ref sizes) = series.sizes {
                    if let Some(&size) = sizes.get(point_idx) {
                        // Draw a circle proportional to size
                        let radius = ((size / 100.0).sqrt() * 2.0).max(1.0) as u16;
                        for dy in 0..radius {
                            for dx in 0..radius {
                                let bx = screen_x.saturating_sub(radius / 2) + dx;
                                let by = screen_y.saturating_sub(radius / 2) + dy;
                                if bx >= chart_area.x
                                    && bx < chart_area.x + chart_area.width
                                    && by >= chart_area.y
                                    && by < chart_area.y + chart_area.height
                                {
                                    let mut cell = Cell::new('●');
                                    cell.fg = Some(color);
                                    ctx.set(bx, by, cell);
                                }
                            }
                        }
                        continue;
                    }
                }

                // Regular marker
                if screen_x >= chart_area.x
                    && screen_x < chart_area.x + chart_area.width
                    && screen_y >= chart_area.y
                    && screen_y < chart_area.y + chart_area.height
                {
                    let mut cell = Cell::new(marker_char);
                    cell.fg = Some(color);
                    ctx.set(screen_x, screen_y, cell);
                }
            }
        }
    }
}
