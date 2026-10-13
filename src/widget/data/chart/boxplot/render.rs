//! Box plot drawing state: the groups, bounds and plot area shared by the boxes and axes

use super::group::BoxGroup;
use super::stats::WhiskerStyle;
use crate::layout::Rect;
use crate::style::Color;

/// Box plot rendering state
pub struct BoxPlotRender<'a> {
    /// Box groups
    pub groups: &'a [BoxGroup],
    /// Value to screen coordinate mapping state
    pub bounds: (f64, f64),
    pub chart_area: Rect,
    pub box_width: f64,
    pub whisker_style: WhiskerStyle,
    pub show_outliers: bool,
    /// Pinch each box in around the median's confidence interval
    pub notched: bool,
    pub group_count: usize,
}

impl<'a> BoxPlotRender<'a> {
    /// Create new render state
    pub fn new(
        groups: &'a [BoxGroup],
        bounds: (f64, f64),
        chart_area: Rect,
        box_width: f64,
        whisker_style: WhiskerStyle,
        show_outliers: bool,
        notched: bool,
    ) -> Self {
        Self {
            groups,
            bounds,
            chart_area,
            box_width,
            whisker_style,
            show_outliers,
            notched,
            group_count: groups.len(),
        }
    }

    /// Get color for group at index
    pub fn group_color(
        &self,
        index: usize,
        colors: &crate::widget::data::chart::chart_common::ColorScheme,
    ) -> Color {
        self.groups
            .get(index)
            .and_then(|g| g.color)
            .unwrap_or_else(|| colors.get(index))
    }
}
