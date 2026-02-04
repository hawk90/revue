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
        ctx.buffer.set(area.x + x, area.y, cell);
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
    ctx.buffer.set(area.x + width - 2, area.y, cell);

    // Draw text or placeholder
    let display_text = if combobox.input.is_empty() {
        &combobox.placeholder
    } else {
        &combobox.input
    };

    let is_placeholder = combobox.input.is_empty();
    let truncated: String = display_text.chars().take(text_width).collect();

    for (i, ch) in truncated.chars().enumerate() {
        let mut cell = crate::render::Cell::new(ch);
        cell.fg = if is_placeholder {
            combobox.disabled_fg
        } else {
            input_fg
        };
        cell.bg = input_bg;
        ctx.buffer.set(area.x + 1 + i as u16, area.y, cell);
    }

    // Draw cursor (if not placeholder)
    if !is_placeholder && combobox.cursor <= truncated.chars().count() {
        let cursor_x = area.x + 1 + combobox.cursor as u16;
        if cursor_x < area.x + width - 2 {
            if let Some(cell) = ctx.buffer.get_mut(cursor_x, area.y) {
                cell.bg = Some(crate::style::Color::WHITE);
                cell.fg = Some(crate::style::Color::BLACK);
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // Render dropdown (if open)
    // ─────────────────────────────────────────────────────────────────────

    if !combobox.open || area.height <= 1 {
        return;
    }

    let dropdown_height = (area.height - 1) as usize;
    let visible_count = dropdown_height.min(combobox.max_visible);

    // Loading state
    if combobox.loading {
        let y = area.y + 1;
        for x in 0..width {
            let mut cell = crate::render::Cell::new(' ');
            cell.fg = combobox.fg;
            cell.bg = combobox.bg;
            ctx.buffer.set(area.x + x, y, cell);
        }
        for (i, ch) in combobox.loading_text.chars().take(text_width).enumerate() {
            let mut cell = crate::render::Cell::new(ch);
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            ctx.buffer.set(area.x + 1 + i as u16, y, cell);
        }
        return;
    }

    // Empty state
    if combobox.filtered.is_empty() {
        let y = area.y + 1;
        for x in 0..width {
            let mut cell = crate::render::Cell::new(' ');
            cell.fg = combobox.fg;
            cell.bg = combobox.bg;
            ctx.buffer.set(area.x + x, y, cell);
        }
        for (i, ch) in combobox.empty_text.chars().take(text_width).enumerate() {
            let mut cell = crate::render::Cell::new(ch);
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            ctx.buffer.set(area.x + 1 + i as u16, y, cell);
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
        let y = area.y + 1 + row as u16;
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
            ctx.buffer.set(area.x + x, y, cell);
        }

        // Draw selection indicator (for multi-select)
        if combobox.multi_select {
            let indicator = if is_multi_selected { '☑' } else { '☐' };
            let mut cell = crate::render::Cell::new(indicator);
            cell.fg = fg;
            cell.bg = bg;
            ctx.buffer.set(area.x, y, cell);
        } else {
            let indicator = if is_highlighted { '›' } else { ' ' };
            let mut cell = crate::render::Cell::new(indicator);
            cell.fg = fg;
            cell.bg = bg;
            ctx.buffer.set(area.x, y, cell);
        }

        // Get match indices for highlighting
        let match_indices: Vec<usize> = combobox
            .get_match(&option.label)
            .map(|m| m.indices)
            .unwrap_or_default();

        // Draw option text with highlighting
        let truncated: String = option.label.chars().take(text_width - 1).collect();
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

            ctx.buffer.set(area.x + 2 + j as u16, y, cell);
        }
    }

    // Draw scroll indicator if needed
    if combobox.filtered.len() > visible_count {
        let has_more_above = combobox.scroll_offset > 0;
        let has_more_below = combobox.scroll_offset + visible_count < combobox.filtered.len();

        if has_more_above {
            let mut cell = crate::render::Cell::new('↑');
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            ctx.buffer.set(area.x + width - 1, area.y + 1, cell);
        }

        if has_more_below {
            let y = area.y + visible_count as u16;
            let mut cell = crate::render::Cell::new('↓');
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            ctx.buffer.set(area.x + width - 1, y, cell);
        }
    }
}
