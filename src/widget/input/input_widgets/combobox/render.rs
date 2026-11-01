//! View implementation for Combobox

use super::super::dropdown::{
    calculate_dropdown_layout, queue_or_inline_overlay, render_rows, render_status_row,
    DropdownColors, DropdownOption, DropdownRow, MIN_DROPDOWN_WIDTH,
};
use super::super::Combobox;
use super::filter::DisplayRow;
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};

// ── Combobox header layout ───────────────────────────────────────────
//
//  col:  0     1 ............ width-3     width-2     width-1
//        [pad]  [input text]               [arrow ▼]   [pad]

/// Column where input text starts in the combobox header.
const INPUT_TEXT_COL: u16 = 1;

/// Offset from right edge for the dropdown arrow icon.
/// Arrow sits at `width - ARROW_RIGHT_OFFSET`.
const ARROW_RIGHT_OFFSET: u16 = 2;

/// Columns reserved on the right for scroll indicators in the dropdown overlay.
const SCROLL_RESERVE: u16 = 1;

/// Render the combobox widget
pub fn render_combobox(combobox: &Combobox, ctx: &mut crate::widget::traits::RenderContext) {
    let area = ctx.area;
    if area.width < MIN_DROPDOWN_WIDTH || area.height < 1 {
        return;
    }

    let width = combobox.display_width(area.width);

    // ─────────────────────────────────────────────────────────────────────
    // Render input field
    // ─────────────────────────────────────────────────────────────────────

    // Builder, then the stylesheet. The placeholder and the selected option
    // keep their own colors.
    let input_fg = combobox
        .input_fg
        .or(combobox.fg)
        .or_else(|| ctx.css_color_if_set());
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
    ctx.set(width - ARROW_RIGHT_OFFSET, 0, cell);

    // Draw the text or placeholder and the cursor, from column 1 up to the
    // column before the arrow, scrolled so the cursor stays in view
    // Combobox has no focus state of its own: the cursor is drawn, on the
    // text or the placeholder, only while the runtime gives it focus
    let field = EditField::of(&combobox.state, &combobox.placeholder, ctx.is_focused());
    let field_width = width - ARROW_RIGHT_OFFSET - INPUT_TEXT_COL;
    let scroll = field.scroll(combobox.scroll_x.get(), field_width);
    combobox.scroll_x.set(scroll);
    let selected = crate::widget::traits::render_context::edit_line::selection_colors();
    ctx.put_edit_field(
        INPUT_TEXT_COL,
        0,
        field_width,
        &field,
        scroll,
        |kind, ch| {
            let mut cell = crate::render::Cell::new(ch);
            (cell.fg, cell.bg) = match kind {
                FieldCell::Cursor => (
                    Some(crate::style::Color::BLACK),
                    Some(crate::style::Color::WHITE),
                ),
                FieldCell::Selected => (Some(selected.0), Some(selected.1)),
                FieldCell::Placeholder => (combobox.disabled_fg, input_bg),
                FieldCell::Text => (input_fg, input_bg),
            };
            cell
        },
    );

    // ─────────────────────────────────────────────────────────────────────
    // Render dropdown (if open)
    // ─────────────────────────────────────────────────────────────────────

    if !combobox.open {
        return;
    }

    let visible_count = combobox.visible_count();

    let colors = DropdownColors {
        fg: combobox.fg,
        bg: combobox.bg,
        selected_fg: combobox.selected_fg,
        selected_bg: combobox.selected_bg,
        highlight_fg: combobox.highlight_fg,
        disabled_fg: combobox.disabled_fg,
    };

    // The rows: options, with a header above each group. The scroll offset
    // counts rows.
    let rows = combobox.display_rows();

    // The rows in view: the dropdown follows the highlight, whatever moved
    // it (#882)
    combobox.sync_scroll();
    if let Some(highlight) = combobox.highlight_rows() {
        combobox.scroll.track(highlight);
    }
    let shown = combobox.scroll.visible();
    let dropdown_h = if combobox.loading || combobox.filtered.is_empty() {
        1u16
    } else {
        (shown.len() as u16).max(1)
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

    // Build row descriptors
    let dropdown_rows: Vec<DropdownRow<'_>> = rows
        .iter()
        .skip(shown.start)
        .take(shown.len())
        .map(|display_row| {
            let pos = match *display_row {
                DisplayRow::Header(name) => return DropdownRow::Header(name),
                DisplayRow::Option(pos) => pos,
            };
            let option = &combobox.options[combobox.filtered[pos]];
            let is_highlighted = pos == combobox.selected_idx;
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

            DropdownRow::Option(DropdownOption {
                label: &option.label,
                is_highlighted,
                is_disabled: option.disabled,
                match_indices,
                indicator,
            })
        })
        .collect();

    render_rows(&mut entry, &dropdown_rows, width, &colors, SCROLL_RESERVE);

    // Draw scroll indicators
    let total_filtered = rows.len();
    let scroll_col = width - SCROLL_RESERVE;
    if total_filtered > visible_count {
        if combobox.scroll.has_above() {
            let mut cell = crate::render::Cell::new('↑');
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            entry.push(scroll_col, 0, cell);
        }
        if combobox.scroll.has_below() {
            let mut cell = crate::render::Cell::new('↓');
            cell.fg = combobox.disabled_fg;
            cell.bg = combobox.bg;
            entry.push(scroll_col, dropdown_h.saturating_sub(1), cell);
        }
    }

    queue_or_inline_overlay(ctx, entry);
}
