//! Integration tests for TransitionGroup widget public API

use revue::widget::{transition_group, Animation, TransitionGroup};

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
fn test_transition_group_enter() {
    let group = TransitionGroup::new(["a", "b"]).enter(Animation::fade());
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_leave() {
    let group = TransitionGroup::new(["a", "b"]).leave(Animation::fade());
    assert_eq!(group.len(), 2);
}

#[test]
fn test_transition_group_move_animation() {
    let group = TransitionGroup::new(["a", "b"]).move_animation(Animation::slide_left());
    assert_eq!(group.len(), 2);
}

#[test]
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
