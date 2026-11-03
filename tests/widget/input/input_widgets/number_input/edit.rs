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
                let buf: Vec<char> = self.value.to_string().chars().collect();
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
            // A move is the field's even at an edge, as in every
            // single-line field (#985)
            (Key::Left, Some((_, cur))) => *cur = cur.saturating_sub(1),
            (Key::Right, Some((buf, cur))) => *cur = (*cur + 1).min(buf.len()),
            _ => return false,
        }
        true
    }

    fn shown(&self) -> String {
        match &self.edit {
            Some((buf, _)) => buf.iter().collect(),
            // No precision: the value as given
            None => self.value.to_string(),
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
    /// leaves the value outside them. (High to low and NaN: their own tests.)
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

// The standard editing keys while editing (#985)

#[test]
fn editing_takes_the_modified_editing_keys() {
    use revue::event::KeyEvent;
    let mut n = NumberInput::new().value(1234.0);
    // Not editing: an editing key is not the field's
    assert!(!n.handle_key_event(&KeyEvent::ctrl(Key::Backspace)));
    n.handle_key(&Key::Enter);
    // Shift+Left selects the last digit; typing replaces it
    assert!(n.handle_key_event(&KeyEvent {
        shift: true,
        ..KeyEvent::new(Key::Left)
    }));
    n.handle_key(&Key::Char('9'));
    assert_eq!(n.display_string(), "1239");
    // Undo puts back the digit and its selection; past it, a word delete
    // clears the number
    assert!(n.handle_key_event(&KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(n.display_string(), "1234");
    n.handle_key(&Key::Right);
    assert!(n.handle_key_event(&KeyEvent::ctrl(Key::Backspace)));
    assert_eq!(n.display_string(), "");
    // A letter is not typed, with or without Ctrl
    assert!(!n.handle_key(&Key::Char('x')));
    assert!(!n.handle_key_event(&KeyEvent::ctrl(Key::Char('x'))));
    // `-` over a selection that starts the number is allowed
    n.handle_key(&Key::Char('5'));
    n.handle_key_event(&KeyEvent::ctrl(Key::Char('a')));
    n.handle_key(&Key::Char('-'));
    assert_eq!(n.display_string(), "-");
}

/// With a precision the value is held at it: typed, stepped or set, the
/// value read back is the one shown (#1016; as Element Plus and Qt)
#[test]
fn a_precision_holds_the_value_at_its_decimals() {
    use revue::widget::integer_input;
    // integer_input takes 1.5 as 2, shown and held
    let mut input = integer_input().focused(true);
    for k in [Key::Char('1'), Key::Char('.'), Key::Char('5'), Key::Enter] {
        input.handle_key(&k);
    }
    assert_eq!(
        (input.get_value(), input.display_string()),
        (2.0, "2".to_string())
    );
    // Set and stepped values agree with the display
    let mut input = NumberInput::new().precision(1).step(0.1);
    for v in [1.25, 2.555, -0.04, 0.1 + 0.2] {
        input.set_value(v);
        let shown: f64 = input.display_string().parse().unwrap();
        assert_eq!(input.get_value(), shown, "{v}");
        assert!(
            !input.display_string().starts_with("-0"),
            "{v}: no negative zero"
        );
    }
    input.set_value(0.0);
    for _ in 0..3 {
        input.increment();
    }
    assert_eq!(input.get_value(), 0.3);
    // A precision given after the value rounds it then
    let input = NumberInput::new().value(19.987).precision(2);
    assert_eq!(input.get_value(), 19.99);
    // A bound off the decimals is rounded too, so the value stays inside
    // what is shown
    let mut input = NumberInput::new().min(0.04).max(9.96).precision(1);
    input.set_value(-5.0);
    assert_eq!(input.get_value(), 0.0);
    input.set_value(50.0);
    assert_eq!(input.get_value(), 10.0);
}

/// Without a precision (the default) the value is kept and shown as given
#[test]
fn no_precision_keeps_the_value_as_given() {
    let input = NumberInput::new().value(42.5);
    assert_eq!(
        (input.get_value(), input.display_string()),
        (42.5, "42.5".to_string())
    );
    let input = NumberInput::new().value(3.0);
    assert_eq!(input.display_string(), "3");
    let mut input = NumberInput::new().focused(true);
    for k in [
        Key::Char('1'),
        Key::Char('.'),
        Key::Char('2'),
        Key::Char('5'),
        Key::Enter,
    ] {
        input.handle_key(&k);
    }
    assert_eq!(input.get_value(), 1.25);
}
