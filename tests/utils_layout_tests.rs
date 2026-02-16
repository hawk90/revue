//! Tests for layout utilities
//!
//! Extracted from src/utils/layout.rs

use revue::utils::layout::BoxLayout;

#[test]
fn test_basic_layout() {
    let bx = BoxLayout::new(0, 0, 80, 24);

    assert_eq!(bx.content_x(), 1);
    assert_eq!(bx.content_y(), 1);
    assert_eq!(bx.content_width(), 78);
    assert_eq!(bx.content_height(), 22);
    assert_eq!(bx.row_y(0), 1);
    assert_eq!(bx.row_y(5), 6);
    assert_eq!(bx.bottom_y(), 23);
    assert_eq!(bx.footer_y(), 24);
}

#[test]
fn test_fill() {
    let bx = BoxLayout::fill(0, 0, 80, 24, 1);
    assert_eq!(bx.height, 23);
    assert_eq!(bx.footer_y(), 23);
}

#[test]
fn test_fit() {
    let bx = BoxLayout::fit(0, 0, 40, 20, 5, 1);
    // border(1) + header(1) + content(5) + border(1) = 8
    assert_eq!(bx.height, 8);
}

#[test]
fn test_centered() {
    let bx = BoxLayout::centered(80, 24, 40, 10);
    assert_eq!(bx.x, 20);
    assert_eq!(bx.y, 7);
}

#[test]
fn test_visible_rows() {
    let bx = BoxLayout::new(0, 0, 80, 24);
    // content_height = 22, header = 1, extra = 0 -> 21 rows
    assert_eq!(bx.visible_rows(1, 0), 21);
}

#[test]
fn test_with_padding() {
    let bx = BoxLayout::new(0, 0, 80, 24).with_padding(1);
    assert_eq!(bx.content_x(), 2); // border + padding
    assert_eq!(bx.content_width(), 76); // -4 for border+padding on both sides
}
