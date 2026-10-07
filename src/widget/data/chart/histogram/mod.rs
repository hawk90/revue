//! Histogram widget for frequency distribution visualization
//!
//! Supports automatic binning, density normalization, cumulative histograms, and statistics overlay.

mod render;

use super::chart_common::{Axis, ChartGrid, ChartOrientation, Legend};
use super::chart_stats::{self, mean, median};
use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

// Re-export from chart_stats for public API compatibility
pub use super::chart_stats::{BinConfig, HistogramBin};

/// Histogram widget
#[derive(Clone)]
pub struct Histogram {
    /// Raw data values
    data: Vec<f64>,
    /// Computed bins
    bins: Vec<HistogramBin>,
    /// Bin configuration
    bin_config: BinConfig,
    /// Orientation
    orientation: ChartOrientation,
    /// X axis configuration
    x_axis: Axis,
    /// Y axis configuration
    y_axis: Axis,
    /// Legend configuration
    legend: Legend,
    /// Grid configuration
    grid: ChartGrid,
    /// Fill color
    fill_color: Color,
    /// Border color for bars
    bar_border: Option<Color>,
    /// Show cumulative distribution
    cumulative: bool,
    /// Normalize to density
    density: bool,
    /// Show statistics (mean, median)
    show_stats: bool,
    /// Chart title
    title: Option<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Widget properties
    props: WidgetProps,
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl Histogram {
    /// Create a new histogram from data
    pub fn new(data: &[f64]) -> Self {
        let mut hist = Self {
            data: data.to_vec(),
            bins: Vec::new(),
            bin_config: BinConfig::Auto,
            orientation: ChartOrientation::Vertical,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            legend: Legend::none(),
            grid: ChartGrid::new().y(true),
            fill_color: Color::rgb(97, 175, 239),
            bar_border: None,
            cumulative: false,
            density: false,
            show_stats: false,
            title: None,
            bg_color: None,
            props: WidgetProps::new(),
        };
        hist.compute_bins();
        hist
    }

    /// Set data
    pub fn data(mut self, data: &[f64]) -> Self {
        self.data = data.to_vec();
        self.compute_bins();
        self
    }

    /// Set bin configuration
    pub fn bins(mut self, config: BinConfig) -> Self {
        self.bin_config = config;
        self.compute_bins();
        self
    }

    /// Set number of bins
    pub fn bin_count(mut self, count: usize) -> Self {
        self.bin_config = BinConfig::Count(count);
        self.compute_bins();
        self
    }

    /// Set bin width
    pub fn bin_width(mut self, width: f64) -> Self {
        self.bin_config = BinConfig::Width(width);
        self.compute_bins();
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: ChartOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    ///
    /// Bins run top to bottom (lowest first) and bars grow to the right.
    /// The axes keep their meaning: [`x_axis`](Self::x_axis) still describes
    /// the binned values (now labeled down the left side) and
    /// [`y_axis`](Self::y_axis) the counts (now labeled along the bottom).
    pub fn horizontal(mut self) -> Self {
        self.orientation = ChartOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = ChartOrientation::Vertical;
        self
    }

    /// Set X axis configuration
    pub fn x_axis(mut self, axis: Axis) -> Self {
        self.x_axis = axis;
        self
    }

    /// Set Y axis configuration
    pub fn y_axis(mut self, axis: Axis) -> Self {
        self.y_axis = axis;
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

    /// Set fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Alias for fill_color
    pub fn color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Set bar border color
    pub fn bar_border(mut self, color: Color) -> Self {
        self.bar_border = Some(color);
        self
    }

    /// Enable cumulative distribution
    pub fn cumulative(mut self, enabled: bool) -> Self {
        self.cumulative = enabled;
        self
    }

    /// Enable density normalization
    pub fn density(mut self, enabled: bool) -> Self {
        self.density = enabled;
        self
    }

    /// Show statistics (mean, median lines)
    pub fn show_stats(mut self, enabled: bool) -> Self {
        self.show_stats = enabled;
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

    /// Compute bins from data using shared stats module
    fn compute_bins(&mut self) {
        self.bins = chart_stats::compute_bins(&self.data, &self.bin_config);
    }

    /// Get the mean of the data
    pub fn mean(&self) -> Option<f64> {
        mean(&self.data)
    }

    /// Get the median of the data
    pub fn median(&self) -> Option<f64> {
        median(&self.data)
    }
}

impl_styled_view!(Histogram);
impl_props_builders!(Histogram);

/// Create a new histogram
pub fn histogram(data: &[f64]) -> Histogram {
    Histogram::new(data)
}
