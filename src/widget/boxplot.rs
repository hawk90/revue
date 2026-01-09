//! Box Plot widget for statistical distribution visualization
//!
//! Displays median, quartiles, and outliers for comparing distributions across categories.

use super::chart_common::{Axis, ChartGrid, ChartOrientation, ColorScheme, Legend};
use super::chart_render::{fill_background, render_title};
use super::chart_stats::percentile;
use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Whisker calculation style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WhiskerStyle {
    /// 1.5 * IQR (default)
    #[default]
    IQR,
    /// Min and max values
    MinMax,
    /// Percentile (5th and 95th)
    Percentile,
}

/// Pre-computed box plot statistics
#[derive(Clone, Debug)]
pub struct BoxStats {
    /// Minimum value
    pub min: f64,
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Maximum value
    pub max: f64,
    /// Outlier values
    pub outliers: Vec<f64>,
    /// Lower whisker value
    pub whisker_low: f64,
    /// Upper whisker value
    pub whisker_high: f64,
}

impl BoxStats {
    /// Compute statistics from raw data
    pub fn from_data(data: &[f64], whisker_style: WhiskerStyle) -> Option<Self> {
        let mut valid: Vec<f64> = data.iter().filter(|x| x.is_finite()).copied().collect();
        if valid.is_empty() {
            return None;
        }

        valid.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = valid.len();
        let min = valid[0];
        let max = valid[n - 1];

        let median = if n.is_multiple_of(2) {
            (valid[n / 2 - 1] + valid[n / 2]) / 2.0
        } else {
            valid[n / 2]
        };

        let q1 = percentile(&valid, 25.0);
        let q3 = percentile(&valid, 75.0);
        let iqr = q3 - q1;

        let (whisker_low, whisker_high, outliers) = match whisker_style {
            WhiskerStyle::IQR => {
                let lower_fence = q1 - 1.5 * iqr;
                let upper_fence = q3 + 1.5 * iqr;
                let whisker_low = valid
                    .iter()
                    .find(|&&x| x >= lower_fence)
                    .copied()
                    .unwrap_or(min);
                let whisker_high = valid
                    .iter()
                    .rev()
                    .find(|&&x| x <= upper_fence)
                    .copied()
                    .unwrap_or(max);
                let outliers: Vec<f64> = valid
                    .iter()
                    .filter(|&&x| x < lower_fence || x > upper_fence)
                    .copied()
                    .collect();
                (whisker_low, whisker_high, outliers)
            }
            WhiskerStyle::MinMax => (min, max, Vec::new()),
            WhiskerStyle::Percentile => {
                let p5 = percentile(&valid, 5.0);
                let p95 = percentile(&valid, 95.0);
                let outliers: Vec<f64> = valid
                    .iter()
                    .filter(|&&x| x < p5 || x > p95)
                    .copied()
                    .collect();
                (p5, p95, outliers)
            }
        };

        Some(BoxStats {
            min,
            q1,
            median,
            q3,
            max,
            outliers,
            whisker_low,
            whisker_high,
        })
    }
}

/// A box plot group
#[derive(Clone, Debug)]
pub struct BoxGroup {
    /// Group label
    pub label: String,
    /// Raw data (stats will be computed)
    pub data: Vec<f64>,
    /// Pre-computed statistics (optional)
    pub stats: Option<BoxStats>,
    /// Custom color
    pub color: Option<Color>,
}

impl BoxGroup {
    /// Create a group from raw data
    pub fn new(label: impl Into<String>, data: &[f64]) -> Self {
        Self {
            label: label.into(),
            data: data.to_vec(),
            stats: None,
            color: None,
        }
    }

    /// Create a group with pre-computed stats
    pub fn from_stats(label: impl Into<String>, stats: BoxStats) -> Self {
        Self {
            label: label.into(),
            data: Vec::new(),
            stats: Some(stats),
            color: None,
        }
    }

    /// Set custom color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Get statistics (compute if necessary)
    fn get_stats(&self, whisker_style: WhiskerStyle) -> Option<BoxStats> {
        self.stats
            .clone()
            .or_else(|| BoxStats::from_data(&self.data, whisker_style))
    }
}

