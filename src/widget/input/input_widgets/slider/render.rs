//! Drawing horizontal and vertical sliders and measuring them

use super::{Slider, SliderOrientation, SliderStyle};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG};
use crate::widget::traits::{RenderContext, View};

impl Slider {
    /// Format value for display
    ///
    /// Supports the following placeholders in `value_format`:
    /// - `{value}` - the current value (1 decimal place)
    /// - `{pct}` - the value as a percentage of the range (0 decimal places)
    /// - `{}` - the current value (1 decimal place, legacy placeholder)
    fn format_value(&self) -> String {
        if let Some(ref fmt) = self.value_format {
            fmt.replace("{value}", &format!("{:.1}", self.value))
                .replace("{pct}", &format!("{:.0}", self.normalized() * 100.0))
                .replace("{}", &format!("{:.1}", self.value))
        } else if self.step >= 1.0 || (self.step == 0.0 && self.max - self.min >= 10.0) {
            format!("{:.0}", self.value)
        } else {
            format!("{:.1}", self.value)
        }
    }

    /// Render horizontal slider
    fn render_horizontal(&self, ctx: &mut RenderContext) {
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the filled part of the track is the slider's primary element.
        let fill_color = self
            .fill_color
            .unwrap_or_else(|| ctx.css_color(Color::CYAN));
        let area = ctx.area;
        let mut x: u16 = 0;
        let y: u16 = 0;

        // Label
        if let Some(ref label) = self.label {
            let label_fg = if self.disabled {
                DISABLED_FG
            } else {
                Color::WHITE
            };
            ctx.put_str_with(x, y, label, area.width, |ch| Cell::new(ch).fg(label_fg));
            x += crate::utils::display_width(label) as u16 + 1;
        }

        let track_len = self.length.min(area.width.saturating_sub(x));
        let filled = (self.normalized() * track_len.saturating_sub(1) as f64).round() as u16;

        // Render based on style
        match self.style {
            SliderStyle::Block => {
                for i in 0..track_len {
                    let ch = if i <= filled { '█' } else { '░' };
                    let fg = if i <= filled {
                        fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
                    ctx.set(x + i, y, cell);
                }
            }
            SliderStyle::Line => {
                for i in 0..track_len {
                    let is_knob = i == filled;
                    let ch = if is_knob { '●' } else { '━' };
                    let fg = if is_knob {
                        self.knob_color
                    } else if i < filled {
                        fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
                    ctx.set(x + i, y, cell);
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
                    cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
                    ctx.set(x + i, y, cell);
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
                        fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
                    ctx.set(x + i, y, cell);
                }
            }
            SliderStyle::Dots => {
                for i in 0..track_len {
                    let ch = if i <= filled { '●' } else { '○' };
                    let fg = if i <= filled {
                        fill_color
                    } else {
                        self.track_color
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
                    ctx.set(x + i, y, cell);
                }
            }
        }

        x += track_len;

        // Value display
        if self.show_value {
            let value_str = self.format_value();
            x += 1;
            for (i, ch) in value_str.chars().enumerate() {
                if x + i as u16 >= area.width {
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
                ctx.set(x + i as u16, y, cell);
            }
        }

        // Tick marks
        if self.show_ticks && area.height > 1 {
            let tick_y = y + 1;
            for i in 0..self.tick_count {
                let tick_x = (self.label.as_ref().map(|l| l.len() + 1).unwrap_or(0) as u16)
                    + (i as f64 / (self.tick_count - 1) as f64 * track_len.saturating_sub(1) as f64)
                        as u16;
                if tick_x < area.width {
                    let mut cell = Cell::new('┴');
                    cell.fg = Some(self.track_color);
                    ctx.set(tick_x, tick_y, cell);
                }
            }
        }
    }

    /// Render vertical slider
    fn render_vertical(&self, ctx: &mut RenderContext) {
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the filled part of the track is the slider's primary element.
        let fill_color = self
            .fill_color
            .unwrap_or_else(|| ctx.css_color(Color::CYAN));
        let area = ctx.area;
        let x: u16 = 0;
        let track_len = self.length.min(area.height);
        let filled = (self.normalized() * track_len.saturating_sub(1) as f64).round() as u16;

        for i in 0..track_len {
            let from_bottom = track_len - 1 - i;
            let y = i;

            let (ch, fg) = match self.style {
                SliderStyle::Block => {
                    if from_bottom <= filled {
                        ('█', fill_color)
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
                                fill_color
                            } else {
                                self.track_color
                            },
                        )
                    }
                }
                SliderStyle::Gradient | SliderStyle::Dots => {
                    if from_bottom <= filled {
                        ('●', fill_color)
                    } else {
                        ('○', self.track_color)
                    }
                }
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
            ctx.set(x, y, cell);
        }

        // Value display
        if self.show_value && area.width > 2 {
            let value_str = self.format_value();
            let value_y = track_len / 2;
            for (i, ch) in value_str.chars().enumerate() {
                if x + 2 + i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if self.focused {
                    Color::CYAN
                } else {
                    Color::WHITE
                });
                ctx.set(x + 2 + i as u16, value_y, cell);
            }
        }
    }
}

impl View for Slider {
    crate::impl_view_meta!("Slider", focusable, disabled: direct);

    /// The track is [`length`](Slider::length) cells long, so the slider
    /// has a fixed size. Horizontal: the label and a space, the track, and a
    /// space and the value if shown, on one row (two with tick marks).
    /// Vertical: one column (plus a gap and the value if shown), `length`
    /// rows tall.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let value = || {
            crate::utils::unicode::display_width(&self.format_value()).min(u16::MAX as usize) as u16
        };
        let (w, h) = match self.orientation {
            SliderOrientation::Horizontal => {
                let label = self.label.as_deref().map_or(0, |l| {
                    (crate::utils::unicode::display_width(l) as u16).saturating_add(1)
                });
                let value = if self.show_value {
                    value().saturating_add(1)
                } else {
                    0
                };
                let ticks = u16::from(self.show_ticks && self.tick_count > 0);
                (
                    label.saturating_add(self.length).saturating_add(value),
                    1 + ticks,
                )
            }
            SliderOrientation::Vertical => {
                let w = if self.show_value {
                    value().saturating_add(2)
                } else {
                    1
                };
                (w, self.length)
            }
        };
        Some((w.min(max_width), h.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            SliderOrientation::Horizontal => self.render_horizontal(ctx),
            SliderOrientation::Vertical => self.render_vertical(ctx),
        }
    }
}
