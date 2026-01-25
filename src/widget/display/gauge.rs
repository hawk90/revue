//! Gauge widget for displaying metrics and progress
//!
//! Advanced progress indicators with various styles including
//! speedometer, arc, battery, and more.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Gauge style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GaugeStyle {
    /// Horizontal bar with segments
    #[default]
    Bar,
    /// Battery indicator
    Battery,
    /// Thermometer style
    Thermometer,
    /// Circular/arc (text-based)
    Arc,
    /// Percentage circle
    Circle,
    /// Vertical bar
    Vertical,
    /// Segmented blocks
    Segments,
    /// Dot indicator
    Dots,
}

/// Gauge label position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelPosition {
    /// No label
    None,
    /// Inside the gauge
    #[default]
    Inside,
    /// Left of gauge
    Left,
    /// Right of gauge
    Right,
    /// Above gauge
    Above,
    /// Below gauge
    Below,
}

/// Gauge widget
pub struct Gauge {
    /// Current value (0.0 - 1.0)
    value: f64,
    /// Minimum value for display
    min: f64,
    /// Maximum value for display
    max: f64,
    /// Visual style
    style: GaugeStyle,
    /// Width (for horizontal styles)
    width: u16,
    /// Height (for vertical styles)
    height: u16,
    /// Label format
    label: Option<String>,
    /// Label position
    label_position: LabelPosition,
    /// Show percentage
    show_percent: bool,
    /// Filled color
    fill_color: Color,
    /// Empty/track color
    empty_color: Color,
    /// Border color
    border_color: Option<Color>,
    /// Warning threshold (0.0-1.0)
    warning_threshold: Option<f64>,
    /// Critical threshold (0.0-1.0)
    critical_threshold: Option<f64>,
    /// Warning color
    warning_color: Color,
    /// Critical color
    critical_color: Color,
    /// Segments count (for segmented style)
    segments: u16,
    /// Title
    title: Option<String>,
    /// Widget properties
    props: WidgetProps,
}

