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
            palette: vec![
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
            ],
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
        if index < self.layers.len() {
            self.layers[index]
                .color
                .unwrap_or_else(|| self.palette[index % self.palette.len()])
        } else {
            self.palette[index % self.palette.len()]
        }
    }

    /// Compute stack positions for all layers
    pub fn compute_stacks(&self) -> Vec<Vec<(f64, f64)>> {
        if self.layers.is_empty() {
            return Vec::new();
        }

        let num_points = self
            .layers
            .iter()
            .map(|l| l.values.len())
            .max()
            .unwrap_or(0);

        if num_points == 0 {
            return Vec::new();
        }

        // Order layers if needed
        let ordered_indices: Vec<usize> = match self.order {
            StreamOrder::None => (0..self.layers.len()).collect(),
            StreamOrder::Ascending => {
                let mut indices: Vec<_> = (0..self.layers.len()).collect();
                indices.sort_by(|&a, &b| {
                    let sum_a: f64 = self.layers[a].values.iter().sum();
                    let sum_b: f64 = self.layers[b].values.iter().sum();
                    sum_a
                        .partial_cmp(&sum_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                indices
            }
            StreamOrder::Descending => {
                let mut indices: Vec<_> = (0..self.layers.len()).collect();
                indices.sort_by(|&a, &b| {
                    let sum_a: f64 = self.layers[a].values.iter().sum();
                    let sum_b: f64 = self.layers[b].values.iter().sum();
                    sum_b
                        .partial_cmp(&sum_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                indices
            }
            StreamOrder::InsideOut => {
                let mut indices: Vec<_> = (0..self.layers.len()).collect();
                indices.sort_by(|&a, &b| {
                    let sum_a: f64 = self.layers[a].values.iter().sum();
                    let sum_b: f64 = self.layers[b].values.iter().sum();
                    sum_b
                        .partial_cmp(&sum_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                let mut result = Vec::with_capacity(indices.len());
                for (i, idx) in indices.into_iter().enumerate() {
                    if i % 2 == 0 {
                        result.push(idx);
                    } else {
                        result.insert(0, idx);
                    }
                }
                result
            }
        };

        let mut stacks: Vec<Vec<(f64, f64)>> = vec![Vec::new(); self.layers.len()];

        for x in 0..num_points {
            let values: Vec<f64> = ordered_indices
                .iter()
                .map(|&i| self.layers[i].values.get(x).copied().unwrap_or(0.0))
                .collect();

            let total: f64 = values.iter().sum();

            let (y0, scale) = match self.baseline {
                StreamBaseline::Zero => (0.0, 1.0),
                StreamBaseline::Symmetric => (-total / 2.0, 1.0),
                StreamBaseline::Wiggle => {
                    let n = self.layers.len() as f64;
                    let offset: f64 = values
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| (n - i as f64) * v)
                        .sum::<f64>()
                        / n;
                    (-offset / 2.0, 1.0)
                }
                StreamBaseline::Expand => {
                    let scale = if total > 0.0 { 1.0 / total } else { 1.0 };
                    (0.0, scale)
                }
            };

            let mut y = y0;
            for (i, &orig_idx) in ordered_indices.iter().enumerate() {
                let value = values[i] * scale;
                let y_start = y;
                let y_end = y + value;
                stacks[orig_idx].push((y_start, y_end));
                y = y_end;
            }
        }

        stacks
    }
}
