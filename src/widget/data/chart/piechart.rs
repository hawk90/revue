//! Pie Chart widget for proportional data visualization
//!
//! Supports standard pie charts, donut charts, labels, legends, and exploded segments.

use super::chart_common::{ColorScheme, Legend, LegendPosition};
use super::chart_render::{fill_background, render_legend, render_title, LegendItem};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Pie chart style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PieStyle {
    /// Standard pie chart
    #[default]
    Pie,
    /// Donut chart with hollow center
    Donut,
}

/// Label display style for pie slices
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PieLabelStyle {
    /// No labels
    #[default]
    None,
    /// Show value
    Value,
    /// Show percentage
    Percent,
    /// Show slice label
    Label,
    /// Show label and percentage
    LabelPercent,
}

/// A single slice in the pie chart
#[derive(Clone, Debug)]
pub struct PieSlice {
    /// Slice label
    pub label: String,
    /// Slice value
    pub value: f64,
    /// Custom color (uses palette if None)
    pub color: Option<Color>,
}

impl PieSlice {
    /// Create a new slice
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Create a slice with custom color
    pub fn with_color(label: impl Into<String>, value: f64, color: Color) -> Self {
        Self {
            label: label.into(),
            value,
            color: Some(color),
        }
    }
}

/// Pie chart widget
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

    /// Get total of all slice values
    fn total(&self) -> f64 {
        self.slices.iter().map(|s| s.value).sum()
    }

    /// Get color for slice at index
    fn slice_color(&self, index: usize) -> Color {
        self.slices
            .get(index)
            .and_then(|s| s.color)
            .unwrap_or_else(|| self.colors.get(index))
    }

    /// Calculate angle for a slice
    fn slice_angle(&self, value: f64) -> f64 {
        let total = self.total();
        if total == 0.0 {
            0.0
        } else {
            (value / total) * 360.0
        }
    }

    /// Render the pie chart using simple ASCII/Unicode
    fn render_pie(&self, ctx: &mut RenderContext, center_x: u16, center_y: u16, radius: u16) {
        let total = self.total();
        if total == 0.0 || self.slices.is_empty() {
            return;
        }

        // Aspect ratio correction for terminal characters (typically 2:1)
        let aspect_ratio = 2.0;

        // Draw the pie using polar coordinates
        let mut current_angle = self.start_angle;

        for (slice_idx, slice) in self.slices.iter().enumerate() {
            let slice_angle = self.slice_angle(slice.value);
            let color = self.slice_color(slice_idx);

            // Calculate explode offset if this slice is exploded
            let (offset_x, offset_y) = if self.explode == Some(slice_idx) {
                let mid_angle = current_angle + slice_angle / 2.0;
                let rad = mid_angle.to_radians();
                let offset = self.explode_distance * radius as f64;
                (
                    (offset * rad.cos() * aspect_ratio) as i16,
                    (offset * rad.sin()) as i16,
                )
            } else {
                (0, 0)
            };

            // Draw filled slice
            for y in 0..=(radius * 2) {
                for x in 0..=(radius * 2) {
                    let dx = x as f64 - radius as f64;
                    let dy = (y as f64 - radius as f64) * aspect_ratio;

                    // Check if point is within the slice
                    let distance = (dx * dx + dy * dy).sqrt();
                    let inner_radius = if self.style == PieStyle::Donut {
                        radius as f64 * self.donut_ratio
                    } else {
                        0.0
                    };

                    if distance > radius as f64 || distance < inner_radius {
                        continue;
                    }

                    // Calculate angle of this point
                    let point_angle = dy.atan2(dx).to_degrees();
                    let point_angle = ((point_angle - self.start_angle) % 360.0 + 360.0) % 360.0;

                    // Check if within slice
                    let slice_start = ((current_angle - self.start_angle) % 360.0 + 360.0) % 360.0;
                    let slice_end = slice_start + slice_angle;

                    let in_slice = if slice_end <= 360.0 {
                        point_angle >= slice_start && point_angle < slice_end
                    } else {
                        point_angle >= slice_start || point_angle < (slice_end - 360.0)
                    };

                    if in_slice {
                        let screen_x =
                            (center_x as i16 + offset_x + x as i16 - radius as i16) as u16;
                        let screen_y =
                            (center_y as i16 + offset_y + (y as i16 - radius as i16) / 2) as u16;

                        if screen_x < ctx.buffer.width() && screen_y < ctx.buffer.height() {
                            let mut cell = Cell::new('█');
                            cell.fg = Some(color);
                            ctx.buffer.set(screen_x, screen_y, cell);
                        }
                    }
                }
            }

            current_angle += slice_angle;
        }
    }

    /// Render labels around the pie
    fn render_labels(&self, ctx: &mut RenderContext, center_x: u16, center_y: u16, radius: u16) {
        if matches!(self.labels, PieLabelStyle::None) {
            return;
        }

        let total = self.total();
        if total == 0.0 {
            return;
        }

        let mut current_angle = self.start_angle;

        for slice in &self.slices {
            let slice_angle = self.slice_angle(slice.value);
            let mid_angle = current_angle + slice_angle / 2.0;
            let rad = mid_angle.to_radians();

            // Position label outside the pie
            let label_distance = radius as f64 * 1.3;
            let label_x = center_x as f64 + label_distance * rad.cos() * 2.0;
            let label_y = center_y as f64 + label_distance * rad.sin();

            let label_text = match self.labels {
                PieLabelStyle::None => String::new(),
                PieLabelStyle::Value => format!("{:.1}", slice.value),
                PieLabelStyle::Percent => {
                    format!("{:.0}%", (slice.value / total) * 100.0)
                }
                PieLabelStyle::Label => slice.label.clone(),
                PieLabelStyle::LabelPercent => {
                    format!("{} ({:.0}%)", slice.label, (slice.value / total) * 100.0)
                }
            };

            // Draw label
            let start_x = if mid_angle.cos() < 0.0 {
                (label_x - label_text.len() as f64).max(0.0) as u16
            } else {
                label_x as u16
            };

            for (i, ch) in label_text.chars().enumerate() {
                let x = start_x + i as u16;
                let y = label_y as u16;
                if x < ctx.buffer.width() && y < ctx.buffer.height() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.buffer.set(x, y, cell);
                }
            }

            current_angle += slice_angle;
        }
    }
}

impl View for PieChart {
    crate::impl_view_meta!("PieChart");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 3 || area.height < 3 {
            return;
        }

        // Fill background if set
        if let Some(bg) = self.bg_color {
            fill_background(ctx, area, bg);
        }

        // Draw title using shared function
        let title_offset = render_title(ctx, area, self.title.as_deref(), Color::WHITE);

        // Calculate pie center and radius
        let chart_area_height = area.height.saturating_sub(title_offset);
        let radius = (chart_area_height.min(area.width / 2))
            .saturating_sub(2)
            .max(1);
        let center_x = area.x + area.width / 2;
        let center_y = area.y + title_offset + chart_area_height / 2;

        // Render pie
        self.render_pie(ctx, center_x, center_y, radius);

        // Render labels
        self.render_labels(ctx, center_x, center_y, radius);

        // Render legend using shared function
        let legend_items: Vec<LegendItem<'_>> = self
            .slices
            .iter()
            .enumerate()
            .map(|(i, s)| LegendItem {
                label: &s.label,
                color: self.slice_color(i),
            })
            .collect();
        render_legend(ctx, area, &self.legend, &legend_items);
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
