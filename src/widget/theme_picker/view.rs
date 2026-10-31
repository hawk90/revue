//! Drawing: the header, the compact swatches and the dropdown list

use super::ThemePicker;
use crate::render::Cell;
use crate::style::{get_theme, use_theme, Color, Theme};
use crate::widget::traits::{RenderContext, View};

/// Narrowest the picker draws: its label, a swatch and the frame need this.
const MIN_WIDTH: u16 = 10;

impl View for ThemePicker {
    /// The header (35 columns at most unless a width is set; compact: the
    /// swatch), and the open list, which is drawn in place.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let known = self
            .themes
            .iter()
            .filter(|id| get_theme(id).is_some())
            .count();
        let known = u16::try_from(known).unwrap_or(u16::MAX);
        let open = self.open && !self.themes.is_empty();
        let (w, list) = if self.compact {
            (MIN_WIDTH, if open { known } else { 0 })
        } else {
            (
                self.width.unwrap_or(max_width.min(35)).max(MIN_WIDTH),
                if open { known.saturating_add(2) } else { 0 },
            )
        };
        Some((w.min(max_width), list.saturating_add(1).min(max_height)))
    }
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < MIN_WIDTH || area.height < 1 {
            return;
        }

        let current_theme = use_theme().get();
        let width = self.width.unwrap_or(area.width.min(35)).max(MIN_WIDTH);

        let fg = self.fg.unwrap_or(current_theme.colors.text);
        let bg = self.bg.unwrap_or(current_theme.colors.surface);

        if self.compact {
            self.render_compact(ctx, &current_theme);
        } else {
            self.render_header(ctx, &current_theme, width, fg, bg);

            if self.open && !self.themes.is_empty() {
                self.render_dropdown(ctx, &current_theme, width);
            }
        }
    }
}

impl ThemePicker {
    /// Draw color swatch at position, returns width used
    fn draw_swatch(&self, ctx: &mut RenderContext, x: u16, y: u16, theme: &Theme) -> u16 {
        let swatch_colors = [
            theme.colors.background,
            theme.palette.primary,
            theme.palette.success,
            theme.palette.error,
        ];

        for (i, color) in swatch_colors.iter().enumerate() {
            let mut cell = Cell::new(' ');
            cell.bg = Some(*color);
            ctx.set(x + i as u16, y, cell);
        }

        swatch_colors.len() as u16
    }

    /// Compact mode: the current theme's swatch, and when open one row per
    /// theme with a `>` marker and its swatch
    fn render_compact(&self, ctx: &mut RenderContext, current_theme: &Theme) {
        let area = ctx.area;
        self.draw_swatch(ctx, 0, 0, current_theme);

        if self.open && !self.themes.is_empty() {
            let mut y: u16 = 1;
            for (i, theme_id) in self.themes.iter().enumerate() {
                if y >= area.height {
                    break;
                }
                if let Some(theme) = get_theme(theme_id) {
                    let selected = i == self.selected_index;

                    // Selection indicator
                    let indicator = if selected { '>' } else { ' ' };
                    put(ctx, 0, y, indicator, Some(theme.palette.primary), None);

                    // Swatch
                    self.draw_swatch(ctx, 1, y, &theme);
                    y += 1;
                }
            }
        }
    }

    /// Full mode's first row: `Theme: <name>`, the swatch, the arrow, and
    /// the background up to `width`
    fn render_header(
        &self,
        ctx: &mut RenderContext,
        current_theme: &Theme,
        width: u16,
        fg: Color,
        bg: Color,
    ) {
        // "Theme: " label
        let mut x = draw_run(ctx, 0, 0, "Theme: ", fg, bg);

        // Theme name
        x += ctx.put_str_with(x, 0, &current_theme.name, width.saturating_sub(6), |ch| {
            Cell::new(ch).fg(fg).bg(bg)
        });

        if self.show_preview {
            // Space
            put(ctx, x, 0, ' ', None, Some(bg));
            x += 1;

            // Swatch
            x += self.draw_swatch(ctx, x, 0, current_theme);
        }

        // Dropdown indicator
        let indicator = if self.open { " ▲" } else { " ▼" };
        x = draw_run(ctx, x, 0, indicator, fg, bg);

        // Fill remaining header
        while x < width {
            put(ctx, x, 0, ' ', None, Some(bg));
            x += 1;
        }
    }

