//! TransitionGroup tests
//!
//! Construction, push/remove/len/items are covered by
//! tests/transition_group_tests.rs; these cover the remaining list
//! behavior and rendering.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{TransitionGroup, View};

fn render(group: &TransitionGroup, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    group.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

#[test]
fn test_transition_group_new_from_iterator() {
    let group = TransitionGroup::new((0..3).map(|i| format!("item{}", i)));
    assert_eq!(group.items(), &["item0", "item1", "item2"]);
}

#[test]
fn test_transition_group_push_appends_in_order() {
    let mut group = TransitionGroup::new(["a"]);
    group.push("b");
    group.push(String::from("c"));
    assert_eq!(group.items(), &["a", "b", "c"]);
}

#[test]
fn test_transition_group_remove_keeps_remaining_order() {
    let mut group = TransitionGroup::new(["a", "b", "c", "d"]);
    assert_eq!(group.remove(1), Some("b".to_string()));
    assert_eq!(group.items(), &["a", "c", "d"]);
}

#[test]
fn test_transition_group_remove_all() {
    let mut group = TransitionGroup::new(["a", "b", "c"]);
    group.remove(0);
    group.remove(0);
    group.remove(0);
    assert!(group.is_empty());
    assert_eq!(group.remove(0), None);
}

#[test]
fn test_transition_group_keeps_empty_strings() {
    let group = TransitionGroup::new(["", "b", ""]);
    assert_eq!(group.len(), 3);
    assert_eq!(group.items(), &["", "b", ""]);
}

#[test]
fn test_render_one_item_per_row() {
    let group = TransitionGroup::new(["one", "two", "three"]);
    let buffer = render(&group, 6, 4);
    assert_eq!(row(&buffer, 0, 6), "one   ");
    assert_eq!(row(&buffer, 1, 6), "two   ");
    assert_eq!(row(&buffer, 2, 6), "three ");
    assert_eq!(row(&buffer, 3, 6), "      ");

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert_eq!(cell.bg, Some(Color::BLACK));
}

#[test]
fn test_render_stops_at_area_height() {
    let group = TransitionGroup::new(["a", "b", "c"]);
    let buffer = render(&group, 3, 2);
    assert_eq!(row(&buffer, 0, 3), "a  ");
    assert_eq!(row(&buffer, 1, 3), "b  ");
}

#[test]
fn test_render_clips_items_to_width() {
    let group = TransitionGroup::new(["abcdef"]);
    let buffer = render(&group, 4, 1);
    assert_eq!(row(&buffer, 0, 4), "abcd");
}

#[test]
fn test_render_wide_chars() {
    let group = TransitionGroup::new(["한글", "x한"]);
    let buffer = render(&group, 3, 2);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    // '글' needs columns 2-3 but only column 2 exists: it is dropped
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, 'x');
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '한');
}

#[test]
fn test_render_reflects_push_and_remove() {
    let mut group = TransitionGroup::new(["a", "b"]);
    group.remove(0);
    group.push("c");
    let buffer = render(&group, 2, 3);
    assert_eq!(row(&buffer, 0, 2), "b ");
    assert_eq!(row(&buffer, 1, 2), "c ");
    assert_eq!(row(&buffer, 2, 2), "  ");
}

#[test]
fn test_render_empty_group_draws_nothing() {
    let group = TransitionGroup::default();
    let buffer = render(&group, 3, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_none());
}
