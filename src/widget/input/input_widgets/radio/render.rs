//! Drawing the options and measuring the group

use super::{RadioGroup, RadioLayout};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View};

impl RadioGroup {
    /// Render a single radio option (x, y are relative coordinates)
    fn render_option(&self, ctx: &mut RenderContext, index: usize, x: u16, y: u16) -> u16 {
        let area = ctx.area;
        if x >= area.width || y >= area.height {
            return 0;
        }

        let is_selected = self.selection.is_selected(index);
        let (selected_char, unselected_char) = self.style.chars();
        let (left_bracket, right_bracket) = self.style.brackets();
        let has_brackets = self.style.has_brackets();

        let label_fg = if self.disabled {
            DISABLED_FG
        } else {
            self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE))
        };

        let indicator_fg = if self.disabled {
            DISABLED_FG
        } else if is_selected {
            self.selected_fg.unwrap_or(Color::CYAN)
        } else {
            self.fg.unwrap_or_else(|| ctx.css_color(LIGHT_GRAY))
        };

        let mut current_x = x;

        // Render indicator
        if has_brackets {
            let mut left_cell = Cell::new(left_bracket);
            left_cell.fg = Some(label_fg);
            ctx.set(current_x, y, left_cell);
            current_x += 1;

            let indicator = if is_selected {
                selected_char
            } else {
                unselected_char
            };
            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(indicator_fg);
            ctx.set(current_x, y, ind_cell);
            current_x += 1;

            let mut right_cell = Cell::new(right_bracket);
            right_cell.fg = Some(label_fg);
            ctx.set(current_x, y, right_cell);
            current_x += 1;
        } else {
            let indicator = if is_selected {
                selected_char
            } else {
                unselected_char
            };
            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(indicator_fg);
            ctx.set(current_x, y, ind_cell);
            current_x += 1;
        }

        // Space before label
        ctx.set(current_x, y, Cell::new(' '));
        current_x += 1;

        // Render label, by terminal columns
        if let Some(option) = self.options.get(index) {
            let bold = is_selected && self.focused && !self.disabled;
            current_x += ctx.put_str_with(current_x, y, option, area.width, |ch| {
                let mut cell = Cell::new(ch);
                cell.fg = Some(label_fg);
                if bold {
                    cell.modifier = crate::render::Modifier::BOLD;
                }
                cell
            });
        }

        current_x - x
    }
}

impl RadioGroup {
    /// Columns one option takes: the indicator, a space and its label.
    fn option_width(&self, option: &str) -> usize {
        let indicator = if self.style.has_brackets() { 3 } else { 1 };
        indicator + 1 + crate::utils::unicode::display_width(option)
    }
}

impl View for RadioGroup {
    crate::impl_view_meta!("RadioGroup", focusable, disabled: direct);

    /// Vertical: one row per option (plus the gaps), as wide as the widest.
    /// Horizontal: one row, the options side by side with their spacing.
    /// Focus puts the two-column `> ` marker in front, as `render` does.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        if self.options.is_empty() {
            return Some((0, 0));
        }
        let focus = if self.focused && !self.disabled { 2 } else { 0 };
        let n = self.options.len();
        let gap = self.gap as usize;
        let (w, h) = match self.layout {
            RadioLayout::Vertical => {
                let widest = self.options.iter().map(|o| self.option_width(o)).max();
                (focus + widest.unwrap_or(0), n + gap * (n - 1))
            }
            RadioLayout::Horizontal => {
                let options: usize = self.options.iter().map(|o| self.option_width(o)).sum();
                (focus + options + (2 + gap) * (n - 1), 1)
            }
        };
        let clamp = |v: usize, max: u16| v.min(max as usize) as u16;
        Some((clamp(w, max_width), clamp(h, max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.options.is_empty() {
            return;
        }

        // Render focus indicator for the group
        let start_x: u16 = if self.focused && !self.disabled {
            let mut arrow = Cell::new('>');
            arrow.fg = Some(Color::CYAN);
            ctx.set(0, 0, arrow);
            2
        } else {
            0
        };

        match self.layout {
            RadioLayout::Vertical => {
                let mut y: u16 = 0;
                for (i, _) in self.options.iter().enumerate() {
                    if y >= area.height {
                        break;
                    }
                    self.render_option(ctx, i, start_x, y);
                    y += 1 + self.gap;
                }
            }
            RadioLayout::Horizontal => {
                let mut x = start_x;
                for (i, _option) in self.options.iter().enumerate() {
                    if x >= area.width {
                        break;
                    }
                    let width = self.render_option(ctx, i, x, 0);
                    x += width + 2 + self.gap; // 2 for spacing between options
                }
            }
        }
    }
}