/// Box plot widget
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

    /// Get color for group at index
    fn group_color(&self, index: usize) -> Color {
        self.groups
            .get(index)
            .and_then(|g| g.color)
            .unwrap_or_else(|| self.colors.get(index))
    }

    /// Compute value bounds
    fn compute_bounds(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for group in &self.groups {
            if let Some(stats) = group.get_stats(self.whisker_style) {
                min = min.min(stats.whisker_low);
                max = max.max(stats.whisker_high);
                for &outlier in &stats.outliers {
                    min = min.min(outlier);
                    max = max.max(outlier);
                }
            }
        }

        // Apply axis overrides
        let min = self.value_axis.min.unwrap_or(min);
        let max = self.value_axis.max.unwrap_or(max);

        // Add padding
        let range = (max - min).max(1.0);
        let padding = range * 0.1;

        (min - padding, max + padding)
    }

    /// Map value to screen coordinate
    fn value_to_screen(&self, value: f64, bounds: (f64, f64), length: u16) -> u16 {
        let (min, max) = bounds;
        let range = (max - min).max(1.0);
        ((value - min) / range * (length as f64 - 1.0)) as u16
    }

    /// Render box plots
    fn render_boxes(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if self.groups.is_empty() {
            return;
        }

        let bounds = self.compute_bounds();
        let n_groups = self.groups.len();
        let group_width = chart_area.width / n_groups as u16;
        let box_width = (group_width as f64 * self.box_width) as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let Some(stats) = group.get_stats(self.whisker_style) else {
                continue;
            };

            let color = self.group_color(i);
            let group_center = chart_area.x + (i as u16 * group_width) + group_width / 2;
            let box_left = group_center.saturating_sub(box_width / 2);
            let box_right = box_left + box_width;

            // Calculate y positions (inverted because y increases downward)
            let y_whisker_low = chart_area.y + chart_area.height
                - 1
                - self.value_to_screen(stats.whisker_low, bounds, chart_area.height);
            let y_q1 = chart_area.y + chart_area.height
                - 1
                - self.value_to_screen(stats.q1, bounds, chart_area.height);
            let y_median = chart_area.y + chart_area.height
                - 1
                - self.value_to_screen(stats.median, bounds, chart_area.height);
            let y_q3 = chart_area.y + chart_area.height
                - 1
                - self.value_to_screen(stats.q3, bounds, chart_area.height);
            let y_whisker_high = chart_area.y + chart_area.height
                - 1
                - self.value_to_screen(stats.whisker_high, bounds, chart_area.height);

            // Draw whiskers (vertical line in center)
            for y in y_whisker_high.min(y_whisker_low)..=y_whisker_high.max(y_whisker_low) {
                if y >= chart_area.y && y < chart_area.y + chart_area.height {
                    let mut cell = Cell::new('│');
                    cell.fg = Some(color);
                    ctx.buffer.set(group_center, y, cell);
                }
            }

            // Draw whisker caps
            for x in box_left..=box_right {
                if x >= chart_area.x && x < chart_area.x + chart_area.width {
                    // Lower whisker cap
                    if y_whisker_low >= chart_area.y
                        && y_whisker_low < chart_area.y + chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y_whisker_low, cell);
                    }
                    // Upper whisker cap
                    if y_whisker_high >= chart_area.y
                        && y_whisker_high < chart_area.y + chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y_whisker_high, cell);
                    }
                }
            }

            // Draw box (Q1 to Q3)
            for y in y_q3.min(y_q1)..=y_q3.max(y_q1) {
                if y < chart_area.y || y >= chart_area.y + chart_area.height {
                    continue;
                }
                for x in box_left..=box_right {
                    if x < chart_area.x || x >= chart_area.x + chart_area.width {
                        continue;
                    }

                    let ch = if y == y_q1.min(y_q3) {
                        if x == box_left {
                            '┌'
                        } else if x == box_right {
                            '┐'
                        } else {
                            '─'
                        }
                    } else if y == y_q1.max(y_q3) {
                        if x == box_left {
                            '└'
                        } else if x == box_right {
                            '┘'
                        } else {
                            '─'
                        }
                    } else if x == box_left || x == box_right {
                        '│'
                    } else {
                        ' '
                    };

                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    ctx.buffer.set(x, y, cell);
                }
            }

            // Draw median line
            for x in box_left..=box_right {
                if x >= chart_area.x
                    && x < chart_area.x + chart_area.width
                    && y_median >= chart_area.y
                    && y_median < chart_area.y + chart_area.height
                {
                    let ch = if x == box_left {
                        '├'
                    } else if x == box_right {
                        '┤'
                    } else {
                        '─'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.buffer.set(x, y_median, cell);
                }
            }

            // Draw outliers
            if self.show_outliers {
                for &outlier in &stats.outliers {
                    let y = chart_area.y + chart_area.height
                        - 1
                        - self.value_to_screen(outlier, bounds, chart_area.height);
                    if y >= chart_area.y
                        && y < chart_area.y + chart_area.height
                        && group_center >= chart_area.x
                        && group_center < chart_area.x + chart_area.width
                    {
                        let mut cell = Cell::new('○');
                        cell.fg = Some(color);
                        ctx.buffer.set(group_center, y, cell);
                    }
                }
            }
        }
    }

    /// Render axis labels
    fn render_axes(&self, ctx: &mut RenderContext, area: Rect) {
        if self.groups.is_empty() {
            return;
        }

        let bounds = self.compute_bounds();
        let (min, max) = bounds;

        // Value axis labels (left side)
        let y_label_width = 6u16;
        for i in 0..=4 {
            let value = max - (max - min) * i as f64 / 4.0;
            let label = self.value_axis.format_value(value);
            let y = area.y + 1 + (i as u16 * (area.height - 3) / 4);

            for (j, ch) in label.chars().take(y_label_width as usize - 1).enumerate() {
                let x = area.x + j as u16;
                if x < area.x + y_label_width && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.value_axis.color);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Category axis labels (bottom)
        let n_groups = self.groups.len();
        let chart_width = area.width.saturating_sub(y_label_width);
        let group_width = chart_width / n_groups as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let x = area.x + y_label_width + (i as u16 * group_width) + group_width / 2;
            let y = area.y + area.height - 1;
            let label_start = x.saturating_sub(group.label.len() as u16 / 2);

            for (j, ch) in group.label.chars().enumerate() {
                let label_x = label_start + j as u16;
                if label_x >= area.x + y_label_width && label_x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.category_axis.color);
                    ctx.buffer.set(label_x, y, cell);
                }
            }
        }
    }
}