impl Gauge {
    /// Create a new gauge
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            style: GaugeStyle::Bar,
            width: 20,
            height: 5,
            label: None,
            label_position: LabelPosition::Inside,
            show_percent: true,
            fill_color: Color::GREEN,
            empty_color: Color::rgb(60, 60, 60),
            border_color: None,
            warning_threshold: None,
            critical_threshold: None,
            warning_color: Color::YELLOW,
            critical_color: Color::RED,
            segments: 10,
            title: None,
            props: WidgetProps::new(),
        }
    }

    /// Set value (0.0 - 1.0)
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    /// Set value with custom range
    ///
    /// If min >= max, they will be swapped to ensure valid range.
    /// If min == max after potential swap, value defaults to 0.0.
    pub fn value_range(mut self, value: f64, min: f64, max: f64) -> Self {
        // Ensure min < max
        let (min, max) = if min < max { (min, max) } else { (max, min) };

        self.min = min;
        self.max = max;

        // Avoid division by zero
        let range = max - min;
        if range.abs() < f64::EPSILON {
            self.value = 0.0;
        } else {
            self.value = ((value - min) / range).clamp(0.0, 1.0);
        }
        self
    }

    /// Set percentage (0-100)
    pub fn percent(mut self, percent: f64) -> Self {
        self.value = (percent / 100.0).clamp(0.0, 1.0);
        self
    }

    /// Set style
    pub fn style(mut self, style: GaugeStyle) -> Self {
        self.style = style;
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(4);
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = height.max(1);
        self
    }

    /// Set custom label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set label position
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Show/hide percentage
    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    /// Set fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Set empty color
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = color;
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set thresholds for color changes
    ///
    /// Warning threshold should be less than critical threshold.
    /// If warning >= critical, they will be swapped.
    pub fn thresholds(mut self, warning: f64, critical: f64) -> Self {
        let warning = warning.clamp(0.0, 1.0);
        let critical = critical.clamp(0.0, 1.0);
        // Ensure warning < critical
        let (warning, critical) = if warning < critical {
            (warning, critical)
        } else {
            (critical, warning)
        };
        self.warning_threshold = Some(warning);
        self.critical_threshold = Some(critical);
        self
    }

    /// Set warning color
    pub fn warning_color(mut self, color: Color) -> Self {
        self.warning_color = color;
        self
    }

    /// Set critical color
    pub fn critical_color(mut self, color: Color) -> Self {
        self.critical_color = color;
        self
    }

    /// Set segments count
    pub fn segments(mut self, count: u16) -> Self {
        self.segments = count.max(2);
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Get current display color based on thresholds
    fn current_color(&self) -> Color {
        if let Some(critical) = self.critical_threshold {
            if self.value >= critical {
                return self.critical_color;
            }
        }
        if let Some(warning) = self.warning_threshold {
            if self.value >= warning {
                return self.warning_color;
            }
        }
        self.fill_color
    }

    /// Get label text
    fn get_label(&self) -> String {
        if let Some(ref label) = self.label {
            label.clone()
        } else if self.show_percent {
            format!("{:.0}%", self.value * 100.0)
        } else {
            let display_value = self.min + self.value * (self.max - self.min);
            format!("{:.0}", display_value)
        }
    }

    /// Update value
    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(0.0, 1.0);
    }

    /// Get current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Render bar style
    fn render_bar(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let width = self.width.min(area.width);
        let filled = (self.value * width as f64).round() as u16;
        let color = self.current_color();

        // Draw bar
        for x in 0..width {
            let ch = if x < filled { '█' } else { '░' };
            let fg = if x < filled { color } else { self.empty_color };

            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Draw label inside
        if matches!(self.label_position, LabelPosition::Inside) {
            let label = self.get_label();
            let label_x = area.x + (width.saturating_sub(label.len() as u16)) / 2;
            for (i, ch) in label.chars().enumerate() {
                let x = label_x + i as u16;
                if x < area.x + width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    cell.modifier |= Modifier::BOLD;
                    ctx.buffer.set(x, area.y, cell);
                }
            }
        }
    }

    /// Render battery style
    fn render_battery(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let width = self.width.min(area.width).max(6);
        let inner_width = width - 3; // Account for borders and cap
        let filled = (self.value * inner_width as f64).round() as u16;
        let color = self.current_color();

        // Battery body
        let mut left = Cell::new('[');
        left.fg = Some(Color::WHITE);
        ctx.buffer.set(area.x, area.y, left);

        for x in 0..inner_width {
            let ch = if x < filled { '█' } else { ' ' };
            let fg = if x < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x + 1 + x, area.y, cell);
        }

        let mut right = Cell::new(']');
        right.fg = Some(Color::WHITE);
        ctx.buffer.set(area.x + 1 + inner_width, area.y, right);

        // Battery cap
        let mut cap = Cell::new('▌');
        cap.fg = Some(Color::WHITE);
        ctx.buffer.set(area.x + 2 + inner_width, area.y, cap);
    }

    /// Render thermometer style
    fn render_thermometer(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.min(area.height).max(3);
        let filled = (self.value * (height - 1) as f64).round() as u16;
        let color = self.current_color();

        // Bulb at bottom
        let mut bulb = Cell::new('●');
        bulb.fg = Some(color);
        ctx.buffer.set(area.x, area.y + height - 1, bulb);

        // Tube
        for y in 0..height - 1 {
            let from_bottom = height - 2 - y;
            let ch = if from_bottom < filled { '█' } else { '│' };
            let fg = if from_bottom < filled {
                color
            } else {
                self.empty_color
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x, area.y + y, cell);
        }
    }

    /// Render arc style
    fn render_arc(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let color = self.current_color();

        // Simple text-based arc: ╭───────╮
        //                        │ 75%   │
        //                        ╰───────╯
        let width = self.width.min(area.width).max(8);

        // Top arc
        let mut tl = Cell::new('╭');
        tl.fg = Some(color);
        ctx.buffer.set(area.x, area.y, tl);

        for x in 1..width - 1 {
            let progress = (x - 1) as f64 / (width - 3) as f64;
            let ch = if progress <= self.value { '━' } else { '─' };
            let fg = if progress <= self.value {
                color
            } else {
                self.empty_color
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        let mut tr = Cell::new('╮');
        tr.fg = Some(color);
        ctx.buffer.set(area.x + width - 1, area.y, tr);

        // Middle with label
        if area.height > 1 {
            let label = self.get_label();
            let label_x = area.x + (width.saturating_sub(label.len() as u16)) / 2;

            let mut left = Cell::new('│');
            left.fg = Some(color);
            ctx.buffer.set(area.x, area.y + 1, left);

            for (i, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(label_x + i as u16, area.y + 1, cell);
            }

            let mut right = Cell::new('│');
            right.fg = Some(color);
            ctx.buffer.set(area.x + width - 1, area.y + 1, right);
        }

        // Bottom arc
        if area.height > 2 {
            let mut bl = Cell::new('╰');
            bl.fg = Some(color);
            ctx.buffer.set(area.x, area.y + 2, bl);

            for x in 1..width - 1 {
                let mut cell = Cell::new('─');
                cell.fg = Some(self.empty_color);
                ctx.buffer.set(area.x + x, area.y + 2, cell);
            }

            let mut br = Cell::new('╯');
            br.fg = Some(color);
            ctx.buffer.set(area.x + width - 1, area.y + 2, br);
        }
    }

    /// Render circle style (text-based)
    fn render_circle(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let color = self.current_color();

        // Braille-based circle approximation
        // ⠀⢀⣴⣾⣿⣷⣦⡀⠀
        // ⠀⣿⣿⣿⣿⣿⣿⣿⠀
        // ⠀⠻⣿⣿⣿⣿⣿⠟⠀

        let label = self.get_label();

        // Simple representation: (●●●○○) 60%
        let segments = 5u16;
        let filled = (self.value * segments as f64).round() as u16;

        let mut open = Cell::new('(');
        open.fg = Some(Color::WHITE);
        ctx.buffer.set(area.x, area.y, open);

        for i in 0..segments {
            let ch = if i < filled { '●' } else { '○' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x + 1 + i, area.y, cell);
        }

        let mut close = Cell::new(')');
        close.fg = Some(Color::WHITE);
        ctx.buffer.set(area.x + 1 + segments, area.y, close);

        // Label
        let label_x = area.x + 3 + segments;
        for (i, ch) in label.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.buffer.set(label_x + i as u16, area.y, cell);
        }
    }

    /// Render vertical style
    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.min(area.height);
        let filled = (self.value * height as f64).round() as u16;
        let color = self.current_color();

        for y in 0..height {
            let from_bottom = height - 1 - y;
            let ch = if from_bottom < filled { '█' } else { '░' };
            let fg = if from_bottom < filled {
                color
            } else {
                self.empty_color
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x, area.y + y, cell);
        }
    }

    /// Render segments style
    fn render_segments(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let segments = self.segments.min(area.width / 2);
        let filled = (self.value * segments as f64).round() as u16;
        let color = self.current_color();

        for i in 0..segments {
            let ch = if i < filled { '▰' } else { '▱' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x + i * 2, area.y, cell);
        }
    }

    /// Render dots style
    fn render_dots(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let dots = self.segments.min(area.width);
        let filled = (self.value * dots as f64).round() as u16;
        let color = self.current_color();

        for i in 0..dots {
            let ch = if i < filled { '●' } else { '○' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.buffer.set(area.x + i, area.y, cell);
        }
    }
}

