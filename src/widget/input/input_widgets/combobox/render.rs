//! View implementation for Combobox

use super::super::dropdown::{
    calculate_dropdown_layout, queue_or_inline_overlay, render_options, render_status_row,
    DropdownColors, DropdownOption,
};
use super::super::Combobox;

/// Render the combobox widget
pub fn render_combobox(combobox: &Combobox, ctx: &mut crate::widget::traits::RenderContext) {
    let area = ctx.area;
    if area.width < 3 || area.height < 1 {
        return;
    }

    let width = combobox.display_width(area.width);
    let text_width = (width - 2) as usize;

    // ─────────────────────────────────────────────────────────────────────
    // Render input field
    // ─────────────────────────────────────────────────────────────────────

    let input_fg = combobox.input_fg.or(combobox.fg);
    let input_bg = combobox.input_bg.or(combobox.bg);

    // Draw background
    ctx.fill_row(0, width, input_fg, input_bg);

    // Draw dropdown indicator
    let icon = if combobox.loading {
        '⟳'
    } else if combobox.open {
        '▲'
    } else {
        '▼'
    };
    let mut cell = crate::render::Cell::new(icon);
    cell.fg = input_fg;
    cell.bg = input_bg;
    ctx.set(width - 2, 0, cell);

    // Draw text or placeholder
    let display_text = if combobox.input.is_empty() {
        &combobox.placeholder
    } else {
        &combobox.input
    };

    let is_placeholder = combobox.input.is_empty();
    let truncated = crate::utils::truncate_to_width(display_text, text_width);

    let mut cx: u16 = 1;
    for ch in truncated.chars() {
        let mut cell = crate::render::Cell::new(ch);
        cell.fg = if is_placeholder {
            combobox.disabled_fg
        } else {
            input_fg
        };
        cell.bg = input_bg;
        ctx.set(cx, 0, cell);
        cx += crate::utils::char_width(ch) as u16;
    }

    // Draw cursor (if not placeholder)
    if !is_placeholder && combobox.cursor <= truncated.chars().count() {
        // Calculate cursor x from display widths of characters before cursor
        let cursor_display_width: usize = truncated
            .chars()
            .take(combobox.cursor)
            .map(crate::utils::char_width)
            .sum();
        let cursor_x = 1 + cursor_display_width as u16;
        if cursor_x < width - 2 {
            if let Some(cell) = ctx.get_mut(cursor_x, 0) {
                cell.bg = Some(crate::style::Color::WHITE);
                cell.fg = Some(crate::style::Color::BLACK);
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // Render dropdown (if open)
    // ─────────────────────────────────────────────────────────────────────

    if !combobox.open {
        return;
    }

    let visible_count = combobox
        .max_visible
        .min(crate::widget::theme::MAX_DROPDOWN_VISIBLE as usize);

    let colors = DropdownColors {
        fg: combobox.fg,
        bg: combobox.bg,
        selected_fg: combobox.selected_fg,
        selected_bg: combobox.selected_bg,
        highlight_fg: combobox.highlight_fg,
        disabled_fg: combobox.disabled_fg,
    };

    // Calculate dropdown height, accounting for scroll offset
    let items_after_scroll = combobox
        .filtered
        .len()
        .saturating_sub(combobox.scroll_offset);
    let dropdown_h = if combobox.loading || combobox.filtered.is_empty() {
        1u16
    } else {
        (items_after_scroll.min(visible_count) as u16).max(1)
    };

    // Position the overlay
    let layout = calculate_dropdown_layout(ctx, dropdown_h);
    let (abs_x, _) = ctx.absolute_position();
    let overlay_area = crate::layout::Rect::new(abs_x, layout.overlay_y, width, dropdown_h);
    let mut entry = crate::widget::traits::OverlayEntry::new(100, overlay_area);

    // Loading state
    if combobox.loading {
        render_status_row(
            &mut entry,
            &combobox.loading_text,
            width,
            combobox.fg,
            combobox.bg,
            combobox.disabled_fg,
        );
        queue_or_inline_overlay(ctx, entry);
        return;
    }

    // Empty state
    if combobox.filtered.is_empty() {
        render_status_row(
            &mut entry,
            &combobox.empty_text,
            width,
            combobox.fg,
            combobox.bg,
            combobox.disabled_fg,
        );
        queue_or_inline_overlay(ctx, entry);
        return;
    }

    // Build option descriptors
    let dropdown_options: Vec<DropdownOption<'_>> = combobox
        .filtered
        .iter()
        .skip(combobox.scroll_offset)
        .take(visible_count)
        .enumerate()
        .map(|(row, &option_idx)| {
            let option = &combobox.options[option_idx];
            let is_highlighted = row + combobox.scroll_offset == combobox.selected_idx;
            let is_multi_selected =
                combobox.multi_select && combobox.is_selected(option.get_value());

            let indicator = if combobox.multi_select {
                if is_multi_selected {
                    '☑'
                } else {
                    '☐'
                }
            } else if is_highlighted {
                '›'
            } else {
                ' '
            };

            let match_indices: std::collections::HashSet<usize> = combobox
                .get_match(&option.label)
                .map(|m| m.indices.into_iter().collect())
                .unwrap_or_default();

            DropdownOption {
                label: &option.label,
                is_highlighted,
                is_disabled: option.disabled,
                match_indices,
                indicator,
            }
        })
        .collect();

    render_options(&mut entry, &dropdown_options, width, &colors);

    // Draw scroll indicators
    let total_filtered = combobox.filtered.len();
    if total_filtered > visible_count {
        if combobox.scroll_offset > 0 {
            let mut cell = crate::render::Cell::new('↑');
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            entry.push(width - 1, 0, cell);
        }
        if combobox.scroll_offset + visible_count < total_filtered {
            let mut cell = crate::render::Cell::new('↓');
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            entry.push(width - 1, dropdown_h.saturating_sub(1), cell);
        }
    }

    queue_or_inline_overlay(ctx, entry);
}
