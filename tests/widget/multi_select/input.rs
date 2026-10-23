//! MultiSelect keys: Enter and Escape in both states, the dropdown keys
//! while open, the tag keys while closed, the Interactive Tab, blur and
//! focus, and a model of the state under any keys

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::utils::fuzzy_match;
use revue::widget::traits::{EventResult, Interactive};
use revue::widget::{multi_select, MultiSelect, MultiSelectOption};

fn fruits() -> MultiSelect {
    multi_select().options(vec!["Apple", "Banana", "Cherry"])
}

fn press(v: &mut MultiSelect, keys: &[Key]) -> Vec<bool> {
    keys.iter().map(|k| v.handle_key(k)).collect()
}

#[test]
fn enter_opens_then_toggles_the_option_under_the_cursor() {
    let mut v = fruits();
    assert_eq!(press(&mut v, &[Key::Enter]), [true]);
    assert!(v.is_open());
    assert!(v.get_selected_indices().is_empty());
    press(&mut v, &[Key::Down, Key::Enter]);
    assert_eq!(v.get_selected_indices(), &[1]);
    assert!(v.is_open(), "Enter keeps the dropdown open");
    press(&mut v, &[Key::Enter]);
    assert!(v.get_selected_indices().is_empty());
}

#[test]
fn enter_skips_disabled_options_and_stops_at_the_maximum() {
    let mut v = multi_select()
        .option_detailed(MultiSelectOption::new("A", "a").disabled(true))
        .option("B")
        .option("C")
        .max_selections(1);
    assert_eq!(press(&mut v, &[Key::Enter, Key::Enter]), [true, true]);
    assert!(v.get_selected_indices().is_empty());
    press(&mut v, &[Key::Down, Key::Enter, Key::Down, Key::Enter]);
    assert_eq!(v.get_selected_indices(), &[1]);
}

#[test]
fn space_toggles_only_without_search() {
    let mut v = fruits().searchable(false);
    assert_eq!(press(&mut v, &[Key::Char(' ')]), [false], "closed");
    press(&mut v, &[Key::Enter, Key::Char(' ')]);
    assert_eq!(v.get_selected_indices(), &[0]);
    // Searchable: the space is typed into the query
    let mut v = fruits();
    press(&mut v, &[Key::Enter, Key::Char(' ')]);
    assert!(v.get_selected_indices().is_empty());
    assert_eq!(v.query(), " ");
}

#[test]
fn escape_closes_then_drops_the_tag_cursor() {
    let mut v = fruits();
    v.select_option(0);
    press(&mut v, &[Key::Enter, Key::Char('a')]);
    assert_eq!(press(&mut v, &[Key::Escape]), [true]);
    assert!(!v.is_open());
    assert_eq!(v.query(), "");
    assert_eq!(v.get_filtered(), &[0, 1, 2]);
    press(&mut v, &[Key::Left]);
    assert_eq!(v.get_tag_cursor(), Some(0));
    assert_eq!(press(&mut v, &[Key::Escape, Key::Escape]), [true, false]);
    assert_eq!(v.get_tag_cursor(), None);
}

#[test]
fn arrows_move_the_cursor_and_wrap() {
    let mut v = fruits();
    v.open();
    press(&mut v, &[Key::Up]);
    assert_eq!(v.current_option(), Some(2));
    press(&mut v, &[Key::Down]);
    assert_eq!(v.current_option(), Some(0));
    // j and k too, when the widget does not search (#889)
    let mut v = fruits().searchable(false);
    v.open();
    press(&mut v, &[Key::Char('j'), Key::Char('j')]);
    assert_eq!(v.current_option(), Some(2));
    press(&mut v, &[Key::Char('k')]);
    assert_eq!(v.current_option(), Some(1));
}

#[test]
fn arrows_move_tags_only_while_closed() {
    let mut v = fruits();
    v.select_all();
    assert_eq!(press(&mut v, &[Key::Left, Key::Left]), [true, true]);
    assert_eq!(v.get_tag_cursor(), Some(1));
    press(&mut v, &[Key::Right, Key::Right]);
    assert_eq!(v.get_tag_cursor(), None);
    // Open: Left and Right do nothing, and Up and Down leave the tags
    v.open();
    assert_eq!(press(&mut v, &[Key::Left, Key::Right]), [false, false]);
    assert_eq!(v.get_tag_cursor(), None);
    // Closed: Up and Down do nothing
    v.close();
    assert_eq!(press(&mut v, &[Key::Up, Key::Down]), [false, false]);
    assert_eq!(v.current_option(), Some(0));
}

