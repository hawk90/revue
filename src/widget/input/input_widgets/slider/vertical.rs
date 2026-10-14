//! Drawing a vertical slider: the track from the bottom, and the value

use super::{Slider, SliderStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::RenderContext;

impl Slider {
    /// Render vertical slider
    pub(super) fn render_vertical(&self, ctx: &mut RenderContext) {
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
