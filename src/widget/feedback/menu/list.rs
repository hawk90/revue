//! The item list both menus draw and move through: the MenuBar dropdown and
//! the ContextMenu take the same `MenuItem`s, so they size, draw and skip
//! them with this one model (#1051)

use super::frame::draw_frame;
use super::types::MenuItem;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::display_width;
use crate::utils::ScrollState;
use crate::widget::traits::RenderContext;

/// The colors of a menu's list
#[derive(Clone, Copy)]
pub(super) struct ListColors {
    /// Plain item text and the frame
    pub fg: Color,
    /// The list's background
    pub bg: Color,
    /// The selected item's text
    pub selected_fg: Color,
    /// The selected item's background
    pub selected_bg: Color,
    /// A disabled item's text and the separators
    pub disabled_fg: Color,
    /// The shortcuts
    pub shortcut_fg: Color,
}

/// Whether the item can be selected: everything but a separator. A disabled
/// item can be selected, and choosing it does nothing.
pub(super) fn selectable(item: &MenuItem) -> bool {
    !item.separator
}

/// Whether choosing the item runs it
pub(super) fn choosable(item: &MenuItem) -> bool {
    !item.separator && !item.disabled
}

/// The first selectable item, or `None` when there is none
pub(super) fn first_selectable(items: &[MenuItem]) -> Option<usize> {
    items.iter().position(selectable)
}

/// The next selectable item from `from` in `dir` (1 down, -1 up), wrapping
/// around when `wrap`; `from` itself when no other one is selectable that
/// way
pub(super) fn step(items: &[MenuItem], from: usize, dir: i8, wrap: bool) -> usize {
    let n = items.len();
    if n == 0 {
        return from;
    }
    let mut i = from.min(n - 1);
    for _ in 0..n {
        i = match (dir > 0, wrap) {
            (true, _) if i + 1 < n => i + 1,
            (true, true) => 0,
            (false, _) if i > 0 => i - 1,
            (false, true) => n - 1,
            _ => return from,
        };
        if selectable(&items[i]) {
            return i;
        }
    }
    from
}

/// The columns the list needs: the borders and one blank column inside each,
/// the check mark's two columns when any item can be checked, the widest
/// label, and a shortcut with two columns before it. 10 for a list of
/// separators only.
pub(super) fn list_width(items: &[MenuItem]) -> u16 {
    let check = if items.iter().any(|i| i.checked.is_some()) {
        2
    } else {
        0
    };
    let widest = items
        .iter()
        .filter(|item| !item.separator)
        .map(|item| {
            let shortcut = item.shortcut.as_ref().map_or(0, |s| display_width(s) + 2);
            display_width(&item.label) + shortcut
        })
        .max();
    match widest {
        Some(w) => u16::try_from(w + check + 4).unwrap_or(u16::MAX),
        None => 10,
    }
}

/// The left column for a list `width` wide that would start at `x`: moved
/// left as far as needed to end inside an area `area_width` wide
pub(super) fn fit_x(x: u16, width: u16, area_width: u16) -> u16 {
    x.min(area_width.saturating_sub(width))
}

/// The list's frame at `(x, y)`, `width` x `height`, and the items scrolled
/// to keep `selected` in view. A frame with no row inside (under 3 columns
/// or 3 rows) draws only the frame.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_list(
    ctx: &mut RenderContext,
    items: &[MenuItem],
    (x, y): (u16, u16),
    (width, height): (u16, u16),
    selected: Option<usize>,
    scroll: &ScrollState,
    colors: ListColors,
) {
    if width == 0 || height == 0 {
        return;
    }
    draw_frame(ctx, x, y, width, height, colors.fg, colors.bg);
    if width < 3 || height < 3 {
        return;
    }

    // The rows inside the border, scrolled to keep the selected item in
    // view (#973)
    scroll.set_content(items.len());
    scroll.set_viewport(usize::from(height - 2));
    let highlight = selected.unwrap_or(0);
    scroll.track(highlight..highlight + 1);
    let shown = scroll.visible();

    for (row, (i, item)) in items
        .iter()
        .enumerate()
        .skip(shown.start)
        .take(shown.len())
        .enumerate()
    {
        let item_y = y + 1 + row as u16;
        if item.separator {
            draw_separator_row(ctx, x, width, item_y, colors);
        } else {
            draw_item_row(ctx, item, selected == Some(i), x, width, item_y, colors);
        }
    }
}

/// A separator: a dim line across the list, joined to its border
fn draw_separator_row(ctx: &mut RenderContext, x: u16, width: u16, y: u16, colors: ListColors) {
    for dx in 1..width - 1 {
        ctx.set(
            x + dx,
            y,
            Cell::new('─').fg(colors.disabled_fg).bg(colors.bg),
        );
    }
    ctx.set(x, y, Cell::new('├').fg(colors.fg).bg(colors.bg));
    ctx.set(x + width - 1, y, Cell::new('┤').fg(colors.fg).bg(colors.bg));
}

/// An item: its check mark, label, right-aligned shortcut and submenu
/// marker, in the selected, disabled or plain colors. Every part stays
/// inside the border, and the label stops before the shortcut.
fn draw_item_row(
    ctx: &mut RenderContext,
    item: &MenuItem,
    is_selected: bool,
    x: u16,
    width: u16,
    y: u16,
    colors: ListColors,
) {
    let bg = if is_selected {
        colors.selected_bg
    } else {
        colors.bg
    };
    let fg = if item.disabled {
        colors.disabled_fg
    } else if is_selected {
        colors.selected_fg
    } else {
        colors.fg
    };

    for dx in 1..width - 1 {
        ctx.set(x + dx, y, Cell::new(' ').bg(bg));
    }

    // The columns inside the blank column next to each border
    let start = x + 2;
    let end = x + width - 2;

    let mut text_x = start;
    if let Some(checked) = item.checked {
        if text_x < end {
            ctx.set(
                text_x,
                y,
                Cell::new(if checked { '✓' } else { ' ' }).fg(fg).bg(bg),
            );
        }
        text_x = text_x.saturating_add(2);
    }

    // The shortcut, right-aligned at the end; in a list cut narrower than it,
    // it starts at the first column and is cut at the end
    let mut label_end = end;
    if let Some(ref shortcut) = item.shortcut {
        let sw = u16::try_from(display_width(shortcut)).unwrap_or(u16::MAX);
        let shortcut_x = end.saturating_sub(sw).max(start);
        let shortcut_fg = colors.shortcut_fg;
        ctx.put_str_with(shortcut_x, y, shortcut, end, |ch| {
            Cell::new(ch).fg(shortcut_fg).bg(bg)
        });
        // One blank column between the label and the shortcut
        label_end = shortcut_x.saturating_sub(1).max(start);
    }

    if text_x < label_end {
        ctx.put_str_with(text_x, y, &item.label, label_end, |ch| {
            Cell::new(ch).fg(fg).bg(bg)
        });
    }

    // The submenu marker, in the blank column before the right border
    if item.has_submenu() {
        ctx.set(end, y, Cell::new('▶').fg(fg).bg(bg));
    }
}
