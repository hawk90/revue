//! Pagination widget for page navigation

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Pagination style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaginationStyle {
    /// Full pagination with page numbers
    #[default]
    Full,
    /// Simple prev/next only
    Simple,
    /// Compact with current/total display
    Compact,
    /// Dots indicator
    Dots,
}

/// A pagination widget for navigating pages
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// pagination(10)
///     .current(3)
///     .on_change(|page| { /* ... */ })
/// ```
pub struct Pagination {
    /// Total number of pages
    total: u16,
    /// Current page (1-indexed)
    current: u16,
    /// Style
    style: PaginationStyle,
    /// Max visible page buttons
    max_visible: u16,
    /// Show prev/next buttons
    show_arrows: bool,
    /// Show first/last buttons
    show_edges: bool,
    /// Active color
    active_color: Color,
    /// Inactive color
    inactive_color: Color,
    /// Is focused
    focused: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Pagination {
    /// Create a new pagination with total pages
    pub fn new(total: u16) -> Self {
        Self {
            total,
            current: 1,
            style: PaginationStyle::Full,
            max_visible: 7,
            show_arrows: true,
            show_edges: true,
            active_color: Color::rgb(60, 120, 200),
            inactive_color: Color::rgb(120, 120, 120),
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set current page
    pub fn current(mut self, page: u16) -> Self {
        self.current = page.max(1).min(self.total);
        self
    }

    /// Set style
    pub fn style(mut self, style: PaginationStyle) -> Self {
        self.style = style;
        self
    }

    /// Simple style shorthand
    pub fn simple(mut self) -> Self {
        self.style = PaginationStyle::Simple;
        self
    }

    /// Compact style shorthand
    pub fn compact(mut self) -> Self {
        self.style = PaginationStyle::Compact;
        self
    }

    /// Dots style shorthand
    pub fn dots(mut self) -> Self {
        self.style = PaginationStyle::Dots;
        self
    }

    /// Set max visible page buttons
    pub fn max_visible(mut self, max: u16) -> Self {
        self.max_visible = max.max(3);
        self
    }

    /// Hide arrows
    pub fn no_arrows(mut self) -> Self {
        self.show_arrows = false;
        self
    }

    /// Hide first/last buttons
    pub fn no_edges(mut self) -> Self {
        self.show_edges = false;
        self
    }

    /// Set active color
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Set inactive color
    pub fn inactive_color(mut self, color: Color) -> Self {
        self.inactive_color = color;
        self
    }

    /// Set focused state
    pub fn focused(mut self) -> Self {
        self.focused = true;
        self
    }

    /// Go to next page
    pub fn next_page(&mut self) -> bool {
        if self.current < self.total {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Go to previous page
    pub fn prev_page(&mut self) -> bool {
        if self.current > 1 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    /// Go to first page
    pub fn first(&mut self) {
        self.current = 1;
    }

    /// Go to last page
    pub fn last(&mut self) {
        self.current = self.total;
    }

    /// Go to specific page
    pub fn goto(&mut self, page: u16) {
        self.current = page.max(1).min(self.total);
    }

    /// Get current page
    pub fn get_current(&self) -> u16 {
        self.current
    }

    /// Get total pages
    pub fn get_total(&self) -> u16 {
        self.total
    }

    /// Set total pages
    pub fn set_total(&mut self, total: u16) {
        self.total = total;
        if self.current > total {
            self.current = total.max(1);
        }
    }

    /// Check if on first page
    pub fn is_first(&self) -> bool {
        self.current == 1
    }

    /// Check if on last page
    pub fn is_last(&self) -> bool {
        self.current == self.total
    }

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

        let end = (start + self.max_visible - 1).min(self.total);
        (start.max(1), end)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new(1)
    }
}

impl View for Pagination {
    crate::impl_view_meta!("Pagination");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let mut x = area.x;

        match self.style {
            PaginationStyle::Full => {
                // « < 1 2 [3] 4 5 > »
                if self.show_edges {
                    // First button
                    let color = if self.is_first() {
                        self.inactive_color
                    } else {
                        self.active_color
                    };
                    let mut cell = Cell::new('«');
                    cell.fg = Some(color);
                    ctx.buffer.set(x, area.y, cell);
                    x += 2;
                }

                if self.show_arrows {
                    // Prev button
                    let color = if self.is_first() {
                        self.inactive_color
                    } else {
                        self.active_color
                    };
                    let mut cell = Cell::new('‹');
                    cell.fg = Some(color);
                    ctx.buffer.set(x, area.y, cell);
                    x += 2;
                }

                // Page numbers
                let (start, end) = self.visible_range();

                // Ellipsis before
                if start > 1 {
                    let mut one = Cell::new('1');
                    one.fg = Some(self.inactive_color);
                    ctx.buffer.set(x, area.y, one);
                    x += 2;

                    if start > 2 {
                        let mut dots = Cell::new('…');
                        dots.fg = Some(self.inactive_color);
                        ctx.buffer.set(x, area.y, dots);
                        x += 2;
                    }
                }

                // Page numbers
                for page in start..=end {
                    let is_current = page == self.current;

                    if is_current {
                        let mut lb = Cell::new('[');
                        lb.fg = Some(self.active_color);
                        ctx.buffer.set(x, area.y, lb);
                        x += 1;
                    }

                    let page_str = page.to_string();
                    for ch in page_str.chars() {
                        let mut cell = Cell::new(ch);
                        if is_current {
                            cell.fg = Some(self.active_color);
                            cell.modifier |= Modifier::BOLD;
                        } else {
                            cell.fg = Some(self.inactive_color);
                        }
                        ctx.buffer.set(x, area.y, cell);
                        x += 1;
                    }

                    if is_current {
                        let mut rb = Cell::new(']');
                        rb.fg = Some(self.active_color);
                        ctx.buffer.set(x, area.y, rb);
                        x += 1;
                    }

                    x += 1; // Space between
                }

                // Ellipsis after
                if end < self.total {
                    if end < self.total - 1 {
                        let mut dots = Cell::new('…');
                        dots.fg = Some(self.inactive_color);
                        ctx.buffer.set(x, area.y, dots);
                        x += 2;
                    }

                    let total_str = self.total.to_string();
                    for ch in total_str.chars() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.inactive_color);
                        ctx.buffer.set(x, area.y, cell);
                        x += 1;
                    }
                    x += 1;
                }

                if self.show_arrows {
                    // Next button
                    let color = if self.is_last() {
                        self.inactive_color
                    } else {
                        self.active_color
                    };
                    let mut cell = Cell::new('›');
                    cell.fg = Some(color);
                    ctx.buffer.set(x, area.y, cell);
                    x += 2;
                }

                if self.show_edges {
                    // Last button
                    let color = if self.is_last() {
                        self.inactive_color
                    } else {
                        self.active_color
                    };
                    let mut cell = Cell::new('»');
                    cell.fg = Some(color);
                    ctx.buffer.set(x, area.y, cell);
                }
            }
            PaginationStyle::Simple => {
                // ← Page 3 of 10 →
                let prev_color = if self.is_first() {
                    self.inactive_color
                } else {
                    self.active_color
                };
                let mut prev = Cell::new('←');
                prev.fg = Some(prev_color);
                ctx.buffer.set(x, area.y, prev);
                x += 2;

                let text = format!("Page {} of {}", self.current, self.total);
                for ch in text.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.inactive_color);
                    ctx.buffer.set(x, area.y, cell);
                    x += 1;
                }
                x += 1;

                let next_color = if self.is_last() {
                    self.inactive_color
                } else {
                    self.active_color
                };
                let mut next = Cell::new('→');
                next.fg = Some(next_color);
                ctx.buffer.set(x, area.y, next);
            }
            PaginationStyle::Compact => {
                // 3/10
                let text = format!("{}/{}", self.current, self.total);
                for ch in text.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.active_color);
                    cell.modifier |= Modifier::BOLD;
                    ctx.buffer.set(x, area.y, cell);
                    x += 1;
                }
            }
            PaginationStyle::Dots => {
                // ○ ○ ● ○ ○
                for page in 1..=self.total {
                    if x >= area.x + area.width {
                        break;
                    }

                    let is_current = page == self.current;
                    let ch = if is_current { '●' } else { '○' };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if is_current {
                        self.active_color
                    } else {
                        self.inactive_color
                    });
                    ctx.buffer.set(x, area.y, cell);
                    x += 2;
                }
            }
        }
    }
}

