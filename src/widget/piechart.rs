//! Pie Chart widget for proportional data visualization
//!
//! Supports standard pie charts, donut charts, labels, legends, and exploded segments.

use super::chart_common::{ColorScheme, Legend, LegendPosition};
use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
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

    /// Render legend
    fn render_legend(&self, ctx: &mut RenderContext, area: Rect) {
        if !self.legend.is_visible() || self.slices.is_empty() {
            return;
        }

        let legend_width = self
            .slices
            .iter()
            .map(|s| s.label.len() + 4)
            .max()
            .unwrap_or(10) as u16;
        let legend_height = self.slices.len() as u16 + 2;

        let (legend_x, legend_y) = match self.legend.position {
            LegendPosition::TopLeft => (area.x + 1, area.y + 1),
            LegendPosition::TopCenter => (area.x + (area.width - legend_width) / 2, area.y + 1),
            LegendPosition::TopRight => (
                area.x + area.width.saturating_sub(legend_width + 1),
                area.y + 1,
            ),
            LegendPosition::BottomLeft => (
                area.x + 1,
                area.y + area.height.saturating_sub(legend_height + 1),
            ),
            LegendPosition::BottomCenter => (
                area.x + (area.width - legend_width) / 2,
                area.y + area.height.saturating_sub(legend_height + 1),
            ),
            LegendPosition::BottomRight => (
                area.x + area.width.saturating_sub(legend_width + 1),
                area.y + area.height.saturating_sub(legend_height + 1),
            ),
            LegendPosition::Left => (area.x + 1, area.y + (area.height - legend_height) / 2),
            LegendPosition::Right => (
                area.x + area.width.saturating_sub(legend_width + 1),
                area.y + (area.height - legend_height) / 2,
            ),
            LegendPosition::None => return,
        };

        // Draw legend background
        for dy in 0..legend_height {
            for dx in 0..legend_width {
                let x = legend_x + dx;
                let y = legend_y + dy;
                if x < area.x + area.width && y < area.y + area.height {
                    let ch = if dy == 0 && dx == 0 {
                        '┌'
                    } else if dy == 0 && dx == legend_width - 1 {
                        '┐'
                    } else if dy == legend_height - 1 && dx == 0 {
                        '└'
                    } else if dy == legend_height - 1 && dx == legend_width - 1 {
                        '┘'
                    } else if dy == 0 || dy == legend_height - 1 {
                        '─'
                    } else if dx == 0 || dx == legend_width - 1 {
                        '│'
                    } else {
                        ' '
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(100, 100, 100));
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw legend entries
        for (i, slice) in self.slices.iter().enumerate() {
            let y = legend_y + 1 + i as u16;
            if y >= area.y + area.height - 1 {
                break;
            }

            // Color indicator
            let x = legend_x + 1;
            if x < area.x + area.width {
                let mut cell = Cell::new('■');
                cell.fg = Some(self.slice_color(i));
                ctx.buffer.set(x, y, cell);
            }

            // Label
            for (j, ch) in slice.label.chars().enumerate() {
                let x = legend_x + 3 + j as u16;
                if x < legend_x + legend_width - 1 {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.buffer.set(x, y, cell);
                }
            }
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
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw title if set
        let title_offset = if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            for (i, ch) in title.chars().enumerate() {
                let x = title_x + i as u16;
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.buffer.set(x, area.y, cell);
                }
            }
            1
        } else {
            0
        };

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

        // Render legend
        self.render_legend(ctx, area);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pie_chart_new() {
        let chart = PieChart::new();
        assert!(chart.slices.is_empty());
        assert_eq!(chart.style, PieStyle::Pie);
        assert_eq!(chart.start_angle, -90.0);
    }

    #[test]
    fn test_pie_chart_slices() {
        let chart = PieChart::new()
            .slice("A", 30.0)
            .slice("B", 50.0)
            .slice("C", 20.0);

        assert_eq!(chart.slices.len(), 3);
        assert_eq!(chart.total(), 100.0);
    }

    #[test]
    fn test_pie_chart_slice_angles() {
        let chart = PieChart::new()
            .slice("A", 25.0)
            .slice("B", 25.0)
            .slice("C", 25.0)
            .slice("D", 25.0);

        assert_eq!(chart.slice_angle(25.0), 90.0);
    }

    #[test]
    fn test_pie_chart_colors() {
        let chart = PieChart::new()
            .slice("A", 30.0)
            .slice_colored("B", 50.0, Color::RED);

        // First slice uses palette
        let color0 = chart.slice_color(0);
        assert_ne!(color0.r, 0);

        // Second slice uses custom color
        let color1 = chart.slice_color(1);
        assert_eq!(color1.r, 255);
    }

    #[test]
    fn test_donut_chart() {
        let chart = PieChart::new().donut(0.6);

        assert_eq!(chart.style, PieStyle::Donut);
        assert_eq!(chart.donut_ratio, 0.6);
    }

    #[test]
    fn test_pie_chart_explode() {
        let chart = PieChart::new().slice("A", 30.0).slice("B", 50.0).explode(0);

        assert_eq!(chart.explode, Some(0));
    }

    #[test]
    fn test_pie_chart_labels() {
        let chart = PieChart::new().labels(PieLabelStyle::Percent);
        assert_eq!(chart.labels, PieLabelStyle::Percent);
    }

    #[test]
    fn test_pie_chart_legend() {
        let chart = PieChart::new().legend(Legend::bottom_left());
        assert_eq!(chart.legend.position, LegendPosition::BottomLeft);

        let chart = PieChart::new().no_legend();
        assert!(!chart.legend.is_visible());
    }

    #[test]
    fn test_pie_chart_builder_chain() {
        let chart = PieChart::new()
            .title("Sales")
            .slice("Product A", 100.0)
            .slice("Product B", 200.0)
            .slice("Product C", 150.0)
            .donut(0.4)
            .labels(PieLabelStyle::LabelPercent)
            .legend(Legend::right())
            .explode(1)
            .start_angle(0.0);

        assert_eq!(chart.title, Some("Sales".to_string()));
        assert_eq!(chart.slices.len(), 3);
        assert_eq!(chart.style, PieStyle::Donut);
        assert_eq!(chart.donut_ratio, 0.4);
        assert_eq!(chart.labels, PieLabelStyle::LabelPercent);
        assert_eq!(chart.explode, Some(1));
        assert_eq!(chart.start_angle, 0.0);
    }

    #[test]
    fn test_pie_helpers() {
        let chart = pie_chart();
        assert_eq!(chart.style, PieStyle::Pie);

        let chart = donut_chart();
        assert_eq!(chart.style, PieStyle::Donut);
        assert_eq!(chart.donut_ratio, 0.5);
    }

    #[test]
    fn test_pie_slice_struct() {
        let slice = PieSlice::new("Test", 42.0);
        assert_eq!(slice.label, "Test");
        assert_eq!(slice.value, 42.0);
        assert!(slice.color.is_none());

        let slice = PieSlice::with_color("Colored", 100.0, Color::BLUE);
        assert!(slice.color.is_some());
    }

    // ========== Render Tests ==========

    #[test]
    fn test_pie_chart_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = PieChart::new().slice("A", 50.0).slice("B", 50.0);

        chart.render(&mut ctx);

        // Verify something was rendered (not all spaces)
        let mut has_content = false;
        for y in 0..15 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_pie_chart_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = PieChart::new().title("Test Chart").slice("A", 100.0);

        chart.render(&mut ctx);

        // Title should be rendered at the top
        let mut title_found = false;
        for x in 0..30 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'T' {
                    title_found = true;
                    break;
                }
            }
        }
        assert!(title_found);
    }

    #[test]
    fn test_pie_chart_render_donut() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = donut_chart().slice("A", 50.0).slice("B", 50.0);

        chart.render(&mut ctx);

        // Verify donut renders (has content)
        let mut has_content = false;
        for y in 0..15 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_pie_chart_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        // Very small area - should handle gracefully
        let mut buffer = Buffer::new(5, 3);
        let area = Rect::new(0, 0, 5, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = PieChart::new().slice("A", 100.0);

        // Should not panic
        chart.render(&mut ctx);
    }

    #[test]
    fn test_pie_chart_render_with_legend() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = PieChart::new()
            .slice("Alpha", 50.0)
            .slice("Beta", 30.0)
            .slice("Gamma", 20.0)
            .legend(Legend::bottom_center());

        chart.render(&mut ctx);

        // Verify legend area has content (look for legend markers)
        let mut legend_found = false;
        for y in 15..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '■' || cell.symbol == '●' {
                        legend_found = true;
                        break;
                    }
                }
            }
        }
        assert!(legend_found);
    }

    #[test]
    fn test_pie_chart_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Empty chart - should not panic
        let chart = PieChart::new();
        chart.render(&mut ctx);
    }

    #[test]
    fn test_pie_chart_render_labels() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(50, 25);
        let area = Rect::new(0, 0, 50, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let chart = PieChart::new()
            .slice("A", 50.0)
            .slice("B", 50.0)
            .labels(PieLabelStyle::Percent);

        chart.render(&mut ctx);

        // Check if percentage labels are rendered (look for %)
        let mut percent_found = false;
        for y in 0..25 {
            for x in 0..50 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '%' {
                        percent_found = true;
                        break;
                    }
                }
            }
        }
        assert!(percent_found);
    }
}
