//! Streamline Chart Widget (Stream Graph / ThemeRiver)
//!
//! A stacked area chart with smooth, flowing layers that shows how multiple
//! categories contribute to a total over time. Also known as stream graph
//! or ThemeRiver visualization.
//!
//! # Features
//!
//! - Multiple stacked layers with smooth transitions
//! - Various baseline modes (zero, symmetric, weighted)
//! - Automatic color assignment for layers
//! - Labels for each stream
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{streamline, StreamLayer};
//!
//! let chart = streamline()
//!     .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0, 25.0]))
//!     .layer(StreamLayer::new("Marketing").data(vec![5.0, 8.0, 12.0, 10.0]))
//!     .baseline(StreamBaseline::Symmetric);
//! ```

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_styled_view, impl_props_builders};

/// A single layer in the stream graph
#[derive(Debug, Clone)]
pub struct StreamLayer {
    /// Name of this layer
    pub name: String,
    /// Data values for each time point
    pub values: Vec<f64>,
    /// Layer color (auto-assigned if None)
    pub color: Option<Color>,
}

impl StreamLayer {
    /// Create a new stream layer
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: Vec::new(),
            color: None,
        }
    }

    /// Set data values
    pub fn data(mut self, values: Vec<f64>) -> Self {
        self.values = values;
        self
    }

    /// Set layer color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// Baseline calculation mode for the stream graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamBaseline {
    /// Stack from zero (traditional stacked area)
    Zero,
    /// Symmetric around center (classic stream graph)
    #[default]
    Symmetric,
    /// Weighted wiggle minimization
    Wiggle,
    /// Expand to fill height (100% stacked)
    Expand,
}

/// Stream sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamOrder {
    /// Keep original order
    #[default]
    None,
    /// Sort by total value (ascending)
    Ascending,
    /// Sort by total value (descending)
    Descending,
    /// Inside-out ordering (largest in middle)
    InsideOut,
}

/// Streamline chart widget
#[derive(Debug, Clone)]
pub struct Streamline {
    /// Title
    title: Option<String>,
    /// Layers in the stream
    layers: Vec<StreamLayer>,
    /// Baseline mode
    baseline: StreamBaseline,
    /// Layer ordering
    order: StreamOrder,
    /// Show legend
    show_legend: bool,
    /// Show labels on streams
    show_labels: bool,
    /// X-axis labels
    x_labels: Vec<String>,
    /// Background color
    bg_color: Option<Color>,
    /// Height
    height: Option<u16>,
    /// Color palette
    palette: Vec<Color>,
    /// Highlighted layer index
    highlighted: Option<usize>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
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
                Color::rgb(66, 133, 244),   // Blue
                Color::rgb(234, 67, 53),    // Red
                Color::rgb(251, 188, 5),    // Yellow
                Color::rgb(52, 168, 83),    // Green
                Color::rgb(155, 89, 182),   // Purple
                Color::rgb(241, 196, 15),   // Gold
                Color::rgb(26, 188, 156),   // Teal
                Color::rgb(230, 126, 34),   // Orange
                Color::rgb(149, 165, 166),  // Gray
                Color::rgb(231, 76, 60),    // Coral
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

    fn get_layer_color(&self, index: usize) -> Color {
        self.layers[index].color.unwrap_or_else(|| {
            self.palette[index % self.palette.len()]
        })
    }

