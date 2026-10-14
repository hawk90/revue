//! The value label, measuring, and the View impl

use super::{Slider, SliderOrientation};
use crate::widget::traits::{RenderContext, View};

impl Slider {
    /// Format value for display
    ///
    /// Supports the following placeholders in `value_format`:
    /// - `{value}` - the current value (1 decimal place)
    /// - `{pct}` - the value as a percentage of the range (0 decimal places)
    /// - `{}` - the current value (1 decimal place, legacy placeholder)
    pub(super) fn format_value(&self) -> String {
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
