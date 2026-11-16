//! Drawing the pagination: one row, laid out by its style

use super::{Pagination, PaginationStyle};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::SUBTLE_GRAY;
use crate::widget::traits::{RenderContext, View};

/// Draw `ch` at column `x` of the first row in `fg`, with `modifier`.
fn put(ctx: &mut RenderContext, x: u16, ch: char, fg: Color, modifier: Modifier) {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.modifier |= modifier;
    ctx.set(x, 0, cell);
}

/// Draw `text` one cell per character from column `x`; returns the column
/// after it.
fn put_str(ctx: &mut RenderContext, mut x: u16, text: &str, fg: Color, modifier: Modifier) -> u16 {
    for ch in text.chars() {
        put(ctx, x, ch, fg, modifier);
        x = x.saturating_add(1);
    }
    x
}

/// The pages Full draws: the window `start..=end`, and whether page 1 and
/// the last page are added where the window does not reach them
#[derive(Clone, Copy)]
struct Pages {
    start: u16,
    end: u16,
    ends: bool,
}

impl Pagination {
    /// The window of `max_visible` pages, with both ends, as wide as the row
    /// may be
    fn visible_pages(&self) -> Pages {
        let (start, end) = self.window(self.max_visible);
        Pages {
            start,
            end,
            ends: true,
        }
    }

    /// A run of `size` pages (all of them when fewer) with the current page
    /// as close to its middle as the ends allow
    fn window(&self, size: u16) -> (u16, u16) {
        let size = size.max(1);
        let half = size / 2;
        let start = if self.current <= half {
            1
        } else if self.current >= self.total.saturating_sub(half) {
            self.total.saturating_sub(size - 1)
        } else {
            self.current - half
        };

        let end = start.saturating_add(size - 1).min(self.total);
        (start.max(1), end)
    }

    /// The pages Full draws in `width` columns (#1104): `max_visible` pages
    /// and both ends, or as many fewer pages as it takes for the row to fit,
    /// down to the current page alone; if that still does not fit, without
    /// the ends. The window always holds the current page, so a narrow row
    /// gives up the pages beside it first, then page 1 and the last page,
    /// and the current page last.
    fn fitted_pages(&self, width: u16) -> Pages {
        // A page takes at least two columns, so more than `width / 2 + 1`
        // pages never fit; this also bounds the search for a huge
        // `max_visible` (#1371).
        let most = self.max_visible.min(self.total.max(1)).min(width / 2 + 1);
        (1..=most)
            .rev()
            .map(|size| {
                let (start, end) = self.window(size);
                Pages {
                    start,
                    end,
                    ends: true,
                }
            })
            .find(|&pages| self.full_width(pages) <= width)
            .unwrap_or_else(|| {
                let (start, end) = self.window(1);
                Pages {
                    start,
                    end,
                    ends: false,
                }
            })
    }

    /// The color of a button that moves away from an end: inactive when the
    /// pagination is already at that end.
    fn nav_color(&self, at_end: bool, inactive_color: Color) -> Color {
        if at_end {
            inactive_color
        } else {
            self.active_color
        }
    }

    /// The columns the current style draws. Follows the `render_*` methods.
    fn natural_width(&self) -> u16 {
        let digits = |n: u16| n.to_string().len() as u16;
        match self.style {
            PaginationStyle::Full => self.full_width(self.visible_pages()),
            // `← ` the text ` →`
            PaginationStyle::Simple => 8 + 4 + digits(self.current) + digits(self.total) + 1,
            PaginationStyle::Compact => digits(self.current) + digits(self.total) + 1,
            PaginationStyle::Dots => (self.total.saturating_mul(2)).saturating_sub(1),
        }
    }

    /// The columns Full draws with `pages`. Follows `render_full`.
    fn full_width(&self, Pages { start, end, ends }: Pages) -> u16 {
        let digits = |n: u16| n.to_string().len() as u16;
        let mut x = 0u16;
        // The button after the pages: its own column, no space after.
        if self.show_edges {
            x = x.saturating_add(2 + 1);
        }
        if self.show_arrows {
            x = x.saturating_add(2 + 2);
        }
        if ends && start > 1 {
            x = x.saturating_add(2 + if start > 2 { 2 } else { 0 });
        }
        for page in start..=end {
            x = x.saturating_add(digits(page) + 1);
            if page == self.current {
                x = x.saturating_add(2);
            }
        }
        if ends && end < self.total {
            if end < self.total - 1 {
                x = x.saturating_add(2);
            }
            x = x.saturating_add(digits(self.total) + 1);
        }
        // Without a last button, the space after the final page or arrow is
        // not drawn.
        if self.show_edges {
            x
        } else {
            x.saturating_sub(1)
        }
    }

    /// Full: `« ‹ 1 2 [3] 4 5 › »`
    fn render_full(&self, ctx: &mut RenderContext, inactive_color: Color, focus_mark: Modifier) {
        let mut x: u16 = 0;
        if self.show_edges {
            // First button
            let color = self.nav_color(self.is_first(), inactive_color);
            put(ctx, x, '«', color, Modifier::empty());
            x = x.saturating_add(2);
        }

        if self.show_arrows {
            // Prev button
            let color = self.nav_color(self.is_first(), inactive_color);
            put(ctx, x, '‹', color, Modifier::empty());
            x = x.saturating_add(2);
        }

        let pages = self.fitted_pages(ctx.area.width);
        x = self.render_pages(ctx, x, pages, inactive_color, focus_mark);

        if self.show_arrows {
            // Next button
            let color = self.nav_color(self.is_last(), inactive_color);
            put(ctx, x, '›', color, Modifier::empty());
            x = x.saturating_add(2);
        }

        if self.show_edges {
            // Last button
            let color = self.nav_color(self.is_last(), inactive_color);
            put(ctx, x, '»', color, Modifier::empty());
        }
    }