    fn compute_stacks(&self) -> Vec<Vec<(f64, f64)>> {
        if self.layers.is_empty() {
            return Vec::new();
        }

        let num_points = self.layers.iter()
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
                    sum_a.partial_cmp(&sum_b).unwrap_or(std::cmp::Ordering::Equal)
                });
                indices
            }
            StreamOrder::Descending => {
                let mut indices: Vec<_> = (0..self.layers.len()).collect();
                indices.sort_by(|&a, &b| {
                    let sum_a: f64 = self.layers[a].values.iter().sum();
                    let sum_b: f64 = self.layers[b].values.iter().sum();
                    sum_b.partial_cmp(&sum_a).unwrap_or(std::cmp::Ordering::Equal)
                });
                indices
            }
            StreamOrder::InsideOut => {
                let mut indices: Vec<_> = (0..self.layers.len()).collect();
                indices.sort_by(|&a, &b| {
                    let sum_a: f64 = self.layers[a].values.iter().sum();
                    let sum_b: f64 = self.layers[b].values.iter().sum();
                    sum_b.partial_cmp(&sum_a).unwrap_or(std::cmp::Ordering::Equal)
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
            let values: Vec<f64> = ordered_indices.iter()
                .map(|&i| {
                    self.layers[i].values.get(x).copied().unwrap_or(0.0)
                })
                .collect();

            let total: f64 = values.iter().sum();

            let (y0, scale) = match self.baseline {
                StreamBaseline::Zero => (0.0, 1.0),
                StreamBaseline::Symmetric => (-total / 2.0, 1.0),
                StreamBaseline::Wiggle => {
                    let n = self.layers.len() as f64;
                    let offset: f64 = values.iter().enumerate()
                        .map(|(i, &v)| (n - i as f64) * v)
                        .sum::<f64>() / n;
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

impl View for Streamline {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 5 || height < 3 {
            return;
        }

        // Background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + height.min(area.height) {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        let mut chart_y = area.y;
        let mut chart_height = height.min(area.height);

        // Title
        if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            ctx.buffer.put_str_styled(title_x, chart_y, title, Some(Color::WHITE), self.bg_color);
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        // Legend
        if self.show_legend && !self.layers.is_empty() {
            let mut x = area.x + 1;
            for (i, layer) in self.layers.iter().enumerate() {
                let color = self.get_layer_color(i);
                let mut cell = Cell::new('█');
                cell.fg = Some(color);
                ctx.buffer.set(x, chart_y, cell);
                x += 2;
                ctx.buffer.put_str_styled(x, chart_y, &layer.name, Some(Color::WHITE), self.bg_color);
                x += layer.name.len() as u16 + 2;

                if x > area.x + area.width - 10 {
                    break;
                }
            }
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        // Reserve space for x-axis labels
        let plot_height = if self.x_labels.is_empty() {
            chart_height
        } else {
            chart_height.saturating_sub(1)
        };

        if plot_height < 2 {
            return;
        }

        let stacks = self.compute_stacks();
        if stacks.is_empty() || stacks[0].is_empty() {
            return;
        }

        let num_points = stacks[0].len();

        // Find global min/max for scaling
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for layer_stack in &stacks {
            for &(y0, y1) in layer_stack {
                min_y = min_y.min(y0);
                max_y = max_y.max(y1);
            }
        }

        if min_y == max_y {
            max_y = min_y + 1.0;
        }

        let y_range = max_y - min_y;

        // Render each layer
        for (layer_idx, layer_stack) in stacks.iter().enumerate() {
            let color = self.get_layer_color(layer_idx);
            let is_highlighted = self.highlighted.is_none() || self.highlighted == Some(layer_idx);

            let display_color = if is_highlighted {
                color
            } else {
                Color::rgb(
                    (color.r as u16 / 3) as u8,
                    (color.g as u16 / 3) as u8,
                    (color.b as u16 / 3) as u8,
                )
            };

            for x_idx in 0..num_points {
                let (y0, y1) = layer_stack[x_idx];

                let screen_x = area.x + (x_idx as f64 / (num_points - 1).max(1) as f64 * (area.width - 1) as f64) as u16;

                let screen_y0 = chart_y + plot_height - 1 - ((y0 - min_y) / y_range * (plot_height - 1) as f64) as u16;
                let screen_y1 = chart_y + plot_height - 1 - ((y1 - min_y) / y_range * (plot_height - 1) as f64) as u16;

                let (top_y, bottom_y) = if screen_y0 <= screen_y1 {
                    (screen_y0, screen_y1)
                } else {
                    (screen_y1, screen_y0)
                };

                for y in top_y..=bottom_y {
                    if y >= chart_y && y < chart_y + plot_height {
                        let mut cell = Cell::new('█');
                        cell.fg = Some(display_color);
                        ctx.buffer.set(screen_x, y, cell);
                    }
                }
            }

            // Draw label on the stream
            if self.show_labels && !self.layers[layer_idx].name.is_empty() {
                let mut max_width_x = 0;
                let mut max_width = 0.0f64;

                for (x_idx, &(y0, y1)) in layer_stack.iter().enumerate() {
                    let width = (y1 - y0).abs();
                    if width > max_width {
                        max_width = width;
                        max_width_x = x_idx;
                    }
                }

                let (y0, y1) = layer_stack[max_width_x];
                let mid_y = (y0 + y1) / 2.0;
                let screen_x = area.x + (max_width_x as f64 / (num_points - 1).max(1) as f64 * (area.width - 1) as f64) as u16;
                let screen_y = chart_y + plot_height - 1 - ((mid_y - min_y) / y_range * (plot_height - 1) as f64) as u16;

                let label = &self.layers[layer_idx].name;
                let label_x = screen_x.saturating_sub(label.len() as u16 / 2);

                if screen_y >= chart_y && screen_y < chart_y + plot_height {
                    ctx.buffer.put_str_styled(label_x, screen_y, label, Some(Color::WHITE), Some(display_color));
                }
            }
        }

        // X-axis labels
        if !self.x_labels.is_empty() {
            let label_y = chart_y + plot_height;
            let num_labels = self.x_labels.len().min(area.width as usize / 8);

            for (i, label) in self.x_labels.iter().take(num_labels).enumerate() {
                let x = area.x + (i as f64 / (num_labels - 1).max(1) as f64 * (area.width - 1) as f64) as u16;
                let label_x = x.saturating_sub(label.len() as u16 / 2);
                ctx.buffer.put_str_styled(label_x, label_y, label, Some(Color::WHITE), self.bg_color);
            }
        }
    }

    crate::impl_view_meta!("Streamline");
}

impl_styled_view!(Streamline);
impl_props_builders!(Streamline);

// Convenience constructors

/// Create a new streamline chart
pub fn streamline() -> Streamline {
    Streamline::new()
}

/// Create a streamline chart with layers
pub fn streamline_with_data(layers: Vec<StreamLayer>) -> Streamline {
    let mut chart = Streamline::new();
    for layer in layers {
        chart = chart.layer(layer);
    }
    chart
}

/// Create a music genre popularity stream graph
pub fn genre_stream(data: Vec<(&str, Vec<f64>)>) -> Streamline {
    let mut chart = Streamline::new()
        .title("Music Genre Trends")
        .baseline(StreamBaseline::Symmetric)
        .order(StreamOrder::InsideOut);

    let colors = [
        Color::rgb(231, 76, 60),
        Color::rgb(52, 152, 219),
        Color::rgb(46, 204, 113),
        Color::rgb(155, 89, 182),
        Color::rgb(241, 196, 15),
        Color::rgb(230, 126, 34),
    ];

    for (i, (name, values)) in data.into_iter().enumerate() {
        let layer = StreamLayer::new(name)
            .data(values)
            .color(colors[i % colors.len()]);
        chart = chart.layer(layer);
    }

    chart
}

/// Create a traffic source stream graph
pub fn traffic_stream(data: Vec<(&str, Vec<f64>)>) -> Streamline {
    let mut chart = Streamline::new()
        .title("Traffic Sources")
        .baseline(StreamBaseline::Expand)
        .order(StreamOrder::Descending);

    for (name, values) in data {
        chart = chart.layer(StreamLayer::new(name).data(values));
    }

    chart
}

/// Create a resource usage stream
pub fn resource_stream(
    cpu: Vec<f64>,
    memory: Vec<f64>,
    disk: Vec<f64>,
    network: Vec<f64>,
) -> Streamline {
    Streamline::new()
        .title("Resource Usage")
        .baseline(StreamBaseline::Zero)
        .layer(StreamLayer::new("CPU").data(cpu).color(Color::rgb(52, 152, 219)))
        .layer(StreamLayer::new("Memory").data(memory).color(Color::rgb(155, 89, 182)))
        .layer(StreamLayer::new("Disk").data(disk).color(Color::rgb(46, 204, 113)))
        .layer(StreamLayer::new("Network").data(network).color(Color::rgb(241, 196, 15)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streamline_creation() {
        let chart = streamline()
            .title("Test")
            .layer(StreamLayer::new("A").data(vec![1.0, 2.0, 3.0]));

        assert_eq!(chart.title, Some("Test".to_string()));
        assert_eq!(chart.layers.len(), 1);
    }

    #[test]
    fn test_stream_layer() {
        let layer = StreamLayer::new("Test")
            .data(vec![1.0, 2.0, 3.0])
            .color(Color::RED);

        assert_eq!(layer.name, "Test");
        assert_eq!(layer.values, vec![1.0, 2.0, 3.0]);
        assert_eq!(layer.color, Some(Color::RED));
    }

    #[test]
    fn test_baseline_modes() {
        let chart = streamline()
            .baseline(StreamBaseline::Symmetric);

        assert_eq!(chart.baseline, StreamBaseline::Symmetric);
    }

    #[test]
    fn test_compute_stacks() {
        let chart = streamline()
            .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
            .layer(StreamLayer::new("B").data(vec![5.0, 10.0]))
            .baseline(StreamBaseline::Zero);

        let stacks = chart.compute_stacks();
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].len(), 2);
    }
}
