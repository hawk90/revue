//! Sparkline widget for inline data visualization

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Block characters for sparkline (8 levels)
const SPARK_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Width allocated for showing max/min bounds (e.g., "max:100")
const SPARKLINE_BOUNDS_WIDTH: usize = 8;

/// Sparkline style variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SparklineStyle {
    /// Block characters (default): ▁▂▃▄▅▆▇█
    #[default]
    Block,
    /// Braille dots pattern
    Braille,
    /// Simple ASCII: _.-=+*#
    Ascii,
}

impl SparklineStyle {
    /// Get characters for this style (from lowest to highest)
    fn chars(&self) -> &[char] {
        match self {
            SparklineStyle::Block => &SPARK_CHARS,
            SparklineStyle::Braille => &['⠀', '⣀', '⣤', '⣶', '⣿', '⣿', '⣿', '⣿'],
            SparklineStyle::Ascii => &['_', '.', '-', '=', '+', '*', '#', '@'],
        }
    }
}

/// A sparkline widget for compact data visualization
#[derive(Clone)]
pub struct Sparkline {
    data: Vec<f64>,
    max: Option<f64>,
    min: Option<f64>,
    style: SparklineStyle,
    fg: Option<Color>,
    bg: Option<Color>,
    show_bounds: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Sparkline {
    /// Create a new sparkline with data
    pub fn new<I>(data: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        Self {
            data: data.into_iter().collect(),
            max: None,
            min: None,
            style: SparklineStyle::default(),
            fg: None,
            bg: None,
            show_bounds: false,
            props: WidgetProps::new(),
        }
    }

    /// Create an empty sparkline
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Set data
    pub fn data<I>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        self.data = data.into_iter().collect();
        self
    }

    /// Set maximum value (for scaling)
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set minimum value (for scaling)
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set sparkline style
    pub fn style(mut self, style: SparklineStyle) -> Self {
        self.style = style;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Show min/max bounds as labels
    pub fn show_bounds(mut self, show: bool) -> Self {
        self.show_bounds = show;
        self
    }

    /// Push a new data point
    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Push and shift (for streaming data)
    pub fn push_shift(&mut self, value: f64, max_len: usize) {
        self.data.push(value);
        while self.data.len() > max_len {
            self.data.remove(0);
        }
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get current data
    pub fn get_data(&self) -> &[f64] {
        &self.data
    }

    /// Calculate actual min/max from data
    fn calc_bounds(&self) -> (f64, f64) {
        let data_min = self.data.iter().cloned().fold(f64::INFINITY, f64::min);
        let data_max = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let min = self.min.unwrap_or(data_min.min(0.0));
        let max = self.max.unwrap_or(data_max);

        // Ensure we have a valid range
        if (max - min).abs() < f64::EPSILON {
            (min - 1.0, max + 1.0)
        } else {
            (min, max)
        }
    }

    /// Map a value to a character index (0-7)
    fn value_to_index(&self, value: f64, min: f64, max: f64) -> usize {
        let range = max - min;
        if range.abs() < f64::EPSILON {
            return 4; // Middle value
        }

        let normalized = ((value - min) / range).clamp(0.0, 1.0);
        let index = (normalized * 7.0).round() as usize;
        index.min(7)
    }
}

impl Default for Sparkline {
    fn default() -> Self {
        Self::empty()
    }
}

impl View for Sparkline {
    crate::impl_view_meta!("Sparkline");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let fg = self.fg.unwrap_or(Color::CYAN);
        let chars = self.style.chars();

        // Calculate bounds
        let (min, max) = self.calc_bounds();

        // Calculate how many data points we can show
        let bounds_width = if self.show_bounds {
            SPARKLINE_BOUNDS_WIDTH
        } else {
            0
        };
        let available_width = (area.width as usize).saturating_sub(bounds_width);

        // Get the data slice to display
        let data_slice = if self.data.len() > available_width {
            &self.data[self.data.len() - available_width..]
        } else {
            &self.data[..]
        };

        // Render sparkline
        let mut x = area.x;

        // Show max bound
        if self.show_bounds {
            let max_str = format!("{:.0}", max);
            for ch in max_str.chars() {
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(100, 100, 100));
                    ctx.buffer.set(x, area.y, cell);
                    x += 1;
                }
            }
            // Space
            ctx.buffer.set(x, area.y, Cell::new(' '));
            x += 1;
        }

        // Render data points
        for &value in data_slice {
            if x >= area.x + area.width {
                break;
            }

            let char_index = self.value_to_index(value, min, max);
            let ch = chars[char_index];

            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            if let Some(bg) = self.bg {
                cell.bg = Some(bg);
            }
            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }

        // Fill remaining space if data is shorter
        let remaining_data = available_width.saturating_sub(data_slice.len());
        for _ in 0..remaining_data {
            if x >= area.x + area.width {
                break;
            }
            let mut cell = Cell::new(chars[0]); // Lowest bar
            cell.fg = Some(Color::rgb(50, 50, 50));
            if let Some(bg) = self.bg {
                cell.bg = Some(bg);
            }
            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }
    }
}

impl_styled_view!(Sparkline);
impl_props_builders!(Sparkline);

/// Create a sparkline
pub fn sparkline<I>(data: I) -> Sparkline
where
    I: IntoIterator<Item = f64>,
{
    Sparkline::new(data)
}