    /// The pages from column `x`: the window, and (with `ends`) the first and
    /// last page and an ellipsis where the window does not reach them;
    /// returns the column after it.
    fn render_pages(
        &self,
        ctx: &mut RenderContext,
        mut x: u16,
        Pages { start, end, ends }: Pages,
        inactive_color: Color,
        focus_mark: Modifier,
    ) -> u16 {
        // Ellipsis before
        if ends && start > 1 {
            put(ctx, x, '1', inactive_color, Modifier::empty());
            x = x.saturating_add(2);

            if start > 2 {
                put(ctx, x, '…', inactive_color, Modifier::empty());
                x = x.saturating_add(2);
            }
        }

        // Page numbers
        for page in start..=end {
            x = self.render_page(ctx, x, page, inactive_color, focus_mark);
        }

        // Ellipsis after
        if ends && end < self.total {
            if end < self.total - 1 {
                put(ctx, x, '…', inactive_color, Modifier::empty());
                x = x.saturating_add(2);
            }

            x = put_str(
                ctx,
                x,
                &self.total.to_string(),
                inactive_color,
                Modifier::empty(),
            );
            x = x.saturating_add(1);
        }
        x
    }

    /// One page number from column `x`, bracketed and bold when it is the
    /// current page; returns the column after it and its space.
    fn render_page(
        &self,
        ctx: &mut RenderContext,
        mut x: u16,
        page: u16,
        inactive_color: Color,
        focus_mark: Modifier,
    ) -> u16 {
        let is_current = page == self.current;

        if is_current {
            put(ctx, x, '[', self.active_color, Modifier::empty());
            x = x.saturating_add(1);
        }

        let (fg, modifier) = if is_current {
            (self.active_color, Modifier::BOLD | focus_mark)
        } else {
            (inactive_color, Modifier::empty())
        };
        x = put_str(ctx, x, &page.to_string(), fg, modifier);

        if is_current {
            put(ctx, x, ']', self.active_color, Modifier::empty());
            x = x.saturating_add(1);
        }

        x = x.saturating_add(1); // Space between
        x
    }

    /// Simple: `← Page 3 of 10 →`
    fn render_simple(&self, ctx: &mut RenderContext, inactive_color: Color, focus_mark: Modifier) {
        let mut x: u16 = 0;
        let prev_color = self.nav_color(self.is_first(), inactive_color);
        put(ctx, x, '←', prev_color, Modifier::empty());
        x = x.saturating_add(2);

        let text = format!("Page {} of {}", self.current, self.total);
        x = put_str(ctx, x, &text, inactive_color, focus_mark);
        x = x.saturating_add(1);

        let next_color = self.nav_color(self.is_last(), inactive_color);
        put(ctx, x, '→', next_color, Modifier::empty());
    }

    /// Compact: `3/10`
    fn render_compact(&self, ctx: &mut RenderContext, focus_mark: Modifier) {
        let text = format!("{}/{}", self.current, self.total);
        put_str(
            ctx,
            0,
            &text,
            self.active_color,
            Modifier::BOLD | focus_mark,
        );
    }

    /// Dots: `○ ○ ● ○ ○`, as many as fit, around the current page (#1104)
    fn render_dots(&self, ctx: &mut RenderContext, inactive_color: Color, focus_mark: Modifier) {
        let (start, end) = self.window(ctx.area.width.div_ceil(2));
        let mut x: u16 = 0;
        for page in start..=end {
            let is_current = page == self.current;
            let ch = if is_current { '●' } else { '○' };
            let (fg, modifier) = if is_current {
                (self.active_color, focus_mark)
            } else {
                (inactive_color, Modifier::empty())
            };
            put(ctx, x, ch, fg, modifier);
            x = x.saturating_add(2);
        }
    }
}

impl View for Pagination {
    /// One row as wide as the current style draws it.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((self.natural_width().min(max_width), 1.min(max_height)))
    }
    crate::impl_view_meta!("Pagination", focusable);

    fn render(&self, ctx: &mut RenderContext) {
        // Inactive page numbers take `color`; the active one keeps its highlight.
        let inactive_color = self
            .inactive_color
            .unwrap_or_else(|| ctx.css_color(SUBTLE_GRAY));
        // Focused, the current page is underlined.
        let focus_mark = if self.focused {
            Modifier::UNDERLINE
        } else {
            Modifier::empty()
        };

        match self.style {
            PaginationStyle::Full => self.render_full(ctx, inactive_color, focus_mark),
            PaginationStyle::Simple => self.render_simple(ctx, inactive_color, focus_mark),
            PaginationStyle::Compact => self.render_compact(ctx, focus_mark),
            PaginationStyle::Dots => self.render_dots(ctx, inactive_color, focus_mark),
        }
    }
}
