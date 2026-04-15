//! View implementation for Select

use crate::render::Cell;
use crate::style::Color;
use crate::utils::truncate_to_width;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{RenderContext, View};

use super::super::dropdown::{
    calculate_dropdown_layout, dropdown_height, queue_or_inline_overlay, render_options,
    render_status_row, DropdownColors, DropdownOption,
};
use super::Select;

impl View for Select {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let width = self.display_width(area.width);
        let text_width = (width - 2) as usize;

        // Determine colors based on state
        let fg = if self.disabled {
            Some(PLACEHOLDER_FG)
        } else if self.focused {
            self.fg.or(Some(Color::CYAN))
        } else {
            self.fg
        };
        let bg = self.bg;

        // Render the closed/header row
        let display_text = if self.open && self.searchable && !self.query.is_empty() {
            &self.query
        } else {
            self.value().unwrap_or(&self.placeholder)
        };
        let arrow = if self.open { "▲" } else { "▼" };

        // Draw background for header
        ctx.fill_row(0, width, fg, bg);

        // Draw focus indicator (inside area bounds)
        if self.focused && !self.disabled {
            ctx.draw_focus_brackets(0, width, Color::CYAN);
        }

        // Draw arrow (or search icon when searching)
        let icon = if self.open && self.searchable {
            "🔍".chars().next().unwrap_or('?')
        } else {
            arrow.chars().next().unwrap_or('▼')
        };
        let mut cell = Cell::new(icon);
        cell.fg = fg;
        cell.bg = bg;
        ctx.set(0, 0, cell);

        // Draw text
        let truncated = truncate_to_width(display_text, text_width);
        let mut cx: u16 = 2;
        for ch in truncated.chars() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(cx, 0, cell);
            cx += crate::utils::char_width(ch) as u16;
        }

        // If open, render dropdown options as overlay
        if !self.open {
            return;
        }

        // Determine which options to show
        let visible_options: Vec<(usize, &String)> = if self.query.is_empty() {
            self.options.iter().enumerate().collect()
        } else {
            self.filtered
                .iter()
                .filter_map(|&i| self.options.get(i).map(|opt| (i, opt)))
                .collect()
        };

        let dropdown_h = dropdown_height(visible_options.len(), None);

        let layout = calculate_dropdown_layout(ctx, dropdown_h);
        let (abs_x, _) = ctx.absolute_position();
        let overlay_area = crate::layout::Rect::new(abs_x, layout.overlay_y, width, dropdown_h);
        let mut entry = crate::widget::traits::OverlayEntry::new(100, overlay_area);

        // Empty state
        if visible_options.is_empty() && !self.query.is_empty() {
            render_status_row(
                &mut entry,
                "No results",
                width,
                self.fg,
                self.bg,
                Some(PLACEHOLDER_FG),
            );
            queue_or_inline_overlay(ctx, entry);
            return;
        }

        let colors = DropdownColors {
            fg: self.fg,
            bg: self.bg,
            selected_fg: self.selected_fg,
            selected_bg: self.selected_bg,
            highlight_fg: self.highlight_fg,
            disabled_fg: None,
        };

        // Build option descriptors
        let dropdown_options: Vec<DropdownOption<'_>> = visible_options
            .iter()
            .take(dropdown_h as usize)
            .map(|(option_idx, option)| {
                let is_selected = self.selection.is_selected(*option_idx);
                let indicator = if is_selected { '›' } else { ' ' };

                let match_indices: std::collections::HashSet<usize> = self
                    .get_match(option)
                    .map(|m| m.indices.into_iter().collect())
                    .unwrap_or_default();

                DropdownOption {
                    label: option.as_str(),
                    is_highlighted: is_selected,
                    is_disabled: false,
                    match_indices,
                    indicator,
                }
            })
            .collect();

        render_options(&mut entry, &dropdown_options, width, &colors);
        queue_or_inline_overlay(ctx, entry);
    }

    crate::impl_view_meta!("Select");
}
