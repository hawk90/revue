//! `Selection` and `KeyedSelection` against reference models (#1162)
//!
//! Random moves and list changes go to the real type and to a model; after
//! each one the selected item and the view must equal the model's.

use proptest::prelude::*;
use revue::utils::{KeyedSelection, Selection};

// =========================================================================
// Selection: an index and a window of `visible` rows that shows it
// =========================================================================

#[derive(Clone, Debug)]
enum SelOp {
    Next,
    Prev,
    Down,
    Up,
    First,
    Last,
    Set(usize),
    SetLen(usize),
    SetVisible(usize),
    PageUp,
    PageDown,
}

fn sel_op() -> impl Strategy<Value = SelOp> {
    prop_oneof![
        Just(SelOp::Next),
        Just(SelOp::Prev),
        Just(SelOp::Down),
        Just(SelOp::Up),
        Just(SelOp::First),
        Just(SelOp::Last),
        (0usize..30).prop_map(SelOp::Set),
        (0usize..25).prop_map(SelOp::SetLen),
        (0usize..8).prop_map(SelOp::SetVisible),
        Just(SelOp::PageUp),
        Just(SelOp::PageDown),
    ]
}

#[derive(Clone, Debug)]
struct SelModel {
    index: usize,
    len: usize,
    offset: usize,
    /// None until set: everything is visible
    visible: Option<usize>,
    /// An index set past the end, selected once the items reach it
    requested: Option<usize>,
}

impl SelModel {
    /// Of the windows that show the index, the one nearest the old one
    fn show_index(&mut self) {
        let Some(visible) = self.visible.filter(|&v| v > 0) else {
            return;
        };
        let max = self.len.saturating_sub(visible);
        self.offset = (0..=max)
            .filter(|&o| o <= self.index && self.index < o + visible)
            .min_by_key(|&o| o.abs_diff(self.offset))
            // Only an index past the end has no window: the last one
            .unwrap_or(max);
    }

    fn apply(&mut self, op: &SelOp) {
        if !matches!(op, SelOp::Set(_) | SelOp::SetLen(_) | SelOp::SetVisible(_)) {
            self.requested = None;
        }
        let len = self.len;
        let page = self.visible.unwrap_or(usize::MAX);
        match *op {
            SelOp::Next if len > 0 => self.index = (self.index + 1) % len,
            SelOp::Prev if len > 0 => self.index = (self.index + len - 1) % len,
            SelOp::Down if self.index + 1 < len => self.index += 1,
            SelOp::Up => self.index = self.index.saturating_sub(1),
            SelOp::First => {
                self.index = 0;
                self.offset = 0;
            }
            SelOp::Last if len > 0 => self.index = len - 1,
            SelOp::Set(i) => {
                self.index = i.min(len.saturating_sub(1));
                self.requested = (i >= len).then_some(i);
            }
            SelOp::SetLen(n) => {
                self.len = n;
                match self.requested {
                    Some(r) if r < n => {
                        self.index = r;
                        self.requested = None;
                    }
                    _ if n > 0 => self.index = self.index.min(n - 1),
                    _ => {}
                }
            }
            SelOp::SetVisible(v) => self.visible = Some(v),
            SelOp::PageDown if page > 0 && len > 0 => {
                self.index = self.index.saturating_add(page).min(len - 1)
            }
            SelOp::PageUp if page > 0 => self.index = self.index.saturating_sub(page),
            _ => {}
        }
        self.show_index();
    }
}

fn run_selection(sel: &mut Selection, op: &SelOp) {
    match *op {
        SelOp::Next => sel.next(),
        SelOp::Prev => sel.prev(),
        SelOp::Down => sel.down(),
        SelOp::Up => sel.up(),
        SelOp::First => sel.first(),
        SelOp::Last => sel.last(),
        SelOp::Set(i) => sel.set(i),
        SelOp::SetLen(n) => sel.set_len(n),
        SelOp::SetVisible(v) => sel.set_visible(v),
        SelOp::PageUp => sel.page_up(),
        SelOp::PageDown => sel.page_down(),
    }
}