    /// The open list under the header: a box `width` wide with one row per
    /// theme, cut to the area's height
    fn render_dropdown(&self, ctx: &mut RenderContext, current_theme: &Theme, width: u16) {
        let area = ctx.area;
        let border_color = current_theme.colors.border;
        let mut y: u16 = 1;

        // Border top
        if y < area.height {
            draw_border_row(ctx, y, width, ('┌', '┐'), border_color);
            y += 1;
        }

        // Theme list: the rows between the borders, scrolled to keep the
        // selected theme in view (#973)
        let known: Vec<(usize, Theme)> = self
            .themes
            .iter()
            .enumerate()
            .filter_map(|(i, id)| get_theme(id).map(|theme| (i, theme)))
            .collect();
        let room = usize::from(area.height.saturating_sub(y + 1));
        self.scroll.set_content(known.len());
        self.scroll.set_viewport(room.min(known.len()));
        if let Some(row) = known.iter().position(|(i, _)| *i == self.selected_index) {
            self.scroll.track(row..row + 1);
        }
        for (i, theme) in &known[self.scroll.visible()] {
            let selected = *i == self.selected_index;
            self.draw_item(ctx, y, width, current_theme, theme, selected);
            y += 1;
        }

        // Border bottom
        if y < area.height {
            draw_border_row(ctx, y, width, ('└', '┘'), border_color);
        }
    }

    /// One dropdown row: borders, the `▶` marker, the swatch and the name,
    /// in the selection colors when `selected`
    fn draw_item(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        width: u16,
        current_theme: &Theme,
        theme: &Theme,
        selected: bool,
    ) {
        let border_color = current_theme.colors.border;
        let item_bg = if selected {
            current_theme.colors.selection
        } else {
            current_theme.colors.surface
        };
        let item_fg = if selected {
            current_theme.colors.selection_text
        } else {
            current_theme.colors.text
        };

        // Left border
        put(ctx, 0, y, '│', Some(border_color), None);

        let mut cx: u16 = 1;

        // Selection indicator
        let indicator = if selected { '▶' } else { ' ' };
        put(
            ctx,
            cx,
            y,
            indicator,
            Some(theme.palette.primary),
            Some(item_bg),
        );
        cx += 1;

        if self.show_preview {
            // Swatch
            cx += self.draw_swatch(ctx, cx, y, theme);
        }

        // Space
        put(ctx, cx, y, ' ', None, Some(item_bg));
        cx += 1;

        // Name, up to the right border
        let mut name = Cell::new(' ');
        name.fg = Some(item_fg);
        name.bg = Some(item_bg);
        cx += ctx.put_str_with(cx, y, &theme.name, width.saturating_sub(1), |ch| {
            let mut cell = name;
            cell.symbol = ch;
            cell
        });

        // Padding
        while cx < width - 1 {
            put(ctx, cx, y, ' ', None, Some(item_bg));
            cx += 1;
        }

        // Right border
        put(ctx, width - 1, y, '│', Some(border_color), None);
    }
}

/// Set one cell with these colors
fn put(ctx: &mut RenderContext, x: u16, y: u16, ch: char, fg: Option<Color>, bg: Option<Color>) {
    let mut cell = Cell::new(ch);
    cell.fg = fg;
    cell.bg = bg;
    ctx.set(x, y, cell);
}

/// Draw `text` in terminal columns from `x`; returns the column after it
fn draw_run(ctx: &mut RenderContext, x: u16, y: u16, text: &str, fg: Color, bg: Color) -> u16 {
    x + ctx.put_str_with(x, y, text, u16::MAX, |ch| Cell::new(ch).fg(fg).bg(bg))
}

/// A box's top or bottom row `width` wide: the two corners and `─` between
fn draw_border_row(
    ctx: &mut RenderContext,
    y: u16,
    width: u16,
    (left, right): (char, char),
    color: Color,
) {
    put(ctx, 0, y, left, Some(color), None);

    for i in 1..width.saturating_sub(1) {
        put(ctx, i, y, '─', Some(color), None);
    }

    put(ctx, width - 1, y, right, Some(color), None);
}