#[test]
fn backspace_removes_the_tag_at_the_cursor_or_the_last_tag() {
    let mut v = fruits();
    v.select_all();
    press(&mut v, &[Key::Backspace]);
    assert_eq!(v.get_selected_indices(), &[0, 1]);
    press(&mut v, &[Key::Left, Key::Left, Key::Backspace]);
    assert_eq!(v.get_selected_indices(), &[1]);
    assert_eq!(v.get_tag_cursor(), Some(0));
    assert_eq!(
        press(&mut v, &[Key::Backspace, Key::Backspace]),
        [true, true]
    );
    assert!(v.get_selected_indices().is_empty());
    assert_eq!(v.get_tag_cursor(), None);
}

#[test]
fn delete_needs_the_tag_cursor() {
    let mut v = fruits();
    v.select_all();
    assert_eq!(press(&mut v, &[Key::Delete]), [false]);
    assert_eq!(v.selection_count(), 3);
    press(&mut v, &[Key::Left, Key::Delete]);
    assert_eq!(v.get_selected_indices(), &[0, 1]);
    assert_eq!(v.get_tag_cursor(), Some(1));
}

#[test]
fn backspace_while_open_edits_the_query() {
    let mut v = fruits();
    v.select_option(0);
    press(&mut v, &[Key::Enter, Key::Char('a'), Key::Char('n')]);
    assert_eq!(v.get_filtered(), &[1]);
    press(&mut v, &[Key::Backspace]);
    assert_eq!(v.query(), "a");
    assert_eq!(v.get_selected_indices(), &[0], "no tag is removed");
    assert_eq!(v.get_filtered().len(), 2);
    // Without search it does nothing
    let mut v = fruits().searchable(false);
    v.select_option(0);
    v.open();
    assert_eq!(press(&mut v, &[Key::Backspace]), [false]);
    assert_eq!(v.get_selected_indices(), &[0]);
}

#[test]
fn a_and_c_select_all_and_clear_while_closed_and_type_while_open() {
    let mut v = multi_select()
        .option("A")
        .option_detailed(MultiSelectOption::new("B", "b").disabled(true))
        .option("C");
    assert_eq!(press(&mut v, &[Key::Char('a')]), [true]);
    assert_eq!(v.get_selected_indices(), &[0, 2]);
    press(&mut v, &[Key::Left, Key::Char('c')]);
    assert!(v.get_selected_indices().is_empty());
    assert_eq!(v.get_tag_cursor(), None);
    press(&mut v, &[Key::Enter, Key::Char('a'), Key::Char('c')]);
    assert_eq!(v.query(), "ac");
    assert!(v.get_selected_indices().is_empty());
    // Without search, open: ignored
    let mut v = fruits().searchable(false);
    v.open();
    assert_eq!(
        press(&mut v, &[Key::Char('a'), Key::Char('c')]),
        [false, false]
    );
    assert!(v.get_selected_indices().is_empty());
}

#[test]
fn other_keys_are_ignored() {
    let mut v = fruits();
    let keys = [Key::Home, Key::End, Key::PageDown, Key::Char('x'), Key::Tab];
    assert_eq!(press(&mut v, &keys), [false; 5]);
    v.open();
    assert_eq!(press(&mut v, &[Key::Home, Key::PageUp]), [false, false]);
    assert!(v.is_open());
}

#[test]
fn a_disabled_widget_ignores_every_key() {
    let mut v = fruits().disabled(true);
    v.select_option(0);
    for k in [
        Key::Enter,
        Key::Char('a'),
        Key::Char('c'),
        Key::Left,
        Key::Backspace,
    ] {
        assert!(!v.handle_key(&k), "{k:?}");
    }
    assert!(!v.is_open());
    assert_eq!(v.get_selected_indices(), &[0]);
    assert_eq!(v.get_tag_cursor(), None);
    assert!(!v.focusable());
}

