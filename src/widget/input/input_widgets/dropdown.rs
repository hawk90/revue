//! Shared dropdown rendering helpers for Combobox and Select widgets
//!
//! Extracts common overlay positioning, option rendering, and overlay queuing
//! logic to eliminate duplication between dropdown-style widgets.

use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::MAX_DROPDOWN_VISIBLE;
use crate::widget::traits::{OverlayEntry, RenderContext};

/// Calculated overlay position and dimensions for a dropdown
pub struct DropdownLayout {
    /// Absolute Y coordinate for the overlay
    pub overlay_y: u16,
    /// Height of the dropdown in rows
    pub height: u16,
}

/// Calculate dropdown overlay position, flipping above the anchor if
/// there isn't enough space below.
pub fn calculate_dropdown_layout(ctx: &RenderContext, dropdown_height: u16) -> DropdownLayout {
    let (_, abs_y) = ctx.absolute_position();
    let buf_height = ctx.buffer.height();
    let space_below = buf_height.saturating_sub(abs_y + 1);
    let overlay_y = if space_below >= dropdown_height {
        abs_y + 1
    } else {
        abs_y.saturating_sub(dropdown_height)
    };
    DropdownLayout {
        overlay_y,
        height: dropdown_height,
    }
}

/// Calculate the visible dropdown height from a count of items and an
/// optional per-widget cap.
pub fn dropdown_height(item_count: usize, max_visible: Option<usize>) -> u16 {
    if item_count == 0 {
        return 1; // room for "No results" / loading row
    }
    let cap = max_visible
        .map(|mv| mv.min(MAX_DROPDOWN_VISIBLE as usize))
        .unwrap_or(MAX_DROPDOWN_VISIBLE as usize);
    (item_count.min(cap) as u16).max(1)
}

/// A single option to be rendered in the dropdown.
pub struct DropdownOption<'a> {
    /// Display text for this option
    pub label: &'a str,
    /// Whether this option is currently highlighted/selected
    pub is_highlighted: bool,
    /// Whether this option is disabled (grayed out)
    pub is_disabled: bool,
    /// Character indices that matched the search query (for highlighting)
    pub match_indices: std::collections::HashSet<usize>,
    /// Leading indicator character (e.g. '›', '☑', '☐', ' ')
    pub indicator: char,
}

/// Colors used for dropdown rendering.
pub struct DropdownColors {
    /// Default foreground
    pub fg: Option<Color>,
    /// Default background
    pub bg: Option<Color>,
    /// Foreground for highlighted/selected option
    pub selected_fg: Option<Color>,
    /// Background for highlighted/selected option
    pub selected_bg: Option<Color>,
    /// Foreground for fuzzy-match highlighted characters
    pub highlight_fg: Option<Color>,
    /// Foreground for disabled options
    pub disabled_fg: Option<Color>,
}

/// Render a status row (loading / empty) into the overlay entry.
pub fn render_status_row(
    entry: &mut OverlayEntry,
    text: &str,
    width: u16,
    fg: Option<Color>,
    bg: Option<Color>,
    text_fg: Option<Color>,
) {
    let text_width = width.saturating_sub(2) as usize;
    // Background
    for x in 0..width {
        let mut cell = Cell::new(' ');
        cell.fg = fg;
        cell.bg = bg;
        entry.push(x, 0, cell);
    }
    // Text
    let truncated = crate::utils::truncate_to_width(text, text_width);
    let mut cx: u16 = 1;
    for ch in truncated.chars() {
        let mut cell = Cell::new(ch);
        cell.fg = text_fg;
        cell.bg = bg;
        entry.push(cx, 0, cell);
        cx += crate::utils::char_width(ch) as u16;
    }
}

/// Render a list of options into an overlay entry.
///
/// Each option gets: background fill, indicator character, label with
/// fuzzy-match highlighting.
pub fn render_options(
    entry: &mut OverlayEntry,
    options: &[DropdownOption<'_>],
    width: u16,
    colors: &DropdownColors,
) {
    let text_width = width.saturating_sub(2) as usize;

    for (row, opt) in options.iter().enumerate() {
        let y = row as u16;

        let (fg, bg) = if opt.is_highlighted {
            (colors.selected_fg, colors.selected_bg)
        } else {
            (colors.fg, colors.bg)
        };

        let fg = if opt.is_disabled {
            colors.disabled_fg
        } else {
            fg
        };

        // Background
        for x in 0..width {
            let mut cell = Cell::new(' ');
            cell.fg = fg;
            cell.bg = bg;
            entry.push(x, y, cell);
        }

        // Indicator
        let mut cell = Cell::new(opt.indicator);
        cell.fg = fg;
        cell.bg = bg;
        entry.push(0, y, cell);

        // Label with match highlighting
        let truncated = crate::utils::truncate_to_width(opt.label, text_width.saturating_sub(1));
        let mut cx: u16 = 2;
        for (j, ch) in truncated.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.bg = bg;

            if opt.is_disabled {
                cell.fg = colors.disabled_fg;
            } else if opt.match_indices.contains(&j) {
                cell.fg = colors.highlight_fg;
            } else {
                cell.fg = fg;
            }

            entry.push(cx, y, cell);
            cx += crate::utils::char_width(ch) as u16;
        }
    }
}

/// Queue an overlay entry, falling back to inline rendering if the
/// overlay system is unavailable.
pub fn queue_or_inline_overlay(ctx: &mut RenderContext, entry: OverlayEntry) {
    if ctx.has_overlay_support() {
        ctx.queue_overlay(entry);
    } else {
        for oc in &entry.cells {
            ctx.set(oc.x, oc.y.saturating_add(1), oc.cell);
        }
    }
}