impl View for BoxPlot {
    crate::impl_view_meta!("BoxPlot");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        if area.width < 10 || area.height < 8 {
            return;
        }

        // Fill background using shared function
        if let Some(bg) = self.bg_color {
            fill_background(ctx, area, bg);
        }

        // Draw title using shared function
        let title_offset = render_title(ctx, area, self.title.as_deref(), Color::WHITE);

        // Calculate chart area
        let y_label_width = 6u16;
        let x_label_height = 1u16;

        let chart_area = Rect {
            x: area.x + y_label_width,
            y: area.y + title_offset,
            width: area.width.saturating_sub(y_label_width + 1),
            height: area
                .height
                .saturating_sub(title_offset + x_label_height + 1),
        };

        if chart_area.width < 5 || chart_area.height < 5 {
            return;
        }

        // Render components
        self.render_boxes(ctx, chart_area);
        self.render_axes(ctx, area);
    }
}

impl_styled_view!(BoxPlot);
impl_props_builders!(BoxPlot);

/// Create a new box plot
pub fn boxplot() -> BoxPlot {
    BoxPlot::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxplot_new() {
        let bp = BoxPlot::new();
        assert!(bp.groups.is_empty());
    }

    #[test]
    fn test_boxplot_group() {
        let bp = BoxPlot::new()
            .group("A", &[1.0, 2.0, 3.0, 4.0, 5.0])
            .group("B", &[2.0, 3.0, 4.0, 5.0, 6.0]);

        assert_eq!(bp.groups.len(), 2);
    }

    #[test]
    fn test_boxstats_from_data() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();

        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.median, 5.5);
    }

    #[test]
    fn test_boxstats_quartiles() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();

        assert!(stats.q1 >= 2.0 && stats.q1 <= 3.0);
        assert!(stats.q3 >= 6.0 && stats.q3 <= 7.0);
    }

    #[test]
    fn test_boxstats_outliers() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier
        let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();

        assert!(!stats.outliers.is_empty());
        assert!(stats.outliers.contains(&100.0));
    }

    #[test]
    fn test_boxplot_orientation() {
        let bp = BoxPlot::new().horizontal();
        assert_eq!(bp.orientation, ChartOrientation::Horizontal);

        let bp = BoxPlot::new().vertical();
        assert_eq!(bp.orientation, ChartOrientation::Vertical);
    }

    #[test]
    fn test_boxplot_whisker_style() {
        let bp = BoxPlot::new().whisker_style(WhiskerStyle::MinMax);
        assert_eq!(bp.whisker_style, WhiskerStyle::MinMax);

        let bp = BoxPlot::new().whisker_style(WhiskerStyle::Percentile);
        assert_eq!(bp.whisker_style, WhiskerStyle::Percentile);
    }

    #[test]
    fn test_boxplot_notched() {
        let bp = BoxPlot::new().notched(true);
        assert!(bp.notched);
    }

    #[test]
    fn test_boxplot_show_outliers() {
        let bp = BoxPlot::new().show_outliers(false);
        assert!(!bp.show_outliers);
    }

    #[test]
    fn test_boxplot_box_width() {
        let bp = BoxPlot::new().box_width(0.8);
        assert_eq!(bp.box_width, 0.8);
    }

    #[test]
    fn test_boxplot_builder() {
        let bp = BoxPlot::new()
            .title("Distribution")
            .group("A", &[1.0, 2.0, 3.0])
            .group("B", &[4.0, 5.0, 6.0])
            .value_axis(Axis::new().title("Value"))
            .whisker_style(WhiskerStyle::IQR)
            .show_outliers(true)
            .notched(false);

        assert_eq!(bp.title, Some("Distribution".to_string()));
        assert_eq!(bp.groups.len(), 2);
    }

    #[test]
    fn test_boxplot_helper() {
        let bp = boxplot();
        assert!(bp.groups.is_empty());
    }

    #[test]
    fn test_percentile() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&sorted, 0.0), 1.0);
        assert_eq!(percentile(&sorted, 50.0), 3.0);
        assert_eq!(percentile(&sorted, 100.0), 5.0);
    }

    #[test]
    fn test_box_group() {
        let group = BoxGroup::new("Test", &[1.0, 2.0, 3.0]);
        assert_eq!(group.label, "Test");
        assert_eq!(group.data.len(), 3);

        let group = group.color(Color::RED);
        assert_eq!(group.color, Some(Color::RED));
    }

    // ========== Render Tests ==========

    #[test]
    fn test_boxplot_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new().group("Data", &data);
        bp.render(&mut ctx);

        // Verify box elements are rendered
        let mut has_box = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '┌'
                        || cell.symbol == '┐'
                        || cell.symbol == '└'
                        || cell.symbol == '┘'
                        || cell.symbol == '│'
                        || cell.symbol == '─'
                    {
                        has_box = true;
                        break;
                    }
                }
            }
        }
        assert!(has_box);
    }

    #[test]
    fn test_boxplot_render_multiple_groups() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(60, 25);
        let area = Rect::new(0, 0, 60, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new()
            .group("Group A", &[1.0, 2.0, 3.0, 4.0, 5.0])
            .group("Group B", &[3.0, 4.0, 5.0, 6.0, 7.0])
            .group("Group C", &[5.0, 6.0, 7.0, 8.0, 9.0]);

        bp.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..25 {
            for x in 0..60 {
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
    fn test_boxplot_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new()
            .title("Test BoxPlot")
            .group("D", &[1.0, 2.0, 3.0, 4.0, 5.0]);

        bp.render(&mut ctx);

        // Title should be rendered
        let mut title_found = false;
        for x in 0..40 {
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
    fn test_boxplot_render_with_outliers() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Data with outliers
        let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
        data.push(100.0); // Outlier
        data.push(-50.0); // Outlier

        let bp = BoxPlot::new()
            .group("Data", &data)
            .show_outliers(true)
            .whisker_style(WhiskerStyle::IQR);

        bp.render(&mut ctx);

        // Should render without panic and show outliers
        let mut has_outliers = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '○' {
                        has_outliers = true;
                        break;
                    }
                }
            }
        }
        assert!(has_outliers);
    }

    #[test]
    fn test_boxplot_render_horizontal() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new()
            .group("Data", &[1.0, 2.0, 3.0, 4.0, 5.0])
            .horizontal();

        bp.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
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
    fn test_boxplot_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bp = BoxPlot::new().group("D", &[1.0, 2.0, 3.0]);

        // Should not panic on small area
        bp.render(&mut ctx);
    }

    #[test]
    fn test_boxplot_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Empty boxplot
        let bp = BoxPlot::new();
        bp.render(&mut ctx);
    }

    #[test]
    fn test_boxplot_render_notched() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..30).map(|x| x as f64).collect();
        let bp = BoxPlot::new().group("Data", &data).notched(true);

        bp.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
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
    fn test_boxplot_render_minmax_whiskers() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let data: Vec<f64> = (0..20).map(|x| x as f64).collect();
        let bp = BoxPlot::new()
            .group("Data", &data)
            .whisker_style(WhiskerStyle::MinMax);

        bp.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
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
}
