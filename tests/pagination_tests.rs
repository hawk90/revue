//! Integration tests for Pagination widget

use revue::style::Color;
use revue::widget::{pagination, Pagination, PaginationStyle};

#[test]
fn test_pagination_new() {
    let p = Pagination::new(10);
    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_current() {
    let p = Pagination::new(10).current(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_pagination_style_full() {
    let _p = Pagination::new(10).style(PaginationStyle::Full);
}

#[test]
fn test_pagination_style_simple() {
    let _p = pagination(10).simple();
    // Simple style was set
}

#[test]
fn test_pagination_style_compact() {
    let _p = pagination(10).compact();
    // Compact style was set
}

#[test]
fn test_pagination_style_dots() {
    let _p = pagination(10).dots();
    // Dots style was set
}

#[test]
fn test_pagination_max_visible() {
    let _p = Pagination::new(10).max_visible(5);
}

#[test]
fn test_pagination_no_arrows() {
    let _p = pagination(10).no_arrows();
}

#[test]
fn test_pagination_no_edges() {
    let _p = pagination(10).no_edges();
}

#[test]
fn test_pagination_active_color() {
    let _p = Pagination::new(10).active_color(Color::CYAN);
}

#[test]
fn test_pagination_inactive_color() {
    let _p = Pagination::new(10).inactive_color(Color::BLUE);
}

#[test]
fn test_pagination_focused() {
    let _p = Pagination::new(10).focused();
}

#[test]
fn test_pagination_next_page() {
    let mut p = Pagination::new(10);
    assert!(p.next_page());
    assert_eq!(p.get_current(), 2);
}

#[test]
fn test_pagination_next_page_at_end() {
    let mut p = Pagination::new(10).current(10);
    assert!(!p.next_page());
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_pagination_prev_page() {
    let mut p = Pagination::new(10).current(5);
    assert!(p.prev_page());
    assert_eq!(p.get_current(), 4);
}

#[test]
fn test_pagination_prev_page_at_start() {
    let mut p = Pagination::new(10);
    assert!(!p.prev_page());
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_first() {
    let mut p = Pagination::new(10).current(5);
    p.first();
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_last() {
    let mut p = Pagination::new(10);
    p.last();
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_pagination_goto() {
    let mut p = Pagination::new(10);
    p.goto(7);
    assert_eq!(p.get_current(), 7);
}

#[test]
fn test_pagination_goto_clamps_below() {
    let mut p = Pagination::new(10).current(5);
    p.goto(0);
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_goto_clamps_above() {
    let mut p = Pagination::new(10).current(5);
    p.goto(20);
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_pagination_is_first() {
    let p = Pagination::new(10);
    assert!(p.is_first());

    let p = p.current(5);
    assert!(!p.is_first());
}

#[test]
fn test_pagination_is_last() {
    let mut p = Pagination::new(10);
    assert!(!p.is_last());

    p.last();
    assert!(p.is_last());
}

#[test]
fn test_pagination_set_total() {
    let mut p = pagination(10).current(8);
    p.set_total(5);
    assert_eq!(p.get_total(), 5);
    assert_eq!(p.get_current(), 5); // Clamped
}

#[test]
fn test_pagination_set_total_increase() {
    let mut p = pagination(5).current(3);
    p.set_total(10);
    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 3); // Unchanged
}

#[test]
fn test_pagination_single_page() {
    let p = Pagination::new(1);
    assert!(p.is_first());
    assert!(p.is_last());
}

#[test]
fn test_pagination_helper() {
    let p = pagination(15);
    assert_eq!(p.get_total(), 15);
}

#[test]
fn test_pagination_builder_pattern() {
    let p = pagination(10)
        .current(3)
        .simple()
        .max_visible(5)
        .active_color(Color::CYAN)
        .inactive_color(Color::BLUE);

    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 3);
}

#[test]
fn test_pagination_navigation_sequence() {
    let mut p = Pagination::new(10);

    // Go to middle
    p.goto(5);
    assert_eq!(p.get_current(), 5);

    // Go back
    p.prev_page();
    assert_eq!(p.get_current(), 4);

    // Go forward
    p.next_page();
    assert_eq!(p.get_current(), 5);

    // Jump to first
    p.first();
    assert_eq!(p.get_current(), 1);

    // Jump to last
    p.last();
    assert_eq!(p.get_current(), 10);
}
