//! Slider widget for selecting numeric values
//!
//! Provides horizontal and vertical sliders with customizable
//! ranges, steps, and visual styles.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Slider orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderOrientation {
    /// Horizontal slider
    #[default]
    Horizontal,
    /// Vertical slider
    Vertical,
}

/// Slider style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderStyle {
    /// Default block style
    #[default]
    Block,
    /// Line style with knob
    Line,
    /// Thin line
    Thin,
    /// Gradient fill
    Gradient,
    /// Dots
    Dots,
}

/// Slider widget
pub struct Slider {
    /// Current value
    value: f64,
    /// Minimum value
    min: f64,
    /// Maximum value
    max: f64,
    /// Step size (0 = continuous)
    step: f64,
    /// Orientation
    orientation: SliderOrientation,
    /// Visual style
    style: SliderStyle,
    /// Length (width for horizontal, height for vertical)
    length: u16,
    /// Show value label
    show_value: bool,
    /// Value format string
    value_format: Option<String>,
    /// Track color
    track_color: Color,
    /// Fill color
    fill_color: Color,
    /// Knob color
    knob_color: Color,
    /// Focused state
    focused: bool,
    /// Disabled state
    disabled: bool,
    /// Label
    label: Option<String>,
    /// Show tick marks
    show_ticks: bool,
    /// Number of ticks
    tick_count: u16,
    /// Widget properties
    props: WidgetProps,
}

