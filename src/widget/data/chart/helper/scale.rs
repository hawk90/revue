//! Chart value range and scale: data bounds and the value-to-cell mapping

use super::geometry::GridPoint;
use super::Chart;

impl Chart {
    /// Compute data bounds
    ///
    /// Returns (x_min, x_max, y_min, y_max) with safe defaults for edge cases.
    pub(super) fn compute_bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut has_data = false;

        for series in &self.series {
            for &(x, y) in &series.data {
                // Skip NaN and infinite values
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                has_data = true;
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Default bounds for empty data
        if !has_data {
            x_min = self.x_axis.min.unwrap_or(0.0);
            x_max = self.x_axis.max.unwrap_or(1.0);
            y_min = self.y_axis.min.unwrap_or(0.0);
            y_max = self.y_axis.max.unwrap_or(1.0);
        } else {
            // Use axis bounds if specified
            x_min = self.x_axis.min.unwrap_or(x_min);
            x_max = self.x_axis.max.unwrap_or(x_max);
            y_min = self.y_axis.min.unwrap_or(y_min);
            y_max = self.y_axis.max.unwrap_or(y_max);
        }

        // Ensure non-zero ranges (avoid division by zero)
        const EPSILON: f64 = 1e-10;
        if (x_max - x_min).abs() < EPSILON {
            let center = (x_max + x_min) / 2.0;
            x_min = center - 0.5;
            x_max = center + 0.5;
        }
        if (y_max - y_min).abs() < EPSILON {
            let center = (y_max + y_min) / 2.0;
            y_min = center - 0.5;
            y_max = center + 0.5;
        }

        // Add padding for auto bounds
        let y_range = y_max - y_min;
        let y_min = if self.y_axis.min.is_none() {
            y_min - y_range * 0.05
        } else {
            y_min
        };
        let y_max = if self.y_axis.max.is_none() {
            y_max + y_range * 0.05
        } else {
            y_max
        };

        (x_min, x_max, y_min, y_max)
    }

    /// Map a data point onto a `gw` x `gh` grid laid over the plot area.
    ///
    /// Returns fractional offsets: columns from the left edge and rows up from
    /// the bottom edge. Points inside the axis bounds land in
    /// `[0, gw - 1] x [0, gh - 1]`; points outside fall outside that range
    /// (they are clipped by `clip_segment` and `grid_contains`, never
    /// cast to `u16` unchecked).
    pub(super) fn grid_offset(
        x: f64,
        y: f64,
        bounds: (f64, f64, f64, f64),
        gw: u16,
        gh: u16,
    ) -> GridPoint {
        let (x_min, x_max, y_min, y_max) = bounds;
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        let ox = if x_range > 0.0 {
            (x - x_min) / x_range * (gw as f64 - 1.0)
        } else {
            (gw / 2) as f64
        };
        let oy = if y_range > 0.0 {
            (y - y_min) / y_range * (gh as f64 - 1.0)
        } else {
            (gh - 1 - gh / 2) as f64
        };
        (ox, oy)
    }
}
