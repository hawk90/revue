//! Drawing a horizontal slider: the label, the track, the value and the
//! tick marks

use super::{Slider, SliderStyle};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, DISABLED_FG};
use crate::widget::traits::RenderContext;

impl Slider {
    /// Render horizontal slider
    pub(super) fn render_horizontal(&self, ctx: &mut RenderContext) {
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

        // The track, one glyph per cell in the style
        for i in 0..track_len {
            let (ch, fg) = self.horizontal_glyph(i, filled, track_len, fill_color);
            let mut cell = Cell::new(ch);
            cell.fg = Some(if self.disabled { DARK_GRAY } else { fg });
            ctx.set(x + i, y, cell);
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
    /// The glyph and color of cell `i` of a `track_len`-cell horizontal
    /// track whose knob or fill ends at cell `filled`
    fn horizontal_glyph(
        &self,
        i: u16,
        filled: u16,
        track_len: u16,
        fill_color: Color,
    ) -> (char, Color) {
        match self.style {
            SliderStyle::Block => {
                let ch = if i <= filled { '█' } else { '░' };
                let fg = if i <= filled {
                    fill_color
                } else {
                    self.track_color
                };
                (ch, fg)
            }
            SliderStyle::Line => {
                let is_knob = i == filled;
                let ch = if is_knob { '●' } else { '━' };
                let fg = if is_knob {
                    self.knob_color
                } else if i < filled {
                    fill_color
                } else {
                    self.track_color
                };
                (ch, fg)
            }
            SliderStyle::Thin => {
                let is_knob = i == filled;
                let ch = if is_knob { '┃' } else { '─' };
                let fg = if is_knob {
                    self.knob_color
                } else {
                    self.track_color
                };
                (ch, fg)
            }
            SliderStyle::Gradient => {
                let blocks = ['░', '▒', '▓', '█'];
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
                (ch, fg)
            }
            SliderStyle::Dots => {
                let ch = if i <= filled { '●' } else { '○' };
                let fg = if i <= filled {
                    fill_color
                } else {
                    self.track_color
                };
                (ch, fg)
            }
        }
    }
}
