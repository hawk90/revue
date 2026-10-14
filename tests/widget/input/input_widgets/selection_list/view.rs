//! SelectionList placement, and a model of the selection under keys

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::{SelectionItem, SelectionList};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn items() -> Vec<SelectionItem> {
    vec![
        SelectionItem::new("Apple").description("red"),
        SelectionItem::new("Banana"),
        SelectionItem::new("Cherry").disabled(true),
        SelectionItem::new("한글"),
        SelectionItem::new("Elder"),
    ]
}

#[test]
fn test_selection_list_zero_areas_draw_nothing() {
    let list = SelectionList::new(items())
        .title("T")
        .show_count(true)
        .show_descriptions(true)
        .selected(vec![0])
        .focused(true);
    assert_zero_areas_draw_nothing(&list);
}

#[test]
fn test_selection_list_same_when_offset() {
    let list = SelectionList::new(items())
        .title("T")
        .show_count(true)
        .max_visible(3)
        .selected(vec![1])
        .focused(true);
    assert_same_when_offset(list, 50, 8);
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Char('k')),
        Just(Key::Char('j')),
        Just(Key::Char(' ')),
        Just(Key::Char('a')),
        Just(Key::Char('n')),
        Just(Key::Char('x')),
        Just(Key::Enter),
    ]
}

proptest! {
    /// Any keys leave the selection a plain model gives: Space toggles the
    /// highlighted item unless it is disabled, the selection would go below
    /// the minimum or above the maximum; `a` selects every enabled item up
    /// to the maximum; `n` keeps the first `min` selected. The selection
    /// stays sorted, unique, enabled and within the bounds.
    #[test]
    fn prop_selection_list_keys_follow_the_model(
        min in 0usize..3,
        max in 0usize..4,
        keys in prop::collection::vec(key(), 0..40),
    ) {
        let disabled = [false, false, true, false, false];
        let mut list = SelectionList::new(items()).min_selections(min).max_selections(max);
        let (mut sel, mut hl): (Vec<usize>, usize) = (Vec::new(), 0);
        for k in &keys {
            let used = match k {
                Key::Up | Key::Char('k') => { hl = hl.saturating_sub(1); true }
                Key::Down | Key::Char('j') => { hl = (hl + 1).min(4); true }
                Key::Char(' ') => {
                    if !disabled[hl] {
                        if let Some(p) = sel.iter().position(|&i| i == hl) {
                            if sel.len() > min { sel.remove(p); }
                        } else if max == 0 || sel.len() < max {
                            sel.push(hl);
                            sel.sort();
                        }
                    }
                    true
                }
                Key::Char('a') => {
                    sel = (0..5).filter(|&i| !disabled[i]).collect();
                    if max > 0 { sel.truncate(max); }
                    true
                }
                Key::Char('n') => {
                    if min == 0 { sel.clear() } else { sel.truncate(min) }
                    true
                }
                _ => false,
            };
            prop_assert_eq!(list.handle_key(k), used, "{:?}", k);
            prop_assert_eq!(list.get_selected(), sel.as_slice(), "after {:?}", k);
            prop_assert!(sel.iter().all(|&i| !disabled[i]));
            prop_assert!(max == 0 || sel.len() <= max);
        }
    }
}

#[test]
fn test_selection_list_count_shows_the_maximum() {
    let list = SelectionList::new(items())
        .selected(vec![0])
        .show_count(true);
    assert_eq!(super::rows(&list)[0], "Selected: 1");
    assert_eq!(super::rows(&list.max_selections(3))[0], "Selected: 1/3");
}
