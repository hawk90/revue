//! SearchBar keys: a model of the edit keys, and parsing after each change

use proptest::prelude::*;
use revue::event::Key;
use revue::query::Query;
use revue::widget::SearchBar;

/// A plain model of the text and the cursor (a char index).
#[derive(Default)]
struct Model {
    text: Vec<char>,
    cursor: usize,
}

impl Model {
    fn key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(c) => {
                self.text.insert(self.cursor, *c);
                self.cursor += 1;
            }
            Key::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.text.remove(self.cursor);
            }
            Key::Delete if self.cursor < self.text.len() => {
                self.text.remove(self.cursor);
            }
            Key::Left => self.cursor = self.cursor.saturating_sub(1),
            Key::Right => self.cursor = (self.cursor + 1).min(self.text.len()),
            Key::Home => self.cursor = 0,
            Key::End => self.cursor = self.text.len(),
            Key::Escape => *self = Model::default(),
            Key::Backspace | Key::Delete => {}
            _ => return false,
        }
        true
    }
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char(':')),
        Just(Key::Char('>')),
        Just(Key::Char(' ')),
        Just(Key::Char('한')),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::Escape),
        Just(Key::Enter),
        Just(Key::PageUp),
        Just(Key::Tab),
    ]
}

proptest! {
    /// From a valid or an invalid query, any keys leave the text a char
    /// model gives, and the cursor where the model has it (checked by
    /// typing a marker at the end); the query and the error follow
    /// `Query::parse` of the text after every key.
    #[test]
    fn prop_search_bar_keys_follow_the_model(
        // Only a sort with a bad direction or no field is invalid, which
        // single keys rarely type: start from one, or a valid query
        start in prop_oneof![Just(""), Just("sort:"), Just("sort:a:x"), Just("sort:a:asc"), Just("a:>1")],
        keys in prop::collection::vec(key(), 0..60),
    ) {
        let mut s = SearchBar::new();
        s.set_query(start);
        let mut model = Model { text: start.chars().collect(), cursor: start.chars().count() };
        for k in &keys {
            prop_assert_eq!(s.handle_key(k), model.key(k), "{:?}", k);
            let text: String = model.text.iter().collect();
            prop_assert_eq!(s.get_input(), text.as_str(), "after {:?}", k);
            let parsed = Query::parse(&text);
            prop_assert_eq!(s.is_valid(), parsed.is_ok(), "{:?}", text);
            prop_assert_eq!(s.query().is_some(), parsed.is_ok());
            prop_assert_eq!(s.error().is_some(), parsed.is_err());
        }
        s.input('#');
        model.key(&Key::Char('#'));
        prop_assert_eq!(s.get_input(), model.text.iter().collect::<String>());
    }
}
