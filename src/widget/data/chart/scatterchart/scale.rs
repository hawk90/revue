//! ScatterChart value range and scale: data bounds and the value-to-cell mapping

use super::ScatterChart;
use crate::layout::Rect;

impl ScatterChart {
    /// Compute data bounds
    pub(super) fn compute_bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for series in &self.series {
            for &(x, y) in &series.data {
                if x.is_finite() {
                    x_min = x_min.min(x);
                    x_max = x_max.max(x);
                }
                if y.is_finite() {
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
            }
        }

        // Apply axis overrides
        let x_min = self.x_axis.min.unwrap_or(x_min);
        let x_max = self.x_axis.max.unwrap_or(x_max);
        let y_min = self.y_axis.min.unwrap_or(y_min);
        let y_max = self.y_axis.max.unwrap_or(y_max);

        // Add padding
        let x_range = (x_max - x_min).max(1.0);
        let y_range = (y_max - y_min).max(1.0);
        let x_padding = x_range * 0.05;
        let y_padding = y_range * 0.05;

        (
            x_min - x_padding,
            x_max + x_padding,
            y_min - y_padding,
            y_max + y_padding,
        )
    }

    /// Map data coordinates to screen coordinates (relative)
    pub(super) fn map_to_screen(
        &self,
        x: f64,
        y: f64,
        bounds: (f64, f64, f64, f64),
        chart_area: Rect,
    ) -> (u16, u16) {
        let (x_min, x_max, y_min, y_max) = bounds;

        let x_range = (x_max - x_min).max(1.0);
        let y_range = (y_max - y_min).max(1.0);

        let screen_x =
            chart_area.x + ((x - x_min) / x_range * (chart_area.width as f64 - 1.0)) as u16;
        let screen_y = chart_area.y + chart_area.height
            - 1
            - ((y - y_min) / y_range * (chart_area.height as f64 - 1.0)) as u16;

        (
            screen_x.clamp(chart_area.x, chart_area.x + chart_area.width - 1),
            screen_y.clamp(chart_area.y, chart_area.y + chart_area.height - 1),
        )
    }
}
