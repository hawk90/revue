//! TransitionGroup for animating lists with automatic reordering tests

use revue::widget::{Animation, TransitionGroup};

#[test]
fn test_transition_group_new() {
    let group = TransitionGroup::new(vec!["a", "b", "c"]);
    assert_eq!(group.len(), 3);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_new_empty() {
    let group = TransitionGroup::new(std::iter::empty::<&str>());
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_from_vec() {
    let items = vec!["x", "y", "z"];
    let group = TransitionGroup::new(items);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_default() {
    let group = TransitionGroup::default();
    assert_eq!(group.len(), 0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_enter() {
    let group = TransitionGroup::new(vec!["a"]).enter(Animation::fade());
    let _ = group;
}

#[test]
fn test_transition_group_leave() {
    let group = TransitionGroup::new(vec!["a"]).leave(Animation::fade());
    let _ = group;
}

#[test]
fn test_transition_group_move_animation() {
    let group = TransitionGroup::new(vec!["a"]).move_animation(Animation::slide_left());
    let _ = group;
}

#[test]
fn test_transition_group_stagger() {
    let group = TransitionGroup::new(vec!["a", "b"]).stagger(100);
    let _ = group;
}

#[test]
fn test_transition_group_push() {
    let mut group = TransitionGroup::new(vec!["a"]);
    assert_eq!(group.len(), 1);
    group.push("b");
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_push_string() {
    let mut group = TransitionGroup::new(vec!["a"]);
    group.push("b".to_string());
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_valid() {
    let mut group = TransitionGroup::new(vec!["a", "b", "c"]);
    let removed = group.remove(1);
    assert_eq!(removed, Some("b".to_string()));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_invalid() {
    let mut group = TransitionGroup::new(vec!["a", "b"]);
    let removed = group.remove(5);
    assert_eq!(removed, None);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_len() {
    let group = TransitionGroup::new(vec!["a", "b", "c", "d"]);
    assert_eq!(group.len(), 4);
}

#[test]
fn test_transition_group_is_empty() {
    let mut group = TransitionGroup::new(vec!["a"]);
    assert!(!group.is_empty());
    group.remove(0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_items() {
    let items = vec!["x", "y", "z"];
    let group = TransitionGroup::new(items.clone());
    let retrieved = group.items();
    assert_eq!(retrieved.len(), 3);
    assert_eq!(retrieved[0], "x");
    assert_eq!(retrieved[1], "y");
    assert_eq!(retrieved[2], "z");
}

// =========================================================================
// Additional tests for edge cases and behavior
// =========================================================================

#[test]
fn test_transition_group_builder_chain() {
    let group = TransitionGroup::new(vec!["a", "b"])
        .enter(Animation::fade())
        .leave(Animation::fade())
        .move_animation(Animation::slide_left())
        .stagger(50);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_push_multiple() {
    let mut group = TransitionGroup::new(vec!["a"]);
    group.push("b");
    group.push("c");
    group.push("d");
    assert_eq!(group.len(), 4);
}

#[test]
fn test_transition_group_remove_first() {
    let mut group = TransitionGroup::new(vec!["a", "b", "c"]);
    let removed = group.remove(0);
    assert_eq!(removed, Some("a".to_string()));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_last() {
    let mut group = TransitionGroup::new(vec!["a", "b", "c"]);
    let removed = group.remove(2);
    assert_eq!(removed, Some("c".to_string()));
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_remove_all() {
    let mut group = TransitionGroup::new(vec!["a", "b", "c"]);
    group.remove(0);
    group.remove(0);
    group.remove(0);
    assert!(group.is_empty());
}

#[test]
fn test_transition_group_with_empty_string() {
    let group = TransitionGroup::new(vec!["", "b", ""]);
    assert_eq!(group.len(), 3);
}

#[test]
fn test_transition_group_with_long_strings() {
    let long_string = "a".repeat(1000);
    let group = TransitionGroup::new(vec![long_string.clone()]);
    let items = group.items();
    assert_eq!(items[0].len(), 1000);
}

#[test]
fn test_transition_group_with_unicode() {
    let group = TransitionGroup::new(vec!["🎉", "🔥", "✨"]);
    assert_eq!(group.len(), 3);
    assert_eq!(group.items()[0], "🎉");
}

#[test]
fn test_transition_group_stagger_zero() {
    let group = TransitionGroup::new(vec!["a", "b"]).stagger(0);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_stagger_large() {
    let group = TransitionGroup::new(vec!["a"]).stagger(10000);
    assert_eq!(group.len(), 1);
}

#[test]
fn test_transition_group_single_item() {
    let group = TransitionGroup::new(vec!["single"]);
    assert_eq!(group.len(), 1);
    assert!(!group.is_empty());
}

#[test]
fn test_transition_group_many_items() {
    let items: Vec<&str> = (0..100).map(|_i| "item").collect();
    let group = TransitionGroup::new(items);
    assert_eq!(group.len(), 100);
}

#[test]
fn test_transition_group_items_returns_slice() {
    let group = TransitionGroup::new(vec!["a", "b"]);
    let items = group.items();
    // Verify we get a slice that can be iterated
    let count = items.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_transition_group_new_from_iterator() {
    let group = TransitionGroup::new((0..5).map(|i| format!("item{}", i)));
    assert_eq!(group.len(), 5);
}
