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
        x += 1;
    }
    x
}

impl Pagination {
    /// Calculate visible page range
    fn visible_range(&self) -> (u16, u16) {
        let half = self.max_visible / 2;
        let start = if self.current <= half {
            1
        } else if self.current >= self.total - half {
            self.total.saturating_sub(self.max_visible - 1)
        } else {
            self.current - half
        };

        let end = start.saturating_add(self.max_visible - 1).min(self.total);
        (start.max(1), end)
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
            PaginationStyle::Full => {
                let (start, end) = self.visible_range();
                let mut x = 0u16;
                // The button after the pages: its own column, no space after.
                if self.show_edges {
                    x += 2 + 1;
                }
                if self.show_arrows {
                    x += 2 + 2;
                }
                if start > 1 {
                    x += 2 + if start > 2 { 2 } else { 0 };
                }
                for page in start..=end {
                    x = x.saturating_add(digits(page) + 1);
                    if page == self.current {
                        x += 2;
                    }
                }
                if end < self.total {
                    if end < self.total - 1 {
                        x += 2;
                    }
                    x = x.saturating_add(digits(self.total) + 1);
                }
                // Without a last button, the space after the final page or
                // arrow is not drawn.
                if self.show_edges {
                    x
                } else {
                    x.saturating_sub(1)
                }
            }
            // `← ` the text ` →`
            PaginationStyle::Simple => 8 + 4 + digits(self.current) + digits(self.total) + 1,
            PaginationStyle::Compact => digits(self.current) + digits(self.total) + 1,
            PaginationStyle::Dots => (self.total.saturating_mul(2)).saturating_sub(1),
        }
    }

    /// Full: `« ‹ 1 2 [3] 4 5 › »`
    fn render_full(&self, ctx: &mut RenderContext, inactive_color: Color, focus_mark: Modifier) {
        let mut x: u16 = 0;
        if self.show_edges {
            // First button
            let color = self.nav_color(self.is_first(), inactive_color);
            put(ctx, x, '«', color, Modifier::empty());
            x += 2;
        }

        if self.show_arrows {
            // Prev button
            let color = self.nav_color(self.is_first(), inactive_color);
            put(ctx, x, '‹', color, Modifier::empty());
            x += 2;
        }

        x = self.render_pages(ctx, x, inactive_color, focus_mark);

        if self.show_arrows {
            // Next button
            let color = self.nav_color(self.is_last(), inactive_color);
            put(ctx, x, '›', color, Modifier::empty());
            x += 2;
        }

        if self.show_edges {
            // Last button
            let color = self.nav_color(self.is_last(), inactive_color);
            put(ctx, x, '»', color, Modifier::empty());
        }
    }

    /// The page window from column `x`, with the first and last page and an
    /// ellipsis where the window does not reach them; returns the column
    /// after it.
    fn render_pages(
        &self,
        ctx: &mut RenderContext,
        mut x: u16,
        inactive_color: Color,
        focus_mark: Modifier,
    ) -> u16 {
        let (start, end) = self.visible_range();

        // Ellipsis before
        if start > 1 {
            put(ctx, x, '1', inactive_color, Modifier::empty());
            x += 2;

            if start > 2 {
                put(ctx, x, '…', inactive_color, Modifier::empty());
                x += 2;
            }
        }

        // Page numbers
        for page in start..=end {
            x = self.render_page(ctx, x, page, inactive_color, focus_mark);
        }

        // Ellipsis after
        if end < self.total {
            if end < self.total - 1 {
                put(ctx, x, '…', inactive_color, Modifier::empty());
                x += 2;
            }

            x = put_str(
                ctx,
                x,
                &self.total.to_string(),
                inactive_color,
                Modifier::empty(),
            );
            x += 1;
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
            x += 1;
        }

        let (fg, modifier) = if is_current {
            (self.active_color, Modifier::BOLD | focus_mark)
        } else {
            (inactive_color, Modifier::empty())
        };
        x = put_str(ctx, x, &page.to_string(), fg, modifier);

        if is_current {
            put(ctx, x, ']', self.active_color, Modifier::empty());
            x += 1;
        }

        x += 1; // Space between
        x
    }

    /// Simple: `← Page 3 of 10 →`
    fn render_simple(&self, ctx: &mut RenderContext, inactive_color: Color, focus_mark: Modifier) {
        let mut x: u16 = 0;
        let prev_color = self.nav_color(self.is_first(), inactive_color);
        put(ctx, x, '←', prev_color, Modifier::empty());
        x += 2;

        let text = format!("Page {} of {}", self.current, self.total);
        x = put_str(ctx, x, &text, inactive_color, focus_mark);
        x += 1;

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

    /// Dots: `○ ○ ● ○ ○`
    fn render_dots(&self, ctx: &mut RenderContext, inactive_color: Color, focus_mark: Modifier) {
        let area = ctx.area;
        let mut x: u16 = 0;
        for page in 1..=self.total {
            if x >= area.width {
                break;
            }

            let is_current = page == self.current;
            let ch = if is_current { '●' } else { '○' };
            let (fg, modifier) = if is_current {
                (self.active_color, focus_mark)
            } else {
                (inactive_color, Modifier::empty())
            };
            put(ctx, x, ch, fg, modifier);
            x += 2;
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