impl_styled_view!(Pagination);
impl_props_builders!(Pagination);

/// Create a new pagination
pub fn pagination(total: u16) -> Pagination {
    Pagination::new(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_pagination_new() {
        let p = Pagination::new(10);
        assert_eq!(p.total, 10);
        assert_eq!(p.current, 1);
    }

    #[test]
    fn test_pagination_navigation() {
        let mut p = Pagination::new(10);

        assert!(p.next_page());
        assert_eq!(p.current, 2);

        assert!(p.prev_page());
        assert_eq!(p.current, 1);

        assert!(!p.prev_page()); // Can't go below 1

        p.last();
        assert_eq!(p.current, 10);

        assert!(!p.next_page()); // Can't go above total

        p.first();
        assert_eq!(p.current, 1);

        p.goto(5);
        assert_eq!(p.current, 5);
    }

    #[test]
    fn test_pagination_styles() {
        let p = pagination(10).simple();
        assert_eq!(p.style, PaginationStyle::Simple);

        let p = pagination(10).compact();
        assert_eq!(p.style, PaginationStyle::Compact);

        let p = pagination(10).dots();
        assert_eq!(p.style, PaginationStyle::Dots);
    }

    #[test]
    fn test_pagination_visible_range() {
        let p = pagination(20).current(10);
        let (start, end) = p.visible_range();
        assert!(start <= p.current);
        assert!(end >= p.current);
    }

    #[test]
    fn test_pagination_is_first_last() {
        let mut p = pagination(10);
        assert!(p.is_first());
        assert!(!p.is_last());

        p.last();
        assert!(!p.is_first());
        assert!(p.is_last());
    }

    #[test]
    fn test_pagination_render_full() {
        let mut buffer = Buffer::new(50, 1);
        let area = Rect::new(0, 0, 50, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = pagination(10).current(5);
        p.render(&mut ctx);

        // Should have navigation symbols
        let text: String = (0..50)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains('5'));
    }

    #[test]
    fn test_pagination_render_dots() {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = pagination(5).current(3).dots();
        p.render(&mut ctx);

        let text: String = (0..30)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains('●'));
        assert!(text.contains('○'));
    }

    #[test]
    fn test_set_total() {
        let mut p = pagination(10).current(8);
        p.set_total(5);
        assert_eq!(p.total, 5);
        assert_eq!(p.current, 5); // Clamped to new total
    }

    #[test]
    fn test_helper_function() {
        let p = pagination(15);
        assert_eq!(p.total, 15);
    }
}
