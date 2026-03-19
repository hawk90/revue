//! View implementation for Combobox

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
    for x in 0..width {
        let mut cell = crate::render::Cell::new(' ');
        cell.fg = input_fg;
        cell.bg = input_bg;
        ctx.set(x, 0, cell);
    }

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

    let visible_count = combobox.max_visible.min(10);

    // Calculate overlay position, flip above if near bottom
    let (abs_x, abs_y) = ctx.absolute_position();
    let dropdown_h = if combobox.loading || combobox.filtered.is_empty() {
        1u16
    } else {
        (combobox
            .filtered
            .len()
            .saturating_sub(combobox.scroll_offset)
            .min(visible_count) as u16)
            .max(1)
    };
    let buf_height = ctx.buffer.height();
    let space_below = buf_height.saturating_sub(abs_y + 1);
    let overlay_y = if space_below >= dropdown_h {
        abs_y + 1
    } else {
        abs_y.saturating_sub(dropdown_h)
    };
    let overlay_area = crate::layout::Rect::new(abs_x, overlay_y, width, dropdown_h);
    let mut entry = crate::widget::traits::OverlayEntry::new(100, overlay_area);

    // Loading state
    if combobox.loading {
        for x in 0..width {
            let mut cell = crate::render::Cell::new(' ');
            cell.fg = combobox.fg;
            cell.bg = combobox.bg;
            entry.push(x, 0, cell);
        }
        let loading_truncated = crate::utils::truncate_to_width(&combobox.loading_text, text_width);
        let mut cx: u16 = 1;
        for ch in loading_truncated.chars() {
            let mut cell = crate::render::Cell::new(ch);
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            entry.push(cx, 0, cell);
            cx += crate::utils::char_width(ch) as u16;
        }
        if !ctx.queue_overlay(entry.clone()) {
            for oc in &entry.cells {
                ctx.set(oc.x, oc.y + 1, oc.cell);
            }
        }
        return;
    }

    // Empty state
    if combobox.filtered.is_empty() {
        for x in 0..width {
            let mut cell = crate::render::Cell::new(' ');
            cell.fg = combobox.fg;
            cell.bg = combobox.bg;
            entry.push(x, 0, cell);
        }
        let empty_truncated = crate::utils::truncate_to_width(&combobox.empty_text, text_width);
        let mut cx: u16 = 1;
        for ch in empty_truncated.chars() {
            let mut cell = crate::render::Cell::new(ch);
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            entry.push(cx, 0, cell);
            cx += crate::utils::char_width(ch) as u16;
        }
        if !ctx.queue_overlay(entry.clone()) {
            for oc in &entry.cells {
                ctx.set(oc.x, oc.y + 1, oc.cell);
            }
        }
        return;
    }

    // Render visible options
    for (row, &option_idx) in combobox
        .filtered
        .iter()
        .skip(combobox.scroll_offset)
        .take(visible_count)
        .enumerate()
    {
        let y = row as u16;
        let option = &combobox.options[option_idx];
        let is_highlighted = row + combobox.scroll_offset == combobox.selected_idx;
        let is_multi_selected = combobox.multi_select && combobox.is_selected(option.get_value());

        let (fg, bg) = if is_highlighted {
            (combobox.selected_fg, combobox.selected_bg)
        } else {
            (combobox.fg, combobox.bg)
        };

        let fg = if option.disabled {
            combobox.disabled_fg
        } else {
            fg
        };

        // Draw background
        for x in 0..width {
            let mut cell = crate::render::Cell::new(' ');
            cell.fg = fg;
            cell.bg = bg;
            entry.push(x, y, cell);
        }

        // Draw selection indicator
        if combobox.multi_select {
            let indicator = if is_multi_selected { '☑' } else { '☐' };
            let mut cell = crate::render::Cell::new(indicator);
            cell.fg = fg;
            cell.bg = bg;
            entry.push(0, y, cell);
        } else {
            let indicator = if is_highlighted { '›' } else { ' ' };
            let mut cell = crate::render::Cell::new(indicator);
            cell.fg = fg;
            cell.bg = bg;
            entry.push(0, y, cell);
        }

        // Get match indices for highlighting (HashSet for O(1) lookup)
        let match_indices: std::collections::HashSet<usize> = combobox
            .get_match(&option.label)
            .map(|m| m.indices.into_iter().collect())
            .unwrap_or_default();

        // Draw option text with highlighting
        let truncated =
            crate::utils::truncate_to_width(&option.label, text_width.saturating_sub(1));
        let mut cx: u16 = 2;
        for (j, ch) in truncated.chars().enumerate() {
            let mut cell = crate::render::Cell::new(ch);
            cell.bg = bg;

            if option.disabled {
                cell.fg = combobox.disabled_fg;
            } else if match_indices.contains(&j) {
                cell.fg = combobox.highlight_fg;
            } else {
                cell.fg = fg;
            }

            entry.push(cx, y, cell);
            cx += crate::utils::char_width(ch) as u16;
        }
    }

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

    // Queue as overlay; fallback to inline
    if !ctx.queue_overlay(entry.clone()) {
        for oc in &entry.cells {
            ctx.set(oc.x, oc.y + 1, oc.cell);
        }
    }
}
