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

    /// The column the switch starts at: after a label on the left and its
    /// space, which then holds the left focus bracket; otherwise the first,
    /// or the second while focus is drawn, the first holding that bracket.
    fn switch_x(&self) -> u16 {
        match &self.label {
            Some(label) if self.label_left => {
                (crate::utils::display_width(label) as u16).saturating_add(1)
            }
            _ => u16::from(self.shows_focus()),
        }
    }

    /// The columns `render` paints given room: the label and a space on
    /// whichever side it sits, the switch, and - while focused - the focus
    /// brackets on either side of it.
    fn natural_width(&self) -> u16 {
        let label = self.label.as_deref().map_or(0, |l| {
            (crate::utils::display_width(l) as u16).saturating_add(1)
        });
        let mut width = self.track_width().saturating_add(label);
        if self.shows_focus() {
            // The left bracket: in the space after a label on the left,
            // its own column otherwise
            if !(self.label_left && self.label.is_some()) {
                width = width.saturating_add(1);
            }
            // The right bracket: in the space before a label on the right,
            // its own column otherwise
            if self.label.is_none() || self.label_left {
                width = width.saturating_add(1);
            }
        }
        width
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

        let y: u16 = 0;

        // Render label if on left
        if self.label_left {
            if let Some(ref label) = self.label {
                let label_color = self.label_color(ctx);
                ctx.draw_text_clipped(0, y, label, label_color, area.width);
            }
        }
        let mut x = self.switch_x();

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

    /// The cyan brackets around the switch while focused, in the columns
    /// on either side of it (never over it)
    fn draw_focus_brackets(&self, ctx: &mut RenderContext, width: u16) {
        if !self.shows_focus() {
            return;
        }
        let switch_x = self.switch_x();
        let left_x = switch_x - 1; // one at least while focus is drawn
        ctx.set(left_x, 0, Cell::new('[').fg(Color::CYAN));
        let right_x = switch_x.saturating_add(self.track_width());
        if right_x < width {
            ctx.set(right_x, 0, Cell::new(']').fg(Color::CYAN));
        }
    }
}