/// Page Down before the view's height is known used `index + usize::MAX`
/// and overflowed past the first item
#[test]
fn page_down_before_the_view_is_drawn_goes_to_the_last_item() {
    let mut sel = Selection::new(10);
    sel.set(3);
    sel.page_down();
    assert_eq!(sel.index, 9);
}

/// With no rows shown the offset is kept, so items removed then left
/// `visible_range` going backwards (4..0)
#[test]
fn visible_range_stays_in_the_items_while_nothing_is_shown() {
    let mut sel = Selection::new(5);
    sel.set_visible(1);
    sel.last();
    sel.set_visible(0);
    sel.set_len(0);
    assert_eq!(sel.visible_range(), 0..0);
}

proptest! {
    /// Any moves and length changes: the index and the window equal the
    /// model's; the index is an item when there are any; and a drawn view
    /// shows the selected item.
    #[test]
    fn selection_follows_the_model(
        len in 0usize..20,
        ops in prop::collection::vec(sel_op(), 1..60),
    ) {
        let mut sel = Selection::new(len);
        let mut m = SelModel { index: 0, len, offset: 0, visible: None, requested: None };
        for op in &ops {
            run_selection(&mut sel, op);
            m.apply(op);
            prop_assert_eq!((sel.index, sel.len, sel.offset()), (m.index, m.len, m.offset), "after {:?}", op);
            let range = sel.visible_range();
            prop_assert!(range.start <= range.end && range.end <= sel.len, "{:?} of {}", range, sel.len);
            if sel.len > 0 {
                prop_assert!(sel.index < sel.len, "index {} of {}", sel.index, sel.len);
            }
            if let Some(visible) = m.visible.filter(|&v| v > 0 && sel.len > 0) {
                let range = sel.visible_range();
                prop_assert!(range.contains(&sel.index), "{} not in {:?}", sel.index, range);
                prop_assert_eq!(range.len(), visible.min(sel.len), "a short window: {:?}", range);
            }
        }
    }
}

// =========================================================================
// KeyedSelection: the selected item, by key, across list changes
// =========================================================================

#[derive(Clone, Debug)]
enum KeyOp {
    /// Insert a new key at a fraction (in sixteenths) of the list
    Insert(usize),
    /// Remove the key at a fraction (in sixteenths) of the list
    Remove(usize),
    Reverse,
    Sort,
    Rotate(usize),
    Empty,
    MoveBy(isize),
    Next,
    Prev,
    First,
    Last,
    Select(usize),
    /// Select a key, which may be gone
    SelectKey(u32),
    Clear,
}

fn key_op() -> impl Strategy<Value = KeyOp> {
    prop_oneof![
        3 => (0usize..=16).prop_map(KeyOp::Insert),
        3 => (0usize..16).prop_map(KeyOp::Remove),
        1 => Just(KeyOp::Reverse),
        1 => Just(KeyOp::Sort),
        1 => (1usize..5).prop_map(KeyOp::Rotate),
        1 => Just(KeyOp::Empty),
        2 => (-4isize..5).prop_map(KeyOp::MoveBy),
        1 => Just(KeyOp::Next),
        1 => Just(KeyOp::Prev),
        1 => Just(KeyOp::First),
        1 => Just(KeyOp::Last),
        1 => (0usize..20).prop_map(KeyOp::Select),
        1 => (0u32..30).prop_map(KeyOp::SelectKey),
        1 => Just(KeyOp::Clear),
    ]
}

/// The model holds the key and the position it was last seen at
#[derive(Clone, Debug, Default)]
struct KeyModel {
    keys: Vec<u32>,
    next_key: u32,
    selected: Option<(u32, usize)>,
}

impl KeyModel {
    fn select(&mut self, index: usize) {
        self.selected = (!self.keys.is_empty()).then(|| {
            let at = index.min(self.keys.len() - 1);
            (self.keys[at], at)
        });
    }

    /// The list changed: the same item if it is there, else the item now
    /// at its position, else the last item
    fn sync(&mut self) {
        self.selected =
            self.selected
                .and_then(|(key, at)| match self.keys.iter().position(|&k| k == key) {
                    Some(now) => Some((key, now)),
                    None => {
                        let now = at.min(self.keys.len().checked_sub(1)?);
                        Some((self.keys[now], now))
                    }
                });
    }

