//! View implementation for Streamline chart

use super::core::Streamline;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::traits::{RenderContext, View};

/// Where the plot sits and how stacked values map to its cells
struct Plot {
    /// First row of the plot
    top: u16,
    /// Rows in the plot
    height: u16,
    /// Columns in the plot (the whole area width)
    width: u16,
    /// Number of data points across
    num_points: usize,
    /// Lowest stacked value
    min_y: f64,
    /// Stacked value span over the plot's height
    y_range: f64,
}

impl Plot {
    /// The frame for `stacks` in `height` rows from `top`, `width` columns wide
    fn new(stacks: &[Vec<(f64, f64)>], top: u16, height: u16, width: u16) -> Self {
        let num_points = stacks[0].len();

        // Find global min/max for scaling
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for layer_stack in stacks {
            for &(y0, y1) in layer_stack {
                min_y = min_y.min(y0);
                max_y = max_y.max(y1);
            }
        }

        if min_y == max_y {
            max_y = min_y + 1.0;
        }

        let y_range = max_y - min_y;

        Self {
            top,
            height,
            width,
            num_points,
            min_y,
            y_range,
        }
    }

    /// The column of data point `x_idx`
    fn col(&self, x_idx: usize) -> u16 {
        (x_idx as f64 / (self.num_points - 1).max(1) as f64 * (self.width - 1) as f64) as u16
    }

    /// The row of stacked value `value`
    fn row(&self, value: f64) -> u16 {
        self.top + self.height
            - 1
            - ((value - self.min_y) / self.y_range * (self.height - 1) as f64) as u16
    }

    /// Whether `y` is one of the plot's rows
    fn contains_row(&self, y: u16) -> bool {
        y >= self.top && y < self.top + self.height
    }
}

impl View for Streamline {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 5 || height < 3 {
            return;
        }

        // The layer palette carries the data - flattening ten series to one
        // color leaves a chart that shows nothing - so it stays. The chrome
        // around it (title, legend names, axis labels) is ordinary text.
        let chrome = ctx.css_color(crate::style::Color::WHITE);

        // Background
        if let Some(bg) = self.bg_color {
            fill_background(ctx, height.min(area.height), bg);
        }

        let mut chart_y: u16 = 0;
        let mut chart_height = height.min(area.height);

        // Title
        if let Some(ref title) = self.title {
            let title_x = (area.width.saturating_sub(display_width(title) as u16)) / 2;
            ctx.draw_text(title_x, chart_y, title, chrome);
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        // Legend
        if self.show_legend && !self.layers.is_empty() {
            self.draw_legend(ctx, chart_y, chrome);
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

        let plot = Plot::new(&stacks, chart_y, plot_height, area.width);

        // Render each layer
        for (layer_idx, layer_stack) in stacks.iter().enumerate() {
            let display_color = self.display_color(layer_idx);

            self.draw_bands(ctx, &plot, layer_stack, display_color);

            // Draw label on the stream
            if self.show_labels && !self.layers[layer_idx].name.is_empty() {
                let label = &self.layers[layer_idx].name;
                draw_stream_label(ctx, &plot, layer_stack, label, chrome, display_color);
            }
        }

        // X-axis labels
        if !self.x_labels.is_empty() {
            self.draw_x_labels(ctx, chart_y + plot_height, chrome);
        }
    }

    crate::impl_view_meta!("Streamline");
}

impl Streamline {
    /// One row of `█ name` entries from column 1, stopping 10 columns
    /// before the right edge
    fn draw_legend(&self, ctx: &mut RenderContext, y: u16, chrome: Color) {
        let area = ctx.area;
        let mut x: u16 = 1;
        for (i, layer) in self.layers.iter().enumerate() {
            let color = self.get_layer_color(i);
            let mut cell = Cell::new('█');
            cell.fg = Some(color);
            ctx.set(x, y, cell);
            x += 2;
            draw_chars(ctx, x, y, &layer.name, chrome, self.bg_color);
            x = x.saturating_add(display_width(&layer.name) as u16 + 2);

            if x > area.width.saturating_sub(10) {
                break;
            }
        }
    }

    /// The layer's color, dimmed to a third when another layer is
    /// highlighted
    fn display_color(&self, layer_idx: usize) -> Color {
        let color = self.get_layer_color(layer_idx);
        let is_highlighted = self.highlighted.is_none() || self.highlighted == Some(layer_idx);

        if is_highlighted {
            color
        } else {
            crate::style::Color::rgb(
                (color.r as u16 / 3) as u8,
                (color.g as u16 / 3) as u8,
                (color.b as u16 / 3) as u8,
            )
        }
    }

    /// One layer's band at each data point's column: `█` from its bottom
    /// row to its top row
    fn draw_bands(
        &self,
        ctx: &mut RenderContext,
        plot: &Plot,
        layer_stack: &[(f64, f64)],
        display_color: Color,
    ) {
        for x_idx in 0..plot.num_points {
            let (y0, y1) = layer_stack[x_idx];

            let screen_x = plot.col(x_idx);

            let screen_y0 = plot.row(y0);
            let screen_y1 = plot.row(y1);

            let (top_y, bottom_y) = if screen_y0 <= screen_y1 {
                (screen_y0, screen_y1)
            } else {
                (screen_y1, screen_y0)
            };

            for y in top_y..=bottom_y {
                if plot.contains_row(y) {
                    let mut cell = Cell::new('█');
                    cell.fg = Some(display_color);
                    ctx.set(screen_x, y, cell);
                }
            }
        }
    }

    /// The x labels on row `y`, spread evenly over the width (at most one
    /// per 8 columns)
    fn draw_x_labels(&self, ctx: &mut RenderContext, y: u16, chrome: Color) {
        let area = ctx.area;
        let num_labels = self.x_labels.len().min(area.width as usize / 8);

        for (i, label) in self.x_labels.iter().take(num_labels).enumerate() {
            let x = (i as f64 / (num_labels - 1).max(1) as f64 * (area.width - 1) as f64) as u16;
            let label_x = x.saturating_sub(display_width(label) as u16 / 2);
            draw_chars(ctx, label_x, y, label, chrome, self.bg_color);
        }
    }
}

/// The layer's name centered on its widest point, on the layer's color
fn draw_stream_label(
    ctx: &mut RenderContext,
    plot: &Plot,
    layer_stack: &[(f64, f64)],
    label: &str,
    chrome: Color,
    display_color: Color,
) {
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
    let screen_x = plot.col(max_width_x);
    let screen_y = plot.row(mid_y);

    let label_x = screen_x.saturating_sub(display_width(label) as u16 / 2);

    if plot.contains_row(screen_y) {
        draw_chars(ctx, label_x, screen_y, label, chrome, Some(display_color));
    }
}

/// Fill the first `rows` rows of the area with `bg`
fn fill_background(ctx: &mut RenderContext, rows: u16, bg: Color) {
    for y in 0..rows {
        for x in 0..ctx.area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg);
            ctx.set(x, y, cell);
        }
    }
}

/// Draw `text` from column `x` on row `y`, one cell per char, advancing by
/// each char's width and stopping at the area's right edge
fn draw_chars(ctx: &mut RenderContext, x: u16, y: u16, text: &str, fg: Color, bg: Option<Color>) {
    let width = ctx.area.width;
    let mut dx: u16 = 0;
    for ch in text.chars() {
        let cw = char_width(ch) as u16;
        if x + dx >= width {
            break;
        }
        let mut c = Cell::new(ch);
        c.fg = Some(fg);
        c.bg = bg;
        ctx.set(x + dx, y, c);
        dx += cw;
    }
}
