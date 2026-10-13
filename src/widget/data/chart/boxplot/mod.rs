//! Box Plot widget for statistical distribution visualization
//!
//! Displays median, quartiles, and outliers for comparing distributions across categories.

use super::chart_common::{Axis, ChartGrid, ChartOrientation, ColorScheme, Legend};
use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

mod axes;
mod boxes;
mod group;
mod render;
mod scale;
mod stats;
mod view;

pub use group::BoxGroup;
pub use stats::{BoxStats, WhiskerStyle};

/// Box plot widget
#[derive(Clone)]
pub struct BoxPlot {
    /// Box groups
    groups: Vec<BoxGroup>,
    /// Orientation
    orientation: ChartOrientation,
    /// Value axis configuration
    value_axis: Axis,
    /// Category axis configuration
    category_axis: Axis,
    /// Legend configuration
    legend: Legend,
    /// Grid configuration
    grid: ChartGrid,
    /// Color palette
    colors: ColorScheme,
    /// Show outliers
    show_outliers: bool,
    /// Notched box plot
    notched: bool,
    /// Whisker calculation style
    whisker_style: WhiskerStyle,
    /// Box width (0.0-1.0)
    box_width: f64,
    /// Chart title
    title: Option<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Widget properties
    props: WidgetProps,
}

impl Default for BoxPlot {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxPlot {
    /// Create a new box plot
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            orientation: ChartOrientation::Vertical,
            value_axis: Axis::default(),
            category_axis: Axis::default(),
            legend: Legend::none(),
            grid: ChartGrid::new().y(true),
            colors: ColorScheme::default_palette(),
            show_outliers: true,
            notched: false,
            whisker_style: WhiskerStyle::IQR,
            box_width: 0.6,
            title: None,
            bg_color: None,
            props: WidgetProps::new(),
        }
    }

    /// Add a group from raw data
    pub fn group(mut self, label: impl Into<String>, data: &[f64]) -> Self {
        self.groups.push(BoxGroup::new(label, data));
        self
    }

    /// Add a group with pre-computed stats
    pub fn group_stats(mut self, label: impl Into<String>, stats: BoxStats) -> Self {
        self.groups.push(BoxGroup::from_stats(label, stats));
        self
    }

    /// Add a box group
    pub fn add_group(mut self, group: BoxGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: ChartOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = ChartOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = ChartOrientation::Vertical;
        self
    }

    /// Set value axis configuration
    pub fn value_axis(mut self, axis: Axis) -> Self {
        self.value_axis = axis;
        self
    }

    /// Set category axis configuration
    pub fn category_axis(mut self, axis: Axis) -> Self {
        self.category_axis = axis;
        self
    }

    /// Set legend configuration
    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = legend;
        self
    }

    /// Set grid configuration
    pub fn grid(mut self, grid: ChartGrid) -> Self {
        self.grid = grid;
        self
    }

    /// Set color palette
    pub fn colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    /// Show/hide outliers
    pub fn show_outliers(mut self, show: bool) -> Self {
        self.show_outliers = show;
        self
    }

    /// Enable notched box plot
    ///
    /// A notched box pinches in around the median over its approximate 95%
    /// confidence interval (`median ± 1.57 * IQR / sqrt(n)`); boxes whose
    /// notches don't overlap have medians that differ significantly.
    ///
    /// The interval needs the sample size, so only groups added from raw
    /// data ([`group`](Self::group), [`BoxGroup::new`]) are notched; groups
    /// built from precomputed [`BoxStats`] are drawn plain. A notch that does
    /// not fit inside the box at the rendered size is also left out.
    pub fn notched(mut self, enabled: bool) -> Self {
        self.notched = enabled;
        self
    }

    /// Set whisker style
    pub fn whisker_style(mut self, style: WhiskerStyle) -> Self {
        self.whisker_style = style;
        self
    }

    /// Set box width (0.0-1.0)
    pub fn box_width(mut self, width: f64) -> Self {
        self.box_width = width.clamp(0.1, 1.0);
        self
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }
}

impl_styled_view!(BoxPlot);
impl_props_builders!(BoxPlot);

/// Create a new box plot
pub fn boxplot() -> BoxPlot {
    BoxPlot::new()
}