impl Default for Gauge {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Gauge {
    crate::impl_view_meta!("Gauge");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Draw title if present
        let mut y_offset = 0u16;
        if let Some(ref title) = self.title {
            for (i, ch) in title.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(area.x + i as u16, area.y, cell);
            }
            y_offset = 1;
        }

        let adjusted_area = crate::layout::Rect::new(
            area.x,
            area.y + y_offset,
            area.width,
            area.height.saturating_sub(y_offset),
        );

        let mut adjusted_ctx = RenderContext::new(ctx.buffer, adjusted_area);

        match self.style {
            GaugeStyle::Bar => self.render_bar(&mut adjusted_ctx),
            GaugeStyle::Battery => self.render_battery(&mut adjusted_ctx),
            GaugeStyle::Thermometer => self.render_thermometer(&mut adjusted_ctx),
            GaugeStyle::Arc => self.render_arc(&mut adjusted_ctx),
            GaugeStyle::Circle => self.render_circle(&mut adjusted_ctx),
            GaugeStyle::Vertical => self.render_vertical(&mut adjusted_ctx),
            GaugeStyle::Segments => self.render_segments(&mut adjusted_ctx),
            GaugeStyle::Dots => self.render_dots(&mut adjusted_ctx),
        }
    }
}

impl_styled_view!(Gauge);
impl_props_builders!(Gauge);

