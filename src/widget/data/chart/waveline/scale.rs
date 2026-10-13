//! Sampling the data: the value at a column (by the interpolation method)
//! and the color at a height (by the gradient)

use super::{Interpolation, Waveline};
use crate::style::Color;

impl Waveline {
    pub(super) fn get_color(&self, ratio: f64) -> Color {
        if let Some(end) = self.gradient_color {
            let r = (self.color.r as f64 * (1.0 - ratio) + end.r as f64 * ratio).round() as u8;
            let g = (self.color.g as f64 * (1.0 - ratio) + end.g as f64 * ratio).round() as u8;
            let b = (self.color.b as f64 * (1.0 - ratio) + end.b as f64 * ratio).round() as u8;
            Color::rgb(r, g, b)
        } else {
            self.color
        }
    }

    pub(super) fn get_interpolated_value(&self, data: &[f64], x: usize, width: usize) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let ratio = x as f64 / (width - 1).max(1) as f64;
        let idx = ratio * (data.len() - 1) as f64;
        let idx_floor = idx.floor() as usize;
        let idx_ceil = (idx_floor + 1).min(data.len() - 1);
        let t = idx - idx_floor as f64;

        match self.interpolation {
            Interpolation::Linear => data[idx_floor] * (1.0 - t) + data[idx_ceil] * t,
            Interpolation::Step => data[idx_floor],
            Interpolation::Bezier | Interpolation::CatmullRom => {
                let p0_idx = idx_floor.saturating_sub(1);
                let p3_idx = (idx_ceil + 1).min(data.len() - 1);

                let p0 = data[p0_idx];
                let p1 = data[idx_floor];
                let p2 = data[idx_ceil];
                let p3 = data[p3_idx];

                let t2 = t * t;
                let t3 = t2 * t;

                0.5 * ((2.0 * p1)
                    + (-p0 + p2) * t
                    + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
                    + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
            }
        }
    }
}