#[test]
fn tab_leaves_a_closed_widget_and_closes_an_open_one() {
    let mut v = fruits();
    let tab = KeyEvent::new(Key::Tab);
    assert_eq!(Interactive::handle_key(&mut v, &tab), EventResult::Ignored);
    v.open();
    v.set_query("b");
    assert_eq!(
        Interactive::handle_key(&mut v, &tab),
        EventResult::ConsumedAndRender
    );
    assert!(!v.is_open());
    assert_eq!(v.query(), "");
    // Other keys: consumed when they change something
    let down = KeyEvent::new(Key::Down);
    assert_eq!(Interactive::handle_key(&mut v, &down), EventResult::Ignored);
    let enter = KeyEvent::new(Key::Enter);
    assert_eq!(
        Interactive::handle_key(&mut v, &enter),
        EventResult::ConsumedAndRender
    );
}

#[test]
fn blur_closes_the_dropdown_and_focus_comes_back_closed() {
    let mut v = fruits().focused(true);
    v.open();
    v.set_query("ch");
    Interactive::on_blur(&mut v);
    assert!(!v.is_focused());
    assert!(!v.is_open());
    assert_eq!(v.query(), "");
    Interactive::on_focus(&mut v);
    assert!(v.is_focused());
    assert!(!v.is_open());
    assert!(v.focusable());
}

// A model of the state under any keys

const LABELS: [&str; 6] = ["Apple", "Banana", "Cherry", "Grape", "Mango", "Peach"];

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Enter),
        Just(Key::Escape),
        Just(Key::Char(' ')),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Char('j')),
        Just(Key::Char('k')),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Tab),
        Just(Key::Char('a')),
        Just(Key::Char('c')),
        Just(Key::Char('e')),
        Just(Key::Char('n')),
        Just(Key::Char('z')),
        Just(Key::Home),
    ]
}

/// The state the keys should leave
#[derive(Debug, Default)]
struct Model {
    disabled: Vec<bool>,
    max: Option<usize>,
    searchable: bool,
    open: bool,
    selected: Vec<usize>,
    cursor: usize,
    tag: Option<usize>,
    query: String,
    filtered: Vec<usize>,
}

impl Model {
    fn refilter(&mut self) {
        let mut m: Vec<(usize, i32)> = LABELS[..self.disabled.len()]
            .iter()
            .enumerate()
            .filter_map(|(i, l)| {
                if self.query.is_empty() {
                    Some((i, 0))
                } else {
                    fuzzy_match(&self.query, l).map(|f| (i, f.score))
                }
            })
            .collect();
        m.sort_by_key(|&(_, s)| std::cmp::Reverse(s));
        self.filtered = m.into_iter().map(|(i, _)| i).collect();
        self.cursor = 0;
    }

    fn toggle_current(&mut self) {
        if let Some(&i) = self.filtered.get(self.cursor) {
            if let Some(p) = self.selected.iter().position(|&s| s == i) {
                self.selected.remove(p);
                self.clamp_tag();
            } else if !self.disabled[i] && self.max.is_none_or(|m| self.selected.len() < m) {
                self.selected.push(i);
            }
        }
    }

    fn clamp_tag(&mut self) {
        self.tag = match self.tag {
            Some(_) if self.selected.is_empty() => None,
            Some(t) => Some(t.min(self.selected.len() - 1)),
            None => None,
        };
    }

    fn remove_at_tag(&mut self) {
        if let Some(t) = self.tag {
            self.selected.remove(t);
            self.clamp_tag();
        }
    }

    fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.refilter();
    }

    /// Apply `k`; returns whether the widget should report a change
    fn press(&mut self, k: &Key) -> bool {
        match k {
            Key::Enter if self.open => self.toggle_current(),
            Key::Enter => {
                self.open = true;
                self.tag = None;
                self.refilter();
            }
            Key::Escape if self.open => self.close(),
            Key::Escape if self.tag.is_some() => self.tag = None,
            Key::Escape => return false,
            _ if self.open => return self.press_open(k),
            _ => return self.press_closed(k),
        }
        true
    }

    fn press_open(&mut self, k: &Key) -> bool {
        let len = self.filtered.len();
        match k {
            Key::Char(' ') if !self.searchable => self.toggle_current(),
            // j and k move only when they are not search text (#889)
            Key::Down if len > 0 => self.cursor = (self.cursor + 1) % len,
            Key::Up if len > 0 => self.cursor = (self.cursor + len - 1) % len,
            Key::Char('j') if !self.searchable && len > 0 => self.cursor = (self.cursor + 1) % len,
            Key::Char('k') if !self.searchable && len > 0 => {
                self.cursor = (self.cursor + len - 1) % len
            }
            Key::Down | Key::Up => {}
            Key::Char('j' | 'k') if !self.searchable => {}
            Key::Backspace if self.searchable => {
                self.query.pop();
                self.refilter();
            }
            Key::Char(c) if self.searchable => {
                self.query.push(*c);
                self.refilter();
            }
            Key::Tab => self.close(),
            _ => return false,
        }
        true
    }

    fn press_closed(&mut self, k: &Key) -> bool {
        let n = self.selected.len();
        match k {
            Key::Left if n > 0 => self.tag = Some(self.tag.map_or(n - 1, |t| t.saturating_sub(1))),
            Key::Left => {}
            Key::Right => {
                self.tag = match self.tag {
                    Some(t) if t + 1 < n => Some(t + 1),
                    _ => None,
                }
            }
            Key::Backspace if self.tag.is_some() => self.remove_at_tag(),
            Key::Backspace => {
                self.selected.pop();
            }
            Key::Delete if self.tag.is_some() => self.remove_at_tag(),
            Key::Char('a') => {
                self.selected = (0..self.disabled.len())
                    .filter(|&i| !self.disabled[i])
                    .collect();
                if let Some(m) = self.max {
                    self.selected.truncate(m);
                }
            }
            Key::Char('c') => {
                self.selected.clear();
                self.tag = None;
            }
            _ => return false,
        }
        true
    }
}

proptest! {
    /// Any keys leave the open state, selection, cursors, query and
    /// filter a plain model gives, and the selection stays unique, enabled
    /// and within the maximum, with the tag cursor on a tag.
    #[test]
    fn prop_keys_follow_the_model(
        n in 1usize..=6,
        disabled in prop::collection::vec(prop::bool::weighted(0.25), 6),
        max in prop::option::of(0usize..4),
        searchable in any::<bool>(),
        start in prop::collection::vec(0usize..6, 0..4),
        keys in prop::collection::vec(key(), 0..60),
    ) {
        let mut v = LABELS[..n].iter().zip(&disabled).fold(multi_select(), |v, (l, &d)| {
            v.option_detailed(MultiSelectOption::new(*l, *l).disabled(d))
        });
        v = v.searchable(searchable);
        if let Some(m) = max {
            v = v.max_selections(m);
        }
        let mut m = Model {
            disabled: disabled[..n].to_vec(),
            max,
            searchable,
            ..Model::default()
        };
        m.refilter();
        // Start from a selection made the valid way
        for &i in &start {
            v.select_option(i);
            if i < n && !m.disabled[i] && !m.selected.contains(&i) && max.is_none_or(|x| m.selected.len() < x) {
                m.selected.push(i);
            }
        }
        for k in &keys {
            let want = m.press(k);
            prop_assert_eq!(v.handle_key(k), want, "{:?}", k);
            prop_assert_eq!(v.is_open(), m.open, "after {:?}", k);
            prop_assert_eq!(v.get_selected_indices(), m.selected.as_slice(), "after {:?}", k);
            prop_assert_eq!(v.get_tag_cursor(), m.tag, "after {:?}", k);
            prop_assert_eq!(v.query(), m.query.as_str(), "after {:?}", k);
            prop_assert_eq!(v.get_filtered(), m.filtered.as_slice(), "after {:?}", k);
            prop_assert_eq!(v.get_dropdown_cursor(), m.cursor, "after {:?}", k);
            let sel = v.get_selected_indices();
            prop_assert!(sel.iter().all(|&i| !m.disabled[i]));
            prop_assert!((1..sel.len()).all(|j| !sel[..j].contains(&sel[j])));
            prop_assert!(max.is_none_or(|x| sel.len() <= x));
            prop_assert!(v.get_tag_cursor().is_none_or(|t| t < sel.len()));
        }
    }
}

#[test]
fn j_and_k_are_search_text_in_a_searchable_list() {
    // Typing "jak" searches for "jak" (#889); without search they move
    let mut v = fruits().searchable(true);
    v.handle_key(&Key::Enter);
    for ch in "jak".chars() {
        v.handle_key(&Key::Char(ch));
    }
    assert_eq!(v.query(), "jak");
}