impl Slider {
    /// Create a new slider
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 0.0,
            orientation: SliderOrientation::Horizontal,
            style: SliderStyle::Block,
            length: 20,
            show_value: true,
            value_format: None,
            track_color: Color::rgb(60, 60, 60),
            fill_color: Color::CYAN,
            knob_color: Color::WHITE,
            focused: false,
            disabled: false,
            label: None,
            show_ticks: false,
            tick_count: 5,
            props: WidgetProps::new(),
        }
    }

    /// Set value
    pub fn value(mut self, value: f64) -> Self {
        self.value = self.clamp_value(value);
        self
    }

    /// Set range
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self.value = self.clamp_value(self.value);
        self
    }

    /// Set step size
    pub fn step(mut self, step: f64) -> Self {
        self.step = step.abs();
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Make horizontal
    pub fn horizontal(mut self) -> Self {
        self.orientation = SliderOrientation::Horizontal;
        self
    }

    /// Make vertical
    pub fn vertical(mut self) -> Self {
        self.orientation = SliderOrientation::Vertical;
        self
    }

    /// Set style
    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    /// Set length
    pub fn length(mut self, length: u16) -> Self {
        self.length = length.max(3);
        self
    }

    /// Show/hide value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set value format
    pub fn value_format(mut self, format: impl Into<String>) -> Self {
        self.value_format = Some(format.into());
        self
    }

    /// Set track color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Set fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Set knob color
    pub fn knob_color(mut self, color: Color) -> Self {
        self.knob_color = color;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show tick marks
    pub fn ticks(mut self, count: u16) -> Self {
        self.show_ticks = true;
        self.tick_count = count.max(2);
        self
    }

    /// Clamp value to range and step
    fn clamp_value(&self, value: f64) -> f64 {
        let clamped = value.clamp(self.min, self.max);
        if self.step > 0.0 {
            let steps = ((clamped - self.min) / self.step).round();
            (self.min + steps * self.step).clamp(self.min, self.max)
        } else {
            clamped
        }
    }

    /// Get normalized value (0.0 - 1.0)
    fn normalized(&self) -> f64 {
        if (self.max - self.min).abs() < f64::EPSILON {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    /// Set value
    pub fn set_value(&mut self, value: f64) {
        self.value = self.clamp_value(value);
    }

    /// Get current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Increment value
    pub fn increment(&mut self) {
        let step = if self.step > 0.0 {
            self.step
        } else {
            (self.max - self.min) / 100.0
        };
        self.value = self.clamp_value(self.value + step);
    }

    /// Decrement value
    pub fn decrement(&mut self) {
        let step = if self.step > 0.0 {
            self.step
        } else {
            (self.max - self.min) / 100.0
        };
        self.value = self.clamp_value(self.value - step);
    }

    /// Set to minimum
    pub fn set_min(&mut self) {
        self.value = self.min;
    }

    /// Set to maximum
    pub fn set_max(&mut self) {
        self.value = self.max;
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if self.disabled || !self.focused {
            return false;
        }

        match (&self.orientation, key) {
            (SliderOrientation::Horizontal, Key::Right | Key::Char('l'))
            | (SliderOrientation::Vertical, Key::Up | Key::Char('k')) => {
                self.increment();
                true
            }
            (SliderOrientation::Horizontal, Key::Left | Key::Char('h'))
            | (SliderOrientation::Vertical, Key::Down | Key::Char('j')) => {
                self.decrement();
                true
            }
            (_, Key::Home) => {
                self.set_min();
                true
            }
            (_, Key::End) => {
                self.set_max();
                true
            }
            _ => false,
        }
    }

    /// Format value for display
    fn format_value(&self) -> String {
        if let Some(ref fmt) = self.value_format {
            fmt.replace("{}", &format!("{:.1}", self.value))
        } else if self.step >= 1.0 || self.step == 0.0 && self.max - self.min >= 10.0 {
            format!("{:.0}", self.value)
        } else {
            format!("{:.1}", self.value)
        }
    }

    /// Render horizontal slider
    fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let mut x = area.x;
        let y = area.y;

        // Label
        if let Some(ref label) = self.label {
            for (i, ch) in label.chars().enumerate() {
                if x + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if self.disabled {
                    Color::rgb(100, 100, 100)
                } else {
                    Color::WHITE
                });
                ctx.buffer.set(x + i as u16, y, cell);
            }
            x += label.len() as u16 + 1;
        }

        let track_len = self.length.min(area.width.saturating_sub(x - area.x));
        let filled = (self.normalized() * (track_len - 1) as f64).round() as u16;

        // Render based on style
        match self.style {
            SliderStyle::Block => {
                for i in 0..track_len {
                    let ch = if i <= filled { '█' } else { '░' };
                    let fg = if i <= filled {
                        self.fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(80, 80, 80)
                    } else {
                        fg
                    });
                    ctx.buffer.set(x + i, y, cell);
                }
            }
            SliderStyle::Line => {
                for i in 0..track_len {
                    let is_knob = i == filled;
                    let ch = if is_knob { '●' } else { '━' };
                    let fg = if is_knob {
                        self.knob_color
                    } else if i < filled {
                        self.fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(80, 80, 80)
                    } else {
                        fg
                    });
                    ctx.buffer.set(x + i, y, cell);
                }
            }
            SliderStyle::Thin => {
                for i in 0..track_len {
                    let is_knob = i == filled;
                    let ch = if is_knob { '┃' } else { '─' };
                    let fg = if is_knob {
                        self.knob_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(80, 80, 80)
                    } else {
                        fg
                    });
                    ctx.buffer.set(x + i, y, cell);
                }
            }
            SliderStyle::Gradient => {
                let blocks = ['░', '▒', '▓', '█'];
                for i in 0..track_len {
                    let progress = i as f64 / track_len as f64;
                    let block_idx = if progress <= self.normalized() {
                        ((progress / self.normalized()) * 3.0).min(3.0) as usize
                    } else {
                        0
                    };
                    let ch = if i as f64 / track_len as f64 <= self.normalized() {
                        blocks[block_idx.min(3)]
                    } else {
                        '░'
                    };
                    let fg = if i as f64 / track_len as f64 <= self.normalized() {
                        self.fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(80, 80, 80)
                    } else {
                        fg
                    });
                    ctx.buffer.set(x + i, y, cell);
                }
            }
            SliderStyle::Dots => {
                for i in 0..track_len {
                    let ch = if i <= filled { '●' } else { '○' };
                    let fg = if i <= filled {
                        self.fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled {
                        Color::rgb(80, 80, 80)
                    } else {
                        fg
                    });
                    ctx.buffer.set(x + i, y, cell);
                }
            }
        }

        x += track_len;

        // Value display
        if self.show_value {
            let value_str = self.format_value();
            x += 1;
            for (i, ch) in value_str.chars().enumerate() {
                if x + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if self.focused {
                    Color::CYAN
                } else {
                    Color::WHITE
                });
                if self.focused {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x + i as u16, y, cell);
            }
        }

        // Tick marks
        if self.show_ticks && area.height > 1 {
            let tick_y = y + 1;
            for i in 0..self.tick_count {
                let tick_x = area.x
                    + (self.label.as_ref().map(|l| l.len() + 1).unwrap_or(0) as u16)
                    + (i as f64 / (self.tick_count - 1) as f64 * (track_len - 1) as f64) as u16;
                if tick_x < area.x + area.width {
                    let mut cell = Cell::new('┴');
                    cell.fg = Some(self.track_color);
                    ctx.buffer.set(tick_x, tick_y, cell);
                }
            }
        }
    }

    /// Render vertical slider
    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let x = area.x;
        let track_len = self.length.min(area.height);
        let filled = (self.normalized() * (track_len - 1) as f64).round() as u16;

        for i in 0..track_len {
            let from_bottom = track_len - 1 - i;
            let y = area.y + i;

            let (ch, fg) = match self.style {
                SliderStyle::Block => {
                    if from_bottom <= filled {
                        ('█', self.fill_color)
                    } else {
                        ('░', self.track_color)
                    }
                }
                SliderStyle::Line | SliderStyle::Thin => {
                    if from_bottom == filled {
                        ('●', self.knob_color)
                    } else {
                        (
                            '│',
                            if from_bottom < filled {
                                self.fill_color
                            } else {
                                self.track_color
                            },
                        )
                    }
                }
                SliderStyle::Gradient | SliderStyle::Dots => {
                    if from_bottom <= filled {
                        ('●', self.fill_color)
                    } else {
                        ('○', self.track_color)
                    }
                }
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(if self.disabled {
                Color::rgb(80, 80, 80)
            } else {
                fg
            });
            ctx.buffer.set(x, y, cell);
        }

        // Value display
        if self.show_value && area.width > 2 {
            let value_str = self.format_value();
            let value_y = area.y + track_len / 2;
            for (i, ch) in value_str.chars().enumerate() {
                if x + 2 + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if self.focused {
                    Color::CYAN
                } else {
                    Color::WHITE
                });
                ctx.buffer.set(x + 2 + i as u16, value_y, cell);
            }
        }
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Slider {
    crate::impl_view_meta!("Slider");

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            SliderOrientation::Horizontal => self.render_horizontal(ctx),
            SliderOrientation::Vertical => self.render_vertical(ctx),
        }
    }
}

