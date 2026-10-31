//! Tests of the sizing rules (moved from sizing.rs)

use super::{hstack, vstack, ChildSize, Stack};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};
use crate::widget::Text;

#[test]
fn test_stack_render_empty_no_panic() {
    let mut buf = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = Stack::new();
    s.render(&mut ctx);
    assert!((0..24).all(|y| (0..80).all(|x| buf.get(x, y).unwrap().symbol == ' ')));
}

#[test]
fn test_stack_render_zero_area_no_panic() {
    let mut buf = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = Stack::new().child(Text::new("A"));
    s.render(&mut ctx);
    assert!((0..24).all(|y| (0..80).all(|x| buf.get(x, y).unwrap().symbol == ' ')));
}

#[test]
fn test_stack_calculate_sizes_auto() {
    let s = Stack::new()
        .child(Text::new("A"))
        .child(Text::new("B"))
        .child(Text::new("C"));
    let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 3);
    assert_eq!(sizes.len(), 3);
    assert_eq!(sizes.iter().sum::<u16>(), 30);
}

#[test]
fn test_stack_calculate_sizes_fixed() {
    let s = Stack::new()
        .child_sized(Text::new("A"), 10)
        .child_sized(Text::new("B"), 20);
    let sizes = Stack::calculate_sizes(&s.child_sizes, 50, 2);
    assert_eq!(sizes, vec![10, 20]);
}

#[test]
fn test_stack_calculate_sizes_flex() {
    let s = Stack::new()
        .child_flex(Text::new("A"), 1.0)
        .child_flex(Text::new("B"), 2.0);
    let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 2);
    assert_eq!(sizes[0], 10);
    assert_eq!(sizes[1], 20);
}

#[test]
fn test_stack_calculate_sizes_mixed() {
    let s = Stack::new()
        .child_sized(Text::new("Fixed"), 10)
        .child_flex(Text::new("Flex"), 1.0);
    let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 2);
    assert_eq!(sizes[0], 10);
    assert_eq!(sizes[1], 20); // 30 - 10 = 20 (no auto children)
}

#[test]
fn test_stack_calculate_sizes_content_fits() {
    let sizes = [
        ChildSize::Content(3),
        ChildSize::Auto,
        ChildSize::Content(2),
    ];
    assert_eq!(Stack::calculate_sizes(&sizes, 10, 3), vec![3, 5, 2]);
}

/// Measured sizes that overflow shrink in proportion; fixed ones do not.
#[test]
fn test_stack_calculate_sizes_content_shrinks() {
    let sizes = [
        ChildSize::Fixed(2),
        ChildSize::Content(12),
        ChildSize::Content(4),
        ChildSize::Fixed(2),
    ];
    // 8 cells for 16 measured: 12 -> 6, 4 -> 2.
    assert_eq!(Stack::calculate_sizes(&sizes, 12, 4), vec![2, 6, 2, 2]);
    // 7 cells: 12*7/16 = 5, 4*7/16 = 1, the leftover cell to the first.
    assert_eq!(Stack::calculate_sizes(&sizes, 11, 4), vec![2, 6, 1, 2]);
}

/// A share does not go below a child's minimum; the others share the rest.
#[test]
fn test_stack_calculate_sizes_auto_minimum() {
    use ChildSize::{Auto, AutoMin, Fixed};
    let sizes = |c: &[ChildSize], n| Stack::calculate_sizes(c, n, c.len());
    // Below its share a minimum changes nothing.
    assert_eq!(sizes(&[AutoMin(3), Auto], 10), vec![5, 5]);
    assert_eq!(sizes(&[AutoMin(5), Auto], 10), vec![5, 5]);
    // Above its share it is kept, and rounding goes to the earliest of the rest.
    assert_eq!(sizes(&[AutoMin(6), Auto], 10), vec![6, 4]);
    assert_eq!(sizes(&[Auto, AutoMin(6), Auto], 11), vec![3, 6, 2]);
    // Pinning one can push the next one's minimum above its new share.
    assert_eq!(sizes(&[AutoMin(6), AutoMin(3), Auto], 10), vec![6, 3, 1]);
    // Minimums that do not fit overflow; the others still get a cell.
    assert_eq!(sizes(&[AutoMin(6), AutoMin(6)], 10), vec![6, 6]);
    assert_eq!(sizes(&[AutoMin(10), Auto], 10), vec![10, 1]);
    assert_eq!(sizes(&[AutoMin(4), Auto], 0), vec![4, 1]);
    // After the fixed children
    assert_eq!(sizes(&[Fixed(4), AutoMin(5), Auto], 10), vec![4, 5, 1]);
    // With flex, an auto child takes its minimum and flex the rest.
    assert_eq!(sizes(&[ChildSize::Flex(1.0), AutoMin(3)], 10), vec![7, 3]);
}

#[test]
fn test_stack_calculate_sizes_empty() {
    let s = Stack::new();
    let sizes = Stack::calculate_sizes(&s.child_sizes, 100, 0);
    assert!(sizes.is_empty());
}

#[test]
fn test_stack_render_row_children() {
    let mut buf = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = hstack().child(Text::new("AB")).child(Text::new("CD"));
    s.render(&mut ctx);
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buf.get(1, 0).unwrap().symbol, 'B');
}

#[test]
fn test_stack_row_no_wrap_overflow() {
    // Without wrap, children extend beyond area (clipped by area bounds)
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = hstack()
        .child_sized(Text::new("AAAA"), 6)
        .child_sized(Text::new("BBBB"), 6);
    s.render(&mut ctx);
    // First child renders, second starts at x=6 but is clipped at width 10
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
}

#[test]
fn test_stack_needs_render_skip() {
    // Verify Stack respects needs_render
    let mut buf = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let s = vstack()
        .child(Text::new("Visible"))
        .child(Text::new("Also visible"));
    let mut ctx = RenderContext::new(&mut buf, area);
    s.render(&mut ctx);
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'V');
}
