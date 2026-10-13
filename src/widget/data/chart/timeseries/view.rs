//! Time series `View`: the background and header rows (title, legend, unit),
//! then the grid, markers, series and time labels in the plot

use super::types::TimeLineStyle;
use super::TimeSeries;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::traits::{RenderContext, View};

/// Where the plot sits in the chart area and the time and value ranges it spans
#[derive(Clone, Copy)]
pub(super) struct Plot {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub time_min: u64,
    pub time_max: u64,
    pub time_range: u64,
    pub val_min: f64,
    pub val_max: f64,
    pub val_range: f64,
}

impl TimeSeries {
    /// The title, centered on row `y`
    fn draw_title(&self, ctx: &mut RenderContext, title: &str, y: u16, chrome: Color) {
        let area = ctx.area;
        let current_y = y;

        let title_x = (area.width.saturating_sub(display_width(title) as u16)) / 2;
        let mut dx: u16 = 0;
        for ch in title.chars() {
            let x = title_x + dx;
            if x < area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(chrome);
                cell.bg = self.bg_color;
                ctx.set(x, current_y, cell);
            }
            dx += char_width(ch) as u16;
        }
    }

    /// Each series' line glyph and name on row `y`
    fn draw_legend(&self, ctx: &mut RenderContext, y: u16, chrome: Color) {
        let area = ctx.area;
        let current_y = y;

        let mut x = 2u16;
        for series in &self.series {
            let marker = match series.line_style {
                TimeLineStyle::Solid => "─",
                TimeLineStyle::Dashed => "╌",
                TimeLineStyle::Dotted => "·",
                TimeLineStyle::Step => "┐",
            };
            for ch in marker.chars() {
                if x < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(series.color);
                    cell.bg = self.bg_color;
                    ctx.set(x, current_y, cell);
                }
                x += 1;
            }
            x += 1;
            for ch in series.name.chars() {
                if x < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(chrome);
                    cell.bg = self.bg_color;
                    ctx.set(x, current_y, cell);
                }
                x += 1;
            }
            x += 3;
        }
    }

    /// The y-axis label (the unit) on row `y`, cut at the area's edge
    fn draw_y_label(&self, ctx: &mut RenderContext, label: &str, y: u16, chrome: Color) {
        let area = ctx.area;
        let current_y = y;

        let mut x: u16 = 0;
        for ch in label.chars() {
            let cw = char_width(ch) as u16;
            if x + cw > area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(chrome);
            cell.bg = self.bg_color;
            ctx.set(x, current_y, cell);
            x += cw;
        }
    }
}

impl View for TimeSeries {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 10 || height < 5 {
            return;
        }

        // The series palette carries the data and the grid has its own builder
        // field; the chrome around them - title, legend, axis labels - is
        // ordinary text.
        let chrome = ctx.css_color(crate::style::Color::WHITE);

        let mut current_y = 0u16;

        // Background
        if let Some(bg) = self.bg_color {
            for y in 0..height.min(area.height) {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.set(x, y, cell);
                }
            }
        }

        // Title
        if let Some(ref title) = self.title {
            self.draw_title(ctx, title, current_y, chrome);
            current_y += 1;
        }

        // Legend
        if self.show_legend && !self.series.is_empty() {
            self.draw_legend(ctx, current_y, chrome);
            current_y += 1;
        }

        // Y-axis label (the unit), on its own row above the value axis
        if let Some(ref label) = self.y_label {
            self.draw_y_label(ctx, label, current_y, chrome);
            current_y += 1;
        }

        let y_label_width = 8u16;
        let plot_x = y_label_width;
        let plot_width = area.width.saturating_sub(y_label_width + 1);
        let plot_y = current_y;
        let plot_height = height.saturating_sub(current_y + 2);

        if plot_width < 5 || plot_height < 3 {
            return;
        }

        let (time_min, time_max) = self.get_time_bounds();
        let (val_min, val_max) = self.get_value_bounds();
        let time_range = time_max.saturating_sub(time_min);
        let val_range = val_max - val_min;

        let plot = Plot {
            x: plot_x,
            y: plot_y,
            width: plot_width,
            height: plot_height,
            time_min,
            time_max,
            time_range,
            val_min,
            val_max,
            val_range,
        };

        // Draw grid
        if self.show_grid {
            self.draw_grid(ctx, &plot, chrome);
        }

        // Draw markers
        self.draw_markers(ctx, &plot);

        // Draw series
        self.render_series(ctx, &plot);

        // X-axis time labels
        self.draw_time_labels(ctx, &plot, height, chrome);
    }

    crate::impl_view_meta!("TimeSeries");
}
