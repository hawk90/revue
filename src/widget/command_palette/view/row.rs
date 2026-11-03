//! One command row: icon, highlighted label, description and shortcut.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::theme::SUBTLE_GRAY;
use crate::widget::traits::RenderContext;

use super::super::command::Command;
use super::{CommandPalette, Layout};

impl CommandPalette {
    /// Draw `cmd` on row `item_y`, on the selection background if selected.
    pub(super) fn draw_row(
        &self,
        ctx: &mut RenderContext,
        layout: &Layout,
        item_y: u16,
        cmd: &Command,
        is_selected: bool,
    ) {
        let x = layout.x;

        // Left border
        self.draw_left(ctx, layout, item_y);

        // Background for selected
        let row_bg = if is_selected {
            self.selected_bg
        } else {
            self.bg_color
        };
        for dx in 1..layout.width.saturating_sub(1) {
            let mut cell = Cell::new(' ');
            cell.bg = Some(row_bg);
            ctx.set(x + dx, item_y, cell);
        }

        let mut content_x = x + 2;

        // Icon
        if self.show_icons {
            if let Some(icon) = cmd.icon {
                let mut cell = Cell::new(icon);
                cell.fg = Some(Color::CYAN);
                cell.bg = Some(row_bg);
                ctx.set(content_x, item_y, cell);
            }
            content_x += 2;
        }

        // The shortcut, right-aligned in the text columns, is drawn only
        // when it leaves the label a column and a gap; the label and the
        // description stop a column before it (#1091)
        let shortcut_x = cmd
            .shortcut
            .as_ref()
            .filter(|_| self.show_shortcuts)
            .map(|shortcut| {
                layout
                    .text_end
                    .saturating_sub(display_width(shortcut) as u16)
            })
            .filter(|&sx| sx >= content_x + 2 && sx < layout.text_end);
        let text_end = shortcut_x.map_or(layout.text_end, |sx| sx - 1);

        content_x = self.draw_label(ctx, item_y, content_x, text_end, cmd, row_bg);
        self.draw_details(ctx, item_y, content_x, text_end, shortcut_x, cmd, row_bg);

        // Right border
        self.draw_right(ctx, layout, item_y);
    }

    /// The label from `content_x` up to `label_end`, query matches in the
    /// match color; returns the column after it.
    fn draw_label(
        &self,
        ctx: &mut RenderContext,
        item_y: u16,
        mut content_x: u16,
        label_end: u16,
        cmd: &Command,
        row_bg: Color,
    ) -> u16 {
        // Label with highlighting
        let highlighted = self.highlight_match(&cmd.label);
        for (ch, is_match) in highlighted {
            let cw = char_width(ch) as u16;
            if content_x + cw > label_end {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_match {
                self.match_color
            } else {
                Color::WHITE
            });
            cell.bg = Some(row_bg);
            if is_match {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(content_x, item_y, cell);
            content_x += cw;
        }
        content_x
    }

    /// The dimmed description after the label, up to `text_end`, and the
    /// shortcut at `shortcut_x` when it is drawn.
    #[allow(clippy::too_many_arguments)]
    fn draw_details(
        &self,
        ctx: &mut RenderContext,
        item_y: u16,
        content_x: u16,
        text_end: u16,
        shortcut_x: Option<u16>,
        cmd: &Command,
        row_bg: Color,
    ) {
        // Description, dimmed, after the label and clear of the shortcut
        if self.show_descriptions {
            if let Some(ref desc) = cmd.description {
                let mut dx = content_x + 2;
                for ch in desc.chars() {
                    let cw = char_width(ch) as u16;
                    if dx + cw > text_end {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(SUBTLE_GRAY);
                    cell.bg = Some(row_bg);
                    ctx.set(dx, item_y, cell);
                    dx += cw;
                }
            }
        }

        // Shortcut (right-aligned)
        if let (Some(shortcut_x), Some(shortcut)) = (shortcut_x, cmd.shortcut.as_ref()) {
            let mut dx: u16 = 0;
            for ch in shortcut.chars() {
                let cw = char_width(ch) as u16;
                let mut cell = Cell::new(ch);
                cell.fg = Some(SUBTLE_GRAY);
                cell.bg = Some(row_bg);
                ctx.set(shortcut_x + dx, item_y, cell);
                dx += cw;
            }
        }
    }
}