impl_styled_view!(Slider);
impl_props_builders!(Slider);

/// Helper to create a slider
pub fn slider() -> Slider {
    Slider::new()
}

/// Helper to create a slider with range
pub fn slider_range(min: f64, max: f64) -> Slider {
    Slider::new().range(min, max)
}

/// Helper to create a percentage slider
pub fn percentage_slider() -> Slider {
    Slider::new().range(0.0, 100.0).value_format("{}%")
}

/// Helper to create a volume slider
pub fn volume_slider() -> Slider {
    Slider::new()
        .range(0.0, 100.0)
        .label("Vol")
        .style(SliderStyle::Block)
}

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_new() {
        let s = Slider::new();
        assert_eq!(s.value, 0.0);
        assert_eq!(s.min, 0.0);
        assert_eq!(s.max, 100.0);
    }

    #[test]
    fn test_slider_value() {
        let s = Slider::new().value(50.0);
        assert_eq!(s.value, 50.0);
    }

    #[test]
    fn test_slider_range() {
        let s = Slider::new().range(10.0, 20.0).value(15.0);
        assert_eq!(s.min, 10.0);
        assert_eq!(s.max, 20.0);
        assert_eq!(s.value, 15.0);
    }

    #[test]
    fn test_slider_clamp() {
        let s = Slider::new().range(0.0, 100.0).value(150.0);
        assert_eq!(s.value, 100.0);

        let s = Slider::new().range(0.0, 100.0).value(-50.0);
        assert_eq!(s.value, 0.0);
    }

    #[test]
    fn test_slider_step() {
        let s = Slider::new().range(0.0, 100.0).step(10.0).value(25.0);
        // Should snap to nearest step (30.0)
        assert!((s.value - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_slider_normalized() {
        let s = Slider::new().range(0.0, 100.0).value(50.0);
        assert!((s.normalized() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_slider_orientation() {
        let h = Slider::new().horizontal();
        assert!(matches!(h.orientation, SliderOrientation::Horizontal));

        let v = Slider::new().vertical();
        assert!(matches!(v.orientation, SliderOrientation::Vertical));
    }

    #[test]
    fn test_slider_helpers() {
        let s = slider().value(50.0);
        assert_eq!(s.value, 50.0);

        let s = slider_range(0.0, 10.0);
        assert_eq!(s.max, 10.0);

        let s = percentage_slider().value(75.0);
        assert!(s.value_format.is_some());

        let s = volume_slider();
        assert!(s.label.is_some());
    }

    #[test]
    fn test_slider_format_value() {
        let s = Slider::new().value(50.0);
        assert_eq!(s.format_value(), "50");

        let s = Slider::new().value(50.5).step(0.1);
        assert_eq!(s.format_value(), "50.5");

        let s = Slider::new().value(50.0).value_format("{}%");
        assert_eq!(s.format_value(), "50.0%");
    }

    #[test]
    fn test_slider_ticks() {
        let s = Slider::new().ticks(5);
        assert!(s.show_ticks);
        assert_eq!(s.tick_count, 5);
    }
}
