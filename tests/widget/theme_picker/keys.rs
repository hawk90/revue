//! ThemePicker keys against a plain model of the open state, the selection
//! and the applied theme

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::style::{get_theme, set_theme_by_id, use_theme};
use revue::widget::traits::EventResult;
use revue::widget::{Interactive, ThemePicker};
use serial_test::serial;

/// Theme ids to build lists from; "bogus" is not registered
const POOL: [&str; 5] = ["light", "dracula", "nord", "monokai", "bogus"];

const KEYS: [Key; 10] = [
    Key::Enter,
    Key::Char(' '),
    Key::Up,
    Key::Down,
    Key::Char('j'),
    Key::Char('k'),
    Key::Escape,
    Key::Tab,
    Key::Char('x'),
    Key::Left,
];

struct Model {
    ids: Vec<&'static str>,
    open: bool,
    selected: usize,
    /// Name of the theme the app shows
    current: String,
}

impl Model {
    fn apply(&mut self) {
        if let Some(theme) = self.ids.get(self.selected).and_then(|id| get_theme(id)) {
            self.current = theme.name;
        }
    }

    fn next(&mut self) {
        if self.selected + 1 < self.ids.len() {
            self.selected += 1;
        }
    }

    fn press(&mut self, key: Key) -> EventResult {
        match key {
            Key::Enter | Key::Char(' ') => {
                if self.open {
                    self.apply();
                    self.open = false;
                } else {
                    self.open = true;
                }
                EventResult::Consumed
            }
            Key::Up | Key::Char('k') if self.open => {
                self.selected = self.selected.saturating_sub(1);
                EventResult::Consumed
            }
            Key::Down | Key::Char('j') if self.open => {
                self.next();
                EventResult::Consumed
            }
            Key::Escape if self.open => {
                self.open = false;
                EventResult::Consumed
            }
            Key::Tab => {
                self.next();
                self.apply();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}

proptest! {
    #[test]
    #[serial]
    fn keys_match_the_model(
        picks in proptest::collection::vec(0..POOL.len(), 0..5),
        keys in proptest::collection::vec(0..KEYS.len(), 0..40),
    ) {
        set_theme_by_id("dark");
        let ids: Vec<&'static str> = picks.iter().map(|&i| POOL[i]).collect();
        let mut picker = ThemePicker::new().themes(ids.clone());
        let mut model = Model {
            ids,
            open: false,
            selected: 0,
            current: use_theme().get().name,
        };

        for &k in &keys {
            let key = KEYS[k];
            if key == Key::Tab && model.selected + 1 >= model.ids.len() {
                // Tab at the last theme stops instead of wrapping (#1097)
                break;
            }
            let expected = model.press(key);
            let result = picker.handle_key(&KeyEvent::new(key));
            prop_assert_eq!(result, expected, "{:?}", key);
            prop_assert_eq!(picker.is_open(), model.open);
            prop_assert_eq!(picker.selected_id(), model.ids.get(model.selected).copied());
            prop_assert_eq!(use_theme().get().name, model.current.clone());
        }
        set_theme_by_id("dark");
    }
}

#[test]
#[serial]
fn enter_applies_the_theme_it_closes_on() {
    set_theme_by_id("dark");
    let mut picker = ThemePicker::new().themes(["light", "nord"]);
    for key in [Key::Enter, Key::Down, Key::Enter] {
        picker.handle_key(&KeyEvent::new(key));
    }
    assert_eq!(use_theme().get().name, get_theme("nord").unwrap().name);

    // Escape closes without applying
    for key in [Key::Enter, Key::Up, Key::Escape] {
        picker.handle_key(&KeyEvent::new(key));
    }
    assert!(!picker.is_open());
    assert_eq!(picker.selected_id(), Some("light"));
    assert_eq!(use_theme().get().name, get_theme("nord").unwrap().name);
    set_theme_by_id("dark");
}

#[test]
#[serial]
fn tab_applies_the_next_theme_while_closed() {
    set_theme_by_id("dark");
    let mut picker = ThemePicker::new().themes(["light", "nord", "dracula"]);
    assert_eq!(
        picker.handle_key(&KeyEvent::new(Key::Tab)),
        EventResult::Consumed
    );
    assert!(!picker.is_open());
    assert_eq!(picker.selected_id(), Some("nord"));
    assert_eq!(use_theme().get().name, get_theme("nord").unwrap().name);
    set_theme_by_id("dark");
}
