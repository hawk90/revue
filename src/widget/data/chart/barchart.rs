//! Bar chart widget for data visualization
//!
//! Displays data as horizontal or vertical bars.

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Bar orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BarOrientation {
    /// Horizontal bars (default)
    #[default]
    Horizontal,
    /// Vertical bars
    Vertical,
}

/// A single bar in the chart
#[derive(Clone, Debug)]
pub struct Bar {
    /// Label for the bar
    pub label: String,
    /// Value of the bar
    pub value: f64,
    /// Optional color for this bar
    pub color: Option<Color>,
}

impl Bar {
    /// Create a new bar
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set bar color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

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

    /// Calculate the maximum value in the data
    fn calculate_max(&self) -> f64 {
        self.max.unwrap_or_else(|| {
            self.bars
                .iter()
                .map(|b| b.value)
                .fold(0.0, f64::max)
                .max(1.0)
        })
    }

    /// Render horizontal bars
    fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.bars.is_empty() {
            return;
        }

        let max_value = self.calculate_max();

        // Calculate label width
        let label_width = self.label_width.unwrap_or_else(|| {
            self.bars
                .iter()
                .map(|b| b.label.len() as u16)
                .max()
                .unwrap_or(0)
                .min(area.width / 3)
        });

        // Calculate available bar space
        let value_width = if self.show_values { 8 } else { 0 };
        let bar_area_width = area.width.saturating_sub(label_width + 2 + value_width);

        let mut y = 0u16;
        for bar in &self.bars {
            if y >= area.height {
                break;
            }

            // Calculate bar length
            let bar_length = if max_value > 0.0 {
                ((bar.value / max_value) * bar_area_width as f64) as u16
            } else {
                0
            };

            let color = bar.color.unwrap_or(self.fg);

            // Render for each row of bar_width
            for row in 0..self.bar_width {
                if y + row >= area.height {
                    break;
                }

                // Draw label (only on first row)
                if row == 0 {
                    let label: String = if bar.label.len() > label_width as usize {
                        bar.label.chars().take(label_width as usize).collect()
                    } else {
                        format!("{:>width$}", bar.label, width = label_width as usize)
                    };

                    for (i, ch) in label.chars().enumerate() {
                        if (i as u16) < area.width {
                            ctx.buffer.set(area.x + i as u16, area.y + y, Cell::new(ch));
                        }
                    }
                }

                // Draw bar
                let bar_start = label_width + 1;
                for i in 0..bar_length {
                    if bar_start + i < area.width {
                        let mut cell = Cell::new('█');
                        cell.fg = Some(color);
                        ctx.buffer
                            .set(area.x + bar_start + i, area.y + y + row, cell);
                    }
                }

                // Draw value (only on first row)
                if row == 0 && self.show_values {
                    let value_str = format!(" {:.1}", bar.value);
                    let value_x = bar_start + bar_length;
                    for (i, ch) in value_str.chars().enumerate() {
                        if value_x + (i as u16) < area.width {
                            ctx.buffer.set(
                                area.x + value_x + (i as u16),
                                area.y + y,
                                Cell::new(ch),
                            );
                        }
                    }
                }
            }

            y += self.bar_width + self.gap;
        }
    }

    /// Render vertical bars
    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.bars.is_empty() {
            return;
        }

        let max_value = self.calculate_max();

        // Reserve space for labels and values
        let label_height = 1;
        let value_height = if self.show_values { 1 } else { 0 };
        let bar_area_height = area.height.saturating_sub(label_height + value_height);

        let total_bar_width = self.bar_width + self.gap;
        let mut x = 0u16;

        for bar in &self.bars {
            if x + self.bar_width > area.width {
                break;
            }

            // Calculate bar height
            let bar_height = if max_value > 0.0 {
                ((bar.value / max_value) * bar_area_height as f64) as u16
            } else {
                0
            };

            let color = bar.color.unwrap_or(self.fg);

            // Draw bar (from bottom up)
            for row in 0..bar_height {
                let y = area.y + bar_area_height - 1 - row;
                for col in 0..self.bar_width {
                    if x + col < area.width {
                        let mut cell = Cell::new('█');
                        cell.fg = Some(color);
                        ctx.buffer.set(area.x + x + col, y, cell);
                    }
                }
            }

            // Draw value above bar
            if self.show_values && bar_area_height > 0 {
                let value_str = format!("{:.0}", bar.value);
                let value_y =
                    area.y + bar_area_height - bar_height.saturating_sub(1).min(bar_area_height);
                for (i, ch) in value_str.chars().enumerate() {
                    if x + (i as u16) < area.width && value_y > area.y {
                        ctx.buffer
                            .set(area.x + x + (i as u16), value_y - 1, Cell::new(ch));
                    }
                }
            }

            // Draw label below
            if label_height > 0 {
                let label_y = area.y + area.height - 1;
                let label: String = bar.label.chars().take(self.bar_width as usize).collect();
                for (i, ch) in label.chars().enumerate() {
                    if x + (i as u16) < area.width {
                        ctx.buffer
                            .set(area.x + x + (i as u16), label_y, Cell::new(ch));
                    }
                }
            }

            x += total_bar_width;
        }
    }
}

impl Default for BarChart {
    fn default() -> Self {
        Self::new()
    }
}

impl View for BarChart {
    crate::impl_view_meta!("BarChart");

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            BarOrientation::Horizontal => self.render_horizontal(ctx),
            BarOrientation::Vertical => self.render_vertical(ctx),
        }
    }
}

impl_styled_view!(BarChart);
impl_props_builders!(BarChart);

/// Create a bar chart
pub fn barchart() -> BarChart {
    BarChart::new()
}

// Tests - KEEP HERE - private tests for bar chart
