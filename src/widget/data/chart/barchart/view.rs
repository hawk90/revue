//! Bar chart `View`: the value scale both orientations share, and the dispatch

use super::{BarChart, BarOrientation};
use crate::widget::traits::{RenderContext, View};

impl BarChart {
    /// Calculate the maximum absolute value in the data for scaling
    pub(super) fn calculate_max(&self) -> f64 {
        self.max.unwrap_or_else(|| {
            self.bars
                .iter()
                .map(|b| b.value.abs())
                .fold(f64::NEG_INFINITY, f64::max)
                .max(1.0)
        })
    }
}

impl View for BarChart {
    crate::impl_view_meta!("BarChart");

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            BarOrientation::Horizontal => self.render_horizontal(ctx),
            BarOrientation::Vertical => self.render_vertical(ctx),
        }
    }
}