/// Helper to create a gauge
pub fn gauge() -> Gauge {
    Gauge::new()
}

/// Helper to create a percentage gauge
pub fn percentage(value: f64) -> Gauge {
    Gauge::new().percent(value)
}

/// Helper to create a battery gauge
pub fn battery(level: f64) -> Gauge {
    Gauge::new()
        .percent(level)
        .style(GaugeStyle::Battery)
        .thresholds(0.5, 0.2)
}

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauge_new() {
        let g = Gauge::new();
        assert_eq!(g.value, 0.0);
    }

    #[test]
    fn test_gauge_value() {
        let g = Gauge::new().value(0.5);
        assert_eq!(g.value, 0.5);
    }

    #[test]
    fn test_gauge_percent() {
        let g = Gauge::new().percent(75.0);
        assert_eq!(g.value, 0.75);
    }

    #[test]
    fn test_gauge_value_clamp() {
        let g1 = Gauge::new().value(1.5);
        assert_eq!(g1.value, 1.0);

        let g2 = Gauge::new().value(-0.5);
        assert_eq!(g2.value, 0.0);
    }

    #[test]
    fn test_gauge_value_range() {
        let g = Gauge::new().value_range(50.0, 0.0, 100.0);
        assert_eq!(g.value, 0.5);
    }

    #[test]
    fn test_gauge_style() {
        let g = Gauge::new().style(GaugeStyle::Battery);
        assert!(matches!(g.style, GaugeStyle::Battery));
    }

    #[test]
    fn test_gauge_thresholds() {
        let g = Gauge::new().thresholds(0.7, 0.9).value(0.95);

        assert_eq!(g.current_color(), g.critical_color);
    }

    #[test]
    fn test_gauge_warning_color() {
        let g = Gauge::new().thresholds(0.7, 0.9).value(0.75);

        assert_eq!(g.current_color(), g.warning_color);
    }

    #[test]
    fn test_gauge_normal_color() {
        let g = Gauge::new().thresholds(0.7, 0.9).value(0.5);

        assert_eq!(g.current_color(), g.fill_color);
    }

    #[test]
    fn test_gauge_get_label() {
        let g = Gauge::new().percent(50.0);
        assert_eq!(g.get_label(), "50%");
    }

    #[test]
    fn test_gauge_custom_label() {
        let g = Gauge::new().label("Custom");
        assert_eq!(g.get_label(), "Custom");
    }

    #[test]
    fn test_gauge_helper_value() {
        let g = gauge().percent(50.0);
        assert_eq!(g.value, 0.5);
    }

    #[test]
    fn test_percentage_helper_value() {
        let g = percentage(75.0);
        assert_eq!(g.value, 0.75);
    }

    #[test]
    fn test_battery_helper_fields() {
        let g = battery(80.0);
        assert!(matches!(g.style, GaugeStyle::Battery));
        assert_eq!(g.value, 0.8);
    }

    #[test]
    fn test_value_range_validation() {
        // Normal case
        let g = Gauge::new().value_range(50.0, 0.0, 100.0);
        assert_eq!(g.value, 0.5);

        // Swapped min/max
        let g = Gauge::new().value_range(50.0, 100.0, 0.0);
        assert_eq!(g.min, 0.0);
        assert_eq!(g.max, 100.0);
        assert_eq!(g.value, 0.5);

        // Equal min/max (division by zero case)
        let g = Gauge::new().value_range(50.0, 50.0, 50.0);
        assert_eq!(g.value, 0.0);
    }

    #[test]
    fn test_thresholds_validation() {
        // Normal case
        let g = Gauge::new().thresholds(0.5, 0.8);
        assert_eq!(g.warning_threshold, Some(0.5));
        assert_eq!(g.critical_threshold, Some(0.8));

        // Swapped thresholds
        let g = Gauge::new().thresholds(0.8, 0.5);
        assert_eq!(g.warning_threshold, Some(0.5));
        assert_eq!(g.critical_threshold, Some(0.8));
    }
}
