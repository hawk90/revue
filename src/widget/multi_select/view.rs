//! Drawing for the multi-select widget: the header row (tags or placeholder,
//! query, arrow) and the dropdown of options

use crate::impl_view_meta;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::input_widgets::dropdown::{
    calculate_dropdown_layout, queue_or_inline_overlay, MIN_DROPDOWN_WIDTH,
};
use crate::widget::theme::{
    DISABLED_FG, LIGHT_GRAY, MAX_DROPDOWN_VISIBLE, PLACEHOLDER_FG, SECONDARY_TEXT,
};
use crate::widget::traits::{OverlayEntry, RenderContext, View};

use super::{MultiSelect, MultiSelectOption};
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};

/// The columns the tags leave for the search query while it is typed
const QUERY_ROOM: u16 = 8;

/// A `ch` cell in `fg` on `bg`
fn colored_cell(ch: char, fg: Color, bg: Color) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.bg = Some(bg);
    cell
}

impl MultiSelect {
    /// Calculate display width
    fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            // The header needs room for its arrow; a smaller configured
            // width would leave none.
            return w.max(MIN_DROPDOWN_WIDTH).min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| crate::utils::display_width(&o.label))
            .max()
            .unwrap_or(crate::utils::display_width(&self.placeholder));

        ((max_option_len + 4) as u16).min(max_width)
    }

    /// The header row: its background, the arrow, and the tags (with the
    /// query while searching) or the placeholder
    fn draw_header(&self, ctx: &mut RenderContext, width: u16, fg: Color, bg: Color) {
        // Draw background for header row
        ctx.fill_row(0, width, Some(fg), Some(bg));

        // Draw arrow
        let arrow = if self.open { '▲' } else { '▼' };
        ctx.draw_char(width - 1, 0, arrow, fg);

        // Draw tags or placeholder
        let max_x = width - 2; // Leave room for arrow

        if self.selected.is_empty() && !self.open {
            // Draw placeholder
            ctx.draw_text(0, 0, &self.placeholder, PLACEHOLDER_FG);
        } else {
            // While searching, the tags leave room for the query: up to
            // QUERY_ROOM columns, half the row at most
            let searching = self.open && self.searchable;
            let room = if searching {
                QUERY_ROOM.min(max_x / 2)
            } else {
                0
            };
            let x = self.draw_tags(ctx, max_x - room);

            // While searching, the query after the tags is a field with its
            // cursor, scrolled to it in the room before the arrow
            if searching {
                let from = (x + 1).min(max_x);
                let field = EditField::of(&self.query, "", self.state.focused);
                let w = max_x - from;
                let scroll = field.scroll(self.query_scroll.get(), w);
                self.query_scroll.set(scroll);
                let selected = crate::widget::traits::render_context::edit_line::selection_colors();
                ctx.put_edit_field(from, 0, w, &field, scroll, |kind, ch| match kind {
                    FieldCell::Cursor => colored_cell(ch, Color::BLACK, Color::WHITE),
                    FieldCell::Selected => colored_cell(ch, selected.0, selected.1),
                    FieldCell::Text | FieldCell::Placeholder => colored_cell(ch, Color::CYAN, bg),
                });
            }
        }
    }

    /// The selected options as `[label]` tags, and `...` in place of the first
    /// tag that does not fit before `max_x`. The row scrolls (in columns) to
    /// keep the tag cursor in view, starts at a whole tag (#882), and starts
    /// with `... ` if tags are scrolled off to the left. Returns the column
    /// after the tags.
    fn draw_tags(&self, ctx: &mut RenderContext, max_x: u16) -> u16 {
        // Each tag's first column on the whole row: "[label] "
        let mut starts = Vec::with_capacity(self.selected.len());
        let mut total = 0;
        for &opt_idx in &self.selected {
            starts.push(total);
            let label = self.options.get(opt_idx).map_or("", |o| o.label.as_str());
            total += crate::utils::display_width(label) + 3;
        }
        let scroll = &self.tag_scroll;
        scroll.set_content(total);
        // With more than one tag, the one in view may need " ..." after it
        // and "... " before it
        let markers = if self.selected.len() > 1 { 8 } else { 0 };
        // A viewport of none would stop the row from scrolling at all.
        scroll.set_viewport(max_x.saturating_sub(markers).max(1) as usize);
        if let Some(c) = self.tag_cursor.filter(|&c| c < starts.len()) {
            let end = starts.get(c + 1).copied().unwrap_or(total);
            // Without the space after the tag
            scroll.track(starts[c]..end.saturating_sub(1).max(starts[c] + 1));
        }
        // The first whole tag at or after the offset: it keeps the cursor's
        // tag in view, which ends within the viewport from the offset
        let first = starts
            .iter()
            .position(|&s| s >= scroll.offset())
            .unwrap_or(starts.len());

        let mut x: u16 = 0;
        let last = self.selected.len().saturating_sub(1);
        // Only if the first tag still fits after it: the tag under the cursor
        // comes before the marker.
        if first > 0 && first <= last {
            let tag = starts.get(first + 1).copied().unwrap_or(total) - starts[first] - 1;
            let after = if first < last { 4 } else { 0 };
            if 4 + tag + after <= max_x as usize {
                ctx.draw_text(0, 0, "...", LIGHT_GRAY);
                x = 4;
            }
        }
        for (i, &opt_idx) in self.selected.iter().enumerate().skip(first) {
            if let Some(opt) = self.options.get(opt_idx) {
                let label = &opt.label;
                // "[label]"; the space after it is not needed to fit. A tag
                // that more tags follow also needs room for " ..." after it,
                // so the hidden ones are always marked
                let tag_len = (crate::utils::display_width(label) + 2) as u16;
                let marker = if i < last { 4 } else { 0 };

                if x + tag_len + marker > max_x {
                    // Draw overflow indicator; the column after it is
                    // where the query goes
                    ctx.draw_text(x, 0, "...", LIGHT_GRAY);
                    x += 3;
                    break;
                }

                x = self.draw_tag(ctx, x, max_x, label, self.tag_cursor == Some(i));
            }
        }
        x
    }

    /// One `[label]` tag at column `x`, in the tag cursor's colors when
    /// `is_tag_selected`. Returns the column after it and its space.
    fn draw_tag(
        &self,
        ctx: &mut RenderContext,
        mut x: u16,
        max_x: u16,
        label: &str,
        is_tag_selected: bool,
    ) -> u16 {
        let tag_fg = if is_tag_selected {
            Color::WHITE
        } else {
            SECONDARY_TEXT
        };
        let tag_bg_color = if is_tag_selected {
            Color::rgb(100, 100, 200)
        } else {
            self.tag_bg.unwrap_or(Color::rgb(60, 60, 140))
        };

        // Draw tag with brackets
        ctx.draw_char_bg(x, 0, '[', tag_fg, tag_bg_color);
        x += 1;

        for ch in label.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if x + cw > max_x - 1 {
                break;
            }
            ctx.draw_char_bg(x, 0, ch, tag_fg, tag_bg_color);
            x += cw;
        }

        ctx.draw_char_bg(x, 0, ']', tag_fg, tag_bg_color);
        x += 1;

        // Space between tags
        if x < max_x {
            x += 1;
        }
        x
    }

    /// The open dropdown: up to [`MAX_DROPDOWN_VISIBLE`] filtered options,
    /// scrolled to keep the cursor in view, queued as an overlay below the
    /// header (above it when the screen has no room below), or drawn below
    /// the header without an overlay queue
    fn draw_dropdown(&self, ctx: &mut RenderContext, width: u16, fg: Color, bg: Color) {
        let max_visible = self.filtered.len().min(MAX_DROPDOWN_VISIBLE as usize);
        let dropdown_h = max_visible.max(1) as u16;
        let scroll = &self.dropdown_scroll;
        scroll.set_content(self.filtered.len());
        scroll.set_viewport(max_visible);
        scroll.track(self.dropdown_cursor..self.dropdown_cursor + 1);
        let shown = scroll.visible();

        let (abs_x, _) = ctx.absolute_position();
        let layout = calculate_dropdown_layout(ctx, dropdown_h);
        let overlay_area = crate::layout::Rect::new(abs_x, layout.overlay_y, width, dropdown_h);
        let mut entry = OverlayEntry::new(100, overlay_area);

        for (row, (i, &opt_idx)) in self
            .filtered
            .iter()
            .enumerate()
            .skip(shown.start)
            .take(shown.len())
            .enumerate()
        {
            let y = row as u16;
            let is_cursor = i == self.dropdown_cursor;
            let is_selected = self.is_selected(opt_idx);

            if let Some(opt) = self.options.get(opt_idx) {
                let colors = if is_cursor {
                    (Color::WHITE, Color::rgb(80, 80, 150))
                } else {
                    (fg, bg)
                };
                self.push_option_row(&mut entry, y, opt, is_selected, width, colors);
            }
        }

        // Queue as overlay; fallback to inline
        queue_or_inline_overlay(ctx, entry);
    }

    /// Dropdown row `y`: its background, the `[x]`/`[ ]` checkbox and the
    /// label, its query matches in the highlight color
    fn push_option_row(
        &self,
        entry: &mut OverlayEntry,
        y: u16,
        opt: &MultiSelectOption,
        is_selected: bool,
        width: u16,
        (row_fg, row_bg): (Color, Color),
    ) {
        // Row background
        for dx in 0..width {
            entry.push(dx, y, colored_cell(' ', row_fg, row_bg));
        }

        // Checkbox
        let checkbox_str = if is_selected { "[x]" } else { "[ ]" };
        for (i, ch) in checkbox_str.chars().enumerate() {
            entry.push(i as u16, y, colored_cell(ch, row_fg, row_bg));
        }

        // Draw label with highlight (HashSet for O(1) lookup)
        let match_indices: std::collections::HashSet<usize> = self
            .get_match(&opt.label)
            .map(|m| m.indices.into_iter().collect())
            .unwrap_or_default();

        let mut cx: u16 = 4;
        for (j, ch) in opt.label.chars().enumerate() {
            let cw = crate::utils::char_width(ch) as u16;
            if cx + cw > width {
                break;
            }

            let char_fg = if match_indices.contains(&j) {
                self.highlight_fg.unwrap_or(Color::YELLOW)
            } else if opt.disabled {
                DISABLED_FG
            } else {
                row_fg
            };

            entry.push(cx, y, colored_cell(ch, char_fg, row_bg));
            cx += cw;
        }
    }
}

impl View for MultiSelect {
    /// One row: the open list is an overlay and takes no space here.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((self.display_width(max_width), 1.min(max_height)))
    }
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        let (fg, bg) =
            self.state
                .resolve_colors_interactive(ctx.style, Color::WHITE, Color::rgb(50, 50, 50));

        let width = self.display_width(area.width);

        self.draw_header(ctx, width, fg, bg);

        // Draw dropdown if open (as overlay to escape parent clipping)
        if self.open {
            self.draw_dropdown(ctx, width, fg, bg);
        }
    }

    impl_view_meta!("MultiSelect");
}
