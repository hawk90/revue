//! Pie Chart widget for proportional data visualization
//!
//! Supports standard pie charts, donut charts, labels, legends, and exploded segments.

mod render;
mod types;

pub use types::{PieLabelStyle, PieSlice, PieStyle};

use super::chart_common::{ColorScheme, Legend, LegendPosition};
use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Pie chart widget
#[derive(Clone)]
pub struct PieChart {
    /// Pie slices
    slices: Vec<PieSlice>,
    /// Chart style (pie or donut)
    style: PieStyle,
    /// Legend configuration
    legend: Legend,
    /// Color palette
    colors: ColorScheme,
    /// Start angle in degrees (-90 = top)
    start_angle: f64,
    /// Index of exploded slice
    explode: Option<usize>,
    /// Explode distance (0.0-1.0)
    explode_distance: f64,
    /// Label style
    labels: PieLabelStyle,
    /// Donut hole ratio (0.0-1.0)
    donut_ratio: f64,
    /// Chart title
    title: Option<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Widget properties
    props: WidgetProps,
}

impl Default for PieChart {
    fn default() -> Self {
        Self::new()
    }
}

impl PieChart {
    /// Create a new pie chart
    pub fn new() -> Self {
        Self {
            slices: Vec::new(),
            style: PieStyle::Pie,
            legend: Legend::new().position(LegendPosition::TopRight),
            colors: ColorScheme::default_palette(),
            start_angle: -90.0, // Start from top
            explode: None,
            explode_distance: 0.15,
            labels: PieLabelStyle::None,
            donut_ratio: 0.0,
            title: None,
            bg_color: None,
            props: WidgetProps::new(),
        }
    }

    /// Add a slice
    pub fn slice(mut self, label: impl Into<String>, value: f64) -> Self {
        self.slices.push(PieSlice::new(label, value));
        self
    }

    /// Add a colored slice
    pub fn slice_colored(mut self, label: impl Into<String>, value: f64, color: Color) -> Self {
        self.slices.push(PieSlice::with_color(label, value, color));
        self
    }

    /// Add multiple slices from iterator
    pub fn slices<I, S>(mut self, slices: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        for (label, value) in slices {
            self.slices.push(PieSlice::new(label, value));
        }
        self
    }

    /// Set chart style (pie or donut)
    pub fn style(mut self, style: PieStyle) -> Self {
        self.style = style;
        if style == PieStyle::Donut && self.donut_ratio == 0.0 {
            self.donut_ratio = 0.5;
        }
        self
    }

    /// Make it a donut chart with specified hole ratio
    pub fn donut(mut self, ratio: f64) -> Self {
        self.style = PieStyle::Donut;
        self.donut_ratio = ratio.clamp(0.0, 0.9);
        self
    }

    /// Set legend configuration
    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = legend;
        self
    }

    /// Hide the legend
    pub fn no_legend(mut self) -> Self {
        self.legend = Legend::none();
        self
    }

    /// Set color scheme
    pub fn colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    /// Set start angle in degrees (-90 = top, 0 = right)
    pub fn start_angle(mut self, angle: f64) -> Self {
        self.start_angle = angle;
        self
    }

    /// Explode a slice (pull it out)
    pub fn explode(mut self, index: usize) -> Self {
        self.explode = Some(index);
        self
    }

    /// Set explode distance
    pub fn explode_distance(mut self, distance: f64) -> Self {
        self.explode_distance = distance.clamp(0.0, 0.5);
        self
    }

    /// Set label style
    pub fn labels(mut self, style: PieLabelStyle) -> Self {
        self.labels = style;
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

impl_styled_view!(PieChart);
impl_props_builders!(PieChart);

/// Create a new pie chart
pub fn pie_chart() -> PieChart {
    PieChart::new()
}

/// Create a donut chart
pub fn donut_chart() -> PieChart {
    PieChart::new().donut(0.5)
}