    /// Applies `op`; true when the list changed
    fn apply(&mut self, op: &KeyOp) -> bool {
        let len = self.keys.len();
        let index = self.selected.map(|(_, at)| at);
        match *op {
            KeyOp::Insert(at) => {
                self.keys.insert(at * len / 16, self.next_key);
                self.next_key += 1;
            }
            KeyOp::Remove(at) if len > 0 => {
                self.keys.remove(at * len / 16);
            }
            KeyOp::Reverse => self.keys.reverse(),
            KeyOp::Sort => self.keys.sort_unstable_by_key(|k| (k % 3, *k)),
            KeyOp::Rotate(n) if len > 0 => self.keys.rotate_left(n % len),
            KeyOp::Empty => self.keys.clear(),
            KeyOp::MoveBy(d) => match index {
                _ if len == 0 => self.selected = None,
                None => self.select(if d < 0 { len - 1 } else { 0 }),
                Some(i) => self.select(i.saturating_add_signed(d)),
            },
            KeyOp::Next => self.select(index.map_or(0, |i| (i + 1) % len.max(1))),
            KeyOp::Prev => self.select(match index {
                Some(i) if i > 0 => i - 1,
                _ => len.saturating_sub(1),
            }),
            KeyOp::First => self.select(0),
            KeyOp::Last => self.select(len.saturating_sub(1)),
            KeyOp::Select(i) => self.select(i),
            KeyOp::SelectKey(k) => {
                if let Some(at) = self.keys.iter().position(|&x| x == k) {
                    self.selected = Some((k, at));
                }
            }
            KeyOp::Clear => self.selected = None,
            KeyOp::Remove(_) | KeyOp::Rotate(_) => {}
        }
        let changed = matches!(
            op,
            KeyOp::Insert(_)
                | KeyOp::Remove(_)
                | KeyOp::Reverse
                | KeyOp::Sort
                | KeyOp::Rotate(_)
                | KeyOp::Empty
        );
        if changed {
            self.sync();
        }
        changed
    }
}

proptest! {
    /// Any moves and list changes, each change followed by `sync`: the
    /// selection equals the model's; it stays on its item while the item is
    /// in the list; and its index is where its key is.
    #[test]
    fn keyed_selection_follows_the_model(
        len in 0u32..10,
        ops in prop::collection::vec(key_op(), 1..60),
    ) {
        let mut m = KeyModel { keys: (0..len).collect(), next_key: len, selected: None };
        let mut sel = KeyedSelection::new();
        for op in &ops {
            let before = m.selected;
            let changed = m.apply(op);
            let keys = m.keys.clone();
            let (n, key_at) = (keys.len(), |i: usize| keys[i]);
            match *op {
                _ if changed => sel.sync(keys.iter().copied()),
                KeyOp::MoveBy(d) => sel.move_by(d, n, key_at),
                KeyOp::Next => sel.next_wrapping(n, key_at),
                KeyOp::Prev => sel.prev_wrapping(n, key_at),
                KeyOp::First => sel.first(n, key_at),
                KeyOp::Last => sel.last(n, key_at),
                KeyOp::Select(i) => sel.select(i, n, key_at),
                KeyOp::SelectKey(k) => {
                    let found = sel.select_key(&k, keys.iter().copied());
                    prop_assert_eq!(found, keys.contains(&k));
                }
                KeyOp::Clear => sel.clear(),
                _ => {}
            }
            prop_assert_eq!(
                (sel.key().copied(), sel.index()),
                (m.selected.map(|(k, _)| k), m.selected.map(|(_, at)| at)),
                "after {:?} on {:?}", op, keys
            );
            if let Some(key) = sel.key() {
                prop_assert_eq!(keys.get(sel.index().unwrap()), Some(key), "index off its key");
            }
            if let (true, Some((key, _))) = (changed, before) {
                if keys.contains(&key) {
                    prop_assert_eq!(sel.key(), Some(&key), "left its item after {:?}", op);
                }
            }
            prop_assert_eq!(sel.key().is_some(), m.selected.is_some() && !keys.is_empty());
        }
    }
}
