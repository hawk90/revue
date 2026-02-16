//! Tests for Pagination widget
//!
//! Extracted from src/widget/pagination.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{pagination, Pagination, PaginationStyle};

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
    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 1);
    assert_eq!(p.get_style(), PaginationStyle::Full);
    assert_eq!(p.get_max_visible(), 7);
    assert!(p.get_show_arrows());
    assert!(p.get_show_edges());
    assert!(!p.get_focused());
}

#[test]
fn test_pagination_new_single_page() {
    let p = Pagination::new(1);
    assert_eq!(p.get_total(), 1);
    assert_eq!(p.get_current(), 1);
}

// =========================================================================
// Pagination builder tests
// =========================================================================

#[test]
fn test_pagination_current() {
    let p = Pagination::new(10).current(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_pagination_current_clamps_low() {
    let p = Pagination::new(10).current(0);
    assert_eq!(p.get_current(), 1); // Clamped to 1
}

#[test]
fn test_pagination_current_clamps_high() {
    let p = Pagination::new(10).current(15);
    assert_eq!(p.get_current(), 10); // Clamped to total
}

#[test]
fn test_pagination_style() {
    let p = Pagination::new(10).style(PaginationStyle::Compact);
    assert_eq!(p.get_style(), PaginationStyle::Compact);
}

#[test]
fn test_pagination_simple() {
    let p = Pagination::new(10).simple();
    assert_eq!(p.get_style(), PaginationStyle::Simple);
}

#[test]
fn test_pagination_compact() {
    let p = Pagination::new(10).compact();
    assert_eq!(p.get_style(), PaginationStyle::Compact);
}

#[test]
fn test_pagination_dots() {
    let p = Pagination::new(10).dots();
    assert_eq!(p.get_style(), PaginationStyle::Dots);
}

#[test]
fn test_pagination_max_visible() {
    let p = Pagination::new(10).max_visible(5);
    assert_eq!(p.get_max_visible(), 5);
}

#[test]
fn test_pagination_max_visible_clamps() {
    let p = Pagination::new(10).max_visible(2);
    assert_eq!(p.get_max_visible(), 3); // Clamped to minimum 3
}

#[test]
fn test_pagination_no_arrows() {
    let p = Pagination::new(10).no_arrows();
    assert!(!p.get_show_arrows());
}

#[test]
fn test_pagination_no_edges() {
    let p = Pagination::new(10).no_edges();
    assert!(!p.get_show_edges());
}

#[test]
fn test_pagination_active_color() {
    let p = Pagination::new(10).active_color(Color::RED);
    assert_eq!(p.get_active_color(), Color::RED);
}

#[test]
fn test_pagination_inactive_color() {
    let p = Pagination::new(10).inactive_color(Color::BLUE);
    assert_eq!(p.get_inactive_color(), Color::BLUE);
}

#[test]
fn test_pagination_focused() {
    let p = Pagination::new(10).focused();
    assert!(p.get_focused());
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

    assert_eq!(p.get_total(), 20);
    assert_eq!(p.get_current(), 5);
    assert_eq!(p.get_style(), PaginationStyle::Simple);
    assert_eq!(p.get_max_visible(), 5);
    assert!(!p.get_show_arrows());
    assert!(!p.get_show_edges());
    assert!(p.get_focused());
}

// =========================================================================
// Pagination navigation tests
// =========================================================================

#[test]
fn test_pagination_navigation() {
    let mut p = Pagination::new(10);

    assert!(p.next_page());
    assert_eq!(p.get_current(), 2);

    assert!(p.prev_page());
    assert_eq!(p.get_current(), 1);

    assert!(!p.prev_page()); // Can't go below 1

    p.last();
    assert_eq!(p.get_current(), 10);

    assert!(!p.next_page()); // Can't go above total

    p.first();
    assert_eq!(p.get_current(), 1);

    p.goto(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_next_page_at_end() {
    let mut p = Pagination::new(5).current(5);
    assert!(!p.next_page());
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_prev_page_at_start() {
    let mut p = Pagination::new(5);
    assert!(!p.prev_page());
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_first() {
    let mut p = Pagination::new(10).current(5);
    p.first();
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_last() {
    let mut p = Pagination::new(10).current(5);
    p.last();
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_goto_clamps_low() {
    let mut p = Pagination::new(10).current(5);
    p.goto(0);
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_goto_clamps_high() {
    let mut p = Pagination::new(10).current(5);
    p.goto(20);
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_goto_middle() {
    let mut p = Pagination::new(10);
    p.goto(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_goto_same_page() {
    let mut p = Pagination::new(10).current(5);
    p.goto(5);
    assert_eq!(p.get_current(), 5);
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
    assert_eq!(p.get_total(), 5);
    assert_eq!(p.get_current(), 5); // Clamped to new total
}

#[test]
fn test_set_total_no_clamp_needed() {
    let mut p = pagination(10).current(5);
    p.set_total(20);
    assert_eq!(p.get_total(), 20);
    assert_eq!(p.get_current(), 5); // Unchanged
}

#[test]
fn test_set_total_to_one() {
    let mut p = pagination(10).current(5);
    p.set_total(1);
    assert_eq!(p.get_total(), 1);
    assert_eq!(p.get_current(), 1); // Clamped to 1
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
    assert_eq!(p.get_total(), 1);
    assert_eq!(p.get_current(), 1);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_helper_function() {
    let p = pagination(15);
    assert_eq!(p.get_total(), 15);
}

#[test]
fn test_pagination_styles() {
    let p = pagination(10).simple();
    assert_eq!(p.get_style(), PaginationStyle::Simple);

    let p = pagination(10).compact();
    assert_eq!(p.get_style(), PaginationStyle::Compact);

    let p = pagination(10).dots();
    assert_eq!(p.get_style(), PaginationStyle::Dots);
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
    assert_eq!(p.get_current(), 2);
    assert!(p.is_last());
}

#[test]
fn test_pagination_large_total() {
    let p = pagination(1000).current(500);
    assert_eq!(p.get_current(), 500);
    assert_eq!(p.get_total(), 1000);
}
