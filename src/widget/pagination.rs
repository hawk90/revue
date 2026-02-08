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

    // =========================================================================
    // PaginationStyle enum tests
    // =========================================================================

    #[test]
    fn test_pagination_style_default() {
        assert_eq!(PaginationStyle::default(), PaginationStyle::Full);
    }

    #[test]
    fn test_pagination_style_clone() {
        let style = PaginationStyle::Simple;
        assert_eq!(style, style.clone());
    }

    #[test]
    fn test_pagination_style_copy() {
        let s1 = PaginationStyle::Compact;
        let s2 = s1;
        assert_eq!(s1, PaginationStyle::Compact);
        assert_eq!(s2, PaginationStyle::Compact);
    }

    #[test]
    fn test_pagination_style_debug() {
        let debug_str = format!("{:?}", PaginationStyle::Dots);
        assert!(debug_str.contains("Dots"));
    }

    #[test]
    fn test_pagination_style_partial_eq() {
        assert_eq!(PaginationStyle::Full, PaginationStyle::Full);
        assert_eq!(PaginationStyle::Simple, PaginationStyle::Simple);
        assert_eq!(PaginationStyle::Compact, PaginationStyle::Compact);
        assert_eq!(PaginationStyle::Dots, PaginationStyle::Dots);
        assert_ne!(PaginationStyle::Full, PaginationStyle::Simple);
    }

    // =========================================================================
    // Pagination::new tests
    // =========================================================================

    #[test]
    fn test_pagination_new() {
        let p = Pagination::new(10);
        assert_eq!(p.total, 10);
        assert_eq!(p.current, 1);
        assert_eq!(p.style, PaginationStyle::Full);
        assert_eq!(p.max_visible, 7);
        assert!(p.show_arrows);
        assert!(p.show_edges);
        assert!(!p.focused);
    }

    #[test]
    fn test_pagination_new_single_page() {
        let p = Pagination::new(1);
        assert_eq!(p.total, 1);
        assert_eq!(p.current, 1);
    }

    // =========================================================================
    // Pagination builder tests
    // =========================================================================

    #[test]
    fn test_pagination_current() {
        let p = Pagination::new(10).current(5);
        assert_eq!(p.current, 5);
    }

    #[test]
    fn test_pagination_current_clamps_low() {
        let p = Pagination::new(10).current(0);
        assert_eq!(p.current, 1); // Clamped to 1
    }

    #[test]
    fn test_pagination_current_clamps_high() {
        let p = Pagination::new(10).current(15);
        assert_eq!(p.current, 10); // Clamped to total
    }

    #[test]
    fn test_pagination_style() {
        let p = Pagination::new(10).style(PaginationStyle::Compact);
        assert_eq!(p.style, PaginationStyle::Compact);
    }

    #[test]
    fn test_pagination_simple() {
        let p = Pagination::new(10).simple();
        assert_eq!(p.style, PaginationStyle::Simple);
    }

    #[test]
    fn test_pagination_compact() {
        let p = Pagination::new(10).compact();
        assert_eq!(p.style, PaginationStyle::Compact);
    }

    #[test]
    fn test_pagination_dots() {
        let p = Pagination::new(10).dots();
        assert_eq!(p.style, PaginationStyle::Dots);
    }

    #[test]
    fn test_pagination_max_visible() {
        let p = Pagination::new(10).max_visible(5);
        assert_eq!(p.max_visible, 5);
    }

    #[test]
    fn test_pagination_max_visible_clamps() {
        let p = Pagination::new(10).max_visible(2);
        assert_eq!(p.max_visible, 3); // Clamped to minimum 3
    }

    #[test]
    fn test_pagination_no_arrows() {
        let p = Pagination::new(10).no_arrows();
        assert!(!p.show_arrows);
    }

    #[test]
    fn test_pagination_no_edges() {
        let p = Pagination::new(10).no_edges();
        assert!(!p.show_edges);
    }

    #[test]
    fn test_pagination_active_color() {
        let p = Pagination::new(10).active_color(Color::RED);
        assert_eq!(p.active_color, Color::RED);
    }

    #[test]
    fn test_pagination_inactive_color() {
        let p = Pagination::new(10).inactive_color(Color::BLUE);
        assert_eq!(p.inactive_color, Color::BLUE);
    }

    #[test]
    fn test_pagination_focused() {
        let p = Pagination::new(10).focused();
        assert!(p.focused);
    }

    #[test]
    fn test_pagination_builder_chain() {
        let p = Pagination::new(20)
            .current(5)
            .simple()
            .max_visible(5)
            .no_arrows()
            .no_edges()
            .active_color(Color::CYAN)
            .inactive_color(Color::rgb(128, 128, 128))
            .focused();

        assert_eq!(p.total, 20);
        assert_eq!(p.current, 5);
        assert_eq!(p.style, PaginationStyle::Simple);
        assert_eq!(p.max_visible, 5);
        assert!(!p.show_arrows);
        assert!(!p.show_edges);
        assert!(p.focused);
    }

    // =========================================================================
    // Pagination navigation tests
    // =========================================================================

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
    fn test_next_page_at_end() {
        let mut p = Pagination::new(5).current(5);
        assert!(!p.next_page());
        assert_eq!(p.current, 5);
    }

    #[test]
    fn test_prev_page_at_start() {
        let mut p = Pagination::new(5);
        assert!(!p.prev_page());
        assert_eq!(p.current, 1);
    }

    #[test]
    fn test_first() {
        let mut p = Pagination::new(10).current(5);
        p.first();
        assert_eq!(p.current, 1);
    }

    #[test]
    fn test_last() {
        let mut p = Pagination::new(10).current(5);
        p.last();
        assert_eq!(p.current, 10);
    }

    #[test]
    fn test_goto_clamps_low() {
        let mut p = Pagination::new(10).current(5);
        p.goto(0);
        assert_eq!(p.current, 1);
    }

    #[test]
    fn test_goto_clamps_high() {
        let mut p = Pagination::new(10).current(5);
        p.goto(20);
        assert_eq!(p.current, 10);
    }

    #[test]
    fn test_goto_middle() {
        let mut p = Pagination::new(10);
        p.goto(5);
        assert_eq!(p.current, 5);
    }

    #[test]
    fn test_goto_same_page() {
        let mut p = Pagination::new(10).current(5);
        p.goto(5);
        assert_eq!(p.current, 5);
    }

    // =========================================================================
    // Pagination query tests
    // =========================================================================

    #[test]
    fn test_get_current() {
        let p = Pagination::new(10).current(5);
        assert_eq!(p.get_current(), 5);
    }

    #[test]
    fn test_get_total() {
        let p = Pagination::new(10);
        assert_eq!(p.get_total(), 10);
    }

    #[test]
    fn test_is_first() {
        let p = Pagination::new(10).current(1);
        assert!(p.is_first());
    }

    #[test]
    fn test_is_first_false() {
        let p = Pagination::new(10).current(5);
        assert!(!p.is_first());
    }

    #[test]
    fn test_is_last() {
        let p = Pagination::new(10).current(10);
        assert!(p.is_last());
    }

    #[test]
    fn test_is_last_false() {
        let p = Pagination::new(10).current(5);
        assert!(!p.is_last());
    }

    #[test]
    fn test_is_first_last() {
        let mut p = pagination(10);
        assert!(p.is_first());
        assert!(!p.is_last());

        p.last();
        assert!(!p.is_first());
        assert!(p.is_last());
    }

    #[test]
    fn test_set_total() {
        let mut p = pagination(10).current(8);
        p.set_total(5);
        assert_eq!(p.total, 5);
        assert_eq!(p.current, 5); // Clamped to new total
    }

    #[test]
    fn test_set_total_no_clamp_needed() {
        let mut p = pagination(10).current(5);
        p.set_total(20);
        assert_eq!(p.total, 20);
        assert_eq!(p.current, 5); // Unchanged
    }

    #[test]
    fn test_set_total_to_one() {
        let mut p = pagination(10).current(5);
        p.set_total(1);
        assert_eq!(p.total, 1);
        assert_eq!(p.current, 1); // Clamped to 1
    }

    // =========================================================================
    // Pagination::visible_range tests
    // =========================================================================

    #[test]
    fn test_pagination_visible_range() {
        let p = pagination(20).current(10);
        let (start, end) = p.visible_range();
        assert!(start <= p.current);
        assert!(end >= p.current);
    }

    #[test]
    fn test_pagination_visible_range_near_start() {
        let p = pagination(20).current(2);
        let (start, end) = p.visible_range();
        assert_eq!(start, 1);
    }

    #[test]
    fn test_pagination_visible_range_near_end() {
        let p = pagination(20).current(19);
        let (start, end) = p.visible_range();
        assert_eq!(end, 20);
    }

    #[test]
    fn test_pagination_visible_range_small_total() {
        let p = pagination(5).current(3);
        let (start, end) = p.visible_range();
        assert_eq!(start, 1);
        assert_eq!(end, 5);
    }

    // =========================================================================
    // Pagination render tests
    // =========================================================================

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
    fn test_pagination_render_simple() {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = pagination(10).current(5).simple();
        p.render(&mut ctx);

        let text: String = (0..30)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains('5'));
    }

    #[test]
    fn test_pagination_render_compact() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = pagination(10).current(5).compact();
        p.render(&mut ctx);

        let text: String = (0..20)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains('5'));
        assert!(text.contains('/'));
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

    // =========================================================================
    // Pagination Default tests
    // =========================================================================

    #[test]
    fn test_pagination_default() {
        let p = Pagination::default();
        assert_eq!(p.total, 1);
        assert_eq!(p.current, 1);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_helper_function() {
        let p = pagination(15);
        assert_eq!(p.total, 15);
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

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_pagination_single_page_no_nav() {
        let mut p = pagination(1);
        assert!(!p.next_page());
        assert!(!p.prev_page());
        assert!(p.is_first());
        assert!(p.is_last());
    }

    #[test]
    fn test_pagination_two_pages() {
        let mut p = pagination(2);
        assert!(p.next_page());
        assert_eq!(p.current, 2);
        assert!(p.is_last());
    }

    #[test]
    fn test_pagination_large_total() {
        let p = pagination(1000).current(500);
        assert_eq!(p.get_current(), 500);
        assert_eq!(p.get_total(), 1000);
    }
}
