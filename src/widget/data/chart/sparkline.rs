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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_sparkline_new() {
        let sl = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(sl.data.len(), 5);
    }

    #[test]
    fn test_sparkline_empty() {
        let sl = Sparkline::empty();
        assert!(sl.data.is_empty());
    }

    #[test]
    fn test_sparkline_builder() {
        let sl = Sparkline::new(vec![1.0, 2.0])
            .max(10.0)
            .min(0.0)
            .style(SparklineStyle::Ascii)
            .fg(Color::GREEN)
            .bg(Color::BLACK)
            .show_bounds(true);

        assert_eq!(sl.max, Some(10.0));
        assert_eq!(sl.min, Some(0.0));
        assert_eq!(sl.style, SparklineStyle::Ascii);
        assert_eq!(sl.fg, Some(Color::GREEN));
        assert_eq!(sl.bg, Some(Color::BLACK));
        assert!(sl.show_bounds);
    }

    #[test]
    fn test_sparkline_push() {
        let mut sl = Sparkline::new(vec![1.0, 2.0]);
        sl.push(3.0);
        assert_eq!(sl.data.len(), 3);
        assert_eq!(sl.data[2], 3.0);
    }

    #[test]
    fn test_sparkline_push_shift() {
        let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
        sl.push_shift(4.0, 3);
        assert_eq!(sl.data.len(), 3);
        assert_eq!(sl.data, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_sparkline_clear() {
        let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
        sl.clear();
        assert!(sl.data.is_empty());
    }

    #[test]
    fn test_sparkline_calc_bounds() {
        let sl = Sparkline::new(vec![1.0, 5.0, 3.0]);
        let (min, max) = sl.calc_bounds();
        assert_eq!(min, 0.0); // Default min is 0
        assert_eq!(max, 5.0);
    }

    #[test]
    fn test_sparkline_calc_bounds_custom() {
        let sl = Sparkline::new(vec![1.0, 5.0, 3.0]).min(-10.0).max(10.0);
        let (min, max) = sl.calc_bounds();
        assert_eq!(min, -10.0);
        assert_eq!(max, 10.0);
    }

    #[test]
    fn test_sparkline_value_to_index() {
        let sl = Sparkline::new(vec![0.0, 100.0]);

        // With bounds 0-100
        assert_eq!(sl.value_to_index(0.0, 0.0, 100.0), 0);
        assert_eq!(sl.value_to_index(100.0, 0.0, 100.0), 7);
        assert_eq!(sl.value_to_index(50.0, 0.0, 100.0), 4);
    }

    #[test]
    fn test_sparkline_styles() {
        let block = SparklineStyle::Block;
        assert_eq!(block.chars().len(), 8);
        assert_eq!(block.chars()[0], '▁');
        assert_eq!(block.chars()[7], '█');

        let ascii = SparklineStyle::Ascii;
        assert_eq!(ascii.chars()[0], '_');
        assert_eq!(ascii.chars()[7], '@');
    }

    #[test]
    fn test_sparkline_render() {
        let sl = Sparkline::new(vec![1.0, 4.0, 2.0, 8.0, 5.0, 3.0]);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(1, 1, 15, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sl.render(&mut ctx);
        // Should render sparkline bars
    }

    #[test]
    fn test_sparkline_render_with_bounds() {
        let sl = Sparkline::new(vec![1.0, 5.0, 3.0]).show_bounds(true);
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(1, 1, 25, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sl.render(&mut ctx);
        // Should render with bounds label
    }

    #[test]
    fn test_sparkline_helper() {
        let sl = sparkline(vec![1.0, 2.0, 3.0]);
        assert_eq!(sl.data.len(), 3);
    }

    #[test]
    fn test_sparkline_get_data() {
        let sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_sparkline_flat_data() {
        // All same values
        let sl = Sparkline::new(vec![5.0, 5.0, 5.0, 5.0]);
        let (min, max) = sl.calc_bounds();
        // Should handle flat data gracefully
        assert!(max > min);
    }

    // =========================================================================
    // SparklineStyle enum tests
    // =========================================================================

    #[test]
    fn test_sparkline_style_default() {
        assert_eq!(SparklineStyle::default(), SparklineStyle::Block);
    }

    #[test]
    fn test_sparkline_style_clone() {
        let style = SparklineStyle::Braille;
        assert_eq!(style, style.clone());
    }

    #[test]
    fn test_sparkline_style_copy() {
        let style1 = SparklineStyle::Ascii;
        let style2 = style1;
        assert_eq!(style1, SparklineStyle::Ascii);
        assert_eq!(style2, SparklineStyle::Ascii);
    }

    #[test]
    fn test_sparkline_style_equality() {
        assert_eq!(SparklineStyle::Block, SparklineStyle::Block);
        assert_eq!(SparklineStyle::Braille, SparklineStyle::Braille);
        assert_ne!(SparklineStyle::Block, SparklineStyle::Ascii);
    }

    #[test]
    fn test_sparkline_style_debug() {
        let debug_str = format!("{:?}", SparklineStyle::Block);
        assert!(debug_str.contains("Block"));
    }

    #[test]
    fn test_sparkline_style_chars_block() {
        let chars = SparklineStyle::Block.chars();
        assert_eq!(chars.len(), 8);
        assert_eq!(chars[0], '▁');
        assert_eq!(chars[7], '█');
    }

    #[test]
    fn test_sparkline_style_chars_braille() {
        let chars = SparklineStyle::Braille.chars();
        assert_eq!(chars.len(), 8);
        assert_eq!(chars[0], '⠀');
    }

    #[test]
    fn test_sparkline_style_chars_ascii() {
        let chars = SparklineStyle::Ascii.chars();
        assert_eq!(chars.len(), 8);
        assert_eq!(chars[0], '_');
        assert_eq!(chars[7], '@');
    }

    // =========================================================================
    // Sparkline struct tests
    // =========================================================================

    #[test]
    fn test_sparkline_default() {
        let sl = Sparkline::default();
        assert!(sl.data.is_empty());
    }

    #[test]
    fn test_sparkline_clone() {
        let sl1 = Sparkline::new(vec![1.0, 2.0, 3.0]);
        let sl2 = sl1.clone();
        assert_eq!(sl1.data, sl2.data);
    }

    #[test]
    fn test_sparkline_data_builder() {
        let sl = Sparkline::empty().data(vec![1.0, 2.0, 3.0]);
        assert_eq!(sl.data.len(), 3);
    }

    #[test]
    fn test_sparkline_data_from_iterator() {
        let data = vec![1.0, 2.0, 3.0];
        let sl = Sparkline::new(data.iter().copied());
        assert_eq!(sl.data.len(), 3);
    }

    #[test]
    fn test_sparkline_negative_values() {
        let sl = Sparkline::new(vec![-5.0, 0.0, 5.0]);
        let (min, max) = sl.calc_bounds();
        assert_eq!(min, -5.0);
        assert_eq!(max, 5.0);
    }

    #[test]
    fn test_sparkline_all_zeros() {
        let sl = Sparkline::new(vec![0.0, 0.0, 0.0]);
        let index = sl.value_to_index(0.0, -1.0, 1.0);
        // Should handle zero range gracefully
        assert!(index <= 7);
    }

    #[test]
    fn test_sparkline_clamp_to_range() {
        let sl = Sparkline::new(vec![0.0, 100.0]);
        // Value above max should clamp to 7
        assert_eq!(sl.value_to_index(200.0, 0.0, 100.0), 7);
        // Value below min should clamp to 0
        assert_eq!(sl.value_to_index(-50.0, 0.0, 100.0), 0);
    }

    #[test]
    fn test_sparkline_single_value() {
        let sl = Sparkline::new(vec![42.0]);
        let (min, max) = sl.calc_bounds();
        // Should create a range even with one value
        assert!(max > min);
    }

    #[test]
    fn test_sparkline_push_shift_exact_limit() {
        let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
        sl.push_shift(4.0, 3);
        assert_eq!(sl.data.len(), 3);
        assert_eq!(sl.data, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_sparkline_push_shift_under_limit() {
        let mut sl = Sparkline::new(vec![1.0, 2.0]);
        sl.push_shift(3.0, 5);
        assert_eq!(sl.data.len(), 3);
        assert_eq!(sl.data, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_sparkline_data_mutator() {
        let mut sl = Sparkline::empty();
        sl.data = vec![5.0, 4.0, 3.0];
        assert_eq!(sl.get_data(), &[5.0, 4.0, 3.0]);
    }

    #[test]
    fn test_sparkline_show_bounds_builder() {
        let sl = Sparkline::new(vec![1.0]).show_bounds(true);
        assert!(sl.show_bounds);
    }

    #[test]
    fn test_sparkline_builder_chain() {
        let sl = Sparkline::new(vec![1.0, 2.0, 3.0])
            .max(10.0)
            .min(0.0)
            .style(SparklineStyle::Braille)
            .fg(Color::RED)
            .bg(Color::WHITE)
            .show_bounds(true);

        assert_eq!(sl.max, Some(10.0));
        assert_eq!(sl.min, Some(0.0));
        assert_eq!(sl.style, SparklineStyle::Braille);
        assert_eq!(sl.fg, Some(Color::RED));
        assert_eq!(sl.bg, Some(Color::WHITE));
        assert!(sl.show_bounds);
    }

    #[test]
    fn test_sparkline_single_element() {
        let sl = Sparkline::new(vec![7.5]);
        assert_eq!(sl.data.len(), 1);
        assert_eq!(sl.data[0], 7.5);
    }

    #[test]
    fn test_sparkline_large_dataset() {
        let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let sl = Sparkline::new(data);
        assert_eq!(sl.data.len(), 1000);
    }

    #[test]
    fn test_sparkline_nan_handling() {
        let sl = Sparkline::new(vec![1.0, f64::NAN, 3.0]);
        // NaN should be handled in calculations
        let (min, _max) = sl.calc_bounds();
        // min/max should skip NaN
        assert!(min.is_finite());
    }

    #[test]
    fn test_sparkline_infinity_handling() {
        let sl = Sparkline::new(vec![1.0, f64::INFINITY, 3.0]);
        let (min, max) = sl.calc_bounds();
        // calc_bounds uses data_min.min(0.0), so min is 0.0 even when data_min is 1.0
        assert_eq!(min, 0.0);
        assert!(max.is_infinite());
    }
}
