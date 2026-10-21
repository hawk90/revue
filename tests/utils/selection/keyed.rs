//! `KeyedSelection`: the selection stays on its item when the list changes (#975)

use proptest::prelude::*;
use revue::utils::KeyedSelection;

fn select(items: &[&'static str], index: usize) -> KeyedSelection<&'static str> {
    let mut sel = KeyedSelection::new();
    sel.select(index, items.len(), |i| items[i]);
    sel
}

/// csv_viewer, datagrid, procmon: a sort moved the selection to the row at
/// the old position.
#[test]
fn a_reorder_keeps_the_item() {
    let mut sel = select(&["z", "y", "x"], 0);
    sel.sync(["x", "y", "z"]);
    assert_eq!((sel.key(), sel.index()), (Some(&"z"), Some(2)));
}

/// log_viewer `max_entries`, notification expiry, accordion `remove_section`:
/// removing an item above moved the selection to the next one.
#[test]
fn removing_an_item_above_keeps_the_item() {
    let mut sel = select(&["a", "b", "c"], 1);
    sel.sync(["b", "c"]);
    assert_eq!((sel.key(), sel.index()), (Some(&"b"), Some(0)));
}

#[test]
fn a_removed_item_hands_the_selection_to_the_one_now_in_its_place() {
    let mut sel = select(&["a", "b", "c"], 1);
    sel.sync(["a", "c"]);
    assert_eq!(sel.key(), Some(&"c"));
}

/// sidebar `collapse_all`, modal `buttons`, timeline: a position past the end.
#[test]
fn a_removed_last_item_hands_the_selection_to_the_new_last() {
    let mut sel = select(&["a", "b", "c"], 2);
    sel.sync(["a"]);
    assert_eq!((sel.key(), sel.index()), (Some(&"a"), Some(0)));
}

#[test]
fn an_empty_list_has_no_selection() {
    let mut sel = select(&["a"], 0);
    sel.sync([]);
    assert_eq!(sel.key(), None);
    sel.move_by(1, 0, |_| "x");
    assert_eq!(sel.index(), None);
}

/// Index keys give today's position behavior.
#[test]
fn index_keys_behave_as_a_position() {
    let mut sel = KeyedSelection::new();
    sel.select(1, 3, |i| i);
    sel.sync(0..3);
    assert_eq!(sel.index(), Some(1));
    sel.sync(0..1);
    assert_eq!(sel.index(), Some(0));
}

#[test]
fn moves() {
    let items = ["a", "b", "c"];
    let key = |i: usize| items[i];
    let mut sel = KeyedSelection::new();
    sel.move_by(1, 3, key);
    assert_eq!(
        sel.index(),
        Some(0),
        "a move with no selection starts at the top"
    );
    sel.move_by(5, 3, key);
    assert_eq!(sel.index(), Some(2));
    sel.move_by(-1, 3, key);
    assert_eq!(sel.key(), Some(&"b"));
    sel.next_wrapping(3, key);
    sel.next_wrapping(3, key);
    assert_eq!(sel.index(), Some(0));
    sel.prev_wrapping(3, key);
    assert_eq!(sel.index(), Some(2));
    sel.first(3, key);
    assert!(sel.is_selected(0));
    sel.last(3, key);
    assert!(sel.is_selected(2));

    let mut up = KeyedSelection::new();
    up.move_by(-1, 3, key);
    assert_eq!(
        up.index(),
        Some(2),
        "a move up with no selection starts at the bottom"
    );
}

#[test]
fn select_key_and_clear() {
    let mut sel = KeyedSelection::new();
    assert!(sel.select_key(&"b", ["a", "b"]));
    assert_eq!(sel.index(), Some(1));
    assert!(!sel.select_key(&"q", ["a", "b"]));
    assert_eq!(sel.key(), Some(&"b"), "a missing key leaves the selection");
    sel.clear();
    assert_eq!(sel.index(), None);
}

proptest! {
    /// Any change to a list of distinct keys: the selection is on its item
    /// if the item is still there, else on the item at its old position, or
    /// on the last item; and none in an empty list.
    #[test]
    fn prop_sync_follows_the_item(
        len in 1usize..12,
        pick in 0usize..12,
        keep in prop::collection::vec(any::<bool>(), 12),
        extra in 0u32..4,
        seed in any::<u64>(),
    ) {
        let before: Vec<u32> = (0..len as u32).collect();
        let mut sel = KeyedSelection::new();
        sel.select(pick, len, |i| before[i]);
        let (key, old) = (*sel.key().unwrap(), sel.index().unwrap());

        let mut after: Vec<u32> = before.iter().copied().filter(|k| keep[*k as usize]).collect();
        after.extend(100..100 + extra);
        // A deterministic shuffle
        let mut state = seed;
        for i in (1..after.len()).rev() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            after.swap(i, (state >> 33) as usize % (i + 1));
        }
        sel.sync(after.iter().copied());

        match after.iter().position(|k| *k == key) {
            Some(at) => prop_assert_eq!((sel.key(), sel.index()), (Some(&key), Some(at))),
            None if after.is_empty() => prop_assert_eq!(sel.index(), None),
            None => {
                let at = old.min(after.len() - 1);
                prop_assert_eq!((sel.key(), sel.index()), (Some(&after[at]), Some(at)));
            }
        }
    }
}
