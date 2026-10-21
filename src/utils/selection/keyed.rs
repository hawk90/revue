//! A selection that stays on its item when the list changes (#975)

/// The selected item of a list, held by its key
///
/// [`Selection`](super::Selection) keeps a position, so sorting, filtering or
/// trimming the list moves it onto another item. `KeyedSelection` keeps the
/// item's key, and [`sync`](Self::sync) finds the item again after the list
/// changes. When the item is gone, the selection moves to the item now at its
/// last position (the last item if the list got shorter). An empty list has
/// no selection.
///
/// A list whose items have no identity can use the index as the key, which
/// gives position behavior, as Vue's `v-for` without `:key` does. A widget
/// that creates its items (a log's entries) can give them a sequence number.
///
/// The moves take the list's length and a function from an index to that
/// item's key, so the widget does not build a key list for each key press.
/// Where the view is scrolled to is a separate
/// [`ScrollState`](crate::utils::ScrollState).
///
/// # Example
///
/// ```
/// use revue::utils::KeyedSelection;
///
/// let mut files = vec!["b.rs", "c.rs", "a.rs"];
/// let mut sel = KeyedSelection::new();
/// sel.select(1, files.len(), |i| files[i]); // "c.rs"
///
/// files.sort();
/// sel.sync(files.iter().copied());
/// assert_eq!(sel.index(), Some(2));
/// assert_eq!(sel.key(), Some(&"c.rs"));
///
/// files.retain(|f| *f != "c.rs");
/// sel.sync(files.iter().copied());
/// assert_eq!(sel.key(), Some(&"b.rs")); // the nearest item
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyedSelection<K> {
    selected: Option<(K, usize)>,
}

impl<K> Default for KeyedSelection<K> {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl<K: Clone + PartialEq> KeyedSelection<K> {
    /// No selection
    pub fn new() -> Self {
        Self::default()
    }

    /// The selected item's key
    pub fn key(&self) -> Option<&K> {
        self.selected.as_ref().map(|(key, _)| key)
    }

    /// The selected item's position in the list as of the last change
    pub fn index(&self) -> Option<usize> {
        self.selected.as_ref().map(|&(_, index)| index)
    }

    /// Whether the item at `index` is the selected one
    pub fn is_selected(&self, index: usize) -> bool {
        self.index() == Some(index)
    }

    /// Select the item at `index`, clamped to the list. An empty list clears
    /// the selection.
    pub fn select(&mut self, index: usize, len: usize, key_at: impl Fn(usize) -> K) {
        self.selected = (len > 0).then(|| {
            let index = index.min(len - 1);
            (key_at(index), index)
        });
    }

    /// Select the item with `key`, if the list has it. Returns whether it did.
    pub fn select_key(&mut self, key: &K, keys: impl IntoIterator<Item = K>) -> bool {
        match keys.into_iter().position(|k| k == *key) {
            Some(index) => {
                self.selected = Some((key.clone(), index));
                true
            }
            None => false,
        }
    }

    /// Clear the selection
    pub fn clear(&mut self) {
        self.selected = None;
    }

    /// Find the selected item again after the list changed. When its key is
    /// gone, select the item now at its last position, or the last item.
    pub fn sync(&mut self, keys: impl IntoIterator<Item = K>) {
        let Some((key, old)) = self.selected.take() else {
            return;
        };
        let mut at_old = None;
        let mut last = None;
        for (index, k) in keys.into_iter().enumerate() {
            if k == key {
                self.selected = Some((key, index));
                return;
            }
            if index == old {
                at_old = Some((k.clone(), index));
            }
            last = Some((k, index));
        }
        self.selected = at_old.or(last);
    }

    /// Move by `delta` items (negative is up), stopping at either end. With
    /// no selection, a move selects the first item (down) or the last (up).
    pub fn move_by(&mut self, delta: isize, len: usize, key_at: impl Fn(usize) -> K) {
        if len == 0 {
            self.selected = None;
            return;
        }
        let target = match self.index() {
            None if delta < 0 => len - 1,
            None => 0,
            Some(index) if delta < 0 => index.saturating_sub(delta.unsigned_abs()),
            Some(index) => index.saturating_add(delta as usize),
        };
        self.select(target, len, key_at);
    }

    /// Move to the next item, from the last to the first
    pub fn next_wrapping(&mut self, len: usize, key_at: impl Fn(usize) -> K) {
        let target = match self.index() {
            Some(index) if index + 1 < len => index + 1,
            _ => 0,
        };
        self.select(target, len, key_at);
    }

    /// Move to the previous item, from the first to the last
    pub fn prev_wrapping(&mut self, len: usize, key_at: impl Fn(usize) -> K) {
        let target = match self.index() {
            Some(index) if index > 0 && index <= len => index - 1,
            _ => len.saturating_sub(1),
        };
        self.select(target, len, key_at);
    }

    /// Select the first item
    pub fn first(&mut self, len: usize, key_at: impl Fn(usize) -> K) {
        self.select(0, len, key_at);
    }

    /// Select the last item
    pub fn last(&mut self, len: usize, key_at: impl Fn(usize) -> K) {
        self.select(len.saturating_sub(1), len, key_at);
    }
}
