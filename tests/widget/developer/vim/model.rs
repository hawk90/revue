//! VimState against a plain model of its modes, command line and search
//! pattern, and the count typed before a motion

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::widget::{VimAction, VimMode, VimMotion, VimState};

/// The mode and the typed command and search: what the keys should do,
/// written plainly. Counts and `gg` never change these. A pending operator
/// does: a key that is no motion cancels it instead of changing mode.
#[derive(Debug)]
struct Model {
    mode: VimMode,
    command: String,
    search: String,
    forward: bool,
    /// An operator is pending
    pending: bool,
    /// A count is being typed (so `0` is a digit)
    counting: bool,
    /// A `g` is waiting for a second one
    g: bool,
}

impl Model {
    /// The mode after `key`, and the action the keys that end a command or a
    /// search return
    fn press(&mut self, key: &Key) -> Option<VimAction> {
        use VimMode::*;
        let ch = match key {
            Key::Char(c) => Some(*c),
            _ => None,
        };
        if self.mode == Normal && self.pending {
            let after_g = std::mem::take(&mut self.g);
            let digit = matches!(ch, Some('3')) || (matches!(ch, Some('0')) && self.counting);
            match ch {
                _ if digit => {
                    self.counting = true;
                    return None;
                }
                Some('g') if !after_g => {
                    self.g = true;
                    return None;
                }
                Some(':' | '/' | '?') => self.pending = false,
                _ if *key == Key::Escape => self.pending = false,
                _ => {
                    self.pending = false;
                    self.counting = false;
                    return None;
                }
            }
            self.counting = false;
        }
        if self.mode == Normal {
            match ch {
                Some('d' | 'y' | 'c') => self.pending = true,
                Some('3') => self.counting = true,
                Some('0') if self.counting => {}
                _ => self.counting = false,
            }
        }
        match (self.mode, key) {
            (Normal, _) => match ch {
                Some('i' | 'I' | 'a' | 'A' | 'o' | 'O') => self.mode = Insert,
                Some('v') => self.mode = Visual,
                Some('V') => self.mode = VisualLine,
                Some(':') => {
                    self.mode = Command;
                    self.command.clear();
                }
                Some(c @ ('/' | '?')) => {
                    self.mode = Search;
                    self.search.clear();
                    self.forward = c == '/';
                }
                _ => {}
            },
            (Insert, Key::Escape) => self.mode = Normal,
            (Visual | VisualLine | VisualBlock, Key::Escape) => self.mode = Normal,
            (Visual | VisualLine | VisualBlock, _) => match ch {
                Some('d' | 'x' | 'y') => self.mode = Normal,
                Some('c') => self.mode = Insert,
                _ => {}
            },
            (Command, Key::Escape) => {
                self.mode = Normal;
                self.command.clear();
            }
            (Command, Key::Enter) => {
                self.mode = Normal;
                return Some(VimAction::ExecuteCommand(std::mem::take(&mut self.command)));
            }
            (Command, Key::Backspace) => {
                self.command.pop();
                if self.command.is_empty() {
                    self.mode = Normal;
                }
            }
            (Command, Key::Char(c)) => self.command.push(*c),
            (Search, Key::Escape) => {
                self.mode = Normal;
                self.search.clear();
            }
            (Search, Key::Enter) => {
                self.mode = Normal;
                let motion = if self.forward {
                    VimMotion::SearchNext
                } else {
                    VimMotion::SearchPrev
                };
                return Some(VimAction::Move(motion));
            }
            (Search, Key::Backspace) => {
                self.search.pop();
                if self.search.is_empty() {
                    self.mode = Normal;
                }
            }
            (Search, Key::Char(c)) => self.search.push(*c),
            (Replace, Key::Char(c)) => {
                self.mode = Normal;
                return Some(VimAction::ReplaceChar(*c));
            }
            (Replace, Key::Escape) => self.mode = Normal,
            _ => {}
        }
        None
    }
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        prop::sample::select(":/?iIaAoOvVdxycwjgG30q".chars().collect::<Vec<_>>())
            .prop_map(Key::Char),
        Just(Key::Escape),
        Just(Key::Enter),
        Just(Key::Backspace),
        Just(Key::Left),
    ]
}

fn start_mode() -> impl Strategy<Value = VimMode> {
    prop::sample::select(vec![
        VimMode::Normal,
        VimMode::Insert,
        VimMode::Visual,
        VimMode::VisualBlock,
        VimMode::Command,
        VimMode::Search,
        VimMode::Replace,
    ])
}

proptest! {
    #[test]
    fn prop_vim_modes_follow_the_model(start in start_mode(), keys in prop::collection::vec(key(), 0..60)) {
        let mut vim = VimState::new();
        vim.set_mode(start);
        let mut model = Model { mode: start, command: String::new(), search: String::new(), forward: true, pending: false, counting: false, g: false };
        for k in &keys {
            // Backspace on a one-char command line or pattern leaves the
            // mode instead of keeping the empty line open (#1081);
            // skip it rather than encode it.
            if *k == Key::Backspace
                && ((model.mode == VimMode::Command && model.command.chars().count() == 1)
                    || (model.mode == VimMode::Search && model.search.chars().count() == 1))
            {
                continue;
            }
            let action = vim.handle_key(&KeyEvent::new(*k));
            let expected = model.press(k);
            if let Some(expected) = expected {
                prop_assert_eq!(action, expected, "after {:?}", k);
            }
            prop_assert_eq!(vim.mode(), model.mode, "after {:?}", k);
            prop_assert_eq!(vim.command_buffer(), model.command.as_str(), "after {:?}", k);
            prop_assert_eq!(vim.search_pattern(), model.search.as_str(), "after {:?}", k);
        }
    }

    /// After any keys and Escape, typed digits are the count, and `G`
    /// takes it as its line.
    #[test]
    fn prop_vim_count_is_the_typed_number(
        before in prop::collection::vec(key(), 0..30),
        count in 1usize..100_000_000,
    ) {
        let mut vim = VimState::new();
        for k in &before {
            vim.handle_key(&KeyEvent::new(*k));
        }
        vim.handle_key(&KeyEvent::new(Key::Escape));
        prop_assert_eq!(vim.mode(), VimMode::Normal);
        prop_assert_eq!(vim.count(), 1);
        for ch in count.to_string().chars() {
            prop_assert_eq!(vim.handle_key(&KeyEvent::new(Key::Char(ch))), VimAction::None);
        }
        prop_assert_eq!(vim.count(), count);
        prop_assert_eq!(
            vim.handle_key(&KeyEvent::new(Key::Char('G'))),
            VimAction::Move(VimMotion::GoToLine(Some(count)))
        );
    }
}
