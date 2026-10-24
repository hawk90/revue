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

// The standard editing keys, with the word keys over the whole value (#985)

#[test]
fn word_keys_go_over_the_whole_value() {
    use revue::event::KeyEvent;
    let mut input = MaskedInput::new().value("my secret pass");
    input.set_cursor(9);
    // Ctrl+Left goes to the start, not to "secret"
    assert!(input.handle_key_event(&KeyEvent::ctrl(Key::Left)));
    assert_eq!(input.get_cursor(), 0);
    assert!(input.handle_key_event(&KeyEvent::ctrl(Key::Right)));
    assert_eq!(input.get_cursor(), 14);
    // Ctrl+Backspace deletes back to the start
    input.set_cursor(9);
    assert!(input.handle_key_event(&KeyEvent::ctrl(Key::Backspace)));
    assert_eq!(input.get_value(), " pass");
    assert_eq!(input.get_cursor(), 0);
    // Undo puts it back; Ctrl+Delete deletes to the end
    assert!(input.handle_key_event(&KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(input.get_value(), "my secret pass");
    input.set_cursor(2);
    assert!(input.handle_key_event(&KeyEvent::ctrl(Key::Delete)));
    assert_eq!(input.get_value(), "my");
    // Shift+Ctrl+Left selects to the start; typing replaces it
    assert!(input.handle_key_event(&KeyEvent {
        shift: true,
        ..KeyEvent::ctrl(Key::Left)
    }));
    input.handle_key_event(&KeyEvent::new(Key::Char('x')));
    assert_eq!(input.get_value(), "x");
    // A Ctrl letter it does not use is the app's
    assert!(!input.handle_key_event(&KeyEvent::ctrl(Key::Char('k'))));
}

#[test]
fn typing_over_a_selection_is_allowed_at_max_length() {
    use revue::event::KeyEvent;
    let mut input = MaskedInput::new().max_length(4).value("1234");
    // Full: a typed char is eaten
    assert!(input.handle_key(&Key::Char('5')));
    assert_eq!(input.get_value(), "1234");
    // With the last char selected, typing replaces it
    input.handle_key_event(&KeyEvent {
        shift: true,
        ..KeyEvent::new(Key::Left)
    });
    assert!(input.handle_key(&Key::Char('5')));
    assert_eq!(input.get_value(), "1235");
}

// ─── Characters as seen (#1298) ─────────────────────────────────────────────

/// One mask char per char let the cursor moves show where a letter with its
/// accent or an emoji sequence was (two bullets in one move); the mask now
/// has one per character as seen, the unit the cursor moves by
#[test]
fn the_mask_has_one_char_per_character_the_cursor_moves_over() {
    let input = MaskedInput::new()
        .mask_style(MaskStyle::Full)
        .value("e\u{301}x👩\u{200D}💻");
    assert_eq!(input.masked_display(), "●●●");

    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(1))
        .value("ab👩\u{200D}💻");
    assert_eq!(input.masked_display(), "●●👩\u{200D}💻");
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowFirst(1))
        .value("e\u{301}xy");
    assert_eq!(input.masked_display(), "e\u{301}●●");
}

/// Masked, Backspace takes the whole character: taking only the accent
/// would change nothing the mask shows. Revealed, it takes the accent first,
/// as in any field.
#[test]
fn masked_backspace_takes_a_whole_character() {
    let mut input = MaskedInput::new().allow_reveal(true).value("ae\u{301}");
    input.handle_key(&Key::Backspace);
    assert_eq!(input.get_value(), "a");
    assert_eq!(input.masked_display(), "●");

    let mut input = MaskedInput::new().allow_reveal(true).value("ae\u{301}");
    input.toggle_reveal();
    input.handle_key(&Key::Backspace);
    assert_eq!(input.get_value(), "ae");
}

/// max_length and min_length count characters as seen
#[test]
fn lengths_count_characters_as_seen() {
    let mut input = MaskedInput::new().max_length(2).value("e\u{301}");
    input.handle_key(&Key::Char('x'));
    assert_eq!(input.get_value(), "e\u{301}x");
    input.handle_key(&Key::Char('y'));
    assert_eq!(input.get_value(), "e\u{301}x", "full at two characters");

    let input = MaskedInput::new().max_length(1).value("👩\u{200D}💻z");
    assert_eq!(input.get_value(), "👩\u{200D}💻");
}
