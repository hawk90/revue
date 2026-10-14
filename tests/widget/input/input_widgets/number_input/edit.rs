//! NumberInput keys: a model of direct entry and the step keys, and the
//! value kept within the bounds

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::NumberInput;

const MIN: f64 = -50.0;
const MAX: f64 = 50.0;

/// A plain model: the value, and the edit buffer with its cursor while
/// editing.
struct Model {
    value: f64,
    edit: Option<(Vec<char>, usize)>,
}

impl Model {
    fn clamp(v: f64) -> f64 {
        v.clamp(MIN, MAX)
    }

    fn commit(&mut self) {
        if let Some((buf, _)) = self.edit.take() {
            if let Ok(v) = buf.iter().collect::<String>().parse::<f64>() {
                self.value = Self::clamp(v);
            }
        }
    }

    fn key(&mut self, key: &Key) -> bool {
        let step = match key {
            Key::Up => Some(1.0),
            Key::Down => Some(-1.0),
            Key::PageUp => Some(10.0),
            Key::PageDown => Some(-10.0),
            _ => None,
        };
        if let Some(d) = step {
            self.commit();
            self.value = Self::clamp(self.value + d);
            return true;
        }
        match (key, &mut self.edit) {
            (Key::Home, _) => {
                self.commit();
                self.value = MIN;
            }
            (Key::End, _) => {
                self.commit();
                self.value = MAX;
            }
            (Key::Enter, Some(_)) => self.commit(),
            (Key::Enter, None) => {
                let buf: Vec<char> = format!("{:.0}", self.value).chars().collect();
                let len = buf.len();
                self.edit = Some((buf, len));
            }
            (Key::Escape, Some(_)) => self.edit = None,
            (Key::Char(c), _) if c.is_ascii_digit() || *c == '.' || *c == '-' => {
                let (buf, cur) = self.edit.get_or_insert((Vec::new(), 0));
                let ok = match c {
                    '-' => *cur == 0 && !buf.contains(&'-'),
                    '.' => !buf.contains(&'.'),
                    _ => true,
                };
                if ok {
                    buf.insert(*cur, *c);
                    *cur += 1;
                }
            }
            (Key::Backspace, Some((buf, cur))) => {
                if *cur > 0 {
                    *cur -= 1;
                    buf.remove(*cur);
                }
            }
            (Key::Delete, Some((buf, cur))) => {
                if *cur < buf.len() {
                    buf.remove(*cur);
                }
            }
            (Key::Left, Some((_, cur))) if *cur > 0 => *cur -= 1,
            (Key::Right, Some((buf, cur))) if *cur < buf.len() => *cur += 1,
            _ => return false,
        }
        true
    }

    fn shown(&self) -> String {
        match &self.edit {
            Some((buf, _)) => buf.iter().collect(),
            None => format!("{:.0}", self.value),
        }
    }
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        (0u32..10).prop_map(|d| Key::Char(char::from_digit(d, 10).unwrap())),
        Just(Key::Char('.')),
        Just(Key::Char('-')),
        Just(Key::Char('x')),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Enter),
        Just(Key::Escape),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::PageUp),
        Just(Key::PageDown),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::Tab),
    ]
}

proptest! {
    /// Any keys leave the value, the editing state, the shown text and the
    /// cursor a plain model gives (the cursor checked by typing a digit at
    /// the end), and report use as the model does.
    #[test]
    fn prop_number_input_keys_follow_the_model(
        start in -60i32..60,
        keys in prop::collection::vec(key(), 0..50),
    ) {
        let mut input = NumberInput::new().min(MIN).max(MAX).value(start as f64);
        let mut model = Model { value: Model::clamp(start as f64), edit: None };
        for k in &keys {
            prop_assert_eq!(input.handle_key(k), model.key(k), "{:?}", k);
            prop_assert_eq!(input.get_value(), model.value, "after {:?}", k);
            prop_assert_eq!(input.is_editing(), model.edit.is_some(), "after {:?}", k);
            prop_assert_eq!(input.display_string(), model.shown(), "after {:?}", k);
        }
        input.handle_key(&Key::Char('7'));
        model.key(&Key::Char('7'));
        prop_assert_eq!(input.display_string(), model.shown());
    }

    /// With bounds given low to high, no sequence of sets, steps or edits
    /// leaves the value outside them. (High to low and NaN: #1016.)
    #[test]
    fn prop_number_input_value_stays_in_bounds(
        a in -1e3f64..1e3,
        b in -1e3f64..1e3,
        step in 0.0f64..100.0,
        sets in prop::collection::vec(-1e4f64..1e4, 0..10),
        keys in prop::collection::vec(key(), 0..30),
    ) {
        let (lo, hi) = (a.min(b), a.max(b));
        let mut input = NumberInput::new().min(lo).max(hi).step(step);
        for v in sets {
            input.set_value(v);
            prop_assert!((lo..=hi).contains(&input.get_value()));
        }
        for k in &keys {
            input.handle_key(k);
            input.commit_edit();
            prop_assert!((lo..=hi).contains(&input.get_value()), "{} after {:?}", input.get_value(), k);
        }
    }
}
