//! Measuring and drawing: the focus marker, the box in its style, and the
//! label

use super::Checkbox;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{LIGHT_GRAY, SUBTLE_GRAY};
use crate::widget::traits::{RenderContext, View};

impl Checkbox {
    /// The columns `render` paints given room: the focus marker while
    /// focused, the box, and a space and the label when there is one.
    fn natural_width(&self) -> u16 {
        let focus = if self.state.focused && !self.state.disabled {
            2
        } else {
            0
        };
        let check = if self.style.brackets().is_some() {
            3
        } else {
            1
        };
        let label = if self.label.is_empty() {
            0
        } else {
            1 + crate::utils::unicode::display_width(&self.label)
        };
        (focus + check + label).min(u16::MAX as usize) as u16
    }
}

impl View for Checkbox {
    /// One row: the box and its label. Focus puts the two-column `> ` marker
    /// in front, as `render` does.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((self.natural_width().min(max_width), 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut x: u16 = 0;

        // Resolve colors with CSS cascade: disabled > widget override > CSS > default
        let label_fg = self.state.resolve_fg(ctx.style, Color::WHITE);

        let check_fg = self.check_color();

        // Render focus indicator
        if self.state.focused && !self.state.disabled {
            let mut cell = Cell::new('>');
            cell.fg = Some(Color::CYAN);
            ctx.set(x, 0, cell);
            x += 1;

            let space = Cell::new(' ');
            ctx.set(x, 0, space);
            x += 1;
        }

        // Render checkbox
        x = self.draw_box(ctx, x, check_fg, label_fg);

        // Space before label
        ctx.set(x, 0, Cell::new(' '));
        x += 1;

        // Render label, by terminal columns
        let bold = self.state.focused && !self.state.disabled;
        ctx.put_str_with(x, 0, &self.label, area.width, |ch| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(label_fg);
            if bold {
                cell.modifier = crate::render::Modifier::BOLD;
            }
            cell
        });
    }

    crate::impl_view_meta!("Checkbox", focusable, disabled: state);
}

impl Checkbox {
    /// The mark's color: grays when disabled, else the check color when
    /// checked, else the widget's fg
    fn check_color(&self) -> Color {
        if self.state.disabled {
            if self.checked {
                SUBTLE_GRAY // Brighter gray for disabled+checked
            } else {
                Color::rgb(70, 70, 70) // Darker gray for disabled+unchecked
            }
        } else if self.checked {
            self.check_fg.unwrap_or(Color::GREEN)
        } else {
            self.state.fg.unwrap_or(LIGHT_GRAY)
        }
    }

    /// The box at `x` in its style; returns the column after it
    fn draw_box(
        &self,
        ctx: &mut RenderContext,
        mut x: u16,
        check_fg: Color,
        label_fg: Color,
    ) -> u16 {
        let (checked_char, unchecked_char) = self.style.chars();
        let brackets = self.style.brackets();
        if let Some((left, right)) = brackets {
            // Square style: [x] or [ ]
            let mut left_cell = Cell::new(left);
            left_cell.fg = Some(label_fg);
            ctx.set(x, 0, left_cell);
            x += 1;

            let check_char = if self.checked {
                checked_char
            } else {
                unchecked_char
            };
            let mut check_cell = Cell::new(check_char);
            check_cell.fg = Some(check_fg);
            ctx.set(x, 0, check_cell);
            x += 1;

            let mut right_cell = Cell::new(right);
            right_cell.fg = Some(label_fg);
            ctx.set(x, 0, right_cell);
            x += 1;
        } else {
            // Unicode style: ☑ or ☐
            let check_char = if self.checked {
                checked_char
            } else {
                unchecked_char
            };
            let mut check_cell = Cell::new(check_char);
            check_cell.fg = Some(check_fg);
            ctx.set(x, 0, check_cell);
            x += 1;
        }
        x
    }
}
