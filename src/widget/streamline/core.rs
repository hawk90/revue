//! Core Streamline chart implementation

use super::types::{StreamBaseline, StreamLayer, StreamOrder};
use crate::style::Color;
use crate::widget::traits::WidgetProps;

/// Streamline chart widget
#[derive(Debug, Clone)]
pub struct Streamline {
    /// Title
    pub title: Option<String>,
    /// Layers in the stream
    pub layers: Vec<StreamLayer>,
    /// Baseline mode
    pub baseline: StreamBaseline,
    /// Layer ordering
    pub order: StreamOrder,
    /// Show legend
    pub show_legend: bool,
    /// Show labels on streams
    pub show_labels: bool,
    /// X-axis labels
    pub x_labels: Vec<String>,
    /// Background color
    pub bg_color: Option<Color>,
    /// Height
    pub height: Option<u16>,
    /// Color palette
    pub palette: Vec<Color>,
    /// Highlighted layer index
    pub highlighted: Option<usize>,
    /// CSS styling properties (id, classes)
    pub props: WidgetProps,
}

/// Layer colors used when no palette is set, or when it is set empty.
const DEFAULT_PALETTE: [Color; 10] = [
    Color::rgb(66, 133, 244),  // Blue
    Color::rgb(234, 67, 53),   // Red
    Color::rgb(251, 188, 5),   // Yellow
    Color::rgb(52, 168, 83),   // Green
    Color::rgb(155, 89, 182),  // Purple
    Color::rgb(241, 196, 15),  // Gold
    Color::rgb(26, 188, 156),  // Teal
    Color::rgb(230, 126, 34),  // Orange
    Color::rgb(149, 165, 166), // Gray
    Color::rgb(231, 76, 60),   // Coral
];

impl Default for Streamline {
    fn default() -> Self {
        Self::new()
    }
}

impl Streamline {
    /// Create a new streamline chart
    pub fn new() -> Self {
        Self {
            title: None,
            layers: Vec::new(),
            baseline: StreamBaseline::Symmetric,
            order: StreamOrder::None,
            show_legend: true,
            show_labels: true,
            x_labels: Vec::new(),
            bg_color: None,
            height: None,
            palette: DEFAULT_PALETTE.to_vec(),
            highlighted: None,
            props: WidgetProps::new(),
        }
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a layer
    pub fn layer(mut self, layer: StreamLayer) -> Self {
        self.layers.push(layer);
        self
    }

    /// Add multiple layers
    pub fn layers(mut self, layers: impl IntoIterator<Item = StreamLayer>) -> Self {
        self.layers.extend(layers);
        self
    }

    /// Set baseline mode
    pub fn baseline(mut self, mode: StreamBaseline) -> Self {
        self.baseline = mode;
        self
    }

    /// Set layer ordering
    pub fn order(mut self, order: StreamOrder) -> Self {
        self.order = order;
        self
    }

    /// Show or hide legend
    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show or hide stream labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set X-axis labels
    pub fn x_labels(mut self, labels: Vec<String>) -> Self {
        self.x_labels = labels;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Set color palette
    ///
    /// An empty palette falls back to the default colors.
    pub fn palette(mut self, colors: Vec<Color>) -> Self {
        self.palette = colors;
        self
    }

    /// Highlight a specific layer
    pub fn highlight(mut self, index: usize) -> Self {
        self.highlighted = Some(index);
        self
    }

    /// Get the color for a layer at the given index
    pub fn get_layer_color(&self, index: usize) -> Color {
        let palette: &[Color] = if self.palette.is_empty() {
            &DEFAULT_PALETTE
        } else {
            &self.palette
        };
        self.layers
            .get(index)
            .and_then(|layer| layer.color)
            .unwrap_or(palette[index % palette.len()])
    }
}
