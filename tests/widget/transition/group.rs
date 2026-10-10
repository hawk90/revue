//! TransitionGroup tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::transition_group;
use revue::widget::Animation;
use revue::widget::TransitionGroup;
use revue::widget::View;

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

// ============================================================================
// TransitionGroup Widget Tests
// ============================================================================

#[test]
fn test_transition_group_new_vec() {
    let items = vec!["a", "b", "c"];
    let group = TransitionGroup::new(items);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_new_array() {
    let group = TransitionGroup::new(["x", "y", "z"]);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_new_empty() {
    let group = TransitionGroup::new(Vec::<String>::new());
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_default() {
    let group = TransitionGroup::default();
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
#[allow(deprecated)] // the animation builders are deprecated no-ops (#799)
fn test_transition_group_enter() {
    let group = TransitionGroup::new(["a", "b"]).enter(Animation::fade());
    assert_eq!(group.len(), 2);
}

#[test]
#[allow(deprecated)] // the animation builders are deprecated no-ops (#799)
fn test_transition_group_leave() {
    let group = TransitionGroup::new(["a", "b"]).leave(Animation::fade());
    assert_eq!(group.len(), 2);
}

#[test]
#[allow(deprecated)] // the animation builders are deprecated no-ops (#799)
fn test_transition_group_move_animation() {
    let group = TransitionGroup::new(["a", "b"]).move_animation(Animation::slide_left());
    assert_eq!(group.len(), 2);
}

#[test]
#[allow(deprecated)] // the animation builders are deprecated no-ops (#799)
fn test_transition_group_stagger() {
    let group = TransitionGroup::new(["a", "b", "c"]).stagger(100);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_push() {
    let mut group = TransitionGroup::new(["a"]);
    group.push("b");
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_push_multiple() {
    let mut group = TransitionGroup::new(["a"]);
    group.push("b");
    group.push("c");
    group.push("d");
    assert_eq!(group.len(), 4);
}

#[test]
fn test_transition_group_push_string() {
    let mut group = TransitionGroup::new(["a"]);
    group.push("b".to_string());
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_first() {
    let mut group = TransitionGroup::new(["a", "b", "c"]);
    let removed = group.remove(0);
    assert_eq!(removed, Some(String::from("a")));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_middle() {
    let mut group = TransitionGroup::new(["a", "b", "c"]);
    let removed = group.remove(1);
    assert_eq!(removed, Some(String::from("b")));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_last() {
    let mut group = TransitionGroup::new(["a", "b", "c"]);
    let removed = group.remove(2);
    assert_eq!(removed, Some(String::from("c")));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_invalid() {
    let mut group = TransitionGroup::new(["a", "b"]);
    let removed = group.remove(5);
    assert_eq!(removed, None);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_from_empty() {
    let mut group = TransitionGroup::new(Vec::<String>::new());
    let removed = group.remove(0);
    assert_eq!(removed, None);
    assert_eq!(group.len(), 0);
}

#[test]
fn test_transition_group_len() {
    let group = TransitionGroup::new(["a", "b", "c", "d", "e"]);
    assert_eq!(group.len(), 5);
}

#[test]
fn test_transition_group_len_empty() {
    let group = TransitionGroup::new(Vec::<String>::new());
    assert_eq!(group.len(), 0);
}

#[test]
fn test_transition_group_is_empty_true() {
    let group = TransitionGroup::new(Vec::<String>::new());
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_is_empty_false() {
    let group = TransitionGroup::new(["a"]);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_items() {
    let group = TransitionGroup::new(["a", "b", "c"]);
    let items = group.items();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0], "a");
    assert_eq!(items[1], "b");
    assert_eq!(items[2], "c");
}

#[test]
fn test_transition_group_items_empty() {
    let group = TransitionGroup::new(Vec::<String>::new());
    let items = group.items();
    assert_eq!(items.len(), 0);
}

#[test]
#[allow(deprecated)] // the animation builders are deprecated no-ops (#799)
fn test_transition_group_builder_chain() {
    let group = TransitionGroup::new(["a", "b", "c"])
        .enter(Animation::fade())
        .leave(Animation::fade())
        .move_animation(Animation::slide_left())
        .stagger(50);

    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_single_item() {
    let group = TransitionGroup::new(["single"]);
    assert_eq!(group.len(), 1);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_remove_then_push() {
    let mut group = TransitionGroup::new(["a", "b", "c"]);
    group.remove(1);
    assert_eq!(group.len(), 2);

    group.push("d");
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_push_after_empty() {
    let mut group = TransitionGroup::new(Vec::<String>::new());
    assert!(group.is_empty());

    group.push("first");
    assert!(!group.is_empty());
    assert_eq!(group.len(), 1);
}

#[test]
fn test_transition_group_many_items() {
    let items: Vec<String> = (0..100).map(|i| format!("item{}", i)).collect();
    let group = TransitionGroup::new(items);
    assert_eq!(group.len(), 100);
}

#[test]
fn test_transition_group_with_unicode() {
    let group = TransitionGroup::new(["Hello", "世界", "🌍"]);
    assert_eq!(group.len(), 3);

    let items = group.items();
    assert_eq!(items[1], "世界");
    assert_eq!(items[2], "🌍");
}

#[test]
fn test_transition_group_helper_function() {
    let group = transition_group(["a", "b", "c"]);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_items_reference() {
    let group = TransitionGroup::new(["a", "b"]);
    let items = group.items();

    // items() returns a reference, so we can read from it
    assert_eq!(items[0], "a");
    assert_eq!(items[1], "b");
}
