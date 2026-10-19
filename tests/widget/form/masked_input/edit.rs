//! MaskedInput keys: a model of the edit keys, `max_length`, `disabled` and
//! the peek countdown and what it reveals

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::{MaskStyle, MaskedInput};

/// The peek countdown typing starts (`DEFAULT_PEEK_TIMEOUT`).
const PEEK: usize = 10;

/// A plain model of the value (chars) and the cursor (a char index).
#[derive(Clone, Debug, Default)]
struct Model {
    text: Vec<char>,
    cursor: usize,
    max: usize,
    disabled: bool,
    peek: bool,
    countdown: usize,
    /// The char typed last
    typed: Option<char>,
}

#[derive(Clone, Debug)]
enum Op {
    Key(Key),
    Update,
}

impl Model {
    /// Apply `key`; whether the input handles it.
    fn key(&mut self, key: &Key) -> bool {
        if self.disabled {
            return false;
        }
        match key {
            Key::Char(c) if !c.is_control() => {
                if self.max == 0 || self.text.len() < self.max {
                    self.text.insert(self.cursor, *c);
                    self.cursor += 1;
                    self.typed = Some(*c);
                    if self.peek {
                        self.countdown = PEEK;
                    }
                }
                return true;
            }
            Key::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.text.remove(self.cursor);
                }
            }
            Key::Delete => {
                if self.cursor < self.text.len() {
                    self.text.remove(self.cursor);
                }
            }
            Key::Left => self.cursor = self.cursor.saturating_sub(1),
            Key::Right => self.cursor = (self.cursor + 1).min(self.text.len()),
            Key::Home => self.cursor = 0,
            Key::End => self.cursor = self.text.len(),
            _ => return false,
        }
        // Moving and deleting end the peek: the char before the cursor is
        // no longer the one just typed (#1073)
        self.countdown = 0;
        true
    }

    /// The masked text: all masked, but for the char just typed while the
    /// peek runs
    fn display(&self) -> String {
        (0..self.text.len())
            .map(|i| {
                if self.countdown > 0 && i + 1 == self.cursor {
                    self.typed.unwrap()
                } else {
                    '●'
                }
            })
            .collect()
    }
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        // Typing is weighted up so the value reaches `max_length`.
        4 => prop::sample::select(vec!['a', 'Z', '7', '!', ' ', '한', '\u{7}'])
            .prop_map(|c| Op::Key(Key::Char(c))),
        1 => Just(Op::Key(Key::Backspace)),
        1 => Just(Op::Key(Key::Delete)),
        1 => Just(Op::Key(Key::Left)),
        1 => Just(Op::Key(Key::Right)),
        1 => Just(Op::Key(Key::Home)),
        1 => Just(Op::Key(Key::End)),
        1 => prop::sample::select(vec![Key::Up, Key::Tab, Key::Enter, Key::Escape])
            .prop_map(Op::Key),
        1 => Just(Op::Update),
    ]
}

proptest! {
    #[test]
    fn prop_keys_match_model(
        start in prop::sample::select(vec!["", "ab", "비밀번호"]),
        max in prop::sample::select(vec![0usize, 3, 5]),
        disabled in prop::bool::weighted(0.15),
        peek in any::<bool>(),
        ops in prop::collection::vec(op(), 0..40),
    ) {
        let style = if peek { MaskStyle::Peek } else { MaskStyle::Full };
        let mut input = MaskedInput::new()
            .mask_style(style)
            .max_length(max)
            .value(start)
            .disabled(disabled);
        let text: Vec<char> = start.chars().collect();
        let text = if max > 0 { text.into_iter().take(max).collect() } else { text };
        let mut model = Model {
            cursor: text.len(),
            text,
            max,
            disabled,
            peek,
            countdown: 0,
            typed: None,
        };
        prop_assert_eq!(input.get_value(), model.text.iter().collect::<String>());
        for op in &ops {
            match op {
                Op::Key(key) => {
                    let handled = input.handle_key(key);
                    prop_assert_eq!(handled, model.key(key), "{:?}", key);
                }
                Op::Update => {
                    input.update();
                    model.countdown = model.countdown.saturating_sub(1);
                }
            }
            prop_assert_eq!(input.get_value(), model.text.iter().collect::<String>());
            prop_assert_eq!(input.get_cursor(), model.cursor);
            prop_assert_eq!(input.get_peek_countdown(), model.countdown);
            prop_assert_eq!(input.masked_display(), model.display(), "after {:?}", op);
            prop_assert!(max == 0 || input.get_value().chars().count() <= max);
        }
    }

    /// The methods behind the keys do what the keys do, also on a disabled
    /// input for the cursor moves (only edits are blocked).
    #[test]
    fn prop_methods_match_keys(
        start in prop::sample::select(vec!["", "ab", "비밀번호"]),
        disabled in any::<bool>(),
        ops in prop::collection::vec(0u8..7, 0..30),
    ) {
        let mut by_key = MaskedInput::new().value(start);
        let mut by_method = by_key.clone().disabled(disabled);
        for op in ops {
            let key = match op {
                0 => { by_method.insert_char('x'); Key::Char('x') }
                1 => { by_method.delete_backward(); Key::Backspace }
                2 => { by_method.delete_forward(); Key::Delete }
                3 => { by_method.move_left(); Key::Left }
                4 => { by_method.move_right(); Key::Right }
                5 => { by_method.move_start(); Key::Home }
                _ => { by_method.move_end(); Key::End }
            };
            if !disabled || op >= 3 {
                by_key.handle_key(&key);
            }
            prop_assert_eq!(by_method.get_value(), by_key.get_value());
            prop_assert_eq!(by_method.get_cursor(), by_key.get_cursor());
        }
    }
}
