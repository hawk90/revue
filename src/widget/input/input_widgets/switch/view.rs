//! Measuring and drawing: the label on either side, the switch in its
//! style, and the focus brackets

use super::{Switch, SwitchStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::{RenderContext, View};

impl Switch {
    /// Columns the switch itself takes, without its label.
    fn track_width(&self) -> u16 {
        match self.style {
            SwitchStyle::Text => {
                let text = if self.on {
                    self.on_text.as_deref().unwrap_or("ON")
                } else {
                    self.off_text.as_deref().unwrap_or("OFF")
                };
                (crate::utils::display_width(text) as u16).saturating_add(2)
            }
            SwitchStyle::Emoji => 2,
            _ => self.width,
        }
    }

    /// The columns `render` paints given room: the label and a space on
    /// whichever side it sits, the switch, and - while focused - the focus
    /// bracket `render` puts in the column after the switch.
    fn natural_width(&self) -> u16 {
        let label = self.label.as_deref().map_or(0, |l| {
            (crate::utils::display_width(l) as u16).saturating_add(1)
        });
        let width = self.track_width().saturating_add(label);
        // The right bracket lands just after the switch: inside the gap
        // before a label on the right, past the end otherwise.
        let bracket_outside = self.label.is_none() || self.label_left;
        if self.focused && !self.disabled && bracket_outside {
            width.saturating_add(1)
        } else {
            width
        }
    }
}

impl View for Switch {
    /// One row: the switch and its label (see `natural_width`).
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((self.natural_width().min(max_width), 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut x: u16 = 0;
        let y: u16 = 0;

        // Render label if on left
        if self.label_left {
            if let Some(ref label) = self.label {
                let label_color = self.label_color(ctx);
                ctx.draw_text_clipped(x, y, label, label_color, area.width.saturating_sub(x));
                x += crate::utils::display_width(label) as u16 + 1;
            }
        }

        // Render switch
        if x < area.width {
            match self.style {
                SwitchStyle::Default => self.render_default(ctx, x, y),
                SwitchStyle::IOS => self.render_ios(ctx, x, y),
                SwitchStyle::Material => self.render_material(ctx, x, y),
                SwitchStyle::Text => self.render_text(ctx, x, y),
                SwitchStyle::Emoji => self.render_emoji(ctx, x, y),
                SwitchStyle::Block => self.render_block(ctx, x, y),
            }

            x += self.track_width();
        }

        // Render label if on right
        if !self.label_left {
            if let Some(ref label) = self.label {
                x += 1;
                let label_color = self.label_color(ctx);
                ctx.draw_text_clipped(x, y, label, label_color, area.width.saturating_sub(x));
            }
        }

        self.draw_focus_brackets(ctx, area.width);
    }

    crate::impl_view_meta!("Switch", focusable, disabled: direct);
}

impl Switch {
    /// The label's color: gray when disabled, else the stylesheet's
    /// `color`, else white
    fn label_color(&self, ctx: &RenderContext) -> Color {
        if self.disabled {
            DISABLED_FG
        } else if let Some(css_fg) = ctx
            .style
            .map(|s| s.visual.color)
            .filter(|c| *c != Color::default())
        {
            css_fg
        } else {
            Color::WHITE
        }
    }

    /// The cyan brackets around the switch while focused
    fn draw_focus_brackets(&self, ctx: &mut RenderContext, width: u16) {
        let y: u16 = 0;
        // Render focus indicator
        if self.focused && !self.disabled {
            // Find switch start position (relative)
            let switch_x = if self.label_left {
                self.label
                    .as_ref()
                    .map(|l| crate::utils::display_width(l) as u16 + 1)
                    .unwrap_or(0)
            } else {
                0u16
            };

            // Draw focus bracket on left
            if switch_x > 0 {
                let mut left = Cell::new('[');
                left.fg = Some(Color::CYAN);
                ctx.set(switch_x.saturating_sub(1), y, left);
            } else {
                let mut left = Cell::new('[');
                left.fg = Some(Color::CYAN);
                ctx.set(0, y, left);
            }

            // Draw focus bracket on right
            let switch_width = self.track_width();
            let right_x = switch_x + switch_width;
            if right_x < width {
                let mut right = Cell::new(']');
                right.fg = Some(Color::CYAN);
                ctx.set(right_x, y, right);
            }
        }
    }
}
