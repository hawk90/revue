//! Bar chart widget for data visualization
//!
//! Displays data as horizontal or vertical bars.

mod bar;
mod horizontal;
mod vertical;
mod view;

pub use bar::{Bar, BarOrientation};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// A bar chart widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let chart = BarChart::new()
///     .bar("Sales", 150.0)
///     .bar("Revenue", 200.0)
///     .bar("Profit", 75.0)
///     .max(250.0)
///     .bar_width(2)
///     .fg(Color::CYAN);
/// ```
#[derive(Clone)]
pub struct BarChart {
    bars: Vec<Bar>,
    orientation: BarOrientation,
    max: Option<f64>,
    bar_width: u16,
    gap: u16,
    show_values: bool,
    fg: Color,
    label_width: Option<u16>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl BarChart {
    /// Create a new bar chart
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            orientation: BarOrientation::default(),
            max: None,
            bar_width: 1,
            gap: 1,
            show_values: true,
            fg: Color::CYAN,
            label_width: None,
            props: WidgetProps::new(),
        }
    }

    /// Add a bar to the chart
    pub fn bar(mut self, label: impl Into<String>, value: f64) -> Self {
        self.bars.push(Bar::new(label, value));
        self
    }

    /// Add a bar with a specific color
    pub fn bar_colored(mut self, label: impl Into<String>, value: f64, color: Color) -> Self {
        self.bars.push(Bar::new(label, value).color(color));
        self
    }

    /// Add multiple bars from data
    pub fn data<I, S>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        for (label, value) in data {
            self.bars.push(Bar::new(label, value));
        }
        self
    }

    /// Set bar orientation
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set to horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = BarOrientation::Horizontal;
        self
    }

    /// Set to vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = BarOrientation::Vertical;
        self
    }

    /// Set maximum value (auto-calculated if not set)
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set bar width (thickness)
    pub fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = width.max(1);
        self
    }

    /// Set gap between bars
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Show or hide values
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set default foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set fixed label width
    pub fn label_width(mut self, width: u16) -> Self {
        self.label_width = Some(width);
        self
    }
}

impl Default for BarChart {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(BarChart);
impl_props_builders!(BarChart);

/// Create a bar chart
pub fn barchart() -> BarChart {
    BarChart::new()
}
